//! Rows inserted and deleted on byte-built worksheets
//! (`specs/0018-rows-and-columns.md`, "P2", with the P2 rulings): each test
//! builds the smallest package with its worksheet holding a given body,
//! records one insertion or deletion, and compares the written part with the
//! bytes the rule gives, every other part asserted untouched.

use super::*;

/// The worksheet part after `edit` on the smallest package whose worksheet
/// holds `body`, as text; every other part is asserted untouched.
#[track_caller]
fn shifted(body: &str, edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> String {
    let mut parts = smallest();
    set(&mut parts, SHEET, worksheet(body));
    let output = edited_output(parts.clone(), edit);
    assert_untouched(&output, &parts, &[SHEET]);
    String::from_utf8(stored_data(&output, SHEET)).expect("the part is UTF-8")
}

/// The refusal a write of `edit` gives on the package `parts`, with no
/// output.
#[track_caller]
fn refused_on(parts: Parts, edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> Error {
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

/// The smallest package whose worksheet holds `body`.
fn with_body(body: &str) -> Parts {
    let mut parts = smallest();
    set(&mut parts, SHEET, worksheet(body));
    parts
}

/// An edit that records one shift, boxed so insertions and deletions share
/// a type in a table of cases.
type ShiftEdit = Box<dyn FnOnce(&mut Package<Cursor<Vec<u8>>>)>;

fn insert(at: u32, count: u32) -> ShiftEdit {
    Box::new(move |p| p.insert_rows(SHEET, at, count).expect("recorded"))
}

fn delete(at: u32, count: u32) -> ShiftEdit {
    Box::new(move |p| p.delete_rows(SHEET, at, count).expect("recorded"))
}

const CUTS: &str = "an insertion or deletion cuts a shared, array or data table formula";
const UNPARSED: &str = "a range an insertion or deletion would move does not parse";

// --- the rule's arithmetic ---

#[test]
fn the_rule_moves_indices_and_intervals_as_the_table_gives() {
    // One row per case of "The shift rule", printed for test.log
    // (`specs/0018-rows-and-columns.md`, "P2").
    let ins = Shift {
        insert: true,
        at: 5,
        count: 2,
        limit: 20,
    };
    let del = Shift {
        insert: false,
        at: 5,
        count: 2,
        limit: 20,
    };
    let indices: [(Shift, u32, Option<u64>); 7] = [
        (ins, 4, Some(4)),
        (ins, 5, Some(7)),
        (ins, 9, Some(11)),
        (del, 4, Some(4)),
        (del, 5, None),
        (del, 6, None),
        (del, 7, Some(5)),
    ];
    for (shift, i, want) in indices {
        let got = shift.index(i);
        println!(
            "shift index insert={} at=5 count=2 i={i} -> {:?}",
            shift.insert, got
        );
        assert_eq!(got, want, "index {i} under {shift:?}");
    }
    let intervals: [(Shift, u32, u32, Moved); 11] = [
        (ins, 1, 4, Moved::Whole(1, 4)),
        (ins, 5, 6, Moved::Whole(7, 8)),
        (ins, 3, 5, Moved::Changed(3, 7)),
        (ins, 3, 9, Moved::Changed(3, 11)),
        (del, 1, 4, Moved::Whole(1, 4)),
        (del, 7, 9, Moved::Whole(5, 7)),
        (del, 5, 6, Moved::Gone),
        (del, 3, 5, Moved::Changed(3, 4)),
        (del, 6, 9, Moved::Changed(5, 7)),
        (del, 3, 9, Moved::Changed(3, 7)),
        (del, 5, 9, Moved::Changed(5, 7)),
    ];
    for (shift, a, b, want) in intervals {
        let got = shift.interval(a, b);
        println!(
            "shift interval insert={} at=5 count=2 [{a},{b}] -> {:?}",
            shift.insert, got
        );
        assert_eq!(got, want, "interval [{a}, {b}] under {shift:?}");
    }
    // The clip: an interval pushed past the grid's end is cut to it, one
    // wholly past it is gone.
    let near = Shift {
        insert: true,
        at: 5,
        count: 2,
        limit: 10,
    };
    let clipped = near.area_clipped(Area::parse(b"A4:B9").expect("area"));
    println!("shift clip limit=10 A4:B9 -> {:?}", clipped.map(|a| a.a1()));
    assert_eq!(clipped.map(|a| a.a1()), Some("A4:B10".to_string()));
    let past = near.area_clipped(Area::parse(b"A9:B10").expect("area"));
    println!("shift clip limit=10 A9:B10 -> {:?}", past.map(|a| a.a1()));
    assert_eq!(past, None);
}

// --- rows and cells ---

const ROWS: &str = "<sheetData>\
    <row r=\"1\" spans=\"1:2\"><c r=\"A1\"><v>1</v></c></row>\
    <row r=\"2\" ht=\"20\" customHeight=\"1\"><c r=\"A2\" s=\"3\" t=\"s\"><v>0</v></c><c r=\"B2\"/></row>\
    <row r=\"3\"/>\
    <row r=\"4\"><c r=\"C4\"><v>4</v></c></row>\
    </sheetData>";

#[test]
fn rows_before_at_and_after_an_insertion() {
    assert_eq!(
        shifted(ROWS, insert(2, 3)),
        worksheet(
            "<sheetData>\
             <row r=\"1\" spans=\"1:2\"><c r=\"A1\"><v>1</v></c></row>\
             <row r=\"5\" ht=\"20\" customHeight=\"1\"><c r=\"A5\" s=\"3\" t=\"s\"><v>0</v></c><c r=\"B5\"/></row>\
             <row r=\"6\"/>\
             <row r=\"7\"><c r=\"C7\"><v>4</v></c></row>\
             </sheetData>"
        )
    );
}

#[test]
fn a_deleted_row_goes_whole_and_the_rest_move_up() {
    assert_eq!(
        shifted(ROWS, delete(2, 2)),
        worksheet(
            "<sheetData>\
             <row r=\"1\" spans=\"1:2\"><c r=\"A1\"><v>1</v></c></row>\
             <row r=\"2\"><c r=\"C2\"><v>4</v></c></row>\
             </sheetData>"
        )
    );
    // A deletion past every row changes no byte.
    assert_eq!(shifted(ROWS, delete(9, 1)), worksheet(ROWS));
}

#[test]
fn a_row_moved_off_the_grid_is_refused() {
    let parts =
        with_body("<sheetData><row r=\"1048576\"><c r=\"A1048576\"><v>1</v></c></row></sheetData>");
    assert_package(
        refused_on(parts, insert(1, 1)),
        "a row or column would be moved off the worksheet",
    );
}

#[test]
fn the_order_rule_holds_on_the_input() {
    let parts = with_body("<sheetData><row r=\"3\"/><row r=\"2\"/></sheetData>");
    assert_package(
        refused_on(parts, insert(1, 1)),
        "a worksheet whose rows or cells are out of order cannot be edited",
    );
    let parts = with_body("<sheetData><row><c r=\"A1\"/></row></sheetData>");
    assert_package(
        refused_on(parts, insert(1, 1)),
        "a worksheet without explicit references cannot be edited",
    );
}

// --- formulas ---

const SHARED: &str = "<sheetData>\
    <row r=\"2\"><c r=\"A2\"><f t=\"shared\" ref=\"A2:A4\" si=\"0\">B2</f><v>1</v></c></row>\
    <row r=\"3\"><c r=\"A3\"><f t=\"shared\" si=\"0\"/><v>1</v></c></row>\
    <row r=\"4\"><c r=\"A4\"><f t=\"shared\" si=\"0\"/><v>1</v></c></row>\
    </sheetData>";

#[test]
fn a_shared_formula_before_after_straddling_inside_and_deleted_whole() {
    // Before the point: untouched, and no formula moves.
    let parts = with_body(SHARED);
    let out = edited_output(parts.clone(), insert(5, 1));
    assert_untouched(&out, &parts, &[]);
    // After the point: its ref moves with its cells, text, si untouched.
    let got = String::from_utf8(stored_data(
        &edited_output(parts.clone(), insert(1, 2)),
        SHEET,
    ))
    .expect("UTF-8");
    assert_eq!(
        got,
        worksheet(
            "<sheetData>\
             <row r=\"4\"><c r=\"A4\"><f t=\"shared\" ref=\"A4:A6\" si=\"0\">B2</f><v>1</v></c></row>\
             <row r=\"5\"><c r=\"A5\"><f t=\"shared\" si=\"0\"/><v>1</v></c></row>\
             <row r=\"6\"><c r=\"A6\"><f t=\"shared\" si=\"0\"/><v>1</v></c></row>\
             </sheetData>"
        )
    );
    // Straddling an insertion, and a deletion inside it: refused.
    assert_package(refused_on(parts.clone(), insert(3, 1)), CUTS);
    assert_package(refused_on(parts.clone(), delete(3, 1)), CUTS);
    // A deletion cutting its first rows: refused.
    assert_package(refused_on(parts.clone(), delete(1, 2)), CUTS);
    // The master deleted with its whole range: allowed (P2 ruling 1).
    let got =
        String::from_utf8(stored_data(&edited_output(parts, delete(2, 3)), SHEET)).expect("UTF-8");
    assert_eq!(got, worksheet("<sheetData></sheetData>"));
}

#[test]
fn an_array_formula_likewise_and_one_whose_ref_does_not_parse() {
    let body = "<sheetData><row r=\"2\"><c r=\"A2\"><f t=\"array\" ref=\"A2:B3\">X</f><v>1</v></c></row></sheetData>";
    let parts = with_body(body);
    let got = String::from_utf8(stored_data(
        &edited_output(parts.clone(), insert(1, 1)),
        SHEET,
    ))
    .expect("UTF-8");
    assert_eq!(
        got,
        worksheet(
            "<sheetData><row r=\"3\"><c r=\"A3\"><f t=\"array\" ref=\"A3:B4\">X</f><v>1</v></c></row></sheetData>"
        )
    );
    assert_package(refused_on(parts.clone(), insert(3, 1)), CUTS);
    let got =
        String::from_utf8(stored_data(&edited_output(parts, delete(2, 2)), SHEET)).expect("UTF-8");
    assert_eq!(got, worksheet("<sheetData></sheetData>"));
    let bad = with_body(
        "<sheetData><row r=\"2\"><c r=\"A2\"><f t=\"array\" ref=\"A2:\">X</f></c></row></sheetData>",
    );
    assert_package(refused_on(bad, insert(1, 1)), UNPARSED);
    // A data table's ref follows the same rule; r1 and r2 are untouched.
    let table = with_body(
        "<sheetData><row r=\"2\"><c r=\"B2\"><f t=\"dataTable\" ref=\"B2:C3\" r1=\"A1\"/><v>1</v></c></row></sheetData>",
    );
    let got =
        String::from_utf8(stored_data(&edited_output(table, insert(2, 1)), SHEET)).expect("UTF-8");
    assert!(
        got.contains("<f t=\"dataTable\" ref=\"B3:C4\" r1=\"A1\"/>"),
        "{got}"
    );
}

#[test]
fn a_ref_pushed_past_the_grid_is_a_grow_and_one_moved_past_it_off_the_grid() {
    // P2 ruling 5.
    let grow = with_body(
        "<sheetData><row r=\"1048570\"><c r=\"A1048570\"><f t=\"array\" ref=\"A1048570:A1048576\">X</f></c></row></sheetData>",
    );
    assert_package(refused_on(grow, insert(1, 3)), CUTS);
    let off = with_body(
        "<sheetData><row r=\"1048575\"><c r=\"A1048575\"><f t=\"array\" ref=\"A1048575:A1048576\">X</f></c></row></sheetData>",
    );
    assert_package(
        refused_on(off, insert(1, 5)),
        "a row or column would be moved off the worksheet",
    );
}

#[test]
fn the_chain_rule_fires_when_a_formula_moves_and_not_otherwise() {
    let rows_after = "<row r=\"1\"><c r=\"A1\"><v>2</v></c></row><row r=\"3\"><c r=\"B3\"><f>A1*2</f><v>4</v></c></row>";
    let mut parts = with_a_calc_chain();
    set(&mut parts, SHEET, sheet_with(rows_after));
    // A formula at or after the point: the chain goes, the flag is set.
    for edit in [insert(2, 1), insert(3, 1)] {
        let out = edited_output(parts.clone(), edit);
        assert_chain_gone(&out);
        assert_part(&out, WORKBOOK, &flagged_workbook());
    }
    let out = edited_output(parts.clone(), delete(3, 1));
    assert_chain_gone(&out);
    // Only cells before the point, or no formula at all: neither.
    let out = edited_output(parts.clone(), insert(4, 1));
    assert_untouched(&out, &parts, &[]);
    let mut plain = with_a_calc_chain();
    set(
        &mut plain,
        SHEET,
        sheet_with("<row r=\"3\"><c r=\"B3\"><v>4</v></c></row>"),
    );
    let out = edited_output(plain.clone(), insert(1, 1));
    assert_untouched(&out, &plain, &[SHEET]);
}

// --- dimension and merges ---

#[test]
fn a_dimension_moved_grown_cut_and_emptied() {
    let body = |d: &str| format!("<dimension ref=\"{d}\"/><sheetData/>");
    for (d, edit, want) in [
        ("A3:C5", insert(2, 2), "A5:C7"),
        ("A1:C5", insert(2, 2), "A1:C7"),
        ("A1:C5", delete(2, 2), "A1:C3"),
        ("A3:C5", delete(3, 3), "A1"),
        ("A1:C1", insert(2, 1), "A1:C1"),
    ] {
        assert_eq!(shifted(&body(d), edit), worksheet(&body(want)), "{d}");
    }
    assert_package(refused_on(with_body(&body("A1:")), insert(1, 1)), UNPARSED);
}

#[test]
fn merges_moved_grown_cut_dropped_and_the_container_dropped() {
    let merges = |list: &[&str]| {
        if list.is_empty() {
            return String::new();
        }
        let inner: String = list
            .iter()
            .map(|r| format!("<mergeCell ref=\"{r}\"/>"))
            .collect();
        format!("<mergeCells count=\"{}\">{inner}</mergeCells>", list.len())
    };
    let body = |list: &[&str]| format!("<sheetData/>{}<pageMargins left=\"1\"/>", merges(list));
    let input = ["A1:B1", "A3:B4", "A2:B5", "C3:C4"];
    assert_eq!(
        shifted(&body(&input), insert(3, 1)),
        worksheet(&body(&["A1:B1", "A4:B5", "A2:B6", "C4:C5"]))
    );
    // A deletion of row 3: A3:B4 cut to A3:B3, A2:B5 to A2:B4, C3:C4 cut
    // to one cell and dropped, count following.
    assert_eq!(
        shifted(&body(&input), delete(3, 1)),
        worksheet(&body(&["A1:B1", "A3:B3", "A2:B4"]))
    );
    // Every merge removed: the container goes.
    assert_eq!(
        shifted(&body(&["A3:B4", "C3:C4"]), delete(3, 2)),
        worksheet(&body(&[]))
    );
    assert_package(
        refused_on(with_body(&body(&["A1:"])), insert(1, 1)),
        UNPARSED,
    );
}

// --- the four lists ---

#[test]
fn the_four_lists_move_are_cut_lose_ranges_and_drop_their_elements() {
    let body = "<sheetData/><autoFilter ref=\"A3:C9\"/>\
                <conditionalFormatting sqref=\"A3 B1:B2\"><cfRule type=\"x\" priority=\"1\"/></conditionalFormatting>\
                <conditionalFormatting sqref=\"D3:D4\"><cfRule type=\"y\" priority=\"2\"/></conditionalFormatting>\
                <dataValidations count=\"2\"><dataValidation sqref=\"E3 E9\"><formula1>1</formula1></dataValidation>\
                <dataValidation sqref=\"F3:F4\"/></dataValidations>\
                <hyperlinks><hyperlink ref=\"G3\" r:id=\"rId1\" xmlns:r=\"urn:r\"/></hyperlinks>";
    // Rows 3 and 4 deleted: the filter cut, one CF range dropped from its
    // list, the other CF element dropped whole, a DV range dropped and the
    // other DV dropped with the count following, the hyperlinks dropped.
    assert_eq!(
        shifted(body, delete(3, 2)),
        worksheet(
            "<sheetData/><autoFilter ref=\"A3:C7\"/>\
             <conditionalFormatting sqref=\"B1:B2\"><cfRule type=\"x\" priority=\"1\"/></conditionalFormatting>\
             <dataValidations count=\"1\"><dataValidation sqref=\"E7\"><formula1>1</formula1></dataValidation>\
             </dataValidations>"
        )
    );
    // An insertion moves every range; unchanged ranges keep their text.
    assert_eq!(
        shifted(body, insert(3, 1)),
        worksheet(
            "<sheetData/><autoFilter ref=\"A4:C10\"/>\
             <conditionalFormatting sqref=\"A4 B1:B2\"><cfRule type=\"x\" priority=\"1\"/></conditionalFormatting>\
             <conditionalFormatting sqref=\"D4:D5\"><cfRule type=\"y\" priority=\"2\"/></conditionalFormatting>\
             <dataValidations count=\"2\"><dataValidation sqref=\"E4 E10\"><formula1>1</formula1></dataValidation>\
             <dataValidation sqref=\"F4:F5\"/></dataValidations>\
             <hyperlinks><hyperlink ref=\"G4\" r:id=\"rId1\" xmlns:r=\"urn:r\"/></hyperlinks>"
        )
    );
    // A hyperlink kept beside one dropped keeps the container.
    let two =
        "<sheetData/><hyperlinks><hyperlink ref=\"G3\"/> <hyperlink ref=\"G9\"/></hyperlinks>";
    assert_eq!(
        shifted(two, delete(3, 1)),
        worksheet("<sheetData/><hyperlinks> <hyperlink ref=\"G8\"/></hyperlinks>")
    );
    assert_package(
        refused_on(
            with_body("<sheetData/><conditionalFormatting sqref=\"A1 B\"/>"),
            insert(1, 1),
        ),
        UNPARSED,
    );
}

// --- tables and pivot tables ---

/// The smallest package whose worksheet names a table part and a pivot
/// table part with these refs.
fn with_tables(table: &str, pivot: &str) -> Parts {
    let mut parts = with_body("<sheetData/>");
    parts.push((
        SHEET_RELS.to_vec(),
        rels(&format!(
            "{}{}",
            relationship("rId1", &format!("{ATTR_NS}/table"), "../tables/table1.xml"),
            relationship(
                "rId2",
                &format!("{ATTR_NS}/pivotTable"),
                "../pivotTables/pivotTable1.xml"
            ),
        ))
        .into_bytes(),
    ));
    parts.push((
        b"xl/tables/table1.xml".to_vec(),
        format!("<table xmlns=\"{MAIN_NS}\" id=\"1\" ref=\"{table}\"/>").into_bytes(),
    ));
    parts.push((
        b"xl/pivotTables/pivotTable1.xml".to_vec(),
        format!("<pivotTableDefinition xmlns=\"{MAIN_NS}\"><location ref=\"{pivot}\"/></pivotTableDefinition>")
            .into_bytes(),
    ));
    parts
}

#[test]
fn a_table_or_pivot_before_the_point_stays_and_one_at_or_after_it_is_refused() {
    let moved = "an insertion or deletion would move a table or a pivot table";
    // Both wholly before row 5: allowed.
    let parts = with_tables("A1:B3", "D1:E4");
    let out = edited_output(parts.clone(), insert(5, 1));
    assert_untouched(&out, &parts, &[]);
    // The table at the point, the table after it, and likewise the pivot.
    for (table, pivot) in [
        ("A5:B6", "D1:E2"),
        ("A7:B8", "D1:E2"),
        ("A1:B2", "D4:E5"),
        ("A1:B2", "D9:E9"),
    ] {
        assert_package(refused_on(with_tables(table, pivot), insert(5, 1)), moved);
        assert_package(refused_on(with_tables(table, pivot), delete(5, 1)), moved);
    }
    // A table ref that does not parse (P2 ruling 2).
    assert_package(refused_on(with_tables("A1:", "D1:E2"), insert(5, 1)), moved);
}

// --- the one-shift rule and the calls ---

#[test]
fn a_second_shift_or_any_other_edit_for_the_part_is_refused() {
    let alone = "a worksheet with a row or column insertion or deletion takes no other edit in the same write";
    type Edit = fn(&mut Package<Cursor<Vec<u8>>>);
    let others: [Edit; 7] = [
        |p| p.delete_rows(SHEET, 9, 1).expect("recorded"),
        |p| p.set_cell(SHEET, at("A1"), Value::Number(1.0)),
        |p| p.set_cell_style(SHEET, at("A1"), None),
        |p| p.set_row_hidden(SHEET, 1, true).expect("recorded"),
        |p| p.freeze_panes(SHEET, Some(at("B2"))).expect("recorded"),
        |p| {
            p.merge_cells(SHEET, Area::parse(b"A1:B1").expect("area"))
                .expect("recorded")
        },
        |p| {
            p.unmerge_cells(SHEET, Area::parse(b"A1:B1").expect("area"))
                .expect("recorded")
        },
    ];
    for other in others {
        let error = refused_on(with_body("<sheetData/>"), |p| {
            p.insert_rows(SHEET, 2, 1).expect("recorded");
            other(p);
        });
        assert_package(error, alone);
    }
}

#[test]
fn a_count_of_zero_or_rows_past_the_grid_are_refused_at_the_call() {
    let index = "a row or column index or count is out of range";
    let (_, mut package) = opened(smallest());
    for (at, count) in [(0, 1), (1, 0), (1_048_576, 2), (u32::MAX, u32::MAX)] {
        assert_package(
            package.insert_rows(SHEET, at, count).expect_err("insert"),
            index,
        );
        assert_package(
            package.delete_rows(SHEET, at, count).expect_err("delete"),
            index,
        );
    }
    // Nothing refused was recorded.
    let out = written(package);
    assert_untouched(&out, &smallest(), &[]);
    let (_, mut package) = opened(smallest());
    package
        .insert_rows(SHEET, 1_048_576, 1)
        .expect("the last row");
    package
        .delete_rows(b"xl/worksheets/other.xml", 1, 1_048_576)
        .expect("every row");
}

// --- the cases carried from P2's check 1 (`specs/0018-rows-and-columns.md`,
// "Planning verification and rulings on P2", rulings 1 and 3) ---

#[test]
fn an_array_formula_before_the_point_stays_and_a_deletion_inside_it_is_refused() {
    // Ruling 1, F4: the two array cases the shared test has and the array
    // test did not.
    let parts = with_body(
        "<sheetData><row r=\"2\"><c r=\"A2\"><f t=\"array\" ref=\"A2:B4\">X</f><v>1</v></c></row></sheetData>",
    );
    let out = edited_output(parts.clone(), insert(5, 1));
    assert_untouched(&out, &parts, &[]);
    assert_package(refused_on(parts.clone(), delete(3, 1)), CUTS);
    assert_package(refused_on(parts, delete(4, 1)), CUTS);
}

#[test]
fn list_ranges_cut_an_emptied_filter_dropped_and_each_list_unparsed_refused() {
    // Ruling 1, F5: a conditional format's and a validation's range cut, a
    // filter dropped when its range goes, and the validation, hyperlink and
    // filter attributes each refused when they do not parse.
    let body = "<sheetData/><autoFilter ref=\"A3:C4\"/>\
                <conditionalFormatting sqref=\"A3:A6\"><cfRule type=\"x\" priority=\"1\"/></conditionalFormatting>\
                <dataValidations count=\"1\"><dataValidation sqref=\"B2:B5\"/></dataValidations>";
    assert_eq!(
        shifted(body, delete(3, 2)),
        worksheet(
            "<sheetData/>\
             <conditionalFormatting sqref=\"A3:A4\"><cfRule type=\"x\" priority=\"1\"/></conditionalFormatting>\
             <dataValidations count=\"1\"><dataValidation sqref=\"B2:B3\"/></dataValidations>"
        )
    );
    for bad in [
        "<sheetData/><dataValidations count=\"1\"><dataValidation sqref=\"B\"/></dataValidations>",
        "<sheetData/><hyperlinks><hyperlink ref=\"C1:\"/></hyperlinks>",
        "<sheetData/><autoFilter ref=\"A1:\"/>",
    ] {
        assert_package(refused_on(with_body(bad), insert(1, 1)), UNPARSED);
    }
}

#[test]
fn a_second_insertion_for_the_part_is_refused() {
    // Ruling 1, F6: two insertions, where the existing test pairs an
    // insertion with a deletion.
    let error = refused_on(with_body("<sheetData/>"), |p| {
        p.insert_rows(SHEET, 2, 1).expect("recorded");
        p.insert_rows(SHEET, 5, 1).expect("recorded");
    });
    assert_package(
        error,
        "a worksheet with a row or column insertion or deletion takes no other edit in the same write",
    );
}

#[test]
fn a_deletion_that_moves_or_removes_a_formula_sets_the_flag() {
    // Ruling 1, F7: the flag asserted beside the chain on deletions, one
    // that removes the formula's row and one that moves it.
    let rows_after = "<row r=\"1\"><c r=\"A1\"><v>2</v></c></row><row r=\"3\"><c r=\"B3\"><f>A1*2</f><v>4</v></c></row>";
    let mut parts = with_a_calc_chain();
    set(&mut parts, SHEET, sheet_with(rows_after));
    for edit in [delete(3, 1), delete(2, 1)] {
        let out = edited_output(parts.clone(), edit);
        assert_chain_gone(&out);
        assert_part(&out, WORKBOOK, &flagged_workbook());
    }
}

#[test]
fn a_table_ref_absent_a_table_part_missing_or_a_pivot_ref_unparsed_is_refused() {
    // Ruling 1, F8: the three forms the catalogue names beside the
    // unparsed table ref.
    let moved = "an insertion or deletion would move a table or a pivot table";
    let mut absent = with_tables("A1:B2", "D1:E2");
    set(
        &mut absent,
        b"xl/tables/table1.xml",
        format!("<table xmlns=\"{MAIN_NS}\" id=\"1\"/>"),
    );
    assert_package(refused_on(absent, insert(5, 1)), moved);
    let mut missing = with_tables("A1:B2", "D1:E2");
    missing.retain(|(name, _)| name.as_slice() != b"xl/tables/table1.xml");
    assert_package(refused_on(missing, insert(5, 1)), moved);
    assert_package(refused_on(with_tables("A1:B2", "D1:"), insert(5, 1)), moved);
    // The control: the same package with every ref whole is allowed.
    let parts = with_tables("A1:B2", "D1:E2");
    let out = edited_output(parts.clone(), insert(5, 1));
    assert_untouched(&out, &parts, &[]);
}

#[test]
fn a_range_moved_wholly_past_the_grid_is_dropped_and_the_dimension_becomes_a1() {
    // Ruling 3, F10: what clipping leaves empty is dropped (a merge, a list
    // range) and an emptied dimension becomes A1.
    let body = "<dimension ref=\"A1048575:B1048576\"/><sheetData/>\
                <mergeCells count=\"1\"><mergeCell ref=\"A1048575:B1048576\"/></mergeCells>\
                <conditionalFormatting sqref=\"A1 A1048576\"><cfRule type=\"x\" priority=\"1\"/></conditionalFormatting>";
    assert_eq!(
        shifted(body, insert(2, 2)),
        worksheet(
            "<dimension ref=\"A1\"/><sheetData/>\
             <conditionalFormatting sqref=\"A1\"><cfRule type=\"x\" priority=\"1\"/></conditionalFormatting>"
        )
    );
}
