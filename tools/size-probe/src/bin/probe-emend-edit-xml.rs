//! probe-emend-edit-xml: the size probe the budget is judged on
//! (`specs/0008-zip-layer.md`, "Size", from P3's repair round). It is
//! `probe-emend-edit` with the streaming `quick-xml` element count that
//! `probe-zip-xml` makes, run over the replaced entry's bytes, so it is the
//! consumer the budget describes: one that already carries `zip` and
//! `quick-xml` and reads entries. It prints the entry count it read and the
//! element count.
//!
//! Every edit comes from the command line, so the compiler cannot prove any
//! edit path dead and drop it.

use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::os::unix::ffi::OsStrExt;
use std::process::ExitCode;

use emend::{Archive, Edits, Replacement};
use quick_xml::Reader;
use quick_xml::events::Event;

const USAGE: &str = "usage: probe-emend-edit-xml <archive> <output> <entry to replace> <entry to remove> <entry to add>";

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let [input, output, replace, remove, add] = args.as_slice() else {
        let _ = writeln!(io::stderr(), "error: {USAGE}");
        return ExitCode::from(2);
    };
    let names = Names {
        replace: replace.as_bytes(),
        remove: remove.as_bytes(),
        add: add.as_bytes(),
    };
    match run(input, output, &names) {
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

/// The three entry names the edits are made from.
struct Names<'a> {
    replace: &'a [u8],
    remove: &'a [u8],
    add: &'a [u8],
}

fn run(
    input: &std::ffi::OsStr,
    output: &std::ffi::OsStr,
    names: &Names,
) -> Result<(u64, u64), String> {
    let file = File::open(input).map_err(|e| e.kind().to_string())?;
    let mut archive = Archive::open(BufReader::new(file)).map_err(|e| e.to_string())?;
    let entries = archive.entries().len() as u64;

    let replace = archive
        .index_of(names.replace)
        .ok_or("no entry to replace by that name")?;
    let remove = archive
        .index_of(names.remove)
        .ok_or("no entry to remove by that name")?;

    // The replaced entry's own bytes, read through the layer, so the
    // decompressing path is linked as an editing consumer links it.
    let mut own = Vec::new();
    archive
        .read(replace)
        .map_err(|e| e.to_string())?
        .read_to_end(&mut own)
        .map_err(|e| e.kind().to_string())?;
    let elements = count_elements(&own)?;
    let added = names.add.to_vec();

    let mut edits = Edits::new();
    edits.replace(replace, bytes(own));
    edits.remove(remove);
    edits.add(names.add.to_vec(), bytes(added));

    let out = File::create(output).map_err(|e| e.kind().to_string())?;
    let mut writer = BufWriter::new(out);
    archive
        .write_to(&mut writer, edits)
        .map_err(|e| e.to_string())?;
    writer.flush().map_err(|e| e.kind().to_string())?;
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

/// A replacement that writes these bytes.
fn bytes(data: Vec<u8>) -> Replacement {
    Box::new(move |out: &mut dyn Write| out.write_all(&data))
}
