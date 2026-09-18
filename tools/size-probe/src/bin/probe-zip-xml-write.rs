//! probe-zip-xml-write: the size baseline that already carries a writer
//! (`specs/0009-package-layer.md`, "Size"). It is `probe-zip-xml` plus one
//! archive written through the `zip` crate's own `ZipWriter`: one deflated
//! entry, whose bytes are the ones it just read.
//!
//! The deflate encoder and the crate's writer are therefore in this baseline,
//! which is what the maintainer's ruling asks for: an emend probe measured
//! over this one shows the layer's own code apart from the compressor's.
//!
//! Every path and name comes from the command line, so the compiler cannot
//! prove any of it dead and drop it.

use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::process::ExitCode;

use quick_xml::Reader;
use quick_xml::events::Event;
use zip::ZipArchive;
use zip::write::{SimpleFileOptions, ZipWriter};

const USAGE: &str = "usage: probe-zip-xml-write <archive> <entry> <output>";

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let [path, entry, output] = args.as_slice() else {
        let _ = writeln!(io::stderr(), "error: {USAGE}");
        return ExitCode::from(2);
    };
    match run(path, entry, output) {
        Ok((entries, elements)) => {
            let _ = writeln!(io::stdout(), "entries {entries}\nelements {elements}");
            ExitCode::SUCCESS
        }
        Err(message) => {
            let _ = writeln!(io::stderr(), "error: {message}");
            ExitCode::from(2)
        }
    }
}

fn run(
    path: &std::ffi::OsStr,
    entry: &std::ffi::OsStr,
    output: &std::ffi::OsStr,
) -> Result<(u64, u64), String> {
    let file = File::open(path).map_err(|e| e.kind().to_string())?;
    let mut archive =
        ZipArchive::new(BufReader::new(file)).map_err(|_| "not a readable zip archive")?;
    let entries = archive.len() as u64;
    let name = entry.to_str().ok_or("the entry name is not UTF-8")?;
    let mut own = Vec::new();
    {
        let mut part = archive
            .by_name(name)
            .map_err(|_| "no readable entry of that name")?;
        part.read_to_end(&mut own)
            .map_err(|e| e.kind().to_string())?;
    }
    let elements = count_elements(&own)?;

    // The writer, with the deflate encoder behind it: the one thing this
    // baseline has that `probe-zip-xml` does not.
    let out = File::create(output).map_err(|e| e.kind().to_string())?;
    let mut writer = ZipWriter::new(BufWriter::new(out));
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    writer
        .start_file(name, options)
        .map_err(|_| "cannot start the output entry")?;
    writer.write_all(&own).map_err(|e| e.kind().to_string())?;
    let mut inner = writer.finish().map_err(|_| "cannot finish the output")?;
    inner.flush().map_err(|e| e.kind().to_string())?;
    Ok((entries, elements))
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
