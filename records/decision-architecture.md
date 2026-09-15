# Architecture decision: greenfield hybrid

- Date: 2026-09-14
- Decided by: the maintainer
- Recorded by: the planning session

## Decision

emend is a greenfield hybrid. It parses only what an edit touches and
passes every other zip entry through byte for byte. umya-spreadsheet and
rust_xlsxwriter are reference material only and are never a linked
dependency.

## Basis

The maintainer made this decision before any step-zero measurement. It
rests on a reading of the published umya-spreadsheet 3.1.0 crate (study
only; nothing was copied) and on its published metadata, both checked
2026-09-14:

- On every write, `write_zip_to_writer` in `src/writer/xlsx.rs`
  regenerates document properties, root relationships, theme, persons,
  shared strings, styles, the workbook, workbook relationships and
  content types from its model. CLAUDE.md requires entries an edit does
  not touch to stay byte-identical. That is not reachable without
  replacing this writer.
- `write_writer` builds the whole output archive in memory
  (`make_buffer`) before writing it out. emend has to stream where the
  format allows, because it targets 32-bit processes.
- With `lazy_read`, an unparsed worksheet is written back from its raw
  bytes, but under the name `sheet{position}.xml` and recompressed.
  Charts, drawings, comments, tables and pivots are written from the
  model, and only for sheets that were parsed.
- No file under `src/` mentions calcChain, customXml, timeline,
  externalLink, richData, queryTable, chartsheet or sparkline.
- umya-spreadsheet 3.1.0 has 21 normal dependencies and requires
  quick-xml ^0.41, while the current quick-xml release is 0.42.0.
  CLAUDE.md allows the core to depend on `zip` and `quick-xml` only.

## What was not measured

No workbook was round-tripped through umya-spreadsheet. The points above
come from reading source text, not from runs. In particular, whether
umya-spreadsheet drops the parts it never mentions is an inference, not
a measurement. No README or record cites these points as fidelity
results.

## Consequences

- Step zero's umya-spreadsheet arms (T3 A1 and A2) are dropped. T3 keeps
  the plain zip-copy passthrough run over the corpus, on the host and on
  `i686-unknown-linux-musl`, plus the size baseline.
- The names-only XML comparison (formerly T4) moves into Milestone 1,
  where it shows that edited entries differ only where the edit
  requires.
- Decision criteria E1 to E5 are retired without being evaluated.
- Open risks:
  - Passthrough on real archives from different producers (zip64, data
    descriptors, unusual layouts). T3 measures this before any editing
    code exists.
  - Excel's strictness about edited XML. A machine with desktop Excel is
    needed before Milestone 1 can be accepted.
