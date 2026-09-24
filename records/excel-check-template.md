# Excel check, Milestone 1 (template)

The procedure and record form of `specs/0015-open-checks.md`, "The Excel
check". The maintainer fills a copy of this file on the machine with Excel;
the filled copy is committed as `records/excel-check-m1-<date>.md` after
planning has checked every row's hash against the run directory and the keep
directory it names.

## Before starting

1. The list: `scripts/excel-check-list.py --run <run dir> --keep <keep dir>
   --record <open-check record .txt> --out <list .txt>`, written on the
   development machine. The list is never committed: its rows name cell
   positions in corpus workbooks.
2. Copy every file the list names to the Excel machine: the corpus rows'
   files from the corpus root, the fixture rows' files from the keep
   directory. Nothing else of the corpus is copied.
3. Open files one at a time, each from a fresh start of Excel's file-open
   dialog. Do not save any file.

## The machine

- Excel version and build:
- Operating system and version:
- Date of the check (UTC):

## Results

One line per list row, in the list's order. Nothing else is typed.

- `result` is one of:
  - `pass`: the file opens with no repair prompt and no recovery notice;
  - `repair`: Excel offered or made repairs;
  - `fail`: Excel did not open the file.
- For a corpus row with references, `seen1`, `seen2` and `seen3` are the
  values Excel shows in the cells `ref1`, `ref2` and `ref3` of the first
  worksheet (expected `expect1`, `expect2` and `expect3`; `seen3` is empty
  when `ref3` is). A row with no references leaves all three empty.
- The cell references themselves are not copied here; the list holds them.

| source | id_or_test | label | bytes | sha256 | result | seen1 | seen2 | seen3 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |

## Acceptance

Milestone 1 is accepted when every row reads `pass` and every `seen` value
equals its `expect` value. Planning checks each row's `bytes` and `sha256`
against the file in the run directory or the keep directory before the
filled record is committed.
