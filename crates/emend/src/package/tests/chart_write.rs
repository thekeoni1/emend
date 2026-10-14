//! Charts written on byte-built packages (`specs/0035-charts-write-first-
//! set.md`, "P1: the writer, bar and line charts", P1 rulings A to F, and
//! "P2: pie, scatter and area charts; the catalogue"; `specs/0036-charts-
//! write-second-set.md`, "P1: the compaction, doughnut, radar, bubble and
//! stock charts"):
//! each test builds the smallest package, records `add_chart` calls on its
//! worksheet, writes, compares the parts the design gives byte for byte
//! where it names them, and reads every chart written back through
//! `charts`, field by field. Every content is fictional. The Cargo feature
//! `charts`; the tests that also add or remove an image, `images` too.

use super::*;
use crate::package::{
    Anchor, AxisKind, AxisPosition, BarDirection, Chart, ChartAnchor, ChartAxis, ChartEntry,
    ChartKind, Data, EditAs, Grouping, ImageAnchor, LegendPosition, NewChart, NewPlot, NewSeries,
    NewSeriesName, Plot, RadarStyle, ScatterStyle, Series, SeriesName,
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
    },
    NewSeries {
        name: Some(NewSeriesName::Reference("Sheet1!$C$1")),
        categories: None,
        values: "Sheet1!$C$2:$C$4",
        sizes: None,
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
        legend: Some(LegendPosition::Bottom),
        size: None,
    }
}

/// A chart with one series and nothing optional, of `plot`.
fn bare(plot: NewPlot) -> NewChart<'static> {
    const ONE: [NewSeries<'static>; 1] = [NewSeries {
        name: None,
        categories: None,
        values: "Sheet1!$B$2:$B$4",
        sizes: None,
    }];
    NewChart {
        plot,
        title: None,
        series: &ONE,
        category_axis_title: None,
        value_axis_title: None,
        legend: None,
        size: None,
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
        legend: Some(LegendPosition::Right),
        size: None,
    }
}

/// One series with a reference name, categories and values.
const ONE_NAMED: [NewSeries<'static>; 1] = [NewSeries {
    name: Some(NewSeriesName::Reference("Sheet1!$B$1")),
    categories: Some("Sheet1!$A$2:$A$4"),
    values: "Sheet1!$B$2:$B$4",
    sizes: None,
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
    },
    NewSeries {
        name: Some(NewSeriesName::Reference("Sheet1!$C$1")),
        categories: Some("Sheet1!$A$2:$A$4"),
        values: "Sheet1!$C$2:$C$4",
        sizes: None,
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
    },
    NewSeries {
        name: None,
        categories: None,
        values: "Sheet1!$D$2:$D$4",
        sizes: Some("Sheet1!$E$2:$E$4"),
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
    },
    NewSeries {
        name: Some(NewSeriesName::Text("high")),
        categories: Some("Sheet1!$A$2:$A$4"),
        values: "Sheet1!$C$2:$C$4",
        sizes: None,
    },
    NewSeries {
        name: Some(NewSeriesName::Text("low")),
        categories: Some("Sheet1!$A$2:$A$4"),
        values: "Sheet1!$D$2:$D$4",
        sizes: None,
    },
    NewSeries {
        name: Some(NewSeriesName::Text("close")),
        categories: Some("Sheet1!$A$2:$A$4"),
        values: "Sheet1!$E$2:$E$4",
        sizes: None,
    },
    NewSeries {
        name: Some(NewSeriesName::Text("extra")),
        categories: Some("Sheet1!$A$2:$A$4"),
        values: "Sheet1!$F$2:$F$4",
        sizes: None,
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
