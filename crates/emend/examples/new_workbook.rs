//! Starts a new workbook from emend's template, writes a header row and two
//! rows of text and numbers into the first sheet, and saves it.
//!
//! Run it from the repository's root:
//!
//!     cargo run -p emend --example new_workbook -- <output>

use emend::{CellRef, Value, package::Package};
use std::fs::File;
use std::io::BufWriter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let [_, output] = args.as_slice() else {
        eprintln!("usage: new_workbook <output>");
        std::process::exit(2);
    };
    let mut book = Package::new()?;
    let sheet = book
        .workbook()?
        .sheets
        .first()
        .ok_or("no sheet")?
        .part
        .clone();
    let cells = [
        ("A1", Value::Text("Item".into())),
        ("B1", Value::Text("Count".into())),
        ("A2", Value::Text("Bolts".into())),
        ("B2", Value::Number(120.0)),
        ("A3", Value::Text("Washers".into())),
        ("B3", Value::Number(2.5)),
    ];
    for (reference, value) in cells {
        let cell = CellRef::parse(reference.as_bytes()).ok_or("not an A1 reference")?;
        book.set_cell(&sheet, cell, value);
    }
    book.write_to(BufWriter::new(File::create(output)?))?;
    Ok(())
}
