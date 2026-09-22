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
//! - `--drop-calc-chain`: the recipe of `specs/0009-package-layer.md`,
//!   "Module `package`", recorded by `Package::drop_calc_chain`: the
//!   workbook's `calcChain` relationship, the `Override` for its target when
//!   the package has one, and the entry when the package has one, each its own
//!   removal and nothing implicit. A VALID package with no chain records no
//!   edit, so its output must be byte-identical to its input; a package with
//!   one must come back with those three gone, the two rewritten parts
//!   `changed` and every other entry `identical` and unflagged. The content
//!   types part is read before the chain is looked for, so a package without
//!   one is refused here as `--survey` refuses it, whether or not its edit
//!   would be empty (part 2 P3 verification ruling 6).
//! - `--survey`: no edit either, so the output must be byte-identical again,
//!   and one line goes to stdout: the word `survey` then tab-separated
//!   `key=value` pairs, COUNTS ONLY. The package layer's whole map is walked
//!   to produce them, so a corpus file whose content types, relationships,
//!   targets or sheets the layer cannot read is an error (exit 2) and the
//!   runner records the run as one. The line names no entry and no value from
//!   the workbook: a count is not content.
//! - `--shared-strings`: no edit either, so the output must be
//!   byte-identical again, and one line goes to stdout: the word `strings`
//!   then tab-separated `key=value`, `items` (the table's length) and
//!   `bytes` (the sum of the items' UTF-8 lengths). Two counts and nothing
//!   else: a shared string IS content, so no item, no length of one and no
//!   fragment of one is printed. A workbook the layer cannot read the table
//!   of is an error (exit 2), as `--survey` is.
//! - `--cells`: no edit either, so the output must be byte-identical again,
//!   and one line goes to stdout: the word `cells` then tab-separated
//!   `key=value`, the keys `specs/0010-reading-cells.md`, "Harness runs",
//!   lists. Every sheet of the workbook is streamed in `<sheets>` order.
//!   COUNTS ONLY, for the same reason `--shared-strings` prints two: a
//!   cell's value IS content, so no value, no formula and no shared string
//!   is printed, nor the length of one. A failed `resolve` is counted as
//!   `unresolved` and the run carries on, so that count is the whole
//!   workbook's; an error from any other parse is exit 2 with the message.
//! - `--edit-sample`: the sample cell edit of `specs/0011-editing-cells.md`,
//!   "Harness runs", on the first sheet of kind `Worksheet`: its first
//!   `Number` or `SharedString` cell with a value and no formula set to
//!   42.5, the second such cell cleared, and a text and a boolean added in a
//!   new row after the last. The output is reopened and the sheet read again,
//!   and one line goes to stdout: the word `edit` then tab-separated
//!   `key=value`, COUNTS ONLY, for the reason `--cells` gives. A cell count
//!   after the write other than the one the edits imply is exit 2.
//! - `--edit-formula-sample`: the formula sample edit of the same section:
//!   the sheets of kind `Worksheet` are read in `<sheets>` order until a cell
//!   with a formula of kind `Normal` is found, that cell is set to
//!   `Number(1)` and the package written, so the write's rule drops the
//!   calculation chain when there is one. One line goes to stdout: the word
//!   `edit` then tab-separated `formula_replaced`, `chain_dropped`,
//!   `cells_after` and `read_back`, COUNTS ONLY. A cell count after the
//!   write other than the one before is exit 2. A workbook with no normal
//!   formula gets no edit, so its output is byte-identical, and every key of
//!   its line is 0.
//! - `--styles`: no edit, so the output must be byte-identical again, and one
//!   line goes to stdout: the word `styles` then tab-separated `key=value`,
//!   `xfs`, `num_fmts`, `custom`, `dated` and `date1904`, the keys
//!   `specs/0012-styles.md`, "Harness runs", lists. COUNTS ONLY: a format
//!   code is content, so no code, no font name and no length of either is
//!   printed. A workbook whose styles part the layer cannot read is exit 2
//!   with the message.
//! - `--style-sample`: the sample style edit of `specs/0012-styles.md`,
//!   "Harness runs": two cell formats are added, one over the first sampled
//!   cell's own format with the code `0.000` and one over no base with the
//!   built-in id 14; the sampled cell takes the first and a new cell one row
//!   past the last takes `Number(45000.5)` with the second. One line goes to
//!   stdout: the word `edit` then `styled`, `added`, `xfs_before`,
//!   `xfs_after`, `formats_added`, `cells_before`, `cells_after` and
//!   `read_back`. COUNTS ONLY, as `--styles` is: the format code is this
//!   tool's own constant and no cell's text, reference or format code is
//!   printed. `xfs_after` and `cells_after` are asserted against what the
//!   edits imply; either failing is exit 2 with the message and no output,
//!   as `--edit-sample`'s count assertion is.
//! - `--date-sample`: the sample dated cell of `specs/0012-styles.md`,
//!   "Harness runs", `--style-sample`'s new cell alone: one cell format is
//!   added over no base with the built-in id 22, and a new cell one row past
//!   the last takes the serial of 2024-02-29 12:00:00.000 in the workbook's
//!   own epoch with that format. One line goes to stdout: the word `edit`
//!   then `date1904`, `serial`, `added`, `xfs_before`, `xfs_after`,
//!   `cells_before`, `cells_after` and `read_back` ("Planning rulings during
//!   P3", ruling 5). The serial is this tool's own constant written, 45351.5
//!   or 43889.5, never a number read from the workbook. `xfs_after` and
//!   `cells_after` are asserted as `--style-sample`'s are. The tool enables
//!   the crate's `dates` feature for this operation.
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
use emend::cells::{CellRef, FormulaKind, Kind, Value};
use emend::dates::{DateTime, Epoch};
use emend::package::{CONTENT_TYPES_PART, Package, SheetKind, rels_part_source};
use emend::styles::NumberFormat;
use emend::xml::{BYTE_ORDER_MARK, Stream};

const USAGE: &str = "usage: emend-copy <input.xlsx> <output.xlsx> \
[--replace <name> | --xml-null-rewrite | --survey | --drop-calc-chain \
| --shared-strings | --cells | --edit-sample | --edit-formula-sample | --styles \
| --style-sample | --date-sample]";

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
    /// No edits, and count the package layer's map on stdout.
    Survey,
    /// Drop the calculation chain, the phase's first real edit.
    DropCalcChain,
    /// No edits, and count the shared strings table on stdout.
    SharedStrings,
    /// No edits, and count every cell of every sheet on stdout.
    Cells,
    /// The sample cell edit, and its counts on stdout.
    EditSample,
    /// The formula sample edit, and its counts on stdout.
    EditFormulaSample,
    /// No edits, and count the styles part on stdout.
    Styles,
    /// The sample style edit, and its counts on stdout.
    StyleSample,
    /// The sample dated cell, and its counts on stdout.
    DateSample,
}

/// Exit status for every error, as in `xlsx-diff` and `passthrough`.
const EXIT_ERROR: u8 = 2;

/// Everything that stops a copy.
enum Error {
    Usage,
    ReadInput(io::ErrorKind),
    ReadOutput(io::ErrorKind),
    Layer(LayerError),
    NoFileName,
    Create(io::ErrorKind),
    Finish(io::ErrorKind),
    Rename(io::ErrorKind),
    NoSuchName,
    Extract(io::ErrorKind),
    Rewrite(io::ErrorKind),
    Report(io::ErrorKind),
    NoWorksheet,
    NoNewRow,
    CellsAfter { expected: u64, found: u64 },
    XfsAfter { expected: u64, found: u64 },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Usage => f.write_str(USAGE),
            Error::ReadInput(kind) => write!(f, "cannot read the input file: {kind}"),
            Error::ReadOutput(kind) => write!(f, "cannot read the output file: {kind}"),
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
            Error::Report(kind) => write!(f, "cannot write the report line: {kind}"),
            Error::NoWorksheet => f.write_str("the workbook has no sheet of kind Worksheet"),
            Error::NoNewRow => f.write_str("the worksheet's last row is the last row there is"),
            Error::CellsAfter { expected, found } => write!(
                f,
                "the output's sheet holds {found} cell(s) and the edits imply {expected}"
            ),
            Error::XfsAfter { expected, found } => write!(
                f,
                "the output's styles part holds {found} cell format(s) and the edits imply \
                 {expected}"
            ),
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
        [input, output, flag] if flag == "--survey" => {
            run(Path::new(input), Path::new(output), Operation::Survey)
        }
        [input, output, flag] if flag == "--shared-strings" => run(
            Path::new(input),
            Path::new(output),
            Operation::SharedStrings,
        ),
        [input, output, flag] if flag == "--cells" => {
            run(Path::new(input), Path::new(output), Operation::Cells)
        }
        [input, output, flag] if flag == "--edit-sample" => {
            run(Path::new(input), Path::new(output), Operation::EditSample)
        }
        [input, output, flag] if flag == "--edit-formula-sample" => run(
            Path::new(input),
            Path::new(output),
            Operation::EditFormulaSample,
        ),
        [input, output, flag] if flag == "--styles" => {
            run(Path::new(input), Path::new(output), Operation::Styles)
        }
        [input, output, flag] if flag == "--style-sample" => {
            run(Path::new(input), Path::new(output), Operation::StyleSample)
        }
        [input, output, flag] if flag == "--date-sample" => {
            run(Path::new(input), Path::new(output), Operation::DateSample)
        }
        [input, output, flag] if flag == "--drop-calc-chain" => run(
            Path::new(input),
            Path::new(output),
            Operation::DropCalcChain,
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
    // The recipe records its edits on a `Package` and writes through it, so it
    // does not share the `Edits` path the other operations build.
    if matches!(operation, Operation::DropCalcChain) {
        return drop_calc_chain(input, output);
    }
    if matches!(operation, Operation::EditSample) {
        return edit_sample(input, output);
    }
    if matches!(operation, Operation::EditFormulaSample) {
        return edit_formula_sample(input, output);
    }
    if matches!(operation, Operation::StyleSample) {
        return style_sample(input, output);
    }
    if matches!(operation, Operation::DateSample) {
        return date_sample(input, output);
    }

    // Both reading operations run first, so a package the layer cannot read
    // stops the run before any output file exists. Each yields the one line
    // it prints, so the reporting below is the same code for either.
    let reported = match operation {
        Operation::Survey => Some(survey(input)?.line()),
        Operation::SharedStrings => Some(shared_strings_line(input)?),
        Operation::Cells => Some(cells_line(input)?),
        Operation::Styles => Some(styles_line(input)?),
        _ => None,
    };

    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut archive = Archive::open(BufReader::new(file))?;

    // Every replacement's bytes go to a temporary file first: the layer takes
    // a reader for the whole write, and holding one that borrows the archive
    // would conflict with it.
    let mut edits = Edits::new();
    let extracted = match operation {
        // Handled above, before any output file exists.
        Operation::DropCalcChain => unreachable!("the recipe has its own path"),
        Operation::EditSample => unreachable!("the sample edit has its own path"),
        Operation::EditFormulaSample => unreachable!("the formula sample has its own path"),
        Operation::StyleSample => unreachable!("the style sample has its own path"),
        Operation::DateSample => unreachable!("the date sample has its own path"),
        Operation::None
        | Operation::Survey
        | Operation::SharedStrings
        | Operation::Cells
        | Operation::Styles => Vec::new(),
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
    result?;

    if let Some(line) = reported {
        let mut out = io::stdout().lock();
        writeln!(out, "{line}").map_err(|e| Error::Report(e.kind()))?;
        out.flush().map_err(|e| Error::Report(e.kind()))?;
    }
    Ok(())
}

/// The survey's counts, in the order `specs/0009-package-layer.md`, "Harness
/// runs", lists its keys.
///
/// Every field is `u64`: a count is a count of entries, relationships or
/// sheets, and `usize` is 32 bits on the target that matters.
#[derive(Default)]
struct Survey {
    entries: u64,
    typed: u64,
    untyped: u64,
    defaults: u64,
    overrides: u64,
    overrides_unresolved: u64,
    rels_parts: u64,
    relationships: u64,
    external: u64,
    resolved: u64,
    unresolved: u64,
    case_only: u64,
    sheets: u64,
    sheets_resolved: u64,
    shared_strings: u64,
    styles: u64,
    calc_chain: u64,
    theme: u64,
}

impl Survey {
    /// The one line the tool prints: `survey` then tab-separated pairs.
    fn line(&self) -> String {
        let pairs = [
            ("entries", self.entries),
            ("typed", self.typed),
            ("untyped", self.untyped),
            ("defaults", self.defaults),
            ("overrides", self.overrides),
            ("overrides_unresolved", self.overrides_unresolved),
            ("rels_parts", self.rels_parts),
            ("relationships", self.relationships),
            ("external", self.external),
            ("resolved", self.resolved),
            ("unresolved", self.unresolved),
            ("case_only", self.case_only),
            ("sheets", self.sheets),
            ("sheets_resolved", self.sheets_resolved),
            ("shared_strings", self.shared_strings),
            ("styles", self.styles),
            ("calc_chain", self.calc_chain),
            ("theme", self.theme),
        ];
        let mut line = String::from("survey");
        for (key, value) in pairs {
            line.push('\t');
            line.push_str(key);
            line.push('=');
            line.push_str(&value.to_string());
        }
        line
    }
}

/// Drops the calculation chain, through the package layer.
///
/// [`Package::drop_calc_chain`] records the recipe of
/// `specs/0009-package-layer.md`, "Module `package`": the workbook's
/// `calcChain` relationship, the `Override` for its target when there is one,
/// the entry when there is one, each only when its subject is there. The
/// recipe lived here until `specs/0011-editing-cells.md` moved it into the
/// crate; the harness sees exactly the three changes it asks for, and a valid
/// package with no chain records nothing and comes back byte for byte, which
/// is the control for this operation.
///
/// The workbook and then the content types part are read before the chain is
/// looked for, so a package without a content types part is refused whether
/// or not it has a chain. Part 2 P3 measured the other order: with no chain
/// the recipe recorded no edit, `write_to` returned from `validate` at once,
/// and a package with no `[Content_Types].xml` was copied byte for byte while
/// `--survey` refused it. Planning ruled (part 2 P3 verification ruling 6)
/// that an operation which would refuse a package should refuse it whether or
/// not its edit turns out to be empty, so the two operations answer the same
/// question the same way. The method reads the content types part only when
/// there is a chain, so that read stays here.
fn drop_calc_chain(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    // Read for the refusals, in the order the recipe met them before it moved:
    // the answers are cached, so the method's own reads cost nothing extra.
    package.workbook()?;
    package.content_types()?;
    package.drop_calc_chain()?;

    let temporary = temporary_path(output, "")?;
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|e| Error::Create(e.kind()))?;
    // From here the temporary is this run's own, as in `run`: a failure
    // removes it rather than leaving a partial output for the harness.
    let result = write_package(package, file)
        .and_then(|()| fs::rename(&temporary, output).map_err(|e| Error::Rename(e.kind())));
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result
}

/// What the first read of the sample sheet finds.
struct Sampled {
    /// A: the first `Number` or `SharedString` cell with a value and no
    /// formula.
    first: Option<CellRef>,
    /// A's `s` and its value text, which the style sample needs: the `s` as
    /// the base of its addition, the text to compare the read-back against.
    /// Neither reaches a line or a message.
    first_style: Option<u32>,
    first_value: Option<String>,
    /// B: the second such cell, and whether it carried `s`.
    second: Option<(CellRef, bool)>,
    /// M: the largest row number, 0 for a sheet with no cells.
    max_row: u32,
    cells: u64,
}

/// Reads one sheet through `Package::cells` for what the sample edit needs.
fn sample<R: io::Read + io::Seek>(package: &mut Package<R>, part: &[u8]) -> Result<Sampled, Error> {
    let mut sampled = Sampled {
        first: None,
        first_style: None,
        first_value: None,
        second: None,
        max_row: 0,
        cells: 0,
    };
    for cell in package.cells(part)? {
        let cell = cell?;
        sampled.cells += 1;
        sampled.max_row = sampled.max_row.max(cell.reference.row);
        let candidate = matches!(cell.kind, Kind::Number | Kind::SharedString)
            && cell.value.is_some()
            && cell.formula.is_none();
        if candidate && sampled.first.is_none() {
            sampled.first = Some(cell.reference);
            sampled.first_style = cell.style;
            sampled.first_value = cell.value.clone();
        } else if candidate && sampled.second.is_none() {
            sampled.second = Some((cell.reference, cell.style.is_some()));
        }
    }
    Ok(sampled)
}

/// The sample cell edit of `specs/0011-editing-cells.md`, "Harness runs".
///
/// The first sheet of kind `Worksheet` in workbook order is read once for A,
/// B and M ([`Sampled`]); A is set to `Number(42.5)`, B cleared, `(M + 1, 1)`
/// set to `Text("emend")` and `(M + 1, 2)` to `Boolean(true)`, and the
/// package is written. The output is then opened and the same part read, for
/// `cells_after` and `read_back`. No reference, value or text reaches the
/// line or a message: a clear of a styled cell keeps it as `<c r s/>`, so
/// only an unstyled B is `dropped` and the count falls by it. A failure
/// after the output is in place, the count's assertion included, removes the
/// output, so a refused run leaves none.
fn edit_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::NoWorksheet)?;
    let before = sample(&mut package, &part)?;
    let new_row = before.max_row.checked_add(1).ok_or(Error::NoNewRow)?;
    let text = CellRef {
        row: new_row,
        col: 1,
    };
    let boolean = CellRef {
        row: new_row,
        col: 2,
    };

    if let Some(first) = before.first {
        package.set_cell(&part, first, Value::Number(42.5));
    }
    if let Some((second, _)) = before.second {
        package.clear_cell(&part, second);
    }
    package.set_cell(&part, text, Value::Text("emend".to_string()));
    package.set_cell(&part, boolean, Value::Boolean(true));

    let temporary = temporary_path(output, "")?;
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|e| Error::Create(e.kind()))?;
    // From here the temporary is this run's own, as in `run`.
    let result = write_package(package, file)
        .and_then(|()| fs::rename(&temporary, output).map_err(|e| Error::Rename(e.kind())));
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result?;

    let set = 2 + u64::from(before.first.is_some());
    let cleared = u64::from(before.second.is_some());
    let dropped = u64::from(matches!(before.second, Some((_, false))));
    let expected = before.cells + 2 - dropped;

    // The output is in place: a failure from here on removes it, as every
    // other refusal of the tool leaves no output (P2 verification ruling 3).
    let checked = read_back(output, &part, before.first, text, boolean, expected);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (cells_after, read_back) = checked?;
    edit_line(&[
        ("set", set),
        ("cleared", cleared),
        ("dropped", dropped),
        ("rows_added", 1),
        ("cells_before", before.cells),
        ("cells_after", cells_after),
        ("read_back", read_back),
    ])
}

/// Reopens the output and reads the edited sheet back: its cell count,
/// asserted equal to `expected`, the count the edits imply, and how many of
/// the three set values read back as written.
fn read_back(
    output: &Path,
    part: &[u8],
    first: Option<CellRef>,
    text: CellRef,
    boolean: CellRef,
    expected: u64,
) -> Result<(u64, u64), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut written = Package::open(BufReader::new(file))?;
    let mut cells_after = 0u64;
    let mut read_back = 0u64;
    for cell in written.cells(part)? {
        let cell = cell?;
        cells_after += 1;
        let at = cell.reference;
        if Some(at) == first && cell.number() == Some(42.5) {
            read_back += 1;
        }
        if at == text && cell.kind == Kind::InlineString && cell.value.as_deref() == Some("emend") {
            read_back += 1;
        }
        if at == boolean && cell.boolean() == Some(true) {
            read_back += 1;
        }
    }
    if cells_after != expected {
        return Err(Error::CellsAfter {
            expected,
            found: cells_after,
        });
    }
    Ok((cells_after, read_back))
}

/// The formula sample edit of `specs/0011-editing-cells.md`, "Harness runs".
///
/// The sheets of kind `Worksheet` are read in `<sheets>` order until one holds
/// a cell with a formula of kind `Normal`; that sheet is read to its end for
/// its count, the first such cell is set to `Number(1)` and the package
/// written, and the write's rule drops the calculation chain when the
/// workbook has one. The output is reopened and that sheet read back for the
/// line: `formula_replaced` (0 or 1), `chain_dropped` (1 when the input's
/// workbook names a chain and the output's does not, measured from both
/// reads), `cells_after` (the read-back count, asserted equal to the count
/// before: exit 2 when it differs) and `read_back` (1 when the edited cell
/// reads back as `number()` 1 with no formula). With no normal formula
/// nothing is recorded, so the output is byte-identical, and every key is 0:
/// a `cells_after` of 0 then means no sheet was read (P3 rulings 4 to 6). A
/// failure after the output is in place removes it, as `edit_sample`'s does.
fn edit_formula_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let workbook = package.workbook()?;
    let chain_before = workbook.calc_chain.is_some();
    let parts: Vec<Vec<u8>> = workbook
        .sheets
        .iter()
        .filter(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .collect();
    let mut found = None;
    for part in &parts {
        let mut first = None;
        let mut cells = 0u64;
        for cell in package.cells(part)? {
            let cell = cell?;
            cells += 1;
            let normal = cell
                .formula
                .as_ref()
                .is_some_and(|formula| formula.kind == FormulaKind::Normal);
            if normal && first.is_none() {
                first = Some(cell.reference);
            }
        }
        if let Some(reference) = first {
            found = Some((part.as_slice(), reference, cells));
            break;
        }
    }
    if let Some((part, reference, _)) = found {
        package.set_cell(part, reference, Value::Number(1.0));
    }

    let temporary = temporary_path(output, "")?;
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|e| Error::Create(e.kind()))?;
    // From here the temporary is this run's own, as in `run`.
    let result = write_package(package, file)
        .and_then(|()| fs::rename(&temporary, output).map_err(|e| Error::Rename(e.kind())));
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result?;

    // The output is in place: a failure from here on removes it, as in
    // `edit_sample`.
    let checked = formula_read_back(output, found);
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (chain_after, cells_after, read_back) = checked?;
    edit_line(&[
        ("formula_replaced", u64::from(found.is_some())),
        ("chain_dropped", u64::from(chain_before && !chain_after)),
        ("cells_after", cells_after),
        ("read_back", read_back),
    ])
}

/// Reopens the output for the formula sample's line: whether its workbook
/// still names a calculation chain; the edited sheet's cell count, asserted
/// equal to the count before; and whether the edited cell reads back as
/// `number()` 1 with no formula. Both counts are 0 when no sheet was edited.
fn formula_read_back(
    output: &Path,
    edited: Option<(&[u8], CellRef, u64)>,
) -> Result<(bool, u64, u64), Error> {
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut written = Package::open(BufReader::new(file))?;
    let chain_after = written.workbook()?.calc_chain.is_some();
    let Some((part, reference, before)) = edited else {
        return Ok((chain_after, 0, 0));
    };
    let mut cells_after = 0u64;
    let mut read_back = 0u64;
    for cell in written.cells(part)? {
        let cell = cell?;
        cells_after += 1;
        if cell.reference == reference && cell.number() == Some(1.0) && cell.formula.is_none() {
            read_back += 1;
        }
    }
    if cells_after != before {
        return Err(Error::CellsAfter {
            expected: before,
            found: cells_after,
        });
    }
    Ok((chain_after, cells_after, read_back))
}

/// The sample style edit of `specs/0012-styles.md`, "Harness runs".
///
/// The styles part is read for X (`cell_xfs.len()`) and F (`num_fmts.len()`),
/// and the first sheet of kind `Worksheet` in workbook order is read once
/// through `Package::cells` for A, the first `Number` or `SharedString` cell
/// with a value and no formula, and M, the largest row number ([`Sampled`]).
/// Two cell formats are then recorded: S1 over A's own `s` (`None` when A has
/// none or does not exist) with the code `0.000`, and S2 over no base with
/// the built-in id 14. A takes `Some(S1)` when it exists, and `(M + 1, 1)` is
/// set to `Number(45000.5)` with `Some(S2)`.
///
/// The output is written, reopened, and its styles part and the same sheet
/// read back. `xfs_after` is asserted equal to X + 2 and `cells_after` to
/// `cells_before + 1`; either failing removes the output, as `edit_sample`'s
/// assertion does, so a refused run leaves none. No reference, value, text or
/// format code reaches the line or a message: the line is counts only.
fn style_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let part = package
        .workbook()?
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::NoWorksheet)?;
    let styles = package.styles()?;
    let xfs_before = styles.cell_xfs.len() as u64;
    let formats_before = styles.num_fmts.len() as u64;
    let before = sample(&mut package, &part)?;
    let new_row = before.max_row.checked_add(1).ok_or(Error::NoNewRow)?;
    let new_cell = CellRef {
        row: new_row,
        col: 1,
    };

    // S1 takes A's own format as its base, so the addition keeps whatever
    // font, fill and border A had; S2 takes none, so it is the derived
    // format's plain shape. The code is one no corpus part defines, which is
    // what makes `formats_added` 1 in every workbook.
    let coded =
        package.add_cell_style(before.first_style, NumberFormat::Code("0.000".to_string()))?;
    let dated = package.add_cell_style(None, NumberFormat::BuiltIn(14))?;
    if let Some(first) = before.first {
        package.set_cell_style(&part, first, Some(coded));
    }
    package.set_cell(&part, new_cell, Value::Number(45000.5));
    package.set_cell_style(&part, new_cell, Some(dated));

    let temporary = temporary_path(output, "")?;
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|e| Error::Create(e.kind()))?;
    // From here the temporary is this run's own, as in `run`.
    let result = write_package(package, file)
        .and_then(|()| fs::rename(&temporary, output).map_err(|e| Error::Rename(e.kind())));
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result?;

    // The output is in place: a failure from here on removes it, as
    // `edit_sample`'s assertion does.
    let checked = style_read_back(
        output,
        &part,
        &before,
        new_cell,
        (coded, dated),
        (xfs_before + 2, before.cells + 1),
    );
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (xfs_after, formats_after, cells_after, read_back) = checked?;
    edit_line(&[
        ("styled", u64::from(before.first.is_some())),
        ("added", 1),
        ("xfs_before", xfs_before),
        ("xfs_after", xfs_after),
        (
            "formats_added",
            formats_after.saturating_sub(formats_before),
        ),
        ("cells_before", before.cells),
        ("cells_after", cells_after),
        ("read_back", read_back),
    ])
}

/// Reopens the output for the style sample's line: the styles part's cell
/// format count, asserted equal to `expected.0`; its `numFmt` count; the
/// edited sheet's cell count, asserted equal to `expected.1`; and how many of
/// the three read-backs hold: A's value text unchanged with `s` the new
/// coded format, the new cell's number with `s` the new dated format, and the
/// output's own number formats for the two new indices.
fn style_read_back(
    output: &Path,
    part: &[u8],
    before: &Sampled,
    new_cell: CellRef,
    added: (u32, u32),
    expected: (u64, u64),
) -> Result<(u64, u64, u64, u64), Error> {
    let (coded, dated) = added;
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut written = Package::open(BufReader::new(file))?;
    let styles = written.styles()?;
    let xfs_after = styles.cell_xfs.len() as u64;
    let formats_after = styles.num_fmts.len() as u64;
    let formats_read_back = styles.number_format(coded)
        == Some(NumberFormat::Code("0.000".to_string()))
        && styles.number_format(dated) == Some(NumberFormat::BuiltIn(14));
    if xfs_after != expected.0 {
        return Err(Error::XfsAfter {
            expected: expected.0,
            found: xfs_after,
        });
    }
    let mut cells_after = 0u64;
    let mut read_back = u64::from(formats_read_back);
    for cell in written.cells(part)? {
        let cell = cell?;
        cells_after += 1;
        let at = cell.reference;
        if Some(at) == before.first && cell.value == before.first_value && cell.style == Some(coded)
        {
            read_back += 1;
        }
        if at == new_cell && cell.number() == Some(45000.5) && cell.style == Some(dated) {
            read_back += 1;
        }
    }
    if cells_after != expected.1 {
        return Err(Error::CellsAfter {
            expected: expected.1,
            found: cells_after,
        });
    }
    Ok((xfs_after, formats_after, cells_after, read_back))
}

/// The date and time the date sample writes: its serial is 45351.5 in the
/// 1900 system and 43889.5 in the 1904 system (`specs/0012-styles.md`,
/// "Module `dates`", the pinned values plus noon).
const SAMPLE_DATE: DateTime = DateTime {
    year: 2024,
    month: 2,
    day: 29,
    hour: 12,
    minute: 0,
    second: 0,
    millisecond: 0,
};

/// The sample dated cell of `specs/0012-styles.md`, "Harness runs":
/// `--style-sample`'s new cell alone.
///
/// The first sheet of kind `Worksheet` in workbook order is read once for M
/// ([`Sampled`]); one cell format is added over no base with the built-in
/// id 22, and `(M + 1, 1)` is set to `Number` of [`SAMPLE_DATE`]'s serial in
/// the workbook's epoch with that format. The output is written, reopened,
/// and its styles part and the same sheet read back. `xfs_after` is asserted
/// equal to X + 1 and `cells_after` to `cells_before + 1`; either failing
/// removes the output, as the style sample's assertion does. The line
/// carries the serial, which is this tool's constant and no workbook's
/// content, and counts.
fn date_sample(input: &Path, output: &Path) -> Result<(), Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let workbook = package.workbook()?;
    let epoch = Epoch::of(workbook);
    let part = workbook
        .sheets
        .iter()
        .find(|sheet| sheet.kind == SheetKind::Worksheet)
        .map(|sheet| sheet.part.clone())
        .ok_or(Error::NoWorksheet)?;
    let xfs_before = package.styles()?.cell_xfs.len() as u64;
    let before = sample(&mut package, &part)?;
    let new_row = before.max_row.checked_add(1).ok_or(Error::NoNewRow)?;
    let new_cell = CellRef {
        row: new_row,
        col: 1,
    };
    let serial = SAMPLE_DATE
        .serial(epoch)
        .expect("2024-02-29 12:00 is a date in both epochs");

    let dated = package.add_cell_style(None, NumberFormat::BuiltIn(22))?;
    package.set_cell(&part, new_cell, Value::Number(serial));
    package.set_cell_style(&part, new_cell, Some(dated));

    let temporary = temporary_path(output, "")?;
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|e| Error::Create(e.kind()))?;
    // From here the temporary is this run's own, as in `run`.
    let result = write_package(package, file)
        .and_then(|()| fs::rename(&temporary, output).map_err(|e| Error::Rename(e.kind())));
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result?;

    // The output is in place: a failure from here on removes it, as
    // `style_sample`'s assertion does.
    let checked = date_read_back(
        output,
        &part,
        new_cell,
        (epoch, serial, dated),
        (xfs_before + 1, before.cells + 1),
    );
    if checked.is_err() {
        let _ = fs::remove_file(output);
    }
    let (xfs_after, cells_after, read_back) = checked?;
    let mut line = format!(
        "edit\tdate1904={}\tserial={serial}",
        u8::from(epoch == Epoch::Y1904)
    );
    for (key, value) in [
        ("added", 1),
        ("xfs_before", xfs_before),
        ("xfs_after", xfs_after),
        ("cells_before", before.cells),
        ("cells_after", cells_after),
        ("read_back", read_back),
    ] {
        line.push_str(&format!("\t{key}={value}"));
    }
    print_line(&line)
}

/// Reopens the output for the date sample's line: the styles part's cell
/// format count, asserted equal to `expected.0`; the edited sheet's cell
/// count, asserted equal to `expected.1`; and the read-back, 1 when the new
/// cell holds the serial written with `s` the new format and the serial
/// reads back through [`DateTime::from_serial`] in the same epoch as
/// [`SAMPLE_DATE`], else 0.
fn date_read_back(
    output: &Path,
    part: &[u8],
    new_cell: CellRef,
    written: (Epoch, f64, u32),
    expected: (u64, u64),
) -> Result<(u64, u64, u64), Error> {
    let (epoch, serial, dated) = written;
    let file = File::open(output).map_err(|e| Error::ReadOutput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let xfs_after = package.styles()?.cell_xfs.len() as u64;
    if xfs_after != expected.0 {
        return Err(Error::XfsAfter {
            expected: expected.0,
            found: xfs_after,
        });
    }
    let mut cells_after = 0u64;
    let mut read_back = 0u64;
    for cell in package.cells(part)? {
        let cell = cell?;
        cells_after += 1;
        if cell.reference == new_cell
            && cell.number() == Some(serial)
            && cell.style == Some(dated)
            && cell.number().and_then(|n| DateTime::from_serial(epoch, n)) == Some(SAMPLE_DATE)
        {
            read_back = 1;
        }
    }
    if cells_after != expected.1 {
        return Err(Error::CellsAfter {
            expected: expected.1,
            found: cells_after,
        });
    }
    Ok((xfs_after, cells_after, read_back))
}

/// Prints one `edit` line: the word, then tab-separated `key=value`.
fn edit_line(pairs: &[(&str, u64)]) -> Result<(), Error> {
    let mut line = String::from("edit");
    for (key, value) in pairs {
        line.push('\t');
        line.push_str(key);
        line.push('=');
        line.push_str(&value.to_string());
    }
    print_line(&line)
}

/// Prints `line` and a line feed on stdout, reporting a failure to write or
/// to flush.
fn print_line(line: &str) -> Result<(), Error> {
    let mut out = io::stdout().lock();
    writeln!(out, "{line}").map_err(|e| Error::Report(e.kind()))?;
    out.flush().map_err(|e| Error::Report(e.kind()))
}

/// Writes the package back, reporting a buffering failure rather than losing
/// it in a drop, as [`write()`] does for the archive.
fn write_package<R: io::Read + io::Seek>(package: Package<R>, file: File) -> Result<(), Error> {
    let mut out = BufWriter::new(file);
    package.write_to(&mut out)?;
    out.into_inner()
        .map_err(|e| Error::Finish(e.error().kind()))?
        .sync_all()
        .map_err(|e| Error::Finish(e.kind()))?;
    Ok(())
}

/// Whether an entry has exactly that name.
fn named(names: &[Vec<u8>], name: &[u8]) -> bool {
    names.iter().any(|entry| entry.as_slice() == name)
}

/// Whether an entry has that name after ASCII case folding, and not before.
///
/// This is what `case_only` counts: it measures the exact-bytes rule instead
/// of assuming it, so a corpus that needed case folding would say so.
fn named_case_only(names: &[Vec<u8>], name: &[u8]) -> bool {
    !named(names, name) && names.iter().any(|entry| entry.eq_ignore_ascii_case(name))
}

/// Reads the shared strings table and counts it.
///
/// Two numbers, both derived from the items and neither of them an item:
/// `items` is the table's length and `bytes` the sum of the items' UTF-8
/// lengths. The sum is taken over the public accessor rather than a method on
/// the table, because nothing in the library needs it; the tool wants it to
/// show the corpus's tables have been read whole and not truncated.
///
/// A workbook with no shared strings relationship gives an empty table, so
/// this prints `items=0 bytes=0` for one rather than refusing it: the absence
/// is not an error, and the run script knows which corpus files have a table.
fn shared_strings_line(input: &Path) -> Result<String, Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let table = package.shared_strings()?;
    let items = u64::from(table.len());
    // `u64` and not `usize`: the sum of a corpus table's items overflows
    // neither, but `usize` is 32 bits on the target that matters and every
    // count in this tool is already `u64`.
    let mut bytes = 0u64;
    for index in 0..table.len() {
        let item = table.get(index).unwrap_or_default();
        bytes += item.len() as u64;
    }
    Ok(format!("strings\titems={items}\tbytes={bytes}"))
}

/// The counts `--cells` prints, one field per key.
///
/// Every field is `u64` and not `usize`: the corpus's 2094601 cells fit
/// either, but `usize` is 32 bits on the target that matters and every count
/// in this tool is already `u64`.
#[derive(Default)]
struct CellCounts {
    sheets: u64,
    cells: u64,
    number: u64,
    shared: u64,
    inline: u64,
    formula_string: u64,
    boolean: u64,
    error: u64,
    date: u64,
    values: u64,
    empty: u64,
    formulas: u64,
    shared_master: u64,
    shared_follower: u64,
    array: u64,
    data_table: u64,
    numbers_parsed: u64,
    booleans_parsed: u64,
    resolved: u64,
    unresolved: u64,
    max_row: u64,
    max_col: u64,
    styled: u64,
}

impl CellCounts {
    /// The one line the tool prints: `cells` then tab-separated pairs, in the
    /// order `specs/0010-reading-cells.md`, "Harness runs", lists the keys.
    fn line(&self) -> String {
        let pairs = [
            ("sheets", self.sheets),
            ("cells", self.cells),
            ("number", self.number),
            ("shared", self.shared),
            ("inline", self.inline),
            ("formula_string", self.formula_string),
            ("boolean", self.boolean),
            ("error", self.error),
            ("date", self.date),
            ("values", self.values),
            ("empty", self.empty),
            ("formulas", self.formulas),
            ("shared_master", self.shared_master),
            ("shared_follower", self.shared_follower),
            ("array", self.array),
            ("data_table", self.data_table),
            ("numbers_parsed", self.numbers_parsed),
            ("booleans_parsed", self.booleans_parsed),
            ("resolved", self.resolved),
            ("unresolved", self.unresolved),
            ("max_row", self.max_row),
            ("max_col", self.max_col),
            ("styled", self.styled),
        ];
        let mut line = String::from("cells");
        for (key, value) in pairs {
            line.push('\t');
            line.push_str(key);
            line.push('=');
            line.push_str(&value.to_string());
        }
        line
    }
}

/// Streams every sheet of the workbook in `<sheets>` order and counts it.
///
/// COUNTS ONLY. A cell's value IS content, so no value, no formula, no item
/// of the shared strings table and no length of one is printed: the line
/// names the keys the spec lists and nothing else.
///
/// The shared strings table is read once, before any sheet, and held while
/// the cells stream through a mutable borrow of the same package: that is
/// what `Package::shared_strings` handing its table over OWNED is for.
///
/// A failed `resolve` is COUNTED and does not stop the run, so `unresolved`
/// is the whole count over the workbook rather than the first one found. An
/// error from any other parse is this tool's exit 2 with the message, as
/// `--survey` and `--shared-strings` are.
fn cells_line(input: &Path) -> Result<String, Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let table = package.shared_strings()?;
    // The part names are collected first: `cells` borrows the package for as
    // long as its reader streams, and the workbook is lent by the same
    // package.
    let parts: Vec<Vec<u8>> = package
        .workbook()?
        .sheets
        .iter()
        .map(|sheet| sheet.part.clone())
        .collect();
    let mut counted = CellCounts {
        sheets: parts.len() as u64,
        ..CellCounts::default()
    };
    for part in &parts {
        for cell in package.cells(part)? {
            let cell = cell?;
            counted.cells += 1;
            match cell.kind {
                Kind::Number => counted.number += 1,
                Kind::SharedString => counted.shared += 1,
                Kind::InlineString => counted.inline += 1,
                Kind::FormulaString => counted.formula_string += 1,
                Kind::Boolean => counted.boolean += 1,
                Kind::Error => counted.error += 1,
                Kind::Date => counted.date += 1,
            }
            if cell.value.is_some() {
                counted.values += 1;
            }
            // `empty` is neither value nor formula, which is both the `<c/>`
            // a producer writes for a styled blank and a `<c>` holding only
            // something this layer does not read.
            if cell.value.is_none() && cell.formula.is_none() {
                counted.empty += 1;
            }
            if let Some(formula) = &cell.formula {
                counted.formulas += 1;
                match formula.kind {
                    FormulaKind::Normal => {}
                    FormulaKind::Shared { master: true, .. } => counted.shared_master += 1,
                    FormulaKind::Shared { master: false, .. } => counted.shared_follower += 1,
                    FormulaKind::Array => counted.array += 1,
                    FormulaKind::DataTable => counted.data_table += 1,
                }
            }
            if cell.number().is_some() {
                counted.numbers_parsed += 1;
            }
            if cell.boolean().is_some() {
                counted.booleans_parsed += 1;
            }
            if cell.kind == Kind::SharedString {
                match table.resolve(&cell) {
                    Ok(_) => counted.resolved += 1,
                    Err(_) => counted.unresolved += 1,
                }
            }
            counted.max_row = counted.max_row.max(u64::from(cell.reference.row));
            counted.max_col = counted.max_col.max(u64::from(cell.reference.col));
            if cell.style.is_some() {
                counted.styled += 1;
            }
        }
    }
    Ok(counted.line())
}

/// Reads the workbook and its styles part and counts them.
///
/// `specs/0012-styles.md`, "Harness runs": `xfs` is the number of cell
/// formats, `num_fmts` of number formats, `custom` of those with an id of 164
/// or more, `dated` of the cell formats whose number format is a date by the
/// test below, and `date1904` is 1 when the workbook counts from 1904. COUNTS
/// ONLY: a format code is content, so none is printed.
fn styles_line(input: &Path) -> Result<String, Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let date1904 = u64::from(package.workbook()?.date1904);
    let styles = package.styles()?;
    let xfs = styles.cell_xfs.len() as u64;
    let num_fmts = styles.num_fmts.len() as u64;
    let custom = styles
        .num_fmts
        .iter()
        .filter(|format| format.id >= 164)
        .count() as u64;
    let mut dated = 0u64;
    for xf in 0..styles.cell_xfs.len() {
        // In range by construction: `xf` counts the cell formats read.
        let format = u32::try_from(xf)
            .ok()
            .and_then(|xf| styles.number_format(xf));
        if format.as_ref().is_some_and(is_date_format) {
            dated += 1;
        }
    }
    Ok(format!(
        "styles\txfs={xfs}\tnum_fmts={num_fmts}\tcustom={custom}\tdated={dated}\tdate1904={date1904}"
    ))
}

/// Whether a number format shows a date or a time, by the survey's test.
///
/// A built-in id among 14 to 22, 27 to 36, 45 to 47 and 50 to 58, the date,
/// time and locale-defined date ids of ECMA-376 Part 1, 18.8.30; or a code
/// that holds one of the letters `d m h y s`, in either case, once every
/// quoted run, every bracketed run and every backslash escape is removed,
/// over the whole code. This is the survey's test, as `specs/0012-styles.md`,
/// "Facts planning checked", states it (P1 ruling 1), and no library's: it
/// lives in this tool because the layer does not decide which numbers are
/// dates.
fn is_date_format(format: &NumberFormat) -> bool {
    match format {
        NumberFormat::BuiltIn(id) => matches!(id, 14..=22 | 27..=36 | 45..=47 | 50..=58),
        NumberFormat::Code(code) => is_date_code(code),
    }
}

/// The code half of [`is_date_format`].
///
/// A quoted run is a `"` and the next `"`, and a bracketed run a `[` and the
/// next `]`, with what lies between; a `"` or a `[` with no closing partner
/// starts no run and is an ordinary character, as the regular expression
/// `"[^"]*"|\[[^\]]*\]|\\.` the runs are modelled on reads it. A backslash
/// escape is the backslash and the character after it. What is left is
/// searched for the letters.
fn is_date_code(code: &str) -> bool {
    let chars: Vec<char> = code.chars().collect();
    let mut at = 0;
    while at < chars.len() {
        let c = chars[at];
        let closing = match c {
            '"' => Some('"'),
            '[' => Some(']'),
            _ => None,
        };
        if let Some(closing) = closing
            && let Some(offset) = chars[at + 1..].iter().position(|inner| *inner == closing)
        {
            at += offset + 2;
            continue;
        }
        if c == '\\' && at + 1 < chars.len() {
            at += 2;
            continue;
        }
        if matches!(c, 'd' | 'm' | 'h' | 'y' | 's' | 'D' | 'M' | 'H' | 'Y' | 'S') {
            return true;
        }
        at += 1;
    }
    false
}

/// Walks the package layer's whole map and counts it.
fn survey(input: &Path) -> Result<Survey, Error> {
    let file = File::open(input).map_err(|e| Error::ReadInput(e.kind()))?;
    let mut package = Package::open(BufReader::new(file))?;
    let names: Vec<Vec<u8>> = package
        .archive()
        .entries()
        .iter()
        .map(|entry| entry.name().to_vec())
        .collect();
    let mut counted = Survey {
        entries: names.len() as u64,
        ..Survey::default()
    };

    {
        let types = package.content_types()?;
        counted.defaults = types.defaults().len() as u64;
        counted.overrides = types.overrides().len() as u64;
        // Every entry but the content types part itself has a type or does
        // not: that is what the part is for.
        for name in &names {
            if name.as_slice() == CONTENT_TYPES_PART {
                continue;
            }
            if types.of(name).is_some() {
                counted.typed += 1;
            } else {
                counted.untyped += 1;
            }
        }
        // An Override names a part by an absolute name. specs/0009 fixes
        // that every one of the corpus's resolves exactly, and P2's survey
        // measured only the case-folded near miss, so an Override naming no
        // entry at all was counted nowhere (P2 verification, check 2's
        // observation; ruling 11).
        for over in types.overrides() {
            let part = over.part_name.strip_prefix(b"/").unwrap_or(&over.part_name);
            if !named(&names, part) {
                counted.overrides_unresolved += 1;
                if named_case_only(&names, part) {
                    counted.case_only += 1;
                }
            }
        }
    }

    // Every entry named as a rels part, read for the source its name derives.
    let sources: Vec<Vec<u8>> = names
        .iter()
        .filter_map(|name| rels_part_source(name))
        .collect();
    counted.rels_parts = sources.len() as u64;
    for source in &sources {
        let rels = package.relationships(source)?;
        for rel in rels.all() {
            counted.relationships += 1;
            if rel.external {
                counted.external += 1;
                continue;
            }
            let target = rels.target_part(rel)?;
            if named(&names, &target) {
                counted.resolved += 1;
            } else {
                counted.unresolved += 1;
                if named_case_only(&names, &target) {
                    counted.case_only += 1;
                }
            }
        }
    }

    let workbook = package.workbook()?;
    counted.sheets = workbook.sheets.len() as u64;
    counted.sheets_resolved = workbook
        .sheets
        .iter()
        .filter(|sheet| named(&names, &sheet.part))
        .count() as u64;
    counted.shared_strings = u64::from(workbook.shared_strings.is_some());
    counted.styles = u64::from(workbook.styles.is_some());
    counted.calc_chain = u64::from(workbook.calc_chain.is_some());
    counted.theme = u64::from(workbook.theme.is_some());
    Ok(counted)
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
