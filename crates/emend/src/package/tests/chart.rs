//! Charts on byte-built packages (`specs/0034-charts-read.md`, "P1: the
//! feature, the anchors, the plots and series", "P2: axes, legend, pivot
//! source, style; the catalogue", P1 rulings A and B, and the P1
//! verification's rulings E and G, carried to P2):
//! each test builds the smallest package whose worksheet names a drawing
//! part holding the anchors it gives, with a chart part per `c:chart`, and
//! reads the sheet's charts. Every content is fictional. The Cargo feature
//! `charts`.

use super::*;
use crate::package::{
    Anchor, AxisKind, AxisPosition, BarDirection, Chart, ChartAnchor, ChartAxis, ChartEntry,
    ChartKind, Data, EditAs, Grouping, LegendPosition, Plot, ScatterStyle, Series, SeriesName,
};

/// The drawing and chart parts' content types.
const DRAWING_TYPE: &str = "application/vnd.openxmlformats-officedocument.drawing+xml";
const CHART_TYPE: &str = "application/vnd.openxmlformats-officedocument.drawingml.chart+xml";

/// The spreadsheet-drawing, DrawingML, chart and markup-compatibility
/// namespaces, transitional, and the first three strict.
const XDR_NS: &str = "http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing";
const A_NS: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";
const C_NS: &str = "http://schemas.openxmlformats.org/drawingml/2006/chart";
const MC_NS: &str = "http://schemas.openxmlformats.org/markup-compatibility/2006";
const XDR_STRICT: &str = "http://purl.oclc.org/ooxml/drawingml/spreadsheetDrawing";
const A_STRICT: &str = "http://purl.oclc.org/ooxml/drawingml/main";
const C_STRICT: &str = "http://purl.oclc.org/ooxml/drawingml/chart";

/// The message the chart refusals carry, and the drawing's.
const UNREADABLE: &str = "a chart cannot be read";
const DRAWING_UNREADABLE: &str = "a drawing part or its relationship cannot be read";

/// The worksheet body the fixtures use.
const BODY: &str = "<sheetData/><drawing r:id=\"rId1\"/>";

/// A worksheet part whose root binds `r` to the relationships namespace.
fn sheet_r(body: &str) -> String {
    format!(
        "{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">{body}</worksheet>"
    )
}

/// A drawing part binding `xdr`, `a`, `c` and `r` on its root, with
/// `children`.
fn drawing_xml(children: &str) -> String {
    drawing_in(XDR_NS, A_NS, C_NS, children)
}

fn drawing_in(xdr: &str, a: &str, c: &str, children: &str) -> String {
    format!(
        "{DECLARATION}\n<xdr:wsDr xmlns:xdr=\"{xdr}\" xmlns:a=\"{a}\" xmlns:c=\"{c}\" \
         xmlns:r=\"{ATTR_NS}\">{children}</xdr:wsDr>"
    )
}

/// A chart's frame as Excel writes one, its `c:chart` carrying `attributes`.
fn frame_with(attributes: &str) -> String {
    format!(
        "<xdr:graphicFrame macro=\"\"><xdr:nvGraphicFramePr><xdr:cNvPr id=\"2\" \
         name=\"Chart 1\"/><xdr:cNvGraphicFramePr/></xdr:nvGraphicFramePr><xdr:xfrm>\
         <a:off x=\"0\" y=\"0\"/><a:ext cx=\"0\" cy=\"0\"/></xdr:xfrm><a:graphic>\
         <a:graphicData uri=\"{C_NS}\"><c:chart {attributes}/></a:graphicData>\
         </a:graphic></xdr:graphicFrame>"
    )
}

/// A chart's frame naming `id`.
fn frame(id: &str) -> String {
    frame_with(&format!("r:id=\"{id}\""))
}

/// A frame holding something other than a chart.
fn other_frame() -> String {
    "<xdr:graphicFrame macro=\"\"><xdr:nvGraphicFramePr><xdr:cNvPr id=\"3\" name=\"Diagram\"/>\
     <xdr:cNvGraphicFramePr/></xdr:nvGraphicFramePr><xdr:xfrm/><a:graphic>\
     <a:graphicData uri=\"http://schemas.openxmlformats.org/drawingml/2006/diagram\">\
     <a:other/></a:graphicData></a:graphic></xdr:graphicFrame>"
        .to_owned()
}

/// A picture and a shape.
fn pic() -> String {
    "<xdr:pic><xdr:nvPicPr><xdr:cNvPr id=\"4\" name=\"Picture\"/><xdr:cNvPicPr/></xdr:nvPicPr>\
     <xdr:blipFill><a:blip r:embed=\"rId9\"/></xdr:blipFill><xdr:spPr/></xdr:pic>"
        .to_owned()
}
fn shape() -> String {
    "<xdr:sp><xdr:nvSpPr><xdr:cNvPr id=\"5\" name=\"Shape\"/><xdr:cNvSpPr/></xdr:nvSpPr>\
     <xdr:spPr/></xdr:sp>"
        .to_owned()
}

/// A `from` or `to` at zero-based `column` and `row`.
fn marker(name: &str, column: u32, row: u32) -> String {
    format!(
        "<xdr:{name}><xdr:col>{column}</xdr:col><xdr:colOff>0</xdr:colOff><xdr:row>{row}</xdr:row>\
         <xdr:rowOff>0</xdr:rowOff></xdr:{name}>"
    )
}

fn two_cell(edit_as: Option<&str>, from: (u32, u32), to: (u32, u32), content: &str) -> String {
    let edit_as = edit_as.map_or(String::new(), |value| format!(" editAs=\"{value}\""));
    format!(
        "<xdr:twoCellAnchor{edit_as}>{}{}{content}<xdr:clientData/></xdr:twoCellAnchor>",
        marker("from", from.0, from.1),
        marker("to", to.0, to.1)
    )
}

fn one_cell(from: (u32, u32), content: &str) -> String {
    format!(
        "<xdr:oneCellAnchor>{}<xdr:ext cx=\"9525\" cy=\"9525\"/>{content}<xdr:clientData/>\
         </xdr:oneCellAnchor>",
        marker("from", from.0, from.1)
    )
}

fn absolute(content: &str) -> String {
    format!(
        "<xdr:absoluteAnchor><xdr:pos x=\"0\" y=\"0\"/><xdr:ext cx=\"9525\" cy=\"9525\"/>\
         {content}<xdr:clientData/></xdr:absoluteAnchor>"
    )
}

/// A two-cell anchor at A1 to J10 holding the frame naming `id`.
fn anchored(id: &str) -> String {
    two_cell(None, (0, 0), (9, 9), &frame(id))
}

/// A chart part in `c`'s namespace whose `c:chart` holds `body`.
fn chart_in(c: &str, body: &str) -> String {
    chart_space(c, "", body)
}

/// A chart part in `c`'s namespace with the root children `before` ahead
/// of its `c:chart`, which holds `body`, as Excel orders them.
fn chart_space(c: &str, before: &str, body: &str) -> String {
    format!(
        "{DECLARATION}\n<c:chartSpace xmlns:c=\"{c}\" xmlns:a=\"{A_NS}\" xmlns:r=\"{ATTR_NS}\">\
         <c:date1904 val=\"0\"/><c:lang val=\"en-US\"/><c:roundedCorners val=\"0\"/>{before}\
         <c:chart>{body}</c:chart><c:printSettings><c:headerFooter/></c:printSettings>\
         </c:chartSpace>"
    )
}
fn chart_xml(body: &str) -> String {
    chart_in(C_NS, body)
}

/// A chart part whose `plotArea` holds `plots` and the two axes `plot`'s
/// ids name.
fn plots_xml(plots: &str) -> String {
    axes_xml(
        plots,
        &format!(
            "{}{}",
            axis("catAx", 10, "b", 20, ""),
            axis("valAx", 20, "l", 10, "")
        ),
    )
}

/// A chart part whose `plotArea` holds `plots` and then `axes`.
fn axes_xml(plots: &str, axes: &str) -> String {
    chart_xml(&format!(
        "<c:autoTitleDeleted val=\"0\"/><c:plotArea><c:layout/>{plots}{axes}</c:plotArea>\
         <c:plotVisOnly val=\"1\"/>"
    ))
}

/// An axis `kind` with `axId` `id`, `axPos` `position`, `crossAx` `crosses`
/// and the children `inner` after `axPos`, in the schema's order.
fn axis(kind: &str, id: i64, position: &str, crosses: i64, inner: &str) -> String {
    format!(
        "<c:{kind}><c:axId val=\"{id}\"/><c:scaling><c:orientation val=\"minMax\"/></c:scaling>\
         <c:delete val=\"0\"/><c:axPos val=\"{position}\"/>{inner}<c:majorTickMark val=\"out\"/>\
         <c:crossAx val=\"{crosses}\"/><c:crosses val=\"autoZero\"/></c:{kind}>"
    )
}

/// A `ser` with `idx` and `order` both `n` and the children `inner`.
fn ser(n: u32, inner: &str) -> String {
    format!("<c:ser><c:idx val=\"{n}\"/><c:order val=\"{n}\"/>{inner}</c:ser>")
}

/// A number reference with a cache of `count` points (the points' values
/// fictional), and a string reference likewise.
fn num_ref(formula: &str, count: u32) -> String {
    let points: String = (0..count)
        .map(|k| format!("<c:pt idx=\"{k}\"><c:v>{k}</c:v></c:pt>"))
        .collect();
    format!(
        "<c:numRef><c:f>{formula}</c:f><c:numCache><c:formatCode>General</c:formatCode>\
         <c:ptCount val=\"{count}\"/>{points}</c:numCache></c:numRef>"
    )
}
fn str_ref(formula: &str, count: u32) -> String {
    let points: String = (0..count)
        .map(|k| format!("<c:pt idx=\"{k}\"><c:v>p{k}</c:v></c:pt>"))
        .collect();
    format!(
        "<c:strRef><c:f>{formula}</c:f><c:strCache><c:ptCount val=\"{count}\"/>{points}\
         </c:strCache></c:strRef>"
    )
}

/// A plot `kind` with the series `series` and two axis ids.
fn plot(kind: &str, extra: &str, series: &str) -> String {
    format!("<c:{kind}>{extra}{series}<c:axId val=\"10\"/><c:axId val=\"20\"/></c:{kind}>")
}

/// The smallest package whose worksheet names the drawing `drawing`, with
/// the chart parts `charts` named `xl/charts/chart<k>.xml` and related from
/// the drawing as `rId<k>`, plus the relationships `extra`.
fn with_charts(drawing: &str, charts: &[String], extra: &str) -> Parts {
    let mut parts = smallest();
    set(&mut parts, SHEET, sheet_r(BODY));
    with_part(
        &mut parts,
        "xl/worksheets/_rels/sheet1.xml.rels",
        &rels(&relationship(
            "rId1",
            &format!("{ATTR_NS}/drawing"),
            "../drawings/drawing1.xml",
        )),
        None,
    );
    with_part(
        &mut parts,
        "xl/drawings/drawing1.xml",
        drawing,
        Some(DRAWING_TYPE),
    );
    let chart_rels: String = (1..=charts.len())
        .map(|k| {
            relationship(
                &format!("rId{k}"),
                &format!("{ATTR_NS}/chart"),
                &format!("../charts/chart{k}.xml"),
            )
        })
        .collect();
    with_part(
        &mut parts,
        "xl/drawings/_rels/drawing1.xml.rels",
        &rels(&format!("{chart_rels}{extra}")),
        None,
    );
    for (k, chart) in charts.iter().enumerate() {
        with_part(
            &mut parts,
            &format!("xl/charts/chart{}.xml", k + 1),
            chart,
            Some(CHART_TYPE),
        );
    }
    parts
}

/// The sheet's charts.
#[track_caller]
fn charts_of(parts: Parts) -> Result<Vec<ChartEntry>, Error> {
    package(parts).charts(SHEET)
}

/// The one chart of a drawing holding one anchored chart `chart`.
#[track_caller]
fn one_chart(chart: String) -> Chart {
    let mut entries = charts_of(with_charts(&drawing_xml(&anchored("rId1")), &[chart], ""))
        .expect("the chart reads");
    assert_eq!(entries.len(), 1);
    entries.remove(0).chart
}

/// The plots of a chart whose `plotArea` holds `plots`.
#[track_caller]
fn plots_of(plots: &str) -> Vec<Plot> {
    one_chart(plots_xml(plots)).plots
}

/// The series of a chart with one bar plot holding `series`.
#[track_caller]
fn series_of(series: &str) -> Vec<Series> {
    plots_of(&plot("barChart", "<c:barDir val=\"col\"/>", series))
        .remove(0)
        .series
}

/// The refusal reading `chart` as the drawing's one chart gives.
#[track_caller]
fn refused_chart(chart: String) -> Error {
    charts_of(with_charts(&drawing_xml(&anchored("rId1")), &[chart], ""))
        .expect_err("the chart is refused")
}

/// A one-series bar plot, for the fixtures that need any chart.
fn simple() -> String {
    plots_xml(&plot(
        "barChart",
        "<c:barDir val=\"col\"/><c:grouping val=\"clustered\"/>",
        &ser(
            0,
            &format!("<c:val>{}</c:val>", num_ref("Data!$B$2:$B$4", 3)),
        ),
    ))
}

// ------------------------------------------------------------ the anchors

#[test]
fn every_anchor_kind_holding_a_chart_gives_one_and_nothing_else_does() {
    let drawing = drawing_xml(
        &[
            two_cell(Some("oneCell"), (1, 2), (5, 9), &frame("rId1")),
            one_cell((3, 4), &pic()),
            two_cell(None, (0, 0), (2, 2), &shape()),
            one_cell((6, 7), &frame("rId2")),
            two_cell(Some("twoCell"), (0, 0), (2, 2), &other_frame()),
            absolute(&frame("rId3")),
        ]
        .concat(),
    );
    let entries = charts_of(with_charts(&drawing, &[simple(), simple(), simple()], ""))
        .expect("the charts read");
    let read: Vec<(&[u8], ChartAnchor)> = entries
        .iter()
        .map(|entry| (entry.part.as_slice(), entry.anchor))
        .collect();
    assert_eq!(
        read,
        vec![
            (
                b"xl/charts/chart1.xml".as_slice(),
                ChartAnchor {
                    kind: Anchor::TwoCell,
                    from: (1, 2),
                    to: Some((5, 9)),
                    edit_as: Some(EditAs::OneCell),
                }
            ),
            (
                b"xl/charts/chart2.xml".as_slice(),
                ChartAnchor {
                    kind: Anchor::OneCell,
                    from: (6, 7),
                    to: None,
                    edit_as: None,
                }
            ),
            (
                b"xl/charts/chart3.xml".as_slice(),
                ChartAnchor {
                    kind: Anchor::Absolute,
                    from: (0, 0),
                    to: None,
                    edit_as: None,
                }
            ),
        ]
    );
}

#[test]
fn each_edit_as_value_reads_and_an_absent_one_is_none() {
    let drawing = drawing_xml(
        &[
            two_cell(Some("twoCell"), (0, 0), (1, 1), &frame("rId1")),
            two_cell(Some("absolute"), (0, 0), (1, 1), &frame("rId2")),
            two_cell(None, (0, 0), (1, 1), &frame("rId3")),
        ]
        .concat(),
    );
    let edit_as: Vec<Option<EditAs>> =
        charts_of(with_charts(&drawing, &[simple(), simple(), simple()], ""))
            .expect("the charts read")
            .iter()
            .map(|entry| entry.anchor.edit_as)
            .collect();
    assert_eq!(
        edit_as,
        vec![Some(EditAs::TwoCell), Some(EditAs::Absolute), None]
    );
}

#[test]
fn a_root_alternate_content_gives_its_first_choice_s_first_anchor_alone() {
    let choice = format!(
        "{}{}",
        two_cell(None, (1, 1), (2, 2), &frame("rId1")),
        two_cell(None, (3, 3), (4, 4), &frame("rId2"))
    );
    let drawing = drawing_xml(&format!(
        "<mc:AlternateContent xmlns:mc=\"{MC_NS}\"><mc:Choice Requires=\"a14\">{choice}\
         </mc:Choice><mc:Choice Requires=\"x\">{}</mc:Choice><mc:Fallback>{}</mc:Fallback>\
         </mc:AlternateContent>",
        anchored("rId2"),
        anchored("rId2")
    ));
    let entries = charts_of(with_charts(&drawing, &[simple(), simple()], "")).expect("read");
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].part, b"xl/charts/chart1.xml");
    assert_eq!(entries[0].anchor.from, (1, 1));
}

#[test]
fn a_sheet_with_no_drawing_has_no_charts() {
    let mut parts = smallest();
    set(&mut parts, SHEET, sheet_r("<sheetData/>"));
    assert_eq!(charts_of(parts).expect("read"), Vec::new());
}

#[test]
fn a_drawing_with_no_chart_has_none() {
    let drawing = drawing_xml(&[one_cell((0, 0), &pic()), absolute(&other_frame())].concat());
    assert_eq!(
        charts_of(with_charts(&drawing, &[], "")).expect("read"),
        Vec::new()
    );
}

#[test]
fn a_chartsheet_part_is_refused_as_not_a_worksheet() {
    // P1 ruling B: the existing message of the drawing walk.
    let mut parts = smallest();
    set(
        &mut parts,
        SHEET,
        format!(
            "{DECLARATION}\n<chartsheet xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">\
             <drawing r:id=\"rId1\"/></chartsheet>"
        ),
    );
    assert_package(
        charts_of(parts).expect_err("refused"),
        "the worksheet part's root is not worksheet",
    );
}

#[test]
fn a_part_the_archive_lacks_is_no_such_part() {
    let error = package(smallest())
        .charts(b"xl/worksheets/sheet9.xml")
        .expect_err("refused");
    assert_package(error, "no such part");
}

#[test]
fn a_chart_s_anchor_without_a_row_or_with_an_unknown_edit_as_is_refused() {
    let no_row = drawing_xml(
        &two_cell(None, (0, 0), (1, 1), &frame("rId1")).replace("<xdr:row>0</xdr:row>", ""),
    );
    let error = charts_of(with_charts(&no_row, &[simple()], "")).expect_err("refused");
    assert_package(error, DRAWING_UNREADABLE);
    let bad = drawing_xml(&two_cell(Some("sideways"), (0, 0), (1, 1), &frame("rId1")));
    let error = charts_of(with_charts(&bad, &[simple()], "")).expect_err("refused");
    assert_package(error, DRAWING_UNREADABLE);
}

#[test]
fn the_strict_namespaces_and_relationship_type_read() {
    const ATTR_STRICT_NS: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships";
    let drawing = drawing_in(
        XDR_STRICT,
        A_STRICT,
        C_STRICT,
        &two_cell(Some("oneCell"), (2, 3), (4, 5), &frame("rId1")).replace(C_NS, C_STRICT),
    );
    let mut parts = with_charts(&drawing, &[chart_in(C_STRICT, &simple_body())], "");
    set(
        &mut parts,
        b"xl/drawings/_rels/drawing1.xml.rels",
        rels(&relationship(
            "rId1",
            &format!("{ATTR_STRICT_NS}/chart"),
            "../charts/chart1.xml",
        )),
    );
    let entries = charts_of(parts).expect("read");
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].anchor.from, (2, 3));
    assert_eq!(entries[0].chart.plots[0].kind, ChartKind::Bar);
}

/// The `c:chart` body of [`simple`].
fn simple_body() -> String {
    format!(
        "<c:plotArea><c:layout/>{}{}{}</c:plotArea>",
        plot(
            "barChart",
            "<c:barDir val=\"col\"/>",
            &ser(
                0,
                &format!("<c:val>{}</c:val>", num_ref("Data!$B$2:$B$4", 3))
            )
        ),
        axis("catAx", 10, "b", 20, ""),
        axis("valAx", 20, "l", 10, "")
    )
}

// --------------------------------------------------------- kinds and plots

#[test]
fn each_chart_kind_reads() {
    let kinds = [
        ("barChart", ChartKind::Bar),
        ("bar3DChart", ChartKind::Bar3D),
        ("lineChart", ChartKind::Line),
        ("line3DChart", ChartKind::Line3D),
        ("pieChart", ChartKind::Pie),
        ("pie3DChart", ChartKind::Pie3D),
        ("doughnutChart", ChartKind::Doughnut),
        ("ofPieChart", ChartKind::OfPie),
        ("scatterChart", ChartKind::Scatter),
        ("areaChart", ChartKind::Area),
        ("area3DChart", ChartKind::Area3D),
        ("radarChart", ChartKind::Radar),
        ("bubbleChart", ChartKind::Bubble),
        ("stockChart", ChartKind::Stock),
        ("surfaceChart", ChartKind::Surface),
        ("surface3DChart", ChartKind::Surface3D),
    ];
    let charts: Vec<String> = kinds
        .iter()
        .map(|(element, _)| {
            plots_xml(&plot(
                element,
                "",
                &ser(0, &format!("<c:val>{}</c:val>", num_ref("Data!$B$2", 1))),
            ))
        })
        .collect();
    let anchors: String = (1..=kinds.len())
        .map(|k| anchored(&format!("rId{k}")))
        .collect();
    let entries = charts_of(with_charts(&drawing_xml(&anchors), &charts, "")).expect("read");
    let read: Vec<(ChartKind, usize)> = entries
        .iter()
        .map(|entry| (entry.chart.plots[0].kind, entry.chart.plots[0].series.len()))
        .collect();
    let expected: Vec<(ChartKind, usize)> = kinds.iter().map(|(_, kind)| (*kind, 1)).collect();
    assert_eq!(read, expected);
}

#[test]
fn a_combined_chart_reads_each_plot_with_its_axis_ids() {
    let plots = plots_of(&format!(
        "{}{}",
        plot(
            "barChart",
            "<c:barDir val=\"col\"/><c:grouping val=\"clustered\"/><c:varyColors val=\"0\"/>",
            &ser(0, "")
        ),
        plot(
            "lineChart",
            "<c:grouping val=\"standard\"/><c:varyColors val=\"1\"/>",
            &ser(1, "")
        )
    ));
    assert_eq!(
        plots
            .iter()
            .map(|plot| (plot.kind, plot.vary_colors, plot.axis_ids.clone()))
            .collect::<Vec<_>>(),
        vec![
            (ChartKind::Bar, false, vec![10, 20]),
            (ChartKind::Line, true, vec![10, 20]),
        ]
    );
}

#[test]
fn an_axis_id_reads_as_a_signed_integer_as_written() {
    // P1 ruling C: Excel writes the signed 32-bit value it holds; a `+`
    // sign is accepted, as Rust's integer parse accepts it.
    let plots = one_chart(axes_xml(
        "<c:lineChart><c:axId val=\"-2028358176\"/><c:axId val=\"+7\"/>\
         <c:axId val=\"4294967295\"/></c:lineChart>",
        &format!(
            "{}{}{}",
            axis("catAx", -2_028_358_176, "b", 7, ""),
            axis("valAx", 7, "l", -2_028_358_176, ""),
            axis("serAx", 4_294_967_295, "b", 7, "")
        ),
    ))
    .plots;
    assert_eq!(plots[0].axis_ids, vec![-2_028_358_176, 7, 4_294_967_295]);
}

#[test]
fn each_bar_direction_and_grouping_reads_with_the_schema_defaults() {
    let plots = plots_of(
        &[
            plot(
                "barChart",
                "<c:barDir val=\"bar\"/><c:grouping val=\"clustered\"/>",
                "",
            ),
            plot(
                "barChart",
                "<c:barDir val=\"col\"/><c:grouping val=\"stacked\"/>",
                "",
            ),
            plot(
                "barChart",
                "<c:barDir/><c:grouping val=\"percentStacked\"/>",
                "",
            ),
            plot(
                "bar3DChart",
                "<c:barDir val=\"bar\"/><c:grouping val=\"standard\"/>",
                "",
            ),
            plot("barChart", "<c:barDir val=\"col\"/><c:grouping/>", ""),
            plot("lineChart", "<c:grouping/>", ""),
            plot("areaChart", "<c:grouping val=\"stacked\"/>", ""),
            plot("pieChart", "<c:varyColors val=\"1\"/>", ""),
        ]
        .concat(),
    );
    assert_eq!(
        plots
            .iter()
            .map(|plot| (plot.bar_direction, plot.grouping))
            .collect::<Vec<_>>(),
        vec![
            (Some(BarDirection::Bar), Some(Grouping::Clustered)),
            (Some(BarDirection::Column), Some(Grouping::Stacked)),
            (Some(BarDirection::Column), Some(Grouping::PercentStacked)),
            (Some(BarDirection::Bar), Some(Grouping::Standard)),
            (Some(BarDirection::Column), Some(Grouping::Clustered)),
            (None, Some(Grouping::Standard)),
            (None, Some(Grouping::Stacked)),
            (None, None),
        ]
    );
}

#[test]
fn each_scatter_style_reads_and_x_and_y_values_fill_categories_and_values() {
    let styles = [
        ("none", ScatterStyle::None),
        ("line", ScatterStyle::Line),
        ("lineMarker", ScatterStyle::LineMarker),
        ("marker", ScatterStyle::Marker),
        ("smooth", ScatterStyle::Smooth),
        ("smoothMarker", ScatterStyle::SmoothMarker),
    ];
    let series = ser(
        0,
        &format!(
            "<c:xVal>{}</c:xVal><c:yVal>{}</c:yVal><c:smooth val=\"0\"/>",
            num_ref("Data!$A$2:$A$3", 2),
            num_ref("Data!$B$2:$B$3", 2)
        ),
    );
    let mut plots: String = styles
        .iter()
        .map(|(value, _)| {
            plot(
                "scatterChart",
                &format!("<c:scatterStyle val=\"{value}\"/>"),
                &series,
            )
        })
        .collect();
    plots.push_str(&plot("scatterChart", "<c:scatterStyle/>", &series));
    let read = plots_of(&plots);
    let mut expected: Vec<Option<ScatterStyle>> =
        styles.iter().map(|(_, style)| Some(*style)).collect();
    expected.push(Some(ScatterStyle::Marker));
    assert_eq!(
        read.iter()
            .map(|plot| plot.scatter_style)
            .collect::<Vec<_>>(),
        expected
    );
    let first = &read[0].series[0];
    assert_eq!(
        first.categories,
        Some(Data::Reference {
            formula: "Data!$A$2:$A$3".to_owned(),
            cached: Some(2)
        })
    );
    assert_eq!(
        first.values,
        Some(Data::Reference {
            formula: "Data!$B$2:$B$3".to_owned(),
            cached: Some(2)
        })
    );
}

#[test]
fn a_bubble_plot_s_sizes_read() {
    let plots = plots_of(&plot(
        "bubbleChart",
        "<c:varyColors val=\"0\"/>",
        &ser(
            0,
            &format!(
                "<c:xVal>{}</c:xVal><c:yVal>{}</c:yVal><c:bubbleSize>{}</c:bubbleSize>\
                 <c:bubble3D val=\"0\"/>",
                num_ref("Data!$A$2:$A$4", 3),
                num_ref("Data!$B$2:$B$4", 3),
                num_ref("Data!$C$2:$C$4", 3)
            ),
        ),
    ));
    assert_eq!(
        plots[0].series[0].sizes,
        Some(Data::Reference {
            formula: "Data!$C$2:$C$4".to_owned(),
            cached: Some(3)
        })
    );
}

// ------------------------------------------------------------- the series

#[test]
fn each_series_name_form_reads_and_none_is_none() {
    let series = series_of(
        &[
            ser(0, &format!("<c:tx>{}</c:tx>", str_ref("Data!$B$1", 1))),
            ser(1, "<c:tx><c:v>Second</c:v></c:tx>"),
            ser(2, ""),
        ]
        .concat(),
    );
    assert_eq!(
        series
            .iter()
            .map(|series| series.name.clone())
            .collect::<Vec<_>>(),
        vec![
            Some(SeriesName::Reference("Data!$B$1".to_owned())),
            Some(SeriesName::Text("Second".to_owned())),
            None,
        ]
    );
}

#[test]
fn each_data_form_reads_with_and_without_a_cache() {
    let reference = |formula: &str, cached: Option<u64>| Data::Reference {
        formula: formula.to_owned(),
        cached,
    };
    let cases = [
        (
            num_ref("Data!$A$1:$A$3", 3),
            reference("Data!$A$1:$A$3", Some(3)),
        ),
        (
            "<c:numRef><c:f>Data!$A$1</c:f></c:numRef>".to_owned(),
            reference("Data!$A$1", None),
        ),
        (
            str_ref("Data!$B$1:$B$2", 2),
            reference("Data!$B$1:$B$2", Some(2)),
        ),
        (
            "<c:strRef><c:f>Data!$B$1</c:f></c:strRef>".to_owned(),
            reference("Data!$B$1", None),
        ),
        (
            "<c:multiLvlStrRef><c:f>Data!$A$1:$B$4</c:f><c:multiLvlStrCache>\
             <c:ptCount val=\"4\"/><c:lvl><c:pt idx=\"0\"><c:v>x</c:v></c:pt></c:lvl>\
             </c:multiLvlStrCache></c:multiLvlStrRef>"
                .to_owned(),
            reference("Data!$A$1:$B$4", Some(4)),
        ),
        (
            "<c:numRef><c:f>Data!$C$1</c:f><c:numCache><c:formatCode>General</c:formatCode>\
             </c:numCache></c:numRef>"
                .to_owned(),
            reference("Data!$C$1", Some(0)),
        ),
        (
            "<c:numLit><c:formatCode>General</c:formatCode><c:ptCount val=\"2\"/>\
             <c:pt idx=\"0\"><c:v>1</c:v></c:pt><c:pt idx=\"1\"><c:v>2</c:v></c:pt></c:numLit>"
                .to_owned(),
            Data::Literal { count: 2 },
        ),
        (
            "<c:strLit><c:ptCount val=\"1\"/><c:pt idx=\"0\"><c:v>a</c:v></c:pt></c:strLit>"
                .to_owned(),
            Data::Literal { count: 1 },
        ),
        ("<c:numLit/>".to_owned(), Data::Literal { count: 0 }),
    ];
    let series: String = cases
        .iter()
        .enumerate()
        .map(|(k, (form, _))| ser(k as u32, &format!("<c:cat>{form}</c:cat>")))
        .collect();
    let read: Vec<Option<Data>> = series_of(&series)
        .into_iter()
        .map(|series| series.categories)
        .collect();
    let expected: Vec<Option<Data>> = cases.into_iter().map(|(_, data)| Some(data)).collect();
    assert_eq!(read, expected);
}

#[test]
fn a_solid_rgb_fill_reads_as_a_number_and_a_scheme_fill_as_none() {
    let series = series_of(
        &[
            ser(
                0,
                "<c:spPr><a:solidFill><a:srgbClr val=\"4472C4\"><a:alpha val=\"50000\"/>\
                 </a:srgbClr></a:solidFill><a:ln><a:noFill/></a:ln></c:spPr>",
            ),
            ser(
                1,
                "<c:spPr><a:solidFill><a:schemeClr val=\"accent1\"/></a:solidFill></c:spPr>",
            ),
            ser(2, "<c:spPr><a:noFill/></c:spPr>"),
        ]
        .concat(),
    );
    assert_eq!(
        series.iter().map(|series| series.fill).collect::<Vec<_>>(),
        vec![Some(0x4472C4), None, None]
    );
}

#[test]
fn smooth_marker_and_labels_read() {
    let series = series_of(
        &[
            ser(
                0,
                "<c:marker><c:symbol val=\"none\"/></c:marker><c:dLbls><c:showVal val=\"1\"/>\
                 </c:dLbls><c:smooth val=\"1\"/>",
            ),
            ser(1, "<c:smooth/>"),
            ser(2, "<c:smooth val=\"0\"/>"),
        ]
        .concat(),
    );
    assert_eq!(
        series
            .iter()
            .map(|series| (series.smooth, series.marker, series.labels))
            .collect::<Vec<_>>(),
        vec![
            (true, true, true),
            (true, false, false),
            (false, false, false)
        ]
    );
}

#[test]
fn the_unmodelled_children_are_walked_past() {
    let mc = format!(
        "<mc:AlternateContent xmlns:mc=\"{MC_NS}\"><mc:Choice Requires=\"c14\">\
         <a:solidFill><a:srgbClr val=\"FF0000\"/></a:solidFill></mc:Choice><mc:Fallback>\
         <a:solidFill><a:srgbClr val=\"00FF00\"/></a:solidFill></mc:Fallback>\
         </mc:AlternateContent>"
    );
    let series = series_of(&ser(
        3,
        &format!(
            "<c:spPr>{mc}</c:spPr><c:invertIfNegative val=\"0\"/><c:dPt><c:idx val=\"0\"/>\
             <c:spPr><a:solidFill><a:srgbClr val=\"123456\"/></a:solidFill></c:spPr></c:dPt>\
             <c:trendline><c:trendlineType val=\"linear\"/></c:trendline>\
             <c:errBars><c:errDir val=\"y\"/><c:errBarType val=\"both\"/></c:errBars>\
             <c:val>{}</c:val><c:extLst><c:ext uri=\"{{x}}\"><c:idx val=\"9\"/></c:ext>\
             </c:extLst>",
            num_ref("Data!$B$2:$B$3", 2)
        ),
    ));
    assert_eq!(
        series,
        vec![Series {
            index: 3,
            order: 3,
            name: None,
            categories: None,
            values: Some(Data::Reference {
                formula: "Data!$B$2:$B$3".to_owned(),
                cached: Some(2)
            }),
            sizes: None,
            fill: None,
            smooth: false,
            marker: false,
            labels: false,
        }]
    );
}

// -------------------------------------------------------------- the titles

/// The chart part whose `c:chart` opens with `title` and then `deleted`.
fn titled(title: &str, deleted: &str) -> String {
    chart_xml(&format!(
        "{title}{deleted}<c:plotArea><c:layout/></c:plotArea>"
    ))
}

#[test]
fn a_rich_title_of_two_paragraphs_reads_its_runs_joined_by_a_newline() {
    let chart = one_chart(titled(
        "<c:title><c:tx><c:rich><a:bodyPr/><a:lstStyle/><a:p><a:pPr><a:defRPr/></a:pPr>\
         <a:r><a:rPr b=\"1\"/><a:t>First </a:t></a:r><a:r><a:t>line</a:t></a:r></a:p>\
         <a:p><a:r><a:t>Second</a:t></a:r><a:endParaRPr/></a:p></c:rich></c:tx>\
         <c:overlay val=\"0\"/></c:title>",
        "<c:autoTitleDeleted val=\"0\"/>",
    ));
    assert_eq!(chart.title.as_deref(), Some("First line\nSecond"));
    assert!(!chart.auto_title_deleted);
}

#[test]
fn a_reference_title_reads_its_first_point() {
    let chart = one_chart(titled(
        &format!(
            "<c:title><c:tx>{}</c:tx></c:title>",
            str_ref("Data!$A$1", 2)
        ),
        "",
    ));
    assert_eq!(chart.title.as_deref(), Some("p0"));
}

#[test]
fn a_title_with_no_text_and_no_title_are_none_and_auto_title_deleted_reads() {
    let chart = one_chart(titled(
        "<c:title><c:overlay val=\"0\"/></c:title>",
        "<c:autoTitleDeleted val=\"1\"/>",
    ));
    assert_eq!((chart.title, chart.auto_title_deleted), (None, true));
    let chart = one_chart(titled("", "<c:autoTitleDeleted/>"));
    assert_eq!((chart.title, chart.auto_title_deleted), (None, true));
    let chart = one_chart(titled("", ""));
    assert_eq!((chart.title, chart.auto_title_deleted), (None, false));
}

// ------------------------------------------------------------ the refusals

#[test]
fn a_chart_frame_whose_relationship_cannot_be_followed_is_refused() {
    // A relationship of another type naming a chart part that is there:
    // refused for its type alone.
    let image = relationship("rId7", &format!("{ATTR_NS}/image"), "../charts/chart1.xml");
    let external = format!(
        "<Relationship Id=\"rId8\" Type=\"{ATTR_NS}/chart\" Target=\"http://example.invalid/c\" \
         TargetMode=\"External\"/>"
    );
    for drawing in [
        drawing_xml(&two_cell(None, (0, 0), (1, 1), &frame_with(""))),
        drawing_xml(&anchored("rId5")),
        drawing_xml(&anchored("rId7")),
        drawing_xml(&anchored("rId8")),
    ] {
        let error = charts_of(with_charts(
            &drawing,
            &[simple()],
            &format!("{image}{external}"),
        ))
        .expect_err("refused");
        assert_package(error, UNREADABLE);
    }
    // The relationship's part is missing.
    let mut parts = with_charts(&drawing_xml(&anchored("rId1")), &[simple()], "");
    drop_part(&mut parts, b"xl/charts/chart1.xml");
    assert_package(charts_of(parts).expect_err("refused"), UNREADABLE);
}

#[test]
fn a_chart_part_without_its_root_chart_or_plot_area_is_refused() {
    for chart in [
        simple().replace("c:chartSpace", "c:chartSheet"),
        format!(
            "{DECLARATION}\n<c:chartSpace xmlns:c=\"{C_NS}\"><c:lang val=\"en-US\"/></c:chartSpace>"
        ),
        chart_xml("<c:autoTitleDeleted val=\"0\"/><c:plotVisOnly val=\"1\"/>"),
    ] {
        assert_package(refused_chart(chart), UNREADABLE);
    }
}

#[test]
fn the_series_and_value_refusals_are_refused_with_the_message() {
    let val = format!("<c:val>{}</c:val>", num_ref("Data!$B$2", 1));
    for plots in [
        // A `ser` with no `idx`, then no `order`.
        plot(
            "barChart",
            "",
            &format!("<c:ser><c:order val=\"0\"/>{val}</c:ser>"),
        ),
        plot(
            "barChart",
            "",
            &format!("<c:ser><c:idx val=\"0\"/>{val}</c:ser>"),
        ),
        // A reference with no `f`.
        plot(
            "barChart",
            "",
            &ser(0, "<c:val><c:numRef><c:numCache/></c:numRef></c:val>"),
        ),
        // Both `cat` and `xVal`.
        plot(
            "scatterChart",
            "",
            &ser(
                0,
                &format!(
                    "<c:cat>{}</c:cat><c:xVal>{}</c:xVal>",
                    str_ref("Data!$A$2", 1),
                    num_ref("Data!$A$2", 1)
                ),
            ),
        ),
        // Values outside their enumerations.
        plot("barChart", "<c:barDir val=\"sideways\"/>", ""),
        plot("barChart", "<c:grouping val=\"heaped\"/>", ""),
        plot("scatterChart", "<c:scatterStyle val=\"dotted\"/>", ""),
        // Numbers that are not numbers.
        plot(
            "barChart",
            "",
            &ser(
                0,
                "<c:val><c:numLit><c:ptCount val=\"many\"/></c:numLit></c:val>",
            ),
        ),
        plot(
            "barChart",
            "",
            "<c:ser><c:idx val=\"first\"/><c:order val=\"0\"/></c:ser>",
        ),
        "<c:barChart><c:axId val=\"x\"/></c:barChart>".to_owned(),
        "<c:barChart><c:axId val=\"1.0\"/></c:barChart>".to_owned(),
        "<c:barChart><c:axId val=\"9223372036854775808\"/></c:barChart>".to_owned(),
        plot(
            "barChart",
            "",
            &ser(
                0,
                "<c:spPr><a:solidFill><a:srgbClr val=\"blue\"/></a:solidFill></c:spPr>",
            ),
        ),
        plot("barChart", "<c:varyColors val=\"maybe\"/>", ""),
    ] {
        assert_package(refused_chart(plots_xml(&plots)), UNREADABLE);
    }
}

#[test]
fn a_chart_part_that_is_not_well_formed_is_an_xml_error() {
    assert_xml(refused_chart(simple().replace("</c:chartSpace>", "")));
}

#[test]
fn a_multi_level_string_reference_without_a_cache_reads_none_cached() {
    // The P1 verification's ruling G, carried to P2.
    let series = series_of(&ser(
        0,
        "<c:cat><c:multiLvlStrRef><c:f>Data!$A$1:$B$4</c:f></c:multiLvlStrRef></c:cat>",
    ));
    assert_eq!(
        series[0].categories,
        Some(Data::Reference {
            formula: "Data!$A$1:$B$4".to_owned(),
            cached: None,
        })
    );
}

// ------------------------------------------------------------------ the axes

/// The axes of a chart with one line plot naming `ids` and the axes `axes`.
#[track_caller]
fn axes_of(ids: &[i64], axes: &str) -> Vec<ChartAxis> {
    let ids: String = ids
        .iter()
        .map(|id| format!("<c:axId val=\"{id}\"/>"))
        .collect();
    one_chart(axes_xml(&format!("<c:lineChart>{ids}</c:lineChart>"), axes)).axes
}

#[test]
fn a_negative_axis_id_joins_a_negative_cross_axis() {
    // The P1 verification's ruling E (P1 ruling C's test), carried to P2:
    // the ids as Excel writes the signed 32-bit values it holds.
    let axes = axes_of(
        &[-2_028_358_176, -2_028_359_712],
        &format!(
            "{}{}",
            axis("catAx", -2_028_358_176, "b", -2_028_359_712, ""),
            axis("valAx", -2_028_359_712, "l", -2_028_358_176, "")
        ),
    );
    assert_eq!(
        axes.iter()
            .map(|axis| (axis.id, axis.crosses))
            .collect::<Vec<_>>(),
        vec![
            (-2_028_358_176, -2_028_359_712),
            (-2_028_359_712, -2_028_358_176)
        ]
    );
}

#[test]
fn each_axis_kind_and_position_reads() {
    let axes = axes_of(
        &[1, 2],
        &format!(
            "{}{}{}{}",
            axis("catAx", 1, "b", 2, ""),
            axis("valAx", 2, "l", 1, ""),
            axis("dateAx", 3, "r", 2, ""),
            axis("serAx", 4, "t", 2, "")
        ),
    );
    assert_eq!(
        axes.iter()
            .map(|axis| (axis.id, axis.kind, axis.position))
            .collect::<Vec<_>>(),
        vec![
            (1, AxisKind::Category, AxisPosition::Bottom),
            (2, AxisKind::Value, AxisPosition::Left),
            (3, AxisKind::Date, AxisPosition::Right),
            (4, AxisKind::Series, AxisPosition::Top),
        ]
    );
}

#[test]
fn an_axis_s_title_number_format_bounds_deletion_and_gridlines_read() {
    let full = "<c:valAx><c:axId val=\"2\"/><c:scaling><c:orientation val=\"minMax\"/>\
                <c:max val=\"100.5\"/><c:min val=\"-3\"/></c:scaling><c:delete val=\"1\"/>\
                <c:axPos val=\"l\"/><c:majorGridlines/><c:title><c:tx><c:rich><a:bodyPr/>\
                <a:p><a:r><a:t>Units</a:t></a:r></a:p></c:rich></c:tx><c:overlay val=\"0\"/>\
                </c:title><c:numFmt formatCode=\"0.0%\" sourceLinked=\"0\"/>\
                <c:majorTickMark val=\"out\"/><c:crossAx val=\"1\"/><c:crosses val=\"max\"/>\
                </c:valAx>";
    let axes = axes_of(&[1, 2], &format!("{}{full}", axis("catAx", 1, "b", 2, "")));
    assert_eq!(
        axes,
        vec![
            ChartAxis {
                id: 1,
                kind: AxisKind::Category,
                position: AxisPosition::Bottom,
                crosses: 2,
                title: None,
                number_format: None,
                min: None,
                max: None,
                deleted: false,
                major_gridlines: false,
            },
            ChartAxis {
                id: 2,
                kind: AxisKind::Value,
                position: AxisPosition::Left,
                crosses: 1,
                title: Some("Units".to_owned()),
                number_format: Some("0.0%".to_owned()),
                min: Some(-3.0),
                max: Some(100.5),
                deleted: true,
                major_gridlines: true,
            },
        ]
    );
}

// -------------------------------------------- the legend, pivot source, style

#[test]
fn each_legend_position_reads_and_one_without_a_position_is_right() {
    let legend_of = |legend: &str| {
        one_chart(chart_xml(&format!(
            "<c:plotArea><c:layout/></c:plotArea>{legend}<c:plotVisOnly val=\"1\"/>"
        )))
        .legend
    };
    for (value, position) in [
        ("b", LegendPosition::Bottom),
        ("l", LegendPosition::Left),
        ("r", LegendPosition::Right),
        ("t", LegendPosition::Top),
        ("tr", LegendPosition::TopRight),
    ] {
        assert_eq!(
            legend_of(&format!(
                "<c:legend><c:legendPos val=\"{value}\"/><c:overlay val=\"0\"/></c:legend>"
            )),
            Some(position)
        );
    }
    assert_eq!(
        legend_of("<c:legend><c:overlay val=\"0\"/></c:legend>"),
        Some(LegendPosition::Right)
    );
    assert_eq!(legend_of(""), None);
}

/// A chart part with the root children `before`, around [`simple`]'s body.
fn with_root(before: &str) -> Chart {
    one_chart(chart_space(C_NS, before, &simple_body()))
}

#[test]
fn a_pivot_source_reads_its_name() {
    let chart = with_root(
        "<c:pivotSource><c:name>[Book.xlsx]Pivot!PivotTable1</c:name><c:fmtId val=\"0\"/>\
         </c:pivotSource>",
    );
    assert_eq!(
        chart.pivot_source.as_deref(),
        Some("[Book.xlsx]Pivot!PivotTable1")
    );
    assert_eq!(with_root("").pivot_source, None);
}

#[test]
fn a_style_reads_plain_and_from_the_first_choice_not_the_fallback() {
    const C14: &str = "http://schemas.microsoft.com/office/drawing/2007/8/2/chart";
    assert_eq!(with_root("<c:style val=\"2\"/>").style, Some(2));
    let alternate = |choice: &str| {
        format!(
            "<mc:AlternateContent xmlns:mc=\"{MC_NS}\"><mc:Choice Requires=\"c14\" \
             xmlns:c14=\"{C14}\">{choice}</mc:Choice><mc:Fallback><c:style val=\"2\"/>\
             </mc:Fallback></mc:AlternateContent>"
        )
    };
    assert_eq!(
        with_root(&alternate("<c14:style val=\"102\"/>")).style,
        Some(102)
    );
    // A Choice holding no style: the Fallback's is not read.
    assert_eq!(with_root(&alternate("")).style, None);
    assert_eq!(with_root("").style, None);
}

#[test]
fn the_axis_and_legend_refusals_are_refused_with_the_message() {
    let line = "<c:lineChart><c:axId val=\"1\"/><c:axId val=\"2\"/></c:lineChart>";
    let cat = axis("catAx", 1, "b", 2, "");
    let val = axis("valAx", 2, "l", 1, "");
    for axes in [
        // No axId, no axPos, no crossAx.
        format!("{cat}{}", val.replace("<c:axId val=\"2\"/>", "")),
        format!("{cat}{}", val.replace("<c:axPos val=\"l\"/>", "")),
        format!("{cat}{}", val.replace("<c:crossAx val=\"1\"/>", "")),
        // Values outside their enumerations.
        format!("{cat}{}", val.replace("val=\"l\"", "val=\"middle\"")),
        format!("{cat}{}", val.replace("val=\"autoZero\"", "val=\"centre\"")),
        // Numbers that are not numbers.
        format!(
            "{cat}{}",
            val.replace("<c:axId val=\"2\"/>", "<c:axId val=\"two\"/>")
        ),
        format!(
            "{cat}{}",
            val.replace("<c:crossAx val=\"1\"/>", "<c:crossAx val=\"1.0\"/>")
        ),
        format!(
            "{cat}{}",
            val.replace("</c:scaling>", "<c:min val=\"low\"/></c:scaling>")
        ),
        // A plot's axId naming no axis of the chart.
        cat.clone(),
    ] {
        assert_package(refused_chart(axes_xml(line, &axes)), UNREADABLE);
    }
    for chart in [
        chart_xml(
            "<c:plotArea><c:layout/></c:plotArea><c:legend><c:legendPos val=\"centre\"/>\
             </c:legend>",
        ),
        chart_space(C_NS, "<c:style val=\"plain\"/>", &simple_body()),
    ] {
        assert_package(refused_chart(chart), UNREADABLE);
    }
}

// The three refusals of P2 ruling N: a required value absent.

#[test]
fn a_crosses_without_a_val_is_refused() {
    let line = "<c:lineChart><c:axId val=\"1\"/><c:axId val=\"2\"/></c:lineChart>";
    let axes = format!(
        "{}{}",
        axis("catAx", 1, "b", 2, ""),
        axis("valAx", 2, "l", 1, "").replace("<c:crosses val=\"autoZero\"/>", "<c:crosses/>")
    );
    assert_package(refused_chart(axes_xml(line, &axes)), UNREADABLE);
}

#[test]
fn a_number_format_without_a_format_code_is_refused() {
    let line = "<c:lineChart><c:axId val=\"1\"/><c:axId val=\"2\"/></c:lineChart>";
    let axes = format!(
        "{}{}",
        axis("catAx", 1, "b", 2, "<c:numFmt sourceLinked=\"1\"/>"),
        axis("valAx", 2, "l", 1, "")
    );
    assert_package(refused_chart(axes_xml(line, &axes)), UNREADABLE);
}

#[test]
fn a_pivot_source_without_a_name_is_refused() {
    assert_package(
        refused_chart(chart_space(
            C_NS,
            "<c:pivotSource><c:fmtId val=\"0\"/></c:pivotSource>",
            &simple_body(),
        )),
        UNREADABLE,
    );
}
