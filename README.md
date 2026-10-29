# emend

emend is a pure-Rust library for read-modify-write of Office Open XML
spreadsheets: open an existing .xlsx, change what you were asked to
change, hand the file back with everything else intact. It parses only
what an edit touches and passes every other zip entry through byte for
byte.

The comparison point is openpyxl, and the definition is exact: like
openpyxl, emend does not evaluate formulas (it stores formula strings and
reads cached values), does not render, does not autofit, and does not fix
up references on row or column insertion. Those limits are by design and
are not gaps.

## Dependencies and features

The core (open, read, edit, write back) depends on `zip` and `quick-xml`
and nothing else. Everything heavier is a Cargo feature, off by default,
with no dependency of its own. Each feature below is named with the spec
that added it and with its size cost, in section bytes over the core
probe it is measured against, from the size record.

- `dates`, the date arithmetic of `specs/0012-styles.md`: 4840 over the styles probe (`records/size-m6p6-2026-10-24-7ac8b74.md`).
- `rowcol`, the row and column insertions and deletions of `specs/0018-rows-and-columns.md`: 58260 (`records/size-m6p6-2026-10-24-7ac8b74.md`).
- `metadata`, the defined names, hyperlinks, document properties and protection of `specs/0019-names-links-properties-protection.md`: 49748 (`records/size-m6p6-2026-10-24-7ac8b74.md`).
- `rules`, the data validations and conditional formats of `specs/0020-validation-and-conditional-formatting.md`: 48888 (`records/size-m6p6-2026-10-24-7ac8b74.md`).
- `tables`, the tables of `specs/0021-tables.md`: 46196 (`records/size-m6p6-2026-10-24-7ac8b74.md`).
- `comments`, the cell comments of `specs/0022-comments-and-images.md`: 63424 (`records/size-m6p6-2026-10-24-7ac8b74.md`).
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

Size: the core's budget figure, the core probe over a binary that links only `zip` and `quick-xml`, is 451528 section bytes, 7224 under the ceiling of 458752 (`records/size-m6p6-2026-10-24-7ac8b74.md`).

Speed and memory: on generated workbooks up to a 5025024826-byte sheet entry, the streaming samples peaked between 7400 and 8004 kB on every tier they completed, and throughput ran from 2.9 to 4.8 MB/s on the host and from 1.9 to 3.2 MB/s on the static target, the largest tier's figures including the raw-copy samples, whose rewriting samples ran at 1.8 to 3.7 MB/s (`records/scale-m3p1-2026-10-02-6578925.md`). In Milestone 6's rerun of the tiers, emend's roundtrip of the tier c workbook, 1159829654 bytes, took 2.30 s at a peak of 7872 kB on the host and 3.60 s at a peak of 7560 kB on the static target, and of the tier d workbook, 201769244 bytes, 0.40 s on each (`records/compare-m6p6-2026-10-24-d88f735.md`).

Comparison with openpyxl: over the 27 workbooks and the sixteen ops of the comparison, each tool's output compared with its input, emend on the host and on the static target and openpyxl each wrote 27 of 27 outputs for every op, and LibreOffice opened every one (`records/compare-m6p6-2026-10-24-d88f735.md`). On the roundtrip emend left 870 zip entries byte-identical and dropped none, where openpyxl left 28 identical and dropped 533 (`records/compare-m6p6-2026-10-24-d88f735.md`). The median wall time per workbook was 0.10 s for emend on the host and 0.81 s for openpyxl on every op, openpyxl's including its container's start, and the roundtrip's largest peak RSS was 7992 kB for emend on the host and 674516 kB for openpyxl (`records/compare-m6p6-2026-10-24-d88f735.md`). On the scale tiers emend completed every run, and openpyxl ended at its memory cap on the tier c roundtrip, exit 137 after 290.83 s at a peak of 16554684 kB (`records/compare-m6p6-2026-10-24-d88f735.md`).

Excel: Microsoft Excel was asked to open the 6018 files of Milestone 7 part 1's check, every sample's outputs on the private test corpus and the outputs the crate's tests keep with their inputs: on the corpus 1528 passed and 32 needed a repair, of the kept outputs 1864 passed and 365 were refused, and of their inputs 1874 passed and 355 were refused; Milestone 7 part 2 fixed five of the causes and documented the rest as limits (`records/excel-check-m7-2026-10-26.md`).

After the fixes, Microsoft Excel was asked to open the 4640 files of Milestone 7 part 2's check, six samples' outputs over the 27 workbooks and the outputs the crate's tests keep with their inputs: on the corpus 159 passed and 3 needed a repair, the two documented limits' three outputs, opened by hand with a repair offered; of the kept outputs 1878 passed and 361 were refused, and of their inputs 1881 passed and 358 were refused (`records/excel-check-m7p2-2026-10-28.md`).

## Out of scope by design

emend does not evaluate formulas, render, autofit, or fix up references
when rows or columns are inserted or deleted. These are openpyxl's limits
too, and they are kept on purpose.

A cell edit on a table's header row changes the cell and not the table's
column name; the two must be kept in step by the caller, or Excel repairs
the file. A sheet removed leaves every reference to it, and to its tables,
in other sheets' formulas and defined names as they were, as insertions and
deletions leave references (`specs/0045-publish.md`). A style edit assumes
the styles part carries the font, fill, border and cell-style collections
and the `cellXfs` entries that name them (`specs/0012-styles.md`;
`specs/0015-open-checks.md`, Excel-check ruling 4).

## Licence

Licensed under either the MIT licence (`LICENSE-MIT`) or the Apache
licence (`LICENSE-APACHE`), at your option.
