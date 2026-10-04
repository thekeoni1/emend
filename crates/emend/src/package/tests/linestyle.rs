//! Styles on whole rows and columns on byte-built worksheets
//! (`specs/0026-grouping-and-line-styles.md`, "P2: row and column styles"):
//! each test builds the smallest package with four cell formats in its
//! styles part and its worksheet holding a given body, records a row or a
//! column style, and compares the written worksheet with the bytes the
//! design gives, every other part asserted untouched. The Cargo feature
//! `rowcol`.

use super::*;

/// The smallest package with four cell formats and its worksheet holding
/// `body`.
fn with_styles(body: &str) -> Parts {
    let mut parts = smallest();
    set(&mut parts, SHEET, worksheet(body));
    set(&mut parts, STYLES_PART, style_sheet(FOUR_XFS));
    parts
}

/// The worksheet part after `edit` on that package, as text; every other
/// part is asserted untouched.
#[track_caller]
fn edited(body: &str, edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> String {
    let parts = with_styles(body);
    let output = edited_output(parts.clone(), edit);
    assert_untouched(&output, &parts, &[SHEET]);
    String::from_utf8(stored_data(&output, SHEET)).expect("the part is UTF-8")
}

fn row(at: u32, style: Option<u32>) -> impl FnOnce(&mut Package<Cursor<Vec<u8>>>) {
    move |p| p.set_row_style(SHEET, at, style).expect("recorded")
}

fn column(at: u32, style: Option<u32>) -> impl FnOnce(&mut Package<Cursor<Vec<u8>>>) {
    move |p| p.set_column_style(SHEET, at, style).expect("recorded")
}

const COLS5: &str =
    "<cols><col min=\"1\" max=\"5\" width=\"9\" customWidth=\"1\" style=\"3\"/></cols>";

// --- rows ---

#[test]
fn a_row_style_on_a_present_row_adds_the_two_attributes_and_keeps_the_rest() {
    let body = "<sheetData><row r=\"2\" spans=\"1:2\" ht=\"20\" customHeight=\"1\"><c r=\"A2\" s=\"1\"><v>1</v></c><c r=\"B2\"><v>2</v></c></row></sheetData>";
    assert_eq!(
        edited(body, row(2, Some(3))),
        worksheet(
            "<sheetData><row r=\"2\" spans=\"1:2\" ht=\"20\" customHeight=\"1\" s=\"3\" customFormat=\"1\"><c r=\"A2\" s=\"1\"><v>1</v></c><c r=\"B2\"><v>2</v></c></row></sheetData>"
        )
    );
    // A present style is replaced in place.
    let body = "<sheetData><row r=\"2\" s=\"1\" customFormat=\"1\" ht=\"9\"/></sheetData>";
    assert_eq!(
        edited(body, row(2, Some(2))),
        worksheet("<sheetData><row r=\"2\" s=\"2\" customFormat=\"1\" ht=\"9\"/></sheetData>")
    );
}

#[test]
fn a_row_style_on_an_absent_row_creates_it_between_present_ones() {
    let body = "<sheetData><row r=\"1\"><c r=\"A1\"><v>1</v></c></row><row r=\"5\"/></sheetData>";
    assert_eq!(
        edited(body, row(3, Some(2))),
        worksheet(
            "<sheetData><row r=\"1\"><c r=\"A1\"><v>1</v></c></row><row r=\"3\" s=\"2\" customFormat=\"1\"/><row r=\"5\"/></sheetData>"
        )
    );
}

#[test]
fn none_removes_both_row_attributes_and_creates_nothing() {
    let body = "<sheetData><row r=\"2\" s=\"3\" customFormat=\"1\" ht=\"9\"/></sheetData>";
    let edit = |p: &mut Package<Cursor<Vec<u8>>>| {
        p.set_row_style(SHEET, 2, None).expect("recorded");
        p.set_row_style(SHEET, 4, None).expect("recorded");
    };
    assert_eq!(
        edited(body, edit),
        worksheet("<sheetData><row r=\"2\" ht=\"9\"/></sheetData>")
    );
}

#[test]
fn a_row_style_with_a_group_on_the_same_row_writes_the_style_first() {
    let edit = |p: &mut Package<Cursor<Vec<u8>>>| {
        p.set_row_style(SHEET, 2, Some(1)).expect("recorded");
        p.group_rows(SHEET, 2, 2, 1, true).expect("recorded");
    };
    assert_eq!(
        edited("<sheetData/>", edit),
        worksheet(
            "<sheetFormatPr defaultRowHeight=\"15\" outlineLevelRow=\"1\"/><sheetData><row r=\"2\" s=\"1\" customFormat=\"1\" hidden=\"1\" outlineLevel=\"1\"/></sheetData>"
        )
    );
}

// --- columns ---

#[test]
fn a_column_style_inside_a_covering_col_is_split_out() {
    let body = format!("{COLS5}<sheetData/>");
    assert_eq!(
        edited(&body, column(3, Some(2))),
        worksheet(
            "<cols><col min=\"1\" max=\"2\" width=\"9\" customWidth=\"1\" style=\"3\"/><col min=\"3\" max=\"3\" width=\"9\" customWidth=\"1\" style=\"2\"/><col min=\"4\" max=\"5\" width=\"9\" customWidth=\"1\" style=\"3\"/></cols><sheetData/>"
        )
    );
}

#[test]
fn a_column_style_at_each_end_and_over_a_whole_col() {
    let body = format!("{COLS5}<sheetData/>");
    assert_eq!(
        edited(&body, column(1, Some(2))),
        worksheet(
            "<cols><col min=\"1\" max=\"1\" width=\"9\" customWidth=\"1\" style=\"2\"/><col min=\"2\" max=\"5\" width=\"9\" customWidth=\"1\" style=\"3\"/></cols><sheetData/>"
        )
    );
    assert_eq!(
        edited(&body, column(5, Some(2))),
        worksheet(
            "<cols><col min=\"1\" max=\"4\" width=\"9\" customWidth=\"1\" style=\"3\"/><col min=\"5\" max=\"5\" width=\"9\" customWidth=\"1\" style=\"2\"/></cols><sheetData/>"
        )
    );
    // Every column of the element styled: the pieces coalesce back to one.
    let edit = |p: &mut Package<Cursor<Vec<u8>>>| {
        for at in 1..=5 {
            p.set_column_style(SHEET, at, Some(2)).expect("recorded");
        }
    };
    assert_eq!(
        edited(&body, edit),
        worksheet(
            "<cols><col min=\"1\" max=\"5\" width=\"9\" customWidth=\"1\" style=\"2\"/></cols><sheetData/>"
        )
    );
}

#[test]
fn a_column_style_on_uncovered_columns_creates_one_element() {
    let edit = |p: &mut Package<Cursor<Vec<u8>>>| {
        p.set_column_style(SHEET, 8, Some(1)).expect("recorded");
        p.set_column_style(SHEET, 9, Some(1)).expect("recorded");
    };
    assert_eq!(
        edited("<dimension ref=\"A1\"/><sheetData/>", edit),
        worksheet(
            "<dimension ref=\"A1\"/><cols><col min=\"8\" max=\"9\" style=\"1\"/></cols><sheetData/>"
        )
    );
}

#[test]
fn none_removes_the_column_style_and_a_col_left_bare_goes() {
    let body = format!("{COLS5}<sheetData/>");
    assert_eq!(
        edited(&body, column(3, None)),
        worksheet(
            "<cols><col min=\"1\" max=\"2\" width=\"9\" customWidth=\"1\" style=\"3\"/><col min=\"3\" max=\"3\" width=\"9\" customWidth=\"1\"/><col min=\"4\" max=\"5\" width=\"9\" customWidth=\"1\" style=\"3\"/></cols><sheetData/>"
        )
    );
    let body = "<cols><col min=\"3\" max=\"3\" style=\"3\"/></cols><sheetData/>";
    assert_eq!(edited(body, column(3, None)), worksheet("<sheetData/>"));
}

// --- the index ---

#[test]
fn an_index_past_cell_xfs_is_refused_at_the_write() {
    let message = "a style index is out of range";
    let (_, mut package) = opened(with_styles("<sheetData/>"));
    package.set_row_style(SHEET, 2, Some(4)).expect("recorded");
    assert_package(refused(package), message);
    let (_, mut package) = opened(with_styles("<sheetData/>"));
    package
        .set_column_style(SHEET, 2, Some(u32::MAX))
        .expect("recorded");
    assert_package(refused(package), message);
    // The control: the last format there is.
    let (_, mut package) = opened(with_styles("<sheetData/>"));
    package.set_row_style(SHEET, 2, Some(3)).expect("recorded");
    package
        .set_column_style(SHEET, 2, Some(3))
        .expect("recorded");
    written(package);
}

#[test]
fn an_index_add_style_returns_in_the_same_write_is_accepted() {
    let mut parts = smallest();
    set(&mut parts, SHEET, worksheet("<sheetData/>"));
    set(
        &mut parts,
        STYLES_PART,
        style_sheet(&with_collections("<cellXfs count=\"1\"><xf/></cellXfs>")),
    );
    let output = edited_output(parts.clone(), |p| {
        let style = Style {
            number_format: Some(code("0.00%")),
            ..Style::default()
        };
        let added = p.add_style(Some(0), &style).expect("recorded");
        assert_eq!(added, 1);
        p.set_row_style(SHEET, 2, Some(added)).expect("recorded");
        p.set_column_style(SHEET, 2, Some(added)).expect("recorded");
    });
    assert_untouched(&output, &parts, &[SHEET, STYLES_PART]);
    assert_eq!(
        String::from_utf8(stored_data(&output, SHEET)).expect("UTF-8"),
        worksheet(
            "<cols><col min=\"2\" max=\"2\" style=\"1\"/></cols><sheetData><row r=\"2\" s=\"1\" customFormat=\"1\"/></sheetData>"
        )
    );
}

#[test]
fn the_refusals_at_the_call_record_nothing() {
    let index = "a row or column index or count is out of range";
    let (parts, mut package) = opened(with_styles("<sheetData/>"));
    for at in [0, 1_048_577] {
        assert_package(
            package
                .set_row_style(SHEET, at, Some(1))
                .expect_err("refused"),
            index,
        );
    }
    for at in [0, 16_385] {
        assert_package(
            package
                .set_column_style(SHEET, at, Some(1))
                .expect_err("refused"),
            index,
        );
    }
    assert_untouched(&written(package), &parts, &[]);
}

#[test]
fn a_style_with_a_shift_or_on_a_removed_part_is_refused() {
    let shift = "a worksheet with a row or column insertion or deletion takes no other edit in the same write";
    let (_, mut package) = opened(with_styles("<sheetData/>"));
    package.insert_rows(SHEET, 1, 1).expect("recorded");
    package.set_row_style(SHEET, 2, Some(1)).expect("recorded");
    assert_package(refused(package), shift);
    let (_, mut package) = opened(with_styles("<sheetData/>"));
    package
        .set_column_style(SHEET, 2, Some(1))
        .expect("recorded");
    package.remove_part(SHEET);
    assert_package(refused(package), "an edit to a removed part");
}

// --- through a shift ---

#[test]
fn a_styled_row_and_column_are_kept_through_an_insertion_in_a_second_write() {
    let edit = |p: &mut Package<Cursor<Vec<u8>>>| {
        p.set_row_style(SHEET, 2, Some(2)).expect("recorded");
        p.set_column_style(SHEET, 2, Some(2)).expect("recorded");
    };
    let first = edited(
        "<sheetData><row r=\"2\"><c r=\"A2\"><v>1</v></c></row></sheetData>",
        edit,
    );
    assert_eq!(
        first,
        worksheet(
            "<cols><col min=\"2\" max=\"2\" style=\"2\"/></cols><sheetData><row r=\"2\" s=\"2\" customFormat=\"1\"><c r=\"A2\"><v>1</v></c></row></sheetData>"
        )
    );
    // The second and third writes, one shift each (part 3's one-shift rule):
    // the row moves down with its style, then the column right with its.
    let shifted = |text: String, edit: fn(&mut Package<Cursor<Vec<u8>>>)| {
        let mut parts = with_styles("<sheetData/>");
        set(&mut parts, SHEET, text);
        let output = edited_output(parts.clone(), edit);
        assert_untouched(&output, &parts, &[SHEET]);
        String::from_utf8(stored_data(&output, SHEET)).expect("UTF-8")
    };
    let second = shifted(first, |p| p.insert_rows(SHEET, 1, 1).expect("recorded"));
    assert_eq!(
        second,
        worksheet(
            "<cols><col min=\"2\" max=\"2\" style=\"2\"/></cols><sheetData><row r=\"3\" s=\"2\" customFormat=\"1\"><c r=\"A3\"><v>1</v></c></row></sheetData>"
        )
    );
    assert_eq!(
        shifted(second, |p| p.insert_columns(SHEET, 1, 1).expect("recorded")),
        worksheet(
            "<cols><col min=\"3\" max=\"3\" style=\"2\"/></cols><sheetData><row r=\"3\" s=\"2\" customFormat=\"1\"><c r=\"B3\"><v>1</v></c></row></sheetData>"
        )
    );
}
