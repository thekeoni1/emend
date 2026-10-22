//! probe-emend-richcomment-write-xml: the features `comments` and
//! `richtext` together, built with that feature set, measured beside the
//! `comments` probe, whose work it repeats (`specs/0024-rich-text.md`,
//! "Size", under decision 1 A).
//!
//! `probe-emend-comments-write-xml`'s work, unchanged, then on the same
//! package before its write `add_rich_comment` at a second fixed cell
//! ([`RICH_COMMENT_CELL`]) by the command line's author, of two runs, their
//! texts from the command line, the first bold and the second with no font.
//! After the write one more line read back through the feature's read side:
//! `rich_comment_runs`, the rich comment's run count through
//! `CommentEntry::runs`, so both sides are linked and nothing is dead.
//!
//! Every other name, part, reference, code, formula text, font name, index
//! and area comes from the command line, so the compiler cannot prove an
//! edit or a read path dead and drop it.

use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};
use std::os::unix::ffi::OsStrExt;
use std::process::ExitCode;

use emend::cells::{Area, CellRef, FormulaKind, RichText, Run, Value};
use emend::package::{CALC_CHAIN, Comment, Package, THEME};
use emend::styles::{
    Alignment, Border, BorderStyle, Color, Fill, Font, Horizontal, NumberFormat, Pattern,
    Protection, Side, Style, Vertical,
};
use emend::{BYTE_ORDER_MARK, Replacement, Stream};
use quick_xml::Reader;
use quick_xml::events::Event;

const USAGE: &str = "usage: probe-emend-richcomment-write-xml <archive> <output> \
<part to replace> <override part name> <content type> <sheet part> \
<number at> <text at> <boolean at> <clear at> <second output> <format code> \
<formula at> <formula text> <sheet name> <new sheet name> <third output> <font name> \
<column> <row> <freeze at> <merge area> <author> <text> <first run> <second run>";

/// The comment's cell, past every cell, column, row and area the format
/// probe's work takes from its arguments in the size log.
const COMMENT_CELL: &str = "AB10";

/// The rich comment's cell, past the comment's.
const RICH_COMMENT_CELL: &str = "AC10";

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
        author,
        text,
        first_run,
        second_run,
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
    let (Some(added), Some(renamed)) = (added.to_str(), renamed.to_str()) else {
        let _ = writeln!(io::stderr(), "error: a sheet name is not UTF-8");
        return ExitCode::from(2);
    };
    let (Some(author), Some(text)) = (author.to_str(), text.to_str()) else {
        let _ = writeln!(io::stderr(), "error: the author or the text is not UTF-8");
        return ExitCode::from(2);
    };
    let (Some(first_run), Some(second_run)) = (first_run.to_str(), second_run.to_str()) else {
        let _ = writeln!(io::stderr(), "error: a run's text is not UTF-8");
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
        comment: [author.to_string(), text.to_string()],
        runs: [first_run.to_string(), second_run.to_string()],
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
                 full_border {}\nmerges {}\ncomments {}\nrich_comment_runs {}",
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
                written.comments,
                written.rich_comment_runs,
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
    comment: [String; 2],
    runs: [String; 2],
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
    comments: u64,
    rich_comment_runs: u64,
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
    // The feature `comments`' edit (specs/0022 "Size").
    package
        .add_comment(
            sheet,
            CellRef::parse(COMMENT_CELL.as_bytes()).ok_or("the comment cell does not parse")?,
            Comment {
                author: &edits.comment[0],
                text: &edits.comment[1],
            },
        )
        .map_err(|e| e.to_string())?;
    // The features `comments` and `richtext` together (specs/0024 "Size").
    let rich_at = CellRef::parse(RICH_COMMENT_CELL.as_bytes())
        .ok_or("the rich comment cell does not parse")?;
    package
        .add_rich_comment(
            sheet,
            rich_at,
            &edits.comment[0],
            &RichText {
                runs: vec![
                    Run {
                        text: edits.runs[0].clone(),
                        font: Some(Font {
                            bold: true,
                            ..Font::default()
                        }),
                    },
                    Run {
                        text: edits.runs[1].clone(),
                        font: None,
                    },
                ],
            },
        )
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
    let entries = written.comments(sheet).map_err(|e| e.to_string())?;
    let comments = entries.len() as u64;
    let rich_comment_runs = entries
        .iter()
        .find(|entry| entry.cell == rich_at)
        .and_then(|entry| entry.runs.as_ref())
        .map_or(0, |rich| rich.runs.len() as u64);
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
        comments,
        rich_comment_runs,
    })
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
            gradient: None,
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
