//! probe-emend-formulas-write-xml: the consumer the part 6 budget describes,
//! with the whole core to date linked (`specs/0013-formulas.md`, "Size").
//!
//! `probe-emend-styles-write-xml`'s work, unchanged, then the part 6 edit on
//! the same package: `set_cell` of a `Formula`, its text from the command
//! line, on a third cell, so the write's formula-change rule runs and the
//! workbook part's `calcPr` is rewritten. After the write the second output's
//! `Workbook::full_calc_on_load` and `calc_chain` are read, and the third
//! cell's `formula` through `Package::cells` in the same pass that counts the
//! cells read back. It prints what `probe-emend-styles-write-xml` prints and
//! then three lines: `full_calc` and `chain` (1 or 0) and `formula_read_back`
//! (1 when the cell reads back as a normal formula with the text written and
//! no value), so the formula write, the rule, the `calcPr` rewrite and both
//! reads are all reached.
//!
//! What this probe does NOT run: whether the `calcPr` start tag is changed or
//! a new one inserted, and whether the chain is present to drop, depend on
//! the input; the refusals run only on an input that earns them.
//!
//! Every name, part, reference, code and formula text comes from the command
//! line, so the compiler cannot prove an edit or a read path dead and drop
//! it.

use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};
use std::os::unix::ffi::OsStrExt;
use std::process::ExitCode;

use emend::cells::{CellRef, FormulaKind, Value};
use emend::package::{CALC_CHAIN, Package, THEME};
use emend::styles::NumberFormat;
use emend::{BYTE_ORDER_MARK, Replacement, Stream};
use quick_xml::Reader;
use quick_xml::events::Event;

const USAGE: &str = "usage: probe-emend-formulas-write-xml <archive> <output> \
<part to replace> <override part name> <content type> <sheet part> \
<number at> <text at> <boolean at> <clear at> <second output> <format code> \
<formula at> <formula text>";

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
    let edits = CellEdits {
        number_at,
        text_at,
        boolean_at,
        clear_at,
        code: code.to_string(),
        formula_at,
        formula: formula.to_string(),
    };
    let result = run(input, output, &names)
        .and_then(|counts| Ok((counts, edit(input, second, names.sheet, &edits)?)));
    match result {
        Ok((counts, written)) => {
            let _ = writeln!(
                io::stdout(),
                "sheets {}\nelements {}\nitems {}\ncells {}\nsum {}\ntext {}\nwritten {}\n\
                 xfs_before {}\nnumfmts_before {}\nxfs_after {}\nnumfmts_after {}\n\
                 number_style {}\ntext_style {}\nfull_calc {}\nchain {}\nformula_read_back {}",
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

/// Where the part 4 edits go, the code the part 5 addition takes, and where
/// the part 6 formula goes and its text, all of it from the command line.
struct CellEdits {
    number_at: CellRef,
    text_at: CellRef,
    boolean_at: CellRef,
    clear_at: CellRef,
    code: String,
    formula_at: CellRef,
    formula: String,
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
/// formula read back after it.
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
    let workbook = written.workbook().map_err(|e| e.to_string())?;
    let full_calc = workbook.full_calc_on_load;
    let chain = workbook.calc_chain.is_some();
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
    })
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
