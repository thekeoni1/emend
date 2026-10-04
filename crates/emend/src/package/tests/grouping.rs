//! Outline grouping on byte-built worksheets
//! (`specs/0026-grouping-and-line-styles.md`, "P1: outline levels,
//! collapsed, the maximums and the summary", with "Planning rulings during
//! P1"): each test builds the smallest package with its worksheet holding a
//! given body, records a group, a collapsed flag or an outline summary, and
//! compares the written worksheet with the bytes the design gives, every
//! other part asserted untouched. The Cargo feature `rowcol`.

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

/// The refusal a write of `edit` on that package gives.
#[track_caller]
fn refused_body(body: &str, edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> Error {
    let mut parts = smallest();
    set(&mut parts, SHEET, worksheet(body));
    let (_, mut package) = opened(parts);
    edit(&mut package);
    refused(package)
}

fn rows(
    first: u32,
    last: u32,
    level: u8,
    hidden: bool,
) -> impl FnOnce(&mut Package<Cursor<Vec<u8>>>) {
    move |p| {
        p.group_rows(SHEET, first, last, level, hidden)
            .expect("recorded")
    }
}

fn columns(
    first: u32,
    last: u32,
    level: u8,
    hidden: bool,
) -> impl FnOnce(&mut Package<Cursor<Vec<u8>>>) {
    move |p| {
        p.group_columns(SHEET, first, last, level, hidden)
            .expect("recorded")
    }
}

/// The `<sheetFormatPr>` a write creates for the maximums `row` and `col`
/// (0 for none).
fn created_format(row: u8, col: u8) -> String {
    let mut out = String::from("<sheetFormatPr defaultRowHeight=\"15\"");
    if row > 0 {
        out.push_str(&format!(" outlineLevelRow=\"{row}\""));
    }
    if col > 0 {
        out.push_str(&format!(" outlineLevelCol=\"{col}\""));
    }
    out.push_str("/>");
    out
}

const SETTINGS_MESSAGE: &str = "a worksheet has two sheetPr, autoFilter, printOptions, pageMargins, pageSetup or headerFooter elements";
const SHIFT_MESSAGE: &str =
    "a worksheet with a row or column insertion or deletion takes no other edit in the same write";

// --- rows ---

#[test]
fn a_row_group_over_present_rows_adds_the_attributes_in_place() {
    let body = "<sheetData><row r=\"2\" spans=\"1:2\" ht=\"20\" customHeight=\"1\"><c r=\"A2\"><v>1</v></c></row><row r=\"3\"><c r=\"A3\"><v>2</v></c></row><row r=\"4\"/></sheetData>";
    assert_eq!(
        edited(body, rows(2, 3, 1, true)),
        worksheet(&format!(
            "{}<sheetData><row r=\"2\" spans=\"1:2\" ht=\"20\" customHeight=\"1\" hidden=\"1\" outlineLevel=\"1\"><c r=\"A2\"><v>1</v></c></row><row r=\"3\" hidden=\"1\" outlineLevel=\"1\"><c r=\"A3\"><v>2</v></c></row><row r=\"4\"/></sheetData>",
            created_format(1, 0)
        ))
    );
}

#[test]
fn a_row_group_over_absent_rows_creates_each_in_order_between_present_ones() {
    let body = "<sheetData><row r=\"1\"><c r=\"A1\"><v>1</v></c></row><row r=\"5\"/></sheetData>";
    assert_eq!(
        edited(body, rows(2, 3, 2, false)),
        worksheet(&format!(
            "{}<sheetData><row r=\"1\"><c r=\"A1\"><v>1</v></c></row><row r=\"2\" outlineLevel=\"2\"/><row r=\"3\" outlineLevel=\"2\"/><row r=\"5\"/></sheetData>",
            created_format(2, 0)
        ))
    );
}

#[test]
fn level_0_removes_the_level_and_creates_nothing() {
    // Level 0 removes `outlineLevel`; `hidden` is as the call gives it, here
    // set (kept in place) on row 2; rows 3 and 4 are absent and level 0
    // without `hidden` writes nothing there; no maximum is raised.
    let body = "<sheetFormatPr defaultRowHeight=\"15\" outlineLevelRow=\"3\"/><sheetData><row r=\"2\" outlineLevel=\"3\" hidden=\"1\"/></sheetData>";
    assert_eq!(
        edited(body, rows(2, 2, 0, true)),
        worksheet(
            "<sheetFormatPr defaultRowHeight=\"15\" outlineLevelRow=\"3\"/><sheetData><row r=\"2\" hidden=\"1\"/></sheetData>"
        )
    );
    let body = "<sheetData><row r=\"2\" outlineLevel=\"1\"/></sheetData>";
    assert_eq!(
        edited(body, rows(2, 4, 0, false)),
        worksheet("<sheetData><row r=\"2\"/></sheetData>")
    );
}

#[test]
fn hidden_is_set_kept_and_removed_as_the_call_gives_it() {
    // Set on a shown row; kept, in place, on a hidden one; removed by a
    // group with `hidden: false` ("Planning rulings during P1", ruling 3).
    let body = "<sheetData><row r=\"2\"/><row r=\"3\" hidden=\"1\" ht=\"9\"/></sheetData>";
    assert_eq!(
        edited(body, rows(2, 3, 1, true)),
        worksheet(&format!(
            "{}<sheetData><row r=\"2\" hidden=\"1\" outlineLevel=\"1\"/><row r=\"3\" hidden=\"1\" ht=\"9\" outlineLevel=\"1\"/></sheetData>",
            created_format(1, 0)
        ))
    );
    assert_eq!(
        edited(body, rows(3, 3, 1, false)),
        worksheet(&format!(
            "{}<sheetData><row r=\"2\"/><row r=\"3\" ht=\"9\" outlineLevel=\"1\"/></sheetData>",
            created_format(1, 0)
        ))
    );
}

#[test]
fn the_last_call_for_a_row_wins_per_field() {
    let body = "<sheetData/>";
    let edit = |p: &mut Package<Cursor<Vec<u8>>>| {
        p.group_rows(SHEET, 2, 3, 1, true).expect("recorded");
        p.group_rows(SHEET, 3, 3, 2, false).expect("recorded");
    };
    assert_eq!(
        edited(body, edit),
        worksheet(&format!(
            "{}<sheetData><row r=\"2\" hidden=\"1\" outlineLevel=\"1\"/><row r=\"3\" outlineLevel=\"2\"/></sheetData>",
            created_format(2, 0)
        ))
    );
}

// --- collapsed ---

#[test]
fn a_collapsed_row_is_created_set_and_removed() {
    let body = "<sheetData><row r=\"2\" collapsed=\"1\" outlineLevel=\"1\"/></sheetData>";
    let edit = |p: &mut Package<Cursor<Vec<u8>>>| {
        p.set_row_collapsed(SHEET, 2, false).expect("recorded");
        p.set_row_collapsed(SHEET, 4, true).expect("recorded");
    };
    // No level is set, so `<sheetFormatPr>` is neither created nor changed.
    assert_eq!(
        edited(body, edit),
        worksheet(
            "<sheetData><row r=\"2\" outlineLevel=\"1\"/><row r=\"4\" collapsed=\"1\"/></sheetData>"
        )
    );
}

#[test]
fn a_collapsed_column_is_split_out_of_its_covering_col() {
    let body = format!("{COLS5}<sheetData/>");
    assert_eq!(
        edited(&body, |p| p
            .set_column_collapsed(SHEET, 3, true)
            .expect("recorded")),
        worksheet(
            "<cols><col min=\"1\" max=\"2\" width=\"9\" customWidth=\"1\" style=\"3\"/><col min=\"3\" max=\"3\" width=\"9\" customWidth=\"1\" style=\"3\" collapsed=\"1\"/><col min=\"4\" max=\"5\" width=\"9\" customWidth=\"1\" style=\"3\"/></cols><sheetData/>"
        )
    );
    // And removed from the middle column: the pieces stay apart, their
    // attributes differing.
    let body = "<cols><col min=\"2\" max=\"4\" width=\"9\" collapsed=\"1\"/></cols><sheetData/>";
    assert_eq!(
        edited(body, |p| p
            .set_column_collapsed(SHEET, 3, false)
            .expect("recorded")),
        worksheet(
            "<cols><col min=\"2\" max=\"2\" width=\"9\" collapsed=\"1\"/><col min=\"3\" max=\"3\" width=\"9\"/><col min=\"4\" max=\"4\" width=\"9\" collapsed=\"1\"/></cols><sheetData/>"
        )
    );
}

// --- columns and the coalescing ---

const COLS5: &str =
    "<cols><col min=\"1\" max=\"5\" width=\"9\" customWidth=\"1\" style=\"3\"/></cols>";

#[test]
fn a_column_group_over_the_range_one_col_covers_stays_one_element() {
    let body = "<cols><col min=\"2\" max=\"3\" width=\"9\" customWidth=\"1\" style=\"3\"/></cols><sheetData/>";
    assert_eq!(
        edited(body, columns(2, 3, 1, false)),
        worksheet(&format!(
            "{}<cols><col min=\"2\" max=\"3\" width=\"9\" customWidth=\"1\" style=\"3\" outlineLevel=\"1\"/></cols><sheetData/>",
            created_format(0, 1)
        ))
    );
}

#[test]
fn a_column_group_inside_a_wider_col_is_split_then_coalesced() {
    let body = format!("{COLS5}<sheetData/>");
    assert_eq!(
        edited(&body, columns(2, 3, 1, true)),
        worksheet(&format!(
            "{}<cols><col min=\"1\" max=\"1\" width=\"9\" customWidth=\"1\" style=\"3\"/><col min=\"2\" max=\"3\" width=\"9\" customWidth=\"1\" style=\"3\" hidden=\"1\" outlineLevel=\"1\"/><col min=\"4\" max=\"5\" width=\"9\" customWidth=\"1\" style=\"3\"/></cols><sheetData/>",
            created_format(0, 1)
        ))
    );
}

#[test]
fn a_column_group_over_two_cols_of_different_widths_gives_two_elements() {
    let body = "<cols><col min=\"2\" max=\"2\" width=\"9\" customWidth=\"1\"/><col min=\"3\" max=\"3\" width=\"12\" customWidth=\"1\"/></cols><sheetData/>";
    assert_eq!(
        edited(body, columns(2, 3, 1, false)),
        worksheet(&format!(
            "{}<cols><col min=\"2\" max=\"2\" width=\"9\" customWidth=\"1\" outlineLevel=\"1\"/><col min=\"3\" max=\"3\" width=\"12\" customWidth=\"1\" outlineLevel=\"1\"/></cols><sheetData/>",
            created_format(0, 1)
        ))
    );
}

#[test]
fn a_column_group_over_uncovered_columns_creates_one_element() {
    let body = "<cols><col min=\"1\" max=\"1\" width=\"5\" customWidth=\"1\"/></cols><sheetData/>";
    assert_eq!(
        edited(body, columns(3, 6, 2, true)),
        worksheet(&format!(
            "{}<cols><col min=\"1\" max=\"1\" width=\"5\" customWidth=\"1\"/><col min=\"3\" max=\"6\" hidden=\"1\" outlineLevel=\"2\"/></cols><sheetData/>",
            created_format(0, 2)
        ))
    );
    // And a part with no `<cols>` gains one in schema position, after the
    // created `<sheetFormatPr>`.
    assert_eq!(
        edited(
            "<dimension ref=\"A1\"/><sheetData/>",
            columns(2, 3, 1, false)
        ),
        worksheet(&format!(
            "<dimension ref=\"A1\"/>{}<cols><col min=\"2\" max=\"3\" outlineLevel=\"1\"/></cols><sheetData/>",
            created_format(0, 1)
        ))
    );
}

#[test]
fn a_column_group_over_a_half_covered_range_gives_two_elements() {
    let body = "<cols><col min=\"2\" max=\"2\" width=\"9\" customWidth=\"1\"/></cols><sheetData/>";
    assert_eq!(
        edited(body, columns(2, 3, 1, false)),
        worksheet(&format!(
            "{}<cols><col min=\"2\" max=\"2\" width=\"9\" customWidth=\"1\" outlineLevel=\"1\"/><col min=\"3\" max=\"3\" outlineLevel=\"1\"/></cols><sheetData/>",
            created_format(0, 1)
        ))
    );
}

#[test]
fn an_element_the_write_does_not_touch_is_never_merged() {
    // Column 1's element already carries what the group writes on column 2;
    // it is not the write's, so the two stay apart ("Planning rulings during
    // P1", ruling 2).
    let body = "<cols><col min=\"1\" max=\"1\" width=\"9\" outlineLevel=\"1\"/><col min=\"2\" max=\"2\" width=\"9\"/></cols><sheetData/>";
    assert_eq!(
        edited(body, columns(2, 2, 1, false)),
        worksheet(&format!(
            "{}<cols><col min=\"1\" max=\"1\" width=\"9\" outlineLevel=\"1\"/><col min=\"2\" max=\"2\" width=\"9\" outlineLevel=\"1\"/></cols><sheetData/>",
            created_format(0, 1)
        ))
    );
}

#[test]
fn widths_alone_write_today_s_split_without_the_coalescing() {
    // `set_column_width` and `set_column_hidden` alone keep specs/0018's
    // bytes; a group in the same write coalesces every piece.
    let body = format!("{COLS5}<sheetData/>");
    let widths = |p: &mut Package<Cursor<Vec<u8>>>| {
        p.set_column_width(SHEET, 2, Some(12.0)).expect("recorded");
        p.set_column_width(SHEET, 3, Some(12.0)).expect("recorded");
    };
    assert_eq!(
        edited(&body, widths),
        worksheet(
            "<cols><col min=\"1\" max=\"1\" width=\"9\" customWidth=\"1\" style=\"3\"/><col min=\"2\" max=\"2\" width=\"12\" customWidth=\"1\" style=\"3\"/><col min=\"3\" max=\"3\" width=\"12\" customWidth=\"1\" style=\"3\"/><col min=\"4\" max=\"5\" width=\"9\" customWidth=\"1\" style=\"3\"/></cols><sheetData/>"
        )
    );
    let both = |p: &mut Package<Cursor<Vec<u8>>>| {
        widths(p);
        p.group_columns(SHEET, 2, 3, 1, false).expect("recorded");
    };
    assert_eq!(
        edited(&body, both),
        worksheet(&format!(
            "{}<cols><col min=\"1\" max=\"1\" width=\"9\" customWidth=\"1\" style=\"3\"/><col min=\"2\" max=\"3\" width=\"12\" customWidth=\"1\" style=\"3\" outlineLevel=\"1\"/><col min=\"4\" max=\"5\" width=\"9\" customWidth=\"1\" style=\"3\"/></cols><sheetData/>",
            created_format(0, 1)
        ))
    );
}

#[test]
fn hidden_flags_alone_write_today_s_split_without_the_coalescing() {
    // P1 verification ruling 4 (finding 6): `set_column_hidden` alone keeps
    // specs/0018's bytes with the feature on, the equal pieces unmerged.
    let body = format!("{COLS5}<sheetData/>");
    let hidden = |p: &mut Package<Cursor<Vec<u8>>>| {
        p.set_column_hidden(SHEET, 2, true).expect("recorded");
        p.set_column_hidden(SHEET, 3, true).expect("recorded");
    };
    assert_eq!(
        edited(&body, hidden),
        worksheet(
            "<cols><col min=\"1\" max=\"1\" width=\"9\" customWidth=\"1\" style=\"3\"/><col min=\"2\" max=\"2\" width=\"9\" customWidth=\"1\" style=\"3\" hidden=\"1\"/><col min=\"3\" max=\"3\" width=\"9\" customWidth=\"1\" style=\"3\" hidden=\"1\"/><col min=\"4\" max=\"5\" width=\"9\" customWidth=\"1\" style=\"3\"/></cols><sheetData/>"
        )
    );
}

#[test]
fn pieces_merge_across_whitespace_alone_and_not_across_a_comment() {
    // P1 verification ruling 2: a whitespace-only run between two pieces the
    // write produced is dropped when they merge; a comment ends the merge;
    // whitespace between pieces that do not merge is kept.
    let indented = "<cols>\n  <col min=\"2\" max=\"2\" width=\"9\"/>\n  <col min=\"3\" max=\"3\" width=\"9\"/>\n</cols><sheetData/>";
    assert_eq!(
        edited(indented, columns(2, 3, 1, false)),
        worksheet(&format!(
            "{}<cols>\n  <col min=\"2\" max=\"3\" width=\"9\" outlineLevel=\"1\"/>\n</cols><sheetData/>",
            created_format(0, 1)
        ))
    );
    let commented = "<cols><col min=\"2\" max=\"2\" width=\"9\"/><!-- c --><col min=\"3\" max=\"3\" width=\"9\"/></cols><sheetData/>";
    assert_eq!(
        edited(commented, columns(2, 3, 1, false)),
        worksheet(&format!(
            "{}<cols><col min=\"2\" max=\"2\" width=\"9\" outlineLevel=\"1\"/><!-- c --><col min=\"3\" max=\"3\" width=\"9\" outlineLevel=\"1\"/></cols><sheetData/>",
            created_format(0, 1)
        ))
    );
    let unequal = "<cols>\n  <col min=\"2\" max=\"2\" width=\"9\"/>\n  <col min=\"3\" max=\"3\" width=\"12\"/>\n</cols><sheetData/>";
    assert_eq!(
        edited(unequal, columns(2, 3, 1, false)),
        worksheet(&format!(
            "{}<cols>\n  <col min=\"2\" max=\"2\" width=\"9\" outlineLevel=\"1\"/>\n  <col min=\"3\" max=\"3\" width=\"12\" outlineLevel=\"1\"/>\n</cols><sheetData/>",
            created_format(0, 1)
        ))
    );
}

// --- the maximums ---

#[test]
fn the_maximums_are_raised_on_a_present_sheet_format_pr_and_never_lowered() {
    let both = |p: &mut Package<Cursor<Vec<u8>>>| {
        p.group_rows(SHEET, 2, 2, 2, false).expect("recorded");
        p.group_columns(SHEET, 2, 2, 3, false).expect("recorded");
    };
    // Raised: the set attributes first, every other kept in its order.
    let body = "<sheetFormatPr baseColWidth=\"10\" defaultRowHeight=\"15\" outlineLevelRow=\"0\" x14ac:dyDescent=\"0.25\"/><sheetData/>";
    let edited_text = edited(body, both);
    assert!(
        edited_text.contains(
            "<sheetFormatPr outlineLevelRow=\"2\" outlineLevelCol=\"3\" baseColWidth=\"10\" defaultRowHeight=\"15\" x14ac:dyDescent=\"0.25\"/>"
        ),
        "{edited_text}"
    );
    // A higher file value is kept: the element is copied as it is.
    let body = "<sheetFormatPr defaultRowHeight=\"15\" outlineLevelRow=\"5\" outlineLevelCol=\"3\"/><sheetData/>";
    assert!(edited(body, both).contains(
        "<sheetFormatPr defaultRowHeight=\"15\" outlineLevelRow=\"5\" outlineLevelCol=\"3\"/>"
    ));
    // A value that is not a whole number from 0 to 255 counts as 0
    // ("Planning rulings during P1", ruling 4).
    for value in ["x", "-1", "256", ""] {
        let body = format!(
            "<sheetFormatPr defaultRowHeight=\"15\" outlineLevelRow=\"{value}\"/><sheetData/>"
        );
        assert_eq!(
            edited(&body, rows(2, 2, 1, false)),
            worksheet(
                "<sheetFormatPr outlineLevelRow=\"1\" defaultRowHeight=\"15\"/><sheetData><row r=\"2\" outlineLevel=\"1\"/></sheetData>"
            ),
            "{value}"
        );
    }
}

#[test]
fn a_created_sheet_format_pr_goes_after_the_views_and_before_cols() {
    let body = "<sheetPr/><dimension ref=\"A1\"/><sheetViews><sheetView workbookViewId=\"0\"/></sheetViews><cols><col min=\"9\" max=\"9\" width=\"5\"/></cols><sheetData/>";
    let both = |p: &mut Package<Cursor<Vec<u8>>>| {
        p.group_rows(SHEET, 2, 2, 7, false).expect("recorded");
        p.group_columns(SHEET, 2, 2, 4, false).expect("recorded");
    };
    assert_eq!(
        edited(body, both),
        worksheet(&format!(
            "<sheetPr/><dimension ref=\"A1\"/><sheetViews><sheetView workbookViewId=\"0\"/></sheetViews>{}<cols><col min=\"2\" max=\"2\" outlineLevel=\"4\"/><col min=\"9\" max=\"9\" width=\"5\"/></cols><sheetData><row r=\"2\" outlineLevel=\"7\"/></sheetData>",
            created_format(7, 4)
        ))
    );
}

// --- the outline summary ---

#[test]
fn the_summary_creates_outline_pr_under_a_created_sheet_pr() {
    assert_eq!(
        edited("<dimension ref=\"A1\"/><sheetData/>", |p| {
            p.set_outline_summary(SHEET, Some(false), None)
        }),
        worksheet(
            "<sheetPr><outlinePr summaryBelow=\"0\"/></sheetPr><dimension ref=\"A1\"/><sheetData/>"
        )
    );
}

#[test]
fn the_summary_creates_outline_pr_between_tab_color_and_page_set_up_pr() {
    let body = "<sheetPr codeName=\"a\"><tabColor rgb=\"FF000000\"/><pageSetUpPr fitToPage=\"1\"/></sheetPr><sheetData/>";
    assert_eq!(
        edited(body, |p| p.set_outline_summary(SHEET, None, Some(false))),
        worksheet(
            "<sheetPr codeName=\"a\"><tabColor rgb=\"FF000000\"/><outlinePr summaryRight=\"0\"/><pageSetUpPr fitToPage=\"1\"/></sheetPr><sheetData/>"
        )
    );
}

#[test]
fn the_summary_merges_into_a_present_outline_pr() {
    let body = "<sheetPr><outlinePr summaryRight=\"0\" applyStyles=\"1\"/></sheetPr><sheetData/>";
    let edit = |p: &mut Package<Cursor<Vec<u8>>>| {
        p.set_outline_summary(SHEET, Some(false), None);
        p.set_outline_summary(SHEET, Some(true), Some(true));
    };
    assert_eq!(
        edited(body, edit),
        worksheet(
            "<sheetPr><outlinePr summaryBelow=\"1\" summaryRight=\"1\" applyStyles=\"1\"/></sheetPr><sheetData/>"
        )
    );
    // `None` on both writes nothing.
    let (parts, mut package) = opened(smallest());
    package.set_outline_summary(SHEET, None, None);
    assert_untouched(&written(package), &parts, &[SHEET]);
}

// --- the refusals ---

#[test]
fn the_refusals_at_the_call_record_nothing() {
    let index = "a row or column index or count is out of range";
    let level = "an outline level is out of range";
    let (parts, mut package) = opened(smallest());
    for (first, last, lvl, message) in [
        (1, 1, 8, level),
        (1, 1, u8::MAX, level),
        (3, 2, 1, index),
        (0, 2, 1, index),
        (1, 1_048_577, 1, index),
    ] {
        assert_package(
            package
                .group_rows(SHEET, first, last, lvl, true)
                .expect_err("refused"),
            message,
        );
    }
    for (first, last, lvl, message) in [
        (1, 1, 8, level),
        (3, 2, 1, index),
        (0, 2, 1, index),
        (1, 16_385, 1, index),
    ] {
        assert_package(
            package
                .group_columns(SHEET, first, last, lvl, true)
                .expect_err("refused"),
            message,
        );
    }
    // An index off the grid with a level out of range: the index first.
    assert_package(
        package
            .group_rows(SHEET, 0, 1, 8, true)
            .expect_err("refused"),
        index,
    );
    for row in [0, 1_048_577] {
        assert_package(
            package
                .set_row_collapsed(SHEET, row, true)
                .expect_err("refused"),
            index,
        );
    }
    for column in [0, 16_385] {
        assert_package(
            package
                .set_column_collapsed(SHEET, column, true)
                .expect_err("refused"),
            index,
        );
    }
    // Nothing recorded: the write is the input.
    assert_untouched(&written(package), &parts, &[]);
    // The bounds themselves are accepted.
    let (_, mut package) = opened(smallest());
    package
        .group_rows(SHEET, 1_048_576, 1_048_576, 7, true)
        .expect("in range");
    package
        .group_columns(SHEET, 16_384, 16_384, 0, false)
        .expect("in range");
}

#[test]
fn the_write_refusals() {
    // Two `<sheetPr>`, or two `<sheetFormatPr>`, or one out of schema order,
    // with the settings message ("Planning rulings during P1", ruling 1).
    for body in ["<sheetPr/><sheetPr/><sheetData/>", "<sheetData/><sheetPr/>"] {
        assert_package(
            refused_body(body, |p| p.set_outline_summary(SHEET, Some(false), None)),
            SETTINGS_MESSAGE,
        );
    }
    for body in [
        "<sheetFormatPr defaultRowHeight=\"15\"/><sheetFormatPr defaultRowHeight=\"15\"/><sheetData/>",
        "<cols><col min=\"1\" max=\"1\" width=\"5\"/></cols><sheetFormatPr defaultRowHeight=\"15\"/><sheetData/>",
    ] {
        assert_package(refused_body(body, rows(2, 2, 1, false)), SETTINGS_MESSAGE);
        assert_package(
            refused_body(body, columns(2, 2, 1, false)),
            SETTINGS_MESSAGE,
        );
    }
    // The controls: one of each in its place is written, and a write that
    // sets no level leaves two `<sheetFormatPr>` as they are.
    assert!(
        edited(
            "<sheetPr/><sheetFormatPr defaultRowHeight=\"15\"/><sheetData/>",
            rows(2, 2, 1, false)
        )
        .contains("<sheetFormatPr outlineLevelRow=\"1\" defaultRowHeight=\"15\"/>")
    );
    let two = "<sheetFormatPr defaultRowHeight=\"15\"/><sheetFormatPr defaultRowHeight=\"15\"/><sheetData/>";
    assert_eq!(
        edited(two, |p| p
            .set_row_collapsed(SHEET, 2, true)
            .expect("recorded")),
        worksheet(
            "<sheetFormatPr defaultRowHeight=\"15\"/><sheetFormatPr defaultRowHeight=\"15\"/><sheetData><row r=\"2\" collapsed=\"1\"/></sheetData>"
        )
    );
    // A group or a summary with a shift on the part in the same write.
    type Edit = fn(&mut Package<Cursor<Vec<u8>>>);
    let edits: [Edit; 4] = [
        |p| p.group_rows(SHEET, 2, 3, 1, true).expect("recorded"),
        |p| p.group_columns(SHEET, 2, 3, 1, true).expect("recorded"),
        |p| p.set_row_collapsed(SHEET, 4, true).expect("recorded"),
        |p| p.set_outline_summary(SHEET, Some(false), None),
    ];
    for edit in edits {
        let (_, mut package) = opened(smallest());
        package.insert_rows(SHEET, 1, 1).expect("recorded");
        edit(&mut package);
        assert_package(refused(package), SHIFT_MESSAGE);
        // On a removed part, a replaced part and a part the archive lacks.
        let (_, mut package) = opened(smallest());
        edit(&mut package);
        package.remove_part(SHEET);
        assert_package(refused(package), "an edit to a removed part");
        let (_, mut package) = opened(smallest());
        edit(&mut package);
        package.replace_part(SHEET, replacement_from_reader(Cursor::new(b"x".to_vec())));
        assert_package(
            refused(package),
            "a cell edit to a part this write replaces",
        );
    }
    let (_, mut package) = opened(smallest());
    package
        .group_rows(b"xl/worksheets/sheet9.xml", 1, 1, 1, false)
        .expect("recorded");
    assert_package(refused(package), "no such part");
    let (_, mut package) = opened(smallest());
    package.set_outline_summary(b"xl/worksheets/sheet9.xml", Some(true), None);
    assert_package(refused(package), "no such part");
}

// --- through a shift ---

#[test]
fn a_group_is_kept_through_an_insertion_above_it_in_a_second_write() {
    let body = "<sheetData><row r=\"2\"><c r=\"A2\"><v>1</v></c></row><row r=\"3\"/></sheetData>";
    let edit = |p: &mut Package<Cursor<Vec<u8>>>| {
        p.group_rows(SHEET, 2, 3, 1, true).expect("recorded");
        p.set_row_collapsed(SHEET, 4, true).expect("recorded");
        p.group_columns(SHEET, 2, 3, 1, false).expect("recorded");
    };
    let first = edited(body, edit);
    let grouped = format!(
        "{}<cols><col min=\"2\" max=\"3\" outlineLevel=\"1\"/></cols><sheetData><row r=\"2\" hidden=\"1\" outlineLevel=\"1\"><c r=\"A2\"><v>1</v></c></row><row r=\"3\" hidden=\"1\" outlineLevel=\"1\"/><row r=\"4\" collapsed=\"1\"/></sheetData>",
        created_format(1, 1)
    );
    assert_eq!(first, worksheet(&grouped));
    // The second write, on the first's output: the rows move down with their
    // attributes, the columns and the maximums stay.
    let mut parts = smallest();
    set(&mut parts, SHEET, first);
    let output = edited_output(parts.clone(), |p| {
        p.insert_rows(SHEET, 1, 1).expect("recorded")
    });
    assert_untouched(&output, &parts, &[SHEET]);
    assert_eq!(
        String::from_utf8(stored_data(&output, SHEET)).expect("the part is UTF-8"),
        worksheet(&format!(
            "{}<cols><col min=\"2\" max=\"3\" outlineLevel=\"1\"/></cols><sheetData><row r=\"3\" hidden=\"1\" outlineLevel=\"1\"><c r=\"A3\"><v>1</v></c></row><row r=\"4\" hidden=\"1\" outlineLevel=\"1\"/><row r=\"5\" collapsed=\"1\"/></sheetData>",
            created_format(1, 1)
        ))
    );
}
