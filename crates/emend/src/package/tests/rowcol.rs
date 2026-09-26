//! Column and row formats, panes and merges on byte-built worksheets
//! (`specs/0018-rows-and-columns.md`, "P1"): each test builds the smallest
//! package with its worksheet holding a given body, records an edit, and
//! compares the written part with the bytes the design gives, every other
//! part asserted untouched.

use super::*;

/// The worksheet part after `edit` on the smallest package whose worksheet
/// holds `body`, as text; every other part is asserted untouched.
#[track_caller]
fn edited(body: &str, edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> String {
    let mut parts = smallest();
    set(&mut parts, SHEET, worksheet(body));
    let output = edited_output(parts.clone(), edit);
    assert_untouched(&output, &parts, &[SHEET]);
    String::from_utf8(stored_data(&output, SHEET)).expect("the part is UTF-8")
}

/// The refusal a write of `edit` on that package gives, with no output.
#[track_caller]
fn refused(body: &str, edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> Error {
    let mut parts = smallest();
    set(&mut parts, SHEET, worksheet(body));
    let (_, mut package) = opened(parts);
    edit(&mut package);
    match refusal(package) {
        Ok(error) => error,
        Err(out) => panic!(
            "the write was not refused: it wrote the worksheet {}",
            String::from_utf8_lossy(&stored_data(&out, SHEET))
        ),
    }
}

/// The template worksheet after `edit` on a new package, read back through
/// a second open of the output, since an added part is deflated.
#[track_caller]
fn new_sheet(edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> String {
    let mut package = new_package();
    edit(&mut package);
    let out = {
        let _guard = TEMPORARY_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let mut out = Vec::new();
        package.write_to(&mut out).expect("write_to");
        out
    };
    let mut reopened = Package::open(Cursor::new(out)).expect("open the output");
    let index = reopened.part(SHEET).expect("the worksheet");
    let mut text = String::new();
    reopened
        .read_part(index)
        .expect("read_part")
        .read_to_string(&mut text)
        .expect("UTF-8");
    text
}

const COLS5: &str =
    "<cols><col min=\"1\" max=\"5\" width=\"9\" customWidth=\"1\" style=\"3\"/></cols>";

fn area(a1: &str) -> Area {
    Area::parse(a1.as_bytes()).expect("an area")
}

// --- Area ---

#[test]
fn an_area_parses_normalises_and_writes_back() {
    let a = area("B3:A1");
    assert_eq!(a.first, at("A1"));
    assert_eq!(a.last, at("B3"));
    assert_eq!(a.a1(), "A1:B3");
    assert_eq!(a.cells(), 6);
    let one = area("C4");
    assert_eq!((one.first, one.last), (at("C4"), at("C4")));
    assert_eq!(one.a1(), "C4");
    assert_eq!(one.cells(), 1);
    for bad in ["", "A1:", ":A1", "A0", "a1", "A1:B2:C3", "$A$1"] {
        assert_eq!(Area::parse(bad.as_bytes()), None, "{bad:?}");
    }
}

// --- columns ---

#[test]
fn a_col_is_split_at_its_first_middle_and_last_column() {
    let body = |cols: &str| format!("{cols}<sheetData/>");
    let expect = [
        (
            1,
            "<col min=\"1\" max=\"1\" width=\"20.5\" customWidth=\"1\" style=\"3\"/>\
             <col min=\"2\" max=\"5\" width=\"9\" customWidth=\"1\" style=\"3\"/>",
        ),
        (
            3,
            "<col min=\"1\" max=\"2\" width=\"9\" customWidth=\"1\" style=\"3\"/>\
             <col min=\"3\" max=\"3\" width=\"20.5\" customWidth=\"1\" style=\"3\"/>\
             <col min=\"4\" max=\"5\" width=\"9\" customWidth=\"1\" style=\"3\"/>",
        ),
        (
            5,
            "<col min=\"1\" max=\"4\" width=\"9\" customWidth=\"1\" style=\"3\"/>\
             <col min=\"5\" max=\"5\" width=\"20.5\" customWidth=\"1\" style=\"3\"/>",
        ),
    ];
    for (column, cols) in expect {
        let got = edited(&body(COLS5), |p| {
            p.set_column_width(SHEET, column, Some(20.5))
                .expect("recorded")
        });
        assert_eq!(
            got,
            worksheet(&body(&format!("<cols>{cols}</cols>"))),
            "column {column}"
        );
    }
    // A one-column element takes the change in place.
    let one = "<cols><col min=\"2\" max=\"2\" width=\"9\" style=\"1\"/></cols>";
    let got = edited(&body(one), |p| {
        p.set_column_hidden(SHEET, 2, true).expect("recorded")
    });
    assert_eq!(
        got,
        worksheet(&body(
            "<cols><col min=\"2\" max=\"2\" width=\"9\" style=\"1\" hidden=\"1\"/></cols>"
        ))
    );
}

#[test]
fn a_width_none_writes_nothing_on_an_uncovered_column_and_drops_a_bare_piece() {
    let body = "<cols><col min=\"1\" max=\"2\" width=\"9\" customWidth=\"1\"/></cols><sheetData/>";
    // Column 4 has no element: nothing is written, the part is its bytes.
    assert_eq!(
        edited(body, |p| p
            .set_column_width(SHEET, 4, None)
            .expect("recorded")),
        worksheet(body)
    );
    // Column 2 is covered: split, and the piece left with min and max alone
    // is dropped.
    assert_eq!(
        edited(body, |p| p
            .set_column_width(SHEET, 2, None)
            .expect("recorded")),
        worksheet(
            "<cols><col min=\"1\" max=\"1\" width=\"9\" customWidth=\"1\"/></cols><sheetData/>"
        )
    );
    // A hidden flag removed from a column no element covers: nothing either.
    assert_eq!(
        edited(body, |p| p
            .set_column_hidden(SHEET, 7, false)
            .expect("recorded")),
        worksheet(body)
    );
}

#[test]
fn a_column_past_every_element_gets_one_placed_by_min() {
    let body = "<cols><col min=\"2\" max=\"3\" width=\"9\"/><col min=\"6\" max=\"6\" width=\"4\"/></cols><sheetData/>";
    let got = edited(body, |p| {
        p.set_column_width(SHEET, 1, Some(7.0)).expect("recorded");
        p.set_column_hidden(SHEET, 4, true).expect("recorded");
        p.set_column_width(SHEET, 9, Some(12.25)).expect("recorded");
        p.set_column_hidden(SHEET, 9, true).expect("recorded");
    });
    assert_eq!(
        got,
        worksheet(
            "<cols><col min=\"1\" max=\"1\" width=\"7\" customWidth=\"1\"/>\
             <col min=\"2\" max=\"3\" width=\"9\"/>\
             <col min=\"4\" max=\"4\" hidden=\"1\"/>\
             <col min=\"6\" max=\"6\" width=\"4\"/>\
             <col min=\"9\" max=\"9\" width=\"12.25\" customWidth=\"1\" hidden=\"1\"/></cols><sheetData/>"
        )
    );
}

#[test]
fn cols_is_created_before_sheet_data_in_the_template() {
    let got = new_sheet(|p| p.set_column_width(SHEET, 2, Some(20.5)).expect("recorded"));
    assert_eq!(
        got,
        worksheet(
            "<cols><col min=\"2\" max=\"2\" width=\"20.5\" customWidth=\"1\"/></cols><sheetData/>"
        )
    );
}

#[test]
fn cols_is_dropped_when_its_last_child_goes() {
    let body = "<sheetFormatPr defaultRowHeight=\"15\"/><cols><col min=\"3\" max=\"3\" hidden=\"1\"/></cols><sheetData/>";
    assert_eq!(
        edited(body, |p| p
            .set_column_hidden(SHEET, 3, false)
            .expect("recorded")),
        worksheet("<sheetFormatPr defaultRowHeight=\"15\"/><sheetData/>")
    );
}

#[test]
fn cols_out_of_order_overlapping_malformed_or_doubled_are_refused() {
    let edit = |p: &mut Package<Cursor<Vec<u8>>>| {
        p.set_column_width(SHEET, 2, Some(20.5)).expect("recorded")
    };
    for cols in [
        "<col min=\"4\" max=\"5\"/><col min=\"1\" max=\"2\"/>",
        "<col min=\"1\" max=\"3\"/><col min=\"3\" max=\"5\"/>",
        "<col max=\"5\"/>",
        "<col min=\"1\"/>",
        "<col min=\"x\" max=\"5\"/>",
        "<col min=\"01\" max=\"5\"/>",
        "<col min=\"0\" max=\"5\"/>",
        "<col min=\"5\" max=\"4\"/>",
        "<col min=\"1\" max=\"4294967296\"/>",
    ] {
        let error = refused(&format!("<cols>{cols}</cols><sheetData/>"), edit);
        assert_package(
            error,
            "a worksheet whose cols are out of order or overlap cannot be edited",
        );
    }
    let error = refused(
        "<cols><col min=\"1\" max=\"1\"/></cols><cols><col min=\"3\" max=\"3\"/></cols><sheetData/>",
        edit,
    );
    assert_package(
        error,
        "a worksheet has two cols, sheetViews or mergeCells elements, or a sheetViews without a sheetView",
    );
    // A cols the write does not edit is copied raw, however it is written.
    let body = "<cols><col min=\"4\" max=\"5\"/><col min=\"1\" max=\"2\"/></cols><sheetData/>";
    assert_eq!(
        edited(body, |p| p
            .set_row_hidden(SHEET, 1, true)
            .expect("recorded")),
        worksheet(
            "<cols><col min=\"4\" max=\"5\"/><col min=\"1\" max=\"2\"/></cols><sheetData><row r=\"1\" hidden=\"1\"/></sheetData>"
        )
    );
}

#[test]
fn an_element_created_and_then_met_in_the_input_is_refused() {
    // P1 ruling 2: mergeCells written after pageMargins is out of schema
    // order, so the pass creates one before pageMargins and then meets it.
    let error = refused(
        "<sheetData/><pageMargins left=\"1\"/><mergeCells count=\"1\"><mergeCell ref=\"A1:B1\"/></mergeCells>",
        |p| p.merge_cells(SHEET, area("D1:E1")).expect("recorded"),
    );
    assert_package(
        error,
        "a worksheet has two cols, sheetViews or mergeCells elements, or a sheetViews without a sheetView",
    );
}

// --- rows ---

#[test]
fn a_row_is_retagged_with_its_height_and_hidden_flag() {
    let rows = "<row r=\"2\" spans=\"1:2\" customHeight=\"0\"><c r=\"A2\"><v>1</v></c></row>";
    let body = format!("<sheetData>{rows}</sheetData>");
    assert_eq!(
        edited(&body, |p| p
            .set_row_height(SHEET, 2, Some(30.0))
            .expect("recorded")),
        worksheet(
            "<sheetData><row r=\"2\" spans=\"1:2\" customHeight=\"1\" ht=\"30\"><c r=\"A2\"><v>1</v></c></row></sheetData>"
        )
    );
    let tall = "<sheetData><row r=\"2\" ht=\"30\" customHeight=\"1\" hidden=\"1\"><c r=\"A2\"><v>1</v></c></row></sheetData>";
    assert_eq!(
        edited(tall, |p| {
            p.set_row_height(SHEET, 2, None).expect("recorded");
            p.set_row_hidden(SHEET, 2, false).expect("recorded");
        }),
        worksheet("<sheetData><row r=\"2\"><c r=\"A2\"><v>1</v></c></row></sheetData>")
    );
    assert_eq!(
        edited(&body, |p| p
            .set_row_hidden(SHEET, 2, true)
            .expect("recorded")),
        worksheet(
            "<sheetData><row r=\"2\" spans=\"1:2\" customHeight=\"0\" hidden=\"1\"><c r=\"A2\"><v>1</v></c></row></sheetData>"
        )
    );
    // An empty row element is retagged in place.
    assert_eq!(
        edited("<sheetData><row r=\"4\"/></sheetData>", |p| {
            p.set_row_hidden(SHEET, 4, true).expect("recorded")
        }),
        worksheet("<sheetData><row r=\"4\" hidden=\"1\"/></sheetData>")
    );
}

#[test]
fn a_row_is_created_in_order_and_once_when_a_cell_fills_it() {
    let body = "<sheetData><row r=\"1\"><c r=\"A1\"><v>1</v></c></row><row r=\"5\"><c r=\"A5\"><v>5</v></c></row></sheetData>";
    assert_eq!(
        edited(body, |p| {
            p.set_row_height(SHEET, 3, Some(30.0)).expect("recorded");
            p.set_row_hidden(SHEET, 3, true).expect("recorded");
            p.set_row_hidden(SHEET, 7, true).expect("recorded");
        }),
        worksheet(
            "<sheetData><row r=\"1\"><c r=\"A1\"><v>1</v></c></row><row r=\"3\" ht=\"30\" customHeight=\"1\" hidden=\"1\"/><row r=\"5\"><c r=\"A5\"><v>5</v></c></row><row r=\"7\" hidden=\"1\"/></sheetData>"
        )
    );
    assert_eq!(
        edited(body, |p| {
            p.set_row_height(SHEET, 3, Some(30.0)).expect("recorded");
            p.set_cell(SHEET, at("B3"), Value::Number(2.0));
        }),
        worksheet(
            "<sheetData><row r=\"1\"><c r=\"A1\"><v>1</v></c></row><row r=\"3\" ht=\"30\" customHeight=\"1\"><c r=\"B3\"><v>2</v></c></row><row r=\"5\"><c r=\"A5\"><v>5</v></c></row></sheetData>"
        )
    );
    // A format that only removes writes no row.
    assert_eq!(
        edited(body, |p| p
            .set_row_height(SHEET, 3, None)
            .expect("recorded")),
        worksheet(body)
    );
    // An empty sheetData is opened for a created row.
    assert_eq!(
        edited("<sheetData/>", |p| p
            .set_row_height(SHEET, 2, Some(0.75))
            .expect("recorded")),
        worksheet("<sheetData><row r=\"2\" ht=\"0.75\" customHeight=\"1\"/></sheetData>")
    );
}

#[test]
fn a_row_format_on_a_part_without_sheet_data_is_refused() {
    let error = refused("<dimension ref=\"A1\"/>", |p| {
        p.set_row_height(SHEET, 2, Some(30.0)).expect("recorded")
    });
    assert_package(error, "a worksheet without sheetData cannot be edited");
}

#[test]
fn widths_and_heights_are_written_as_numbers_are() {
    for (size, text) in [
        (20.5, "20.5"),
        (30.0, "30"),
        (0.0, "0"),
        (255.0, "255"),
        (8.43, "8.43"),
    ] {
        let got = edited("<sheetData/>", |p| {
            p.set_column_width(SHEET, 1, Some(size)).expect("recorded")
        });
        assert_eq!(
            got,
            worksheet(&format!(
                "<cols><col min=\"1\" max=\"1\" width=\"{text}\" customWidth=\"1\"/></cols><sheetData/>"
            ))
        );
    }
    let got = edited("<sheetData/>", |p| {
        p.set_row_height(SHEET, 1, Some(409.0)).expect("recorded")
    });
    assert_eq!(
        got,
        worksheet("<sheetData><row r=\"1\" ht=\"409\" customHeight=\"1\"/></sheetData>")
    );
}

// --- panes ---

const PANE_B2: &str = "<pane xSplit=\"1\" ySplit=\"1\" topLeftCell=\"B2\" activePane=\"bottomRight\" state=\"frozen\"/><selection pane=\"bottomRight\"/>";

#[test]
fn a_freeze_writes_the_three_pane_forms() {
    let body = "<sheetViews><sheetView tabSelected=\"1\" workbookViewId=\"0\"><selection activeCell=\"C3\" sqref=\"C3\"/></sheetView></sheetViews><sheetData/>";
    let cases = [
        ("B2", PANE_B2.to_string()),
        (
            "B1",
            "<pane xSplit=\"1\" topLeftCell=\"B1\" activePane=\"topRight\" state=\"frozen\"/><selection pane=\"topRight\"/>".to_string(),
        ),
        (
            "A2",
            "<pane ySplit=\"1\" topLeftCell=\"A2\" activePane=\"bottomLeft\" state=\"frozen\"/><selection pane=\"bottomLeft\"/>".to_string(),
        ),
    ];
    for (cell, pane) in cases {
        let got = edited(body, |p| {
            p.freeze_panes(SHEET, Some(at(cell))).expect("recorded")
        });
        assert_eq!(
            got,
            worksheet(&format!(
                "<sheetViews><sheetView tabSelected=\"1\" workbookViewId=\"0\">{pane}</sheetView></sheetViews><sheetData/>"
            )),
            "{cell}"
        );
    }
}

#[test]
fn an_unfreeze_removes_the_pane_and_selections_and_keeps_the_rest() {
    let body = "<sheetViews><sheetView workbookViewId=\"0\"><pane ySplit=\"1\" topLeftCell=\"A2\" activePane=\"bottomLeft\" state=\"frozen\"/><selection pane=\"bottomLeft\" activeCell=\"A5\" sqref=\"A5\"/><extLst><ext uri=\"x\"/></extLst></sheetView><sheetView workbookViewId=\"1\"><pane ySplit=\"2\"/></sheetView></sheetViews><sheetData/>";
    assert_eq!(
        edited(body, |p| p.freeze_panes(SHEET, None).expect("recorded")),
        worksheet(
            "<sheetViews><sheetView workbookViewId=\"0\"><extLst><ext uri=\"x\"/></extLst></sheetView><sheetView workbookViewId=\"1\"><pane ySplit=\"2\"/></sheetView></sheetViews><sheetData/>"
        )
    );
    // A freeze puts the pane before the kept extLst.
    assert_eq!(
        edited(body, |p| p
            .freeze_panes(SHEET, Some(at("B2")))
            .expect("recorded")),
        worksheet(&format!(
            "<sheetViews><sheetView workbookViewId=\"0\">{PANE_B2}<extLst><ext uri=\"x\"/></extLst></sheetView><sheetView workbookViewId=\"1\"><pane ySplit=\"2\"/></sheetView></sheetViews><sheetData/>"
        ))
    );
    // An empty sheetView is opened up for a freeze.
    assert_eq!(
        edited(
            "<sheetViews><sheetView workbookViewId=\"0\"/></sheetViews><sheetData/>",
            |p| { p.freeze_panes(SHEET, Some(at("B2"))).expect("recorded") }
        ),
        worksheet(&format!(
            "<sheetViews><sheetView workbookViewId=\"0\">{PANE_B2}</sheetView></sheetViews><sheetData/>"
        ))
    );
}

#[test]
fn sheet_views_is_created_in_the_template() {
    let got = new_sheet(|p| p.freeze_panes(SHEET, Some(at("B2"))).expect("recorded"));
    assert_eq!(
        got,
        worksheet(&format!(
            "<sheetViews><sheetView workbookViewId=\"0\">{PANE_B2}</sheetView></sheetViews><sheetData/>"
        ))
    );
    // An unfreeze creates nothing.
    let got = new_sheet(|p| p.freeze_panes(SHEET, None).expect("recorded"));
    assert_eq!(got, worksheet("<sheetData/>"));
}

#[test]
fn sheet_views_without_a_sheet_view_or_doubled_is_refused() {
    let message = "a worksheet has two cols, sheetViews or mergeCells elements, or a sheetViews without a sheetView";
    for at_cell in [Some(at("B2")), None] {
        for body in [
            "<sheetViews/><sheetData/>",
            "<sheetViews></sheetViews><sheetData/>",
            "<sheetViews><sheetView workbookViewId=\"0\"/></sheetViews><sheetViews><sheetView workbookViewId=\"0\"/></sheetViews><sheetData/>",
        ] {
            let error = refused(body, |p| p.freeze_panes(SHEET, at_cell).expect("recorded"));
            assert_package(error, message);
        }
    }
}

// --- merges ---

#[test]
fn a_merge_is_appended_and_counted() {
    let body = "<sheetData/><mergeCells count=\"1\"><mergeCell ref=\"A1:B1\"/></mergeCells><pageMargins left=\"1\"/>";
    assert_eq!(
        edited(body, |p| p
            .merge_cells(SHEET, area("D4:C3"))
            .expect("recorded")),
        worksheet(
            "<sheetData/><mergeCells count=\"2\"><mergeCell ref=\"A1:B1\"/><mergeCell ref=\"C3:D4\"/></mergeCells><pageMargins left=\"1\"/>"
        )
    );
}

#[test]
fn merge_cells_is_created_in_schema_position() {
    for (after, before) in [
        ("", "<phoneticPr fontId=\"1\"/>"),
        ("", "<conditionalFormatting sqref=\"A1\"/>"),
        ("<autoFilter ref=\"A1:B2\"/>", "<pageMargins left=\"1\"/>"),
        ("<sortState ref=\"A1:B2\"/><x:other xmlns:x=\"urn:x\"/>", ""),
    ] {
        let body = format!("<sheetData/>{after}{before}");
        let got = edited(&body, |p| {
            p.merge_cells(SHEET, area("A1:B2")).expect("recorded")
        });
        assert_eq!(
            got,
            worksheet(&format!(
                "<sheetData/>{after}<mergeCells count=\"1\"><mergeCell ref=\"A1:B2\"/></mergeCells>{before}"
            )),
            "{body}"
        );
    }
}

#[test]
fn a_merge_clears_the_covered_cells_and_keeps_their_style() {
    let rows = "<row r=\"1\"><c r=\"A1\"><v>1</v></c><c r=\"B1\" s=\"2\"><v>2</v></c><c r=\"C1\"><v>3</v></c></row>\
                <row r=\"2\"><c r=\"A2\" t=\"s\"><v>0</v></c><c r=\"B2\" s=\"4\"/><c r=\"D2\"><v>9</v></c></row>";
    let got = edited(&format!("<sheetData>{rows}</sheetData>"), |p| {
        p.merge_cells(SHEET, area("A1:B2")).expect("recorded")
    });
    assert_eq!(
        got,
        worksheet(
            "<sheetData><row r=\"1\"><c r=\"A1\"><v>1</v></c><c r=\"B1\" s=\"2\"/><c r=\"C1\"><v>3</v></c></row>\
                   <row r=\"2\"><c r=\"B2\" s=\"4\"/><c r=\"D2\"><v>9</v></c></row></sheetData>\
                   <mergeCells count=\"1\"><mergeCell ref=\"A1:B2\"/></mergeCells>"
        )
    );
}

#[test]
fn a_formula_in_a_covered_cell_is_removed_and_the_chain_rule_fires() {
    let mut parts = with_a_calc_chain();
    set(
        &mut parts,
        SHEET,
        sheet_with(
            "<row r=\"1\"><c r=\"A1\"><v>2</v></c><c r=\"B1\" s=\"1\"><f>A1*2</f><v>4</v></c></row>",
        ),
    );
    let output = edited_output(parts, |p| {
        p.merge_cells(SHEET, area("A1:B1")).expect("recorded")
    });
    assert_chain_gone(&output);
    assert_part(&output, WORKBOOK, &flagged_workbook());
    assert_part(
        &output,
        SHEET,
        &worksheet(
            "<sheetData><row r=\"1\"><c r=\"A1\"><v>2</v></c><c r=\"B1\" s=\"1\"/></row></sheetData><mergeCells count=\"1\"><mergeCell ref=\"A1:B1\"/></mergeCells>",
        ),
    );
}

#[test]
fn a_covered_cell_in_an_array_or_with_an_unmodelled_attribute_is_refused() {
    let array = "<sheetData><row r=\"1\"><c r=\"A1\"><f t=\"array\" ref=\"A1:A2\">1</f><v>1</v></c></row><row r=\"2\"><c r=\"A2\"><v>1</v></c></row></sheetData>";
    let error = refused(array, |p| {
        p.merge_cells(SHEET, area("A1:A2")).expect("recorded")
    });
    assert_package(
        error,
        "a cell in a shared, array or data table formula cannot be edited",
    );
    let shared = "<sheetData><row r=\"1\"><c r=\"A1\"><v>1</v></c><c r=\"B1\"><f t=\"shared\" si=\"0\" ref=\"B1:B2\">A1</f><v>1</v></c></row></sheetData>";
    let error = refused(shared, |p| {
        p.merge_cells(SHEET, area("A1:B1")).expect("recorded")
    });
    assert_package(
        error,
        "a cell in a shared, array or data table formula cannot be edited",
    );
    let odd = "<sheetData><row r=\"1\"><c r=\"A1\"><v>1</v></c><c r=\"B1\" cm=\"1\"><v>1</v></c></row></sheetData>";
    let error = refused(odd, |p| {
        p.merge_cells(SHEET, area("A1:B1")).expect("recorded")
    });
    assert_package(
        error,
        "a cell carries an attribute this layer does not model",
    );
}

#[test]
fn a_cell_edit_inside_a_pending_merge_is_refused_and_the_top_left_is_not() {
    let body = "<sheetData/>";
    for edit in [
        |p: &mut Package<Cursor<Vec<u8>>>| p.set_cell(SHEET, at("B2"), Value::Number(1.0)),
        |p: &mut Package<Cursor<Vec<u8>>>| p.clear_cell(SHEET, at("A2")),
        |p: &mut Package<Cursor<Vec<u8>>>| p.set_cell_style(SHEET, at("B1"), None),
    ] {
        let error = refused(body, |p| {
            edit(p);
            p.merge_cells(SHEET, area("A1:B2")).expect("recorded");
        });
        assert_package(error, "a cell edit inside a merge this write records");
    }
    let got = edited(body, |p| {
        p.merge_cells(SHEET, area("A1:B2")).expect("recorded");
        p.set_cell(SHEET, at("A1"), Value::Number(1.0));
    });
    assert_eq!(
        got,
        worksheet(
            "<sheetData><row r=\"1\"><c r=\"A1\"><v>1</v></c></row></sheetData><mergeCells count=\"1\"><mergeCell ref=\"A1:B2\"/></mergeCells>"
        )
    );
}

#[test]
fn a_merge_overlapping_a_merge_is_refused() {
    let message = "a merge overlaps a merge the worksheet has, or a mergeCell ref does not parse";
    let body = "<sheetData/><mergeCells count=\"1\"><mergeCell ref=\"B2:C3\"/></mergeCells>";
    let error = refused(body, |p| {
        p.merge_cells(SHEET, area("C3:D4")).expect("recorded")
    });
    assert_package(error, message);
    let error = refused("<sheetData/>", |p| {
        p.merge_cells(SHEET, area("A1:B2")).expect("recorded");
        p.merge_cells(SHEET, area("B1:C1")).expect("recorded");
    });
    assert_package(error, message);
    // A mergeCell ref that does not parse (P1 ruling 4).
    for bad in ["A1:", "B2:C", "", "A0:B1"] {
        let body =
            format!("<sheetData/><mergeCells count=\"1\"><mergeCell ref=\"{bad}\"/></mergeCells>");
        let error = refused(&body, |p| {
            p.merge_cells(SHEET, area("E5:F6")).expect("recorded")
        });
        assert_package(error, message);
    }
    // A merge overlapping a merge this write unmerges is allowed (P1 ruling
    // 5).
    assert_eq!(
        edited(body, |p| {
            p.unmerge_cells(SHEET, area("B2:C3")).expect("recorded");
            p.merge_cells(SHEET, area("C3:D4")).expect("recorded");
        }),
        worksheet("<sheetData/><mergeCells count=\"1\"><mergeCell ref=\"C3:D4\"/></mergeCells>")
    );
}

#[test]
fn a_merge_of_one_cell_or_off_the_grid_is_refused_at_the_call() {
    let (_, mut package) = opened(smallest());
    assert_package(
        package
            .merge_cells(SHEET, area("C3"))
            .expect_err("one cell"),
        "a merge needs more than one cell",
    );
    let off = Area {
        first: at("A1"),
        last: CellRef {
            row: 1_048_577,
            col: 1,
        },
    };
    assert_package(
        package.merge_cells(SHEET, off).expect_err("off the grid"),
        "a row or column index or count is out of range",
    );
    assert_package(
        package.unmerge_cells(SHEET, off).expect_err("off the grid"),
        "a row or column index or count is out of range",
    );
    let zero = Area {
        first: CellRef { row: 0, col: 1 },
        last: at("B2"),
    };
    assert_package(
        package.merge_cells(SHEET, zero).expect_err("row 0"),
        "a row or column index or count is out of range",
    );
    // Nothing was recorded: the write is the input's bytes.
    let out = written(package);
    assert_untouched(&out, &smallest(), &[]);
}

#[test]
fn an_unnormalised_area_is_normalised_at_the_call() {
    let reversed = Area {
        first: at("D4"),
        last: at("C3"),
    };
    assert_eq!(
        edited("<sheetData/>", |p| p
            .merge_cells(SHEET, reversed)
            .expect("recorded")),
        worksheet("<sheetData/><mergeCells count=\"1\"><mergeCell ref=\"C3:D4\"/></mergeCells>")
    );
    let body = "<sheetData/><mergeCells count=\"1\"><mergeCell ref=\"C3:D4\"/></mergeCells>";
    assert_eq!(
        edited(body, |p| p
            .unmerge_cells(SHEET, reversed)
            .expect("recorded")),
        worksheet("<sheetData/>")
    );
}

#[test]
fn an_unmerge_removes_its_merge_refuses_an_absent_one_and_drops_the_last() {
    let body = "<sheetData/><mergeCells count=\"2\"><mergeCell ref=\"A1:B1\"/><mergeCell ref=\"C3:D4\"/></mergeCells><pageMargins left=\"1\"/>";
    assert_eq!(
        edited(body, |p| p
            .unmerge_cells(SHEET, area("A1:B1"))
            .expect("recorded")),
        worksheet(
            "<sheetData/><mergeCells count=\"1\"><mergeCell ref=\"C3:D4\"/></mergeCells><pageMargins left=\"1\"/>"
        )
    );
    // Two unmerges of one area remove it once; the last merge going drops
    // the container.
    assert_eq!(
        edited(body, |p| {
            p.unmerge_cells(SHEET, area("A1:B1")).expect("recorded");
            p.unmerge_cells(SHEET, area("A1:B1")).expect("recorded");
            p.unmerge_cells(SHEET, area("C3:D4")).expect("recorded");
        }),
        worksheet("<sheetData/><pageMargins left=\"1\"/>")
    );
    let message = "no merge of that range";
    for unmerge in ["A1:B2", "C3:D3"] {
        let error = refused(body, |p| {
            p.unmerge_cells(SHEET, area(unmerge)).expect("recorded")
        });
        assert_package(error, message);
    }
    // No mergeCells at all, and a pending merge unmerged (P1 ruling 5).
    let error = refused("<sheetData/>", |p| {
        p.unmerge_cells(SHEET, area("A1:B1")).expect("recorded")
    });
    assert_package(error, message);
    let error = refused("<sheetData/>", |p| {
        p.merge_cells(SHEET, area("A1:B1")).expect("recorded");
        p.unmerge_cells(SHEET, area("A1:B1")).expect("recorded");
    });
    assert_package(error, message);
    // A second mergeCells is refused.
    let error = refused(
        "<sheetData/><mergeCells count=\"1\"><mergeCell ref=\"A1:B1\"/></mergeCells><mergeCells count=\"1\"><mergeCell ref=\"C1:D1\"/></mergeCells>",
        |p| p.unmerge_cells(SHEET, area("A1:B1")).expect("recorded"),
    );
    assert_package(
        error,
        "a worksheet has two cols, sheetViews or mergeCells elements, or a sheetViews without a sheetView",
    );
}

// --- the calls' own refusals ---

#[test]
fn indices_sizes_and_a_freeze_at_a1_are_refused_at_the_call() {
    let index = "a row or column index or count is out of range";
    let size = "a column width or a row height is out of range";
    let (_, mut package) = opened(smallest());
    for column in [0, 16_385] {
        assert_package(
            package
                .set_column_width(SHEET, column, Some(1.0))
                .expect_err("column"),
            index,
        );
        assert_package(
            package
                .set_column_hidden(SHEET, column, true)
                .expect_err("column"),
            index,
        );
    }
    for row in [0, 1_048_577] {
        assert_package(
            package
                .set_row_height(SHEET, row, Some(1.0))
                .expect_err("row"),
            index,
        );
        assert_package(
            package.set_row_hidden(SHEET, row, true).expect_err("row"),
            index,
        );
    }
    for width in [-0.5, 255.5, f64::NAN, f64::INFINITY] {
        assert_package(
            package
                .set_column_width(SHEET, 1, Some(width))
                .expect_err("width"),
            size,
        );
    }
    for height in [-1.0, 409.25, f64::NAN, f64::NEG_INFINITY] {
        assert_package(
            package
                .set_row_height(SHEET, 1, Some(height))
                .expect_err("height"),
            size,
        );
    }
    assert_package(
        package.freeze_panes(SHEET, Some(at("A1"))).expect_err("A1"),
        "a freeze at A1 freezes nothing",
    );
    assert_package(
        package
            .freeze_panes(
                SHEET,
                Some(CellRef {
                    row: 2,
                    col: 16_385,
                }),
            )
            .expect_err("off the grid"),
        index,
    );
    // Nothing refused was recorded: the write is the input's bytes
    // (specs/0018 P1 verification ruling 6, F7).
    let out = written(package);
    assert_untouched(&out, &smallest(), &[]);
    // The limits themselves are accepted.
    let (_, mut package) = opened(smallest());
    package
        .set_column_width(SHEET, 16_384, Some(255.0))
        .expect("the last column");
    package
        .set_row_height(SHEET, 1_048_576, Some(409.0))
        .expect("the last row");
    package
        .freeze_panes(
            SHEET,
            Some(CellRef {
                row: 1_048_576,
                col: 16_384,
            }),
        )
        .expect("the last cell");
}

#[test]
fn an_edit_on_a_removed_or_replaced_part_is_refused() {
    type Edit = fn(&mut Package<Cursor<Vec<u8>>>);
    let edits: [Edit; 5] = [
        |p| p.set_column_width(SHEET, 1, Some(1.0)).expect("recorded"),
        |p| p.set_row_hidden(SHEET, 1, true).expect("recorded"),
        |p| p.freeze_panes(SHEET, Some(at("B2"))).expect("recorded"),
        |p| p.merge_cells(SHEET, area("A1:B1")).expect("recorded"),
        |p| p.unmerge_cells(SHEET, area("A1:B1")).expect("recorded"),
    ];
    for edit in edits {
        let (_, mut package) = opened(smallest());
        edit(&mut package);
        package.remove_part(SHEET);
        assert_package(
            refusal(package).expect("refused"),
            "an edit to a removed part",
        );
        let (_, mut package) = opened(smallest());
        edit(&mut package);
        package.replace_part(SHEET, replacement_from_reader(Cursor::new(b"x".to_vec())));
        assert_package(
            refusal(package).expect("refused"),
            "a cell edit to a part this write replaces",
        );
    }
}

#[test]
fn an_area_cell_count_saturates_rather_than_overflowing() {
    // specs/0018 P1 verification ruling 6 (F6): a hand-built area can make
    // each side 2^32, whose product u64 cannot hold.
    let wide = Area {
        first: CellRef { row: 0, col: 0 },
        last: CellRef {
            row: u32::MAX,
            col: u32::MAX,
        },
    };
    assert_eq!(wide.cells(), u64::MAX);
    let grid = Area {
        first: at("A1"),
        last: CellRef {
            row: 1_048_576,
            col: 16_384,
        },
    };
    assert_eq!(grid.cells(), 1_048_576 * 16_384);
}

#[test]
fn a_pending_sheet_removed_drops_its_format_pane_and_merge_edits() {
    // specs/0018 P1 verification ruling 6 (F5): the edits recorded on a
    // sheet added and removed again go with it, so the write is the input.
    let (parts, mut package) = opened(smallest());
    let part = package.add_sheet("Two").expect("add");
    package
        .set_column_width(&part, 2, Some(9.0))
        .expect("recorded");
    package.set_column_hidden(&part, 3, true).expect("recorded");
    package
        .set_row_height(&part, 2, Some(30.0))
        .expect("recorded");
    package.set_row_hidden(&part, 3, true).expect("recorded");
    package
        .freeze_panes(&part, Some(at("B2")))
        .expect("recorded");
    package.merge_cells(&part, area("A1:B1")).expect("recorded");
    package
        .unmerge_cells(&part, area("C1:D1"))
        .expect("recorded");
    package.remove_sheet("Two").expect("remove");
    let out = written(package);
    assert_names(&out, &part_names(&parts));
    assert_untouched(&out, &parts, &[]);
}
