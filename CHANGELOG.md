# Changelog

## 0.1.0 (unreleased)

### Added

- The zip layer: an archive opened, every entry indexed with `u64` offsets, and
  the archive written back with every untouched entry byte for byte in its
  original place (`specs/0008-zip-layer.md`).
- The package layer: a part's XML streamed token by token with the exact input
  bytes each token covers, so a rewrite copies what it does not change
  (`specs/0009-package-layer.md`).
- Reading cells: the shared strings table and a streaming read of one
  worksheet's cells (`specs/0010-reading-cells.md`).
- Editing cells: a cell set to a number, a text or a boolean, or cleared, the
  worksheet part rewritten by streaming with every token the edit does not touch
  copied from the input (`specs/0011-editing-cells.md`).
- Styles: the number formats and cell formats read, and a cell format derived
  with a new number format; the feature `dates` adds a civil date and time to
  and from the serial number a cell holds (`specs/0012-styles.md`).
- Formulas: a formula written into a cell with no cached value, the calculation
  chain dropped and a full calculation asked for on load; nothing is evaluated
  (`specs/0013-formulas.md`).
- A new workbook from a fixed, fictional template carried in the crate, and
  sheets added, renamed, moved and removed
  (`specs/0016-new-workbook-and-sheets.md`).
- The fuller styles: a cell given a font, a fill, a border, an alignment and a
  protection without renumbering any style the workbook already has
  (`specs/0017-fuller-styles.md`).
- Rows and columns, with the feature `rowcol`: row and column insertions and
  deletions (`specs/0018-rows-and-columns.md`).
- Names, links, properties and protection, with the feature `metadata`: defined
  names, hyperlinks, document properties and protection
  (`specs/0019-names-links-properties-protection.md`).
- Validations and conditional formats, with the feature `rules`
  (`specs/0020-validation-and-conditional-formatting.md`).
- Tables, with the feature `tables` (`specs/0021-tables.md`).
- Comments and images, with the features `comments` and `images`
  (`specs/0022-comments-and-images.md`), grown by
  `specs/0028-images-and-comments-second-half.md` (removal, anchors, the comment
  box).
- Rich text, with the feature `richtext`: runs with their own font in a cell,
  and with `comments` in a comment (`specs/0024-rich-text.md`).
- Sheet and print settings, with the feature `sheetprops`: the tab colour, the
  sheet view, the print options, margins, page setup, headers and footers
  (`specs/0025-sheet-and-print-settings.md`).
- Grouping and line styles, with the feature `rowcol`: outline levels on rows
  and columns with their hidden and collapsed flags, and a style on a whole row
  or column (`specs/0026-grouping-and-line-styles.md`).
- Read-back, with the feature `read`: a cell's format, the column and row
  formats and the sheet's settings read as the types the writers take
  (`specs/0027-read-back.md`).
- The worksheet copy, with the feature `copy`: a sheet copied within its
  workbook, its comments carried with `comments` too
  (`specs/0028-images-and-comments-second-half.md`).
- Read breadth, with the feature `read`: the named styles, the differential
  styles and the default font read as the types the writers take, and a colour
  resolved to the RGB the workbook defines for it through the palette and the
  theme (`specs/0032-read-breadth-styles.md`).
- Read breadth, with the feature `read`: the part kinds a package holds and the
  parts each sheet relates, counted; the pivot tables read with their caches'
  sources and field names; and the external links read with their targets as
  written (`specs/0033-read-breadth-survey.md`).
- Charts read, with the feature `charts`: each chart a sheet's drawing anchors,
  with its plots and their series references, its title, axes, legend, pivot
  source and style (`specs/0034-charts-read.md`).
- Charts write, with the feature `charts`: a bar, column, line, pie, scatter
  or area chart added to a sheet from series references the caller names, with
  its title, axis titles and legend, anchored as an image is
  (`specs/0035-charts-write-first-set.md`).
- Charts write, the second set, with the feature `charts`: doughnut, radar,
  bubble and stock charts, the 3D bar, line, pie and area charts, combined
  charts with a second plot on a secondary axis, series colours, data labels
  and axis number formats, and a chart removed from a sheet with the parts only
  it reaches (`specs/0036-charts-write-second-set.md`).
- Chartsheets, with the feature `charts`: a chartsheet's chart read as a
  worksheet's; a chartsheet added holding one chart; its chart removed and
  another added; and a chartsheet renamed, moved and removed with the sheet
  calls (`specs/0038-chartsheets.md`).
- Chart kinds and formatting, with the feature `charts`: the surface, 3D
  surface and of-pie charts, and on a chart's series a marker, a line's width
  and dash, and formatted data points, each where the kind takes it
  (`specs/0039-chart-kinds-and-formatting.md`).
- Analytic chart elements, with the feature `charts`: a series' trendline and
  error bars, a chart's drop lines, high-low lines and up-down bars, and its
  axes' scaling, units, label and tick mark skips and a date category axis,
  each where the kind takes it (`specs/0040-analytic-chart-elements.md`).
- Chart text and in-place edits, with the feature `charts`: a written chart's
  title, axis titles, axis labels, data labels and legend given a size, bold,
  italic, a colour and a typeface, and the chart a style value; a kept chart's
  title and a kept series' name and references changed in place, every other
  byte of its chart part kept (`specs/0041-chart-text-and-edits.md`).

### Changed

- `SharedStringItem`, formerly `Item` (`specs/0030-carries-and-api-changes.md`).
- `SharedStringItems`, formerly `Items`
  (`specs/0030-carries-and-api-changes.md`).
- `CellKind`, formerly `Kind`, the crate root's re-export following the rename
  (`specs/0030-carries-and-api-changes.md`).
- `ContentTypeDefault`, formerly `Default`
  (`specs/0030-carries-and-api-changes.md`).
- `HyperlinkTarget`, formerly `Target`
  (`specs/0030-carries-and-api-changes.md`).
- `Archive::prefix_len` made crate-private, compiled for the crate's own tests
  only (`specs/0030-carries-and-api-changes.md`).

### Fixed

- An image or chart added to a sheet whose drawing part has a self-closing
  `<xdr:wsDr/>` root: the shared drawing writer reads the root as an empty
  drawing and appends to it, where it refused before, "a drawing part or its
  relationship cannot be read" (`specs/0037-measurement-and-close.md`,
  decision 3 A).

### Measured

- Fidelity: each of the four editing samples (copy a sheet, move an image's
  anchor, remove an image, size a comment box) ran over the 27 workbooks of the
  private test corpus, and on the host all 27 outputs were written and opened in
  LibreOffice, as were all 27 inputs
  (`records/m3p6-p4-copy-2026-10-08-9cfd92e.md`,
  `records/m3p6-p4-anchor-2026-10-08-9cfd92e.md`,
  `records/m3p6-p4-removeimage-2026-10-08-9cfd92e.md`,
  `records/m3p6-p4-commentbox-2026-10-08-9cfd92e.md`).
- In the copy sample, 784 of the 917 zip entries on the host came back byte for
  byte: the entries the copy did not touch
  (`records/m3p6-p4-copy-2026-10-08-9cfd92e.md`).
- Of the 1093 output files the crate's and the copy tool's tests keep,
  LibreOffice opened 1082; the record's own sections explain the eleven it did
  not (`records/m3p6-open-2026-10-08-9cfd92e.md`).
- Size: the core's budget figure, the core probe over a binary that links only
  `zip` and `quick-xml`, is 436992 section bytes, 21760 under the ceiling of
  458752 (`records/size-m3p7-2026-10-09-09d2e0f.md`).
- Speed and memory: on generated workbooks up to a 5025024826-byte sheet entry,
  the streaming samples peaked between 7400 and 8004 kB on every tier they
  completed, and throughput ran from 2.9 to 4.8 MB/s on the host and from 1.9 to
  3.2 MB/s on the static target, the largest tier's figures including the
  raw-copy samples, whose rewriting samples ran at 1.8 to 3.7 MB/s
  (`records/scale-m3p1-2026-10-02-6578925.md`).

### Not supported by design

- emend does not evaluate formulas (it stores formula strings and reads cached
  values), does not render, does not autofit, and does not fix up references on
  row or column insertion.
