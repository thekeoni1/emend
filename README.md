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

- `dates`, the date arithmetic of `specs/0012-styles.md`: 4840 over the styles probe (`records/size-m3p7-2026-10-09-09d2e0f.md`).
- `rowcol`, the row and column insertions and deletions of `specs/0018-rows-and-columns.md`: 58152 (`records/size-m3p7-2026-10-09-09d2e0f.md`).
- `metadata`, the defined names, hyperlinks, document properties and protection of `specs/0019-names-links-properties-protection.md`: 50036 (`records/size-m3p7-2026-10-09-09d2e0f.md`).
- `rules`, the data validations and conditional formats of `specs/0020-validation-and-conditional-formatting.md`: 48468 (`records/size-m3p7-2026-10-09-09d2e0f.md`).
- `tables`, the tables of `specs/0021-tables.md`: 46536 (`records/size-m3p7-2026-10-09-09d2e0f.md`).
- `comments`, the cell comments of `specs/0022-comments-and-images.md`: 63356 (`records/size-m3p7-2026-10-09-09d2e0f.md`).
- `images`, the images of `specs/0022-comments-and-images.md`: 67832 (`records/size-m3p7-2026-10-09-09d2e0f.md`).
- `richtext`, the rich text runs of `specs/0024-rich-text.md`: 13384 (`records/size-m3p7-2026-10-09-09d2e0f.md`).
- `sheetprops`, the sheet and print settings of `specs/0025-sheet-and-print-settings.md`: 28284 (`records/size-m3p7-2026-10-09-09d2e0f.md`).
- `read`, the typed reads of `specs/0027-read-back.md`: 51988 (`records/size-m3p7-2026-10-09-09d2e0f.md`).
- `copy`, the worksheet copy of `specs/0028-images-and-comments-second-half.md`: 13820 (`records/size-m3p7-2026-10-09-09d2e0f.md`).

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

In the copy sample, 784 of the 917 zip entries on the host came back byte for byte: the entries the copy did not touch (`records/m3p6-p4-copy-2026-10-08-9cfd92e.md`).

Of the 1093 output files the crate's and the copy tool's tests keep, LibreOffice opened 1082; the record's own sections explain the eleven it did not (`records/m3p6-open-2026-10-08-9cfd92e.md`).

Size: the core's budget figure, the core probe over a binary that links only `zip` and `quick-xml`, is 436992 section bytes, 21760 under the ceiling of 458752 (`records/size-m3p7-2026-10-09-09d2e0f.md`).

Speed and memory: on generated workbooks up to a 5025024826-byte sheet entry, the streaming samples peaked between 7400 and 8004 kB on every tier they completed, and throughput ran from 2.9 to 4.8 MB/s on the host and from 1.9 to 3.2 MB/s on the static target, the largest tier's figures including the raw-copy samples, whose rewriting samples ran at 1.8 to 3.7 MB/s (`records/scale-m3p1-2026-10-02-6578925.md`).

## Out of scope by design

emend does not evaluate formulas, render, autofit, or fix up references
when rows or columns are inserted or deleted. These are openpyxl's limits
too, and they are kept on purpose.

## Licence

Licensed under either the MIT licence (`LICENSE-MIT`) or the Apache
licence (`LICENSE-APACHE`), at your option.
