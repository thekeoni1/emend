//! probe-emend-edit: the size probe for a consumer of emend's zip layer that
//! EDITS (`specs/0008-zip-layer.md`, "Size"). It opens the archive named on
//! the command line, replaces one named entry with its own bytes, removes a
//! second, adds a third, writes the result to the output path, and prints the
//! entry count it read.
//!
//! Every edit comes from the command line, so the compiler cannot prove any
//! edit path dead and drop it, as it can for `probe-emend-copy`, which never
//! edits. The size budget is judged on this probe.

use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::os::unix::ffi::OsStrExt;
use std::process::ExitCode;

use emend::{Archive, Edits, Replacement};

const USAGE: &str = "usage: probe-emend-edit <archive> <output> <entry to replace> <entry to remove> <entry to add>";

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
        Ok(entries) => {
            let _ = writeln!(io::stdout(), "entries {entries}");
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

fn run(input: &std::ffi::OsStr, output: &std::ffi::OsStr, names: &Names) -> Result<u64, String> {
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
    Ok(entries)
}

/// A replacement that writes these bytes.
fn bytes(data: Vec<u8>) -> Replacement {
    Box::new(move |out: &mut dyn Write| out.write_all(&data))
}
