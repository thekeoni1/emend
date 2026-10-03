//! Sheet settings on byte-built worksheets
//! (`specs/0025-sheet-and-print-settings.md`, "P1: the sheet properties and
//! the view", "The pass", and "Planning rulings during P1"): each test
//! builds the smallest package, records a tab colour or a sheet view, and
//! compares the written worksheet with the bytes the design gives. The Cargo
//! feature `sheetprops`.

use super::*;
use crate::styles::Color;

/// The worksheet part after `edit` on the smallest package whose worksheet
/// holds `body`; every other part is asserted untouched.
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

const RED: Color = Color::Rgb(0xFFFF_0000);

/// A tab colour of `color` recorded on the smallest package's worksheet.
fn tab(color: Option<Color>) -> impl FnOnce(&mut Package<Cursor<Vec<u8>>>) {
    move |p| p.set_tab_color(SHEET, color).expect("recorded")
}

/// A sheet view of `view` recorded on the smallest package's worksheet.
fn view(view: SheetView) -> impl FnOnce(&mut Package<Cursor<Vec<u8>>>) {
    move |p| p.set_sheet_view(SHEET, &view).expect("recorded")
}

#[test]
fn a_tab_colour_in_each_form_creates_sheet_pr_before_dimension() {
    let body = "<dimension ref=\"A1\"/><sheetData/>";
    for (color, attributes) in [
        (RED, "rgb=\"FFFF0000\""),
        (
            Color::Theme {
                index: 4,
                tint: Some(-0.25),
            },
            "theme=\"4\" tint=\"-0.25\"",
        ),
        (
            Color::Theme {
                index: 5,
                tint: None,
            },
            "theme=\"5\"",
        ),
        (Color::Indexed(10), "indexed=\"10\""),
        (Color::Auto, "auto=\"1\""),
    ] {
        assert_eq!(
            edited(body, tab(Some(color))),
            worksheet(&format!(
                "<sheetPr><tabColor {attributes}/></sheetPr><dimension ref=\"A1\"/><sheetData/>"
            )),
            "{color:?}"
        );
    }
}

#[test]
fn a_created_sheet_pr_takes_the_root_s_prefix() {
    let mut parts = smallest();
    let prefixed =
        format!("{DECLARATION}\n<x:worksheet xmlns:x=\"{MAIN_NS}\"><x:sheetData/></x:worksheet>");
    set(&mut parts, SHEET, prefixed);
    let output = edited_output(parts.clone(), tab(Some(RED)));
    assert_eq!(
        String::from_utf8(stored_data(&output, SHEET)).expect("UTF-8"),
        format!(
            "{DECLARATION}\n<x:worksheet xmlns:x=\"{MAIN_NS}\"><x:sheetPr><x:tabColor rgb=\"FFFF0000\"/></x:sheetPr><x:sheetData/></x:worksheet>"
        )
    );
    // An empty root is opened up for it.
    let mut parts = smallest();
    set(
        &mut parts,
        SHEET,
        format!("{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\"/>"),
    );
    let output = edited_output(parts, tab(Some(RED)));
    assert_eq!(
        String::from_utf8(stored_data(&output, SHEET)).expect("UTF-8"),
        worksheet("<sheetPr><tabColor rgb=\"FFFF0000\"/></sheetPr>")
    );
}

#[test]
fn a_tab_colour_goes_first_and_keeps_the_sheet_pr_s_attributes_and_children() {
    let cases = [
        (
            "<sheetPr codeName=\"Sheet1\" filterMode=\"0\"><pageSetUpPr fitToPage=\"1\"/></sheetPr>",
            "<sheetPr codeName=\"Sheet1\" filterMode=\"0\"><tabColor rgb=\"FFFF0000\"/><pageSetUpPr fitToPage=\"1\"/></sheetPr>",
        ),
        (
            "<sheetPr codeName=\"Sheet1\">\n  <outlinePr summaryBelow=\"0\"/>\n  <pageSetUpPr fitToPage=\"1\"/>\n</sheetPr>",
            "<sheetPr codeName=\"Sheet1\">\n  <tabColor rgb=\"FFFF0000\"/><outlinePr summaryBelow=\"0\"/>\n  <pageSetUpPr fitToPage=\"1\"/>\n</sheetPr>",
        ),
        // An empty `<sheetPr>` with an attribute is opened up.
        (
            "<sheetPr codeName=\"Sheet1\"/>",
            "<sheetPr codeName=\"Sheet1\"><tabColor rgb=\"FFFF0000\"/></sheetPr>",
        ),
    ];
    for (before, after) in cases {
        assert_eq!(
            edited(&format!("{before}<sheetData/>"), tab(Some(RED))),
            worksheet(&format!("{after}<sheetData/>")),
            "{before}"
        );
    }
}

#[test]
fn a_kept_tab_colour_is_replaced() {
    assert_eq!(
        edited(
            "<sheetPr codeName=\"a\"><tabColor theme=\"1\" tint=\"0.5\"/><pageSetUpPr/></sheetPr><sheetData/>",
            tab(Some(Color::Indexed(3)))
        ),
        worksheet(
            "<sheetPr codeName=\"a\"><tabColor indexed=\"3\"/><pageSetUpPr/></sheetPr><sheetData/>"
        )
    );
}

#[test]
fn none_drops_the_tab_colour_and_a_sheet_pr_left_empty_with_it() {
    let cases = [
        ("<sheetPr><tabColor rgb=\"FF000000\"/></sheetPr>", ""),
        ("<sheetPr>\n  <tabColor rgb=\"FF000000\"/>\n</sheetPr>", ""),
        (
            "<sheetPr><tabColor rgb=\"FF000000\"/><pageSetUpPr fitToPage=\"1\"/></sheetPr>",
            "<sheetPr><pageSetUpPr fitToPage=\"1\"/></sheetPr>",
        ),
        (
            "<sheetPr codeName=\"a\"><tabColor rgb=\"FF000000\"/></sheetPr>",
            "<sheetPr codeName=\"a\"></sheetPr>",
        ),
        // Nothing to drop: the element as it is, and none created.
        ("<sheetPr codeName=\"a\"/>", "<sheetPr codeName=\"a\"/>"),
        ("", ""),
    ];
    for (before, after) in cases {
        assert_eq!(
            edited(&format!("{before}<sheetData/>"), tab(None)),
            worksheet(&format!("{after}<sheetData/>")),
            "{before}"
        );
    }
}

/// The corpus's sixteen `<sheetView>` attributes, each with a value, in the
/// order the corpus writes them.
const SIXTEEN: [(&str, &str); 16] = [
    ("showFormulas", "0"),
    ("showGridLines", "1"),
    ("showRowColHeaders", "1"),
    ("showZeros", "1"),
    ("rightToLeft", "0"),
    ("tabSelected", "1"),
    ("showOutlineSymbols", "1"),
    ("defaultGridColor", "1"),
    ("view", "normal"),
    ("topLeftCell", "A1"),
    ("colorId", "64"),
    ("zoomScale", "100"),
    ("zoomScaleNormal", "100"),
    ("zoomScalePageLayoutView", "100"),
    ("zoomScaleSheetLayoutView", "100"),
    ("workbookViewId", "0"),
];

/// A worksheet body whose first `<sheetView>` carries `attributes` and a
/// selection, with a second view after it.
fn views_with(attributes: &str) -> String {
    format!(
        "<sheetViews><sheetView{attributes}><selection activeCell=\"B2\" sqref=\"B2\"/></sheetView><sheetView workbookViewId=\"1\"/></sheetViews><sheetData/>"
    )
}

/// ` name="value"` for each pair.
fn attributes_of(pairs: &[(&str, &str)]) -> String {
    pairs
        .iter()
        .map(|(name, value)| format!(" {name}=\"{value}\""))
        .collect()
}

#[test]
fn each_view_field_is_set_first_and_every_other_attribute_kept_in_order() {
    let before = views_with(&attributes_of(&SIXTEEN));
    let fields: [(SheetView, &str, &str); 7] = [
        (
            SheetView {
                zoom: Some(85),
                ..SheetView::default()
            },
            "zoomScale",
            "85",
        ),
        (
            SheetView {
                show_grid_lines: Some(false),
                ..SheetView::default()
            },
            "showGridLines",
            "0",
        ),
        (
            SheetView {
                show_headings: Some(false),
                ..SheetView::default()
            },
            "showRowColHeaders",
            "0",
        ),
        (
            SheetView {
                show_zeros: Some(false),
                ..SheetView::default()
            },
            "showZeros",
            "0",
        ),
        (
            SheetView {
                show_formulas: Some(true),
                ..SheetView::default()
            },
            "showFormulas",
            "1",
        ),
        (
            SheetView {
                right_to_left: Some(true),
                ..SheetView::default()
            },
            "rightToLeft",
            "1",
        ),
        (
            SheetView {
                view: Some(ViewKind::PageBreakPreview),
                ..SheetView::default()
            },
            "view",
            "pageBreakPreview",
        ),
    ];
    for (fields, name, value) in fields {
        let kept: Vec<(&str, &str)> = SIXTEEN
            .iter()
            .copied()
            .filter(|(other, _)| *other != name)
            .collect();
        let want = views_with(&format!(" {name}=\"{value}\"{}", attributes_of(&kept)));
        assert_eq!(edited(&before, view(fields)), worksheet(&want), "{name}");
    }
    // All seven at once, in the schema's order, the nine others after.
    let all = SheetView {
        zoom: Some(85),
        show_grid_lines: Some(false),
        show_headings: Some(false),
        show_zeros: Some(false),
        show_formulas: Some(true),
        right_to_left: Some(true),
        view: Some(ViewKind::PageLayout),
    };
    let set = [
        "showFormulas",
        "showGridLines",
        "showRowColHeaders",
        "showZeros",
        "rightToLeft",
        "view",
        "zoomScale",
    ];
    let kept: Vec<(&str, &str)> = SIXTEEN
        .iter()
        .copied()
        .filter(|(name, _)| !set.contains(name))
        .collect();
    let want = views_with(&format!(
        " showFormulas=\"1\" showGridLines=\"0\" showRowColHeaders=\"0\" showZeros=\"0\" rightToLeft=\"1\" view=\"pageLayout\" zoomScale=\"85\"{}",
        attributes_of(&kept)
    ));
    assert_eq!(edited(&before, view(all)), worksheet(&want));
    // The kind's third value.
    let normal = SheetView {
        view: Some(ViewKind::Normal),
        ..SheetView::default()
    };
    assert_eq!(
        edited(&views_with(" workbookViewId=\"0\""), view(normal)),
        worksheet(&views_with(" view=\"normal\" workbookViewId=\"0\""))
    );
}

#[test]
fn a_view_with_no_field_set_leaves_the_view_as_it_is() {
    let body = views_with(" tabSelected=\"1\" workbookViewId=\"0\"");
    assert_eq!(edited(&body, view(SheetView::default())), worksheet(&body));
    // And creates nothing on a part with no views.
    assert_eq!(
        edited("<sheetData/>", view(SheetView::default())),
        worksheet("<sheetData/>")
    );
}

#[test]
fn a_part_with_no_sheet_views_gains_one_in_schema_position() {
    let zoomed = SheetView {
        zoom: Some(85),
        show_grid_lines: Some(false),
        ..SheetView::default()
    };
    assert_eq!(
        edited("<dimension ref=\"A1\"/><sheetData/>", view(zoomed)),
        worksheet(
            "<dimension ref=\"A1\"/><sheetViews><sheetView showGridLines=\"0\" zoomScale=\"85\" workbookViewId=\"0\"/></sheetViews><sheetData/>"
        )
    );
    // With a tab colour too: `<sheetPr>` first, the views after `<dimension>`.
    assert_eq!(
        edited("<dimension ref=\"A1\"/><sheetData/>", |p| {
            p.set_tab_color(SHEET, Some(RED)).expect("recorded");
            p.set_sheet_view(SHEET, &zoomed).expect("recorded");
        }),
        worksheet(
            "<sheetPr><tabColor rgb=\"FFFF0000\"/></sheetPr><dimension ref=\"A1\"/><sheetViews><sheetView showGridLines=\"0\" zoomScale=\"85\" workbookViewId=\"0\"/></sheetViews><sheetData/>"
        )
    );
}

const PANE_B2: &str = "<pane xSplit=\"1\" ySplit=\"1\" topLeftCell=\"B2\" activePane=\"bottomRight\" state=\"frozen\"/><selection pane=\"bottomRight\"/>";

#[test]
fn a_view_and_a_freeze_in_one_write_both_apply_the_pane_after_the_attributes() {
    let zoomed = SheetView {
        zoom: Some(120),
        ..SheetView::default()
    };
    let both = |p: &mut Package<Cursor<Vec<u8>>>| {
        p.freeze_panes(SHEET, Some(at("B2"))).expect("recorded");
        p.set_sheet_view(SHEET, &zoomed).expect("recorded");
    };
    assert_eq!(
        edited(&views_with(" tabSelected=\"1\" workbookViewId=\"0\""), both),
        worksheet(&format!(
            "<sheetViews><sheetView zoomScale=\"120\" tabSelected=\"1\" workbookViewId=\"0\">{PANE_B2}</sheetView><sheetView workbookViewId=\"1\"/></sheetViews><sheetData/>"
        ))
    );
    // An empty view opened up; and a created one.
    assert_eq!(
        edited(
            "<sheetViews><sheetView workbookViewId=\"0\"/></sheetViews><sheetData/>",
            both
        ),
        worksheet(&format!(
            "<sheetViews><sheetView zoomScale=\"120\" workbookViewId=\"0\">{PANE_B2}</sheetView></sheetViews><sheetData/>"
        ))
    );
    assert_eq!(
        edited("<sheetData/>", both),
        worksheet(&format!(
            "<sheetViews><sheetView zoomScale=\"120\" workbookViewId=\"0\">{PANE_B2}</sheetView></sheetViews><sheetData/>"
        ))
    );
    // A view alone keeps the pane and selections the part has.
    let frozen = format!(
        "<sheetViews><sheetView workbookViewId=\"0\">{PANE_B2}</sheetView></sheetViews><sheetData/>"
    );
    assert_eq!(
        edited(&frozen, view(zoomed)),
        worksheet(&format!(
            "<sheetViews><sheetView zoomScale=\"120\" workbookViewId=\"0\">{PANE_B2}</sheetView></sheetViews><sheetData/>"
        ))
    );
}

#[test]
fn two_calls_on_one_part_each_set_the_fields_they_give() {
    let body = views_with(" workbookViewId=\"0\"");
    let got = edited(&body, |p| {
        p.set_sheet_view(
            SHEET,
            &SheetView {
                zoom: Some(50),
                show_grid_lines: Some(false),
                ..SheetView::default()
            },
        )
        .expect("recorded");
        p.set_sheet_view(
            SHEET,
            &SheetView {
                zoom: Some(75),
                show_zeros: Some(false),
                ..SheetView::default()
            },
        )
        .expect("recorded");
        // The last tab colour wins.
        p.set_tab_color(SHEET, Some(Color::Indexed(1)))
            .expect("recorded");
        p.set_tab_color(SHEET, Some(RED)).expect("recorded");
    });
    assert_eq!(
        got,
        worksheet(
            "<sheetPr><tabColor rgb=\"FFFF0000\"/></sheetPr><sheetViews><sheetView showGridLines=\"0\" showZeros=\"0\" zoomScale=\"75\" workbookViewId=\"0\"><selection activeCell=\"B2\" sqref=\"B2\"/></sheetView><sheetView workbookViewId=\"1\"/></sheetViews><sheetData/>"
        )
    );
}

#[test]
fn a_setting_out_of_range_is_refused_at_the_call_and_records_nothing() {
    let (parts, mut package) = opened(smallest());
    for zoom in [0, 9, 401, u32::MAX] {
        let zoomed = SheetView {
            zoom: Some(zoom),
            ..SheetView::default()
        };
        assert_package(
            package.set_sheet_view(SHEET, &zoomed).expect_err("refused"),
            "a sheet setting is out of range",
        );
    }
    for tint in [1.5, -1.5, f64::NAN, f64::INFINITY] {
        let tinted = Color::Theme {
            index: 1,
            tint: Some(tint),
        };
        assert_package(
            package
                .set_tab_color(SHEET, Some(tinted))
                .expect_err("refused"),
            "a sheet setting is out of range",
        );
    }
    // Nothing recorded: the write is the input.
    assert_untouched(&written(package), &parts, &[]);
    // The bounds themselves are accepted.
    let (_, mut package) = opened(smallest());
    for zoom in [10, 400] {
        let zoomed = SheetView {
            zoom: Some(zoom),
            ..SheetView::default()
        };
        package.set_sheet_view(SHEET, &zoomed).expect("in range");
    }
    for tint in [1.0, -1.0] {
        let tinted = Color::Theme {
            index: 1,
            tint: Some(tint),
        };
        package
            .set_tab_color(SHEET, Some(tinted))
            .expect("in range");
    }
}

#[test]
fn two_sheet_pr_or_one_out_of_schema_order_are_refused() {
    let message = "a worksheet has two sheetPr, autoFilter, printOptions, pageMargins, pageSetup or headerFooter elements";
    for body in [
        "<sheetPr/><sheetPr/><sheetData/>",
        "<sheetPr codeName=\"a\"><tabColor rgb=\"FF000000\"/></sheetPr><sheetPr/><sheetData/>",
        "<dimension ref=\"A1\"/><sheetPr/><sheetData/>",
        "<sheetData/><sheetPr/>",
    ] {
        for color in [Some(RED), None] {
            assert_package(refused_body(body, tab(color)), message);
        }
    }
    // The control: one `<sheetPr>` in its place is written.
    assert_eq!(
        edited(
            "<sheetPr/><dimension ref=\"A1\"/><sheetData/>",
            tab(Some(RED))
        ),
        worksheet(
            "<sheetPr><tabColor rgb=\"FFFF0000\"/></sheetPr><dimension ref=\"A1\"/><sheetData/>"
        )
    );
}

#[test]
fn a_view_on_two_sheet_views_or_one_without_a_view_is_refused() {
    let message = "a worksheet has two cols, sheetViews or mergeCells elements, or a sheetViews without a sheetView";
    let zoomed = SheetView {
        zoom: Some(85),
        ..SheetView::default()
    };
    for body in [
        "<sheetViews/><sheetData/>",
        "<sheetViews><sheetView workbookViewId=\"0\"/></sheetViews><sheetViews><sheetView workbookViewId=\"0\"/></sheetViews><sheetData/>",
    ] {
        assert_package(refused_body(body, view(zoomed)), message);
    }
}

#[test]
fn a_setting_on_a_removed_replaced_or_absent_part_is_refused() {
    type Edit = fn(&mut Package<Cursor<Vec<u8>>>);
    let edits: [Edit; 2] = [
        |p| p.set_tab_color(SHEET, Some(RED)).expect("recorded"),
        |p| {
            p.set_sheet_view(
                SHEET,
                &SheetView {
                    zoom: Some(85),
                    ..SheetView::default()
                },
            )
            .expect("recorded")
        },
    ];
    for edit in edits {
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
    // A part the archive lacks is recorded and refused at the write
    // ("Planning rulings during P1", ruling 1).
    let (_, mut package) = opened(smallest());
    package
        .set_tab_color(b"xl/worksheets/sheet9.xml", Some(RED))
        .expect("recorded");
    assert_package(refused(package), "no such part");
}

#[cfg(feature = "rowcol")]
#[test]
fn a_setting_on_a_part_with_a_pending_shift_is_refused() {
    let shift = "a worksheet with a row or column insertion or deletion takes no other edit in the same write";
    let (_, mut package) = opened(smallest());
    package.insert_rows(SHEET, 1, 1).expect("recorded");
    package.set_tab_color(SHEET, Some(RED)).expect("recorded");
    assert_package(refused(package), shift);
    let (_, mut package) = opened(smallest());
    package.delete_columns(SHEET, 1, 1).expect("recorded");
    package
        .set_sheet_view(SHEET, &SheetView::default())
        .expect("recorded");
    assert_package(refused(package), shift);
}

#[test]
fn a_tab_colour_on_a_sheet_added_in_the_same_write_lands_on_its_blank_worksheet() {
    // "Planning rulings during P1", ruling 1.
    let (_, mut package) = opened(smallest());
    let part = package.add_sheet("Two").expect("add_sheet");
    package.set_tab_color(&part, Some(RED)).expect("recorded");
    let out = written(package);
    let mut reopened = Package::open(Cursor::new(out)).expect("open the output");
    let index = reopened.part(&part).expect("the added worksheet");
    let mut text = String::new();
    reopened
        .read_part(index)
        .expect("read_part")
        .read_to_string(&mut text)
        .expect("UTF-8");
    assert_eq!(
        text,
        worksheet("<sheetPr><tabColor rgb=\"FFFF0000\"/></sheetPr><sheetData/>")
    );
}

#[test]
fn a_pending_sheet_removed_drops_its_settings() {
    let (parts, mut package) = opened(smallest());
    let part = package.add_sheet("Two").expect("add");
    package.set_tab_color(&part, Some(RED)).expect("recorded");
    package
        .set_sheet_view(&part, &SheetView::default())
        .expect("recorded");
    package.remove_sheet("Two").expect("remove");
    let out = written(package);
    assert_names(&out, &part_names(&parts));
    assert_untouched(&out, &parts, &[]);
}
