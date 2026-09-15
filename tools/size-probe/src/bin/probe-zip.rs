//! probe-zip: the size baseline's consumer of `zip` alone
//! (`specs/0006-passthrough.md`, "Size baseline"). It opens the archive named
//! on the command line and prints its entry count.

use std::fs::File;
use std::io::{self, BufReader, Write};
use std::process::ExitCode;

use zip::ZipArchive;

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let [path] = args.as_slice() else {
        let _ = writeln!(io::stderr(), "error: usage: probe-zip <archive>");
        return ExitCode::from(2);
    };
    let archive = File::open(path)
        .map_err(|e| e.kind().to_string())
        .and_then(|file| {
            ZipArchive::new(BufReader::new(file)).map_err(|_| "not a readable zip archive".into())
        });
    match archive {
        Ok(archive) => {
            let _ = writeln!(io::stdout(), "entries {}", archive.len() as u64);
            ExitCode::SUCCESS
        }
        Err(message) => {
            let _ = writeln!(io::stderr(), "error: {message}");
            ExitCode::from(2)
        }
    }
}
