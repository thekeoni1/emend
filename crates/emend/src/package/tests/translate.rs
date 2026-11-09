//! The translator alone (`specs/0048-the-two-refusals.md`, "The
//! translator"): each case gives a formula's text, the offsets and the text
//! the rule writes.

use super::*;

const OFF: &str = "an unshared formula would leave the grid";

#[track_caller]
fn moved(text: &str, dr: i64, dc: i64) -> String {
    translated_formula(text, dr, dc).expect("translated")
}

#[test]
fn a_cell_and_an_area_move_by_both_offsets() {
    assert_eq!(moved("B2", 1, 0), "B3");
    assert_eq!(moved("B2", 0, 2), "D2");
    assert_eq!(moved("B2*2+C3", 2, 1), "C4*2+D5");
    assert_eq!(moved("SUM(A1:B2)", 3, 1), "SUM(B4:C5)");
    assert_eq!(moved("Z1+AA1", 0, 1), "AA1+AB1");
}

#[test]
fn a_dollar_holds_its_component_and_the_other_moves() {
    assert_eq!(moved("$B$2", 5, 5), "$B$2");
    assert_eq!(moved("$B2", 5, 5), "$B7");
    assert_eq!(moved("B$2", 5, 5), "G$2");
    assert_eq!(moved("SUM($A$1:B2)", 1, 1), "SUM($A$1:C3)");
}

#[test]
fn whole_columns_and_whole_rows_move_on_their_own_axis() {
    assert_eq!(moved("SUM(A:C)", 4, 1), "SUM(B:D)");
    assert_eq!(moved("SUM($A:C)", 4, 1), "SUM($A:D)");
    assert_eq!(moved("SUM(1:3)", 2, 7), "SUM(3:5)");
    assert_eq!(moved("SUM(1:$3)", 2, 7), "SUM(3:$3)");
}

#[test]
fn a_function_whose_name_reads_as_a_reference_is_not_one() {
    assert_eq!(moved("LOG10(A1)", 1, 1), "LOG10(B2)");
    assert_eq!(moved("ATAN2(A1,B1)", 1, 0), "ATAN2(A2,B2)");
}

#[test]
fn a_name_with_a_reference_like_tail_or_head_is_not_one() {
    assert_eq!(moved("Sales_A1+A1", 1, 0), "Sales_A1+A2");
    assert_eq!(moved("A1x+A1", 1, 0), "A1x+A2");
    assert_eq!(moved("x.A1+A1.y+A1", 1, 0), "x.A1+A1.y+A2");
    assert_eq!(moved("ABCD1+A1", 1, 0), "ABCD1+A2");
}

#[test]
fn a_string_holding_a_reference_is_skipped_with_its_escaped_quotes() {
    assert_eq!(moved("\"A1\"&A1", 1, 0), "\"A1\"&A2");
    assert_eq!(
        moved("\"say \"\"B2\"\" \"&B2", 1, 0),
        "\"say \"\"B2\"\" \"&B3"
    );
}

#[test]
fn a_sheet_quoted_or_bare_is_kept_and_its_reference_moves() {
    assert_eq!(moved("'It''s A1'!A1+1", 1, 1), "'It''s A1'!B2+1");
    assert_eq!(moved("'x'!A1:B2", 1, 0), "'x'!A2:B3");
    assert_eq!(moved("Sheet2!A1", 0, 1), "Sheet2!B1");
    assert_eq!(moved("B1!A1", 0, 1), "B1!B1");
}

#[test]
fn a_three_d_sheet_span_is_kept_and_its_reference_moves() {
    assert_eq!(moved("SUM(S1:S3!A1)", 1, 0), "SUM(S1:S3!A2)");
    assert_eq!(moved("SUM(Jan:Mar!B2:C3)", 1, 1), "SUM(Jan:Mar!C3:D4)");
}

#[test]
fn a_structured_reference_with_nested_brackets_is_skipped() {
    assert_eq!(
        moved("SUM(Sales[[#This Row],[A1]])+A1", 1, 0),
        "SUM(Sales[[#This Row],[A1]])+A2"
    );
    assert_eq!(moved("[1]Sheet1!A1", 1, 0), "[1]Sheet1!A2");
}

#[test]
fn error_literals_are_skipped() {
    assert_eq!(moved("IF(A1,#REF!,B1)", 1, 0), "IF(A2,#REF!,B2)");
    assert_eq!(moved("IFERROR(A1,#N/A)", 1, 0), "IFERROR(A2,#N/A)");
    assert_eq!(moved("#DIV/0!+A1", 1, 0), "#DIV/0!+A2");
}

#[test]
fn lowercase_letters_match_and_are_written_back_lowercase() {
    assert_eq!(moved("a1+$b2", 1, 1), "b2+$b3");
    assert_eq!(moved("sum(z1)", 0, 1), "sum(aa1)");
}

#[test]
fn negative_offsets_move_up_and_left() {
    assert_eq!(moved("C5", -2, -1), "B3");
    assert_eq!(moved("SUM(C:E)", 0, -2), "SUM(A:C)");
}

#[test]
fn a_text_with_no_relative_reference_is_returned_unchanged() {
    assert_eq!(moved("$A$1*2+\"B2\"", 3, 3), "$A$1*2+\"B2\"");
    assert_eq!(moved("1.5E+3+TRUE", 3, 3), "1.5E+3+TRUE");
    // A column past XFD or a row past 1048576 in the input is not a
    // reference, so it is not moved.
    assert_eq!(moved("XFE1+A1048577", 1, 1), "XFE1+A1048577");
}

#[test]
fn a_component_moved_below_one_is_refused() {
    assert_package(translated_formula("A1", -1, 0).unwrap_err(), OFF);
    assert_package(translated_formula("A1", 0, -1).unwrap_err(), OFF);
}

#[test]
fn a_component_moved_past_the_grid_is_refused() {
    assert_package(translated_formula("XFD1", 0, 1).unwrap_err(), OFF);
    assert_package(translated_formula("A1048576", 1, 0).unwrap_err(), OFF);
}
