# emend

emend edits existing Excel workbooks (.xlsx files) from Rust. Open
a workbook, change the cells, styles or sheets you need, and write
it back. Every zip entry the edit did not touch comes back byte
for byte, so the charts, pivot tables, macros and formatting you
never looked at are still there.

## When to use it

Use emend when the workbook already exists and has to survive the
edit: a template someone built in Excel, a report with charts and
pivot tables, a file whose formatting you do not own. emend parses
only the parts an edit touches and copies the rest unchanged. On a
round trip of 27 real workbooks it left 870 zip entries
byte-identical and dropped none
(`records/compare-m6p6-2026-10-24-d88f735.md`).

The other Rust crates do different jobs:

- `calamine` reads workbooks.
- `rust_xlsxwriter` writes new workbooks.
- `umya-spreadsheet` reads a workbook into its own model and
  writes a new file from it
  (`records/decision-architecture.md`, a reading of its 3.1.0
  source; emend has not measured it).

The core depends on `zip` and `quick-xml` and nothing else, and
measures 456248 section bytes over a static 32-bit binary that
already links those two (`records/logs/m8p3-p3/gate.log`). It
streams: a round trip of a 1159829654-byte workbook peaked at
7872 kB of memory
(`records/compare-m6p6-2026-10-24-d88f735.md`).

emend has the scope of Python's openpyxl and the same limits: it
does not evaluate formulas, render, or autofit. See "Out of scope
by design" below, and "What has been measured" for the comparison.

## Example

```rust
use emend::{CellRef, Value, package::Package};
use std::io::{Read, Seek, Write};

fn edit<R, W>(input: R, output: W) -> Result<(), Box<dyn std::error::Error>>
where
    R: Read + Seek + 'static,
    W: Write,
{
    let mut book = Package::open(input)?;
    let sheet = book.workbook()?.sheets.first().ok_or("no sheet")?.part.clone();
    let cell = CellRef::parse(b"B2").ok_or("not an A1 reference")?;
    book.set_cell(&sheet, cell, Value::Text("checked".into()));
    book.write_to(output)?;
    Ok(())
}
```

## Dependencies and features

The core (open, read, edit, write back) depends on `zip` and `quick-xml`
and nothing else. Everything heavier is a Cargo feature, off by default,
with no dependency of its own. Each feature below is named with the spec
that added it and with its size cost, in section bytes over the core
probe it is measured against, from the size record.

- `dates`, the date arithmetic of `specs/0012-styles.md`: 4840 over the styles probe (`records/size-m6p6-2026-10-24-7ac8b74.md`).
- `rowcol`, the row and column insertions and deletions of `specs/0018-rows-and-columns.md`: 76872 (`records/size-m8p2-2026-11-10-8065795.md`).
- `metadata`, the defined names, hyperlinks, document properties and protection of `specs/0019-names-links-properties-protection.md`: 49748 (`records/size-m6p6-2026-10-24-7ac8b74.md`).
- `rules`, the data validations and conditional formats of `specs/0020-validation-and-conditional-formatting.md`: 48888 (`records/size-m6p6-2026-10-24-7ac8b74.md`).
- `tables`, the tables of `specs/0021-tables.md`: 46196 (`records/size-m6p6-2026-10-24-7ac8b74.md`).
- `comments`, the cell comments of `specs/0022-comments-and-images.md`: 66388 (`records/size-m8p2-2026-11-10-8065795.md`).
- `images`, the images of `specs/0022-comments-and-images.md`: 56956 (`records/size-m6p6-2026-10-24-7ac8b74.md`).
- `richtext`, the rich text runs of `specs/0024-rich-text.md`: 13704 (`records/size-m6p6-2026-10-24-7ac8b74.md`).
- `sheetprops`, the sheet and print settings of `specs/0025-sheet-and-print-settings.md`: 27092 (`records/size-m6p6-2026-10-24-7ac8b74.md`).
- `read`, the typed reads of `specs/0027-read-back.md`, widened by `specs/0032-read-breadth-styles.md` and `specs/0033-read-breadth-survey.md`: 113528 (`records/size-m6p6-2026-10-24-7ac8b74.md`).
- `copy`, the worksheet copy of `specs/0028-images-and-comments-second-half.md`: 13452 (`records/size-m6p6-2026-10-24-7ac8b74.md`).
- `charts`, the chart reads and writes of `specs/0034-charts-read.md`, `specs/0035-charts-write-first-set.md` and `specs/0036-charts-write-second-set.md`, with the chartsheets, chart kinds and formatting, analytic chart elements and chart text and in-place edits of `specs/0038-chartsheets.md`, `specs/0039-chart-kinds-and-formatting.md`, `specs/0040-analytic-chart-elements.md` and `specs/0041-chart-text-and-edits.md`: 166680 (`records/size-m6p6-2026-10-24-7ac8b74.md`).

The core itself writes and reads gradient fills and named styles, `specs/0042-gradient-fills-and-named-styles.md`, with no feature; the size record's `probe-emend-fullstyles-write-xml` adds a gradient fill and a named style among its styles (`records/size-m6p6-2026-10-24-7ac8b74.md`).

## Building and testing

The toolchain is pinned in `rust-toolchain.toml`, and every change is
built and tested for the host and for the static target that file names,
since the core is meant to be embedded in small, static binaries
(`records/toolchain-2026-09-15.md`). The gate, `scripts/check.sh`, runs
every step, both targets included:

```sh
# the toolchain and the targets: records/toolchain-2026-09-15.md
cargo build --workspace --all-targets
cargo test --workspace
cargo test --workspace --target i686-unknown-linux-musl
cargo test -p emend --all-features
cargo test -p emend --all-features --target i686-unknown-linux-musl
scripts/check.sh
```

## What has been measured

Each sentence here cites the record that holds its figures.

Fidelity: each of the four editing samples (copy a sheet, move an image's anchor, remove an image, size a comment box) ran over the 27 workbooks of the private test corpus, and on the host all 27 outputs were written and opened in LibreOffice, as were all 27 inputs (`records/m3p6-p4-copy-2026-10-08-9cfd92e.md`, `records/m3p6-p4-anchor-2026-10-08-9cfd92e.md`, `records/m3p6-p4-removeimage-2026-10-08-9cfd92e.md`, `records/m3p6-p4-commentbox-2026-10-08-9cfd92e.md`).

Each of Milestone 5 part 5's five chart samples (read the charts, add more chart kinds, add the 3D kinds, add coloured and labelled charts, remove a chart) ran over the same 27 workbooks, and on the host and on the static target all 27 outputs were written and opened in LibreOffice, as were all 27 inputs (`records/m5p5-p4-charts-2026-10-15-a8bfe71.md`, `records/m5p5-p4-chartmore-2026-10-15-a8bfe71.md`, `records/m5p5-p4-chart3d-2026-10-15-a8bfe71.md`, `records/m5p5-p4-chartlabels-2026-10-15-a8bfe71.md`, `records/m5p5-p4-removechart-2026-10-15-a8bfe71.md`).

Each of Milestone 6's six samples (add a chartsheet, format chart series, set chart axes, give chart text its properties, give cells gradient fills, add a named style) ran over the same 27 workbooks, and on the host and on the static target all 27 outputs were written and opened in LibreOffice, as were all 27 inputs (`records/m6p1-p3-chartsheet-2026-10-19-4cc4f2e.md`, `records/m6p2-p3-chartformat-2026-10-20-50a3989.md`, `records/m6p3-p3-chartaxis-2026-10-21-0707236.md`, `records/m6p4-p3-charttext-2026-10-22-1cc0de4.md`, `records/m6p5-p3-gradient-2026-10-23-fe280ff.md`, `records/m6p5-p3-namedstyle-2026-10-23-fe280ff.md`).

In the copy sample, 784 of the 917 zip entries on the host came back byte for byte: the entries the copy did not touch (`records/m3p6-p4-copy-2026-10-08-9cfd92e.md`).

Of the 1093 output files the crate's and the copy tool's tests keep, LibreOffice opened 1082; the record's own sections explain the eleven it did not (`records/m3p6-open-2026-10-08-9cfd92e.md`).

Size: the core's budget figure, the core probe over a binary that links only `zip` and `quick-xml`, is 456248 section bytes, 2504 under the ceiling of 458752, at the 0.1.1 manifest (`records/logs/m8p3-p3/gate.log`); Milestone 8 part 2's size record measured 456264 before the version changed (`records/size-m8p2-2026-11-10-8065795.md`).

Speed and memory: on generated workbooks up to a 5025024826-byte sheet entry, the streaming samples peaked between 7400 and 8004 kB on every tier they completed, and throughput ran from 2.9 to 4.8 MB/s on the host and from 1.9 to 3.2 MB/s on the static target, the largest tier's figures including the raw-copy samples, whose rewriting samples ran at 1.8 to 3.7 MB/s (`records/scale-m3p1-2026-10-02-6578925.md`). In Milestone 6's rerun of the tiers, emend's roundtrip of the tier c workbook, 1159829654 bytes, took 2.30 s at a peak of 7872 kB on the host and 3.60 s at a peak of 7560 kB on the static target, and of the tier d workbook, 201769244 bytes, 0.40 s on each (`records/compare-m6p6-2026-10-24-d88f735.md`).

Comparison with openpyxl: over the 27 workbooks and the sixteen ops of the comparison, each tool's output compared with its input, emend on the host and on the static target and openpyxl each wrote 27 of 27 outputs for every op, and LibreOffice opened every one (`records/compare-m6p6-2026-10-24-d88f735.md`). On the roundtrip emend left 870 zip entries byte-identical and dropped none, where openpyxl left 28 identical and dropped 533 (`records/compare-m6p6-2026-10-24-d88f735.md`). The median wall time per workbook was 0.10 s for emend on the host and 0.81 s for openpyxl on every op, openpyxl's including its container's start, and the roundtrip's largest peak RSS was 7992 kB for emend on the host and 674516 kB for openpyxl (`records/compare-m6p6-2026-10-24-d88f735.md`). On the scale tiers emend completed every run, and openpyxl ended at its memory cap on the tier c roundtrip, exit 137 after 290.83 s at a peak of 16554684 kB (`records/compare-m6p6-2026-10-24-d88f735.md`).

Excel: Microsoft Excel was asked to open the 6018 files of Milestone 7 part 1's check, every sample's outputs on the private test corpus and the outputs the crate's tests keep with their inputs: on the corpus 1528 passed and 32 needed a repair, of the kept outputs 1864 passed and 365 were refused, and of their inputs 1874 passed and 355 were refused; Milestone 7 part 2 fixed five of the causes and documented the rest as limits (`records/excel-check-m7-2026-10-26.md`).

After the fixes, Microsoft Excel was asked to open the 4640 files of Milestone 7 part 2's check, six samples' outputs over the 27 workbooks and the outputs the crate's tests keep with their inputs: on the corpus 159 passed and 3 needed a repair, the two documented limits' three outputs, opened by hand with a repair offered; of the kept outputs 1878 passed and 361 were refused, and of their inputs 1881 passed and 358 were refused (`records/excel-check-m7p2-2026-10-28.md`).

After Milestone 8's changes, Microsoft Excel was asked to open the 4848 files of Milestone 8 part 3's check, eight samples' outputs over the 39 workbooks and the outputs the crate's tests keep with their inputs: on the corpus 311 passed and 1 needed a repair, the documented limit of a removed sheet's references, opened by hand with a repair offered; of the kept outputs 1905 passed and 363 were refused, and of their inputs 1907 passed and 361 were refused (`records/excel-check-m8-2026-11-12.md`).

## Out of scope by design

emend does not evaluate formulas, render, autofit, or fix up references
when rows or columns are inserted or deleted. These are openpyxl's limits
too, and they are kept on purpose.

A cell edit on a table's header row changes the cell and not the table's
column name; the two must be kept in step by the caller, or Excel repairs
the file. A sheet removed leaves every reference to it, and to its tables,
in other sheets' formulas and defined names as they were, as insertions and
deletions leave references (`specs/0045-publish.md`). A sheet a pivot cache
sources is not removed: the core refuses a sheet a cache names by its
`sheet` attribute, the feature `tables` a sheet holding a table a cache
names, and the feature `metadata` a sheet a cache's defined name is
qualified with (`specs/0048-the-two-refusals.md`, "The pivot cache source",
and Amendment 1). A style edit assumes
the styles part carries the font, fill, border and cell-style collections
and the `cellXfs` entries that name them (`specs/0012-styles.md`;
`specs/0015-open-checks.md`, Excel-check ruling 4). A removal does not
undo a namespace declaration, or an emptied relationships part, that an
earlier addition wrote; the result is valid
(`specs/0049-excel-check-and-release.md`, ruling F).

## Licence

Licensed under either the MIT licence (`LICENSE-MIT`) or the Apache
licence (`LICENSE-APACHE`), at your option.
