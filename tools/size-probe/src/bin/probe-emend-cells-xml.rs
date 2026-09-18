//! probe-emend-cells-xml: the consumer the part 3 budget describes, with the
//! whole core to date linked (`specs/0010-reading-cells.md`, "Size").
//!
//! `probe-emend-package-xml`'s work, then the workbook's shared strings, then
//! every cell of the sheet part named on its command line. It prints the cell
//! count, the sum of `number()` over the cells, and the total length of the
//! text `resolve` gives back, so the paths those three reach are in the
//! binary: the reference and the style are read to reach the cell at all,
//! `number()` carries the float parser, and `resolve` carries the table
//! lookup.
//!
//! What this probe does NOT reach, so that the size figure is read for what
//! it is: `Cell::boolean` and `Cell::error` have no caller here. Each is a
//! kind comparison and a return, and neither pulls anything else in, but the
//! part 3 cost this probe measures excludes them. A consumer that reads
//! booleans or errors pays that much more, and how much is not measured.
//!
//! The two reads happen after every edit is recorded and before the write,
//! because `Package::write_to` takes the package by value. So this probe
//! links the same code in a different order from the sentence that describes
//! it, and not less of it.
//!
//! Every name and part comes from the command line, so the compiler cannot
//! prove an edit or a read path dead and drop it.

use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};
use std::os::unix::ffi::OsStrExt;
use std::process::ExitCode;

use emend::package::{CALC_CHAIN, Package, THEME};
use emend::{BYTE_ORDER_MARK, Replacement, Stream};
use quick_xml::Reader;
use quick_xml::events::Event;

const USAGE: &str = "usage: probe-emend-cells-xml <archive> <output> \
<part to replace> <override part name> <content type> <sheet part>";

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let [input, output, replace, over, content_type, sheet] = args.as_slice() else {
        let _ = writeln!(io::stderr(), "error: {USAGE}");
        return ExitCode::from(2);
    };
    let names = Names {
        replace: replace.as_bytes(),
        over: over.as_bytes(),
        content_type: content_type.as_bytes(),
        sheet: sheet.as_bytes(),
    };
    match run(input, output, &names) {
        Ok(counts) => {
            let _ = writeln!(
                io::stdout(),
                "sheets {}\nelements {}\nitems {}\ncells {}\nsum {}\ntext {}",
                counts.sheets,
                counts.elements,
                counts.items,
                counts.cells,
                counts.sum,
                counts.text
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
