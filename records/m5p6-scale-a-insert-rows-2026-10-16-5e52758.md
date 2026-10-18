# step0 passthrough run 2026-10-16-5e52758-scale-a-insert-rows

Written by `scripts/passthrough-record.py` from the run directory `2026-10-16-5e52758-scale-a-insert-rows` (raw outputs stay under the corpus directory's `runs/`). Workbooks are cited by id, producer and tags only (`specs/0005-corpus.md`). The machine-readable companion to this file is `m5p6-scale-a-insert-rows-2026-10-16-5e52758.txt`, the same tab-separated tables and the tool's xml rows (the kind's columns in column_1 onward, `specs/0014-xml-comparison.md` "Output"); it is written as `.txt` because the hygiene gate refuses a tracked spreadsheet extension.

- Record written: 2026-10-16T20:43:23Z
- Commit: 5e52758e379966672baa2e350e4bf997346aee01
- Runs: 2, from 2026-10-16T20:42:28Z to 2026-10-16T20:43:22Z
- Targets: host, i686
- Tool versions: rustc 1.98.1 (48a229cea 2026-09-01); zip 8.6.0
- Settings: timeout 3600 s; open-check timeout 600 s; memory 2048 MiB
- Tool arguments: --insert-rows-sample
- Binaries (host): passthrough sha256 6a8652ede1bb7eed181da445f2b54811b12abbe3a435418aeda4df5dad337e65; xlsx-diff sha256 1445523df9522bff56d4f0e03d7bde2ab333bac294020da256de42c01ef71f6b
- Binaries (i686): passthrough sha256 883acdf355e8b9b780bce3075b09ef9587b0774505b34c452721721b50c67895; xlsx-diff sha256 99d1661d92ffe59b1405530bd2b85af8555804f236125055d2a81da282565fd5

## Files by outcome

| label | files | ok | error | timeout | panic | crash | leftover | lo ok | lo fail | lo error | lo not-run | lo_in ok | lo_in fail | lo_in error | lo_in not-run |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| host | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 1 | 0 | 0 | 0 |
| i686 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 1 | 0 | 0 | 0 |

## Entries by part family: host

| family | entries | identical | recompressed | changed | dropped | added | flagged | mtime | extra | gpflags | descriptor | attrs | comment | local |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| content-types | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| rels | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| workbook | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| worksheet | 2 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| sharedStrings | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| styles | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| docProps | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| total | 9 | 7 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

## Entries by part family: i686

| family | entries | identical | recompressed | changed | dropped | added | flagged | mtime | extra | gpflags | descriptor | attrs | comment | local |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| content-types | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| rels | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| workbook | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| worksheet | 2 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| sharedStrings | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| styles | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| docProps | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| total | 9 | 7 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

## XML comparison of changed entries: host

| family | compared | parse_errors | names_equal | values_equal | in_only | out_only |
| --- | --- | --- | --- | --- | --- | --- |
| workbook | 1 | 0 | 0 | 0 | 0 | 1 |
| worksheet | 1 | 0 | 1 | 0 | 50 | 50 |
| total | 2 | 0 | 1 | 0 | 50 | 51 |

## XML comparison of changed entries: i686

| family | compared | parse_errors | names_equal | values_equal | in_only | out_only |
| --- | --- | --- | --- | --- | --- | --- |
| workbook | 1 | 0 | 0 | 0 | 0 | 1 |
| worksheet | 1 | 0 | 1 | 0 | 50 | 50 |
| total | 2 | 0 | 1 | 0 | 50 | 51 |

## Files not ok, with a diff exit other than 0, with a leftover temporary file, or failing the LibreOffice open check

| id | label | producer | tags | outcome | exit | signal | diff_exit | leftover | lo | lo_in | differs from the expectation |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1b014c92bd83 | host | scale-gen | scale,tiera | ok | 0 | - | 1 | no | ok | ok | changed=2, files_identical=no |
| 1b014c92bd83 | i686 | scale-gen | scale,tiera | ok | 0 | - | 1 | no | ok | ok | changed=2, files_identical=no |

## Files whose open check could not run

| id | label | lo | lo_in |
| --- | --- | --- | --- |
| none |  |  |  |
