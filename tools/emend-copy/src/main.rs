//! emend-copy: opens a zip archive with the emend layer and writes it back.
//!
//! `specs/0008-zip-layer.md`, "Harness runs", and `specs/0009-package-
//! layer.md`, "Harness runs". At most ONE operation per invocation, given
//! after the two paths:
//!
//! - none: no edits, so the output must be byte-identical to the input and the
//!   harness must report every entry `identical` with an empty flags column.
//! - `--replace <name>`: the named entry is replaced by its OWN decompressed
//!   bytes, so the harness must report that entry `recompressed` (or
//!   `identical`) and every other entry `identical` and unflagged.
//! - `--xml-null-rewrite`: every entry whose name ends in `.xml` or `.rels`,
//!   ASCII case-insensitively, is replaced by its own bytes passed through
//!   `emend::xml::Stream`, the mark when present followed by every token's raw
//!   bytes. Each such entry must come back `recompressed` or `identical` and
//!   never `changed`, and every other entry `identical` and unflagged: the
//!   rewrite is the identity, so this is what proves the token stream loses
//!   nothing across the corpus.
//!
//! The entry's bytes go to a temporary file and are handed to the layer as a
//! reader, through the `io::copy` wrapper, rather than being collected into
//! memory: a corpus entry can be tens of megabytes and `usize` is 32 bits on
//! the target that matters.
//!
//! Exit 0 on success, 2 on any error, with the message on stderr as
//! `error: ...`, matching `xlsx-diff` and `passthrough`. Messages name at
//! most an entry index: never a path, never an entry's content.

use std::env;
use std::ffi::OsString;
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Seek, SeekFrom, Write};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::process::{self, ExitCode};

use emend::archive::{Archive, Edits, Error as LayerError, replacement_from_reader};
use emend::xml::{BYTE_ORDER_MARK, Stream};

const USAGE: &str =
    "usage: emend-copy <input.xlsx> <output.xlsx> [--replace <name> | --xml-null-rewrite]";

/// The one operation an invocation may carry, chosen by the arguments after
/// the two paths. At most one: the parser refuses two at once, an unknown
/// flag, and a flag before the paths, each with the usage line.
enum Operation {
    /// No edits: a verbatim round trip.
    None,
    /// Replace one named entry with its own bytes.
    Replace(Vec<u8>),
    /// Replace every XML-named entry with its own bytes through the stream.
    XmlNullRewrite,
}

/// Exit status for every error, as in `xlsx-diff` and `passthrough`.
const EXIT_ERROR: u8 = 2;

/// Everything that stops a copy.
enum Error {
    Usage,
    ReadInput(io::ErrorKind),
    Layer(LayerError),
    NoFileName,
    Create(io::ErrorKind),
    Finish(io::ErrorKind),
    Rename(io::ErrorKind),
    NoSuchName,
    Extract(io::ErrorKind),
    Rewrite(io::ErrorKind),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Usage => f.write_str(USAGE),
            Error::ReadInput(kind) => write!(f, "cannot read the input file: {kind}"),
            Error::Layer(e) => write!(f, "{e}"),
            Error::NoFileName => f.write_str("the output path has no file name"),
            Error::Create(kind) => write!(f, "cannot create the output file: {kind}"),
            Error::Finish(kind) => write!(f, "cannot finish the output file: {kind}"),
            Error::Rename(kind) => write!(f, "cannot move the output into place: {kind}"),
            // The name is NOT echoed: it comes from the command line, but the
            // rule for this tool is that a message names at most an index.
            Error::NoSuchName => f.write_str("the archive has no entry with that name"),
            Error::Extract(kind) => write!(f, "cannot extract the entry to replace: {kind}"),
            Error::Rewrite(kind) => write!(f, "cannot write the rewritten part: {kind}"),
        }
    }
}

impl From<LayerError> for Error {
    fn from(e: LayerError) -> Self {
        Error::Layer(e)
    }
}

fn main() -> ExitCode {
    let args: Vec<_> = env::args_os().skip(1).collect();
    // The operation comes after the two paths, and there is at most one. Any
    // other shape is the usage error: two operations at once and an operation
    // before the paths both leave a flag where a path or nothing belongs.
    let result = match args.as_slice() {
        [input, output] => run(Path::new(input), Path::new(output), Operation::None),
        [input, output, flag] if flag == "--xml-null-rewrite" => run(
            Path::new(input),
            Path::new(output),
            Operation::XmlNullRewrite,
        ),
        [input, output, flag, name] if flag == "--replace" => run(
            Path::new(input),
            Path::new(output),
            Operation::Replace(name.as_bytes().to_vec()),
        ),
        _ => Err(Error::Usage),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            let _ = writeln!(io::stderr().lock(), "error: {err}");
            ExitCode::from(EXIT_ERROR)
        }
    }
}

fn run(input: &Path, output: &Path, operation: Operation) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut archive = Archive::open(BufReader::new(file))?;

    // Every replacement's bytes go to a temporary file first: the layer takes
    // a reader for the whole write, and holding one that borrows the archive
    // would conflict with it.
    let mut edits = Edits::new();
    let extracted = match operation {
        Operation::None => Vec::new(),
        Operation::Replace(name) => {
            let index = archive.index_of(&name).ok_or(Error::NoSuchName)?;
            let (mut file, guard) = temporary(output, "-entry")?;
            io::copy(&mut archive.read(index)?, &mut file).map_err(|e| Error::Extract(e.kind()))?;
            file.seek(SeekFrom::Start(0))
                .map_err(|e| Error::Extract(e.kind()))?;
            edits.replace(index, replacement_from_reader(file));
            vec![guard]
        }
        Operation::XmlNullRewrite => {
            let indices: Vec<usize> = archive
                .entries()
                .iter()
                .enumerate()
                .filter(|(_, entry)| is_xml_name(entry.name()))
                .map(|(index, _)| index)
                .collect();
            let mut guards = Vec::with_capacity(indices.len());
            for index in indices {
                // A separate temporary per entry: they are all handed to the
                // layer and read during the one write.
                let (mut file, guard) = temporary(output, &format!("-xml{index}"))?;
                guards.push(guard);
                rewrite_part(&mut archive, index, &mut file)?;
                file.seek(SeekFrom::Start(0))
                    .map_err(|e| Error::Rewrite(e.kind()))?;
                edits.replace(index, replacement_from_reader(file));
            }
            guards
        }
    };

    let temporary = temporary_path(output, "")?;
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|e| Error::Create(e.kind()))?;

    // From here the temporary file is this run's own, so a failure removes it
    // and leaves no partial output where the harness would read one.
    let result = write(&mut archive, file, edits)
        .and_then(|()| fs::rename(&temporary, output).map_err(|e| Error::Rename(e.kind())));
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    drop(extracted);
    result
}

/// Whether an entry name is one the null rewrite touches: `.xml` or `.rels`,
/// ASCII case-insensitively (`specs/0009-package-layer.md`, "Harness runs").
///
/// A directory entry's name ends in `/`, so none matches.
fn is_xml_name(name: &[u8]) -> bool {
    ends_with_ascii_ci(name, b".xml") || ends_with_ascii_ci(name, b".rels")
}

fn ends_with_ascii_ci(name: &[u8], suffix: &[u8]) -> bool {
    name.len() >= suffix.len() && name[name.len() - suffix.len()..].eq_ignore_ascii_case(suffix)
}

/// Passes one entry's bytes through the token stream and writes them out.
///
/// This is the null rewrite: the mark when present, then every token's raw
/// bytes in order, which is the input byte for byte. It streams, so a 63.7 MB
/// part never lands in memory.
fn rewrite_part<R: io::Read + io::Seek>(
    archive: &mut Archive<R>,
    index: usize,
    file: &mut File,
) -> Result<(), Error> {
    let mut out = BufWriter::new(file);
    let mut stream = Stream::new(archive.read(index)?, index)?;
    if stream.bom() {
        out.write_all(&BYTE_ORDER_MARK)
            .map_err(|e| Error::Rewrite(e.kind()))?;
    }
    while let Some(token) = stream.next()? {
        out.write_all(token.raw)
            .map_err(|e| Error::Rewrite(e.kind()))?;
    }
    out.flush().map_err(|e| Error::Rewrite(e.kind()))?;
    Ok(())
}

/// Creates one of this run's temporary files, and the guard that removes it.
fn temporary(output: &Path, kind: &str) -> Result<(File, Removed), Error> {
    let path = temporary_path(output, kind)?;
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(&path)
        .map_err(|e| Error::Create(e.kind()))?;
    Ok((file, Removed(path)))
}

/// Writes the archive back, and reports a buffering failure rather than losing
/// it in a drop.
fn write<R: io::Read + io::Seek>(
    archive: &mut Archive<R>,
    file: File,
    edits: Edits,
) -> Result<(), Error> {
    let mut out = BufWriter::new(file);
    archive.write_to(&mut out, edits)?;
    out.into_inner()
        .map_err(|e| Error::Finish(e.error().kind()))?
        .sync_all()
        .map_err(|e| Error::Finish(e.kind()))?;
    Ok(())
}

/// A temporary file removed when it goes out of scope, whatever stopped the
/// run.
struct Removed(PathBuf);

impl Drop for Removed {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

/// A name beside the target, hidden and unique to this process, so a
/// concurrent run cannot collide and a crash leaves nothing the harness
/// would mistake for an output.
///
/// With an empty `kind` this is the output's own temporary,
/// `.<name>.emend-copy-<pid>.tmp`, exactly the form P1 made and
/// `specs/0008-zip-layer.md` names; `-entry` marks the extracted entry's file,
/// a different file that the P2 rulings keep distinct from it. Both carry the
/// leading dot and the `.tmp` suffix the runner's leftover glob matches
/// (`specs/0006-passthrough.md`, the P1 ruling of 2026-09-16).
fn temporary_path(output: &Path, kind: &str) -> Result<PathBuf, Error> {
    let name = output.file_name().ok_or(Error::NoFileName)?;
    let mut temporary = OsString::from(".");
    temporary.push(name);
    temporary.push(format!(".emend-copy{kind}-{}.tmp", process::id()));
    Ok(output.with_file_name(temporary))
}
