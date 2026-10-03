//! Rich text on byte-built worksheets and shared strings tables
//! (`specs/0024-rich-text.md`, "P1", and the P1 rulings): each test builds
//! the smallest package with its worksheet holding given rows and, where it
//! says so, a shared strings part, records a rich value or reads one, and
//! compares the written worksheet with the bytes the design gives or the read
//! with the runs the part holds. The tests that go through `edited_sheet`
//! also assert every other part untouched.

use super::*;
use crate::cells::{Cell, Kind, RichText, Run};
use crate::styles::{Color, Font, Scheme, Underline, VerticalAlign};

/// The content type of a shared strings part.
const STRINGS_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml";

/// A run of `text` with `font`.
fn run(text: &str, font: Option<Font>) -> Run {
    Run {
        text: text.to_string(),
        font,
    }
}

/// A rich text of `runs`.
fn rich(runs: Vec<Run>) -> RichText {
    RichText { runs }
}

/// A bold font with nothing else.
fn bold() -> Font {
    Font {
        bold: true,
        ..Font::default()
    }
}

/// The cells of the worksheet of `output`, read back through the crate.
#[track_caller]
fn cells_back(output: Vec<u8>) -> Vec<Cell> {
    let mut package = Package::open(Cursor::new(output)).expect("the output opens");
    package
        .cells(SHEET)
        .expect("the worksheet opens")
        .collect::<Result<Vec<_>, _>>()
        .expect("the worksheet reads")
}

/// The output of `value` set at A1 of an empty row 1.
#[track_caller]
fn written_value(value: Value) -> Vec<u8> {
    edited_output(with_rows("<row r=\"1\"></row>"), |p| {
        p.set_cell(SHEET, at("A1"), value)
    })
}

/// The runs a write of `value` at A1 reads back as, with the cell's value.
#[track_caller]
fn round_trip(value: RichText) -> (Option<RichText>, Option<String>) {
    let cells = cells_back(written_value(Value::Rich(value)));
    assert_eq!(cells.len(), 1, "one cell written");
    let cell = cells.into_iter().next().expect("one cell");
    assert_eq!(cell.kind, Kind::InlineString, "an inline string");
    (cell.runs, cell.value)
}

/// The smallest package with a shared strings part holding `items` (the
/// `<si>` elements as given) and its relationship and `Override`.
fn with_strings(items: &str) -> Parts {
    let mut parts = smallest();
    let workbook_rels = rels(&format!(
        "{}{}{}",
        relationship(
            "rId1",
            &format!("{ATTR_NS}/worksheet"),
            "worksheets/sheet1.xml"
        ),
        relationship("rId2", &format!("{ATTR_NS}/styles"), "styles.xml"),
        relationship(
            "rId3",
            &format!("{ATTR_NS}/sharedStrings"),
            "sharedStrings.xml"
        ),
    ));
    set(&mut parts, b"xl/_rels/workbook.xml.rels", workbook_rels);
    with_part(
        &mut parts,
        "xl/sharedStrings.xml",
        &format!("{DECLARATION}\n<sst xmlns=\"{MAIN_NS}\">{items}</sst>"),
        Some(STRINGS_TYPE),
    );
    parts
}

/// The table of a package whose shared strings part holds `items`.
#[track_caller]
fn table_of(items: &str) -> SharedStrings {
    let (_, mut package) = opened(with_strings(items));
    package.shared_strings().expect("the table reads")
}

/// What the read of a table holding `items` is refused with.
#[track_caller]
fn table_refused(items: &str) -> Error {
    let (_, mut package) = opened(with_strings(items));
    package
        .shared_strings()
        .expect_err("the table was accepted")
}

/// What the read of a sheet whose row 1 holds `cell` is refused with.
#[track_caller]
fn cell_refused(cell: &str) -> Error {
    let (_, mut package) = opened(with_rows(&format!("<row r=\"1\">{cell}</row>")));
    let mut cells = package.cells(SHEET).expect("the worksheet opens");
    match cells.next() {
        Some(Err(error)) => error,
        other => panic!("the cell was not refused: {other:?}"),
    }
}

/// An inline string cell at A1 whose `<is>` holds `body`.
fn inline(body: &str) -> String {
    format!("<c r=\"A1\" t=\"inlineStr\"><is>{body}</is></c>")
}

/// The runs of the inline string cell at A1 whose `<is>` holds `body`.
#[track_caller]
fn inline_runs(body: &str) -> (Option<RichText>, Option<String>) {
    let (_, mut package) = opened(with_rows(&format!("<row r=\"1\">{}</row>", inline(body))));
    let cell = package
        .cells(SHEET)
        .expect("the worksheet opens")
        .next()
        .expect("a cell")
        .expect("the cell reads");
    (cell.runs, cell.value)
}

#[test]
fn a_rich_value_is_written_inline_one_r_per_run_and_reads_back_equal() {
    let value = rich(vec![run("emend", Some(bold())), run(" rich", None)]);
    let got = edited_sheet("<row r=\"1\"></row>", |p| {
        p.set_cell(SHEET, at("A1"), Value::Rich(value.clone()))
    });
    assert_eq!(
        got,
        sheet_with(
            "<row r=\"1\"><c r=\"A1\" t=\"inlineStr\"><is><r><rPr><b/></rPr>\
             <t xml:space=\"preserve\">emend</t></r><r><t xml:space=\"preserve\"> rich</t></r>\
             </is></c></row>"
        )
    );
    let (runs, text) = round_trip(value.clone());
    assert_eq!(runs, Some(value.clone()), "the runs read back equal");
    assert_eq!(text.as_deref(), Some("emend rich"), "the flattened value");
    assert_eq!(value.text(), "emend rich", "RichText::text is the same");
}

#[test]
fn every_font_field_is_written_in_the_measured_order_and_reads_back() {
    let font = Font {
        name: Some("Calibri & Co".to_string()),
        size: Some(11.5),
        bold: true,
        italic: true,
        strike: true,
        underline: Some(Underline::Double),
        vertical_align: Some(VerticalAlign::Superscript),
        color: Some(Color::Theme {
            index: 1,
            tint: Some(0.25),
        }),
        family: Some(2),
        scheme: Some(Scheme::Minor),
    };
    let value = rich(vec![run("x", Some(font))]);
    let got = edited_sheet("<row r=\"1\"></row>", |p| {
        p.set_cell(SHEET, at("A1"), Value::Rich(value.clone()))
    });
    assert_eq!(
        got,
        sheet_with(
            "<row r=\"1\"><c r=\"A1\" t=\"inlineStr\"><is><r><rPr><b/><i/><strike/>\
             <u val=\"double\"/><vertAlign val=\"superscript\"/><sz val=\"11.5\"/>\
             <color theme=\"1\" tint=\"0.25\"/><rFont val=\"Calibri &amp; Co\"/>\
             <family val=\"2\"/><scheme val=\"minor\"/></rPr><t xml:space=\"preserve\">x</t>\
             </r></is></c></row>"
        )
    );
    assert_eq!(round_trip(value.clone()).0, Some(value));
}

#[test]
fn each_underline_alignment_colour_and_scheme_reads_back_as_written() {
    let mut fonts = Vec::new();
    for underline in [
        Underline::Single,
        Underline::Double,
        Underline::SingleAccounting,
        Underline::DoubleAccounting,
    ] {
        fonts.push(Font {
            underline: Some(underline),
            ..Font::default()
        });
    }
    for align in [
        VerticalAlign::Superscript,
        VerticalAlign::Subscript,
        VerticalAlign::Baseline,
    ] {
        fonts.push(Font {
            vertical_align: Some(align),
            ..Font::default()
        });
    }
    for color in [
        Color::Rgb(0xFF12_34AB),
        Color::Theme {
            index: 4,
            tint: None,
        },
        Color::Theme {
            index: 0,
            tint: Some(-0.5),
        },
        Color::Indexed(10),
        Color::Auto,
    ] {
        fonts.push(Font {
            color: Some(color),
            ..Font::default()
        });
    }
    for scheme in [Scheme::Major, Scheme::Minor] {
        fonts.push(Font {
            scheme: Some(scheme),
            ..Font::default()
        });
    }
    for font in fonts {
        let value = rich(vec![run("y", Some(font))]);
        assert_eq!(round_trip(value.clone()).0, Some(value));
    }
}

#[test]
fn a_default_font_is_written_as_an_empty_rpr() {
    let got = edited_sheet("<row r=\"1\"></row>", |p| {
        p.set_cell(
            SHEET,
            at("A1"),
            Value::Rich(rich(vec![run("a", Some(Font::default()))])),
        )
    });
    assert_eq!(
        got,
        sheet_with(
            "<row r=\"1\"><c r=\"A1\" t=\"inlineStr\"><is><r><rPr/>\
             <t xml:space=\"preserve\">a</t></r></is></c></row>"
        )
    );
    assert_eq!(
        round_trip(rich(vec![run("a", Some(Font::default()))])).0,
        Some(rich(vec![run("a", Some(Font::default()))]))
    );
}

#[test]
fn a_leading_space_a_carriage_return_and_a_tab_survive() {
    let value = rich(vec![run(" lead", None), run("a\rb\tc", Some(bold()))]);
    let got = edited_sheet("<row r=\"1\"></row>", |p| {
        p.set_cell(SHEET, at("A1"), Value::Rich(value.clone()))
    });
    assert!(
        got.contains("<t xml:space=\"preserve\"> lead</t>"),
        "the leading space kept: {got}"
    );
    assert!(
        got.contains("<t xml:space=\"preserve\">a&#13;b\tc</t>"),
        "the carriage return as a reference, the tab as it is: {got}"
    );
    assert_eq!(round_trip(value.clone()).0, Some(value));
}

#[test]
fn a_prefixed_worksheet_gets_its_runs_in_its_prefix() {
    let mut parts = smallest();
    set(
        &mut parts,
        SHEET,
        format!(
            "{DECLARATION}\n<x:worksheet xmlns:x=\"{MAIN_NS}\"><x:sheetData>\
             <x:row r=\"1\"></x:row></x:sheetData></x:worksheet>"
        ),
    );
    let value = rich(vec![run("p", Some(bold()))]);
    let output = edited_output(parts, |p| {
        p.set_cell(SHEET, at("A1"), Value::Rich(value.clone()))
    });
    let sheet = String::from_utf8(stored_data(&output, SHEET)).expect("UTF-8");
    assert!(
        sheet.contains(
            "<x:c r=\"A1\" t=\"inlineStr\"><x:is><x:r><x:rPr><x:b/></x:rPr>\
             <x:t xml:space=\"preserve\">p</x:t></x:r></x:is></x:c>"
        ),
        "{sheet}"
    );
    let cells = cells_back(output);
    assert_eq!(cells[0].runs, Some(value));
}

#[test]
fn a_replaced_cell_keeps_its_style_and_loses_its_type() {
    let got = edited_sheet(
        "<row r=\"1\"><c r=\"A1\" s=\"3\" t=\"n\"><v>1</v></c></row>",
        |p| p.set_cell(SHEET, at("A1"), Value::Rich(rich(vec![run("r", None)]))),
    );
    assert_eq!(
        got,
        sheet_with(
            "<row r=\"1\"><c r=\"A1\" s=\"3\" t=\"inlineStr\"><is><r>\
             <t xml:space=\"preserve\">r</t></r></is></c></row>"
        )
    );
}

#[test]
fn the_write_refuses_each_rule() {
    let cases: Vec<(RichText, &str)> = vec![
        (rich(Vec::new()), "a rich text is not valid"),
        (
            rich(vec![run("a", None), run("", None)]),
            "a rich text is not valid",
        ),
        (
            rich(vec![run("a\u{1}", None)]),
            "a text holds a character XML cannot carry",
        ),
        (
            rich(vec![run(
                "a",
                Some(Font {
                    name: Some(String::new()),
                    ..Font::default()
                }),
            )]),
            "a font name is empty or holds a character XML cannot carry",
        ),
        (
            rich(vec![run(
                "a",
                Some(Font {
                    size: Some(0.0),
                    ..Font::default()
                }),
            )]),
            "a style field is out of range",
        ),
        (
            rich(vec![run(
                "a",
                Some(Font {
                    color: Some(Color::Theme {
                        index: 0,
                        tint: Some(1.5),
                    }),
                    ..Font::default()
                }),
            )]),
            "a style field is out of range",
        ),
    ];
    for (value, message) in cases {
        let error = refused_edit("<row r=\"1\"></row>", |p| {
            p.set_cell(SHEET, at("A1"), Value::Rich(value))
        });
        assert_package(error, message);
    }
}

#[test]
fn a_rich_value_off_the_grid_is_refused() {
    let error = refused_edit("<row r=\"1\"></row>", |p| {
        p.set_cell(
            SHEET,
            CellRef {
                row: 1_048_577,
                col: 1,
            },
            Value::Rich(rich(vec![run("a", None)])),
        )
    });
    assert_package(error, "a row or column index or count is out of range");
}

#[test]
fn a_table_item_with_runs_reads_through_runs_of_and_get() {
    let table = table_of(
        "<si><t>plain</t></si>\
         <si><r><rPr><b/><sz val=\"11\"/><color rgb=\"FFFF0000\"/><rFont val=\"Arial\"/>\
         <family val=\"2\"/></rPr><t>one</t></r><r><t xml:space=\"preserve\"> two</t></r></si>\
         <si/>",
    );
    assert_eq!(table.len(), 3);
    assert_eq!(table.runs_of(0), None, "an item without runs");
    assert_eq!(table.get(1), Some("one two"), "get flattens");
    assert_eq!(
        table.runs_of(1),
        Some(&rich(vec![
            run(
                "one",
                Some(Font {
                    name: Some("Arial".to_string()),
                    size: Some(11.0),
                    bold: true,
                    color: Some(Color::Rgb(0xFFFF_0000)),
                    family: Some(2),
                    ..Font::default()
                })
            ),
            run(" two", None),
        ]))
    );
    assert_eq!(table.runs_of(2), None, "an empty item");
    assert_eq!(table.runs_of(3), None, "past the end");
}

#[test]
fn a_plain_t_before_the_runs_reads_as_a_first_run_with_no_font() {
    let table = table_of("<si><t>lead </t><r><rPr><i/></rPr><t>run</t></r></si>");
    assert_eq!(table.get(0), Some("lead run"));
    assert_eq!(
        table.runs_of(0),
        Some(&rich(vec![
            run("lead ", None),
            run(
                "run",
                Some(Font {
                    italic: true,
                    ..Font::default()
                })
            ),
        ]))
    );
    let (runs, text) = inline_runs("<t>lead </t><r><t>run</t></r>");
    assert_eq!(text.as_deref(), Some("lead run"));
    assert_eq!(runs, Some(rich(vec![run("lead ", None), run("run", None)])));
}

#[test]
fn an_inline_string_without_runs_reads_none() {
    let (runs, text) = inline_runs("<t>only</t>");
    assert_eq!(text.as_deref(), Some("only"));
    assert_eq!(runs, None);
}

#[test]
fn an_empty_run_is_dropped_at_the_read() {
    // P1 verification ruling 4: an `<r/>`, and a run whose `<t>` texts total
    // empty, is dropped, its font with it; a container whose every run is
    // empty reads with `runs` `None`. The shared strings table first.
    let table = table_of(
        "<si><r/><r><rPr><b/></rPr><t>a</t></r><r><rPr><i/></rPr><t></t><t/></r>\
         <r><t>b</t></r><r><rPr><b/></rPr></r></si>\
         <si><r/><r><rPr><b/></rPr><t></t></r></si>\
         <si><t>lead</t><r/></si>\
         <si><r><t>c</t></r><t></t></si>",
    );
    assert_eq!(table.get(0), Some("ab"), "get flattens as before");
    assert_eq!(
        table.runs_of(0),
        Some(&rich(vec![run("a", Some(bold())), run("b", None)])),
        "the empty runs are dropped"
    );
    assert_eq!(table.get(1), Some(""));
    assert_eq!(table.runs_of(1), None, "every run empty reads as no runs");
    assert_eq!(table.get(2), Some("lead"));
    assert_eq!(
        table.runs_of(2),
        Some(&rich(vec![run("lead", None)])),
        "a plain <t> before the runs is a run with text, so the item keeps it"
    );
    assert_eq!(
        table.runs_of(3),
        Some(&rich(vec![run("c", None)])),
        "an empty <t> after the runs is no run"
    );
    // The inline string.
    let (runs, text) = inline_runs("<r/><r><rPr><b/></rPr><t>a</t></r><r><t></t></r>");
    assert_eq!(text.as_deref(), Some("a"));
    assert_eq!(runs, Some(rich(vec![run("a", Some(bold()))])));
    let (runs, text) = inline_runs("<r/><r><rPr><i/></rPr><t/></r>");
    assert_eq!(text.as_deref(), Some(""));
    assert_eq!(runs, None, "every run empty reads as no runs");
}

#[test]
fn a_phonetic_run_inside_a_rich_item_is_skipped() {
    let table = table_of(
        "<si><r><t>ab</t></r><rPh sb=\"0\" eb=\"1\"><t>skipped</t></rPh>\
         <phoneticPr fontId=\"0\"/></si>",
    );
    assert_eq!(table.get(0), Some("ab"));
    assert_eq!(table.runs_of(0), Some(&rich(vec![run("ab", None)])));
}

#[test]
fn false_flags_and_no_underline_read_as_off() {
    let table = table_of(
        "<si><r><rPr><b val=\"0\"/><i val=\"false\"/><strike val=\"1\"/><u val=\"none\"/>\
         <scheme val=\"none\"/></rPr><t>a</t></r><r><rPr><b val=\"true\"/><u/></rPr>\
         <t>b</t></r></si>",
    );
    assert_eq!(
        table.runs_of(0),
        Some(&rich(vec![
            run(
                "a",
                Some(Font {
                    strike: true,
                    ..Font::default()
                })
            ),
            run(
                "b",
                Some(Font {
                    bold: true,
                    underline: Some(Underline::Single),
                    ..Font::default()
                })
            ),
        ]))
    );
}

#[test]
fn the_properties_font_has_no_field_for_are_dropped_from_the_typed_view() {
    let table = table_of(
        "<si><r><rPr><charset val=\"238\"/><outline/><shadow val=\"1\"/><condense/>\
         <extend val=\"0\"/><b/></rPr><t>a</t></r></si>",
    );
    assert_eq!(table.runs_of(0), Some(&rich(vec![run("a", Some(bold()))])));
}

#[test]
fn a_run_property_that_cannot_be_read_is_refused() {
    let unreadable = "a run's properties cannot be read";
    for rpr in [
        "<b val=\"yes\"/>",
        "<u val=\"wavy\"/>",
        "<vertAlign/>",
        "<vertAlign val=\"middle\"/>",
        "<sz val=\"big\"/>",
        "<sz/>",
        "<family val=\"x\"/>",
        "<scheme val=\"other\"/>",
        "<rFont/>",
        "<color/>",
        "<color rgb=\"FF0000\"/>",
        "<color rgb=\"GG000000\"/>",
        "<color theme=\"a\"/>",
        "<color indexed=\"-1\"/>",
        "<color auto=\"maybe\"/>",
        "<name val=\"Arial\"/>",
        "<unknown/>",
        "<b><i/></b>",
        "<o:b xmlns:o=\"urn:other\"/>",
    ] {
        let item = format!("<si><r><rPr>{rpr}</rPr><t>a</t></r></si>");
        assert_package(table_refused(&item), unreadable);
        let body = format!("<r><rPr>{rpr}</rPr><t>a</t></r>");
        assert_package(cell_refused(&inline(&body)), unreadable);
    }
}

#[test]
fn an_unbound_prefix_on_a_run_property_is_refused_by_the_prefix_rule() {
    // specs/0024 P3 verification ruling 4: a prefix nothing binds on an
    // `<rPr>` child is refused by part 2's rule before the run reader sees
    // the child, so the table's read gives part 2's message, not this part's.
    assert_package(
        table_refused("<si><r><rPr><o:b/></rPr><t>a</t></r></si>"),
        "an element or attribute uses a prefix its document does not bind",
    );
    // The control: the same child with its prefix bound reaches the run
    // reader, which refuses a child outside the main namespace.
    assert_package(
        table_refused("<si><r><rPr><o:b xmlns:o=\"urn:other\"/></rPr><t>a</t></r></si>"),
        "a run's properties cannot be read",
    );
}

#[test]
fn a_value_or_colour_the_write_would_refuse_is_refused_at_the_read() {
    for rpr in [
        "<sz val=\"NaN\"/>",
        "<sz val=\"inf\"/>",
        "<sz val=\"0\"/>",
        "<sz val=\"-1\"/>",
        "<color theme=\"1\" tint=\"NaN\"/>",
        "<color theme=\"1\" tint=\"1.5\"/>",
        "<color theme=\"1\" tint=\"-2\"/>",
        "<rFont val=\"\"/>",
    ] {
        let item = format!("<si><r><rPr>{rpr}</rPr><t>a</t></r></si>");
        assert_package(table_refused(&item), "a run's properties cannot be read");
    }
}

#[test]
fn runs_compare_their_floats_by_bits() {
    let sized = |size: f64| {
        run(
            "a",
            Some(Font {
                size: Some(size),
                ..Font::default()
            }),
        )
    };
    assert_eq!(sized(f64::NAN), sized(f64::NAN), "a NaN size equals itself");
    assert_ne!(sized(0.0), sized(-0.0), "0.0 and -0.0 differ");
    assert_eq!(sized(11.0), sized(11.0));
    let tinted = |tint: f64| {
        rich(vec![run(
            "a",
            Some(Font {
                color: Some(Color::Theme {
                    index: 1,
                    tint: Some(tint),
                }),
                ..Font::default()
            }),
        )])
    };
    assert_eq!(
        tinted(f64::NAN),
        tinted(f64::NAN),
        "a NaN tint equals itself"
    );
    assert_ne!(tinted(0.0), tinted(-0.0));
    assert_ne!(run("a", None), run("a", Some(Font::default())));
    assert_ne!(run("a", None), run("b", None));
}
