//! probe-emend-charts-write-xml: the feature `charts`'s consumer, built
//! alone with its own feature set, measured beside the core probe as the
//! `read` probe is (`specs/0034-charts-read.md`, "Size and packaging").
//!
//! `probe-emend-format-write-xml`'s work, unchanged. Then a second package
//! opened on its second output, `add_chart` of one column chart with one
//! series on that output's first sheet, by the part `sheets` gives it, from
//! the probe's own constants (`specs/0035-charts-write-first-set.md`, "Size
//! and packaging"), written into memory and reopened, and `charts` of that
//! sheet: two more lines, `charts`, the chart count, and `chart_series`, the
//! series over every plot of every chart, so the writer and the reader are
//! linked and nothing is dead. The second output itself is not changed.
//!
//! The second add (`specs/0036-charts-write-second-set.md`,
//! `specs/0039-chart-kinds-and-formatting.md` and
//! `specs/0040-analytic-chart-elements.md`, "The probe"): on the same
//! package, `remove_chart` of that chart, written into memory and reopened
//! (a removal and an addition on one part in one write are refused), then
//! a second `add_chart` whose plot kind is the command line's index 0 to 15
//! over the sixteen `NewPlot` variants in their declared order, so every
//! plot arm is live. Its series, one on a pie, a 3D pie or an of-pie, three
//! on a stock plot and two otherwise, each take what their kind takes: a
//! colour and labels (none on a surface's), a marker, a line and one
//! formatted point, an explosion on the round kinds, a marker on the point
//! where the series takes one, and on a bar, line, scatter, area or bubble
//! series a linear trendline (R-squared shown, forecast forward 1) and
//! fixed error bars (1, both ways, end caps; direction y on a scatter's or
//! a bubble's). The chart takes the axis titles and formats where the kind
//! takes them; on a bar or an area a combined line on a secondary axis with
//! its title and format, its series as a line's; the plot lines, drop
//! lines, high-low lines and up-down bars on the line, drop lines on the
//! area, drop lines and up-down bars beside a stock plot's high-low lines;
//! and the axis options: the value axis of every kind that writes one
//! bounded 0 to 100 with a major unit of 25 and a minor unit of 5,
//! reversed; the category axis of the bar and area kinds and of the 3D
//! line skipping every second label and tick mark; the line's (index 1)
//! category axis the date form, base unit days, major unit one day; a
//! scatter's x axis of logarithm base 10. Written into memory and
//! reopened, then `charts`.
//!
//! The chartsheet (`specs/0038-chartsheets.md`, "Size and packaging"): on
//! the same package, `add_chartsheet` of the first set's column chart at
//! the chartsheet sample's size, written into memory and reopened, then
//! `charts` of the new part: one more line, `chartsheet_charts`, its chart
//! count (1).
//!
//! Every name, part, reference, code, formula text, font name, index and
//! area of the format probe's work comes from the command line, so the
//! compiler cannot prove an edit or a read path dead and drop it; the
//! chart's references and texts are the probe's own constants (`specs/0035-
//! charts-write-first-set.md`, ruling R).

use std::fs::File;
use std::io::{self, BufReader, BufWriter, Cursor, Write};
use std::os::unix::ffi::OsStrExt;
use std::process::ExitCode;

use emend::cells::{Area, CellRef, FormulaKind, Value};
use emend::package::{
    BarDirection, CALC_CHAIN, DashStyle, EditAs, ErrorBarKind, ErrorDirection, ErrorValue,
    Grouping, ImageAnchor, LabelPosition, LegendPosition, MarkerSymbol, NewAxis, NewChart,
    NewCombined, NewDateAxis, NewErrorBars, NewLabels, NewLine, NewMarker, NewPlot, NewPoint,
    NewSeries, NewSeriesName, NewTrendline, OfPieKind, OfPieSplit, Package, RadarStyle,
    ScatterStyle, THEME, TimeUnit, TrendlineKind,
};
use emend::styles::{
    Alignment, Border, BorderStyle, Color, Fill, Font, Horizontal, NumberFormat, Pattern,
    Protection, Side, Style, Vertical,
};
use emend::{BYTE_ORDER_MARK, Replacement, Stream};
use quick_xml::Reader;
use quick_xml::events::Event;

const USAGE: &str = "usage: probe-emend-charts-write-xml <archive> <output> \
<part to replace> <override part name> <content type> <sheet part> \
<number at> <text at> <boolean at> <clear at> <second output> <format code> \
<formula at> <formula text> <sheet name> <new sheet name> <third output> <font name> \
<column> <row> <freeze at> <merge area> <chart kind>";

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let [
        input,
        output,
        replace,
        over,
        content_type,
        sheet,
        number_at,
        text_at,
        boolean_at,
        clear_at,
        second,
        code,
        formula_at,
        formula,
        added,
        renamed,
        third,
        font_name,
        column,
        row,
        freeze_at,
        merge_area,
        kind,
    ] = args.as_slice()
    else {
        let _ = writeln!(io::stderr(), "error: {USAGE}");
        return ExitCode::from(2);
    };
    let names = Names {
        replace: replace.as_bytes(),
        over: over.as_bytes(),
        content_type: content_type.as_bytes(),
        sheet: sheet.as_bytes(),
    };
    let at = [number_at, text_at, boolean_at, clear_at, formula_at]
        .map(|a| CellRef::parse(a.as_bytes()));
    let [
        Some(number_at),
        Some(text_at),
        Some(boolean_at),
        Some(clear_at),
        Some(formula_at),
    ] = at
    else {
        let _ = writeln!(io::stderr(), "error: a reference is not of the form A1");
        return ExitCode::from(2);
    };
    let Some(code) = code.to_str() else {
        let _ = writeln!(io::stderr(), "error: the format code is not UTF-8");
        return ExitCode::from(2);
    };
    let Some(formula) = formula.to_str() else {
        let _ = writeln!(io::stderr(), "error: the formula text is not UTF-8");
        return ExitCode::from(2);
    };
    let Some(font_name) = font_name.to_str() else {
        let _ = writeln!(io::stderr(), "error: the font name is not UTF-8");
        return ExitCode::from(2);
    };
    let (Some(column), Some(row)) = (
        column.to_str().and_then(|c| c.parse::<u32>().ok()),
        row.to_str().and_then(|r| r.parse::<u32>().ok()),
    ) else {
        let _ = writeln!(io::stderr(), "error: a column or row is not a decimal u32");
        return ExitCode::from(2);
    };
    let (Some(freeze_at), Some(merge_area)) = (
        CellRef::parse(freeze_at.as_bytes()),
        Area::parse(merge_area.as_bytes()),
    ) else {
        let _ = writeln!(
            io::stderr(),
            "error: the freeze cell or the merge area does not parse"
        );
        return ExitCode::from(2);
    };
    let Some(kind) = kind
        .to_str()
        .and_then(|k| k.parse::<usize>().ok())
        .and_then(|k| PLOTS.get(k).copied())
    else {
        let _ = writeln!(io::stderr(), "error: the chart kind is not 0 to 15");
        return ExitCode::from(2);
    };
    let (Some(added), Some(renamed)) = (added.to_str(), renamed.to_str()) else {
        let _ = writeln!(io::stderr(), "error: a sheet name is not UTF-8");
        return ExitCode::from(2);
    };
    let edits = CellEdits {
        number_at,
        text_at,
        boolean_at,
        clear_at,
        code: code.to_string(),
        formula_at,
        formula: formula.to_string(),
        added: added.to_string(),
        renamed: renamed.to_string(),
        font_name: font_name.to_string(),
        column,
        row,
        freeze_at,
        merge_area,
        kind,
    };
    let result = run(input, output, &names).and_then(|counts| {
        let written = edit(input, second, names.sheet, &edits)?;
        Ok((counts, written, new_workbook(third)?))
    });
    match result {
        Ok((counts, written, new_bytes)) => {
            let _ = writeln!(
                io::stdout(),
                "sheets {}\nelements {}\nitems {}\ncells {}\nsum {}\ntext {}\nwritten {}\n\
                 xfs_before {}\nnumfmts_before {}\nxfs_after {}\nnumfmts_after {}\n\
                 number_style {}\ntext_style {}\nfull_calc {}\nchain {}\nformula_read_back {}\n\
                 sheets {}\nfirst_name {}\nfirst_part {}\nnew_bytes {}\n\
                 fonts {}\nfills {}\nborders {}\nfull_style {}\nfull_font {}\nfull_fill {}\n\
                 full_border {}\nmerges {}\ncharts {}\nchart_series {}\nchartsheet_charts {}",
                counts.sheets,
                counts.elements,
                counts.items,
                counts.cells,
                counts.sum,
                counts.text,
                written.cells,
                written.xfs_before,
                written.numfmts_before,
                written.xfs_after,
                written.numfmts_after,
                style(written.number_style),
                style(written.text_style),
                u64::from(written.full_calc),
                u64::from(written.chain),
                u64::from(written.formula_read_back),
                written.sheets,
                written.first_name,
                written.first_part,
                new_bytes,
                style(written.fonts),
                style(written.fills),
                style(written.borders),
                style(written.number_style),
                style(written.full_ids.0),
                style(written.full_ids.1),
                style(written.full_ids.2),
                written.merges,
                written.charts,
                written.chart_series,
                written.chartsheet_charts,
            );
            ExitCode::SUCCESS
        }
        Err(message) => {
            let _ = writeln!(io::stderr(), "error: {message}");
            ExitCode::from(2)
        }
    }
}

/// What the edits and the reads are made from, all of it from the command
/// line.
struct Names<'a> {
    replace: &'a [u8],
    over: &'a [u8],
    content_type: &'a [u8],
    sheet: &'a [u8],
}

/// Where the part 4 edits go, the code the part 5 addition takes, where the
/// part 6 formula goes and its text, and the two sheet names M2 part 1's
/// edits use, all of it from the command line.
struct CellEdits {
    number_at: CellRef,
    text_at: CellRef,
    boolean_at: CellRef,
    clear_at: CellRef,
    code: String,
    formula_at: CellRef,
    formula: String,
    added: String,
    renamed: String,
    font_name: String,
    column: u32,
    row: u32,
    freeze_at: CellRef,
    merge_area: Area,
    kind: NewPlot,
}

/// What the second write is checked by: the cells read back, the styles
/// counts before the edits and after the write, and the two styled cells'
/// `style` read back.
struct Written {
    cells: u64,
    xfs_before: u64,
    numfmts_before: u64,
    xfs_after: u64,
    numfmts_after: u64,
    number_style: Option<u32>,
    text_style: Option<u32>,
    full_calc: bool,
    chain: bool,
    formula_read_back: bool,
    sheets: u64,
    first_name: u64,
    first_part: u64,
    fonts: Option<u32>,
    fills: Option<u32>,
    borders: Option<u32>,
    full_ids: (Option<u32>, Option<u32>, Option<u32>),
    merges: u64,
    charts: u64,
    chart_series: u64,
    chartsheet_charts: u64,
}

/// A cell's `style` as printed: the index, or `none`.
fn style(style: Option<u32>) -> String {
    style.map_or_else(|| "none".to_string(), |s| s.to_string())
}

/// Part 4's own work: the input opened again, the four cell edits and the
/// recipe recorded, the write to `second`, and the cell count of `sheet` read
/// back from it. Part 5's own work on the same package: the styles counts,
/// the two additions and the two style edits before the write, and the
/// counts and the two cells' styles read back after it. Part 6's own work:
/// the formula set before the write, and the workbook's two flags and the
/// formula read back after it. M2 part 1's own work: the four sheet edits
/// before the write, and the sheet count and the first sheet's name and part
/// read back after it.
fn edit(
    input: &std::ffi::OsStr,
    second: &std::ffi::OsStr,
    sheet: &[u8],
    edits: &CellEdits,
) -> Result<Written, String> {
    let file = File::open(input).map_err(|e| e.kind().to_string())?;
    let mut package = Package::open(BufReader::new(file)).map_err(|e| e.to_string())?;
    package.set_cell(sheet, edits.number_at, Value::Number(2.5));
    package.set_cell(sheet, edits.text_at, Value::Text("probe".to_string()));
    package.set_cell(sheet, edits.boolean_at, Value::Boolean(true));
    package.clear_cell(sheet, edits.clear_at);
    package.drop_calc_chain().map_err(|e| e.to_string())?;

    let styles = package.styles().map_err(|e| e.to_string())?;
    let xfs_before = styles.cell_xfs.len() as u64;
    let numfmts_before = styles.num_fmts.len() as u64;
    let coded = package
        .add_cell_style(Some(0), NumberFormat::Code(edits.code.clone()))
        .map_err(|e| e.to_string())?;
    let built_in = package
        .add_cell_style(None, NumberFormat::BuiltIn(14))
        .map_err(|e| e.to_string())?;
    package.set_cell_style(sheet, edits.number_at, Some(coded));
    package.set_cell_style(sheet, edits.text_at, Some(built_in));
    package.set_cell(
        sheet,
        edits.formula_at,
        Value::Formula(edits.formula.clone()),
    );
    let first = package
        .sheets()
        .map_err(|e| e.to_string())?
        .first()
        .map(|sheet| sheet.name.clone())
        .ok_or("the workbook has no sheet")?;
    package.add_sheet(&edits.added).map_err(|e| e.to_string())?;
    package
        .move_sheet(&edits.added, 0)
        .map_err(|e| e.to_string())?;
    package
        .rename_sheet(&edits.added, &edits.renamed)
        .map_err(|e| e.to_string())?;
    package.remove_sheet(&first).map_err(|e| e.to_string())?;
    let full = package
        .add_style(Some(0), &full_style(&edits.font_name))
        .map_err(|e| e.to_string())?;
    package.set_cell_style(sheet, edits.number_at, Some(full));
    let next_column = edits
        .column
        .checked_add(1)
        .ok_or("the column is the last u32")?;
    let next_row = edits.row.checked_add(1).ok_or("the row is the last u32")?;
    package
        .set_column_width(sheet, edits.column, Some(20.5))
        .map_err(|e| e.to_string())?;
    package
        .set_column_hidden(sheet, next_column, true)
        .map_err(|e| e.to_string())?;
    package
        .set_row_height(sheet, edits.row, Some(30.0))
        .map_err(|e| e.to_string())?;
    package
        .set_row_hidden(sheet, next_row, true)
        .map_err(|e| e.to_string())?;
    package
        .freeze_panes(sheet, Some(edits.freeze_at))
        .map_err(|e| e.to_string())?;
    package
        .merge_cells(sheet, edits.merge_area)
        .map_err(|e| e.to_string())?;

    let out = File::create(second).map_err(|e| e.kind().to_string())?;
    let mut writer = BufWriter::new(out);
    package.write_to(&mut writer).map_err(|e| e.to_string())?;
    writer.flush().map_err(|e| e.kind().to_string())?;
    drop(writer);

    let file = File::open(second).map_err(|e| e.kind().to_string())?;
    let mut written = Package::open(BufReader::new(file)).map_err(|e| e.to_string())?;
    let styles = written.styles().map_err(|e| e.to_string())?;
    let xfs_after = styles.cell_xfs.len() as u64;
    let numfmts_after = styles.num_fmts.len() as u64;
    let (fonts, fills, borders) = (styles.fonts(), styles.fills(), styles.borders());
    let full_ids = styles
        .cell_xfs
        .get(full as usize)
        .map_or((None, None, None), |xf| {
            (xf.font_id, xf.fill_id, xf.border_id)
        });
    let workbook = written.workbook().map_err(|e| e.to_string())?;
    let full_calc = workbook.full_calc_on_load;
    let chain = workbook.calc_chain.is_some();
    let sheets = workbook.sheets.len() as u64;
    let (first_name, first_part) = workbook
        .sheets
        .first()
        .map(|sheet| (sheet.name.len() as u64, sheet.part.len() as u64))
        .ok_or("the output has no sheet")?;
    let mut formula_read_back = false;
    let mut count = 0u64;
    let mut number_style = None;
    let mut text_style = None;
    for cell in written.cells(sheet).map_err(|e| e.to_string())? {
        let cell = cell.map_err(|e| e.to_string())?;
        count += 1;
        if cell.reference == edits.number_at {
            number_style = cell.style;
        } else if cell.reference == edits.text_at {
            text_style = cell.style;
        } else if cell.reference == edits.formula_at {
            formula_read_back = cell.value.is_none()
                && cell.formula.as_ref().is_some_and(|formula| {
                    formula.kind == FormulaKind::Normal && formula.text == edits.formula
                });
        }
    }
    let index = written
        .part(sheet)
        .ok_or("the output has no such sheet part")?;
    let merges = count_named(&mut written, index, b"mergeCell")?;
    let (charts, chart_series, chartsheet_charts) = read_first_sheet_charts(second, edits.kind)?;
    Ok(Written {
        cells: count,
        xfs_before,
        numfmts_before,
        xfs_after,
        numfmts_after,
        number_style,
        text_style,
        full_calc,
        chain,
        formula_read_back,
        sheets,
        first_name,
        first_part,
        fonts,
        fills,
        borders,
        full_ids,
        merges,
        charts,
        chart_series,
        chartsheet_charts,
    })
}

/// The feature `charts`'s write and read (specs/0035 "Size and packaging"):
/// a second package opened on `second`, `add_chart` of one column chart with
/// one series on its first sheet from the probe's own constants, written
/// into memory and reopened; then (specs/0036 "The probe") `remove_chart` of
/// it, written and reopened, and `add_chart` of `kind`'s chart, written and
/// reopened; then `charts` of that sheet: the chart count and the series
/// count over every plot of every chart; then (specs/0038 "Size and
/// packaging") `add_chartsheet` of the column chart at 960 by 640, written
/// and reopened, and `charts` of the new part: its chart count.
fn read_first_sheet_charts(
    second: &std::ffi::OsStr,
    kind: NewPlot,
) -> Result<(u64, u64, u64), String> {
    let file = File::open(second).map_err(|e| e.kind().to_string())?;
    let mut package = Package::open(BufReader::new(file)).map_err(|e| e.to_string())?;
    let first = package
        .workbook()
        .map_err(|e| e.to_string())?
        .sheets
        .first()
        .map(|sheet| sheet.part.clone())
        .ok_or("the output has no sheet")?;
    let series = [NewSeries {
        name: Some(NewSeriesName::Text("probe series")),
        categories: Some("'probe'!$A$1:$A$2"),
        values: "'probe'!$B$1:$B$2",
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
        plot: NewPlot::Bar {
            direction: BarDirection::Column,
            grouping: Grouping::Clustered,
        },
        title: Some("probe chart"),
        series: &series,
        category_axis_title: Some("probe categories"),
        value_axis_title: Some("probe values"),
        category_axis_format: None,
        value_axis_format: None,
        legend: Some(LegendPosition::Bottom),
        size: None,
        combined: None,
        drop_lines: false,
        high_low_lines: false,
        up_down_bars: false,
        category_axis: None,
        value_axis: None,
    };
    let anchor = ImageAnchor::TwoCell {
        from: CellRef { row: 3, col: 1 },
        to: CellRef { row: 19, col: 9 },
        from_offset: (0, 0),
        to_offset: (0, 0),
        edit_as: EditAs::OneCell,
    };
    package
        .add_chart(&first, anchor, &chart)
        .map_err(|e| e.to_string())?;
    let mut package = reopened(package)?;
    package.remove_chart(&first, 0).map_err(|e| e.to_string())?;
    let mut package = reopened(package)?;
    let series = second_set_series(kind);
    package
        .add_chart(&first, anchor, &second_set_chart(kind, &series))
        .map_err(|e| e.to_string())?;
    let mut package = reopened(package)?;
    let charts = package.charts(&first).map_err(|e| e.to_string())?;
    let series = charts
        .iter()
        .flat_map(|entry| &entry.chart.plots)
        .map(|plot| plot.series.len() as u64)
        .sum();
    let chartsheet = package
        .add_chartsheet(
            "probe chartsheet",
            &NewChart {
                size: Some((960, 640)),
                ..chart
            },
        )
        .map_err(|e| e.to_string())?;
    let mut package = reopened(package)?;
    let chartsheet_charts = package
        .charts(&chartsheet)
        .map_err(|e| e.to_string())?
        .len() as u64;
    Ok((charts.len() as u64, series, chartsheet_charts))
}

/// A package written into memory and opened again.
fn reopened<R: io::Read + io::Seek>(
    package: Package<R>,
) -> Result<Package<Cursor<Vec<u8>>>, String> {
    let mut out = Cursor::new(Vec::new());
    package.write_to(&mut out).map_err(|e| e.to_string())?;
    out.set_position(0);
    Package::open(out).map_err(|e| e.to_string())
}

/// The sixteen `NewPlot` variants in their declared order, the index the
/// command line names (`specs/0036-charts-write-second-set.md`, "The
/// probe"; the last three `specs/0039-chart-kinds-and-formatting.md`, "The
/// probe").
const PLOTS: [NewPlot; 16] = [
    NewPlot::Bar {
        direction: BarDirection::Column,
        grouping: Grouping::Clustered,
    },
    NewPlot::Line {
        grouping: Grouping::Standard,
    },
    NewPlot::Pie,
    NewPlot::Scatter {
        style: ScatterStyle::LineMarker,
    },
    NewPlot::Area {
        grouping: Grouping::Standard,
    },
    NewPlot::Doughnut { hole_size: 50 },
    NewPlot::Radar {
        style: RadarStyle::Marker,
    },
    NewPlot::Bubble,
    NewPlot::Stock,
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
    NewPlot::Surface { wireframe: true },
    NewPlot::Surface3D { wireframe: false },
    NewPlot::OfPie {
        kind: OfPieKind::Bar,
        split: Some(OfPieSplit::Position(1)),
    },
];

/// The labels each series takes: the value, in a format, outside the end
/// where the kind takes that position.
const fn labels(outside: bool) -> NewLabels<'static> {
    NewLabels {
        value: true,
        category: false,
        series_name: false,
        percent: false,
        position: if outside {
            Some(LabelPosition::OutsideEnd)
        } else {
            None
        },
        number_format: Some("0.0"),
    }
}

/// The point's colour and the marker a point takes where its series takes
/// one (`specs/0039-chart-kinds-and-formatting.md`, "The probe").
const POINT_COLOR: u32 = 0xD6_2728;
const POINT_MARKER: NewMarker = NewMarker {
    symbol: MarkerSymbol::Circle,
    size: Some(7),
    color: Some(POINT_COLOR),
};

/// The one point each series takes: index 0, a colour; an explosion of 10
/// on a round kind; a marker where the series takes one.
const PLAIN_POINT: [NewPoint; 1] = [NewPoint {
    index: 0,
    color: Some(POINT_COLOR),
    explosion: None,
    marker: None,
}];
const ROUND_POINT: [NewPoint; 1] = [NewPoint {
    explosion: Some(10),
    ..PLAIN_POINT[0]
}];
const MARKED_POINT: [NewPoint; 1] = [NewPoint {
    marker: Some(POINT_MARKER),
    ..PLAIN_POINT[0]
}];

/// The trendline each series takes where its kind takes one: linear,
/// showing R-squared, forecast forward 1 (`specs/0040-analytic-chart-
/// elements.md`, "The probe").
const TRENDLINE: NewTrendline = NewTrendline {
    kind: TrendlineKind::Linear,
    forward: Some(1.0),
    backward: None,
    intercept: None,
    show_equation: false,
    show_r_squared: true,
    name: None,
};

/// The error bars each series takes where its kind takes them: fixed 1,
/// both ways, end caps; on a scatter's or a bubble's, direction y.
const ERROR_BARS: [NewErrorBars; 1] = [NewErrorBars {
    direction: None,
    kind: ErrorBarKind::Both,
    value: ErrorValue::Fixed(1.0),
    end_cap: true,
}];
const XY_ERROR_BARS: [NewErrorBars; 1] = [NewErrorBars {
    direction: Some(ErrorDirection::Y),
    ..ERROR_BARS[0]
}];

/// The axis options, each axis's from these (`specs/0040-analytic-chart-
/// elements.md`, "The probe", rulings L and T): the value axis bounded 0
/// to 100, its units 25 and 5, reversed; the category axis of the bar and
/// area kinds and of the 3D line skipping every second label and tick mark;
/// the line's category axis the date form; a scatter's x axis of logarithm
/// base 10.
const NO_AXIS: NewAxis = NewAxis {
    min: None,
    max: None,
    log_base: None,
    reversed: false,
    major_unit: None,
    minor_unit: None,
    tick_label_skip: None,
    tick_mark_skip: None,
    date: None,
};
const VALUE_AXIS: NewAxis = NewAxis {
    min: Some(0.0),
    max: Some(100.0),
    reversed: true,
    major_unit: Some(25.0),
    minor_unit: Some(5.0),
    ..NO_AXIS
};
const SKIPPING_AXIS: NewAxis = NewAxis {
    tick_label_skip: Some(2),
    tick_mark_skip: Some(2),
    ..NO_AXIS
};
const DATE_AXIS: NewAxis = NewAxis {
    date: Some(NewDateAxis {
        base_unit: TimeUnit::Days,
        major: Some((1, TimeUnit::Days)),
        minor: None,
    }),
    ..NO_AXIS
};
const LOG_AXIS: NewAxis = NewAxis {
    log_base: Some(10.0),
    ..NO_AXIS
};

/// One series of the second chart: its values' column, its colour (none on
/// a stock plot or a surface), its labels (none on a surface), sizes on a
/// bubble's; a marker (`Circle`, size 7, the series' colour) on a line, a
/// scatter or a marker radar, a line (28575 EMU, `Dash`) on a line, a 3D
/// line, a scatter, a radar or a stock plot, and one point on every kind but
/// the surfaces (`specs/0039-chart-kinds-and-formatting.md`, "The probe");
/// the trendline and the error bars on a bar, line, scatter, area or bubble
/// series (`specs/0040-analytic-chart-elements.md`, "The probe").
const fn probe_series(values: &'static str, color: u32, kind: NewPlot) -> NewSeries<'static> {
    let surface = matches!(kind, NewPlot::Surface { .. } | NewPlot::Surface3D { .. });
    let round = matches!(
        kind,
        NewPlot::Pie | NewPlot::Pie3D | NewPlot::Doughnut { .. } | NewPlot::OfPie { .. }
    );
    let marked = matches!(
        kind,
        NewPlot::Line { .. } | NewPlot::Scatter { .. } | NewPlot::Radar { .. }
    );
    let lined = matches!(
        kind,
        NewPlot::Line { .. }
            | NewPlot::Line3D { .. }
            | NewPlot::Scatter { .. }
            | NewPlot::Radar { .. }
            | NewPlot::Stock
    );
    let outside = matches!(
        kind,
        NewPlot::Bar {
            grouping: Grouping::Clustered,
            ..
        } | NewPlot::Pie
            | NewPlot::Pie3D
    );
    let analytic = matches!(
        kind,
        NewPlot::Bar { .. }
            | NewPlot::Line { .. }
            | NewPlot::Scatter { .. }
            | NewPlot::Area { .. }
            | NewPlot::Bubble
    );
    let xy = matches!(kind, NewPlot::Scatter { .. } | NewPlot::Bubble);
    NewSeries {
        name: Some(NewSeriesName::Text("probe series")),
        categories: Some("'probe'!$A$1:$A$2"),
        values,
        sizes: if matches!(kind, NewPlot::Bubble) {
            Some("'probe'!$E$1:$E$2")
        } else {
            None
        },
        color: if matches!(kind, NewPlot::Stock) || surface {
            None
        } else {
            Some(color)
        },
        labels: if surface { None } else { Some(labels(outside)) },
        marker: if marked {
            Some(NewMarker {
                symbol: MarkerSymbol::Circle,
                size: Some(7),
                color: Some(color),
            })
        } else {
            None
        },
        line: if lined {
            Some(NewLine {
                width: Some(28575),
                dash: Some(DashStyle::Dash),
            })
        } else {
            None
        },
        points: if surface {
            &[]
        } else if round {
            &ROUND_POINT
        } else if marked {
            &MARKED_POINT
        } else {
            &PLAIN_POINT
        },
        trendline: if analytic { Some(TRENDLINE) } else { None },
        error_bars: if !analytic {
            &[]
        } else if xy {
            &XY_ERROR_BARS
        } else {
            &ERROR_BARS
        },
    }
}

/// The combined line's one series, on the secondary axis, with a line's
/// trendline and error bars.
const COMBINED_SERIES: [NewSeries<'static>; 1] = [NewSeries {
    name: Some(NewSeriesName::Text("probe line")),
    categories: Some("'probe'!$A$1:$A$2"),
    values: "'probe'!$D$1:$D$2",
    sizes: None,
    color: Some(0x2C_A02C),
    labels: None,
    marker: None,
    line: None,
    points: &[],
    trendline: Some(TRENDLINE),
    error_bars: &ERROR_BARS,
}];

/// The three series the second set's chart takes its own from.
fn second_set_series(kind: NewPlot) -> [NewSeries<'static>; 3] {
    [
        probe_series("'probe'!$B$1:$B$2", 0x1F_77B4, kind),
        probe_series("'probe'!$C$1:$C$2", 0xFF_7F0E, kind),
        probe_series("'probe'!$D$1:$D$2", 0x2C_A02C, kind),
    ]
}

/// The second set's chart for `kind` (`specs/0036-charts-write-second-
/// set.md`, "The probe", P4 ruling R): two series (one on a pie, a 3D pie
/// or an of-pie, three on a stock plot), each coloured and labelled where
/// the kind takes it, with its marker, line, point, trendline and error
/// bars; the axis titles and formats where the kind takes them; on a bar or
/// an area a combined line on a secondary axis with its title and format;
/// the plot lines and the axis options where the kind takes them
/// (`specs/0040-analytic-chart-elements.md`, "The probe", ruling L).
fn second_set_chart<'a>(kind: NewPlot, all: &'a [NewSeries<'a>; 3]) -> NewChart<'a> {
    let round = matches!(
        kind,
        NewPlot::Pie | NewPlot::Pie3D | NewPlot::Doughnut { .. } | NewPlot::OfPie { .. }
    );
    let xy = matches!(kind, NewPlot::Scatter { .. } | NewPlot::Bubble);
    let count = match kind {
        NewPlot::Pie | NewPlot::Pie3D | NewPlot::OfPie { .. } => 1,
        NewPlot::Stock => 3,
        _ => 2,
    };
    let combined =
        matches!(kind, NewPlot::Bar { .. } | NewPlot::Area { .. }).then_some(NewCombined {
            plot: NewPlot::Line {
                grouping: Grouping::Standard,
            },
            series: &COMBINED_SERIES,
            secondary_axis: true,
            secondary_axis_title: Some("probe secondary"),
            secondary_axis_format: Some("0%"),
        });
    let category_axis = match kind {
        NewPlot::Line { .. } => Some(DATE_AXIS),
        NewPlot::Bar { .. }
        | NewPlot::Bar3D { .. }
        | NewPlot::Line3D { .. }
        | NewPlot::Area { .. }
        | NewPlot::Area3D { .. } => Some(SKIPPING_AXIS),
        NewPlot::Scatter { .. } => Some(LOG_AXIS),
        _ => None,
    };
    NewChart {
        plot: kind,
        title: Some("probe second set"),
        series: &all[..count],
        category_axis_title: (!round).then_some("probe categories"),
        value_axis_title: (!round).then_some("probe values"),
        category_axis_format: (!round && !xy).then_some("@"),
        value_axis_format: (!round).then_some("#,##0.00"),
        legend: Some(LegendPosition::Right),
        size: None,
        combined,
        drop_lines: matches!(
            kind,
            NewPlot::Line { .. } | NewPlot::Area { .. } | NewPlot::Stock
        ),
        high_low_lines: matches!(kind, NewPlot::Line { .. } | NewPlot::Stock),
        up_down_bars: matches!(kind, NewPlot::Line { .. } | NewPlot::Stock),
        category_axis,
        value_axis: (!round).then_some(VALUE_AXIS),
    }
}

/// The elements named `name` (local name) in one part, through the crate's
/// own `Stream`.
fn count_named<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    index: usize,
    name: &[u8],
) -> Result<u64, String> {
    let part = package.read_part(index).map_err(|e| e.to_string())?;
    let mut stream = Stream::new(part, index).map_err(|e| e.to_string())?;
    let mut count = 0u64;
    while let Some(token) = stream.next().map_err(|e| e.to_string())? {
        if let Event::Start(start) | Event::Empty(start) = &token.event
            && start.local_name().into_inner().as_bytes() == name
        {
            count += 1;
        }
    }
    Ok(count)
}

/// The `Style` the part's edit adds: every element this part composes.
fn full_style(name: &str) -> Style {
    let side = Some(Side {
        style: BorderStyle::Thin,
        color: Some(Color::Theme {
            index: 1,
            tint: Some(-0.25),
        }),
    });
    Style {
        number_format: None,
        font: Some(Font {
            name: Some(name.to_string()),
            size: Some(11.5),
            bold: true,
            color: Some(Color::Rgb(0xFF33_6699)),
            ..Font::default()
        }),
        fill: Some(Fill {
            pattern: Pattern::Solid,
            foreground: Some(Color::Rgb(0xFFDD_EEFF)),
            background: Some(Color::Indexed(64)),
        }),
        border: Some(Border {
            left: side,
            right: side,
            top: side,
            bottom: side,
            diagonal: side,
            diagonal_up: true,
            diagonal_down: true,
        }),
        alignment: Some(Alignment {
            horizontal: Some(Horizontal::Center),
            vertical: Some(Vertical::Top),
            wrap_text: true,
            shrink_to_fit: true,
            indent: Some(1),
            text_rotation: Some(90),
        }),
        protection: Some(Protection {
            locked: Some(false),
            hidden: Some(true),
        }),
    }
}

/// `Package::new()` written to `third`, and the bytes written.
fn new_workbook(third: &std::ffi::OsStr) -> Result<u64, String> {
    let package = Package::new().map_err(|e| e.to_string())?;
    let out = File::create(third).map_err(|e| e.kind().to_string())?;
    let mut writer = BufWriter::new(out);
    package.write_to(&mut writer).map_err(|e| e.to_string())?;
    writer.flush().map_err(|e| e.kind().to_string())?;
    drop(writer);
    let bytes = std::fs::metadata(third)
        .map_err(|e| e.kind().to_string())?
        .len();
    Ok(bytes)
}

/// What the probe prints. A struct rather than a tuple because six numbers in
/// a row are easy to print in the wrong order.
struct Counts {
    sheets: u64,
    elements: u64,
    items: u64,
    cells: u64,
    sum: f64,
    text: u64,
}

fn run(input: &std::ffi::OsStr, output: &std::ffi::OsStr, names: &Names) -> Result<Counts, String> {
    let file = File::open(input).map_err(|e| e.kind().to_string())?;
    let mut package = Package::open(BufReader::new(file)).map_err(|e| e.to_string())?;

    let workbook = package.workbook().map_err(|e| e.to_string())?;
    let sheets = workbook.sheets.len() as u64;
    let workbook_part = workbook.part.clone();

    // The recipe, when this package has a chain: the relationship, the
    // override for its target if there is one, the entry if there is one.
    let chain = {
        let rels = package
            .relationships(&workbook_part)
            .map_err(|e| e.to_string())?;
        match rels.first_of(CALC_CHAIN) {
            Some(rel) => Some((
                rel.id.clone(),
                rels.target_part(rel).map_err(|e| e.to_string())?,
            )),
            None => None,
        }
    };
    if let Some((id, part)) = chain {
        let over = package
            .content_types()
            .map_err(|e| e.to_string())?
            .overrides()
            .iter()
            .find(|over| {
                over.part_name
                    .strip_prefix(b"/".as_slice())
                    .unwrap_or(&over.part_name)
                    == part.as_slice()
            })
            .map(|over| over.part_name.clone());
        let entry = package.part(&part).is_some();
        package.remove_relationship(&workbook_part, &id);
        if let Some(over) = over {
            package.remove_override(&over);
        }
        if entry {
            package.remove_part(&part);
        }
    }

    // The two additions, and the replacement of the named part by its own
    // bytes through the token stream.
    package.add_override(names.over, names.content_type);
    package
        .add_relationship(&workbook_part, THEME[0], names.over, false)
        .map_err(|e| e.to_string())?;

    let index = package
        .part(names.replace)
        .ok_or("no part to replace by that name")?;
    let own = through_the_stream(&mut package, index)?;
    let elements = count_elements(&own)?;
    package.replace_part(names.replace, bytes(own));

    // Part 3's own work: the table, then every cell of the named sheet.
    //
    // Before the write rather than after it, which is the one place this
    // probe departs from the order `specs/0010-reading-cells.md`, "Size",
    // lists: `Package::write_to` takes the package by value, so nothing can
    // be read from it afterwards. The edits above are all recorded by then,
    // so the reads happen against a package with a replacement, an addition
    // and a removal pending, which is the harder case and not the easier
    // one.
    let table = package.shared_strings().map_err(|e| e.to_string())?;
    let items = u64::from(table.len());
    let (cells, sum, text) = read_cells(&mut package, &table, names.sheet)?;

    let out = File::create(output).map_err(|e| e.kind().to_string())?;
    let mut writer = BufWriter::new(out);
    package.write_to(&mut writer).map_err(|e| e.to_string())?;
    writer.flush().map_err(|e| e.kind().to_string())?;

    Ok(Counts {
        sheets,
        elements,
        items,
        cells,
        sum,
        text,
    })
}

/// Every cell of one sheet: how many, the sum of `number()` over them, and
/// the total length in bytes of the text `resolve` answers with.
///
/// The length is over EVERY cell, not the shared ones alone, because that is
/// what `resolve` answers: the table's item for a shared string and the
/// cell's own value for every other kind. A reader of the printed `text`
/// should expect the numbers' own digits in it.
///
/// A `resolve` failure is counted as no text rather than ending the read: a
/// stale index is a fact about the workbook, as it is for the tool
/// (`specs/0010-reading-cells.md`, "Harness runs").
fn read_cells<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    table: &emend::SharedStrings,
    sheet: &[u8],
) -> Result<(u64, f64, u64), String> {
    let cells = package.cells(sheet).map_err(|e| e.to_string())?;
    let mut count = 0u64;
    let mut sum = 0.0f64;
    let mut text = 0u64;
    for cell in cells {
        let cell = cell.map_err(|e| e.to_string())?;
        count += 1;
        if let Some(number) = cell.number() {
            sum += number;
        }
        if let Ok(Some(item)) = table.resolve(&cell) {
            text += item.len() as u64;
        }
    }
    Ok((count, sum, text))
}

/// One part's bytes as the stream reproduces them: the mark when the part has
/// one, then every token's own bytes.
fn through_the_stream<R: io::Read + io::Seek>(
    package: &mut Package<R>,
    index: usize,
) -> Result<Vec<u8>, String> {
    let part = package.read_part(index).map_err(|e| e.to_string())?;
    let mut stream = Stream::new(part, index).map_err(|e| e.to_string())?;
    let mut own = Vec::new();
    if stream.bom() {
        own.extend_from_slice(&BYTE_ORDER_MARK);
    }
    while let Some(token) = stream.next().map_err(|e| e.to_string())? {
        own.extend_from_slice(token.raw);
    }
    Ok(own)
}

/// The element count `probe-zip-xml` makes, streaming, over these bytes.
fn count_elements(xml: &[u8]) -> Result<u64, String> {
    let mut reader = Reader::from_reader(xml);
    let mut buf = Vec::new();
    let mut elements = 0u64;
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(_) | Event::Empty(_)) => elements += 1,
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(_) => return Err("the entry is not well-formed XML".into()),
        }
        buf.clear();
    }
    Ok(elements)
}

/// A replacement that writes these bytes.
fn bytes(data: Vec<u8>) -> Replacement {
    Box::new(move |out: &mut dyn Write| out.write_all(&data))
}
