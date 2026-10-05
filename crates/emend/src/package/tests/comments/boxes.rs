//! Comment boxes on byte-built worksheets (`specs/0028-images-and-comments-
//! second-half.md`, "A comment's box" and "P1: a comment's box"): each test
//! builds the smallest package or the comments tests' fixtures, records a
//! comment with a box or reads one, and compares the written VML part with
//! the bytes the design gives or the read with the box the shape holds. The
//! Cargo feature `comments`.

use super::*;

/// The sample's box: 200 by 100 pixels, shown.
const SHOWN: CommentBox = CommentBox {
    width: 200,
    height: 100,
    visible: true,
};

/// openpyxl's default box, which the writer gives a comment with none set.
const DEFAULT: CommentBox = CommentBox {
    width: 144,
    height: 79,
    visible: false,
};

/// Records `shape` as the box of the comment on `cell` of the worksheet.
#[track_caller]
fn boxed(package: &mut Package<Cursor<Vec<u8>>>, cell: &str, shape: CommentBox) {
    package
        .set_comment_box(SHEET, at(cell), shape)
        .expect("recorded");
}

/// The note shape `specs/0022-comments-and-images.md` gives a comment on A1,
/// the first of a created VML part, written out: the bytes the writer gave
/// before boxes existed.
const A1_NOTE: &str = "<v:shape id=\"_x0000_s1025\" type=\"#_x0000_t202\" \
    style=\"position:absolute;margin-left:59.25pt;margin-top:1.5pt;width:108pt;\
    height:59.25pt;z-index:1;visibility:hidden\" fillcolor=\"#ffffe1\" o:insetmode=\"auto\">\
    <v:fill color2=\"#ffffe1\"/><v:shadow on=\"t\" color=\"black\" obscured=\"t\"/>\
    <v:path o:connecttype=\"none\"/><v:textbox style=\"mso-direction-alt:auto\">\
    <div style=\"text-align:left\"/></v:textbox><x:ClientData ObjectType=\"Note\">\
    <x:MoveWithCells/><x:SizeWithCells/><x:Anchor>1, 15, 0, 2, 3, 15, 4, 2</x:Anchor>\
    <x:AutoFill>False</x:AutoFill><x:Row>0</x:Row><x:Column>0</x:Column></x:ClientData>\
    </v:shape>";

/// [`A1_NOTE`] with the sample's box: `width:150pt;height:75pt`,
/// `visibility:visible` and the empty `x:Visible` child before `x:Row`.
const A1_SHOWN: &str = "<v:shape id=\"_x0000_s1025\" type=\"#_x0000_t202\" \
    style=\"position:absolute;margin-left:59.25pt;margin-top:1.5pt;width:150pt;\
    height:75pt;z-index:1;visibility:visible\" fillcolor=\"#ffffe1\" o:insetmode=\"auto\">\
    <v:fill color2=\"#ffffe1\"/><v:shadow on=\"t\" color=\"black\" obscured=\"t\"/>\
    <v:path o:connecttype=\"none\"/><v:textbox style=\"mso-direction-alt:auto\">\
    <div style=\"text-align:left\"/></v:textbox><x:ClientData ObjectType=\"Note\">\
    <x:MoveWithCells/><x:SizeWithCells/><x:Anchor>1, 15, 0, 2, 3, 15, 4, 2</x:Anchor>\
    <x:AutoFill>False</x:AutoFill><x:Visible/><x:Row>0</x:Row><x:Column>0</x:Column>\
    </x:ClientData></v:shape>";

/// A part of a written output, inflated when the write deflated it.
fn inflated(output: Vec<u8>, name: &[u8]) -> String {
    let mut package = Package::open(Cursor::new(output)).expect("open the output");
    let index = package.part(name).expect("the part");
    let mut text = String::new();
    package
        .read_part(index)
        .expect("read_part")
        .read_to_string(&mut text)
        .expect("UTF-8");
    text
}

/// The created VML part of a write of one comment on A1 of the smallest
/// package, after `edit`.
fn a1_vml(edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> String {
    inflated(
        edited_output(smallest(), edit),
        b"xl/drawings/vmlDrawing1.vml",
    )
}

#[test]
fn an_unset_box_and_the_default_box_write_the_shape_written_before() {
    let want = created_vml(1, A1_NOTE);
    // The helper the comments tests compare with is the same shape.
    assert_eq!(note(VOX, 1025, 1, 0, 0), A1_NOTE);
    assert_eq!(a1_vml(|p| add(p, "A1", "Ann", "one")), want);
    assert_eq!(
        a1_vml(|p| {
            add(p, "A1", "Ann", "one");
            boxed(p, "A1", DEFAULT);
        }),
        want
    );
}

#[test]
fn a_shown_box_of_200_by_100_writes_points_and_the_visible_child_before_row() {
    let vml = a1_vml(|p| {
        add(p, "A1", "Ann", "one");
        boxed(p, "A1", SHOWN);
    });
    assert_eq!(vml, created_vml(1, A1_SHOWN));
    assert!(vml.contains("width:150pt;height:75pt;"), "{vml}");
    assert!(vml.contains(";visibility:visible\""), "{vml}");
    assert!(
        vml.contains("</x:AutoFill><x:Visible/><x:Row>0</x:Row>"),
        "{vml}"
    );
}

#[test]
fn the_points_are_three_quarters_of_the_pixels_to_two_decimals_at_most() {
    for (pixels, points) in [
        (1, "0.75"),
        (2, "1.5"),
        (3, "2.25"),
        (4, "3"),
        (79, "59.25"),
        (144, "108"),
        (65535, "49151.25"),
    ] {
        let vml = a1_vml(|p| {
            add(p, "A1", "Ann", "one");
            boxed(
                p,
                "A1",
                CommentBox {
                    width: pixels,
                    height: 4,
                    visible: false,
                },
            );
        });
        assert!(
            vml.contains(&format!("width:{points}pt;height:3pt;")),
            "{pixels}: {vml}"
        );
    }
}

#[test]
fn the_box_set_before_or_after_the_addition_writes_the_same_bytes() {
    let after = a1_vml(|p| {
        add(p, "A1", "Ann", "one");
        boxed(p, "A1", SHOWN);
    });
    let before = a1_vml(|p| {
        boxed(p, "A1", SHOWN);
        add(p, "A1", "Ann", "one");
    });
    assert_eq!(before, after);
    assert_eq!(before, created_vml(1, A1_SHOWN));
}

#[test]
fn of_two_boxes_on_one_cell_the_last_wins() {
    let shown_last = a1_vml(|p| {
        add(p, "A1", "Ann", "one");
        boxed(p, "A1", DEFAULT);
        boxed(p, "A1", SHOWN);
    });
    assert_eq!(shown_last, created_vml(1, A1_SHOWN));
    let default_last = a1_vml(|p| {
        boxed(p, "A1", SHOWN);
        add(p, "A1", "Ann", "one");
        boxed(p, "A1", DEFAULT);
    });
    assert_eq!(default_last, created_vml(1, A1_NOTE));
}

#[test]
fn a_box_on_a_cell_this_write_adds_no_comment_on_is_refused_at_the_write() {
    let message = "no comment addition on that cell";
    // A box alone.
    assert_package(refused_with(smallest(), |p| boxed(p, "A1", SHOWN)), message);
    // A box on another cell than the addition's.
    assert_package(
        refused_with(smallest(), |p| {
            add(p, "A1", "Ann", "one");
            boxed(p, "B2", SHOWN);
        }),
        message,
    );
    // A box on a kept comment's cell, with no addition there.
    assert_package(
        refused_with(excel_both(), |p| boxed(p, "A1", SHOWN)),
        message,
    );
    // A box on a cell whose comment this write removes.
    assert_package(
        refused_with(excel_both(), |p| {
            remove(p, "A1");
            boxed(p, "A1", SHOWN);
        }),
        message,
    );
}

#[test]
fn the_call_refuses_a_box_out_of_range_and_records_nothing() {
    let shape = |width, height| CommentBox {
        width,
        height,
        visible: true,
    };
    let cases = [
        (at("A1"), shape(0, 100), "a comment box is out of range"),
        (at("A1"), shape(200, 0), "a comment box is out of range"),
        (at("A1"), shape(65536, 100), "a comment box is out of range"),
        (at("A1"), shape(200, 65536), "a comment box is out of range"),
        (
            CellRef {
                row: 1_048_577,
                col: 1,
            },
            SHOWN,
            "a row or column index or count is out of range",
        ),
    ];
    for (cell, shape, message) in cases {
        let (_, mut package) = opened(smallest());
        assert_package(
            package
                .set_comment_box(SHEET, cell, shape)
                .expect_err("refused"),
            message,
        );
        let out = written(package);
        assert_untouched(&out, &smallest(), &[]);
    }
    // The limits themselves are taken and read back.
    for limit in [shape(1, 1), shape(65535, 65535)] {
        let (_, mut package) = opened(smallest());
        add(&mut package, "A1", "Ann", "one");
        package
            .set_comment_box(SHEET, at("A1"), limit)
            .expect("recorded");
        let mut reopened = Package::open(Cursor::new(written(package))).expect("open the output");
        let read = reopened.comments(SHEET).expect("read back");
        assert_eq!(read[0].shape, Some(limit));
    }
}

/// A note shape for the zero-based `row` and `column` with the `style`
/// given and `extra` children in its `ClientData` before `x:Row`.
fn styled_note(id: &str, style: &str, extra: &str, row: u32, column: u32) -> String {
    format!(
        "<v:shape id=\"{id}\" type=\"#_x0000_t202\" style=\"{style}\" fillcolor=\"#ffffe1\">\
         <x:ClientData ObjectType=\"Note\"><x:MoveWithCells/>{extra}<x:Row>{row}</x:Row>\
         <x:Column>{column}</x:Column></x:ClientData></v:shape>"
    )
}

/// A worksheet whose comments part holds a comment on each of `cells` and
/// whose VML part holds `shapes`.
fn read_fixture(cells: &[&str], shapes: &str) -> Parts {
    let comments: String = cells
        .iter()
        .map(|cell| kept_comment(cell, 0, "note"))
        .collect();
    with_parts(
        sheet_r("<sheetData/><legacyDrawing r:id=\"rId2\"/>"),
        Some(&format!(
            "{}{}",
            comments_rel("rId1", "../comments1.xml"),
            vml_rel("rId2", "../drawings/vmlDrawing1.vml")
        )),
        &[
            (
                "xl/comments1.xml",
                comments_xml(&["Ann"], &comments),
                COMMENTS_TYPE,
            ),
            (
                "xl/drawings/vmlDrawing1.vml",
                excel_vml("1", shapes),
                VML_TYPE,
            ),
        ],
    )
}

#[test]
fn the_read_takes_points_and_pixels_the_visible_child_and_no_size() {
    let shapes = [
        // Points, Excel's form: four thirds, so the default reads back.
        styled_note(
            "_x0000_s1025",
            "position:absolute;margin-left:59.25pt;width:108pt;height:59.25pt;\
             z-index:1;visibility:hidden",
            "",
            0,
            0,
        ),
        // Pixels as they are, case folded names.
        styled_note("_x0000_s1026", "WIDTH: 200px; Height:100px", "", 1, 1),
        // No size: None.
        styled_note(
            "_x0000_s1027",
            "position:absolute;visibility:visible",
            "",
            2,
            2,
        ),
        // An `x:Visible` child and no visibility in the style.
        styled_note(
            "_x0000_s1028",
            "width:150pt;height:75pt",
            "<x:Visible/>",
            3,
            3,
        ),
        // A point value rounded to the nearest pixel, a visible style.
        styled_note(
            "_x0000_s1029",
            "width:100.5pt;height:0.38pt;visibility:visible",
            "",
            4,
            4,
        ),
        // A width alone, and a size in another unit: None.
        styled_note("_x0000_s1030", "width:100pt", "", 5, 5),
        styled_note("_x0000_s1031", "width:2in;height:1in", "", 6, 6),
    ]
    .concat();
    let parts = read_fixture(&["A1", "B2", "C3", "D4", "E5", "F6", "G7", "H8"], &shapes);
    let (_, mut package) = opened(parts);
    let read: Vec<Option<CommentBox>> = package
        .comments(SHEET)
        .expect("read")
        .into_iter()
        .map(|entry| entry.shape)
        .collect();
    let shape = |width, height, visible| {
        Some(CommentBox {
            width,
            height,
            visible,
        })
    };
    assert_eq!(
        read,
        [
            shape(144, 79, false),
            shape(200, 100, false),
            None,
            shape(200, 100, true),
            shape(134, 1, true),
            None,
            None,
            // H8 has no shape.
            None,
        ]
    );
}

#[test]
fn a_sheet_with_no_vml_part_reads_no_box() {
    let parts = with_parts(
        sheet_r("<sheetData/>"),
        Some(&comments_rel("rId1", "../comments1.xml")),
        &[(
            "xl/comments1.xml",
            comments_xml(&["Ann"], &kept_comment("A1", 0, "one")),
            COMMENTS_TYPE,
        )],
    );
    let (_, mut package) = opened(parts);
    let read = package.comments(SHEET).expect("read");
    assert_eq!(read.len(), 1);
    assert_eq!(read[0].shape, None);
}

#[test]
fn a_read_refuses_a_vml_part_it_cannot_read() {
    // The root's end tag is missing.
    let mut parts = read_fixture(&["A1"], "");
    let vml = part_text(&parts, b"xl/drawings/vmlDrawing1.vml").replace("</xml>", "");
    set(&mut parts, b"xl/drawings/vmlDrawing1.vml", vml);
    let (_, mut package) = opened(parts);
    assert_package(
        package.comments(SHEET).expect_err("refused"),
        "a comments part or its drawing cannot be read",
    );
}

#[test]
fn the_sample_s_box_round_trips_beside_kept_shapes_which_stay_byte_identical() {
    let parts = excel_both();
    let output = edited_output(parts.clone(), |p| {
        add(p, "B5", "Bob", "emend box");
        boxed(p, "B5", SHOWN);
    });
    let shown = A1_SHOWN
        .replace("_x0000_s1025", "_x0000_s1027")
        .replace("z-index:1;", "z-index:3;")
        .replace("1, 15, 0, 2, 3, 15, 4, 2", "2, 15, 3, 2, 4, 15, 7, 2")
        .replace(
            "<x:Row>0</x:Row><x:Column>0</x:Column>",
            "<x:Row>4</x:Row><x:Column>1</x:Column>",
        );
    assert_eq!(shown, {
        let mut want = note(VOX, 1027, 3, 4, 1)
            .replace("width:108pt;height:59.25pt;", "width:150pt;height:75pt;")
            .replace("visibility:hidden", "visibility:visible");
        want = want.replace("<x:Row>", "<x:Visible/><x:Row>");
        want
    });
    let vml = part_text(&parts, b"xl/drawings/vmlDrawing1.vml")
        .replace("</xml>", &format!("{shown}</xml>"));
    assert_eq!(text_of(&output, b"xl/drawings/vmlDrawing1.vml"), vml);
    let mut reopened = Package::open(Cursor::new(output)).expect("open the output");
    let read = reopened.comments(SHEET).expect("read back");
    assert_eq!(
        read.iter().map(|entry| entry.shape).collect::<Vec<_>>(),
        [None, None, Some(SHOWN)]
    );
    assert_eq!(read[2].text, "emend box");
}

#[cfg(feature = "richtext")]
#[test]
fn a_box_applies_to_a_rich_comment_addition() {
    use crate::cells::{RichText, Run};
    let text = RichText {
        runs: vec![Run {
            text: "rich".to_string(),
            font: None,
        }],
    };
    let (_, mut package) = opened(smallest());
    boxed(&mut package, "A1", SHOWN);
    package
        .add_rich_comment(SHEET, at("A1"), "Ann", &text)
        .expect("recorded");
    let output = written(package);
    assert!(
        inflated(output.clone(), b"xl/drawings/vmlDrawing1.vml")
            .contains("width:150pt;height:75pt;")
    );
    let mut reopened = Package::open(Cursor::new(output)).expect("open the output");
    let read = reopened.comments(SHEET).expect("read back");
    assert_eq!(read[0].shape, Some(SHOWN));
    assert_eq!(read[0].runs, Some(text));
}
