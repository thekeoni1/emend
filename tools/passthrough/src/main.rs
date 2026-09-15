//! passthrough: copies every entry of a zip archive, in order, with the zip
//! crate's raw copy, and the archive comment, into a new archive
//! (`specs/0006-passthrough.md`). It edits nothing; what the zip crate's
//! writer changes on the way is what T3 measures.

use std::env;
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::{self, ExitCode};
use std::thread;
use std::time::Duration;

use zip::result::ZipError;
use zip::{ZipArchive, ZipWriter};

const USAGE: &str = "usage: passthrough <input.xlsx> <output.xlsx>";

/// Exit status for every error, as in `xlsx-diff`.
const EXIT_ERROR: u8 = 2;

/// Test-only fault injection for the runner's tests (`sleep-copying` added in
/// T3 P2 on planning's ruling, for the leftover temporary file). It is read from
/// `PASSTHROUGH_FAULT` and refused unless `PASSTHROUGH_FAULT_ALLOWED=1` is
/// also set, so a stray value cannot change a corpus run.
const FAULT_VARIABLE: &str = "PASSTHROUGH_FAULT";
const FAULT_ALLOWED_VARIABLE: &str = "PASSTHROUGH_FAULT_ALLOWED";

/// Bytes the `alloc` fault allocates and touches.
const FAULT_ALLOCATION: usize = 512 * 1024 * 1024;

enum Fault {
    Sleep,
    Panic,
    Abort,
    Alloc,
    Exit3,
    SleepCopying,
}

/// Everything that stops a copy. The messages name at most an entry index.
/// They never carry entry content, file paths or text from the zip crate.
enum Error {
    FaultNotAllowed,
    FaultUnknown,
    ReadInput(io::ErrorKind),
    NotZip,
    Unsupported,
    Entry(u64),
    Copy(u64, Option<io::ErrorKind>),
    Comment,
    NoFileName,
    Create(io::ErrorKind),
    Finish(Option<io::ErrorKind>),
    Rename(io::ErrorKind),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::FaultNotAllowed => write!(
                f,
                "{FAULT_VARIABLE} is set without {FAULT_ALLOWED_VARIABLE}=1 (test-only)"
            ),
            Error::FaultUnknown => write!(f, "{FAULT_VARIABLE} has an unknown value"),
            Error::ReadInput(kind) => write!(f, "cannot read the input file: {kind}"),
            Error::NotZip => f.write_str("the input file is not a readable zip archive"),
            Error::Unsupported => f.write_str("the input archive uses an unsupported zip feature"),
            Error::Entry(index) => write!(f, "cannot read entry {index} of the input archive"),
            Error::Copy(index, Some(kind)) => write!(f, "cannot copy entry {index}: {kind}"),
            Error::Copy(index, None) => write!(f, "the zip writer refused entry {index}"),
            Error::Comment => f.write_str("the zip writer refused the archive comment"),
            Error::NoFileName => f.write_str("the output path has no file name"),
            Error::Create(kind) => write!(f, "cannot create the output file: {kind}"),
            Error::Finish(Some(kind)) => write!(f, "cannot finish the output archive: {kind}"),
            Error::Finish(None) => {
                f.write_str("the zip writer could not finish the output archive")
            }
            Error::Rename(kind) => write!(f, "cannot move the output into place: {kind}"),
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<_> = env::args_os().skip(1).collect();
    let result = match args.as_slice() {
        [input, output] => run(Path::new(input), Path::new(output)),
        _ => {
            let _ = writeln!(io::stderr().lock(), "error: {USAGE}");
            return ExitCode::from(EXIT_ERROR);
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            let _ = writeln!(io::stderr().lock(), "error: {err}");
            ExitCode::from(EXIT_ERROR)
        }
    }
}

/// Reads the fault variables. A set fault without the allow variable is an
/// error, whatever its value.
fn fault() -> Result<Option<Fault>, Error> {
    let Some(value) = env::var_os(FAULT_VARIABLE) else {
        return Ok(None);
    };
    if env::var_os(FAULT_ALLOWED_VARIABLE).as_deref() != Some(OsStr::new("1")) {
        return Err(Error::FaultNotAllowed);
    }
    match value.to_str() {
        Some("sleep") => Ok(Some(Fault::Sleep)),
        Some("panic") => Ok(Some(Fault::Panic)),
        Some("abort") => Ok(Some(Fault::Abort)),
        Some("alloc") => Ok(Some(Fault::Alloc)),
        Some("exit3") => Ok(Some(Fault::Exit3)),
        Some("sleep-copying") => Ok(Some(Fault::SleepCopying)),
        _ => Err(Error::FaultUnknown),
    }
}

/// Runs a fault after the input is open and before the output exists, apart
/// from `sleep-copying`, which `copy` runs once the first entry is copied.
fn inject(fault: Fault) {
    match fault {
        Fault::Sleep => thread::sleep(Duration::from_secs(30)),
        Fault::Panic => panic!("passthrough fault injection: panic"),
        Fault::Abort => process::abort(),
        Fault::Alloc => {
            let mut block = vec![0u8; FAULT_ALLOCATION];
            for byte in block.iter_mut().step_by(4096) {
                *byte = 1;
            }
            std::hint::black_box(&block);
        }
        Fault::Exit3 => process::exit(3),
        Fault::SleepCopying => {}
    }
}

fn run(input: &Path, output: &Path) -> Result<(), Error> {
    let fault = fault()?;
    let sleep_copying = matches!(fault, Some(Fault::SleepCopying));
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut archive = ZipArchive::new(BufReader::new(file)).map_err(open_error)?;
    if let Some(fault) = fault {
        inject(fault);
    }
    let temporary = temporary_path(output)?;
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|e| Error::Create(e.kind()))?;
    // From here on the temporary file is this run's own, so a failure
    // removes it and leaves no partial output.
    let result = copy(&mut archive, file, sleep_copying)
        .and_then(|()| fs::rename(&temporary, output).map_err(|e| Error::Rename(e.kind())));
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result
}

/// A name beside the target, hidden and unique to this process.
fn temporary_path(output: &Path) -> Result<PathBuf, Error> {
    let name = output.file_name().ok_or(Error::NoFileName)?;
    let mut temporary = OsString::from(".");
    temporary.push(name);
    temporary.push(format!(".passthrough-{}.tmp", process::id()));
    Ok(output.with_file_name(temporary))
}

/// With `sleep_copying` (a test-only fault), sleeps after the first entry is
/// copied, or before the comment when there is none, while the temporary file
/// exists.
fn copy(
    archive: &mut ZipArchive<BufReader<File>>,
    file: File,
    sleep_copying: bool,
) -> Result<(), Error> {
    let mut writer = ZipWriter::new(BufWriter::new(file));
    for index in 0..archive.len() {
        let number = index as u64;
        let entry = archive
            .by_index_raw(index)
            .map_err(|_| Error::Entry(number))?;
        writer
            .raw_copy_file(entry)
            .map_err(|e| Error::Copy(number, io_kind(&e)))?;
        if sleep_copying && index == 0 {
            thread::sleep(Duration::from_secs(30));
        }
    }
    if sleep_copying && archive.is_empty() {
        thread::sleep(Duration::from_secs(30));
    }
    writer
        .set_raw_comment(archive.comment().into())
        .map_err(|_| Error::Comment)?;
    let buffered = writer.finish().map_err(|e| Error::Finish(io_kind(&e)))?;
    buffered
        .into_inner()
        .map_err(|e| Error::Finish(Some(e.error().kind())))?;
    Ok(())
}

fn io_kind(err: &ZipError) -> Option<io::ErrorKind> {
    match err {
        ZipError::Io(e) => Some(e.kind()),
        _ => None,
    }
}

/// Maps an error from the zip crate while opening the input to a message
/// that carries no crate text.
fn open_error(err: ZipError) -> Error {
    match err {
        ZipError::Io(e) => Error::ReadInput(e.kind()),
        ZipError::UnsupportedArchive(_) | ZipError::CompressionMethodNotSupported(_) => {
            Error::Unsupported
        }
        _ => Error::NotZip,
    }
}
