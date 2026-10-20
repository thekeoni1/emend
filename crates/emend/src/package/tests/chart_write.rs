//! Charts written on byte-built packages (`specs/0035-charts-write-first-
//! set.md`, "P1: the writer, bar and line charts", P1 rulings A to F, and
//! "P2: pie, scatter and area charts; the catalogue"; `specs/0036-charts-
//! write-second-set.md`, "P1: the compaction, doughnut, radar, bubble and
//! stock charts", "P2: the 3D kinds and combined charts", P2 rulings F
//! to J, and "P3: labels, colours, number formats; `remove_chart`; the
//! catalogue", P3 rulings L and O; `specs/0039-chart-kinds-and-
//! formatting.md`, "P1: the surface and of-pie kinds" and "P2: markers,
//! lines and points"; `specs/0040-analytic-chart-elements.md`, "P1:
//! trendlines and error bars"):
//! each test builds the smallest package, records `add_chart` calls on its
//! worksheet, writes, compares the parts the design gives byte for byte
//! where it names them, and reads every chart written back through
//! `charts`, field by field. Every content is fictional. The Cargo feature
//! `charts`; the tests that also add or remove an image, `images` too.

use super::*;
use crate::package::{
    Anchor, AxisKind, AxisPosition, BarDirection, Chart, ChartAnchor, ChartAxis, ChartEntry,
    ChartKind, DashStyle, Data, EditAs, ErrorBarKind, ErrorDirection, ErrorValue, Grouping,
    ImageAnchor, LabelPosition, LegendPosition, MarkerSymbol, NewChart, NewCombined, NewErrorBars,
    NewLabels, NewLine, NewMarker, NewPlot, NewPoint, NewSeries, NewSeriesName, NewTrendline,
    OfPieKind, OfPieSplit, Plot, RadarStyle, ScatterStyle, Series, SeriesName, TrendlineKind,
};

/// The drawing and chart parts' content types.
const DRAWING_TYPE: &str = "application/vnd.openxmlformats-officedocument.drawing+xml";
const CHART_TYPE: &str = "application/vnd.openxmlformats-officedocument.drawingml.chart+xml";

/// The spreadsheet-drawing, DrawingML and chart namespaces, transitional
/// and strict, and the strict relationships namespace.
const XDR_NS: &str = "http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing";
const A_NS: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";
const C_NS: &str = "http://schemas.openxmlformats.org/drawingml/2006/chart";
const XDR_STRICT: &str = "http://purl.oclc.org/ooxml/drawingml/spreadsheetDrawing";
const A_STRICT: &str = "http://purl.oclc.org/ooxml/drawingml/main";
const C_STRICT: &str = "http://purl.oclc.org/ooxml/drawingml/chart";
const R_STRICT: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships";

/// The parts the fixtures and the writes name.
const DRAWING: &[u8] = b"xl/drawings/drawing1.xml";
const DRAWING_RELS: &[u8] = b"xl/drawings/_rels/drawing1.xml.rels";
const CHART1: &[u8] = b"xl/charts/chart1.xml";

/// The messages of the part, and the ones it shares.
const TEXT: &str = "a chart text is empty or holds a control character";
const OPTION: &str = "a chart option does not apply to its kind";
const SIZE: &str = "a chart size does not apply to its anchor";
const ANCHOR: &str = "a chart anchor is out of range";
const GRID: &str = "a row or column index or count is out of range";
const DRAWING_UNREADABLE: &str = "a drawing part or its relationship cannot be read";
const RANGE: &str = "a chart value is out of its range";

/// The worksheet body the drawing fixtures use.
const BODY: &str = "<sheetData/><drawing r:id=\"rId1\"/>";

/// A worksheet part whose root binds `r` to the relationships namespace.
fn sheet_r(body: &str) -> String {
    format!(
        "{DECLARATION}\n<worksheet xmlns=\"{MAIN_NS}\" xmlns:r=\"{ATTR_NS}\">{body}</worksheet>"
    )
}

/// A drawing part binding `xdr` and `a` on its root, with `children`.
fn drawing_xml(children: &str) -> String {
    format!(
        "{DECLARATION}\n<xdr:wsDr xmlns:xdr=\"{XDR_NS}\" xmlns:a=\"{A_NS}\">{children}</xdr:wsDr>"
    )
}

/// A picture as Excel writes one, its `cNvPr` `id` and its `blip` naming
/// `embed`.
fn pic(id: u32, embed: &str) -> String {
    format!(
        "<xdr:pic><xdr:nvPicPr><xdr:cNvPr id=\"{id}\" name=\"Picture {id}\"/><xdr:cNvPicPr/>\
         </xdr:nvPicPr><xdr:blipFill><a:blip xmlns:r=\"{ATTR_NS}\" r:embed=\"{embed}\"/>\
         <a:stretch><a:fillRect/></a:stretch></xdr:blipFill><xdr:spPr><a:prstGeom prst=\"rect\">\
         <a:avLst/></a:prstGeom></xdr:spPr></xdr:pic>"
    )
}

/// A kept two-cell anchor at zero-based `column` and `row` holding
/// `content`.
fn kept_anchor(column: u32, row: u32, content: &str) -> String {
    format!(
        "<xdr:twoCellAnchor><xdr:from><xdr:col>{column}</xdr:col><xdr:colOff>0</xdr:colOff>\
         <xdr:row>{row}</xdr:row><xdr:rowOff>0</xdr:rowOff></xdr:from><xdr:to><xdr:col>9\
         </xdr:col><xdr:colOff>0</xdr:colOff><xdr:row>9</xdr:row><xdr:rowOff>0</xdr:rowOff>\
         </xdr:to>{content}<xdr:clientData/></xdr:twoCellAnchor>"
    )
}

/// The smallest package with its worksheet's body `body` naming the drawing
/// part `drawing` as `rId1`, with its `Override`, its rels part holding
/// `drawing_rels` when given, and a PNG at `xl/media/image1.png` when
/// `media`.
fn with_drawing(body: &str, drawing: String, drawing_rels: Option<&str>, media: bool) -> Parts {
    let mut parts = smallest();
    set(&mut parts, SHEET, sheet_r(body));
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
        &drawing,
        Some(DRAWING_TYPE),
    );
    if let Some(body) = drawing_rels {
        with_part(
            &mut parts,
            "xl/drawings/_rels/drawing1.xml.rels",
            &rels(body),
            None,
        );
    }
    if media {
        parts.push((b"xl/media/image1.png".to_vec(), PNG.to_vec()));
    }
    parts
}

/// The 82-byte PNG of the image tests, a 2 by 2 RGB picture.
const PNG: [u8; 82] = [
    0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x08, 0x02, 0x00, 0x00, 0x00, 0xfd, 0xd4, 0x9a,
    0x73, 0x00, 0x00, 0x00, 0x19, 0x49, 0x44, 0x41, 0x54, 0x78, 0x01, 0x01, 0x0e, 0x00, 0xf1, 0xff,
    0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0x1f, 0xee,
    0x05, 0xfb, 0xde, 0xdd, 0xec, 0x2b, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42,
    0x60, 0x82,
];

/// A part of the output, read back through a reopened package.
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

/// The charts `charts` reads from the output's worksheet.
fn charts_of(output: &[u8]) -> Vec<ChartEntry> {
    let mut package = Package::open(Cursor::new(output.to_vec())).expect("open the output");
    package.charts(SHEET).expect("charts")
}

/// The refusal a write of `edit` on `parts` gives, with no output.
#[track_caller]
fn refused_with(parts: Parts, edit: impl FnOnce(&mut Package<Cursor<Vec<u8>>>)) -> Error {
    let (_, mut package) = opened(parts);
    edit(&mut package);
    refused(package)
}

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
/// titles and a legend at the bottom, sized for no anchor.
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
        size: None,
        combined: None,
    }
}

/// A chart with one series and nothing optional, of `plot`.
fn bare(plot: NewPlot) -> NewChart<'static> {
    const ONE: [NewSeries<'static>; 1] = [NewSeries {
        name: None,
        categories: None,
        values: "Sheet1!$B$2:$B$4",
        sizes: None,
        color: None,
        labels: None,
        marker: None,
        line: None,
        points: &[],
        trendline: None,
        error_bars: &[],
    }];
    NewChart {
        plot,
        title: None,
        series: &ONE,
        category_axis_title: None,
        value_axis_title: None,
        category_axis_format: None,
        value_axis_format: None,
        legend: None,
        size: None,
        combined: None,
    }
}

/// A two-cell anchor from B3 to F12 (zero-based 1,2 to 5,11), `editAs`
/// `oneCell`.
fn box_anchor() -> ImageAnchor {
    ImageAnchor::TwoCell {
        from: at("B3"),
        to: at("F12"),
        from_offset: (0, 0),
        to_offset: (0, 0),
        edit_as: EditAs::OneCell,
    }
}

/// Records `chart` at `anchor` on the worksheet.
#[track_caller]
fn add(package: &mut Package<Cursor<Vec<u8>>>, anchor: ImageAnchor, chart: &NewChart<'_>) {
    package.add_chart(SHEET, anchor, chart).expect("recorded");
}

/// What a call gives on the smallest package, and that nothing was
/// recorded when it refuses: the package then writes its input back.
#[track_caller]
fn call(anchor: ImageAnchor, chart: &NewChart<'_>) -> Result<(), Error> {
    let (_, mut package) = opened(smallest());
    let result = package.add_chart(SHEET, anchor, chart);
    if result.is_err() {
        let output = written(package);
        assert!(
            charts_of(&output).is_empty(),
            "a refused call records nothing"
        );
    }
    result
}

/// The `graphicFrame` the writer adds, in the created drawing's form, its
/// `cNvPr` `id` and its `c:chart` naming `rel`.
fn frame(id: u32, rel: &str) -> String {
    format!(
        "<xdr:graphicFrame macro=\"\"><xdr:nvGraphicFramePr><xdr:cNvPr id=\"{id}\" \
         name=\"Chart {id}\"/><xdr:cNvGraphicFramePr/></xdr:nvGraphicFramePr><xdr:xfrm>\
         <a:off x=\"0\" y=\"0\"/><a:ext cx=\"0\" cy=\"0\"/></xdr:xfrm><a:graphic>\
         <a:graphicData uri=\"{C_NS}\"><c:chart xmlns:c=\"{C_NS}\" xmlns:r=\"{ATTR_NS}\" \
         r:id=\"{rel}\"/></a:graphicData></a:graphic></xdr:graphicFrame>"
    )
}

/// The two-cell anchor `box_anchor` writes, holding `content`.
fn written_box(content: &str) -> String {
    format!(
        "<xdr:twoCellAnchor editAs=\"oneCell\"><xdr:from><xdr:col>1</xdr:col><xdr:colOff>0\
         </xdr:colOff><xdr:row>2</xdr:row><xdr:rowOff>0</xdr:rowOff></xdr:from><xdr:to>\
         <xdr:col>5</xdr:col><xdr:colOff>0</xdr:colOff><xdr:row>11</xdr:row><xdr:rowOff>0\
         </xdr:rowOff></xdr:to>{content}<xdr:clientData/></xdr:twoCellAnchor>"
    )
}

/// A title's form with `text`.
fn title(text: &str) -> String {
    format!(
        "<c:title><c:tx><c:rich><a:bodyPr/><a:lstStyle/><a:p><a:r><a:t>{text}</a:t></a:r>\
         </a:p></c:rich></c:tx><c:overlay val=\"0\"/></c:title>"
    )
}

/// The two axes, the category axis at `cat` and the value axis at `val`,
/// with their titles' forms.
fn axes(cat: &str, val: &str, cat_title: &str, val_title: &str) -> String {
    format!(
        "<c:catAx><c:axId val=\"10\"/><c:scaling><c:orientation val=\"minMax\"/></c:scaling>\
         <c:delete val=\"0\"/><c:axPos val=\"{cat}\"/>{cat_title}<c:numFmt formatCode=\"General\" \
         sourceLinked=\"1\"/><c:majorTickMark val=\"out\"/><c:minorTickMark val=\"none\"/>\
         <c:tickLblPos val=\"nextTo\"/><c:crossAx val=\"20\"/><c:crosses val=\"autoZero\"/>\
         <c:auto val=\"1\"/><c:lblAlgn val=\"ctr\"/><c:lblOffset val=\"100\"/>\
         <c:noMultiLvlLbl val=\"0\"/></c:catAx><c:valAx><c:axId val=\"20\"/><c:scaling>\
         <c:orientation val=\"minMax\"/></c:scaling><c:delete val=\"0\"/><c:axPos val=\"{val}\"/>\
         <c:majorGridlines/>{val_title}<c:numFmt formatCode=\"General\" sourceLinked=\"1\"/>\
         <c:majorTickMark val=\"out\"/><c:minorTickMark val=\"none\"/><c:tickLblPos \
         val=\"nextTo\"/><c:crossAx val=\"10\"/><c:crosses val=\"autoZero\"/><c:crossBetween \
         val=\"between\"/></c:valAx>"
    )
}

/// A chart part in the transitional form around `chart`, the content of
/// `c:chart`.
fn chart_part(chart: &str) -> String {
    format!(
        "{DECLARATION}\n<c:chartSpace xmlns:c=\"{C_NS}\" xmlns:a=\"{A_NS}\"><c:roundedCorners \
         val=\"0\"/><c:chart>{chart}</c:chart></c:chartSpace>"
    )
}

/// A reference form `form` holding `formula`.
fn reference(form: &str, formula: &str) -> String {
    format!("<c:{form}><c:f>{formula}</c:f></c:{form}>")
}

/// A series as written, its references as given.
fn series_ref(name: Option<SeriesName>, categories: Option<&str>, values: &str, k: u32) -> Series {
    let data = |formula: &str| Data::Reference {
        formula: formula.to_owned(),
        cached: None,
    };
    Series {
        index: k,
        order: k,
        name,
        categories: categories.map(data),
        values: Some(data(values)),
        sizes: None,
        fill: None,
        smooth: false,
        marker: false,
        labels: false,
    }
}

/// The two axes as read: the category axis at `cat` and the value axis at
/// `val`, with their titles.
fn read_axes(
    cat: AxisPosition,
    val: AxisPosition,
    cat_title: Option<&str>,
    val_title: Option<&str>,
) -> Vec<ChartAxis> {
    let axis = |id, kind, position, crosses, title: Option<&str>, gridlines| ChartAxis {
        id,
        kind,
        position,
        crosses,
        title: title.map(str::to_owned),
        number_format: Some("General".to_owned()),
        min: None,
        max: None,
        deleted: false,
        major_gridlines: gridlines,
    };
    vec![
        axis(10, AxisKind::Category, cat, 20, cat_title, false),
        axis(20, AxisKind::Value, val, 10, val_title, true),
    ]
}

/// The column chart as `charts` reads it.
fn column_read() -> Chart {
    Chart {
        title: Some("Sales".to_owned()),
        auto_title_deleted: false,
        plots: vec![Plot {
            kind: ChartKind::Bar,
            bar_direction: Some(BarDirection::Column),
            grouping: Some(Grouping::Clustered),
            scatter_style: None,
            vary_colors: false,
            series: vec![
                series_ref(
                    Some(SeriesName::Text("North".to_owned())),
                    Some("Sheet1!$A$2:$A$4"),
                    "Sheet1!$B$2:$B$4",
                    0,
                ),
                series_ref(
                    Some(SeriesName::Reference("Sheet1!$C$1".to_owned())),
                    None,
                    "Sheet1!$C$2:$C$4",
                    1,
                ),
            ],
            axis_ids: vec![10, 20],
        }],
        axes: read_axes(
            AxisPosition::Bottom,
            AxisPosition::Left,
            Some("Month"),
            Some("Units"),
        ),
        legend: Some(LegendPosition::Bottom),
        pivot_source: None,
        style: None,
    }
}

/// The anchor `box_anchor` reads back as.
const BOX_READ: ChartAnchor = ChartAnchor {
    kind: Anchor::TwoCell,
    from: (1, 2),
    to: Some((5, 11)),
    edit_as: Some(EditAs::OneCell),
};

#[test]
fn a_column_chart_on_a_sheet_with_no_drawing_creates_the_parts() {
    let mut parts = smallest();
    set(&mut parts, SHEET, sheet_r("<sheetData/>"));
    let output = edited_output(parts.clone(), |p| add(p, box_anchor(), &column_chart()));
    assert_untouched(&output, &parts, &[SHEET, b"[Content_Types].xml".as_slice()]);
    let series = format!(
        "<c:ser><c:idx val=\"0\"/><c:order val=\"0\"/><c:tx><c:v>North</c:v></c:tx><c:cat>{}\
         </c:cat><c:val>{}</c:val></c:ser><c:ser><c:idx val=\"1\"/><c:order val=\"1\"/><c:tx>{}\
         </c:tx><c:val>{}</c:val></c:ser>",
        reference("strRef", "Sheet1!$A$2:$A$4"),
        reference("numRef", "Sheet1!$B$2:$B$4"),
        reference("strRef", "Sheet1!$C$1"),
        reference("numRef", "Sheet1!$C$2:$C$4"),
    );
    assert_eq!(
        text_of(&output, CHART1),
        chart_part(&format!(
            "{}<c:autoTitleDeleted val=\"0\"/><c:plotArea><c:layout/><c:barChart><c:barDir \
             val=\"col\"/><c:grouping val=\"clustered\"/><c:varyColors val=\"0\"/>{series}\
             <c:gapWidth val=\"150\"/><c:axId val=\"10\"/><c:axId val=\"20\"/></c:barChart>{}\
             </c:plotArea><c:legend><c:legendPos val=\"b\"/><c:overlay val=\"0\"/></c:legend>\
             <c:plotVisOnly val=\"1\"/><c:dispBlanksAs val=\"gap\"/>",
            title("Sales"),
            axes("b", "l", &title("Month"), &title("Units")),
        ))
    );
    assert_eq!(
        text_of(&output, DRAWING),
        drawing_xml(&written_box(&frame(1, "rId1")))
    );
    assert_eq!(
        text_of(&output, DRAWING_RELS),
        rels(&relationship(
            "rId1",
            &format!("{ATTR_NS}/chart"),
            "../charts/chart1.xml"
        ))
    );
    assert_eq!(
        text_of(&output, SHEET),
        sheet_r("<sheetData/><drawing r:id=\"rId1\"/>")
    );
    assert_eq!(
        text_of(&output, SHEET_RELS),
        rels(&relationship(
            "rId1",
            &format!("{ATTR_NS}/drawing"),
            "../drawings/drawing1.xml"
        ))
    );
    assert_eq!(
        text_of(&output, b"[Content_Types].xml"),
        part_text(&parts, b"[Content_Types].xml").replace(
            "</Types>",
            &format!(
                "<Override PartName=\"/xl/charts/chart1.xml\" ContentType=\"{CHART_TYPE}\"/>\
                 <Override PartName=\"/xl/drawings/drawing1.xml\" ContentType=\"{DRAWING_TYPE}\"/>\
                 </Types>"
            )
        )
    );
    assert_eq!(
        charts_of(&output),
        vec![ChartEntry {
            part: CHART1.to_vec(),
            anchor: BOX_READ,
            chart: column_read(),
        }]
    );
}

#[test]
fn a_chart_is_appended_to_a_kept_drawing_after_its_picture() {
    let kept = kept_anchor(0, 0, &pic(4, "rId1"));
    let picture_rel = relationship("rId1", &format!("{ATTR_NS}/image"), "../media/image1.png");
    let parts = with_drawing(BODY, drawing_xml(&kept), Some(&picture_rel), true);
    let output = edited_output(parts.clone(), |p| add(p, box_anchor(), &column_chart()));
    assert_untouched(
        &output,
        &parts,
        &[DRAWING, DRAWING_RELS, b"[Content_Types].xml".as_slice()],
    );
    assert_eq!(
        text_of(&output, DRAWING),
        drawing_xml(&format!("{kept}{}", written_box(&frame(5, "rId2"))))
    );
    assert_eq!(
        text_of(&output, DRAWING_RELS),
        rels(&format!(
            "{picture_rel}{}",
            relationship("rId2", &format!("{ATTR_NS}/chart"), "../charts/chart1.xml")
        ))
    );
    assert_eq!(
        charts_of(&output),
        vec![ChartEntry {
            part: CHART1.to_vec(),
            anchor: BOX_READ,
            chart: column_read(),
        }]
    );
}

#[test]
fn a_kept_drawing_with_no_rels_part_gets_one() {
    let parts = with_drawing(BODY, drawing_xml(""), None, false);
    let output = edited_output(parts, |p| add(p, box_anchor(), &column_chart()));
    assert_eq!(
        text_of(&output, DRAWING_RELS),
        rels(&relationship(
            "rId1",
            &format!("{ATTR_NS}/chart"),
            "../charts/chart1.xml"
        ))
    );
    assert_eq!(charts_of(&output).len(), 1);
}

#[test]
fn a_chart_is_appended_to_a_self_closing_drawing_root() {
    // Decision 3 A of `specs/0037-measurement-and-close.md`: the shared
    // drawing writer reads the root `<xdr:wsDr .../>` as an empty drawing
    // and appends the frame inside it, the bytes after the root kept.
    let drawing = format!("{DECLARATION}\n<xdr:wsDr xmlns:xdr=\"{XDR_NS}\" xmlns:a=\"{A_NS}\"/>\n");
    let parts = with_drawing(BODY, drawing, None, false);
    let output = edited_output(parts.clone(), |p| add(p, box_anchor(), &column_chart()));
    assert_untouched(
        &output,
        &parts,
        &[DRAWING, DRAWING_RELS, b"[Content_Types].xml".as_slice()],
    );
    assert_eq!(
        text_of(&output, DRAWING),
        format!("{}\n", drawing_xml(&written_box(&frame(1, "rId1"))))
    );
    assert_eq!(
        text_of(&output, DRAWING_RELS),
        rels(&relationship(
            "rId1",
            &format!("{ATTR_NS}/chart"),
            "../charts/chart1.xml"
        ))
    );
    assert_eq!(
        charts_of(&output),
        vec![ChartEntry {
            part: CHART1.to_vec(),
            anchor: BOX_READ,
            chart: column_read(),
        }]
    );
}

#[test]
fn a_bar_chart_with_direction_bar_swaps_the_axes() {
    let chart = bare(NewPlot::Bar {
        direction: BarDirection::Bar,
        grouping: Grouping::Clustered,
    });
    let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
    let part = text_of(&output, CHART1);
    assert!(part.contains(&axes("l", "b", "", "")), "{part}");
    let read = &charts_of(&output)[0].chart;
    assert_eq!(read.plots[0].bar_direction, Some(BarDirection::Bar));
    assert_eq!(
        read.axes,
        read_axes(AxisPosition::Left, AxisPosition::Bottom, None, None)
    );
}

#[test]
fn each_bar_grouping_writes_overlap_when_stacked() {
    for (grouping, value, overlap) in [
        (Grouping::Clustered, "clustered", false),
        (Grouping::Stacked, "stacked", true),
        (Grouping::PercentStacked, "percentStacked", true),
    ] {
        let chart = bare(NewPlot::Bar {
            direction: BarDirection::Column,
            grouping,
        });
        let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
        let part = text_of(&output, CHART1);
        assert!(
            part.contains(&format!("<c:grouping val=\"{value}\"/>")),
            "{part}"
        );
        assert_eq!(
            part.contains("<c:gapWidth val=\"150\"/><c:overlap val=\"100\"/><c:axId"),
            overlap,
            "{part}"
        );
        assert_eq!(
            part.contains("<c:overlap"),
            overlap,
            "no overlap but the stacked ones'"
        );
        assert_eq!(
            charts_of(&output)[0].chart.plots[0].grouping,
            Some(grouping)
        );
    }
    let chart = bare(NewPlot::Bar {
        direction: BarDirection::Column,
        grouping: Grouping::Standard,
    });
    assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
}

#[test]
fn a_line_chart_takes_each_grouping_but_clustered() {
    for (grouping, value) in [
        (Grouping::Standard, "standard"),
        (Grouping::Stacked, "stacked"),
        (Grouping::PercentStacked, "percentStacked"),
    ] {
        let chart = bare(NewPlot::Line { grouping });
        let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
        let part = text_of(&output, CHART1);
        assert!(
            part.contains(&format!(
                "<c:lineChart><c:grouping val=\"{value}\"/><c:varyColors val=\"0\"/><c:ser>"
            )),
            "{part}"
        );
        assert!(
            part.contains(
                "</c:ser><c:marker val=\"1\"/><c:axId val=\"10\"/><c:axId val=\"20\"/></c:lineChart>"
            ),
            "{part}"
        );
        let read = &charts_of(&output)[0].chart;
        assert_eq!(read.plots[0].kind, ChartKind::Line);
        assert_eq!(read.plots[0].grouping, Some(grouping));
        assert!(
            !read.plots[0].series[0].marker,
            "the plot's marker, not a series'"
        );
    }
    let chart = bare(NewPlot::Line {
        grouping: Grouping::Clustered,
    });
    assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
}

#[test]
fn no_title_no_legend_and_no_axis_titles_write_none() {
    let chart = bare(NewPlot::Line {
        grouping: Grouping::Standard,
    });
    let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
    let part = text_of(&output, CHART1);
    assert!(
        part.contains("<c:chart><c:autoTitleDeleted val=\"1\"/><c:plotArea>"),
        "{part}"
    );
    assert!(
        part.contains("</c:plotArea><c:plotVisOnly val=\"1\"/>"),
        "{part}"
    );
    assert!(!part.contains("<c:title>"), "{part}");
    assert!(!part.contains("<c:tx>"), "{part}");
    let read = &charts_of(&output)[0].chart;
    assert_eq!(read.title, None);
    assert!(read.auto_title_deleted);
    assert_eq!(read.legend, None);
    assert_eq!(
        read.axes,
        read_axes(AxisPosition::Bottom, AxisPosition::Left, None, None)
    );
    assert_eq!(
        read.plots[0].series,
        vec![series_ref(None, None, "Sheet1!$B$2:$B$4", 0)]
    );
}

#[test]
fn each_anchor_form_with_its_size() {
    let sized = NewChart {
        size: Some((480, 288)),
        ..column_chart()
    };
    // One-cell: the extent from the size.
    let one = ImageAnchor::OneCell {
        from: at("C4"),
        offset: (2, 3),
    };
    let output = edited_output(smallest(), |p| add(p, one, &sized));
    assert_eq!(
        text_of(&output, DRAWING),
        drawing_xml(&format!(
            "<xdr:oneCellAnchor><xdr:from><xdr:col>2</xdr:col><xdr:colOff>19050</xdr:colOff>\
             <xdr:row>3</xdr:row><xdr:rowOff>28575</xdr:rowOff></xdr:from><xdr:ext \
             cx=\"4572000\" cy=\"2743200\"/>{}<xdr:clientData/></xdr:oneCellAnchor>",
            frame(1, "rId1")
        ))
    );
    assert_eq!(
        charts_of(&output)[0].anchor,
        ChartAnchor {
            kind: Anchor::OneCell,
            from: (2, 3),
            to: None,
            edit_as: None,
        }
    );
    // Absolute: the position and the extent.
    let absolute = ImageAnchor::Absolute { position: (10, 20) };
    let output = edited_output(smallest(), |p| add(p, absolute, &sized));
    assert_eq!(
        text_of(&output, DRAWING),
        drawing_xml(&format!(
            "<xdr:absoluteAnchor><xdr:pos x=\"95250\" y=\"190500\"/><xdr:ext cx=\"4572000\" \
             cy=\"2743200\"/>{}<xdr:clientData/></xdr:absoluteAnchor>",
            frame(1, "rId1")
        ))
    );
    assert_eq!(charts_of(&output)[0].anchor.kind, Anchor::Absolute);
    // Two-cell: the cells set the box, a size refused; the others need one.
    assert_package(call(box_anchor(), &sized).expect_err("refused"), SIZE);
    assert_package(call(one, &column_chart()).expect_err("refused"), SIZE);
    assert_package(call(absolute, &column_chart()).expect_err("refused"), SIZE);
}

#[test]
fn the_size_and_anchor_rules_refuse_and_record_nothing() {
    let sized = |size| NewChart {
        size: Some(size),
        ..column_chart()
    };
    let one = ImageAnchor::OneCell {
        from: at("A1"),
        offset: (0, 0),
    };
    // A width or height of 0, or one past `i32::MAX` EMU.
    let most = i32::MAX.unsigned_abs() / 9525;
    for size in [(0, 10), (10, 0), (most + 1, 10), (10, most + 1)] {
        assert_package(call(one, &sized(size)).expect_err("refused"), ANCHOR);
    }
    assert!(call(one, &sized((most, most))).is_ok(), "the largest size");
    // An offset or position past `i32::MAX` EMU, and a `to` before `from`.
    let far = ImageAnchor::OneCell {
        from: at("A1"),
        offset: (most + 1, 0),
    };
    assert_package(call(far, &sized((1, 1))).expect_err("refused"), ANCHOR);
    let far = ImageAnchor::Absolute {
        position: (0, most + 1),
    };
    assert_package(call(far, &sized((1, 1))).expect_err("refused"), ANCHOR);
    for (from, to) in [("C3", "B5"), ("C3", "E2")] {
        let backward = ImageAnchor::TwoCell {
            from: at(from),
            to: at(to),
            from_offset: (0, 0),
            to_offset: (0, 0),
            edit_as: EditAs::TwoCell,
        };
        assert_package(
            call(backward, &column_chart()).expect_err("refused"),
            ANCHOR,
        );
    }
    let late = ImageAnchor::TwoCell {
        from: at("A1"),
        to: at("B2"),
        from_offset: (0, 0),
        to_offset: (most + 1, 0),
        edit_as: EditAs::Absolute,
    };
    assert_package(call(late, &column_chart()).expect_err("refused"), ANCHOR);
    // A cell off the grid, the grid's message.
    let off = CellRef {
        row: 1_048_577,
        col: 1,
    };
    let anchor = ImageAnchor::OneCell {
        from: off,
        offset: (0, 0),
    };
    assert_package(call(anchor, &sized((1, 1))).expect_err("refused"), GRID);
    let anchor = ImageAnchor::TwoCell {
        from: at("A1"),
        to: CellRef {
            row: 1,
            col: 16_385,
        },
        from_offset: (0, 0),
        to_offset: (0, 0),
        edit_as: EditAs::TwoCell,
    };
    assert_package(call(anchor, &column_chart()).expect_err("refused"), GRID);
}

#[test]
fn the_text_rules_refuse_each_field_and_record_nothing() {
    let long = "x".repeat(32_768);
    let longest = "x".repeat(32_767);
    let long_reference = "x".repeat(8_193);
    let bad = [
        "",
        "a\tb",
        "a\u{1f}",
        "a\u{fffe}",
        "a\u{ffff}",
        long.as_str(),
    ];
    for text in bad {
        let titled = NewChart {
            title: Some(text),
            ..column_chart()
        };
        assert_package(call(box_anchor(), &titled).expect_err("title"), TEXT);
        let titled = NewChart {
            category_axis_title: Some(text),
            ..column_chart()
        };
        assert_package(
            call(box_anchor(), &titled).expect_err("category axis"),
            TEXT,
        );
        let titled = NewChart {
            value_axis_title: Some(text),
            ..column_chart()
        };
        assert_package(call(box_anchor(), &titled).expect_err("value axis"), TEXT);
        let series = [NewSeries {
            name: Some(NewSeriesName::Text(text)),
            ..TWO_SERIES[0]
        }];
        let named = NewChart {
            series: &series,
            ..column_chart()
        };
        assert_package(call(box_anchor(), &named).expect_err("text name"), TEXT);
    }
    for reference in ["", "a\nb", "a\u{ffff}", long_reference.as_str()] {
        for series in [
            NewSeries {
                name: Some(NewSeriesName::Reference(reference)),
                ..TWO_SERIES[0]
            },
            NewSeries {
                categories: Some(reference),
                ..TWO_SERIES[0]
            },
            NewSeries {
                values: reference,
                ..TWO_SERIES[0]
            },
        ] {
            let series = [series];
            let chart = NewChart {
                series: &series,
                ..column_chart()
            };
            assert_package(call(box_anchor(), &chart).expect_err("reference"), TEXT);
        }
    }
    // The limits themselves are taken.
    let titled = NewChart {
        title: Some(&longest),
        ..column_chart()
    };
    assert!(call(box_anchor(), &titled).is_ok(), "32767 bytes");
    let reference = "x".repeat(8_192);
    let series = [NewSeries {
        values: &reference,
        ..TWO_SERIES[0]
    }];
    let chart = NewChart {
        series: &series,
        ..column_chart()
    };
    assert!(call(box_anchor(), &chart).is_ok(), "8192 bytes");
}

#[test]
fn a_chart_with_no_series_is_refused() {
    let chart = NewChart {
        series: &[],
        ..column_chart()
    };
    assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
}

#[test]
fn the_first_rule_broken_in_the_order_gives_the_message() {
    // P1 ruling C: the grid, the anchor's range, the size's applicability,
    // the size's range, the kind's options, then the texts in field order.
    let empty_title = NewChart {
        title: Some(""),
        series: &[],
        ..column_chart()
    };
    // No series and an empty title: the kind's message.
    assert_package(
        call(box_anchor(), &empty_title).expect_err("refused"),
        OPTION,
    );
    // A size on a two-cell anchor and no series: the size's message.
    let sized = NewChart {
        size: Some((10, 10)),
        ..empty_title
    };
    assert_package(call(box_anchor(), &sized).expect_err("refused"), SIZE);
    // A backward anchor and a size: the anchor's message.
    let backward = ImageAnchor::TwoCell {
        from: at("C3"),
        to: at("B2"),
        from_offset: (0, 0),
        to_offset: (0, 0),
        edit_as: EditAs::TwoCell,
    };
    assert_package(call(backward, &sized).expect_err("refused"), ANCHOR);
    // A cell off the grid and a backward anchor: the grid's message.
    let off = ImageAnchor::TwoCell {
        from: at("C3"),
        to: CellRef {
            row: 2,
            col: 16_385,
        },
        from_offset: (0, 0),
        to_offset: (0, 0),
        edit_as: EditAs::TwoCell,
    };
    assert_package(call(off, &sized).expect_err("refused"), GRID);
    // A size of 0 and a bad title: the anchor's message; then the title
    // before a bad axis title and a bad value.
    let one = ImageAnchor::OneCell {
        from: at("A1"),
        offset: (0, 0),
    };
    let zero = NewChart {
        size: Some((0, 1)),
        title: Some(""),
        ..column_chart()
    };
    assert_package(call(one, &zero).expect_err("refused"), ANCHOR);
    let series = [NewSeries {
        values: "",
        ..TWO_SERIES[0]
    }];
    let texts = NewChart {
        title: Some("a\u{1}"),
        value_axis_title: Some(""),
        series: &series,
        ..column_chart()
    };
    assert_package(call(box_anchor(), &texts).expect_err("refused"), TEXT);
}

#[test]
fn texts_are_escaped_and_references_written_as_given() {
    // P1 ruling F: a title escaped as a comment's text; a reference's `&`,
    // `<` and `>` as entities, read back unchanged.
    let series = [NewSeries {
        name: Some(NewSeriesName::Text("Q1 & \"Q2\" <est>")),
        categories: Some("'A&B''s'!$A$2:$A$4"),
        values: "'A&B''s'!$B$2:$B$4",
        sizes: None,
        color: None,
        labels: None,
        marker: None,
        line: None,
        points: &[],
        trendline: None,
        error_bars: &[],
    }];
    let chart = NewChart {
        title: Some("Tom's <plan> & more"),
        series: &series,
        ..column_chart()
    };
    let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
    let part = text_of(&output, CHART1);
    assert!(
        part.contains("<a:t>Tom&apos;s &lt;plan&gt; &amp; more</a:t>"),
        "{part}"
    );
    assert!(
        part.contains("<c:v>Q1 &amp; &quot;Q2&quot; &lt;est&gt;</c:v>"),
        "{part}"
    );
    assert!(part.contains("<c:f>'A&amp;B''s'!$A$2:$A$4</c:f>"), "{part}");
    let read = &charts_of(&output)[0].chart;
    assert_eq!(read.title.as_deref(), Some("Tom's <plan> & more"));
    assert_eq!(
        read.plots[0].series[0],
        series_ref(
            Some(SeriesName::Text("Q1 & \"Q2\" <est>".to_owned())),
            Some("'A&B''s'!$A$2:$A$4"),
            "'A&B''s'!$B$2:$B$4",
            0,
        )
    );
}

#[test]
fn the_chart_part_takes_the_first_free_name() {
    // The media rule: a name in the archive, case folded, is taken, and so
    // is one this write adds before.
    let mut parts = smallest();
    parts.push((b"xl/charts/Chart1.xml".to_vec(), b"<x/>".to_vec()));
    let output = edited_output(parts, |p| {
        add(p, box_anchor(), &column_chart());
        add(p, box_anchor(), &column_chart());
    });
    let read = charts_of(&output);
    assert_eq!(
        read.iter()
            .map(|entry| entry.part.clone())
            .collect::<Vec<_>>(),
        vec![
            b"xl/charts/chart2.xml".to_vec(),
            b"xl/charts/chart3.xml".to_vec()
        ]
    );
    assert_eq!(
        text_of(&output, DRAWING),
        drawing_xml(&format!(
            "{}{}",
            written_box(&frame(1, "rId1")),
            written_box(&frame(2, "rId2"))
        ))
    );
}

#[test]
fn the_drawing_refusals_are_the_image_writer_s() {
    let add_one = |p: &mut Package<Cursor<Vec<u8>>>| add(p, box_anchor(), &column_chart());
    for body in [
        "<sheetData/><drawing r:id=\"rId1\"/><drawing r:id=\"rId1\"/>",
        "<sheetData/><legacyDrawing r:id=\"rId2\"/><drawing r:id=\"rId1\"/>",
    ] {
        let parts = with_drawing(body, drawing_xml(""), None, false);
        assert_package(
            refused_with(parts, add_one),
            "a worksheet has two drawing elements",
        );
    }
    let parts = with_drawing("<sheetData/><drawing/>", drawing_xml(""), None, false);
    assert_package(refused_with(parts, add_one), DRAWING_UNREADABLE);
    let parts = with_drawing(
        "<sheetData/><drawing r:id=\"rId7\"/>",
        drawing_xml(""),
        None,
        false,
    );
    assert_package(refused_with(parts, add_one), DRAWING_UNREADABLE);
    let mut parts = with_drawing(BODY, drawing_xml(""), None, false);
    drop_part(&mut parts, DRAWING);
    assert_package(refused_with(parts, add_one), DRAWING_UNREADABLE);
    let kept = kept_anchor(0, 0, &pic(1, "rId1")).replace("id=\"1\"", "id=\"x\"");
    let parts = with_drawing(BODY, drawing_xml(&kept), None, false);
    assert_package(refused_with(parts, add_one), DRAWING_UNREADABLE);
    let user_shapes = format!("{DECLARATION}\n<c:userShapes xmlns:c=\"{C_NS}\"/>");
    let parts = with_drawing(BODY, user_shapes, None, false);
    assert_package(refused_with(parts, add_one), DRAWING_UNREADABLE);
}

#[test]
fn a_chart_on_a_removed_replaced_or_added_part_is_refused() {
    let add_one = |p: &mut Package<Cursor<Vec<u8>>>| add(p, box_anchor(), &column_chart());
    let (_, mut package) = opened(smallest());
    add_one(&mut package);
    package.remove_part(SHEET);
    assert_package(refused(package), "an edit to a removed part");
    let (_, mut package) = opened(smallest());
    add_one(&mut package);
    package.replace_part(SHEET, replacement_from_reader(Cursor::new(b"x".to_vec())));
    assert_package(
        refused(package),
        "a cell edit to a part this write replaces",
    );
    let (_, mut package) = opened(smallest());
    let part = package.add_sheet("Two").expect("add_sheet");
    package
        .add_chart(&part, box_anchor(), &column_chart())
        .expect("recorded");
    assert_package(refused(package), "no such part");
    let parts = with_drawing(BODY, drawing_xml(""), None, false);
    assert_package(
        refused_with(parts, |p| {
            p.remove_part(DRAWING);
            add_one(p);
        }),
        "an edit to a removed part",
    );
}

#[test]
fn a_strict_sheet_s_created_drawing_takes_the_strict_relationship_types() {
    // P1 ruling E: the created drawing is in the transitional form, so the
    // chart is; the relationships take the sheet's strict types.
    let mut parts = smallest();
    let workbook_rels = part_text(&parts, b"xl/_rels/workbook.xml.rels").replace(
        &format!("{ATTR_NS}/worksheet"),
        &format!("{R_STRICT}/worksheet"),
    );
    set(&mut parts, b"xl/_rels/workbook.xml.rels", workbook_rels);
    let output = edited_output(parts, |p| add(p, box_anchor(), &column_chart()));
    assert_eq!(
        text_of(&output, DRAWING_RELS),
        rels(&relationship(
            "rId1",
            &format!("{R_STRICT}/chart"),
            "../charts/chart1.xml"
        ))
    );
    assert!(text_of(&output, CHART1).contains(&format!("xmlns:c=\"{C_NS}\"")));
    assert_eq!(charts_of(&output)[0].chart, column_read());
}

#[test]
fn a_strict_drawing_takes_the_strict_chart_namespaces() {
    // P1 ruling E: a kept drawing in the strict form; the frame's
    // `graphicData` and `c:chart` and the chart part take the strict
    // namespaces, read back.
    let strict = format!(
        "{DECLARATION}\n<xdr:wsDr xmlns:xdr=\"{XDR_STRICT}\" xmlns:a=\"{A_STRICT}\" \
         xmlns:r=\"{R_STRICT}\"></xdr:wsDr>"
    );
    let mut parts = with_drawing(BODY, strict, None, false);
    let workbook_rels = part_text(&parts, b"xl/_rels/workbook.xml.rels").replace(
        &format!("{ATTR_NS}/worksheet"),
        &format!("{R_STRICT}/worksheet"),
    );
    set(&mut parts, b"xl/_rels/workbook.xml.rels", workbook_rels);
    let sheet_rels = rels(&relationship(
        "rId1",
        &format!("{R_STRICT}/drawing"),
        "../drawings/drawing1.xml",
    ));
    set(&mut parts, SHEET_RELS, sheet_rels);
    let output = edited_output(parts, |p| add(p, box_anchor(), &column_chart()));
    let drawing = text_of(&output, DRAWING);
    assert!(
        drawing.contains(&format!(
            "<a:graphicData uri=\"{C_STRICT}\"><c:chart xmlns:c=\"{C_STRICT}\" r:id=\"rId1\"/>"
        )),
        "{drawing}"
    );
    let part = text_of(&output, CHART1);
    assert!(
        part.starts_with(&format!(
            "{DECLARATION}\n<c:chartSpace xmlns:c=\"{C_STRICT}\" xmlns:a=\"{A_STRICT}\">"
        )),
        "{part}"
    );
    assert_eq!(charts_of(&output)[0].chart, column_read());
}

#[cfg(feature = "images")]
#[test]
fn an_image_and_a_chart_on_one_sheet_take_the_drawing_in_call_order() {
    use crate::package::Image;
    // Two charts told apart by their titles around the image, so the order
    // reversed would read differently.
    let first_chart = NewChart {
        title: Some("First"),
        ..column_chart()
    };
    let second_chart = NewChart {
        title: Some("Second"),
        ..column_chart()
    };
    let output = edited_output(smallest(), |p| {
        add(p, box_anchor(), &first_chart);
        p.add_image(
            SHEET,
            at("H2"),
            Image {
                bytes: &PNG,
                size: None,
            },
        )
        .expect("recorded");
        add(p, box_anchor(), &second_chart);
    });
    let drawing = text_of(&output, DRAWING);
    let first = drawing.find("name=\"Chart 1\"").expect("the first chart");
    let image = drawing.find("name=\"Image 2\"").expect("the image");
    let second = drawing.find("name=\"Chart 3\"").expect("the second chart");
    assert!(first < image && image < second, "{drawing}");
    assert_eq!(
        text_of(&output, DRAWING_RELS),
        rels(&format!(
            "{}{}{}",
            relationship("rId1", &format!("{ATTR_NS}/chart"), "../charts/chart1.xml"),
            relationship("rId2", &format!("{ATTR_NS}/image"), "../media/image1.png"),
            relationship("rId3", &format!("{ATTR_NS}/chart"), "../charts/chart2.xml"),
        ))
    );
    let mut package = Package::open(Cursor::new(output.clone())).expect("open");
    assert_eq!(package.images(SHEET).expect("images").len(), 1);
    assert!(text_of(&output, CHART1).contains("<a:t>First</a:t>"));
    assert_eq!(
        charts_of(&output)
            .iter()
            .map(|entry| entry.chart.title.clone())
            .collect::<Vec<_>>(),
        vec![Some("First".to_owned()), Some("Second".to_owned())]
    );
}

#[cfg(feature = "images")]
#[test]
fn an_image_removal_with_a_chart_on_one_part_is_refused() {
    // P1 ruling B: as an image addition with a removal, the caller writes
    // twice.
    let kept = kept_anchor(0, 0, &pic(1, "rId1"));
    let picture_rel = relationship("rId1", &format!("{ATTR_NS}/image"), "../media/image1.png");
    let parts = with_drawing(BODY, drawing_xml(&kept), Some(&picture_rel), true);
    assert_package(
        refused_with(parts, |p| {
            p.remove_image(SHEET, 0).expect("recorded");
            add(p, box_anchor(), &column_chart());
        }),
        "no image at that position",
    );
}

/// A one-series chart of `plot` with categories, a title, the axis titles
/// given and a legend at the right.
fn titled<'a>(
    plot: NewPlot,
    series: &'a [NewSeries<'a>],
    axis_titles: (Option<&'a str>, Option<&'a str>),
) -> NewChart<'a> {
    NewChart {
        plot,
        title: Some("Share"),
        series,
        category_axis_title: axis_titles.0,
        value_axis_title: axis_titles.1,
        category_axis_format: None,
        value_axis_format: None,
        legend: Some(LegendPosition::Right),
        size: None,
        combined: None,
    }
}

/// One series with a reference name, categories and values.
const ONE_NAMED: [NewSeries<'static>; 1] = [NewSeries {
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
}];

/// `ONE_NAMED` as `charts` reads it.
fn one_named_read() -> Series {
    series_ref(
        Some(SeriesName::Reference("Sheet1!$B$1".to_owned())),
        Some("Sheet1!$A$2:$A$4"),
        "Sheet1!$B$2:$B$4",
        0,
    )
}

#[test]
fn a_pie_with_one_series_writes_no_axes_and_varies_its_colours() {
    let chart = titled(NewPlot::Pie, &ONE_NAMED, (None, None));
    let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
    assert_eq!(
        text_of(&output, CHART1),
        chart_part(&format!(
            "{}<c:autoTitleDeleted val=\"0\"/><c:plotArea><c:layout/><c:pieChart><c:varyColors \
             val=\"1\"/><c:ser><c:idx val=\"0\"/><c:order val=\"0\"/><c:tx>{}</c:tx><c:cat>{}\
             </c:cat><c:val>{}</c:val></c:ser><c:firstSliceAng val=\"0\"/></c:pieChart>\
             </c:plotArea><c:legend><c:legendPos val=\"r\"/><c:overlay val=\"0\"/></c:legend>\
             <c:plotVisOnly val=\"1\"/><c:dispBlanksAs val=\"gap\"/>",
            title("Share"),
            reference("strRef", "Sheet1!$B$1"),
            reference("strRef", "Sheet1!$A$2:$A$4"),
            reference("numRef", "Sheet1!$B$2:$B$4"),
        ))
    );
    assert_eq!(
        charts_of(&output),
        vec![ChartEntry {
            part: CHART1.to_vec(),
            anchor: BOX_READ,
            chart: Chart {
                title: Some("Share".to_owned()),
                auto_title_deleted: false,
                plots: vec![Plot {
                    kind: ChartKind::Pie,
                    bar_direction: None,
                    grouping: None,
                    scatter_style: None,
                    vary_colors: true,
                    series: vec![one_named_read()],
                    axis_ids: Vec::new(),
                }],
                axes: Vec::new(),
                legend: Some(LegendPosition::Right),
                pivot_source: None,
                style: None,
            },
        }]
    );
}

#[test]
fn a_pie_with_two_series_or_an_axis_title_is_refused() {
    let two = titled(NewPlot::Pie, &TWO_SERIES, (None, None));
    assert_package(call(box_anchor(), &two).expect_err("refused"), OPTION);
    for axis_titles in [(Some("Month"), None), (None, Some("Units"))] {
        let chart = titled(NewPlot::Pie, &ONE_NAMED, axis_titles);
        assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
    }
    // Applicability comes before the texts (P1 ruling C).
    let both = NewChart {
        title: Some(""),
        ..two
    };
    assert_package(call(box_anchor(), &both).expect_err("refused"), OPTION);
}

/// A scatter plot's two value axes as written: the x axis at `b` with no
/// gridlines, the y axis at `l`, both `midCat`, with their titles' forms.
fn scatter_axes(x_title: &str, y_title: &str) -> String {
    format!(
        "<c:valAx><c:axId val=\"10\"/><c:scaling><c:orientation val=\"minMax\"/></c:scaling>\
         <c:delete val=\"0\"/><c:axPos val=\"b\"/>{x_title}<c:numFmt formatCode=\"General\" \
         sourceLinked=\"1\"/><c:majorTickMark val=\"out\"/><c:minorTickMark val=\"none\"/>\
         <c:tickLblPos val=\"nextTo\"/><c:crossAx val=\"20\"/><c:crosses val=\"autoZero\"/>\
         <c:crossBetween val=\"midCat\"/></c:valAx><c:valAx><c:axId val=\"20\"/><c:scaling>\
         <c:orientation val=\"minMax\"/></c:scaling><c:delete val=\"0\"/><c:axPos val=\"l\"/>\
         <c:majorGridlines/>{y_title}<c:numFmt formatCode=\"General\" sourceLinked=\"1\"/>\
         <c:majorTickMark val=\"out\"/><c:minorTickMark val=\"none\"/><c:tickLblPos \
         val=\"nextTo\"/><c:crossAx val=\"10\"/><c:crosses val=\"autoZero\"/><c:crossBetween \
         val=\"midCat\"/></c:valAx>"
    )
}

/// A scatter plot's two value axes as read, with their titles.
fn scatter_read_axes(x_title: Option<&str>, y_title: Option<&str>) -> Vec<ChartAxis> {
    let mut axes = read_axes(AxisPosition::Bottom, AxisPosition::Left, x_title, y_title);
    axes[0].kind = AxisKind::Value;
    axes
}

#[test]
fn a_scatter_chart_takes_each_style_with_x_and_y_values() {
    for (style, value) in [
        (ScatterStyle::None, "none"),
        (ScatterStyle::Line, "line"),
        (ScatterStyle::LineMarker, "lineMarker"),
        (ScatterStyle::Marker, "marker"),
        (ScatterStyle::Smooth, "smooth"),
        (ScatterStyle::SmoothMarker, "smoothMarker"),
    ] {
        let chart = titled(
            NewPlot::Scatter { style },
            &ONE_NAMED,
            (Some("Across"), Some("Up")),
        );
        let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
        assert_eq!(
            text_of(&output, CHART1),
            chart_part(&format!(
                "{}<c:autoTitleDeleted val=\"0\"/><c:plotArea><c:layout/><c:scatterChart>\
                 <c:scatterStyle val=\"{value}\"/><c:varyColors val=\"0\"/><c:ser><c:idx \
                 val=\"0\"/><c:order val=\"0\"/><c:tx>{}</c:tx><c:xVal>{}</c:xVal><c:yVal>{}\
                 </c:yVal></c:ser><c:axId val=\"10\"/><c:axId val=\"20\"/></c:scatterChart>{}\
                 </c:plotArea><c:legend><c:legendPos val=\"r\"/><c:overlay val=\"0\"/>\
                 </c:legend><c:plotVisOnly val=\"1\"/><c:dispBlanksAs val=\"gap\"/>",
                title("Share"),
                reference("strRef", "Sheet1!$B$1"),
                reference("numRef", "Sheet1!$A$2:$A$4"),
                reference("numRef", "Sheet1!$B$2:$B$4"),
                scatter_axes(&title("Across"), &title("Up")),
            ))
        );
        let read = &charts_of(&output)[0].chart;
        assert_eq!(
            read.plots,
            vec![Plot {
                kind: ChartKind::Scatter,
                bar_direction: None,
                grouping: None,
                scatter_style: Some(style),
                vary_colors: false,
                series: vec![one_named_read()],
                axis_ids: vec![10, 20],
            }]
        );
        assert_eq!(read.axes, scatter_read_axes(Some("Across"), Some("Up")));
    }
}

#[test]
fn a_scatter_series_with_no_categories_writes_no_x_values() {
    let chart = bare(NewPlot::Scatter {
        style: ScatterStyle::Marker,
    });
    let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
    let part = text_of(&output, CHART1);
    assert!(
        part.contains(&format!(
            "<c:order val=\"0\"/><c:yVal>{}</c:yVal></c:ser>",
            reference("numRef", "Sheet1!$B$2:$B$4")
        )),
        "{part}"
    );
    assert!(!part.contains("<c:xVal>"), "{part}");
    assert!(part.contains(&scatter_axes("", "")), "{part}");
    let read = &charts_of(&output)[0].chart;
    assert_eq!(
        read.plots[0].series,
        vec![series_ref(None, None, "Sheet1!$B$2:$B$4", 0)]
    );
    assert_eq!(read.axes, scatter_read_axes(None, None));
}

#[test]
fn an_area_chart_takes_each_grouping_but_clustered() {
    for (grouping, value) in [
        (Grouping::Standard, "standard"),
        (Grouping::Stacked, "stacked"),
        (Grouping::PercentStacked, "percentStacked"),
    ] {
        let chart = titled(
            NewPlot::Area { grouping },
            &ONE_NAMED,
            (Some("Month"), Some("Units")),
        );
        let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
        let part = text_of(&output, CHART1);
        assert!(
            part.contains(&format!(
                "<c:plotArea><c:layout/><c:areaChart><c:grouping val=\"{value}\"/><c:varyColors \
                 val=\"0\"/><c:ser>"
            )),
            "{part}"
        );
        assert!(
            part.contains(&format!(
                "</c:ser><c:axId val=\"10\"/><c:axId val=\"20\"/></c:areaChart>{}</c:plotArea>",
                axes("b", "l", &title("Month"), &title("Units"))
            )),
            "{part}"
        );
        let read = &charts_of(&output)[0].chart;
        assert_eq!(
            read.plots,
            vec![Plot {
                kind: ChartKind::Area,
                bar_direction: None,
                grouping: Some(grouping),
                scatter_style: None,
                vary_colors: false,
                series: vec![one_named_read()],
                axis_ids: vec![10, 20],
            }]
        );
        assert_eq!(
            read.axes,
            read_axes(
                AxisPosition::Bottom,
                AxisPosition::Left,
                Some("Month"),
                Some("Units")
            )
        );
    }
    let chart = bare(NewPlot::Area {
        grouping: Grouping::Clustered,
    });
    assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
}

/// Two named series with categories, for the doughnut and the radar.
const TWO_NAMED: [NewSeries<'static>; 2] = [
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

/// `TWO_NAMED` as `charts` reads it.
fn two_named_read() -> Vec<Series> {
    vec![
        one_named_read(),
        series_ref(
            Some(SeriesName::Reference("Sheet1!$C$1".to_owned())),
            Some("Sheet1!$A$2:$A$4"),
            "Sheet1!$C$2:$C$4",
            1,
        ),
    ]
}

/// The series forms of `TWO_NAMED` as written.
fn two_named_written() -> String {
    format!(
        "<c:ser><c:idx val=\"0\"/><c:order val=\"0\"/><c:tx>{}</c:tx><c:cat>{}</c:cat><c:val>{}\
         </c:val></c:ser><c:ser><c:idx val=\"1\"/><c:order val=\"1\"/><c:tx>{}</c:tx><c:cat>{}\
         </c:cat><c:val>{}</c:val></c:ser>",
        reference("strRef", "Sheet1!$B$1"),
        reference("strRef", "Sheet1!$A$2:$A$4"),
        reference("numRef", "Sheet1!$B$2:$B$4"),
        reference("strRef", "Sheet1!$C$1"),
        reference("strRef", "Sheet1!$A$2:$A$4"),
        reference("numRef", "Sheet1!$C$2:$C$4"),
    )
}

/// A plot as read with `kind`, its series and its axis ids, and nothing
/// else set but `vary_colors`.
fn plot_read(kind: ChartKind, vary_colors: bool, series: Vec<Series>, axis_ids: Vec<i64>) -> Plot {
    Plot {
        kind,
        bar_direction: None,
        grouping: None,
        scatter_style: None,
        vary_colors,
        series,
        axis_ids,
    }
}

#[test]
fn a_doughnut_with_two_series_writes_its_hole_and_no_axes() {
    let chart = titled(
        NewPlot::Doughnut { hole_size: 50 },
        &TWO_NAMED,
        (None, None),
    );
    let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
    assert_eq!(
        text_of(&output, CHART1),
        chart_part(&format!(
            "{}<c:autoTitleDeleted val=\"0\"/><c:plotArea><c:layout/><c:doughnutChart>\
             <c:varyColors val=\"1\"/>{}<c:firstSliceAng val=\"0\"/><c:holeSize val=\"50\"/>\
             </c:doughnutChart></c:plotArea><c:legend><c:legendPos val=\"r\"/><c:overlay \
             val=\"0\"/></c:legend><c:plotVisOnly val=\"1\"/><c:dispBlanksAs val=\"gap\"/>",
            title("Share"),
            two_named_written(),
        ))
    );
    let read = &charts_of(&output)[0].chart;
    assert_eq!(
        read.plots,
        vec![plot_read(
            ChartKind::Doughnut,
            true,
            two_named_read(),
            Vec::new()
        )]
    );
    assert!(read.axes.is_empty());
    for hole_size in [1, 90] {
        let edge = titled(NewPlot::Doughnut { hole_size }, &ONE_NAMED, (None, None));
        call(box_anchor(), &edge).expect("taken");
    }
}

#[test]
fn a_doughnut_s_hole_outside_1_to_90_or_an_axis_title_is_refused() {
    for hole_size in [0, 91, u8::MAX] {
        let chart = titled(NewPlot::Doughnut { hole_size }, &ONE_NAMED, (None, None));
        assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
    }
    for axis_titles in [(Some("Month"), None), (None, Some("Units"))] {
        let chart = titled(NewPlot::Doughnut { hole_size: 50 }, &ONE_NAMED, axis_titles);
        assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
    }
}

#[test]
fn a_radar_chart_takes_each_style_with_a_line_s_axes() {
    for (style, value) in [
        (RadarStyle::Standard, "standard"),
        (RadarStyle::Marker, "marker"),
        (RadarStyle::Filled, "filled"),
    ] {
        let chart = titled(
            NewPlot::Radar { style },
            &TWO_NAMED,
            (Some("Month"), Some("Units")),
        );
        let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
        let part = text_of(&output, CHART1);
        assert!(
            part.contains(&format!(
                "<c:plotArea><c:layout/><c:radarChart><c:radarStyle val=\"{value}\"/><c:varyColors \
                 val=\"0\"/>{}<c:axId val=\"10\"/><c:axId val=\"20\"/></c:radarChart>{}\
                 </c:plotArea>",
                two_named_written(),
                axes("b", "l", &title("Month"), &title("Units"))
            )),
            "{part}"
        );
        let read = &charts_of(&output)[0].chart;
        assert_eq!(
            read.plots,
            vec![plot_read(
                ChartKind::Radar,
                false,
                two_named_read(),
                vec![10, 20]
            )]
        );
        assert_eq!(
            read.axes,
            read_axes(
                AxisPosition::Bottom,
                AxisPosition::Left,
                Some("Month"),
                Some("Units")
            )
        );
    }
}

/// Two bubble series, each with x values, y values and sizes.
const BUBBLES: [NewSeries<'static>; 2] = [
    NewSeries {
        name: Some(NewSeriesName::Text("Small")),
        categories: Some("Sheet1!$A$2:$A$4"),
        values: "Sheet1!$B$2:$B$4",
        sizes: Some("Sheet1!$C$2:$C$4"),
        color: None,
        labels: None,
        marker: None,
        line: None,
        points: &[],
        trendline: None,
        error_bars: &[],
    },
    NewSeries {
        name: None,
        categories: None,
        values: "Sheet1!$D$2:$D$4",
        sizes: Some("Sheet1!$E$2:$E$4"),
        color: None,
        labels: None,
        marker: None,
        line: None,
        points: &[],
        trendline: None,
        error_bars: &[],
    },
];

#[test]
fn a_bubble_chart_writes_each_series_sizes_with_a_scatter_s_axes() {
    let chart = titled(NewPlot::Bubble, &BUBBLES, (Some("Across"), Some("Up")));
    let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
    let part = text_of(&output, CHART1);
    assert!(
        part.contains(&format!(
            "<c:plotArea><c:layout/><c:bubbleChart><c:varyColors val=\"0\"/><c:ser><c:idx \
             val=\"0\"/><c:order val=\"0\"/><c:tx><c:v>Small</c:v></c:tx><c:xVal>{}</c:xVal>\
             <c:yVal>{}</c:yVal><c:bubbleSize>{}</c:bubbleSize></c:ser><c:ser><c:idx val=\"1\"/>\
             <c:order val=\"1\"/><c:yVal>{}</c:yVal><c:bubbleSize>{}</c:bubbleSize></c:ser>\
             <c:bubbleScale val=\"100\"/><c:showNegBubbles val=\"0\"/><c:axId val=\"10\"/><c:axId \
             val=\"20\"/></c:bubbleChart>{}</c:plotArea>",
            reference("numRef", "Sheet1!$A$2:$A$4"),
            reference("numRef", "Sheet1!$B$2:$B$4"),
            reference("numRef", "Sheet1!$C$2:$C$4"),
            reference("numRef", "Sheet1!$D$2:$D$4"),
            reference("numRef", "Sheet1!$E$2:$E$4"),
            scatter_axes(&title("Across"), &title("Up")),
        )),
        "{part}"
    );
    let read = &charts_of(&output)[0].chart;
    let sized = |mut series: Series, sizes: &str| {
        series.sizes = Some(Data::Reference {
            formula: sizes.to_owned(),
            cached: None,
        });
        series
    };
    assert_eq!(
        read.plots,
        vec![plot_read(
            ChartKind::Bubble,
            false,
            vec![
                sized(
                    series_ref(
                        Some(SeriesName::Text("Small".to_owned())),
                        Some("Sheet1!$A$2:$A$4"),
                        "Sheet1!$B$2:$B$4",
                        0,
                    ),
                    "Sheet1!$C$2:$C$4",
                ),
                sized(
                    series_ref(None, None, "Sheet1!$D$2:$D$4", 1),
                    "Sheet1!$E$2:$E$4",
                ),
            ],
            vec![10, 20],
        )]
    );
    assert_eq!(read.axes, scatter_read_axes(Some("Across"), Some("Up")));
}

#[test]
fn sizes_missing_on_a_bubble_series_or_given_on_another_plot_are_refused() {
    let without = [
        BUBBLES[0],
        NewSeries {
            sizes: None,
            ..BUBBLES[1]
        },
    ];
    let chart = titled(NewPlot::Bubble, &without, (None, None));
    assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
    for plot in [
        NewPlot::Bar {
            direction: BarDirection::Column,
            grouping: Grouping::Clustered,
        },
        NewPlot::Scatter {
            style: ScatterStyle::Marker,
        },
        NewPlot::Pie,
        NewPlot::Stock,
    ] {
        let sized = [BUBBLES[0]; 3];
        let chart = titled(
            plot,
            &sized[..if plot == NewPlot::Stock { 3 } else { 1 }],
            (None, None),
        );
        assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
    }
    // Sizes are a reference, checked after the values (the check order).
    let empty = [NewSeries {
        sizes: Some(""),
        ..BUBBLES[0]
    }];
    let chart = titled(NewPlot::Bubble, &empty, (None, None));
    assert_package(call(box_anchor(), &chart).expect_err("refused"), TEXT);
}

/// Four stock series, open, high, low and close, text names, categories.
const STOCK: [NewSeries<'static>; 5] = [
    NewSeries {
        name: Some(NewSeriesName::Text("open")),
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
        name: Some(NewSeriesName::Text("high")),
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
    NewSeries {
        name: Some(NewSeriesName::Text("low")),
        categories: Some("Sheet1!$A$2:$A$4"),
        values: "Sheet1!$D$2:$D$4",
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
        name: Some(NewSeriesName::Text("close")),
        categories: Some("Sheet1!$A$2:$A$4"),
        values: "Sheet1!$E$2:$E$4",
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
        name: Some(NewSeriesName::Text("extra")),
        categories: Some("Sheet1!$A$2:$A$4"),
        values: "Sheet1!$F$2:$F$4",
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

#[test]
fn a_stock_chart_takes_three_or_four_series_with_high_low_lines() {
    for series in [&STOCK[1..4], &STOCK[..4]] {
        let chart = titled(NewPlot::Stock, series, (Some("Day"), Some("Price")));
        let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
        let part = text_of(&output, CHART1);
        let written: String = series
            .iter()
            .zip(0..)
            .map(|(one, k)| {
                let NewSeriesName::Text(name) = one.name.expect("named") else {
                    unreachable!()
                };
                format!(
                    "<c:ser><c:idx val=\"{k}\"/><c:order val=\"{k}\"/><c:tx><c:v>{name}</c:v>\
                     </c:tx><c:marker><c:symbol val=\"none\"/></c:marker><c:cat>{}</c:cat><c:val>{}\
                     </c:val></c:ser>",
                    reference("strRef", "Sheet1!$A$2:$A$4"),
                    reference("numRef", one.values),
                )
            })
            .collect();
        assert!(
            part.contains(&format!(
                "<c:plotArea><c:layout/><c:stockChart>{written}<c:hiLowLines/><c:axId val=\"10\"/>\
                 <c:axId val=\"20\"/></c:stockChart>{}</c:plotArea>",
                axes("b", "l", &title("Day"), &title("Price"))
            )),
            "{part}"
        );
        let read = &charts_of(&output)[0].chart;
        let expected: Vec<Series> = series
            .iter()
            .zip(0..)
            .map(|(one, k)| {
                let NewSeriesName::Text(name) = one.name.expect("named") else {
                    unreachable!()
                };
                Series {
                    marker: true,
                    ..series_ref(
                        Some(SeriesName::Text(name.to_owned())),
                        Some("Sheet1!$A$2:$A$4"),
                        one.values,
                        k,
                    )
                }
            })
            .collect();
        assert_eq!(
            read.plots,
            vec![plot_read(ChartKind::Stock, false, expected, vec![10, 20])]
        );
        assert_eq!(
            read.axes,
            read_axes(
                AxisPosition::Bottom,
                AxisPosition::Left,
                Some("Day"),
                Some("Price")
            )
        );
    }
}

#[test]
fn a_stock_chart_with_two_or_five_series_is_refused() {
    for series in [&STOCK[1..3], &STOCK[..]] {
        let chart = titled(NewPlot::Stock, series, (None, None));
        assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
    }
}

/// The `view3D` a bar, line or area 3D plot writes.
const VIEW_3D: &str = "<c:view3D><c:rotX val=\"15\"/><c:rotY val=\"20\"/><c:rAngAx val=\"1\"/>\
                       </c:view3D>";

/// The series of `ONE_NAMED` as written.
fn one_named_written() -> String {
    format!(
        "<c:ser><c:idx val=\"0\"/><c:order val=\"0\"/><c:tx>{}</c:tx><c:cat>{}</c:cat><c:val>{}\
         </c:val></c:ser>",
        reference("strRef", "Sheet1!$B$1"),
        reference("strRef", "Sheet1!$A$2:$A$4"),
        reference("numRef", "Sheet1!$B$2:$B$4"),
    )
}

/// The series axis as written.
const SERIES_AXIS: &str = "<c:serAx><c:axId val=\"30\"/><c:scaling><c:orientation val=\"minMax\"/>\
                           </c:scaling><c:delete val=\"0\"/><c:axPos val=\"b\"/><c:majorTickMark \
                           val=\"out\"/><c:minorTickMark val=\"none\"/><c:tickLblPos \
                           val=\"nextTo\"/><c:crossAx val=\"20\"/><c:crosses val=\"autoZero\"/>\
                           </c:serAx>";

/// The two axes as read with their titles `Month` and `Units`, the
/// category axis at `cat` and the value axis at `val`, and the series axis
/// as read when `depth`.
fn read_axes_3d(cat: AxisPosition, val: AxisPosition, depth: bool) -> Vec<ChartAxis> {
    let mut axes = read_axes(cat, val, Some("Month"), Some("Units"));
    if depth {
        axes.push(ChartAxis {
            id: 30,
            kind: AxisKind::Series,
            position: AxisPosition::Bottom,
            crosses: 20,
            title: None,
            number_format: None,
            min: None,
            max: None,
            deleted: false,
            major_gridlines: false,
        });
    }
    axes
}

/// A 3D plot as read: `kind`, its direction and grouping, `ONE_NAMED` and
/// the axis ids, the series axis's when `depth`.
fn plot_3d_read(
    kind: ChartKind,
    bar_direction: Option<BarDirection>,
    grouping: Option<Grouping>,
    depth: bool,
) -> Plot {
    Plot {
        bar_direction,
        grouping,
        ..plot_read(
            kind,
            false,
            vec![one_named_read()],
            if depth {
                vec![10, 20, 30]
            } else {
                vec![10, 20]
            },
        )
    }
}

#[test]
fn a_bar_3d_chart_takes_each_grouping_the_standard_one_with_the_series_axis() {
    for (grouping, value) in [
        (Grouping::Clustered, "clustered"),
        (Grouping::Stacked, "stacked"),
        (Grouping::PercentStacked, "percentStacked"),
        (Grouping::Standard, "standard"),
    ] {
        let depth = grouping == Grouping::Standard;
        let chart = titled(
            NewPlot::Bar3D {
                direction: BarDirection::Column,
                grouping,
            },
            &ONE_NAMED,
            (Some("Month"), Some("Units")),
        );
        let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
        let (third, series_axis) = if depth {
            ("<c:axId val=\"30\"/>", SERIES_AXIS)
        } else {
            ("", "")
        };
        assert_eq!(
            text_of(&output, CHART1),
            chart_part(&format!(
                "{}<c:autoTitleDeleted val=\"0\"/>{VIEW_3D}<c:plotArea><c:layout/><c:bar3DChart>\
                 <c:barDir val=\"col\"/><c:grouping val=\"{value}\"/><c:varyColors val=\"0\"/>{}\
                 <c:gapWidth val=\"150\"/><c:shape val=\"box\"/><c:axId val=\"10\"/><c:axId \
                 val=\"20\"/>{third}</c:bar3DChart>{}{series_axis}</c:plotArea><c:legend>\
                 <c:legendPos val=\"r\"/><c:overlay val=\"0\"/></c:legend><c:plotVisOnly \
                 val=\"1\"/><c:dispBlanksAs val=\"gap\"/>",
                title("Share"),
                one_named_written(),
                axes("b", "l", &title("Month"), &title("Units")),
            ))
        );
        let read = &charts_of(&output)[0].chart;
        assert_eq!(
            read.plots,
            vec![plot_3d_read(
                ChartKind::Bar3D,
                Some(BarDirection::Column),
                Some(grouping),
                depth
            )]
        );
        assert_eq!(
            read.axes,
            read_axes_3d(AxisPosition::Bottom, AxisPosition::Left, depth)
        );
    }
    // Bars swap the axes as a 2D bar plot's do.
    let bars = titled(
        NewPlot::Bar3D {
            direction: BarDirection::Bar,
            grouping: Grouping::Clustered,
        },
        &ONE_NAMED,
        (Some("Month"), Some("Units")),
    );
    let output = edited_output(smallest(), |p| add(p, box_anchor(), &bars));
    let part = text_of(&output, CHART1);
    assert!(
        part.contains("<c:barDir val=\"bar\"/><c:grouping val=\"clustered\"/>"),
        "{part}"
    );
    assert_eq!(
        charts_of(&output)[0].chart.axes,
        read_axes_3d(AxisPosition::Left, AxisPosition::Bottom, false)
    );
}

#[test]
fn a_line_3d_chart_takes_each_grouping_but_clustered_with_the_series_axis() {
    for (grouping, value) in [
        (Grouping::Standard, "standard"),
        (Grouping::Stacked, "stacked"),
        (Grouping::PercentStacked, "percentStacked"),
    ] {
        let chart = titled(
            NewPlot::Line3D { grouping },
            &ONE_NAMED,
            (Some("Month"), Some("Units")),
        );
        let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
        let part = text_of(&output, CHART1);
        assert!(
            part.contains(&format!(
                "<c:autoTitleDeleted val=\"0\"/>{VIEW_3D}<c:plotArea><c:layout/><c:line3DChart>\
                 <c:grouping val=\"{value}\"/><c:varyColors val=\"0\"/>{}<c:axId val=\"10\"/>\
                 <c:axId val=\"20\"/><c:axId val=\"30\"/></c:line3DChart>{}{SERIES_AXIS}\
                 </c:plotArea>",
                one_named_written(),
                axes("b", "l", &title("Month"), &title("Units")),
            )),
            "{part}"
        );
        let read = &charts_of(&output)[0].chart;
        assert_eq!(
            read.plots,
            vec![plot_3d_read(ChartKind::Line3D, None, Some(grouping), true)]
        );
        assert_eq!(
            read.axes,
            read_axes_3d(AxisPosition::Bottom, AxisPosition::Left, true)
        );
    }
    let chart = bare(NewPlot::Line3D {
        grouping: Grouping::Clustered,
    });
    assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
}

#[test]
fn an_area_3d_chart_takes_each_grouping_but_clustered() {
    for (grouping, value) in [
        (Grouping::Standard, "standard"),
        (Grouping::Stacked, "stacked"),
        (Grouping::PercentStacked, "percentStacked"),
    ] {
        let depth = grouping == Grouping::Standard;
        let chart = titled(
            NewPlot::Area3D { grouping },
            &ONE_NAMED,
            (Some("Month"), Some("Units")),
        );
        let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
        let part = text_of(&output, CHART1);
        let (third, series_axis) = if depth {
            ("<c:axId val=\"30\"/>", SERIES_AXIS)
        } else {
            ("", "")
        };
        assert!(
            part.contains(&format!(
                "<c:autoTitleDeleted val=\"0\"/>{VIEW_3D}<c:plotArea><c:layout/><c:area3DChart>\
                 <c:grouping val=\"{value}\"/><c:varyColors val=\"0\"/>{}<c:axId val=\"10\"/>\
                 <c:axId val=\"20\"/>{third}</c:area3DChart>{}{series_axis}</c:plotArea>",
                one_named_written(),
                axes("b", "l", &title("Month"), &title("Units")),
            )),
            "{part}"
        );
        let read = &charts_of(&output)[0].chart;
        assert_eq!(
            read.plots,
            vec![plot_3d_read(ChartKind::Area3D, None, Some(grouping), depth)]
        );
        assert_eq!(
            read.axes,
            read_axes_3d(AxisPosition::Bottom, AxisPosition::Left, depth)
        );
    }
    let chart = bare(NewPlot::Area3D {
        grouping: Grouping::Clustered,
    });
    assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
}

#[test]
fn a_pie_3d_chart_is_written_and_refused_as_a_pie() {
    let chart = titled(NewPlot::Pie3D, &ONE_NAMED, (None, None));
    let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
    assert_eq!(
        text_of(&output, CHART1),
        chart_part(&format!(
            "{}<c:autoTitleDeleted val=\"0\"/><c:view3D><c:rotX val=\"30\"/><c:rotY val=\"0\"/>\
             <c:rAngAx val=\"0\"/></c:view3D><c:plotArea><c:layout/><c:pie3DChart><c:varyColors \
             val=\"1\"/>{}</c:pie3DChart></c:plotArea><c:legend><c:legendPos val=\"r\"/>\
             <c:overlay val=\"0\"/></c:legend><c:plotVisOnly val=\"1\"/><c:dispBlanksAs \
             val=\"gap\"/>",
            title("Share"),
            one_named_written(),
        ))
    );
    let read = &charts_of(&output)[0].chart;
    assert_eq!(
        read.plots,
        vec![plot_read(
            ChartKind::Pie3D,
            true,
            vec![one_named_read()],
            Vec::new()
        )]
    );
    assert!(read.axes.is_empty());
    let two = titled(NewPlot::Pie3D, &TWO_SERIES, (None, None));
    assert_package(call(box_anchor(), &two).expect_err("refused"), OPTION);
    for axis_titles in [(Some("Month"), None), (None, Some("Units"))] {
        let titled = titled(NewPlot::Pie3D, &ONE_NAMED, axis_titles);
        assert_package(call(box_anchor(), &titled).expect_err("refused"), OPTION);
    }
}

/// One series over C, for a combined plot.
const OVER_C: [NewSeries<'static>; 1] = [NewSeries {
    name: Some(NewSeriesName::Text("Share")),
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
}];

/// A second plot `plot` over `OVER_C`, on the primary axes.
fn second(plot: NewPlot) -> NewCombined<'static> {
    NewCombined {
        plot,
        series: &OVER_C,
        secondary_axis: false,
        secondary_axis_title: None,
        secondary_axis_format: None,
    }
}

/// The column chart with `combined`.
fn combined<'a>(combined: NewCombined<'a>) -> NewChart<'a> {
    NewChart {
        combined: Some(combined),
        ..column_chart()
    }
}

/// `OVER_C` as written at position `k`, its x references `x` (`cat` or
/// `xVal`) of form `x_form`, its y references `y`.
fn over_c_written(k: u32, x: &str, x_form: &str, y: &str) -> String {
    format!(
        "<c:ser><c:idx val=\"{k}\"/><c:order val=\"{k}\"/><c:tx><c:v>Share</c:v></c:tx><c:{x}>{}\
         </c:{x}><c:{y}>{}</c:{y}></c:ser>",
        reference(x_form, "Sheet1!$A$2:$A$4"),
        reference("numRef", "Sheet1!$C$2:$C$4"),
    )
}

/// `OVER_C` as read at position `k`.
fn over_c_read(k: u32) -> Series {
    series_ref(
        Some(SeriesName::Text("Share".to_owned())),
        Some("Sheet1!$A$2:$A$4"),
        "Sheet1!$C$2:$C$4",
        k,
    )
}

/// The secondary pair as written: the axis 30 at the bottom, deleted, a
/// `catAx` or (`between` `midCat`) a `valAx`, and the value axis 40 at the
/// right crossing at the maximum, with `title` and `format`'s form.
fn secondary_axes(title: &str, format: &str, between: &str) -> String {
    let tail = if between == "midCat" {
        "<c:crossBetween val=\"midCat\"/>".to_owned()
    } else {
        "<c:auto val=\"1\"/><c:lblAlgn val=\"ctr\"/><c:lblOffset val=\"100\"/><c:noMultiLvlLbl \
         val=\"0\"/>"
            .to_owned()
    };
    let x = if between == "midCat" {
        "valAx"
    } else {
        "catAx"
    };
    format!(
        "<c:{x}><c:axId val=\"30\"/><c:scaling><c:orientation val=\"minMax\"/></c:scaling>\
         <c:delete val=\"1\"/><c:axPos val=\"b\"/><c:numFmt formatCode=\"General\" \
         sourceLinked=\"1\"/><c:majorTickMark val=\"out\"/><c:minorTickMark val=\"none\"/>\
         <c:tickLblPos val=\"nextTo\"/><c:crossAx val=\"40\"/><c:crosses val=\"autoZero\"/>{tail}\
         </c:{x}><c:valAx><c:axId val=\"40\"/><c:scaling><c:orientation val=\"minMax\"/>\
         </c:scaling><c:delete val=\"0\"/><c:axPos val=\"r\"/>{title}<c:numFmt \
         formatCode=\"{format}\" sourceLinked=\"0\"/><c:majorTickMark val=\"out\"/>\
         <c:minorTickMark val=\"none\"/><c:tickLblPos val=\"nextTo\"/><c:crossAx val=\"30\"/>\
         <c:crosses val=\"max\"/><c:crossBetween val=\"{between}\"/></c:valAx>"
    )
}

/// The secondary pair as read, the x axis of `kind`, the value axis's
/// title and format.
fn secondary_read(kind: AxisKind, title: Option<&str>, format: &str) -> Vec<ChartAxis> {
    let base = ChartAxis {
        id: 30,
        kind,
        position: AxisPosition::Bottom,
        crosses: 40,
        title: None,
        number_format: Some("General".to_owned()),
        min: None,
        max: None,
        deleted: true,
        major_gridlines: false,
    };
    vec![
        base.clone(),
        ChartAxis {
            id: 40,
            kind: AxisKind::Value,
            position: AxisPosition::Right,
            crosses: 30,
            title: title.map(str::to_owned),
            number_format: Some(format.to_owned()),
            deleted: false,
            ..base
        },
    ]
}

/// The column chart's plot as read with its axis ids.
fn column_plot_read() -> Plot {
    column_read().plots.remove(0)
}

#[test]
fn a_bar_with_a_combined_line_on_the_primary_axes_reads_two_plots_and_two_axes() {
    let chart = combined(second(NewPlot::Line {
        grouping: Grouping::Standard,
    }));
    let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
    let part = text_of(&output, CHART1);
    assert!(
        part.contains(&format!(
            "<c:axId val=\"10\"/><c:axId val=\"20\"/></c:barChart><c:lineChart><c:grouping \
             val=\"standard\"/><c:varyColors val=\"0\"/>{}<c:marker val=\"1\"/><c:axId \
             val=\"10\"/><c:axId val=\"20\"/></c:lineChart>{}</c:plotArea>",
            over_c_written(2, "cat", "strRef", "val"),
            axes("b", "l", &title("Month"), &title("Units")),
        )),
        "{part}"
    );
    let read = &charts_of(&output)[0].chart;
    assert_eq!(
        read.plots,
        vec![
            column_plot_read(),
            Plot {
                grouping: Some(Grouping::Standard),
                ..plot_read(ChartKind::Line, false, vec![over_c_read(2)], vec![10, 20])
            },
        ]
    );
    assert_eq!(read.axes, column_read().axes);
}

#[test]
fn a_bar_with_a_combined_line_on_a_secondary_axis_reads_four_axes() {
    let chart = combined(NewCombined {
        secondary_axis: true,
        secondary_axis_title: Some("Share & rate"),
        secondary_axis_format: Some("0.0"),
        ..second(NewPlot::Line {
            grouping: Grouping::Stacked,
        })
    });
    let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
    let part = text_of(&output, CHART1);
    assert!(
        part.contains(&format!(
            "</c:barChart><c:lineChart><c:grouping val=\"stacked\"/><c:varyColors val=\"0\"/>{}\
             <c:marker val=\"1\"/><c:axId val=\"30\"/><c:axId val=\"40\"/></c:lineChart>{}{}\
             </c:plotArea>",
            over_c_written(2, "cat", "strRef", "val"),
            axes("b", "l", &title("Month"), &title("Units")),
            secondary_axes(&title("Share &amp; rate"), "0.0", "between"),
        )),
        "{part}"
    );
    let read = &charts_of(&output)[0].chart;
    assert_eq!(
        read.plots[1],
        Plot {
            grouping: Some(Grouping::Stacked),
            ..plot_read(ChartKind::Line, false, vec![over_c_read(2)], vec![30, 40])
        }
    );
    let mut axes = column_read().axes;
    axes.extend(secondary_read(
        AxisKind::Category,
        Some("Share & rate"),
        "0.0",
    ));
    assert_eq!(read.axes, axes);
    assert_eq!(
        read.axes.iter().map(|axis| axis.id).collect::<Vec<_>>(),
        [10, 20, 30, 40]
    );
}

#[test]
fn a_line_with_a_combined_area_shares_the_primary_axes() {
    let chart = NewChart {
        plot: NewPlot::Line {
            grouping: Grouping::Standard,
        },
        series: &ONE_NAMED,
        ..combined(second(NewPlot::Area {
            grouping: Grouping::Standard,
        }))
    };
    let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
    let part = text_of(&output, CHART1);
    assert!(
        part.contains(&format!(
            "<c:marker val=\"1\"/><c:axId val=\"10\"/><c:axId val=\"20\"/></c:lineChart>\
             <c:areaChart><c:grouping val=\"standard\"/><c:varyColors val=\"0\"/>{}<c:axId \
             val=\"10\"/><c:axId val=\"20\"/></c:areaChart>{}</c:plotArea>",
            over_c_written(1, "cat", "strRef", "val"),
            axes("b", "l", &title("Month"), &title("Units")),
        )),
        "{part}"
    );
    let read = &charts_of(&output)[0].chart;
    assert_eq!(
        read.plots.iter().map(|plot| plot.kind).collect::<Vec<_>>(),
        [ChartKind::Line, ChartKind::Area]
    );
    assert_eq!(read.plots[1].series, vec![over_c_read(1)]);
    assert_eq!(read.axes.len(), 2);
}

#[test]
fn a_bar_with_a_combined_scatter_on_a_secondary_axis_reads_two_value_axes() {
    let chart = combined(NewCombined {
        secondary_axis: true,
        secondary_axis_title: None,
        secondary_axis_format: Some("0\" kg\""),
        ..second(NewPlot::Scatter {
            style: ScatterStyle::Marker,
        })
    });
    let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
    let part = text_of(&output, CHART1);
    assert!(
        part.contains(&format!(
            "</c:barChart><c:scatterChart><c:scatterStyle val=\"marker\"/><c:varyColors \
             val=\"0\"/>{}<c:axId val=\"30\"/><c:axId val=\"40\"/></c:scatterChart>{}{}\
             </c:plotArea>",
            over_c_written(2, "xVal", "numRef", "yVal"),
            axes("b", "l", &title("Month"), &title("Units")),
            secondary_axes("", "0&quot; kg&quot;", "midCat"),
        )),
        "{part}"
    );
    let read = &charts_of(&output)[0].chart;
    assert_eq!(read.plots[1].kind, ChartKind::Scatter);
    assert_eq!(read.plots[1].axis_ids, vec![30, 40]);
    let mut axes = column_read().axes;
    axes.extend(secondary_read(AxisKind::Value, None, "0\" kg\""));
    assert_eq!(read.axes, axes);
}

#[test]
fn a_scatter_first_plot_a_same_variant_second_or_a_scatter_on_the_primary_axes_is_refused() {
    let scatter_first = NewChart {
        plot: NewPlot::Scatter {
            style: ScatterStyle::Marker,
        },
        ..combined(NewCombined {
            secondary_axis: true,
            ..second(NewPlot::Bar {
                direction: BarDirection::Column,
                grouping: Grouping::Clustered,
            })
        })
    };
    assert_package(
        call(box_anchor(), &scatter_first).expect_err("refused"),
        OPTION,
    );
    // A bar on a bar is one variant (P2 ruling H), whatever its options.
    let bar_on_bar = combined(second(NewPlot::Bar {
        direction: BarDirection::Bar,
        grouping: Grouping::Stacked,
    }));
    assert_package(
        call(box_anchor(), &bar_on_bar).expect_err("refused"),
        OPTION,
    );
    let scatter_primary = combined(second(NewPlot::Scatter {
        style: ScatterStyle::Marker,
    }));
    assert_package(
        call(box_anchor(), &scatter_primary).expect_err("refused"),
        OPTION,
    );
}

#[test]
fn a_pie_or_another_plot_with_no_axes_and_a_combined_plot_is_refused() {
    let line = second(NewPlot::Line {
        grouping: Grouping::Standard,
    });
    for plot in [
        NewPlot::Pie,
        NewPlot::Pie3D,
        NewPlot::Doughnut { hole_size: 50 },
        NewPlot::Radar {
            style: RadarStyle::Standard,
        },
        NewPlot::Bubble,
        NewPlot::Stock,
    ] {
        let chart = NewChart {
            combined: Some(line),
            ..titled(plot, &ONE_NAMED, (None, None))
        };
        assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
    }
}

#[test]
fn a_combined_3d_second_plot_is_refused() {
    for plot in [
        NewPlot::Bar3D {
            direction: BarDirection::Column,
            grouping: Grouping::Clustered,
        },
        NewPlot::Line3D {
            grouping: Grouping::Standard,
        },
        NewPlot::Pie3D,
        NewPlot::Area3D {
            grouping: Grouping::Standard,
        },
    ] {
        let chart = NewChart {
            plot: NewPlot::Line {
                grouping: Grouping::Standard,
            },
            ..combined(second(plot))
        };
        assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
    }
}

#[test]
fn a_combined_3d_first_plot_is_refused() {
    for plot in [
        NewPlot::Bar3D {
            direction: BarDirection::Column,
            grouping: Grouping::Clustered,
        },
        NewPlot::Line3D {
            grouping: Grouping::Standard,
        },
        NewPlot::Area3D {
            grouping: Grouping::Stacked,
        },
    ] {
        for secondary_axis in [false, true] {
            let chart = NewChart {
                plot,
                ..combined(NewCombined {
                    secondary_axis,
                    ..second(NewPlot::Line {
                        grouping: Grouping::Standard,
                    })
                })
            };
            assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
        }
    }
}

#[test]
fn a_secondary_title_or_format_without_the_secondary_axis_is_refused() {
    let line = second(NewPlot::Line {
        grouping: Grouping::Standard,
    });
    for (title, format) in [(Some("Rate"), None), (None, Some("0.0"))] {
        let chart = combined(NewCombined {
            secondary_axis_title: title,
            secondary_axis_format: format,
            ..line
        });
        assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
    }
}

#[test]
fn the_combined_rules_and_texts_follow_the_check_order() {
    let line = second(NewPlot::Line {
        grouping: Grouping::Standard,
    });
    // The second plot's own rules: a line's clustered grouping, no series,
    // sizes on a series that is not a bubble's.
    let clustered = combined(second(NewPlot::Line {
        grouping: Grouping::Clustered,
    }));
    assert_package(call(box_anchor(), &clustered).expect_err("refused"), OPTION);
    let empty = combined(NewCombined {
        series: &[],
        ..line
    });
    assert_package(call(box_anchor(), &empty).expect_err("refused"), OPTION);
    let sized = [NewSeries {
        sizes: Some("Sheet1!$D$2:$D$4"),
        ..OVER_C[0]
    }];
    let with_sizes = combined(NewCombined {
        series: &sized,
        ..line
    });
    assert_package(
        call(box_anchor(), &with_sizes).expect_err("refused"),
        OPTION,
    );
    // The texts after every rule: the secondary title and format, then the
    // second plot's series.
    let secondary = NewCombined {
        secondary_axis: true,
        ..line
    };
    for chart in [
        combined(NewCombined {
            secondary_axis_title: Some(""),
            ..secondary
        }),
        combined(NewCombined {
            secondary_axis_format: Some("0\u{7}"),
            ..secondary
        }),
        combined(NewCombined {
            series: &[NewSeries {
                values: "",
                ..OVER_C[0]
            }],
            ..secondary
        }),
    ] {
        assert_package(call(box_anchor(), &chart).expect_err("refused"), TEXT);
    }
    // A rule broken beside a bad text gives the rule's message.
    let both = combined(NewCombined {
        series: &[],
        secondary_axis_title: Some(""),
        ..secondary
    });
    assert_package(call(box_anchor(), &both).expect_err("refused"), OPTION);
    // The first plot's texts are checked apart from the second's: a bad
    // title or series reference of the first plot, beside a second plot
    // whose texts are all good, is refused; the same second plot alone is
    // accepted.
    let good = combined(NewCombined {
        secondary_axis_title: Some("emend y2"),
        secondary_axis_format: Some("0.0"),
        ..secondary
    });
    call(box_anchor(), &good).expect("accepted");
    let first_title = NewChart {
        title: Some(""),
        ..good
    };
    assert_package(call(box_anchor(), &first_title).expect_err("refused"), TEXT);
    let first_series = [NewSeries {
        values: "",
        ..OVER_C[0]
    }];
    let first_reference = NewChart {
        series: &first_series,
        ..good
    };
    assert_package(
        call(box_anchor(), &first_reference).expect_err("refused"),
        TEXT,
    );
}

/// Labels showing the value alone at `position`, with no format.
fn value_labels(position: Option<LabelPosition>) -> NewLabels<'static> {
    NewLabels {
        value: true,
        category: false,
        series_name: false,
        percent: false,
        position,
        number_format: None,
    }
}

/// `base` with `labels` on every series.
fn labelled<const N: usize>(
    base: [NewSeries<'static>; N],
    labels: NewLabels<'static>,
) -> [NewSeries<'static>; N] {
    base.map(|one| NewSeries {
        labels: Some(labels),
        ..one
    })
}

/// The `dLbls` form: the format and the position when given, then the
/// four flags `showVal`, `showCatName`, `showSerName`, `showPercent`.
fn labels_written(format: Option<&str>, position: Option<&str>, flags: [u8; 4]) -> String {
    let format = format.map_or(String::new(), |format| {
        format!("<c:numFmt formatCode=\"{format}\" sourceLinked=\"0\"/>")
    });
    let position = position.map_or(String::new(), |position| {
        format!("<c:dLblPos val=\"{position}\"/>")
    });
    let [value, category, name, percent] = flags;
    format!(
        "<c:dLbls>{format}{position}<c:showLegendKey val=\"0\"/><c:showVal val=\"{value}\"/>\
         <c:showCatName val=\"{category}\"/><c:showSerName val=\"{name}\"/><c:showPercent \
         val=\"{percent}\"/><c:showBubbleSize val=\"0\"/></c:dLbls>"
    )
}

/// `ONE_NAMED`'s name as written.
const ONE_NAMED_TX: &str = "<c:tx><c:strRef><c:f>Sheet1!$B$1</c:f></c:strRef></c:tx>";

/// The clustered column plot.
const COLUMN: NewPlot = NewPlot::Bar {
    direction: BarDirection::Column,
    grouping: Grouping::Clustered,
};

#[test]
fn labels_on_a_bar_series_take_each_position_it_takes_and_refuse_another() {
    for (position, written) in [
        (LabelPosition::Center, "ctr"),
        (LabelPosition::InsideEnd, "inEnd"),
        (LabelPosition::InsideBase, "inBase"),
        (LabelPosition::OutsideEnd, "outEnd"),
    ] {
        let series = labelled(ONE_NAMED, value_labels(Some(position)));
        let chart = NewChart {
            series: &series,
            ..bare(COLUMN)
        };
        let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
        let part = text_of(&output, CHART1);
        let expected = format!(
            "{ONE_NAMED_TX}{}<c:cat>",
            labels_written(None, Some(written), [1, 0, 0, 0])
        );
        assert!(part.contains(&expected), "{part}");
        assert!(charts_of(&output)[0].chart.plots[0].series[0].labels);
    }
    // A position a clustered bar does not take, and `OutsideEnd` on a
    // stacked bar, which takes the other three.
    for position in [
        LabelPosition::BestFit,
        LabelPosition::Left,
        LabelPosition::Right,
        LabelPosition::Above,
        LabelPosition::Below,
    ] {
        let series = labelled(ONE_NAMED, value_labels(Some(position)));
        let chart = NewChart {
            series: &series,
            ..bare(COLUMN)
        };
        assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
    }
    for grouping in [Grouping::Stacked, Grouping::PercentStacked] {
        let stacked = NewPlot::Bar {
            direction: BarDirection::Column,
            grouping,
        };
        let outside = labelled(ONE_NAMED, value_labels(Some(LabelPosition::OutsideEnd)));
        let chart = NewChart {
            series: &outside,
            ..bare(stacked)
        };
        assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
        let center = labelled(ONE_NAMED, value_labels(Some(LabelPosition::Center)));
        let chart = NewChart {
            series: &center,
            ..bare(stacked)
        };
        call(box_anchor(), &chart).expect("accepted");
    }
}

#[test]
fn each_plot_takes_the_label_positions_it_takes_and_no_other() {
    use LabelPosition::{
        Above, Below, BestFit, Center, InsideBase, InsideEnd, Left, OutsideEnd, Right,
    };
    const ALL: [LabelPosition; 9] = [
        BestFit, Center, InsideEnd, InsideBase, OutsideEnd, Left, Right, Above, Below,
    ];
    let stock = [STOCK[0], STOCK[1], STOCK[2]];
    let line_like: &[LabelPosition] = &[Center, Left, Right, Above, Below];
    let round: &[LabelPosition] = &[BestFit, Center, InsideEnd, OutsideEnd];
    // P3 ruling L: no position on the 3D bar, line and area; the 3D pie
    // takes the pie's.
    let cases: [(NewPlot, &[NewSeries<'static>], &[LabelPosition]); 13] = [
        (
            NewPlot::Line {
                grouping: Grouping::Standard,
            },
            &ONE_NAMED,
            line_like,
        ),
        (
            NewPlot::Scatter {
                style: ScatterStyle::Marker,
            },
            &ONE_NAMED,
            line_like,
        ),
        (
            NewPlot::Radar {
                style: RadarStyle::Standard,
            },
            &ONE_NAMED,
            line_like,
        ),
        (
            NewPlot::Radar {
                style: RadarStyle::Marker,
            },
            &ONE_NAMED,
            line_like,
        ),
        (
            NewPlot::Radar {
                style: RadarStyle::Filled,
            },
            &ONE_NAMED,
            &[],
        ),
        (NewPlot::Pie, &ONE_NAMED, round),
        (NewPlot::Pie3D, &ONE_NAMED, round),
        (NewPlot::Doughnut { hole_size: 50 }, &ONE_NAMED, &[]),
        (
            NewPlot::Area {
                grouping: Grouping::Standard,
            },
            &ONE_NAMED,
            &[],
        ),
        (NewPlot::Stock, &stock, &[]),
        (NewPlot::Bubble, &BUBBLES, &[]),
        (
            NewPlot::Bar3D {
                direction: BarDirection::Column,
                grouping: Grouping::Clustered,
            },
            &ONE_NAMED,
            &[],
        ),
        (
            NewPlot::Line3D {
                grouping: Grouping::Standard,
            },
            &ONE_NAMED,
            &[],
        ),
    ];
    let area_3d = (
        NewPlot::Area3D {
            grouping: Grouping::Standard,
        },
        &ONE_NAMED[..],
        &[][..],
    );
    for (plot, base, takes) in cases.into_iter().chain([area_3d]) {
        for position in ALL {
            let series: Vec<NewSeries<'static>> = base
                .iter()
                .map(|one| NewSeries {
                    labels: Some(value_labels(Some(position))),
                    ..*one
                })
                .collect();
            let chart = NewChart {
                series: &series,
                ..bare(plot)
            };
            let result = call(box_anchor(), &chart);
            if takes.contains(&position) {
                result.expect("accepted");
            } else {
                assert_package(result.expect_err("refused"), OPTION);
            }
        }
        // Every plot takes labels with no position.
        let series: Vec<NewSeries<'static>> = base
            .iter()
            .map(|one| NewSeries {
                labels: Some(value_labels(None)),
                ..*one
            })
            .collect();
        let chart = NewChart {
            series: &series,
            ..bare(plot)
        };
        call(box_anchor(), &chart).expect("accepted");
    }
}

#[test]
fn a_percentage_on_a_pie_is_written_and_on_a_bar_is_refused() {
    let percent = NewLabels {
        value: false,
        category: true,
        series_name: true,
        percent: true,
        position: Some(LabelPosition::BestFit),
        number_format: None,
    };
    let series = labelled(ONE_NAMED, percent);
    let pie = NewChart {
        series: &series,
        ..bare(NewPlot::Pie)
    };
    let output = edited_output(smallest(), |p| add(p, box_anchor(), &pie));
    let part = text_of(&output, CHART1);
    let expected = format!(
        "{ONE_NAMED_TX}{}<c:cat>",
        labels_written(None, Some("bestFit"), [0, 1, 1, 1])
    );
    assert!(part.contains(&expected), "{part}");
    assert!(charts_of(&output)[0].chart.plots[0].series[0].labels);
    for plot in [NewPlot::Pie3D, NewPlot::Doughnut { hole_size: 50 }] {
        let chart = NewChart {
            series: &labelled(
                ONE_NAMED,
                NewLabels {
                    position: None,
                    ..percent
                },
            ),
            ..bare(plot)
        };
        call(box_anchor(), &chart).expect("accepted");
    }
    let no_position = labelled(
        ONE_NAMED,
        NewLabels {
            position: None,
            ..percent
        },
    );
    for plot in [
        COLUMN,
        NewPlot::Line {
            grouping: Grouping::Standard,
        },
        NewPlot::Area {
            grouping: Grouping::Standard,
        },
    ] {
        let chart = NewChart {
            series: &no_position,
            ..bare(plot)
        };
        assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
    }
}

#[test]
fn a_label_format_is_written_escaped_in_the_label_bytes() {
    // The reader does not model a label's format: the bytes show it.
    let labels = NewLabels {
        value: true,
        category: false,
        series_name: false,
        percent: false,
        position: Some(LabelPosition::OutsideEnd),
        number_format: Some("#,##0\"<&>\""),
    };
    let series = labelled(ONE_NAMED, labels);
    let chart = NewChart {
        series: &series,
        ..bare(COLUMN)
    };
    let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
    let part = text_of(&output, CHART1);
    let expected = format!(
        "{ONE_NAMED_TX}{}<c:cat>",
        labels_written(
            Some("#,##0&quot;&lt;&amp;&gt;&quot;"),
            Some("outEnd"),
            [1, 0, 0, 0]
        )
    );
    assert!(part.contains(&expected), "{part}");
    // Every flag false is accepted and writes the form.
    let none = labelled(
        ONE_NAMED,
        NewLabels {
            value: false,
            position: None,
            number_format: None,
            ..labels
        },
    );
    let chart = NewChart {
        series: &none,
        ..bare(COLUMN)
    };
    let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
    assert!(
        text_of(&output, CHART1).contains(&labels_written(None, None, [0, 0, 0, 0])),
        "the form with every flag false"
    );
    assert!(charts_of(&output)[0].chart.plots[0].series[0].labels);
}

#[test]
fn a_colour_on_a_bar_series_reads_back_and_on_a_line_series_writes_the_line() {
    let coloured = [NewSeries {
        color: Some(0x0A_0B0C),
        ..ONE_NAMED[0]
    }];
    let fill = "<a:solidFill><a:srgbClr val=\"0A0B0C\"/></a:solidFill>";
    let bar = NewChart {
        series: &coloured,
        ..bare(COLUMN)
    };
    let output = edited_output(smallest(), |p| add(p, box_anchor(), &bar));
    let part = text_of(&output, CHART1);
    assert!(
        part.contains(&format!("{ONE_NAMED_TX}<c:spPr>{fill}</c:spPr><c:cat>")),
        "{part}"
    );
    assert_eq!(
        charts_of(&output)[0].chart.plots[0].series[0].fill,
        Some(0x0A_0B0C)
    );
    // The line's colour too on the plots whose line is what shows; the
    // fill alone on the others, a pie's colouring the whole series.
    for (plot, line) in [
        (
            NewPlot::Line {
                grouping: Grouping::Standard,
            },
            true,
        ),
        (
            NewPlot::Line3D {
                grouping: Grouping::Standard,
            },
            true,
        ),
        (
            NewPlot::Scatter {
                style: ScatterStyle::Marker,
            },
            true,
        ),
        (
            NewPlot::Radar {
                style: RadarStyle::Filled,
            },
            true,
        ),
        (
            NewPlot::Area {
                grouping: Grouping::Standard,
            },
            false,
        ),
        (NewPlot::Pie, false),
        (NewPlot::Doughnut { hole_size: 50 }, false),
    ] {
        let chart = NewChart {
            series: &coloured,
            ..bare(plot)
        };
        let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
        let ln = if line {
            format!("<a:ln>{fill}</a:ln>")
        } else {
            String::new()
        };
        let part = text_of(&output, CHART1);
        assert!(
            part.contains(&format!("{ONE_NAMED_TX}<c:spPr>{fill}{ln}</c:spPr>")),
            "{part}"
        );
        assert_eq!(
            charts_of(&output)[0].chart.plots[0].series[0].fill,
            Some(0x0A_0B0C)
        );
    }
    // The colour before the labels, both after the name.
    let both = [NewSeries {
        color: Some(0xFF_FFFF),
        labels: Some(value_labels(None)),
        ..ONE_NAMED[0]
    }];
    let chart = NewChart {
        series: &both,
        ..bare(COLUMN)
    };
    let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
    assert!(text_of(&output, CHART1).contains(&format!(
        "{ONE_NAMED_TX}<c:spPr><a:solidFill><a:srgbClr val=\"FFFFFF\"/></a:solidFill></c:spPr>{}\
         <c:cat>",
        labels_written(None, None, [1, 0, 0, 0])
    )));
}

#[test]
fn a_colour_above_ffffff_is_refused() {
    let series = [NewSeries {
        color: Some(0x100_0000),
        ..ONE_NAMED[0]
    }];
    let chart = NewChart {
        series: &series,
        ..bare(COLUMN)
    };
    assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
}

#[test]
fn a_colour_on_a_stock_series_is_refused() {
    // P1 ruling B: the lines are the high-low lines.
    let mut stock = [STOCK[0], STOCK[1], STOCK[2]];
    stock[1].color = Some(0x1F_77B4);
    let chart = NewChart {
        series: &stock,
        ..bare(NewPlot::Stock)
    };
    assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
}

#[test]
fn each_axis_format_is_written_and_read_back() {
    let chart = NewChart {
        category_axis_format: Some("@"),
        value_axis_format: Some("#,##0.00"),
        ..column_chart()
    };
    let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
    let part = text_of(&output, CHART1);
    assert!(
        part.contains(&format!(
            "{}<c:numFmt formatCode=\"@\" sourceLinked=\"0\"/>",
            title("Month")
        )),
        "{part}"
    );
    assert!(
        part.contains(&format!(
            "{}<c:numFmt formatCode=\"#,##0.00\" sourceLinked=\"0\"/>",
            title("Units")
        )),
        "{part}"
    );
    let read = &charts_of(&output)[0].chart;
    let formats: Vec<Option<&str>> = read
        .axes
        .iter()
        .map(|axis| axis.number_format.as_deref())
        .collect();
    assert_eq!(formats, [Some("@"), Some("#,##0.00")]);
    // A scatter's y axis takes the value format; its x axis keeps
    // `General`.
    let scatter = NewChart {
        value_axis_format: Some("0.0%"),
        ..bare(NewPlot::Scatter {
            style: ScatterStyle::Marker,
        })
    };
    let output = edited_output(smallest(), |p| add(p, box_anchor(), &scatter));
    let formats: Vec<Option<String>> = charts_of(&output)[0]
        .chart
        .axes
        .iter()
        .map(|axis| axis.number_format.clone())
        .collect();
    assert_eq!(
        formats,
        [Some("General".to_owned()), Some("0.0%".to_owned())]
    );
    // A 3D plot's series axis takes no format.
    let bar_3d = NewChart {
        category_axis_format: Some("@"),
        value_axis_format: Some("0"),
        ..bare(NewPlot::Bar3D {
            direction: BarDirection::Column,
            grouping: Grouping::Standard,
        })
    };
    let output = edited_output(smallest(), |p| add(p, box_anchor(), &bar_3d));
    let formats: Vec<Option<String>> = charts_of(&output)[0]
        .chart
        .axes
        .iter()
        .map(|axis| axis.number_format.clone())
        .collect();
    assert_eq!(formats, [Some("@".to_owned()), Some("0".to_owned()), None]);
}

#[test]
fn a_format_on_a_pie_or_a_category_format_on_a_scatter_is_refused() {
    for plot in [
        NewPlot::Pie,
        NewPlot::Pie3D,
        NewPlot::Doughnut { hole_size: 50 },
    ] {
        for (category, value) in [(Some("@"), None), (None, Some("0"))] {
            let chart = NewChart {
                category_axis_format: category,
                value_axis_format: value,
                ..bare(plot)
            };
            assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
        }
    }
    let scatter = NewChart {
        category_axis_format: Some("0"),
        ..bare(NewPlot::Scatter {
            style: ScatterStyle::Marker,
        })
    };
    assert_package(call(box_anchor(), &scatter).expect_err("refused"), OPTION);
    let bubble = NewChart {
        series: &BUBBLES,
        category_axis_format: Some("0"),
        ..bare(NewPlot::Bubble)
    };
    assert_package(call(box_anchor(), &bubble).expect_err("refused"), OPTION);
}

#[test]
fn a_combined_second_plot_s_series_take_labels_and_colours_by_its_own_rules() {
    // P3 ruling L: the second plot's series are checked as its kind's.
    let labelled_c = [NewSeries {
        color: Some(0x2C_A02C),
        labels: Some(value_labels(Some(LabelPosition::Above))),
        ..OVER_C[0]
    }];
    let chart = combined(NewCombined {
        series: &labelled_c,
        ..second(NewPlot::Line {
            grouping: Grouping::Standard,
        })
    });
    let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
    let fill = "<a:solidFill><a:srgbClr val=\"2CA02C\"/></a:solidFill>";
    let part = text_of(&output, CHART1);
    assert!(
        part.contains(&format!(
            "<c:spPr>{fill}<a:ln>{fill}</a:ln></c:spPr>{}<c:cat>",
            labels_written(None, Some("t"), [1, 0, 0, 0])
        )),
        "{part}"
    );
    let read = &charts_of(&output)[0].chart.plots[1].series[0];
    assert_eq!((read.fill, read.labels), (Some(0x2C_A02C), true));
    // A bar's position on the line second, and a percentage, refused.
    for labels in [
        value_labels(Some(LabelPosition::InsideEnd)),
        NewLabels {
            percent: true,
            ..value_labels(None)
        },
    ] {
        let series = [NewSeries {
            labels: Some(labels),
            ..OVER_C[0]
        }];
        let chart = combined(NewCombined {
            series: &series,
            ..second(NewPlot::Line {
                grouping: Grouping::Standard,
            })
        });
        assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
    }
}

#[test]
fn the_new_texts_follow_the_check_order() {
    // The axis formats after the axis titles and before the secondary
    // ones; a label format after the series' references.
    for chart in [
        NewChart {
            category_axis_format: Some(""),
            ..column_chart()
        },
        NewChart {
            value_axis_format: Some("0\u{1}"),
            ..column_chart()
        },
    ] {
        assert_package(call(box_anchor(), &chart).expect_err("refused"), TEXT);
    }
    let bad_format = labelled(
        ONE_NAMED,
        NewLabels {
            number_format: Some("\u{ffff}"),
            ..value_labels(None)
        },
    );
    let chart = NewChart {
        series: &bad_format,
        ..bare(COLUMN)
    };
    assert_package(call(box_anchor(), &chart).expect_err("refused"), TEXT);
    // An option rule broken beside each bad text gives the rule's message.
    let coloured = [NewSeries {
        color: Some(0x100_0000),
        ..bad_format[0]
    }];
    let chart = NewChart {
        series: &coloured,
        category_axis_format: Some(""),
        ..bare(COLUMN)
    };
    assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
}

/// The chart style and colours parts' content and relationship types.
const STYLE_TYPE: &str = "application/vnd.ms-office.chartstyle+xml";
const COLORS_TYPE: &str = "application/vnd.ms-office.chartcolorstyle+xml";
const STYLE_REL: &str = "http://schemas.microsoft.com/office/2011/relationships/chartStyle";
const COLORS_REL: &str = "http://schemas.microsoft.com/office/2011/relationships/chartColorStyle";

/// The messages of the removal.
const NO_CHART: &str = "no chart at that position";
const CHART_PART: &str = "a chart part cannot be read";

/// A kept chart part titled `Kept k`.
fn kept_chart(k: u32) -> String {
    chart_part(&format!(
        "{}<c:autoTitleDeleted val=\"0\"/><c:plotArea><c:layout/></c:plotArea>",
        title(&format!("Kept {k}"))
    ))
}

/// The kept anchor holding the frame of the `k`-th chart, naming `rId<k>`.
fn chart_anchor(k: u32) -> String {
    kept_anchor(k, 0, &frame(k, &format!("rId{k}")))
}

/// The drawing's relationship to the `k`-th chart part.
fn chart_rel(k: u32) -> String {
    relationship(
        &format!("rId{k}"),
        &format!("{ATTR_NS}/chart"),
        &format!("../charts/chart{k}.xml"),
    )
}

/// A sheet whose drawing holds `count` charts, the `k`-th's frame naming
/// `xl/charts/chart<k>.xml` through `rId<k>`, each chart part with its
/// `Override` and a rels part naming a style part and its own colours
/// part, `colors<k>.xml`; the style part is the chart's own,
/// `style<k>.xml`, or `style1.xml` for every chart when `shared`. Each
/// style and colours part has its `Override`.
fn charted(count: u32, shared: bool) -> Parts {
    let anchors: String = (1..=count).map(chart_anchor).collect();
    let drawing_rels: String = (1..=count).map(chart_rel).collect();
    let mut parts = with_drawing(BODY, drawing_xml(&anchors), Some(&drawing_rels), false);
    for k in 1..=count {
        with_part(
            &mut parts,
            &format!("xl/charts/chart{k}.xml"),
            &kept_chart(k),
            Some(CHART_TYPE),
        );
        let style = if shared { 1 } else { k };
        with_part(
            &mut parts,
            &format!("xl/charts/_rels/chart{k}.xml.rels"),
            &rels(&format!(
                "{}{}",
                relationship("rId1", STYLE_REL, &format!("style{style}.xml")),
                relationship("rId2", COLORS_REL, &format!("colors{k}.xml")),
            )),
            None,
        );
        if !shared || k == 1 {
            with_part(
                &mut parts,
                &format!("xl/charts/style{k}.xml"),
                &format!("{DECLARATION}\n<cs:chartStyle xmlns:cs=\"urn:emend:style\" id=\"{k}\"/>"),
                Some(STYLE_TYPE),
            );
        }
        with_part(
            &mut parts,
            &format!("xl/charts/colors{k}.xml"),
            &format!("{DECLARATION}\n<cs:colorStyle xmlns:cs=\"urn:emend:style\" id=\"{k}\"/>"),
            Some(COLORS_TYPE),
        );
    }
    parts
}

/// Records the removal of the chart at `index`.
#[track_caller]
fn remove(package: &mut Package<Cursor<Vec<u8>>>, index: usize) {
    package.remove_chart(SHEET, index).expect("recorded");
}

/// Whether the output holds an entry `name`.
fn has(output: &[u8], name: &str) -> bool {
    let package = Package::open(Cursor::new(output.to_vec())).expect("open the output");
    package.part(name.as_bytes()).is_some()
}

/// The content types of `parts` less the `Override` of each part named.
fn types_without(parts: &Parts, names: &[&str]) -> String {
    let mut types = part_text(parts, b"[Content_Types].xml");
    for name in names {
        let start = types
            .find(&format!("<Override PartName=\"/{name}\""))
            .expect("the override");
        let end = start + types[start..].find("/>").expect("its end") + 2;
        types.replace_range(start..end, "");
    }
    types
}

/// The entries of `parts` other than those named.
fn kept_names<'a>(parts: &'a Parts, gone: &[&str]) -> Vec<&'a [u8]> {
    parts
        .iter()
        .map(|(name, _)| name.as_slice())
        .filter(|name| !gone.iter().any(|gone| gone.as_bytes() == *name))
        .collect()
}

#[test]
fn a_removal_of_the_only_chart_drops_its_parts_and_the_drawing() {
    let mut parts = charted(1, false);
    // An external relationship of the chart's rels part goes with it and
    // reaches nothing.
    let chart_rels = part_text(&parts, b"xl/charts/_rels/chart1.xml.rels").replace(
        "</Relationships>",
        &format!(
            "<Relationship Id=\"rId3\" Type=\"{ATTR_NS}/hyperlink\" \
             Target=\"https://example.invalid/\" TargetMode=\"External\"/></Relationships>"
        ),
    );
    set(&mut parts, b"xl/charts/_rels/chart1.xml.rels", chart_rels);
    let output = edited_output(parts.clone(), |p| remove(p, 0));
    let gone = [
        "xl/drawings/drawing1.xml",
        "xl/drawings/_rels/drawing1.xml.rels",
        "xl/charts/chart1.xml",
        "xl/charts/_rels/chart1.xml.rels",
        "xl/charts/style1.xml",
        "xl/charts/colors1.xml",
    ];
    for name in gone {
        assert!(!has(&output, name), "{name}");
    }
    assert_eq!(text_of(&output, SHEET), sheet_r("<sheetData/>"));
    assert_eq!(text_of(&output, SHEET_RELS), rels(""));
    assert_eq!(
        text_of(&output, b"[Content_Types].xml"),
        types_without(
            &parts,
            &[
                "xl/drawings/drawing1.xml",
                "xl/charts/chart1.xml",
                "xl/charts/style1.xml",
                "xl/charts/colors1.xml",
            ]
        )
    );
    let mut except: Vec<&[u8]> = vec![SHEET, SHEET_RELS, b"[Content_Types].xml"];
    except.extend(gone.iter().map(|name| name.as_bytes()));
    assert_untouched(&output, &parts, &except);
    assert_names(&output, &kept_names(&parts, &gone));
    assert_eq!(charts_of(&output), []);
}

#[test]
fn a_removal_of_the_first_of_two_keeps_the_second_byte_for_byte() {
    let parts = charted(2, false);
    let output = edited_output(parts.clone(), |p| remove(p, 0));
    assert_eq!(text_of(&output, DRAWING), drawing_xml(&chart_anchor(2)));
    assert_eq!(text_of(&output, DRAWING_RELS), rels(&chart_rel(2)));
    let gone = [
        "xl/charts/chart1.xml",
        "xl/charts/_rels/chart1.xml.rels",
        "xl/charts/style1.xml",
        "xl/charts/colors1.xml",
    ];
    assert_eq!(
        text_of(&output, b"[Content_Types].xml"),
        types_without(
            &parts,
            &[
                "xl/charts/chart1.xml",
                "xl/charts/style1.xml",
                "xl/charts/colors1.xml",
            ]
        )
    );
    let mut except: Vec<&[u8]> = vec![DRAWING, DRAWING_RELS, b"[Content_Types].xml"];
    except.extend(gone.iter().map(|name| name.as_bytes()));
    assert_untouched(&output, &parts, &except);
    assert_names(&output, &kept_names(&parts, &gone));
    let read = charts_of(&output);
    assert_eq!(read.len(), 1);
    assert_eq!(read[0].part, b"xl/charts/chart2.xml");
    assert_eq!(read[0].chart.title.as_deref(), Some("Kept 2"));
}

#[test]
fn a_removal_keeps_a_style_part_another_chart_reaches() {
    let parts = charted(2, true);
    let output = edited_output(parts.clone(), |p| remove(p, 0));
    for name in [
        "xl/charts/chart1.xml",
        "xl/charts/_rels/chart1.xml.rels",
        "xl/charts/colors1.xml",
    ] {
        assert!(!has(&output, name), "{name}");
    }
    assert!(has(&output, "xl/charts/style1.xml"));
    assert_eq!(
        text_of(&output, b"[Content_Types].xml"),
        types_without(&parts, &["xl/charts/chart1.xml", "xl/charts/colors1.xml"])
    );
    assert_eq!(
        text_of(&output, b"xl/charts/style1.xml"),
        part_text(&parts, b"xl/charts/style1.xml")
    );
}

#[test]
fn a_removal_takes_the_user_shapes_and_what_only_they_reach() {
    let mut parts = charted(1, false);
    let chart_rels = part_text(&parts, b"xl/charts/_rels/chart1.xml.rels").replace(
        "</Relationships>",
        &format!(
            "{}</Relationships>",
            relationship(
                "rId3",
                &format!("{ATTR_NS}/chartUserShapes"),
                "../drawings/drawing2.xml"
            )
        ),
    );
    set(&mut parts, b"xl/charts/_rels/chart1.xml.rels", chart_rels);
    with_part(
        &mut parts,
        "xl/drawings/drawing2.xml",
        &format!("{DECLARATION}\n<c:userShapes xmlns:c=\"{C_NS}\"/>"),
        Some("application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml"),
    );
    with_part(
        &mut parts,
        "xl/drawings/_rels/drawing2.xml.rels",
        &rels(&relationship(
            "rId1",
            &format!("{ATTR_NS}/image"),
            "../media/image1.png",
        )),
        None,
    );
    parts.push((b"xl/media/image1.png".to_vec(), PNG.to_vec()));
    let output = edited_output(parts, |p| remove(p, 0));
    for name in [
        "xl/charts/chart1.xml",
        "xl/drawings/drawing2.xml",
        "xl/drawings/_rels/drawing2.xml.rels",
        "xl/media/image1.png",
    ] {
        assert!(!has(&output, name), "{name}");
    }
}

#[test]
fn an_index_past_the_list_named_twice_or_with_an_addition_is_refused() {
    let parts = charted(2, false);
    assert_package(refused_with(parts.clone(), |p| remove(p, 2)), NO_CHART);
    assert_package(
        refused_with(parts.clone(), |p| {
            remove(p, 1);
            remove(p, 1);
        }),
        NO_CHART,
    );
    assert_package(
        refused_with(parts.clone(), |p| {
            remove(p, 0);
            add(p, box_anchor(), &column_chart());
        }),
        NO_CHART,
    );
    assert_package(
        refused_with(parts, |p| {
            add(p, box_anchor(), &column_chart());
            remove(p, 0);
        }),
        NO_CHART,
    );
    // A sheet with no drawing has no chart at any position, and a picture
    // is not a chart.
    assert_package(refused_with(smallest(), |p| remove(p, 0)), NO_CHART);
    let picture_rel = relationship("rId1", &format!("{ATTR_NS}/image"), "../media/image1.png");
    let pictured = with_drawing(
        BODY,
        drawing_xml(&kept_anchor(0, 0, &pic(1, "rId1"))),
        Some(&picture_rel),
        true,
    );
    assert_package(refused_with(pictured, |p| remove(p, 0)), NO_CHART);
}

#[test]
fn a_frame_naming_no_chart_part_is_refused_at_the_write() {
    // The chart part missing.
    let mut missing = charted(1, false);
    missing.retain(|(name, _)| name.as_slice() != CHART1);
    assert_package(refused_with(missing, |p| remove(p, 0)), CHART_PART);
    // The relationship missing.
    let mut no_rel = charted(1, false);
    set(&mut no_rel, DRAWING_RELS, rels(""));
    assert_package(refused_with(no_rel, |p| remove(p, 0)), CHART_PART);
    // A part whose root is not `chartSpace`.
    let mut other = charted(1, false);
    set(
        &mut other,
        CHART1,
        format!("{DECLARATION}\n<c:chart xmlns:c=\"{C_NS}\"/>"),
    );
    assert_package(refused_with(other, |p| remove(p, 0)), CHART_PART);
}

#[test]
fn a_chart_removal_on_a_removed_or_replaced_part_is_refused() {
    let parts = charted(1, false);
    assert_package(
        refused_with(parts.clone(), |p| {
            p.remove_part(SHEET);
            remove(p, 0);
        }),
        "an edit to a removed part",
    );
    assert_package(
        refused_with(parts.clone(), |p| {
            p.replace_part(SHEET, replacement_from_reader(Cursor::new(b"x".to_vec())));
            remove(p, 0);
        }),
        "a cell edit to a part this write replaces",
    );
    assert_package(
        refused_with(parts, |p| {
            p.remove_part(DRAWING);
            remove(p, 0);
        }),
        "an edit to a removed part",
    );
}

#[cfg(feature = "images")]
#[test]
fn an_image_and_a_chart_removed_from_one_sheet_in_one_write() {
    let anchors = format!("{}{}", kept_anchor(0, 0, &pic(1, "rId1")), chart_anchor(2));
    let drawing_rels = format!(
        "{}{}",
        relationship("rId1", &format!("{ATTR_NS}/image"), "../media/image1.png"),
        chart_rel(2)
    );
    let mut parts = with_drawing(BODY, drawing_xml(&anchors), Some(&drawing_rels), true);
    with_part(
        &mut parts,
        "xl/charts/chart2.xml",
        &kept_chart(2),
        Some(CHART_TYPE),
    );
    // Each alone keeps the drawing with the other.
    let output = edited_output(parts.clone(), |p| remove(p, 0));
    assert_eq!(
        text_of(&output, DRAWING),
        drawing_xml(&kept_anchor(0, 0, &pic(1, "rId1")))
    );
    assert!(!has(&output, "xl/charts/chart2.xml"));
    let output = edited_output(parts.clone(), |p| {
        p.remove_image(SHEET, 0).expect("recorded")
    });
    assert_eq!(text_of(&output, DRAWING), drawing_xml(&chart_anchor(2)));
    // Both in one write empty the drawing.
    let output = edited_output(parts.clone(), |p| {
        remove(p, 0);
        p.remove_image(SHEET, 0).expect("recorded");
    });
    let gone = [
        "xl/drawings/drawing1.xml",
        "xl/drawings/_rels/drawing1.xml.rels",
        "xl/media/image1.png",
        "xl/charts/chart2.xml",
    ];
    for name in gone {
        assert!(!has(&output, name), "{name}");
    }
    assert_eq!(text_of(&output, SHEET), sheet_r("<sheetData/>"));
    assert_names(&output, &kept_names(&parts, &gone));
    // A chart removal named twice beside an image removal is refused as a
    // chart's.
    assert_package(
        refused_with(parts, |p| {
            p.remove_image(SHEET, 0).expect("recorded");
            remove(p, 0);
            remove(p, 0);
        }),
        NO_CHART,
    );
}

#[cfg(feature = "images")]
#[test]
fn a_chart_removal_on_a_part_the_same_write_adds_an_image_to_is_refused() {
    use crate::package::Image;
    // P3 ruling O: an image added to the part refuses a chart removal there,
    // whichever is recorded first, as a chart added does.
    let image = |p: &mut Package<Cursor<Vec<u8>>>| {
        p.add_image(
            SHEET,
            at("H2"),
            Image {
                bytes: &PNG,
                size: None,
            },
        )
        .expect("recorded");
    };
    let parts = charted(1, false);
    assert_package(
        refused_with(parts.clone(), |p| {
            remove(p, 0);
            image(p);
        }),
        NO_CHART,
    );
    assert_package(
        refused_with(parts, |p| {
            image(p);
            remove(p, 0);
        }),
        NO_CHART,
    );
}

/// Three series over B to D against A, each with a reference name, for the
/// surfaces (`specs/0039-chart-kinds-and-formatting.md`, "P1").
const THREE_NAMED: [NewSeries<'static>; 3] = [
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
    NewSeries {
        name: Some(NewSeriesName::Reference("Sheet1!$D$1")),
        categories: Some("Sheet1!$A$2:$A$4"),
        values: "Sheet1!$D$2:$D$4",
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

/// The series forms of `THREE_NAMED` as written: `idx`, `order`, `tx`,
/// `cat` and `val`, nothing else.
fn three_named_written() -> String {
    ["B", "C", "D"]
        .iter()
        .enumerate()
        .map(|(k, column)| {
            format!(
                "<c:ser><c:idx val=\"{k}\"/><c:order val=\"{k}\"/><c:tx>{}</c:tx><c:cat>{}\
                 </c:cat><c:val>{}</c:val></c:ser>",
                reference("strRef", &format!("Sheet1!${column}$1")),
                reference("strRef", "Sheet1!$A$2:$A$4"),
                reference("numRef", &format!("Sheet1!${column}$2:${column}$4")),
            )
        })
        .collect()
}

/// `THREE_NAMED` as `charts` reads it.
fn three_named_read() -> Vec<Series> {
    ["B", "C", "D"]
        .iter()
        .zip(0..)
        .map(|(column, k)| {
            series_ref(
                Some(SeriesName::Reference(format!("Sheet1!${column}$1"))),
                Some("Sheet1!$A$2:$A$4"),
                &format!("Sheet1!${column}$2:${column}$4"),
                k,
            )
        })
        .collect()
}

/// The contour's view: from above, no perspective.
const CONTOUR_VIEW: &str = "<c:view3D><c:rotX val=\"90\"/><c:rotY val=\"0\"/><c:rAngAx \
                            val=\"0\"/><c:perspective val=\"0\"/></c:view3D>";

#[test]
fn a_contour_and_a_3d_surface_of_three_series_take_each_wireframe_and_three_axes() {
    for (three_d, wireframe) in [(false, false), (false, true), (true, false), (true, true)] {
        let (plot, element, view, kind) = if three_d {
            (
                NewPlot::Surface3D { wireframe },
                "surface3DChart",
                VIEW_3D,
                ChartKind::Surface3D,
            )
        } else {
            (
                NewPlot::Surface { wireframe },
                "surfaceChart",
                CONTOUR_VIEW,
                ChartKind::Surface,
            )
        };
        let chart = titled(plot, &THREE_NAMED, (Some("Month"), Some("Units")));
        let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
        let wire = u8::from(wireframe);
        // The plot: `wireframe`, the series, the three axis ids; no
        // `varyColors`; the axes a line's, then the series axis last.
        assert_eq!(
            text_of(&output, CHART1),
            chart_part(&format!(
                "{}<c:autoTitleDeleted val=\"0\"/>{view}<c:plotArea><c:layout/><c:{element}>\
                 <c:wireframe val=\"{wire}\"/>{}<c:axId val=\"10\"/><c:axId val=\"20\"/>\
                 <c:axId val=\"30\"/></c:{element}>{}{SERIES_AXIS}</c:plotArea><c:legend>\
                 <c:legendPos val=\"r\"/><c:overlay val=\"0\"/></c:legend><c:plotVisOnly \
                 val=\"1\"/><c:dispBlanksAs val=\"gap\"/>",
                title("Share"),
                three_named_written(),
                axes("b", "l", &title("Month"), &title("Units")),
            ))
        );
        let read = &charts_of(&output)[0].chart;
        assert_eq!(
            read.plots,
            vec![plot_read(kind, false, three_named_read(), vec![10, 20, 30])]
        );
        assert_eq!(
            read.axes,
            read_axes_3d(AxisPosition::Bottom, AxisPosition::Left, true)
        );
    }
    // A surface takes one series too, and the axis formats, written and read
    // back with the plot whole (P1 verification ruling E (c)), after the
    // four above so that their outputs keep their order.
    for (three_d, wireframe) in [(false, false), (false, true), (true, false), (true, true)] {
        let (plot, element, kind) = if three_d {
            (
                NewPlot::Surface3D { wireframe },
                "surface3DChart",
                ChartKind::Surface3D,
            )
        } else {
            (
                NewPlot::Surface { wireframe },
                "surfaceChart",
                ChartKind::Surface,
            )
        };
        let wire = u8::from(wireframe);
        let one = NewChart {
            category_axis_format: Some("0"),
            value_axis_format: Some("0.0"),
            ..titled(plot, &ONE_NAMED, (None, None))
        };
        let output = edited_output(smallest(), |p| add(p, box_anchor(), &one));
        let part = text_of(&output, CHART1);
        assert!(
            part.contains(&format!(
                "<c:plotArea><c:layout/><c:{element}><c:wireframe val=\"{wire}\"/>{}<c:axId \
                 val=\"10\"/><c:axId val=\"20\"/><c:axId val=\"30\"/></c:{element}><c:catAx>",
                one_named_written(),
            )),
            "{part}"
        );
        let read = &charts_of(&output)[0].chart;
        assert_eq!(
            read.plots,
            vec![plot_read(
                kind,
                false,
                vec![one_named_read()],
                vec![10, 20, 30]
            )]
        );
        let formats: Vec<_> = read
            .axes
            .iter()
            .map(|axis| axis.number_format.as_deref())
            .collect();
        assert_eq!(formats, [Some("0"), Some("0.0"), None]);
    }
}

#[test]
fn a_surface_series_with_a_colour_or_labels_is_refused() {
    for plot in [
        NewPlot::Surface { wireframe: false },
        NewPlot::Surface3D { wireframe: true },
    ] {
        let coloured = THREE_NAMED.map(|one| NewSeries {
            color: Some(0x00_80FF),
            ..one
        });
        let chart = titled(plot, &coloured, (None, None));
        assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
        let last_coloured = [
            THREE_NAMED[0],
            THREE_NAMED[1],
            NewSeries {
                color: Some(0),
                ..THREE_NAMED[2]
            },
        ];
        let chart = titled(plot, &last_coloured, (None, None));
        assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
        for labels in [
            value_labels(None),
            value_labels(Some(LabelPosition::Center)),
        ] {
            let labelled = labelled(THREE_NAMED, labels);
            let chart = titled(plot, &labelled, (None, None));
            assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
        }
    }
}

#[test]
fn a_surface_as_either_combined_plot_is_refused() {
    let line = second(NewPlot::Line {
        grouping: Grouping::Standard,
    });
    for plot in [
        NewPlot::Surface { wireframe: false },
        NewPlot::Surface3D { wireframe: false },
    ] {
        let first = NewChart {
            combined: Some(line),
            ..titled(plot, &THREE_NAMED, (None, None))
        };
        assert_package(call(box_anchor(), &first).expect_err("refused"), OPTION);
        let as_second = combined(second(plot));
        assert_package(call(box_anchor(), &as_second).expect_err("refused"), OPTION);
        let on_secondary = combined(NewCombined {
            secondary_axis: true,
            ..second(plot)
        });
        assert_package(
            call(box_anchor(), &on_secondary).expect_err("refused"),
            OPTION,
        );
    }
}

/// The of-pie plot's form around `ONE_NAMED`: `ofPieType` `kind` first,
/// then `varyColors` 1, the series, and the options after it, the split's
/// `split` (its `splitType`, then its `splitPos` when given).
fn of_pie_written(kind: &str, split: &str) -> String {
    format!(
        "<c:ofPieChart><c:ofPieType val=\"{kind}\"/><c:varyColors val=\"1\"/>{}<c:gapWidth \
         val=\"150\"/>{split}<c:secondPieSize val=\"75\"/><c:serLines/></c:ofPieChart>",
        one_named_written(),
    )
}

#[test]
fn an_of_pie_of_each_type_and_each_split_writes_its_form() {
    for (kind, kind_value) in [(OfPieKind::Pie, "pie"), (OfPieKind::Bar, "bar")] {
        for (split, split_form) in [
            (None, "<c:splitType val=\"auto\"/>"),
            (
                Some(OfPieSplit::Position(2)),
                "<c:splitType val=\"pos\"/><c:splitPos val=\"2\"/>",
            ),
            (
                Some(OfPieSplit::Position(1)),
                "<c:splitType val=\"pos\"/><c:splitPos val=\"1\"/>",
            ),
            (
                Some(OfPieSplit::Value(2.5)),
                "<c:splitType val=\"val\"/><c:splitPos val=\"2.5\"/>",
            ),
            (
                Some(OfPieSplit::Value(0.0)),
                "<c:splitType val=\"val\"/><c:splitPos val=\"0\"/>",
            ),
            (
                Some(OfPieSplit::Value(1250.0)),
                "<c:splitType val=\"val\"/><c:splitPos val=\"1250\"/>",
            ),
            (
                Some(OfPieSplit::Percent(10.0)),
                "<c:splitType val=\"percent\"/><c:splitPos val=\"10\"/>",
            ),
            (
                Some(OfPieSplit::Percent(12.75)),
                "<c:splitType val=\"percent\"/><c:splitPos val=\"12.75\"/>",
            ),
            (
                Some(OfPieSplit::Percent(100.0)),
                "<c:splitType val=\"percent\"/><c:splitPos val=\"100\"/>",
            ),
        ] {
            let chart = titled(NewPlot::OfPie { kind, split }, &ONE_NAMED, (None, None));
            let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
            // No view, no axes.
            assert_eq!(
                text_of(&output, CHART1),
                chart_part(&format!(
                    "{}<c:autoTitleDeleted val=\"0\"/><c:plotArea><c:layout/>{}</c:plotArea>\
                     <c:legend><c:legendPos val=\"r\"/><c:overlay val=\"0\"/></c:legend>\
                     <c:plotVisOnly val=\"1\"/><c:dispBlanksAs val=\"gap\"/>",
                    title("Share"),
                    of_pie_written(kind_value, split_form),
                ))
            );
            let read = &charts_of(&output)[0].chart;
            assert_eq!(
                read.plots,
                vec![plot_read(
                    ChartKind::OfPie,
                    true,
                    vec![one_named_read()],
                    Vec::new()
                )]
            );
            assert!(read.axes.is_empty());
        }
    }
}

#[test]
fn an_of_pie_takes_a_colour_the_percent_and_a_pie_s_positions() {
    let plot = NewPlot::OfPie {
        kind: OfPieKind::Bar,
        split: None,
    };
    let coloured = [NewSeries {
        color: Some(0x12_34AB),
        ..ONE_NAMED[0]
    }];
    let output = edited_output(smallest(), |p| {
        add(p, box_anchor(), &titled(plot, &coloured, (None, None)));
    });
    let part = text_of(&output, CHART1);
    assert!(
        part.contains(&format!(
            "<c:ofPieChart><c:ofPieType val=\"bar\"/><c:varyColors val=\"1\"/><c:ser><c:idx \
             val=\"0\"/><c:order val=\"0\"/>{ONE_NAMED_TX}<c:spPr><a:solidFill><a:srgbClr \
             val=\"1234AB\"/></a:solidFill></c:spPr><c:cat>{}</c:cat><c:val>{}</c:val></c:ser>\
             <c:gapWidth val=\"150\"/><c:splitType val=\"auto\"/><c:secondPieSize val=\"75\"/>\
             <c:serLines/></c:ofPieChart>",
            reference("strRef", "Sheet1!$A$2:$A$4"),
            reference("numRef", "Sheet1!$B$2:$B$4"),
        )),
        "{part}"
    );
    // Read back, the plot whole (P1 verification ruling E (c)): the series'
    // colour its fill.
    let read = &charts_of(&output)[0].chart;
    assert_eq!(
        read.plots,
        vec![plot_read(
            ChartKind::OfPie,
            true,
            vec![Series {
                fill: Some(0x12_34AB),
                ..one_named_read()
            }],
            Vec::new()
        )]
    );
    use LabelPosition::{
        Above, Below, BestFit, Center, InsideBase, InsideEnd, Left, OutsideEnd, Right,
    };
    for position in [BestFit, Center, InsideEnd, OutsideEnd] {
        let labels = NewLabels {
            percent: true,
            ..value_labels(Some(position))
        };
        let labelled = labelled(ONE_NAMED, labels);
        call(box_anchor(), &titled(plot, &labelled, (None, None))).expect("taken");
    }
    for position in [InsideBase, Left, Right, Above, Below] {
        let labelled = labelled(ONE_NAMED, value_labels(Some(position)));
        let chart = titled(plot, &labelled, (None, None));
        assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
    }
}

#[test]
fn an_of_pie_with_two_series_an_axis_option_or_a_combined_plot_is_refused() {
    let plot = NewPlot::OfPie {
        kind: OfPieKind::Pie,
        split: Some(OfPieSplit::Position(2)),
    };
    let two = titled(plot, &TWO_NAMED, (None, None));
    assert_package(call(box_anchor(), &two).expect_err("refused"), OPTION);
    for axis_titles in [(Some("Month"), None), (None, Some("Units"))] {
        let chart = titled(plot, &ONE_NAMED, axis_titles);
        assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
    }
    let formats = [
        NewChart {
            category_axis_format: Some("0"),
            ..titled(plot, &ONE_NAMED, (None, None))
        },
        NewChart {
            value_axis_format: Some("0"),
            ..titled(plot, &ONE_NAMED, (None, None))
        },
    ];
    for chart in formats {
        assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
    }
    let line = second(NewPlot::Line {
        grouping: Grouping::Standard,
    });
    let first = NewChart {
        combined: Some(line),
        ..titled(plot, &ONE_NAMED, (None, None))
    };
    assert_package(call(box_anchor(), &first).expect_err("refused"), OPTION);
    let as_second = combined(second(plot));
    assert_package(call(box_anchor(), &as_second).expect_err("refused"), OPTION);
}

#[test]
fn an_of_pie_split_out_of_its_range_is_refused() {
    for split in [
        OfPieSplit::Position(0),
        OfPieSplit::Value(-1.0),
        OfPieSplit::Value(-0.5),
        OfPieSplit::Value(f64::NAN),
        OfPieSplit::Value(f64::INFINITY),
        OfPieSplit::Percent(100.5),
        OfPieSplit::Percent(-1.0),
        OfPieSplit::Percent(f64::NAN),
        OfPieSplit::Percent(f64::INFINITY),
    ] {
        let plot = NewPlot::OfPie {
            kind: OfPieKind::Bar,
            split: Some(split),
        };
        let chart = titled(plot, &ONE_NAMED, (None, None));
        assert_package(call(box_anchor(), &chart).expect_err("refused"), RANGE);
        // After the first plot's kind rules: two series give the option
        // message.
        let two = titled(plot, &TWO_NAMED, (None, None));
        assert_package(call(box_anchor(), &two).expect_err("refused"), OPTION);
        // Before the combined plot's rules and the texts.
        let first = NewChart {
            combined: Some(second(NewPlot::Line {
                grouping: Grouping::Standard,
            })),
            ..chart
        };
        assert_package(call(box_anchor(), &first).expect_err("refused"), RANGE);
        let empty_title = NewChart {
            title: Some(""),
            ..chart
        };
        assert_package(
            call(box_anchor(), &empty_title).expect_err("refused"),
            RANGE,
        );
    }
}

/// A marker of `symbol` with `size` and `color`.
fn marker(symbol: MarkerSymbol, size: Option<u8>, color: Option<u32>) -> NewMarker {
    NewMarker {
        symbol,
        size,
        color,
    }
}

/// A point at `index` with `color`, `explosion` and `marker`.
fn point(
    index: u32,
    color: Option<u32>,
    explosion: Option<u32>,
    marker: Option<NewMarker>,
) -> NewPoint {
    NewPoint {
        index,
        color,
        explosion,
        marker,
    }
}

/// The `solidFill` of `rgb`, six hex digits.
fn fill_of(rgb: &str) -> String {
    format!("<a:solidFill><a:srgbClr val=\"{rgb}\"/></a:solidFill>")
}

/// A marker's form: the symbol `symbol`, the size and the fill of `rgb`
/// when given (`specs/0039-chart-kinds-and-formatting.md`, "The chart
/// part").
fn marker_written(symbol: &str, size: Option<u8>, rgb: Option<&str>) -> String {
    let size = size.map_or(String::new(), |size| format!("<c:size val=\"{size}\"/>"));
    let fill = rgb.map_or(String::new(), |rgb| {
        format!("<c:spPr>{}</c:spPr>", fill_of(rgb))
    });
    format!("<c:marker><c:symbol val=\"{symbol}\"/>{size}{fill}</c:marker>")
}

/// `ONE_NAMED`'s series as written with `options` (its `spPr`, marker and
/// points) between the name and the categories.
fn one_named_with(options: &str) -> String {
    format!(
        "<c:ser><c:idx val=\"0\"/><c:order val=\"0\"/>{ONE_NAMED_TX}{options}<c:cat>{}</c:cat>\
         <c:val>{}</c:val></c:ser>",
        reference("strRef", "Sheet1!$A$2:$A$4"),
        reference("numRef", "Sheet1!$B$2:$B$4"),
    )
}

/// The standard line plot.
const LINE: NewPlot = NewPlot::Line {
    grouping: Grouping::Standard,
};

/// `series` with `option` applied to each.
fn each_with<'a>(
    series: &[NewSeries<'a>],
    option: impl Fn(NewSeries<'a>) -> NewSeries<'a>,
) -> Vec<NewSeries<'a>> {
    series.iter().copied().map(option).collect()
}

/// Every kind with series it takes, and whether its series take a marker,
/// a line and an explosion (`specs/0039-chart-kinds-and-formatting.md`,
/// "Applicability").
fn every_kind() -> Vec<(NewPlot, &'static [NewSeries<'static>], [bool; 3])> {
    vec![
        (COLUMN, &ONE_NAMED, [false, false, false]),
        (LINE, &ONE_NAMED, [true, true, false]),
        (NewPlot::Pie, &ONE_NAMED, [false, false, true]),
        (
            NewPlot::Scatter {
                style: ScatterStyle::LineMarker,
            },
            &ONE_NAMED,
            [true, true, false],
        ),
        (
            NewPlot::Area {
                grouping: Grouping::Standard,
            },
            &ONE_NAMED,
            [false, false, false],
        ),
        (
            NewPlot::Doughnut { hole_size: 50 },
            &ONE_NAMED,
            [false, false, true],
        ),
        (
            NewPlot::Radar {
                style: RadarStyle::Standard,
            },
            &ONE_NAMED,
            [true, true, false],
        ),
        (
            NewPlot::Radar {
                style: RadarStyle::Marker,
            },
            &ONE_NAMED,
            [true, true, false],
        ),
        (
            NewPlot::Radar {
                style: RadarStyle::Filled,
            },
            &ONE_NAMED,
            [false, true, false],
        ),
        (NewPlot::Bubble, &BUBBLES, [false, false, false]),
        (NewPlot::Stock, &STOCK[..3], [false, true, false]),
        (
            NewPlot::Bar3D {
                direction: BarDirection::Column,
                grouping: Grouping::Clustered,
            },
            &ONE_NAMED,
            [false, false, false],
        ),
        (
            NewPlot::Line3D {
                grouping: Grouping::Standard,
            },
            &ONE_NAMED,
            [false, true, false],
        ),
        (NewPlot::Pie3D, &ONE_NAMED, [false, false, true]),
        (
            NewPlot::Area3D {
                grouping: Grouping::Standard,
            },
            &ONE_NAMED,
            [false, false, false],
        ),
        (
            NewPlot::Surface { wireframe: false },
            &ONE_NAMED,
            [false, false, false],
        ),
        (
            NewPlot::Surface3D { wireframe: true },
            &ONE_NAMED,
            [false, false, false],
        ),
        (
            NewPlot::OfPie {
                kind: OfPieKind::Pie,
                split: None,
            },
            &ONE_NAMED,
            [false, false, true],
        ),
    ]
}

/// The eleven symbols and their values.
const SYMBOLS: [(MarkerSymbol, &str); 11] = [
    (MarkerSymbol::None, "none"),
    (MarkerSymbol::Auto, "auto"),
    (MarkerSymbol::Circle, "circle"),
    (MarkerSymbol::Dash, "dash"),
    (MarkerSymbol::Diamond, "diamond"),
    (MarkerSymbol::Dot, "dot"),
    (MarkerSymbol::Plus, "plus"),
    (MarkerSymbol::Square, "square"),
    (MarkerSymbol::Star, "star"),
    (MarkerSymbol::Triangle, "triangle"),
    (MarkerSymbol::X, "x"),
];

#[test]
fn a_line_series_takes_each_marker_symbol_with_and_without_a_size_and_a_colour() {
    for (symbol, value) in SYMBOLS {
        for (size, color, rgb) in [
            (None, None, None),
            (Some(7), None, None),
            (None, Some(0xC0_FFEE), Some("C0FFEE")),
            (Some(72), Some(0x01_0203), Some("010203")),
        ] {
            if symbol == MarkerSymbol::None && (size.is_some() || color.is_some()) {
                continue;
            }
            let marked = [NewSeries {
                marker: Some(marker(symbol, size, color)),
                ..ONE_NAMED[0]
            }];
            let chart = NewChart {
                series: &marked,
                ..bare(LINE)
            };
            let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
            // The plot whole: the marker after the name, the plot's own
            // `marker` after the series.
            let part = text_of(&output, CHART1);
            assert!(
                part.contains(&format!(
                    "<c:lineChart><c:grouping val=\"standard\"/><c:varyColors val=\"0\"/>{}\
                     <c:marker val=\"1\"/><c:axId val=\"10\"/><c:axId val=\"20\"/></c:lineChart>",
                    one_named_with(&marker_written(value, size, rgb)),
                )),
                "{part}"
            );
            let read = &charts_of(&output)[0].chart;
            assert_eq!(
                read.plots,
                vec![Plot {
                    grouping: Some(Grouping::Standard),
                    ..plot_read(
                        ChartKind::Line,
                        false,
                        vec![Series {
                            marker: true,
                            ..one_named_read()
                        }],
                        vec![10, 20]
                    )
                }]
            );
        }
    }
}

#[test]
fn a_none_marker_with_a_size_or_a_colour_and_a_size_off_2_to_72_are_refused() {
    let refused = |marker: NewMarker, message: &str| {
        let marked = [NewSeries {
            marker: Some(marker),
            ..ONE_NAMED[0]
        }];
        let chart = NewChart {
            series: &marked,
            ..bare(LINE)
        };
        assert_package(call(box_anchor(), &chart).expect_err("refused"), message);
        // The same marker on a point.
        let points = [point(0, None, None, Some(marker))];
        let pointed = [NewSeries {
            points: &points,
            ..ONE_NAMED[0]
        }];
        let chart = NewChart {
            series: &pointed,
            ..bare(LINE)
        };
        assert_package(call(box_anchor(), &chart).expect_err("refused"), message);
    };
    refused(marker(MarkerSymbol::None, Some(7), None), OPTION);
    refused(marker(MarkerSymbol::None, None, Some(0)), OPTION);
    for size in [0, 1, 73, 255] {
        refused(marker(MarkerSymbol::Circle, Some(size), None), RANGE);
    }
    refused(marker(MarkerSymbol::Square, None, Some(0x100_0000)), OPTION);
    for size in [2, 72] {
        let marked = [NewSeries {
            marker: Some(marker(MarkerSymbol::Circle, Some(size), Some(0xFF_FFFF))),
            ..ONE_NAMED[0]
        }];
        let chart = NewChart {
            series: &marked,
            ..bare(LINE)
        };
        call(box_anchor(), &chart).expect("taken");
    }
}

#[test]
fn a_line_takes_a_width_a_dash_or_both_with_and_without_the_series_colour() {
    let dashes = [
        (DashStyle::Solid, "solid"),
        (DashStyle::Dot, "dot"),
        (DashStyle::Dash, "dash"),
        (DashStyle::LargeDash, "lgDash"),
        (DashStyle::DashDot, "dashDot"),
        (DashStyle::LargeDashDot, "lgDashDot"),
        (DashStyle::LargeDashDotDot, "lgDashDotDot"),
        (DashStyle::SystemDash, "sysDash"),
        (DashStyle::SystemDot, "sysDot"),
        (DashStyle::SystemDashDot, "sysDashDot"),
        (DashStyle::SystemDashDotDot, "sysDashDotDot"),
    ];
    let mut lines: Vec<(NewLine, String, String)> = vec![
        (
            NewLine {
                width: Some(28575),
                dash: None,
            },
            " w=\"28575\"".to_owned(),
            String::new(),
        ),
        (
            NewLine {
                width: Some(0),
                dash: None,
            },
            " w=\"0\"".to_owned(),
            String::new(),
        ),
        (
            NewLine {
                width: Some(20_116_800),
                dash: Some(DashStyle::LargeDash),
            },
            " w=\"20116800\"".to_owned(),
            "<a:prstDash val=\"lgDash\"/>".to_owned(),
        ),
    ];
    for (dash, value) in dashes {
        lines.push((
            NewLine {
                width: None,
                dash: Some(dash),
            },
            String::new(),
            format!("<a:prstDash val=\"{value}\"/>"),
        ));
    }
    for (line, width, dash) in &lines {
        for color in [None, Some(0x33_6699)] {
            let lined = [NewSeries {
                color,
                line: Some(*line),
                ..ONE_NAMED[0]
            }];
            let chart = NewChart {
                series: &lined,
                ..bare(LINE)
            };
            let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
            // The fill, then the line: its width, the fill again, its dash.
            let fill = color.map_or(String::new(), |_| fill_of("336699"));
            let part = text_of(&output, CHART1);
            assert!(
                part.contains(&one_named_with(&format!(
                    "<c:spPr>{fill}<a:ln{width}>{fill}{dash}</a:ln></c:spPr>"
                ))),
                "{part}"
            );
            assert_eq!(
                charts_of(&output)[0].chart.plots[0].series[0],
                Series {
                    fill: color,
                    ..one_named_read()
                }
            );
        }
    }
    for (line, message) in [
        (
            NewLine {
                width: None,
                dash: None,
            },
            OPTION,
        ),
        (
            NewLine {
                width: Some(20_116_801),
                dash: None,
            },
            RANGE,
        ),
        (
            NewLine {
                width: Some(u32::MAX),
                dash: Some(DashStyle::Dot),
            },
            RANGE,
        ),
    ] {
        let lined = [NewSeries {
            line: Some(line),
            ..ONE_NAMED[0]
        }];
        let chart = NewChart {
            series: &lined,
            ..bare(LINE)
        };
        assert_package(call(box_anchor(), &chart).expect_err("refused"), message);
    }
}

#[test]
fn a_marker_and_a_line_are_taken_by_the_kinds_that_take_them_and_refused_by_the_others() {
    let circle = marker(MarkerSymbol::Circle, Some(7), None);
    let dashed = NewLine {
        width: Some(28575),
        dash: Some(DashStyle::Dash),
    };
    for (plot, series, [takes_marker, takes_line, _]) in every_kind() {
        let marked = each_with(series, |one| NewSeries {
            marker: Some(circle),
            ..one
        });
        let chart = NewChart {
            series: &marked,
            ..bare(plot)
        };
        if takes_marker {
            let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
            let part = text_of(&output, CHART1);
            assert!(
                part.contains(&format!(
                    "{ONE_NAMED_TX}{}<c:",
                    marker_written("circle", Some(7), None)
                )),
                "{plot:?}: {part}"
            );
            let read = &charts_of(&output)[0].chart;
            assert!(
                read.plots[0].series.iter().all(|one| one.marker),
                "{plot:?}"
            );
        } else {
            assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
        }
        let lined = each_with(series, |one| NewSeries {
            line: Some(dashed),
            ..one
        });
        let chart = NewChart {
            series: &lined,
            ..bare(plot)
        };
        if takes_line {
            let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
            let part = text_of(&output, CHART1);
            assert!(
                part.contains(
                    "<c:spPr><a:ln w=\"28575\"><a:prstDash val=\"dash\"/></a:ln></c:spPr>"
                ),
                "{plot:?}: {part}"
            );
            let read = &charts_of(&output)[0].chart;
            assert_eq!(read.plots[0].series.len(), series.len(), "{plot:?}");
        } else {
            assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
        }
    }
    // The stock plot keeps its fixed marker with a line, after the `spPr`.
    let lined = each_with(&STOCK[..3], |one| NewSeries {
        line: Some(dashed),
        ..one
    });
    let chart = NewChart {
        series: &lined,
        ..bare(NewPlot::Stock)
    };
    let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
    let part = text_of(&output, CHART1);
    assert_eq!(
        part.matches(
            "<c:spPr><a:ln w=\"28575\"><a:prstDash val=\"dash\"/></a:ln></c:spPr><c:marker>\
             <c:symbol val=\"none\"/></c:marker>"
        )
        .count(),
        3,
        "{part}"
    );
}

#[test]
fn points_are_written_in_the_order_given_on_each_kind_that_takes_them() {
    let red = Some(0xFF_0000);
    let blue = Some(0x00_00FF);
    let square = marker(MarkerSymbol::Square, None, blue);
    // A bar's points, the later index first.
    let bar_points = [point(2, red, None, None), point(0, blue, None, None)];
    // A line's: a point's marker beside the series' own.
    let line_points = [point(1, red, None, Some(square))];
    // A pie's: an exploded slice and a coloured one.
    let pie_points = [point(0, None, Some(10), None), point(1, red, None, None)];
    let red_fill = format!("<c:spPr>{}</c:spPr>", fill_of("FF0000"));
    let blue_fill = format!("<c:spPr>{}</c:spPr>", fill_of("0000FF"));
    let cases: [(NewPlot, NewSeries<'_>, String); 3] = [
        (
            COLUMN,
            NewSeries {
                points: &bar_points,
                ..ONE_NAMED[0]
            },
            format!(
                "<c:dPt><c:idx val=\"2\"/>{red_fill}</c:dPt><c:dPt><c:idx val=\"0\"/>{blue_fill}\
                 </c:dPt>"
            ),
        ),
        (
            LINE,
            NewSeries {
                marker: Some(marker(MarkerSymbol::Circle, Some(5), None)),
                points: &line_points,
                ..ONE_NAMED[0]
            },
            format!(
                "{}<c:dPt><c:idx val=\"1\"/>{}{red_fill}</c:dPt>",
                marker_written("circle", Some(5), None),
                marker_written("square", None, Some("0000FF")),
            ),
        ),
        (
            NewPlot::Pie,
            NewSeries {
                points: &pie_points,
                ..ONE_NAMED[0]
            },
            format!(
                "<c:dPt><c:idx val=\"0\"/><c:explosion val=\"10\"/></c:dPt><c:dPt><c:idx \
                 val=\"1\"/>{red_fill}</c:dPt>"
            ),
        ),
    ];
    for (plot, series, written) in cases {
        let one = [series];
        let chart = NewChart {
            series: &one,
            ..bare(plot)
        };
        let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
        let part = text_of(&output, CHART1);
        assert!(part.contains(&one_named_with(&written)), "{plot:?}: {part}");
        // The reader walks past the points: the series reads as without
        // them, its marker the series' own.
        assert_eq!(
            charts_of(&output)[0].chart.plots[0].series,
            vec![Series {
                marker: series.marker.is_some(),
                ..one_named_read()
            }],
            "{plot:?}"
        );
    }
    // A scatter's point after its `spPr`; a stock's after its fixed marker,
    // the point coloured where the series' colour is refused; a bubble's
    // before its references on each series, the first's `xVal`, the
    // second's `yVal` (it has no categories).
    let one_red = [point(1, red, None, None)];
    let point_written = format!("<c:dPt><c:idx val=\"1\"/>{red_fill}</c:dPt>");
    let scatter = [NewSeries {
        color: blue,
        points: &one_red,
        ..ONE_NAMED[0]
    }];
    let stock = each_with(&STOCK[..3], |one| NewSeries {
        points: &one_red,
        ..one
    });
    let bubbles = each_with(&BUBBLES, |one| NewSeries {
        points: &one_red,
        ..one
    });
    let blue_line = format!(
        "<c:spPr>{}<a:ln>{}</a:ln></c:spPr>",
        fill_of("0000FF"),
        fill_of("0000FF")
    );
    // Each case's patterns, with the count each is written.
    type Expected = Vec<(String, usize)>;
    let cases: [(NewPlot, &[NewSeries<'_>], Expected); 3] = [
        (
            NewPlot::Scatter {
                style: ScatterStyle::Marker,
            },
            &scatter,
            vec![(
                format!("{ONE_NAMED_TX}{blue_line}{point_written}<c:xVal>"),
                1,
            )],
        ),
        (
            NewPlot::Stock,
            &stock,
            vec![(
                format!("<c:marker><c:symbol val=\"none\"/></c:marker>{point_written}<c:cat>"),
                3,
            )],
        ),
        (
            NewPlot::Bubble,
            &bubbles,
            vec![
                (format!("{point_written}<c:xVal>"), 1),
                (format!("{point_written}<c:yVal>"), 1),
            ],
        ),
    ];
    for (plot, series, expected) in cases {
        let chart = NewChart {
            series,
            ..bare(plot)
        };
        let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
        let part = text_of(&output, CHART1);
        for (written, count) in &expected {
            assert_eq!(part.matches(written).count(), *count, "{plot:?}: {part}");
        }
        let read = &charts_of(&output)[0].chart.plots[0];
        assert_eq!(read.series.len(), series.len(), "{plot:?}");
    }
    // An explosion on each round kind, a colour on every kind that takes
    // points.
    let exploded = [point(0, None, Some(25), None)];
    let coloured = [point(0, red, None, None)];
    for (plot, series, [_, _, round]) in every_kind() {
        let surface = matches!(plot, NewPlot::Surface { .. } | NewPlot::Surface3D { .. });
        if surface {
            continue;
        }
        let with_colour = each_with(series, |one| NewSeries {
            points: &coloured,
            ..one
        });
        let chart = NewChart {
            series: &with_colour,
            ..bare(plot)
        };
        call(box_anchor(), &chart).expect("taken");
        let with_explosion = each_with(series, |one| NewSeries {
            points: &exploded,
            ..one
        });
        let chart = NewChart {
            series: &with_explosion,
            ..bare(plot)
        };
        if round {
            let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
            let part = text_of(&output, CHART1);
            assert!(
                part.contains("<c:dPt><c:idx val=\"0\"/><c:explosion val=\"25\"/></c:dPt>"),
                "{plot:?}: {part}"
            );
        } else {
            assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
        }
    }
}

#[test]
fn a_bare_point_a_repeated_index_and_a_point_option_off_its_kind_are_refused() {
    let red = Some(0xFF_0000);
    let square = marker(MarkerSymbol::Square, None, None);
    let cases: [(NewPlot, &[NewPoint]); 7] = [
        (LINE, &[point(0, None, None, None)]),
        (
            LINE,
            &[
                point(1, red, None, None),
                point(1, None, None, Some(square)),
            ],
        ),
        (COLUMN, &[point(0, red, Some(10), None)]),
        (COLUMN, &[point(0, None, None, Some(square))]),
        (
            NewPlot::OfPie {
                kind: OfPieKind::Bar,
                split: None,
            },
            &[point(0, None, None, Some(square))],
        ),
        (COLUMN, &[point(0, Some(0x100_0000), None, None)]),
        (
            NewPlot::Surface { wireframe: false },
            &[point(0, red, None, None)],
        ),
    ];
    for (plot, points) in cases {
        let pointed = [NewSeries {
            points,
            ..ONE_NAMED[0]
        }];
        let chart = NewChart {
            series: &pointed,
            ..bare(plot)
        };
        assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
    }
    // Two indexes, each a field, are taken.
    let two = [
        point(0, red, None, None),
        point(1, None, None, Some(square)),
    ];
    let pointed = [NewSeries {
        points: &two,
        ..ONE_NAMED[0]
    }];
    let chart = NewChart {
        series: &pointed,
        ..bare(LINE)
    };
    call(box_anchor(), &chart).expect("taken");
}

#[test]
fn a_surface_series_with_a_marker_a_line_or_a_point_is_refused() {
    // P1 verification ruling E (a): the fields arrive in P2.
    let points = [point(0, Some(0xFF_0000), None, None)];
    for plot in [
        NewPlot::Surface { wireframe: true },
        NewPlot::Surface3D { wireframe: false },
    ] {
        for series in [
            NewSeries {
                marker: Some(marker(MarkerSymbol::Circle, None, None)),
                ..THREE_NAMED[1]
            },
            NewSeries {
                line: Some(NewLine {
                    width: Some(12700),
                    dash: None,
                }),
                ..THREE_NAMED[1]
            },
            NewSeries {
                points: &points,
                ..THREE_NAMED[1]
            },
        ] {
            let three = [THREE_NAMED[0], series, THREE_NAMED[2]];
            let chart = titled(plot, &three, (None, None));
            assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
        }
    }
}

#[test]
fn a_combined_second_plot_s_series_take_markers_lines_and_points_by_its_own_kind() {
    let points = [point(2, Some(0x00_8000), None, None)];
    let formatted = [NewSeries {
        marker: Some(marker(MarkerSymbol::Diamond, Some(9), None)),
        line: Some(NewLine {
            width: None,
            dash: Some(DashStyle::SystemDot),
        }),
        points: &points,
        ..OVER_C[0]
    }];
    let chart = combined(NewCombined {
        series: &formatted,
        ..second(LINE)
    });
    let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
    let part = text_of(&output, CHART1);
    assert!(
        part.contains(&format!(
            "<c:tx><c:v>Share</c:v></c:tx><c:spPr><a:ln><a:prstDash val=\"sysDot\"/></a:ln>\
             </c:spPr>{}<c:dPt><c:idx val=\"2\"/><c:spPr>{}</c:spPr></c:dPt><c:cat>",
            marker_written("diamond", Some(9), None),
            fill_of("008000"),
        )),
        "{part}"
    );
    let read = &charts_of(&output)[0].chart;
    assert!(read.plots[1].series[0].marker);
    assert!(!read.plots[0].series[0].marker);
    // A scatter second on the secondary axes takes a marker; an area second
    // takes none, and the bar first none.
    let marked = [NewSeries {
        marker: Some(marker(MarkerSymbol::X, None, None)),
        ..OVER_C[0]
    }];
    let scatter = combined(NewCombined {
        series: &marked,
        secondary_axis: true,
        ..second(NewPlot::Scatter {
            style: ScatterStyle::Marker,
        })
    });
    call(box_anchor(), &scatter).expect("taken");
    let area = combined(NewCombined {
        series: &marked,
        ..second(NewPlot::Area {
            grouping: Grouping::Standard,
        })
    });
    assert_package(call(box_anchor(), &area).expect_err("refused"), OPTION);
    let marked_first = each_with(&TWO_SERIES, |one| NewSeries {
        marker: Some(marker(MarkerSymbol::X, None, None)),
        ..one
    });
    let bar_first = NewChart {
        series: &marked_first,
        ..combined(second(LINE))
    };
    assert_package(call(box_anchor(), &bar_first).expect_err("refused"), OPTION);
}

#[test]
fn the_series_options_follow_the_check_order() {
    let small = marker(MarkerSymbol::Circle, Some(1), None);
    let neither = NewLine {
        width: None,
        dash: None,
    };
    let refused = |plot: NewPlot, series: &[NewSeries<'_>], message: &str| {
        let chart = NewChart {
            series,
            ..bare(plot)
        };
        assert_package(call(box_anchor(), &chart).expect_err("refused"), message);
    };
    // The kind rules first: a pie of two series with a bad marker size.
    let two_small = each_with(&TWO_NAMED, |one| NewSeries {
        marker: Some(small),
        ..one
    });
    refused(NewPlot::Pie, &two_small, OPTION);
    // The marker before the line, the line before the points.
    let marker_then_line = [NewSeries {
        marker: Some(small),
        line: Some(neither),
        ..ONE_NAMED[0]
    }];
    refused(LINE, &marker_then_line, RANGE);
    let bare_point = [point(0, None, None, None)];
    let line_then_points = [NewSeries {
        line: Some(NewLine {
            width: Some(20_116_801),
            dash: None,
        }),
        points: &bare_point,
        ..ONE_NAMED[0]
    }];
    refused(LINE, &line_then_points, RANGE);
    // The marker's symbol, then its size, then its colour.
    let none_sized = [NewSeries {
        marker: Some(marker(MarkerSymbol::None, Some(100), None)),
        ..ONE_NAMED[0]
    }];
    refused(LINE, &none_sized, OPTION);
    let small_bright = [NewSeries {
        marker: Some(marker(MarkerSymbol::Circle, Some(1), Some(0x100_0000))),
        ..ONE_NAMED[0]
    }];
    refused(LINE, &small_bright, RANGE);
    // Series by series, and point by point.
    let first_neither = [
        NewSeries {
            line: Some(neither),
            ..TWO_NAMED[0]
        },
        NewSeries {
            marker: Some(small),
            ..TWO_NAMED[1]
        },
    ];
    refused(LINE, &first_neither, OPTION);
    let points = [
        point(0, None, None, None),
        point(1, None, None, Some(small)),
    ];
    let bare_first = [NewSeries {
        points: &points,
        ..ONE_NAMED[0]
    }];
    refused(LINE, &bare_first, OPTION);
    let points = [
        point(0, None, None, Some(small)),
        point(0, None, None, None),
    ];
    let small_first = [NewSeries {
        points: &points,
        ..ONE_NAMED[0]
    }];
    refused(LINE, &small_first, RANGE);
    // A point's explosion before its marker.
    let points = [point(0, None, Some(5), Some(small))];
    let exploded = [NewSeries {
        points: &points,
        ..ONE_NAMED[0]
    }];
    refused(LINE, &exploded, OPTION);
    // After an of-pie's split; before the combined plot's rules and the
    // texts.
    let pie_point = [NewSeries {
        points: &bare_point,
        ..ONE_NAMED[0]
    }];
    refused(
        NewPlot::OfPie {
            kind: OfPieKind::Pie,
            split: Some(OfPieSplit::Position(0)),
        },
        &pie_point,
        RANGE,
    );
    let small_line = [NewSeries {
        marker: Some(small),
        ..ONE_NAMED[0]
    }];
    let same_variant = NewChart {
        series: &small_line,
        combined: Some(second(LINE)),
        ..bare(LINE)
    };
    assert_package(
        call(box_anchor(), &same_variant).expect_err("refused"),
        RANGE,
    );
    let empty_title = NewChart {
        series: &small_line,
        title: Some(""),
        ..bare(LINE)
    };
    assert_package(
        call(box_anchor(), &empty_title).expect_err("refused"),
        RANGE,
    );
    // The second plot's series options after its own kind rules: a second
    // of the first's variant with a bad marker gives the rule's message.
    let small_c = [NewSeries {
        marker: Some(small),
        ..OVER_C[0]
    }];
    let column_second = combined(NewCombined {
        series: &small_c,
        ..second(COLUMN)
    });
    assert_package(
        call(box_anchor(), &column_second).expect_err("refused"),
        OPTION,
    );
    let line_second = combined(NewCombined {
        series: &small_c,
        ..second(LINE)
    });
    assert_package(
        call(box_anchor(), &line_second).expect_err("refused"),
        RANGE,
    );
}

/// A trendline of `kind` with nothing else given.
fn trend(kind: TrendlineKind) -> NewTrendline<'static> {
    NewTrendline {
        kind,
        forward: None,
        backward: None,
        intercept: None,
        show_equation: false,
        show_r_squared: false,
        name: None,
    }
}

/// A set of error bars: `direction`, `kind`, `value`, with end caps.
fn bars(
    direction: Option<ErrorDirection>,
    kind: ErrorBarKind,
    value: ErrorValue<'static>,
) -> NewErrorBars<'static> {
    NewErrorBars {
        direction,
        kind,
        value,
        end_cap: true,
    }
}

/// A trendline's form (`specs/0040-analytic-chart-elements.md`, "The chart
/// part"): `inner` between the type and `dispRSqr`, the two flags.
fn trend_written(name: &str, kind: &str, inner: &str, flags: [u8; 2]) -> String {
    let name = if name.is_empty() {
        String::new()
    } else {
        format!("<c:name>{name}</c:name>")
    };
    format!(
        "<c:trendline>{name}<c:trendlineType val=\"{kind}\"/>{inner}<c:dispRSqr val=\"{}\"/>\
         <c:dispEq val=\"{}\"/></c:trendline>",
        flags[0], flags[1]
    )
}

/// An error bars' form: the direction when not empty, the kind, the
/// value's type, `noEndCap`, then `tail` (the references or the `val`).
fn bars_written(direction: &str, kind: &str, value: &str, no_end_cap: u8, tail: &str) -> String {
    let direction = if direction.is_empty() {
        String::new()
    } else {
        format!("<c:errDir val=\"{direction}\"/>")
    };
    format!(
        "<c:errBars>{direction}<c:errBarType val=\"{kind}\"/><c:errValType val=\"{value}\"/>\
         <c:noEndCap val=\"{no_end_cap}\"/>{tail}</c:errBars>"
    )
}

/// The linear trendline and the fixed bars of 1 every kind that takes them
/// is given, as written, on a scatter or a bubble with the `y` direction.
fn analysis_written(xy: bool) -> String {
    format!(
        "{}{}",
        trend_written("", "linear", "", [0, 0]),
        bars_written(
            if xy { "y" } else { "" },
            "both",
            "fixedVal",
            0,
            "<c:val val=\"1\"/>"
        )
    )
}

/// `ONE_NAMED`'s series on a scatter as written with `options` between the
/// name and the references.
fn one_named_xy_with(options: &str) -> String {
    format!(
        "<c:ser><c:idx val=\"0\"/><c:order val=\"0\"/>{ONE_NAMED_TX}{options}<c:xVal>{}</c:xVal>\
         <c:yVal>{}</c:yVal></c:ser>",
        reference("numRef", "Sheet1!$A$2:$A$4"),
        reference("numRef", "Sheet1!$B$2:$B$4"),
    )
}

/// The marker scatter plot.
const SCATTER: NewPlot = NewPlot::Scatter {
    style: ScatterStyle::LineMarker,
};

#[test]
fn a_trendline_and_error_bars_are_taken_by_the_kinds_that_take_them_and_refused_by_the_others() {
    for (plot, series, _) in every_kind() {
        let xy = matches!(plot, NewPlot::Scatter { .. } | NewPlot::Bubble);
        let takes = xy
            || matches!(
                plot,
                NewPlot::Bar { .. } | NewPlot::Line { .. } | NewPlot::Area { .. }
            );
        let fixed = [bars(
            xy.then_some(ErrorDirection::Y),
            ErrorBarKind::Both,
            ErrorValue::Fixed(1.0),
        )];
        let analysed = each_with(series, |one| NewSeries {
            trendline: Some(trend(TrendlineKind::Linear)),
            error_bars: &fixed,
            ..one
        });
        let chart = NewChart {
            series: &analysed,
            ..bare(plot)
        };
        if takes {
            let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
            let part = text_of(&output, CHART1);
            // Each series' trendline then its bars, after the name and
            // before the first reference.
            let (x, y) = if xy { ("xVal", "yVal") } else { ("cat", "val") };
            let written = analysis_written(xy);
            assert_eq!(
                part.matches(&written).count(),
                series.len(),
                "{plot:?}: {part}"
            );
            for first in [x, y] {
                assert_eq!(
                    part.matches(&format!("{written}<c:{first}>")).count(),
                    series
                        .iter()
                        .filter(|one| one.categories.is_some() == (first == x))
                        .count(),
                    "{plot:?}, the bars before {first}: {part}"
                );
            }
            let read = &charts_of(&output)[0].chart;
            assert_eq!(read.plots[0].series.len(), series.len(), "{plot:?}");
        } else {
            assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
            let trended = each_with(series, |one| NewSeries {
                trendline: Some(trend(TrendlineKind::Linear)),
                ..one
            });
            let chart = NewChart {
                series: &trended,
                ..bare(plot)
            };
            assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
            let barred = each_with(series, |one| NewSeries {
                error_bars: &fixed,
                ..one
            });
            let chart = NewChart {
                series: &barred,
                ..bare(plot)
            };
            assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
        }
    }
}

#[test]
fn each_trendline_kind_writes_its_type_its_order_or_period_and_its_fields() {
    let all_fields = NewTrendline {
        forward: Some(2.0),
        backward: Some(0.5),
        intercept: Some(-1.25),
        show_equation: true,
        show_r_squared: true,
        name: Some("Fit & <trend>"),
        ..trend(TrendlineKind::Linear)
    };
    let cases = [
        (
            trend(TrendlineKind::Linear),
            trend_written("", "linear", "", [0, 0]),
        ),
        (
            all_fields,
            trend_written(
                "Fit &amp; &lt;trend&gt;",
                "linear",
                "<c:forward val=\"2\"/><c:backward val=\"0.5\"/><c:intercept val=\"-1.25\"/>",
                [1, 1],
            ),
        ),
        (
            NewTrendline {
                intercept: Some(0.0),
                show_r_squared: true,
                ..trend(TrendlineKind::Exponential)
            },
            trend_written("", "exp", "<c:intercept val=\"0\"/>", [1, 0]),
        ),
        (
            NewTrendline {
                forward: Some(0.0),
                ..trend(TrendlineKind::Logarithmic)
            },
            trend_written("", "log", "<c:forward val=\"0\"/>", [0, 0]),
        ),
        (
            NewTrendline {
                backward: Some(3.0),
                show_equation: true,
                ..trend(TrendlineKind::Power)
            },
            trend_written("", "power", "<c:backward val=\"3\"/>", [0, 1]),
        ),
        (
            trend(TrendlineKind::Polynomial(2)),
            trend_written("", "poly", "<c:order val=\"2\"/>", [0, 0]),
        ),
        (
            NewTrendline {
                forward: Some(2.0),
                intercept: Some(1.5),
                name: Some("Cubic"),
                ..trend(TrendlineKind::Polynomial(6))
            },
            trend_written(
                "Cubic",
                "poly",
                "<c:order val=\"6\"/><c:forward val=\"2\"/><c:intercept val=\"1.5\"/>",
                [0, 0],
            ),
        ),
        (
            trend(TrendlineKind::MovingAverage(2)),
            trend_written("", "movingAvg", "<c:period val=\"2\"/>", [0, 0]),
        ),
        (
            NewTrendline {
                name: Some("Average"),
                ..trend(TrendlineKind::MovingAverage(u16::MAX))
            },
            trend_written("Average", "movingAvg", "<c:period val=\"65535\"/>", [0, 0]),
        ),
    ];
    for (trendline, written) in cases {
        let trended = [NewSeries {
            trendline: Some(trendline),
            ..ONE_NAMED[0]
        }];
        let chart = NewChart {
            series: &trended,
            ..bare(LINE)
        };
        let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
        let part = text_of(&output, CHART1);
        assert!(
            part.contains(&one_named_with(&written)),
            "{trendline:?}: {part}"
        );
        // The reader walks past the trendline.
        assert_eq!(
            charts_of(&output)[0].chart.plots[0].series[0],
            one_named_read()
        );
    }
}

#[test]
fn each_error_value_writes_its_type_and_its_length_or_its_references() {
    let custom = ErrorValue::Custom {
        plus: "Sheet1!$C$2:$C$4",
        minus: "'A&B'!$D$2:$D$4",
    };
    let values = [
        (
            ErrorValue::Fixed(1.5),
            "fixedVal",
            "<c:val val=\"1.5\"/>".to_owned(),
        ),
        (
            ErrorValue::Percentage(5.0),
            "percentage",
            "<c:val val=\"5\"/>".to_owned(),
        ),
        (
            ErrorValue::StandardDeviation(0.25),
            "stdDev",
            "<c:val val=\"0.25\"/>".to_owned(),
        ),
        (ErrorValue::StandardError, "stdErr", String::new()),
        (
            custom,
            "cust",
            format!(
                "<c:plus>{}</c:plus><c:minus>{}</c:minus>",
                reference("numRef", "Sheet1!$C$2:$C$4"),
                reference("numRef", "'A&amp;B'!$D$2:$D$4"),
            ),
        ),
    ];
    let kinds = [
        (ErrorBarKind::Both, "both"),
        (ErrorBarKind::Plus, "plus"),
        (ErrorBarKind::Minus, "minus"),
    ];
    for (value, value_type, tail) in &values {
        for (kind, kind_value) in kinds {
            for end_cap in [true, false] {
                let set = [NewErrorBars {
                    end_cap,
                    ..bars(None, kind, *value)
                }];
                let barred = [NewSeries {
                    error_bars: &set,
                    ..ONE_NAMED[0]
                }];
                let chart = NewChart {
                    series: &barred,
                    ..bare(COLUMN)
                };
                let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
                let part = text_of(&output, CHART1);
                let written = bars_written("", kind_value, value_type, u8::from(!end_cap), tail);
                assert!(
                    part.contains(&one_named_with(&written)),
                    "{value:?}: {part}"
                );
                assert_eq!(
                    charts_of(&output)[0].chart.plots[0].series[0],
                    one_named_read()
                );
            }
        }
    }
}

#[test]
fn a_scatter_and_a_bubble_take_two_sets_of_error_bars_in_the_order_given() {
    let two = [
        bars(
            Some(ErrorDirection::Y),
            ErrorBarKind::Both,
            ErrorValue::StandardDeviation(1.0),
        ),
        bars(
            Some(ErrorDirection::X),
            ErrorBarKind::Plus,
            ErrorValue::Custom {
                plus: "Sheet1!$C$2:$C$4",
                minus: "Sheet1!$D$2:$D$4",
            },
        ),
    ];
    let written = format!(
        "{}{}{}",
        trend_written("", "exp", "", [0, 0]),
        bars_written("y", "both", "stdDev", 0, "<c:val val=\"1\"/>"),
        bars_written(
            "x",
            "plus",
            "cust",
            0,
            &format!(
                "<c:plus>{}</c:plus><c:minus>{}</c:minus>",
                reference("numRef", "Sheet1!$C$2:$C$4"),
                reference("numRef", "Sheet1!$D$2:$D$4"),
            )
        ),
    );
    let scattered = [NewSeries {
        trendline: Some(trend(TrendlineKind::Exponential)),
        error_bars: &two,
        ..ONE_NAMED[0]
    }];
    let chart = NewChart {
        series: &scattered,
        ..bare(SCATTER)
    };
    let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
    let part = text_of(&output, CHART1);
    assert!(part.contains(&one_named_xy_with(&written)), "{part}");
    assert_eq!(
        charts_of(&output)[0].chart.plots[0].series[0],
        one_named_read()
    );
    // The x set first on a bubble's two series: the sets before each
    // series' first reference, the sizes last.
    let reversed = [two[1], two[0]];
    let bubbles = each_with(&BUBBLES, |one| NewSeries {
        error_bars: &reversed,
        ..one
    });
    let chart = NewChart {
        series: &bubbles,
        ..bare(NewPlot::Bubble)
    };
    let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
    let part = text_of(&output, CHART1);
    let sets = format!(
        "{}{}",
        bars_written(
            "x",
            "plus",
            "cust",
            0,
            &format!(
                "<c:plus>{}</c:plus><c:minus>{}</c:minus>",
                reference("numRef", "Sheet1!$C$2:$C$4"),
                reference("numRef", "Sheet1!$D$2:$D$4"),
            )
        ),
        bars_written("y", "both", "stdDev", 0, "<c:val val=\"1\"/>"),
    );
    assert_eq!(
        part.matches(&format!("{sets}<c:xVal>")).count(),
        1,
        "{part}"
    );
    assert_eq!(
        part.matches(&format!("{sets}<c:yVal>")).count(),
        1,
        "{part}"
    );
    assert_eq!(charts_of(&output)[0].chart.plots[0].series.len(), 2);
}

#[test]
fn the_trendline_and_error_bar_field_rules_are_refused() {
    let refused = |plot: NewPlot, series: NewSeries<'_>, message: &str| {
        let one = [series];
        let chart = NewChart {
            series: &one,
            ..bare(plot)
        };
        assert_package(call(box_anchor(), &chart).expect_err("refused"), message);
    };
    let trended = |trendline: NewTrendline<'static>| NewSeries {
        trendline: Some(trendline),
        ..ONE_NAMED[0]
    };
    let average = trend(TrendlineKind::MovingAverage(3));
    // A moving average takes no forecast, intercept, equation or R-squared.
    for trendline in [
        NewTrendline {
            forward: Some(1.0),
            ..average
        },
        NewTrendline {
            backward: Some(1.0),
            ..average
        },
        NewTrendline {
            intercept: Some(1.0),
            ..average
        },
        NewTrendline {
            show_equation: true,
            ..average
        },
        NewTrendline {
            show_r_squared: true,
            ..average
        },
    ] {
        refused(LINE, trended(trendline), OPTION);
    }
    // An intercept on a logarithmic or power trendline.
    for kind in [TrendlineKind::Logarithmic, TrendlineKind::Power] {
        let trendline = NewTrendline {
            intercept: Some(0.0),
            ..trend(kind)
        };
        refused(LINE, trended(trendline), OPTION);
    }
    // The ranges: the order, the period, the forecasts, the intercept.
    for kind in [
        TrendlineKind::Polynomial(0),
        TrendlineKind::Polynomial(1),
        TrendlineKind::Polynomial(7),
        TrendlineKind::Polynomial(u8::MAX),
        TrendlineKind::MovingAverage(0),
        TrendlineKind::MovingAverage(1),
    ] {
        refused(LINE, trended(trend(kind)), RANGE);
    }
    for number in [-0.5, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let linear = trend(TrendlineKind::Linear);
        refused(
            LINE,
            trended(NewTrendline {
                forward: Some(number),
                ..linear
            }),
            RANGE,
        );
        refused(
            LINE,
            trended(NewTrendline {
                backward: Some(number),
                ..linear
            }),
            RANGE,
        );
    }
    for number in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let trendline = NewTrendline {
            intercept: Some(number),
            ..trend(TrendlineKind::Linear)
        };
        refused(LINE, trended(trendline), RANGE);
    }
    // A length that is not finite or not positive.
    for number in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        for value in [
            ErrorValue::Fixed(number),
            ErrorValue::Percentage(number),
            ErrorValue::StandardDeviation(number),
        ] {
            let set = [bars(None, ErrorBarKind::Both, value)];
            refused(
                LINE,
                NewSeries {
                    error_bars: &set,
                    ..ONE_NAMED[0]
                },
                RANGE,
            );
        }
    }
    // The direction: required on a scatter, refused on a line.
    let fixed = ErrorValue::Fixed(1.0);
    let no_direction = [bars(None, ErrorBarKind::Both, fixed)];
    refused(
        SCATTER,
        NewSeries {
            error_bars: &no_direction,
            ..ONE_NAMED[0]
        },
        OPTION,
    );
    let y = [bars(Some(ErrorDirection::Y), ErrorBarKind::Both, fixed)];
    refused(
        LINE,
        NewSeries {
            error_bars: &y,
            ..ONE_NAMED[0]
        },
        OPTION,
    );
    // Two sets on a line; two of one direction or three on a scatter.
    let two_lines = [no_direction[0], no_direction[0]];
    refused(
        LINE,
        NewSeries {
            error_bars: &two_lines,
            ..ONE_NAMED[0]
        },
        OPTION,
    );
    let two_y = [y[0], y[0]];
    refused(
        SCATTER,
        NewSeries {
            error_bars: &two_y,
            ..ONE_NAMED[0]
        },
        OPTION,
    );
    let x = bars(Some(ErrorDirection::X), ErrorBarKind::Both, fixed);
    let three = [y[0], x, y[0]];
    refused(
        SCATTER,
        NewSeries {
            error_bars: &three,
            ..ONE_NAMED[0]
        },
        OPTION,
    );
    // The texts: a trendline's name and a custom reference.
    for name in ["", "a\u{1}b"] {
        let trendline = NewTrendline {
            name: Some(name),
            ..trend(TrendlineKind::Linear)
        };
        refused(LINE, trended(trendline), TEXT);
    }
    for (plus, minus) in [("", "Sheet1!$C$2:$C$4"), ("Sheet1!$C$2:$C$4", "a\nb")] {
        let set = [bars(
            None,
            ErrorBarKind::Both,
            ErrorValue::Custom { plus, minus },
        )];
        refused(
            LINE,
            NewSeries {
                error_bars: &set,
                ..ONE_NAMED[0]
            },
            TEXT,
        );
    }
}

#[test]
fn the_trendline_and_error_bars_follow_the_check_order() {
    let refused = |plot: NewPlot, series: &[NewSeries<'_>], message: &str| {
        let chart = NewChart {
            series,
            ..bare(plot)
        };
        assert_package(call(box_anchor(), &chart).expect_err("refused"), message);
    };
    let bad_order = Some(trend(TrendlineKind::Polynomial(9)));
    let fixed = ErrorValue::Fixed(1.0);
    let negative = [bars(None, ErrorBarKind::Both, ErrorValue::Fixed(-1.0))];
    let two = [bars(None, ErrorBarKind::Both, fixed); 2];
    // After the marker, the line and the points.
    let small = marker(MarkerSymbol::Circle, Some(1), None);
    let marker_first = [NewSeries {
        marker: Some(small),
        trendline: Some(NewTrendline {
            show_equation: true,
            ..trend(TrendlineKind::MovingAverage(2))
        }),
        ..ONE_NAMED[0]
    }];
    refused(LINE, &marker_first, RANGE);
    let bare_point = [point(0, None, None, None)];
    let points_first = [NewSeries {
        points: &bare_point,
        trendline: bad_order,
        ..ONE_NAMED[0]
    }];
    refused(LINE, &points_first, OPTION);
    // The trendline before the error bars.
    let trend_first = [NewSeries {
        trendline: bad_order,
        error_bars: &two,
        ..ONE_NAMED[0]
    }];
    refused(LINE, &trend_first, RANGE);
    let average_first = [NewSeries {
        trendline: Some(NewTrendline {
            forward: Some(1.0),
            ..trend(TrendlineKind::MovingAverage(2))
        }),
        error_bars: &negative,
        ..ONE_NAMED[0]
    }];
    refused(LINE, &average_first, OPTION);
    // The order or period before the forecasts; the forecast before the
    // intercept; the intercept before the flags.
    let order_first = [NewSeries {
        trendline: Some(NewTrendline {
            forward: Some(1.0),
            ..trend(TrendlineKind::MovingAverage(1))
        }),
        ..ONE_NAMED[0]
    }];
    refused(LINE, &order_first, RANGE);
    let forecast_first = [NewSeries {
        trendline: Some(NewTrendline {
            forward: Some(-1.0),
            intercept: Some(1.0),
            ..trend(TrendlineKind::Power)
        }),
        ..ONE_NAMED[0]
    }];
    refused(LINE, &forecast_first, RANGE);
    // The number of sets before a set's value.
    let two_negative = [negative[0], negative[0]];
    let count_first = [NewSeries {
        error_bars: &two_negative,
        ..ONE_NAMED[0]
    }];
    refused(LINE, &count_first, OPTION);
    // Series by series.
    let first_then_second = [
        NewSeries {
            error_bars: &negative,
            ..TWO_NAMED[0]
        },
        NewSeries {
            error_bars: &two,
            ..TWO_NAMED[1]
        },
    ];
    refused(LINE, &first_then_second, RANGE);
    // Before the combined plot's rules and the texts.
    let ranged = [NewSeries {
        trendline: bad_order,
        ..ONE_NAMED[0]
    }];
    let same_variant = NewChart {
        series: &ranged,
        combined: Some(second(LINE)),
        ..bare(LINE)
    };
    assert_package(
        call(box_anchor(), &same_variant).expect_err("refused"),
        RANGE,
    );
    let empty_title = NewChart {
        series: &ranged,
        title: Some(""),
        ..bare(LINE)
    };
    assert_package(
        call(box_anchor(), &empty_title).expect_err("refused"),
        RANGE,
    );
}

#[test]
fn a_combined_second_plot_s_series_take_a_trendline_and_error_bars_by_its_own_kind() {
    let fixed = [bars(None, ErrorBarKind::Plus, ErrorValue::Percentage(10.0))];
    let analysed = [NewSeries {
        trendline: Some(trend(TrendlineKind::Linear)),
        error_bars: &fixed,
        ..OVER_C[0]
    }];
    let written = format!(
        "{}{}",
        trend_written("", "linear", "", [0, 0]),
        bars_written("", "plus", "percentage", 0, "<c:val val=\"10\"/>")
    );
    // A line second over a column first takes them, written on the second
    // plot's series before its categories.
    let chart = combined(NewCombined {
        series: &analysed,
        ..second(LINE)
    });
    let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
    let part = text_of(&output, CHART1);
    assert!(
        part.contains(&format!("<c:tx><c:v>Share</c:v></c:tx>{written}<c:cat>")),
        "{part}"
    );
    assert_eq!(charts_of(&output)[0].chart.plots.len(), 2);
    // A scatter second on the secondary axes needs the direction.
    let chart = combined(NewCombined {
        series: &analysed,
        secondary_axis: true,
        ..second(SCATTER)
    });
    assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
    let y = [bars(
        Some(ErrorDirection::Y),
        ErrorBarKind::Plus,
        ErrorValue::Percentage(10.0),
    )];
    let directed = [NewSeries {
        error_bars: &y,
        ..analysed[0]
    }];
    let chart = combined(NewCombined {
        series: &directed,
        secondary_axis: true,
        ..second(SCATTER)
    });
    let output = edited_output(smallest(), |p| add(p, box_anchor(), &chart));
    let part = text_of(&output, CHART1);
    let written = format!(
        "{}{}",
        trend_written("", "linear", "", [0, 0]),
        bars_written("y", "plus", "percentage", 0, "<c:val val=\"10\"/>")
    );
    assert!(
        part.contains(&format!("<c:tx><c:v>Share</c:v></c:tx>{written}<c:xVal>")),
        "{part}"
    );
    // A 3D column first refuses a combined plot; its own series refuse a
    // trendline.
    let column_3d = NewPlot::Bar3D {
        direction: BarDirection::Column,
        grouping: Grouping::Clustered,
    };
    let chart = NewChart {
        series: &analysed,
        ..bare(column_3d)
    };
    assert_package(call(box_anchor(), &chart).expect_err("refused"), OPTION);
}
