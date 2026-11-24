//! Opens a workbook, sets B2 of its first sheet to a text, and writes the
//! result to a new file. Every other part of the input is copied unchanged.
//!
//! Run it from the repository's root:
//!
//!     cargo run -p emend --example edit_cell -- <input> <output>

use emend::{CellRef, Value, package::Package};
use std::fs::File;
use std::io::BufWriter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let [_, input, output] = args.as_slice() else {
        eprintln!("usage: edit_cell <input> <output>");
        std::process::exit(2);
    };
    let mut book = Package::open(File::open(input)?)?;
    let sheet = book
        .workbook()?
        .sheets
        .first()
        .ok_or("no sheet")?
        .part
        .clone();
    let cell = CellRef::parse(b"B2").ok_or("not an A1 reference")?;
    book.set_cell(&sheet, cell, Value::Text("checked".into()));
    book.write_to(BufWriter::new(File::create(output)?))?;
    Ok(())
}
