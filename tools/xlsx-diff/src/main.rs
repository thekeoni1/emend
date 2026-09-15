//! xlsx-diff command line: argument handling and the exit status only. The
//! comparison lives in the library (`src/lib.rs`).

use std::io::{self, Write};
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let mut stderr = io::stderr().lock();
    let [input, output] = args.as_slice() else {
        let _ = writeln!(stderr, "{}", xlsx_diff::USAGE);
        return ExitCode::from(xlsx_diff::EXIT_ERROR);
    };
    let mut stdout = io::stdout().lock();
    let mut status = xlsx_diff::run(
        Path::new(input),
        Path::new(output),
        &mut stdout,
        &mut stderr,
    );
    if let Err(err) = stdout.flush() {
        let _ = writeln!(stderr, "error: cannot write the rows: {}", err.kind());
        status = xlsx_diff::EXIT_ERROR;
    }
    ExitCode::from(status)
}
