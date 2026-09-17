//! probe-emend-copy: the size baseline's consumer of emend's zip layer
//! (`specs/0008-zip-layer.md`, "Size"). It opens the archive named on the
//! command line with the layer, writes it back with no edits to the second
//! path, and prints its entry count.
//!
//! Built as the baseline's probes are, its size minus `probe-zip` is what the
//! zip layer's no-edit path costs. Its delta over `probe-zip-xml` is NOT the
//! budget's figure: the budget is judged on `probe-emend-edit-xml`, which
//! keeps the edit paths alive and links both crates (`specs/0008-zip-
//! layer.md`, "Size", and the maintainer's ruling of 2026-09-17 recorded
//! there). This probe stands beside it as the no-edit comparison.

use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};
use std::process::ExitCode;

use emend::{Archive, Edits};

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let [input, output] = args.as_slice() else {
        let _ = writeln!(
            io::stderr(),
            "error: usage: probe-emend-copy <archive> <output>"
        );
        return ExitCode::from(2);
    };
    match run(input, output) {
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

fn run(input: &std::ffi::OsStr, output: &std::ffi::OsStr) -> Result<u64, String> {
    let file = File::open(input).map_err(|e| e.kind().to_string())?;
    let mut archive = Archive::open(BufReader::new(file)).map_err(|e| e.to_string())?;
    let entries = archive.entries().len() as u64;
    let out = File::create(output).map_err(|e| e.kind().to_string())?;
    let mut writer = BufWriter::new(out);
    archive
        .write_to(&mut writer, Edits::new())
        .map_err(|e| e.to_string())?;
    writer.flush().map_err(|e| e.kind().to_string())?;
    Ok(entries)
}
