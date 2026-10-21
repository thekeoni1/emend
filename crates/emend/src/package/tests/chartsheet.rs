//! Chartsheets on byte-built packages (`specs/0038-chartsheets.md`, "P1:
//! the reader and the writer", decisions 1 to 3 A, P1 rulings A to C;
//! `specs/0039-chart-kinds-and-formatting.md`, "P1", a surface's):
//! each test builds the smallest package, adds a chartsheet with
//! `add_chartsheet` or holds one built by hand, writes, compares the parts
//! the design names byte for byte, and reads every chartsheet written back
//! through `charts`, field by field. Every content is fictional. The Cargo
//! feature `charts`; the image and comment refusals, `images` and
//! `comments` too.

use super::*;
use crate::package::{
    Anchor, BarDirection, ChartAnchor, ChartEntry, ChartKind, Data, EditAs, Grouping, ImageAnchor,
    LegendPosition, NewChart, NewPlot, NewSeries, NewSeriesName, Plot, Series, SeriesName,
    SheetKind,
};

/// The content types and relationship types the chartsheet parts take.
const CHARTSHEET_TYPE: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml";
const DRAWING_TYPE: &str = "application/vnd.openxmlformats-officedocument.drawing+xml";
const CHART_TYPE: &str = "application/vnd.openxmlformats-officedocument.drawingml.chart+xml";
const XDR_NS: &str = "http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing";
const A_NS: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";
const C_NS: &str = "http://schemas.openxmlformats.org/drawingml/2006/chart";

/// The parts a chartsheet added to the smallest package takes.
const CHARTSHEET: &[u8] = b"xl/chartsheets/sheet1.xml";
const CHARTSHEET_RELS: &[u8] = b"xl/chartsheets/_rels/sheet1.xml.rels";
const DRAWING: &[u8] = b"xl/drawings/drawing1.xml";
const DRAWING_RELS: &[u8] = b"xl/drawings/_rels/drawing1.xml.rels";
const CHART1: &[u8] = b"xl/charts/chart1.xml";

/// The part's three messages, and the ones it shares.
const ROOT: &str = "the sheet part's root is not worksheet or chartsheet";
const ABSOLUTE: &str = "a chartsheet's chart takes an absolute anchor";
const ONE: &str = "a chartsheet holds one chart";
const SIZE: &str = "a chart size does not apply to its anchor";
#[cfg(any(feature = "images", feature = "comments"))]
const NOT_WORKSHEET: &str = "the worksheet part's root is not worksheet";
const NO_CHART_AT: &str = "no chart at that position";

/// The two series of the column chart: a text name with categories, and a
/// reference name with none.
const TWO_SERIES: [NewSeries<'static>; 2] = [
    NewSeries {
        name: Some(NewSeriesName::Text("North")),
        categories: Some("Sheet1!$A$2:$A$4"),
        values: "Sheet1!$B$2:$B$4",
        sizes: None,
        color: None,
        labels: None,
        marker: None,
        line: None,
        points: &[],
        trendline: None,
        error_bars: &[],
    },
    NewSeries {
        name: Some(NewSeriesName::Reference("Sheet1!$C$1")),
        categories: None,
        values: "Sheet1!$C$2:$C$4",
        sizes: None,
        color: None,
        labels: None,
        marker: None,
        line: None,
        points: &[],
        trendline: None,
        error_bars: &[],
    },
];

/// A clustered column chart of the two series with a title, both axis
/// titles and a legend at the bottom, at the sample's 960 by 640 pixels.
fn column_chart() -> NewChart<'static> {
    NewChart {
        plot: NewPlot::Bar {
            direction: BarDirection::Column,
            grouping: Grouping::Clustered,
        },
        title: Some("Sales"),
        series: &TWO_SERIES,
        category_axis_title: Some("Month"),
        value_axis_title: Some("Units"),
        category_axis_format: None,
        value_axis_format: None,
        legend: Some(LegendPosition::Bottom),
        size: Some((960, 640)),
        combined: None,
        drop_lines: false,
        high_low_lines: false,
        up_down_bars: false,
        category_axis: None,
        value_axis: None,
        title_text: None,
        axis_title_text: None,
        axis_label_text: None,
        legend_text: None,
    }
}

/// The same chart with another title, for a replacement.
fn other_chart() -> NewChart<'static> {
    NewChart {
        title: Some("Costs"),
        ..column_chart()
    }
}

/// The absolute anchor at 0,0 a chartsheet's chart takes.
const ORIGIN: ImageAnchor = ImageAnchor::Absolute { position: (0, 0) };

/// What `charts` reads for an absolute anchor.
const READ_ABSOLUTE: ChartAnchor = ChartAnchor {
    kind: Anchor::Absolute,
    from: (0, 0),
    to: None,
    edit_as: None,
};

/// A part of `output`, read back through a reopened package.
fn text_of(output: &[u8], name: &[u8]) -> String {
    let mut package = Package::open(Cursor::new(output.to_vec())).expect("open the output");
    let index = package.part(name).expect("the part");
    let mut bytes = Vec::new();
    package
        .read_part(index)
        .expect("read_part")
        .read_to_end(&mut bytes)
        .expect("read");
    String::from_utf8(bytes).expect("UTF-8")
}

/// The charts `charts` reads from `part` of `output`.
fn charts_of(output: &[u8], part: &[u8]) -> Vec<ChartEntry> {
    let mut package = Package::open(Cursor::new(output.to_vec())).expect("open the output");
    package.charts(part).expect("charts")
}

/// The chart the column chart reads back as, written on the worksheet of
/// the smallest package in the absolute anchor at 0,0: what a chartsheet's
/// chart must read as, field by field.
fn as_on_a_worksheet(chart: &NewChart<'_>) -> ChartEntry {
    let (_, mut package) = opened(smallest());
    package.add_chart(SHEET, ORIGIN, chart).expect("recorded");
    let out = written(package);
    let mut read = charts_of(&out, SHEET);
    assert_eq!(read.len(), 1, "one chart on the worksheet");
    read.remove(0)
}

/// Asserts that `entries` is one chart reading as `chart` does on a
/// worksheet in the absolute anchor, the part aside, and returns its part.
#[track_caller]
fn assert_one_chart(entries: &[ChartEntry], chart: &NewChart<'_>) -> Vec<u8> {
    assert_eq!(entries.len(), 1, "one chart");
    let want = as_on_a_worksheet(chart);
    assert_eq!(entries[0].anchor, READ_ABSOLUTE, "the anchor");
    assert_eq!(entries[0].chart, want.chart, "the chart, every field");
    assert_eq!(entries[0].chart.title.as_deref(), chart.title, "the title");
    entries[0].part.clone()
}

/// The smallest package with a chartsheet holding the column chart, added
/// and written: the fixture of the tests that edit a kept chartsheet.
fn with_chartsheet() -> Vec<u8> {
    let (_, mut package) = opened(smallest());
    package
        .add_chartsheet("Chart1", &column_chart())
        .expect("added");
    written(package)
}

/// The entries of `output`, name and bytes, as a test's parts.
fn parts_of(output: &[u8]) -> Parts {
    let mut package = Package::open(Cursor::new(output.to_vec())).expect("open the output");
    let names: Vec<Vec<u8>> = package
        .archive()
        .entries()
        .iter()
        .map(|entry| entry.name().to_vec())
        .collect();
    names
        .into_iter()
        .map(|name| {
            let index = package.part(&name).expect("the part");
            let mut bytes = Vec::new();
            package
                .read_part(index)
                .expect("read_part")
                .read_to_end(&mut bytes)
                .expect("read");
            (name, bytes)
        })
        .collect()
}

/// Asserts that every entry of `parts` but those named comes back in
/// `output` with the same bytes, uncompressed.
#[track_caller]
fn assert_kept(output: &[u8], parts: &Parts, except: &[&[u8]]) {
    let out = parts_of(output);
    for (name, bytes) in parts {
        if except.contains(&name.as_slice()) {
            continue;
        }
        let kept = out.iter().find(|(have, _)| have == name);
        assert_eq!(
            kept.map(|(_, data)| data),
            Some(bytes),
            "{} is kept byte for byte",
            String::from_utf8_lossy(name)
        );
    }
}

/// The blank chartsheet form (decision 3 A) naming `rId1`.
fn chartsheet_form() -> String {
    format!(
        "{DECLARATION}\n<chartsheet xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\"><sheetViews>\
         <sheetView workbookViewId=\"0\" zoomToFit=\"1\"/></sheetViews><drawing r:id=\"rId1\"/>\
         </chartsheet>"
    )
}

/// The drawing an added chartsheet's chart sits in: one absolute anchor at
/// 0,0 of 9144000 by 6096000 EMU holding the frame naming `rId1`.
fn absolute_drawing() -> String {
    format!(
        "{DECLARATION}\n<xdr:wsDr xmlns:xdr=\"{XDR_NS}\" xmlns:a=\"{A_NS}\"><xdr:absoluteAnchor>\
         <xdr:pos x=\"0\" y=\"0\"/><xdr:ext cx=\"9144000\" cy=\"6096000\"/>{}<xdr:clientData/>\
         </xdr:absoluteAnchor></xdr:wsDr>",
        frame(1, "rId1")
    )
}

/// The `graphicFrame` the writer adds, its `cNvPr` `id` and its `c:chart`
/// naming `rel`.
fn frame(id: u32, rel: &str) -> String {
    format!(
        "<xdr:graphicFrame macro=\"\"><xdr:nvGraphicFramePr><xdr:cNvPr id=\"{id}\" \
         name=\"Chart {id}\"/><xdr:cNvGraphicFramePr/></xdr:nvGraphicFramePr><xdr:xfrm>\
         <a:off x=\"0\" y=\"0\"/><a:ext cx=\"0\" cy=\"0\"/></xdr:xfrm><a:graphic>\
         <a:graphicData uri=\"{C_NS}\"><c:chart xmlns:c=\"{C_NS}\" xmlns:r=\"{ATTR_NS}\" \
         r:id=\"{rel}\"/></a:graphicData></a:graphic></xdr:graphicFrame>"
    )
}

/// What the write refuses with after `edit` on the package `output`.
#[track_caller]
fn refused_after(output: &[u8], edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> Error {
    let mut package = open_fixture(output.to_vec());
    edit(&mut package);
    refused(package)
}

// ------------------------------------------------------------ add_chartsheet

#[test]
fn a_chartsheet_added_to_a_workbook_with_one_worksheet() {
    let (parts, mut package) = opened(smallest());
    let part = package
        .add_chartsheet("Chart1", &column_chart())
        .expect("added");
    assert_eq!(part, CHARTSHEET, "the part");
    // The projected list shows the sheet's kind before the write.
    let projected = package.sheets().expect("sheets");
    assert_eq!(projected.len(), 2, "one sheet more");
    assert_eq!(projected[1].name, "Chart1");
    assert_eq!(projected[1].kind, SheetKind::Chartsheet, "the kind");
    assert_eq!(projected[1].sheet_id, 2, "sheetId one above the largest");
    let out = written(package);
    assert_names(
        &out,
        &[
            b"[Content_Types].xml".as_slice(),
            b"_rels/.rels",
            b"xl/workbook.xml",
            b"xl/_rels/workbook.xml.rels",
            SHEET,
            b"xl/styles.xml",
            CHARTSHEET,
            CHARTSHEET_RELS,
            DRAWING,
            DRAWING_RELS,
            CHART1,
        ],
    );
    assert_untouched(
        &out,
        &parts,
        &[
            b"[Content_Types].xml",
            b"xl/workbook.xml",
            b"xl/_rels/workbook.xml.rels",
        ],
    );
    assert_eq!(text_of(&out, CHARTSHEET), chartsheet_form());
    assert_eq!(
        text_of(&out, CHARTSHEET_RELS),
        rels(&relationship(
            "rId1",
            &format!("{ATTR_NS}/drawing"),
            "../drawings/drawing1.xml"
        ))
    );
    assert_eq!(text_of(&out, DRAWING), absolute_drawing());
    assert_eq!(
        text_of(&out, DRAWING_RELS),
        rels(&relationship(
            "rId1",
            &format!("{ATTR_NS}/chart"),
            "../charts/chart1.xml"
        ))
    );
    // The `<sheet>` last, its relationship of the chartsheet type.
    assert_eq!(
        text_of(&out, b"xl/workbook.xml"),
        part_text(&parts, b"xl/workbook.xml").replace(
            "</sheets>",
            "<sheet name=\"Chart1\" sheetId=\"2\" r:id=\"rId3\"/></sheets>"
        )
    );
    assert_eq!(
        text_of(&out, b"xl/_rels/workbook.xml.rels"),
        part_text(&parts, b"xl/_rels/workbook.xml.rels").replace(
            "</Relationships>",
            &format!(
                "{}</Relationships>",
                relationship(
                    "rId3",
                    &format!("{ATTR_NS}/chartsheet"),
                    "chartsheets/sheet1.xml"
                )
            )
        )
    );
    assert_eq!(
        text_of(&out, b"[Content_Types].xml"),
        part_text(&parts, b"[Content_Types].xml").replace(
            "</Types>",
            &format!(
                "<Override PartName=\"/xl/chartsheets/sheet1.xml\" ContentType=\"{CHARTSHEET_TYPE}\"/>\
                 <Override PartName=\"/xl/drawings/drawing1.xml\" ContentType=\"{DRAWING_TYPE}\"/>\
                 <Override PartName=\"/xl/charts/chart1.xml\" ContentType=\"{CHART_TYPE}\"/></Types>"
            )
        )
    );
    // The written list, and the chart read back as given.
    let mut back = Package::open(Cursor::new(out.clone())).expect("open the output");
    let sheets = back.workbook().expect("workbook").sheets.clone();
    assert_eq!(sheets[1].kind, SheetKind::Chartsheet, "the written kind");
    assert_eq!(sheets[1].part, CHARTSHEET);
    let read = assert_one_chart(&charts_of(&out, CHARTSHEET), &column_chart());
    assert_eq!(read, CHART1, "the chart part");
}

#[test]
fn a_default_for_xml_naming_the_chartsheet_type_takes_no_override() {
    let mut parts = smallest();
    let types = part_text(&parts, b"[Content_Types].xml").replace(
        "<Default Extension=\"xml\" ContentType=\"application/xml\"/>",
        &format!("<Default Extension=\"xml\" ContentType=\"{CHARTSHEET_TYPE}\"/>"),
    );
    set(&mut parts, b"[Content_Types].xml", types);
    let (parts, mut package) = opened(parts);
    package
        .add_chartsheet("Chart1", &column_chart())
        .expect("added");
    let out = written(package);
    assert_eq!(
        text_of(&out, b"[Content_Types].xml"),
        part_text(&parts, b"[Content_Types].xml").replace(
            "</Types>",
            &format!(
                "<Override PartName=\"/xl/drawings/drawing1.xml\" ContentType=\"{DRAWING_TYPE}\"/>\
                 <Override PartName=\"/xl/charts/chart1.xml\" ContentType=\"{CHART_TYPE}\"/></Types>"
            )
        )
    );
    assert_one_chart(&charts_of(&out, CHARTSHEET), &column_chart());
}

#[test]
fn a_strict_workbook_takes_the_strict_relationship_form() {
    const R_STRICT: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships";
    let mut parts = smallest();
    let workbook_rels = part_text(&parts, b"xl/_rels/workbook.xml.rels").replace(
        &format!("{ATTR_NS}/worksheet"),
        &format!("{R_STRICT}/worksheet"),
    );
    set(&mut parts, b"xl/_rels/workbook.xml.rels", workbook_rels);
    let (_, mut package) = opened(parts);
    package
        .add_chartsheet("Chart1", &column_chart())
        .expect("added");
    let out = written(package);
    assert!(
        text_of(&out, b"xl/_rels/workbook.xml.rels").contains(&relationship(
            "rId3",
            &format!("{R_STRICT}/chartsheet"),
            "chartsheets/sheet1.xml"
        )),
        "the workbook's chartsheet relationship"
    );
    assert_eq!(
        text_of(&out, CHARTSHEET_RELS),
        rels(&relationship(
            "rId1",
            &format!("{R_STRICT}/drawing"),
            "../drawings/drawing1.xml"
        ))
    );
    assert_eq!(
        text_of(&out, DRAWING_RELS),
        rels(&relationship(
            "rId1",
            &format!("{R_STRICT}/chart"),
            "../charts/chart1.xml"
        ))
    );
    assert_one_chart(&charts_of(&out, CHARTSHEET), &column_chart());
}

#[test]
fn a_name_taken_a_name_invalid_and_a_missing_size_are_refused_and_record_nothing() {
    let no_size = NewChart {
        size: None,
        ..column_chart()
    };
    for (name, chart, message) in [
        ("Sheet1", column_chart(), "a sheet name is already in use"),
        ("sheet1", column_chart(), "a sheet name is already in use"),
        (
            "a/b",
            column_chart(),
            "a sheet name holds a character Excel refuses",
        ),
        ("", column_chart(), "a sheet name is empty"),
        ("Chart1", no_size, SIZE),
    ] {
        let (parts, mut package) = opened(smallest());
        let error = package.add_chartsheet(name, &chart).expect_err("refused");
        assert_package(error, message);
        assert_eq!(sheet_names(&mut package), ["Sheet1"], "nothing recorded");
        let out = written(package);
        assert_names(&out, &part_names(&parts));
        assert_untouched(&out, &parts, &[]);
    }
}

#[test]
fn a_chartsheet_removed_in_the_write_that_added_it_leaves_nothing() {
    let (parts, mut package) = opened(smallest());
    package
        .add_chartsheet("Chart1", &column_chart())
        .expect("added");
    package.remove_sheet("Chart1").expect("removed");
    assert_eq!(sheet_names(&mut package), ["Sheet1"]);
    let out = written(package);
    assert_names(&out, &part_names(&parts));
    assert_untouched(&out, &parts, &[]);
}

#[test]
fn a_chartsheet_renamed_and_moved_in_the_write_that_added_it_is_written_so() {
    // P1 verification ruling E: a pending chartsheet takes the sheet edits
    // as any sheet of the projected list (specs/0016 P4 ruling 4).
    let (parts, mut package) = opened(smallest());
    let part = package
        .add_chartsheet("Chart1", &column_chart())
        .expect("added");
    package.rename_sheet("Chart1", "Revenue").expect("renamed");
    package.move_sheet("Revenue", 0).expect("moved");
    let projected = package.sheets().expect("sheets");
    assert_eq!(sheet_names(&mut package), ["Revenue", "Sheet1"]);
    assert_eq!(projected[0].kind, SheetKind::Chartsheet, "the kind");
    assert_eq!(projected[0].sheet_id, 2, "sheetId one above the largest");
    let out = written(package);
    assert_names(
        &out,
        &[
            b"[Content_Types].xml".as_slice(),
            b"_rels/.rels",
            b"xl/workbook.xml",
            b"xl/_rels/workbook.xml.rels",
            SHEET,
            b"xl/styles.xml",
            CHARTSHEET,
            CHARTSHEET_RELS,
            DRAWING,
            DRAWING_RELS,
            CHART1,
        ],
    );
    assert_untouched(
        &out,
        &parts,
        &[
            b"[Content_Types].xml",
            b"xl/workbook.xml",
            b"xl/_rels/workbook.xml.rels",
        ],
    );
    assert_eq!(text_of(&out, CHARTSHEET), chartsheet_form());
    assert_eq!(text_of(&out, DRAWING), absolute_drawing());
    // The written list: the chartsheet first under its new name, the
    // worksheet after it, and the chart read back as given.
    let mut back = Package::open(Cursor::new(out.clone())).expect("open the output");
    let sheets = back.workbook().expect("workbook").sheets.clone();
    assert_eq!(sheets.len(), 2, "two sheets");
    assert_eq!(sheets[0].name, "Revenue");
    assert_eq!(sheets[0].kind, SheetKind::Chartsheet, "the written kind");
    assert_eq!(sheets[0].part, part);
    assert_eq!(sheets[0].sheet_id, 2);
    assert_eq!(sheets[1].name, "Sheet1");
    assert_eq!(sheets[1].kind, SheetKind::Worksheet);
    assert_eq!(sheets[1].part, SHEET);
    let read = assert_one_chart(&charts_of(&out, CHARTSHEET), &column_chart());
    assert_eq!(read, CHART1, "the chart part");
}

#[test]
fn the_chart_calls_on_a_chartsheet_added_in_the_same_write_are_no_such_part() {
    // P1 ruling C: as on a worksheet added in the same write.
    let (_, mut package) = opened(smallest());
    let part = package
        .add_chartsheet("Chart1", &column_chart())
        .expect("added");
    assert_package(package.charts(&part).expect_err("refused"), "no such part");
    package
        .add_chart(&part, ORIGIN, &column_chart())
        .expect("recorded");
    assert_package(refused(package), "no such part");
}

// ------------------------------------------------------------------ reading

#[test]
fn a_chartsheet_s_chart_reads_in_each_anchor_form() {
    let kept = parts_of(&with_chartsheet());
    let one_cell = format!(
        "<xdr:oneCellAnchor><xdr:from><xdr:col>1</xdr:col><xdr:colOff>0</xdr:colOff>\
         <xdr:row>2</xdr:row><xdr:rowOff>0</xdr:rowOff></xdr:from><xdr:ext cx=\"9144000\" \
         cy=\"6096000\"/>{}<xdr:clientData/></xdr:oneCellAnchor>",
        frame(1, "rId1")
    );
    let two_cell = format!(
        "<xdr:twoCellAnchor editAs=\"oneCell\"><xdr:from><xdr:col>1</xdr:col><xdr:colOff>0\
         </xdr:colOff><xdr:row>2</xdr:row><xdr:rowOff>0</xdr:rowOff></xdr:from><xdr:to>\
         <xdr:col>5</xdr:col><xdr:colOff>0</xdr:colOff><xdr:row>11</xdr:row><xdr:rowOff>0\
         </xdr:rowOff></xdr:to>{}<xdr:clientData/></xdr:twoCellAnchor>",
        frame(1, "rId1")
    );
    // openpyxl's form: an absolute anchor of extent 0 by 0.
    let zero = absolute_drawing().replace("cx=\"9144000\" cy=\"6096000\"", "cx=\"0\" cy=\"0\"");
    let wrap = |anchor: &str| {
        format!(
            "{DECLARATION}\n<xdr:wsDr xmlns:xdr=\"{XDR_NS}\" xmlns:a=\"{A_NS}\">{anchor}</xdr:wsDr>"
        )
    };
    let want = as_on_a_worksheet(&column_chart());
    for (drawing, anchor) in [
        (absolute_drawing(), READ_ABSOLUTE),
        (zero, READ_ABSOLUTE),
        (
            wrap(&one_cell),
            ChartAnchor {
                kind: Anchor::OneCell,
                from: (1, 2),
                to: None,
                edit_as: None,
            },
        ),
        (
            wrap(&two_cell),
            ChartAnchor {
                kind: Anchor::TwoCell,
                from: (1, 2),
                to: Some((5, 11)),
                edit_as: Some(EditAs::OneCell),
            },
        ),
    ] {
        let mut parts = kept.clone();
        set(&mut parts, DRAWING, drawing);
        let mut package = package(parts);
        let read = package.charts(CHARTSHEET).expect("charts");
        assert_eq!(read.len(), 1, "one chart");
        assert_eq!(read[0].anchor, anchor, "the anchor");
        assert_eq!(read[0].chart, want.chart, "the chart, every field");
        assert_eq!(read[0].part, CHART1);
    }
}

#[test]
fn a_chartsheet_with_no_drawing_element_reads_as_empty() {
    let mut parts = parts_of(&with_chartsheet());
    set(
        &mut parts,
        CHARTSHEET,
        chartsheet_form().replace("<drawing r:id=\"rId1\"/>", ""),
    );
    assert!(
        package(parts)
            .charts(CHARTSHEET)
            .expect("charts")
            .is_empty()
    );
}

#[test]
fn a_chartsheet_with_two_drawing_elements_or_one_out_of_order_is_refused() {
    // The drawing element's place is read in the chartsheet's own list.
    let mut parts = parts_of(&with_chartsheet());
    set(
        &mut parts,
        CHARTSHEET,
        chartsheet_form().replace(
            "<drawing r:id=\"rId1\"/>",
            "<drawing r:id=\"rId1\"/><drawing r:id=\"rId1\"/>",
        ),
    );
    assert_package(
        package(parts).charts(CHARTSHEET).expect_err("refused"),
        "a worksheet has two drawing elements",
    );
    let mut parts = parts_of(&with_chartsheet());
    set(
        &mut parts,
        CHARTSHEET,
        chartsheet_form().replace(
            "<drawing r:id=\"rId1\"/>",
            "<legacyDrawingHF r:id=\"rId9\"/><drawing r:id=\"rId1\"/>",
        ),
    );
    assert_package(
        package(parts).charts(CHARTSHEET).expect_err("refused"),
        "a worksheet has two drawing elements",
    );
    // Control: `tableParts`, after `drawing` in a worksheet's list and in
    // no place of a chartsheet's, does not count before it; `pageMargins`
    // and `headerFooter` before it are in order.
    let mut parts = parts_of(&with_chartsheet());
    set(
        &mut parts,
        CHARTSHEET,
        chartsheet_form().replace(
            "<drawing r:id=\"rId1\"/>",
            "<pageMargins left=\"0.7\" right=\"0.7\" top=\"0.75\" bottom=\"0.75\" header=\"0.3\" \
             footer=\"0.3\"/><headerFooter/><tableParts count=\"0\"/><drawing r:id=\"rId1\"/>",
        ),
    );
    assert_eq!(package(parts).charts(CHARTSHEET).expect("charts").len(), 1);
}

#[test]
fn the_chart_calls_on_a_dialogsheet_part_are_refused_with_the_new_root_message() {
    let mut parts = smallest();
    set(
        &mut parts,
        SHEET,
        format!(
            "{DECLARATION}\n<dialogsheet xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">\
             <drawing r:id=\"rId1\"/></dialogsheet>"
        ),
    );
    assert_package(
        package(parts.clone()).charts(SHEET).expect_err("refused"),
        ROOT,
    );
    let error = refused_with_parts(parts.clone(), |package| {
        package
            .add_chart(SHEET, ORIGIN, &column_chart())
            .expect("recorded");
    });
    assert_package(error, ROOT);
    let error = refused_with_parts(parts, |package| {
        package.remove_chart(SHEET, 0).expect("recorded");
    });
    assert_package(error, ROOT);
}

/// The refusal a write of `edit` on `parts` gives.
#[track_caller]
fn refused_with_parts(parts: Parts, edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> Error {
    let (_, mut package) = opened(parts);
    edit(&mut package);
    refused(package)
}

// -------------------------------------------- add_chart and remove_chart

#[test]
fn a_chart_on_a_chartsheet_in_a_one_cell_or_two_cell_anchor_is_refused() {
    // On a chartsheet whose chart is removed first, so the anchor's
    // refusal is the one met; the two-write rule (P1 ruling A) is the
    // other test's.
    let mut parts = parts_of(&with_chartsheet());
    set(
        &mut parts,
        DRAWING,
        format!("{DECLARATION}\n<xdr:wsDr xmlns:xdr=\"{XDR_NS}\" xmlns:a=\"{A_NS}\"></xdr:wsDr>"),
    );
    for anchor in [
        ImageAnchor::OneCell {
            from: at("B3"),
            offset: (0, 0),
        },
        ImageAnchor::TwoCell {
            from: at("B3"),
            to: at("F12"),
            from_offset: (0, 0),
            to_offset: (0, 0),
            edit_as: EditAs::OneCell,
        },
    ] {
        let chart = match anchor {
            ImageAnchor::TwoCell { .. } => NewChart {
                size: None,
                ..column_chart()
            },
            _ => column_chart(),
        };
        let error = refused_with_parts(parts.clone(), |package| {
            package
                .add_chart(CHARTSHEET, anchor, &chart)
                .expect("recorded");
        });
        assert_package(error, ABSOLUTE);
    }
    // Control: the absolute anchor is taken, at any position.
    let (_, mut package) = opened(parts);
    package
        .add_chart(
            CHARTSHEET,
            ImageAnchor::Absolute { position: (10, 20) },
            &column_chart(),
        )
        .expect("recorded");
    let out = written(package);
    let read = charts_of(&out, CHARTSHEET);
    assert_eq!(read.len(), 1);
    assert_eq!(read[0].anchor.kind, Anchor::Absolute);
}

#[test]
fn a_second_chart_on_a_chartsheet_is_refused() {
    let out = with_chartsheet();
    let error = refused_after(&out, |package| {
        package
            .add_chart(CHARTSHEET, ORIGIN, &other_chart())
            .expect("recorded");
    });
    assert_package(error, ONE);
    // Two in one write on an empty drawing: the count is over the kept
    // frames and the additions.
    let mut parts = parts_of(&out);
    set(
        &mut parts,
        DRAWING,
        format!("{DECLARATION}\n<xdr:wsDr xmlns:xdr=\"{XDR_NS}\" xmlns:a=\"{A_NS}\"></xdr:wsDr>"),
    );
    let error = refused_with_parts(parts, |package| {
        package
            .add_chart(CHARTSHEET, ORIGIN, &column_chart())
            .expect("recorded");
        package
            .add_chart(CHARTSHEET, ORIGIN, &other_chart())
            .expect("recorded");
    });
    assert_package(error, ONE);
}

#[test]
fn a_chart_is_replaced_by_a_removal_and_an_addition_in_two_writes() {
    // P1 ruling A: remove, write, add, write.
    let first = with_chartsheet();
    let mut package = open_fixture(first.clone());
    package.remove_chart(CHARTSHEET, 0).expect("recorded");
    let removed = written(package);
    assert!(charts_of(&removed, CHARTSHEET).is_empty(), "removed");
    let mut package = open_fixture(removed);
    package
        .add_chart(CHARTSHEET, ORIGIN, &other_chart())
        .expect("recorded");
    let replaced = written(package);
    let part = assert_one_chart(&charts_of(&replaced, CHARTSHEET), &other_chart());
    // The second write sees the removed chart part's name free.
    assert_eq!(part, CHART1, "the chart part");
    assert_eq!(text_of(&replaced, CHARTSHEET), chartsheet_form());
    // Control: the removal and the addition in one write are refused with
    // the worksheet's message.
    let error = refused_after(&first, |package| {
        package.remove_chart(CHARTSHEET, 0).expect("recorded");
        package
            .add_chart(CHARTSHEET, ORIGIN, &other_chart())
            .expect("recorded");
    });
    assert_package(error, NO_CHART_AT);
}

#[test]
fn a_removal_leaves_the_chartsheet_s_drawing_empty() {
    let first = with_chartsheet();
    let parts = parts_of(&first);
    let mut package = open_fixture(first);
    package.remove_chart(CHARTSHEET, 0).expect("recorded");
    let out = written(package);
    // The chart part and its `Override` go; the drawing stays, empty, its
    // rels part without the chart's relationship; the chartsheet and its
    // rels part are kept byte for byte.
    assert_names(&out, &names_without(&parts, &["xl/charts/chart1.xml"]));
    assert_eq!(
        text_of(&out, DRAWING),
        format!("{DECLARATION}\n<xdr:wsDr xmlns:xdr=\"{XDR_NS}\" xmlns:a=\"{A_NS}\"></xdr:wsDr>")
    );
    assert_eq!(text_of(&out, DRAWING_RELS), rels(""));
    assert_kept(
        &out,
        &parts,
        &[b"[Content_Types].xml", DRAWING, DRAWING_RELS, CHART1],
    );
    assert_eq!(
        text_of(&out, b"[Content_Types].xml"),
        types_without(&parts, &["xl/charts/chart1.xml"])
    );
    assert!(charts_of(&out, CHARTSHEET).is_empty());
}

#[test]
fn a_chartsheet_whose_drawing_element_is_missing_takes_no_chart() {
    // A chartsheet without `<drawing>` is invalid by the schema; the
    // worksheet pass that creates the element takes a worksheet alone.
    let mut parts = parts_of(&with_chartsheet());
    set(
        &mut parts,
        CHARTSHEET,
        chartsheet_form().replace("<drawing r:id=\"rId1\"/>", ""),
    );
    let error = refused_with_parts(parts, |package| {
        package
            .add_chart(CHARTSHEET, ORIGIN, &column_chart())
            .expect("recorded");
    });
    assert_package(error, "a drawing part or its relationship cannot be read");
}

// ------------------------------------------- images, comments, sheet calls

#[cfg(feature = "images")]
#[test]
fn an_image_on_a_chartsheet_is_refused_as_before() {
    let out = with_chartsheet();
    let mut package = Package::open(Cursor::new(out.clone())).expect("open");
    assert_package(
        package.images(CHARTSHEET).expect_err("refused"),
        NOT_WORKSHEET,
    );
    let error = refused_after(&out, |package| {
        package.remove_image(CHARTSHEET, 0).expect("recorded");
    });
    assert_package(error, NOT_WORKSHEET);
}

#[cfg(feature = "comments")]
#[test]
fn a_comment_on_a_chartsheet_is_refused_as_before() {
    use crate::package::Comment;
    let out = with_chartsheet();
    let error = refused_after(&out, |package| {
        package
            .add_comment(
                CHARTSHEET,
                at("A1"),
                Comment {
                    author: "Author",
                    text: "Note",
                },
            )
            .expect("recorded");
    });
    assert_package(error, NOT_WORKSHEET);
}

#[test]
fn a_chartsheet_is_renamed_moved_and_removed() {
    let out = with_chartsheet();
    let parts = parts_of(&out);
    // Renamed and moved first: the workbook alone changes.
    let mut package = open_fixture(out.clone());
    package.rename_sheet("Chart1", "Revenue").expect("renamed");
    package.move_sheet("Revenue", 0).expect("moved");
    let moved = written(package);
    assert_kept(&moved, &parts, &[b"xl/workbook.xml"]);
    let mut back = Package::open(Cursor::new(moved.clone())).expect("open");
    let sheets = back.workbook().expect("workbook").sheets.clone();
    assert_eq!(sheets[0].name, "Revenue");
    assert_eq!(sheets[0].kind, SheetKind::Chartsheet);
    assert_one_chart(&charts_of(&moved, CHARTSHEET), &column_chart());
    // Removed: the chartsheet goes with every part only it reaches.
    let mut package = open_fixture(out);
    package.remove_sheet("Chart1").expect("removed");
    let removed = written(package);
    assert_names(
        &removed,
        &names_without(
            &parts,
            &[
                "xl/chartsheets/sheet1.xml",
                "xl/chartsheets/_rels/sheet1.xml.rels",
                "xl/drawings/drawing1.xml",
                "xl/drawings/_rels/drawing1.xml.rels",
                "xl/charts/chart1.xml",
            ],
        ),
    );
}

#[test]
fn a_chartsheet_of_a_surface_reads_back_as_on_a_worksheet() {
    const OVER_B_AND_C: [NewSeries<'static>; 2] = [
        NewSeries {
            name: Some(NewSeriesName::Reference("Sheet1!$B$1")),
            categories: Some("Sheet1!$A$2:$A$4"),
            values: "Sheet1!$B$2:$B$4",
            sizes: None,
            color: None,
            labels: None,
            marker: None,
            line: None,
            points: &[],
            trendline: None,
            error_bars: &[],
        },
        NewSeries {
            name: Some(NewSeriesName::Reference("Sheet1!$C$1")),
            categories: Some("Sheet1!$A$2:$A$4"),
            values: "Sheet1!$C$2:$C$4",
            sizes: None,
            color: None,
            labels: None,
            marker: None,
            line: None,
            points: &[],
            trendline: None,
            error_bars: &[],
        },
    ];
    let chart = NewChart {
        plot: NewPlot::Surface3D { wireframe: true },
        series: &OVER_B_AND_C,
        ..column_chart()
    };
    let (_, mut package) = opened(smallest());
    let part = package.add_chartsheet("Surface", &chart).expect("added");
    assert_eq!(part, CHARTSHEET);
    let out = written(package);
    let entries = charts_of(&out, CHARTSHEET);
    let chart_part = assert_one_chart(&entries, &chart);
    assert_eq!(chart_part, CHART1);
    // The plot read back whole (P1 verification ruling E (c)): the kind, no
    // varied colours, the two series as given, the three axis ids.
    let data = |formula: String| Data::Reference {
        formula,
        cached: None,
    };
    let read_series = ["B", "C"]
        .iter()
        .zip(0..)
        .map(|(column, k)| Series {
            index: k,
            order: k,
            name: Some(SeriesName::Reference(format!("Sheet1!${column}$1"))),
            categories: Some(data("Sheet1!$A$2:$A$4".to_owned())),
            values: Some(data(format!("Sheet1!${column}$2:${column}$4"))),
            sizes: None,
            fill: None,
            smooth: false,
            marker: false,
            labels: false,
        })
        .collect();
    assert_eq!(
        entries[0].chart.plots,
        vec![Plot {
            kind: ChartKind::Surface3D,
            bar_direction: None,
            grouping: None,
            scatter_style: None,
            vary_colors: false,
            series: read_series,
            axis_ids: vec![10, 20, 30],
        }]
    );
    let written_part = text_of(&out, CHART1);
    let series: String = ["B", "C"]
        .iter()
        .enumerate()
        .map(|(k, column)| {
            format!(
                "<c:ser><c:idx val=\"{k}\"/><c:order val=\"{k}\"/><c:tx><c:strRef><c:f>\
                 Sheet1!${column}$1</c:f></c:strRef></c:tx><c:cat><c:strRef><c:f>\
                 Sheet1!$A$2:$A$4</c:f></c:strRef></c:cat><c:val><c:numRef><c:f>\
                 Sheet1!${column}$2:${column}$4</c:f></c:numRef></c:val></c:ser>"
            )
        })
        .collect();
    // The plot whole, then the series axis after the value axis.
    assert!(
        written_part.contains(&format!(
            "<c:plotArea><c:layout/><c:surface3DChart><c:wireframe val=\"1\"/>{series}<c:axId \
             val=\"10\"/><c:axId val=\"20\"/><c:axId val=\"30\"/></c:surface3DChart><c:catAx>"
        )),
        "{written_part}"
    );
    assert!(
        written_part.contains("</c:valAx><c:serAx><c:axId val=\"30\"/>"),
        "{written_part}"
    );
}
