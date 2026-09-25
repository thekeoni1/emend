# Excel check (template)

The procedure and record form of the Excel check: `specs/0015-open-checks.md`,
"The Excel check" and the planning rulings on the Excel check, 1, 2 and 9;
`specs/0016-new-workbook-and-sheets.md`, "The carried items", decision 5 and
the planning rulings during P1, 2 to 6. Planning assembles a filled copy from
what the maintainer brings back, checks every row's hash against the run
directory or the keep directory it names, and the implementing session
commits it as `records/excel-check-<name>-<date>.md` with a scan. The filled
record is retained.

## Before starting

1. The list: `scripts/excel-check-list.py --run <formula-sample run dir>
   --run-plain <other run dir> ... --keep <keep dir> --record <open-check
   record .txt> --out <list .txt>`, written on the development machine with
   mode 600. The list is never committed: its rows name cell positions in
   corpus workbooks.
2. The staged directory, zipped: the list as `excel-check-list.txt`,
   `scripts/excel-check-open.ps1` beside it, and every file the list names at
   `files/<source>/<file>`: corpus rows from the corpus root, fixture rows'
   outputs and input rows' inputs from the keep directory. Nothing else of the
   corpus is copied. The zip goes to the Excel machine and is unzipped outside
   any synced folder.

## Procedure

1. With Excel closed, run the script from the staged directory:
   `powershell -ExecutionPolicy Bypass -File .\excel-check-open.ps1`.
   It opens every row of the list in order (corpus, fixture and input rows)
   through COM with Excel visible and alerts on, read-only, links not
   updated, and closes each without saving. A repair prompt stops it at that
   file: note the row, answer the prompt, and record that row as `repair`.
2. It writes `open-results.txt` beside itself: `source`, `id_or_test`,
   `label`, `result` (`pass` when the open returned, `fail` when it threw),
   `seen1` to `seen3` (for a corpus row, the values at its non-empty
   references on the first worksheet), and Excel's `message`.
3. By-hand fallback, only for a corpus row the script recorded `fail` or
   `read-error`: open that file from Excel's file-open dialog, one at a time,
   never saved; record `pass` (opens with no repair prompt and no recovery
   notice), `repair` (Excel offered or made repairs) or `fail` (Excel did not
   open it), and read the values at the list's references. The row is marked
   `(by hand)`.
4. Bring back `open-results.txt`, and the machine section below filled in.
   Excel's message on a refused corpus row stays in `open-results.txt` and
   never enters the record.

## The machine

- Excel version and build:
- Operating system and version:
- Date of the check (UTC):

## Results

Rows in the list's order. `seen1` to `seen3` are the values Excel shows at
the corpus row's `ref1` to `ref3` (expected `expect1` to `expect3`); a row
with no references leaves them empty. The cell references themselves are not
copied here; the list holds them.

### Corpus rows

| source | id_or_test | label | bytes | sha256 | result | seen1 | seen2 | seen3 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |

### Fixture rows

Each fixture row's output, with its input row's result beside it (`input`).

| source | id_or_test | label | bytes | sha256 | result | input bytes | input sha256 | input |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |

## Acceptance

Every corpus row reads `pass` and every `seen` value equals its `expect`
value. For the fixtures, an input Excel refuses is allowed and listed with
its output's result; an output Excel refuses on an input Excel accepted is a
finding. Planning checks each row's `bytes` and `sha256` against the file in
the run directory or the keep directory before the filled record is
committed.
