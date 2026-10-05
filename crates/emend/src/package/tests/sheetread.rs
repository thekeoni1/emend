//! A worksheet's column and row formats and its settings read back as typed
//! values, and the shared strings table walked item by item, on byte-built
//! parts (`specs/0027-read-back.md`, "P2: the sheet, the lines and the
//! table", with "Planning rulings during P2" 1 and 2): each test builds the
//! smallest package with its worksheet holding a given body and compares
//! what the read gives with the value the bytes give. The Cargo feature
//! `read`.

use super::*;
use crate::styles::Color;

/// The package whose worksheet holds `body` between its root tags.
fn sheet_package(body: &str) -> Package<Cursor<Vec<u8>>> {
    let mut parts = smallest();
    set(&mut parts, SHEET, worksheet(body));
    opened(parts).1
}

#[track_caller]
fn settings(body: &str) -> SheetSettings {
    sheet_package(body).sheet_settings(SHEET).expect("read")
}

#[track_caller]
fn columns(body: &str) -> Vec<ColumnFormat> {
    sheet_package(body).columns(SHEET).expect("read")
}

#[track_caller]
fn rows(body: &str) -> Vec<RowFormat> {
    let mut package = sheet_package(body);
    package
        .rows(SHEET)
        .expect("opens")
        .collect::<Result<Vec<_>, _>>()
        .expect("read")
}

#[track_caller]
fn settings_refused(body: &str, message: &str) {
    match sheet_package(body).sheet_settings(SHEET) {
        Err(error) => assert_package(error, message),
        Ok(read) => panic!("read as {read:?}"),
    }
}

const UNREADABLE: &str = "a sheet setting cannot be read";

// --- columns and rows ---

#[test]
fn every_column_field_is_read_and_a_range_is_one_element() {
    assert_eq!(
        columns(
            "<cols><col min=\"1\" max=\"1\"/><col min=\"2\" max=\"5\" width=\"12.5\" \
             customWidth=\"1\" hidden=\"true\" style=\"3\" outlineLevel=\"7\" collapsed=\"1\" \
             bestFit=\"1\"/></cols><sheetData/>"
        ),
        vec![
            ColumnFormat {
                first: 1,
                last: 1,
                width: None,
                custom_width: false,
                hidden: false,
                style: None,
                outline_level: 0,
                collapsed: false,
            },
            ColumnFormat {
                first: 2,
                last: 5,
                width: Some(12.5),
                custom_width: true,
                hidden: true,
                style: Some(3),
                outline_level: 7,
                collapsed: true,
            },
        ]
    );
    assert_eq!(columns("<sheetData/>"), vec![]);
    assert_eq!(columns("<cols/><sheetData/>"), vec![]);
}

#[test]
fn every_row_field_is_read_and_a_row_without_r_is_numbered_as_cells_numbers_it() {
    let read = rows(
        "<sheetData><row r=\"2\" ht=\"20.25\" customHeight=\"1\" hidden=\"1\" s=\"4\" \
         customFormat=\"1\" outlineLevel=\"2\" collapsed=\"true\" spans=\"1:2\">\
         <c r=\"A2\"><v>1</v></c></row><row/><row r=\"9\"/><row><c><v>2</v></c></row></sheetData>",
    );
    assert_eq!(
        read[0],
        RowFormat {
            row: 2,
            height: Some(20.25),
            custom_height: true,
            hidden: true,
            style: Some(4),
            custom_format: true,
            outline_level: 2,
            collapsed: true,
        }
    );
    // "Planning rulings during P2", ruling 1: the previous row plus one.
    let numbers: Vec<u32> = read.iter().map(|row| row.row).collect();
    assert_eq!(numbers, vec![2, 3, 9, 10]);
    assert_eq!(
        read[1],
        RowFormat {
            row: 3,
            height: None,
            custom_height: false,
            hidden: false,
            style: None,
            custom_format: false,
            outline_level: 0,
            collapsed: false,
        }
    );
    // The cell reader numbers the same rows the same way.
    let mut package = sheet_package(
        "<sheetData><row r=\"2\"><c><v>1</v></c></row><row><c><v>2</v></c></row></sheetData>",
    );
    let cells: Vec<u32> = package
        .cells(SHEET)
        .expect("opens")
        .map(|cell| cell.expect("read").reference.row)
        .collect();
    assert_eq!(cells, vec![2, 3]);
    assert!(rows("<sheetData/>").is_empty());
}

// --- the settings ---

#[test]
fn a_part_holding_no_setting_reads_the_defaults() {
    assert_eq!(settings("<sheetData/>"), SheetSettings::default());
    let read = settings("<sheetData/>");
    assert_eq!(read.view, SheetView::default());
    assert_eq!(read.merges, vec![]);
    assert_eq!(read.protection, None);
    assert_eq!(read.outline_max, (0, 0));
}

#[test]
fn every_setting_is_read_from_a_part_holding_each_element() {
    let mut parts = smallest();
    set(
        &mut parts,
        SHEET,
        format!(
            "{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">\
             <sheetPr filterMode=\"0\"><tabColor theme=\"4\" tint=\"-0.25\"/>\
             <outlinePr summaryBelow=\"0\" summaryRight=\"1\"/><pageSetUpPr fitToPage=\"1\"/>\
             </sheetPr><dimension ref=\"A1:B2\"/>\
             <sheetViews><sheetView zoomScale=\"150\" showGridLines=\"0\" \
             showRowColHeaders=\"false\" showZeros=\"1\" showFormulas=\"1\" rightToLeft=\"0\" \
             view=\"pageLayout\" workbookViewId=\"0\"><pane xSplit=\"1\" ySplit=\"2\" \
             topLeftCell=\"B3\" state=\"frozen\"/><selection pane=\"bottomRight\"/></sheetView>\
             <sheetView workbookViewId=\"1\" zoomScale=\"20\"/></sheetViews>\
             <sheetFormatPr defaultRowHeight=\"15\" outlineLevelRow=\"2\" outlineLevelCol=\"1\"/>\
             <cols><col min=\"1\" max=\"1\"/></cols><sheetData/>\
             <sheetProtection password=\"CC1A\" sheet=\"1\" objects=\"1\" formatCells=\"0\"/>\
             <autoFilter ref=\"A1:B9\"/><mergeCells count=\"2\"><mergeCell ref=\"A1:B1\"/>\
             <mergeCell ref=\"C3:D4\"/></mergeCells>\
             <hyperlinks><hyperlink ref=\"A2\" r:id=\"rId1\" display=\"Site\" tooltip=\"Go\"/>\
             <hyperlink ref=\"C5:D6\" location=\"&apos;Sheet 1&apos;!A1\"/></hyperlinks>\
             <printOptions headings=\"1\" gridLines=\"0\" horizontalCentered=\"true\"/>\
             <pageMargins left=\"0.5\" right=\"0.5\" top=\"1\" bottom=\"1\" header=\"0.3\" \
             footer=\"0.3\"/><pageSetup paperSize=\"9\" scale=\"80\" fitToWidth=\"1\" \
             fitToHeight=\"0\" orientation=\"landscape\" pageOrder=\"overThenDown\" \
             blackAndWhite=\"1\" draft=\"0\" firstPageNumber=\"3\" useFirstPageNumber=\"1\" \
             copies=\"2\" r:id=\"rId2\"/><headerFooter differentFirst=\"1\">\
             <oddHeader>&amp;CTitle</oddHeader><firstFooter>&amp;P</firstFooter></headerFooter>\
             </worksheet>"
        ),
    );
    with_part(
        &mut parts,
        "xl/worksheets/_rels/sheet1.xml.rels",
        &rels(&format!(
            "<Relationship Id=\"rId1\" Type=\"{ATTR_NS}/hyperlink\" \
             Target=\"https://example.com/a?b=1&amp;c=2\" TargetMode=\"External\"/>"
        )),
        None,
    );
    let read = opened(parts).1.sheet_settings(SHEET).expect("read");
    let default = SheetProtection::default();
    assert_eq!(
        read,
        SheetSettings {
            tab_color: Some(Color::Theme {
                index: 4,
                tint: Some(-0.25)
            }),
            outline_summary: (Some(false), Some(true)),
            fit_to_page: Some(true),
            view: SheetView {
                zoom: Some(150),
                show_grid_lines: Some(false),
                show_headings: Some(false),
                show_zeros: Some(true),
                show_formulas: Some(true),
                right_to_left: Some(false),
                view: Some(ViewKind::PageLayout),
            },
            freeze: Some(at("B3")),
            print_options: PrintOptions {
                horizontal_centered: Some(true),
                vertical_centered: None,
                headings: Some(true),
                grid_lines: Some(false),
            },
            margins: Some(PageMargins {
                left: Some(0.5),
                right: Some(0.5),
                top: Some(1.0),
                bottom: Some(1.0),
                header: Some(0.3),
                footer: Some(0.3),
            }),
            page_setup: PageSetup {
                orientation: Some(Orientation::Landscape),
                paper_size: Some(9),
                scale: Some(80),
                fit_to_width: Some(1),
                fit_to_height: Some(0),
                fit_to_page: Some(true),
                first_page_number: Some(3),
                black_and_white: Some(true),
                draft: Some(false),
                copies: Some(2),
                page_order: Some(PageOrder::OverThenDown),
            },
            header_footer: HeaderFooter {
                odd_header: Some("&CTitle".into()),
                first_footer: Some("&P".into()),
                ..HeaderFooter::default()
            },
            auto_filter: Area::parse(b"A1:B9"),
            merges: vec![
                Area::parse(b"A1:B1").expect("area"),
                Area::parse(b"C3:D4").expect("area"),
            ],
            hyperlinks: vec![
                HyperlinkEntry {
                    cell: at("A2"),
                    target: Target::Url("https://example.com/a?b=1&c=2".into()),
                    display: Some("Site".into()),
                    tooltip: Some("Go".into()),
                },
                HyperlinkEntry {
                    cell: at("C5"),
                    target: Target::Location("'Sheet 1'!A1".into()),
                    display: None,
                    tooltip: None,
                },
            ],
            protection: Some(SheetProtection {
                sheet: true,
                objects: true,
                format_cells: false,
                ..default
            }),
            outline_max: (2, 1),
        }
    );
}

#[test]
fn a_split_pane_or_a_pane_after_the_first_view_reads_no_freeze() {
    assert_eq!(
        settings(
            "<sheetViews><sheetView workbookViewId=\"0\"><pane xSplit=\"2000\" \
             topLeftCell=\"C1\" state=\"split\"/></sheetView></sheetViews><sheetData/>"
        )
        .freeze,
        None
    );
    assert_eq!(
        settings(
            "<sheetViews><sheetView workbookViewId=\"0\"/><sheetView workbookViewId=\"1\">\
             <pane topLeftCell=\"C1\" state=\"frozen\"/></sheetView></sheetViews><sheetData/>"
        )
        .freeze,
        None
    );
}

#[test]
fn absent_protection_attributes_take_the_struct_s_defaults() {
    // "Planning rulings during P2", ruling 2: sixteen flags, each the
    // struct's default when absent.
    assert_eq!(
        settings("<sheetData/><sheetProtection/>").protection,
        Some(SheetProtection::default())
    );
    let read = settings(
        "<sheetData/><sheetProtection sheet=\"0\" objects=\"1\" scenarios=\"1\" \
         formatCells=\"0\" formatColumns=\"0\" formatRows=\"0\" insertColumns=\"0\" \
         insertRows=\"0\" insertHyperlinks=\"0\" deleteColumns=\"0\" deleteRows=\"0\" \
         selectLockedCells=\"1\" sort=\"0\" autoFilter=\"0\" pivotTables=\"0\" \
         selectUnlockedCells=\"1\"/>",
    );
    assert_eq!(
        read.protection,
        Some(SheetProtection {
            sheet: false,
            objects: true,
            scenarios: true,
            format_cells: false,
            format_columns: false,
            format_rows: false,
            insert_columns: false,
            insert_rows: false,
            insert_hyperlinks: false,
            delete_columns: false,
            delete_rows: false,
            select_locked_cells: true,
            sort: false,
            auto_filter: false,
            pivot_tables: false,
            select_unlocked_cells: true,
        })
    );
}

#[test]
fn a_first_page_number_without_its_switch_and_a_default_orientation_read_none() {
    let read = settings(
        "<sheetData/><pageSetup orientation=\"default\" firstPageNumber=\"4\" \
         useFirstPageNumber=\"0\"/>",
    );
    assert_eq!(read.page_setup, PageSetup::default());
}

// --- refusals ---

#[test]
fn two_of_an_element_the_writers_refuse_twice_are_refused_with_their_message() {
    let two = "a worksheet has two cols, sheetViews or mergeCells elements, or a sheetViews \
               without a sheetView";
    settings_refused(
        "<sheetData/><mergeCells><mergeCell ref=\"A1:B1\"/></mergeCells>\
         <mergeCells><mergeCell ref=\"A2:B2\"/></mergeCells>",
        two,
    );
    settings_refused(
        "<sheetViews><sheetView workbookViewId=\"0\"/></sheetViews>\
         <sheetViews><sheetView workbookViewId=\"0\"/></sheetViews><sheetData/>",
        two,
    );
    match sheet_package("<cols/><cols/><sheetData/>").columns(SHEET) {
        Err(error) => assert_package(error, two),
        Ok(read) => panic!("read as {read:?}"),
    }
    settings_refused(
        "<sheetData/><hyperlinks/><hyperlinks/>",
        "a worksheet has two hyperlinks or sheetProtection elements",
    );
    settings_refused(
        "<sheetData/><sheetProtection/><sheetProtection/>",
        "a worksheet has two hyperlinks or sheetProtection elements",
    );
    let settings_message = "a worksheet has two sheetPr, autoFilter, printOptions, pageMargins, \
                            pageSetup or headerFooter elements";
    for element in [
        "<sheetPr/>",
        "<sheetFormatPr/>",
        "<autoFilter ref=\"A1\"/>",
        "<printOptions/>",
        "<pageMargins/>",
        "<pageSetup/>",
        "<headerFooter/>",
    ] {
        settings_refused(&format!("{element}<sheetData/>{element}"), settings_message);
    }
    // Schema order is not checked at the read (ruling 2): one element out
    // of place is read.
    assert_eq!(
        settings("<sheetData/><sheetPr><tabColor auto=\"1\"/></sheetPr>").tab_color,
        Some(Color::Auto)
    );
}

#[test]
fn a_value_that_does_not_parse_or_lies_outside_the_writer_s_range_is_refused() {
    for body in [
        "<sheetViews><sheetView zoomScale=\"9\"/></sheetViews><sheetData/>",
        "<sheetViews><sheetView zoomScale=\"x\"/></sheetViews><sheetData/>",
        "<sheetViews><sheetView showZeros=\"2\"/></sheetViews><sheetData/>",
        "<sheetViews><sheetView view=\"wide\"/></sheetViews><sheetData/>",
        "<sheetViews><sheetView><pane topLeftCell=\"B0\" state=\"frozen\"/></sheetView>\
         </sheetViews><sheetData/>",
        "<sheetPr><tabColor theme=\"1\" tint=\"1.5\"/></sheetPr><sheetData/>",
        "<sheetPr><tabColor/></sheetPr><sheetData/>",
        "<sheetFormatPr outlineLevelRow=\"8\"/><sheetData/>",
        "<sheetData/><pageMargins left=\"-1\"/>",
        "<sheetData/><pageMargins left=\"inf\"/>",
        "<sheetData/><pageSetup scale=\"401\"/>",
        "<sheetData/><pageSetup paperSize=\"0\"/>",
        "<sheetData/><pageSetup copies=\"0\"/>",
        "<sheetData/><pageSetup orientation=\"sideways\"/>",
        "<sheetData/><pageSetup pageOrder=\"any\"/>",
        "<sheetData/><printOptions gridLines=\"yes\"/>",
        "<sheetData/><sheetProtection sort=\"maybe\"/>",
    ] {
        settings_refused(body, UNREADABLE);
    }
    let long = "x".repeat(256);
    settings_refused(
        &format!("<sheetData/><headerFooter><oddHeader>{long}</oddHeader></headerFooter>"),
        UNREADABLE,
    );
    for body in [
        "<cols><col max=\"1\"/></cols><sheetData/>",
        "<cols><col min=\"1\" max=\"1\" width=\"256\"/></cols><sheetData/>",
        "<cols><col min=\"1\" max=\"1\" outlineLevel=\"8\"/></cols><sheetData/>",
        "<cols><col min=\"1\" max=\"1\" style=\"-1\"/></cols><sheetData/>",
    ] {
        match sheet_package(body).columns(SHEET) {
            Err(error) => assert_package(error, UNREADABLE),
            Ok(read) => panic!("{body} read as {read:?}"),
        }
    }
    for body in [
        "<sheetData><row r=\"1\" ht=\"410\"/></sheetData>",
        "<sheetData><row r=\"1\" hidden=\"2\"/></sheetData>",
        "<sheetData><row r=\"1\" s=\"x\"/></sheetData>",
    ] {
        let mut package = sheet_package(body);
        let first = package.rows(SHEET).expect("opens").next().expect("an item");
        assert_package(first.expect_err("refused"), UNREADABLE);
    }
}

#[test]
fn a_ref_that_is_not_an_area_is_refused() {
    for body in [
        "<sheetData/><mergeCells><mergeCell ref=\"A1:\"/></mergeCells>",
        "<sheetData/><mergeCells><mergeCell/></mergeCells>",
        "<sheetData/><autoFilter ref=\"1A\"/>",
        "<sheetData/><autoFilter/>",
        "<sheetData/><hyperlinks><hyperlink ref=\"Z\" location=\"A1\"/></hyperlinks>",
    ] {
        settings_refused(body, UNREADABLE);
    }
}

#[test]
fn a_hyperlink_naming_no_relationship_or_no_target_is_refused() {
    let mut parts = smallest();
    set(
        &mut parts,
        SHEET,
        format!(
            "{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\"><sheetData/>\
             <hyperlinks><hyperlink ref=\"A1\" r:id=\"rId9\"/></hyperlinks></worksheet>"
        ),
    );
    match opened(parts).1.sheet_settings(SHEET) {
        Err(error) => assert_package(error, UNREADABLE),
        Ok(read) => panic!("read as {read:?}"),
    }
    settings_refused(
        "<sheetData/><hyperlinks><hyperlink ref=\"A1\"/></hyperlinks>",
        UNREADABLE,
    );
    settings_refused(
        "<sheetData/><hyperlinks><hyperlink ref=\"A1\" location=\"\"/></hyperlinks>",
        UNREADABLE,
    );
}

#[test]
fn a_part_that_is_not_a_worksheet_is_refused_by_each_read() {
    let root = "the worksheet part's root is not worksheet";
    let mut parts = smallest();
    set(
        &mut parts,
        SHEET,
        format!("{DECLARATION}\n<chartsheet xmlns=\"{MAIN_NS}\"/>"),
    );
    let (_, mut package) = opened(parts);
    assert_package(package.sheet_settings(SHEET).expect_err("refused"), root);
    assert_package(package.columns(SHEET).expect_err("refused"), root);
    let first = package.rows(SHEET).expect("opens").next().expect("an item");
    assert_package(first.expect_err("refused"), root);
    assert_package(
        package
            .sheet_settings(b"xl/worksheets/none.xml")
            .expect_err("refused"),
        "no such part",
    );
}

// --- the table ---

/// The smallest package with a shared strings part holding `items`.
fn with_items(items: &str) -> Package<Cursor<Vec<u8>>> {
    let mut parts = smallest();
    set(
        &mut parts,
        b"xl/_rels/workbook.xml.rels",
        rels(&format!(
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
        )),
    );
    with_part(
        &mut parts,
        "xl/sharedStrings.xml",
        &format!("{DECLARATION}\n<sst xmlns=\"{MAIN_NS}\">{items}</sst>"),
        Some("application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml"),
    );
    opened(parts).1
}

#[test]
fn the_items_are_the_held_table_s_one_at_a_time() {
    let items = "<si><t>one</t></si><si/><si><r><rPr><b/></rPr><t>tw</t></r><r><t>o</t></r>\
                 </si><si><t xml:space=\"preserve\"> three\r\n</t><rPh><t>x</t></rPh></si>";
    let mut package = with_items(items);
    let held = package.shared_strings().expect("the table reads");
    let walked: Vec<crate::cells::Item> = package
        .shared_string_items()
        .expect("opens")
        .collect::<Result<_, _>>()
        .expect("read");
    assert_eq!(walked.len(), 4);
    assert_eq!(u32::try_from(walked.len()).ok(), Some(held.len()));
    for (at, item) in (0u32..).zip(&walked) {
        assert_eq!(Some(item.text.as_str()), held.get(at), "item {at}");
        #[cfg(feature = "richtext")]
        assert_eq!(item.runs.as_ref(), held.runs_of(at), "item {at}");
    }
    let texts: Vec<&str> = walked.iter().map(|item| item.text.as_str()).collect();
    assert_eq!(texts, vec!["one", "", "two", " three\n"]);
}

#[test]
fn a_workbook_without_a_table_yields_nothing_and_a_wrong_root_is_refused() {
    let (_, mut package) = opened(smallest());
    assert_eq!(package.shared_string_items().expect("opens").count(), 0);
    let mut parts = smallest();
    set(
        &mut parts,
        b"xl/_rels/workbook.xml.rels",
        rels(&format!(
            "{}{}",
            relationship(
                "rId1",
                &format!("{ATTR_NS}/worksheet"),
                "worksheets/sheet1.xml"
            ),
            relationship(
                "rId3",
                &format!("{ATTR_NS}/sharedStrings"),
                "sharedStrings.xml"
            ),
        )),
    );
    with_part(
        &mut parts,
        "xl/sharedStrings.xml",
        &format!("{DECLARATION}\n<strings xmlns=\"{MAIN_NS}\"/>"),
        None,
    );
    let (_, mut package) = opened(parts);
    let mut items = package.shared_string_items().expect("opens");
    assert_package(
        items.next().expect("an item").expect_err("refused"),
        "the shared strings part's root is not sst",
    );
    assert!(items.next().is_none(), "spent after an error");
}

// --- the writers' forms read back ---

/// Part 3's and part 4's written forms read back equal to what was written
/// (`specs/0027-read-back.md`, "P2": "part 3's and part 4's written forms
/// among them"). The Cargo features `sheetprops`, `metadata` and `rowcol`
/// besides `read`.
#[cfg(all(feature = "sheetprops", feature = "metadata", feature = "rowcol"))]
#[test]
fn the_writers_forms_read_back_as_written() {
    let view = SheetView {
        zoom: Some(85),
        show_grid_lines: Some(false),
        show_headings: Some(true),
        show_zeros: None,
        show_formulas: Some(false),
        right_to_left: None,
        view: Some(ViewKind::PageBreakPreview),
    };
    let options = PrintOptions {
        horizontal_centered: Some(true),
        vertical_centered: Some(false),
        headings: None,
        grid_lines: Some(true),
    };
    let margins = PageMargins {
        left: Some(0.25),
        right: Some(0.25),
        top: Some(0.5),
        bottom: Some(0.5),
        header: Some(0.2),
        footer: Some(0.2),
    };
    let setup = PageSetup {
        orientation: Some(Orientation::Landscape),
        paper_size: Some(8),
        scale: Some(90),
        fit_to_width: Some(2),
        fit_to_height: Some(0),
        fit_to_page: Some(true),
        first_page_number: Some(5),
        black_and_white: Some(true),
        draft: None,
        copies: Some(3),
        page_order: Some(PageOrder::OverThenDown),
    };
    let texts = HeaderFooter {
        odd_header: Some("&LLeft&RRight".into()),
        even_footer: Some("&P of &N".into()),
        ..HeaderFooter::default()
    };
    let output = edited_output(
        with_rows("<row r=\"1\"><c r=\"A1\"><v>1</v></c></row>"),
        |p| {
            p.set_tab_color(SHEET, Some(Color::Rgb(0xFF11_2233)))
                .expect("recorded");
            p.set_sheet_view(SHEET, &view).expect("recorded");
            p.freeze_panes(SHEET, Some(at("C4"))).expect("recorded");
            p.set_print_options(SHEET, &options).expect("recorded");
            p.set_page_margins(SHEET, &margins).expect("recorded");
            p.set_page_setup(SHEET, &setup).expect("recorded");
            p.set_header_footer(SHEET, &texts).expect("recorded");
            p.set_auto_filter(SHEET, Area::parse(b"A1:C9"))
                .expect("recorded");
            p.merge_cells(SHEET, Area::parse(b"E1:F2").expect("area"))
                .expect("recorded");
            p.set_hyperlink(
                SHEET,
                at("A1"),
                LinkTarget::External("https://example.com/"),
                Some("tip"),
            )
            .expect("recorded");
            p.protect_sheet(SHEET, Some("pw"), SheetProtection::default())
                .expect("recorded");
            p.group_rows(SHEET, 3, 4, 2, true).expect("recorded");
            p.group_columns(SHEET, 2, 3, 1, false).expect("recorded");
            p.set_outline_summary(SHEET, Some(false), None);
        },
    );
    let mut package = Package::open(Cursor::new(output)).expect("the output opens");
    let read = package.sheet_settings(SHEET).expect("read");
    assert_eq!(read.tab_color, Some(Color::Rgb(0xFF11_2233)));
    assert_eq!(read.view, view);
    assert_eq!(read.freeze, Some(at("C4")));
    assert_eq!(read.print_options, options);
    assert_eq!(read.margins, Some(margins));
    assert_eq!(read.page_setup, setup);
    assert_eq!(read.fit_to_page, Some(true));
    assert_eq!(read.header_footer, texts);
    assert_eq!(read.auto_filter, Area::parse(b"A1:C9"));
    assert_eq!(read.merges, vec![Area::parse(b"E1:F2").expect("area")]);
    assert_eq!(
        read.hyperlinks,
        vec![HyperlinkEntry {
            cell: at("A1"),
            target: Target::Url("https://example.com/".into()),
            display: None,
            tooltip: Some("tip".into()),
        }]
    );
    assert_eq!(read.protection, Some(SheetProtection::default()));
    assert_eq!(read.outline_summary, (Some(false), None));
    assert_eq!(read.outline_max, (2, 1));
    let rows: Vec<RowFormat> = package
        .rows(SHEET)
        .expect("opens")
        .collect::<Result<_, _>>()
        .expect("read");
    let grouped: Vec<(u32, u8, bool)> = rows
        .iter()
        .filter(|row| row.outline_level > 0)
        .map(|row| (row.row, row.outline_level, row.hidden))
        .collect();
    assert_eq!(grouped, vec![(3, 2, true), (4, 2, true)]);
    let columns = package.columns(SHEET).expect("read");
    assert_eq!(
        columns
            .iter()
            .map(|col| (col.first, col.last, col.outline_level))
            .collect::<Vec<_>>(),
        vec![(2, 3, 1)]
    );
}
