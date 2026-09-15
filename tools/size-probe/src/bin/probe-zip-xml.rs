//! probe-zip-xml: the size baseline's consumer of `zip` and `quick-xml`
//! (`specs/0006-passthrough.md`, "Size baseline"). It opens the archive named
//! on the command line, prints its entry count, then parses the named entry
//! with `quick-xml` in streaming mode and prints its element count.

use std::fs::File;
use std::io::{self, BufReader, Write};
use std::process::ExitCode;

use quick_xml::Reader;
use quick_xml::events::Event;
use zip::ZipArchive;

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let [path, entry] = args.as_slice() else {
        let _ = writeln!(
            io::stderr(),
            "error: usage: probe-zip-xml <archive> <entry>"
        );
        return ExitCode::from(2);
    };
    match run(path, entry) {
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

fn run(path: &std::ffi::OsStr, entry: &std::ffi::OsStr) -> Result<(u64, u64), String> {
    let file = File::open(path).map_err(|e| e.kind().to_string())?;
    let mut archive =
        ZipArchive::new(BufReader::new(file)).map_err(|_| "not a readable zip archive")?;
    let entries = archive.len() as u64;
    let name = entry.to_str().ok_or("the entry name is not UTF-8")?;
    let part = archive
        .by_name(name)
        .map_err(|_| "no readable entry of that name")?;
    let mut reader = Reader::from_reader(BufReader::new(part));
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
    Ok((entries, elements))
}
