# step0 passthrough run 2026-10-01-8d05845-scale-d-validate

Written by `scripts/passthrough-record.py` from the run directory `2026-10-01-8d05845-scale-d-validate` (raw outputs stay under the corpus directory's `runs/`). Workbooks are cited by id, producer and tags only (`specs/0005-corpus.md`). The machine-readable companion to this file is `m3p1-scale-d-validate-2026-10-01-8d05845.txt`, the same tab-separated tables and the tool's xml rows (the kind's columns in column_1 onward, `specs/0014-xml-comparison.md` "Output"); it is written as `.txt` because the hygiene gate refuses a tracked spreadsheet extension.

- Record written: 2026-10-02T00:56:58Z
- Commit: 8d05845de2f80a88dbd3b31fbd6f930410a0789f
- Runs: 2, from 2026-10-02T00:48:51Z to 2026-10-02T00:56:58Z
- Targets: host, i686
- Tool versions: rustc 1.98.1 (48a229cea 2026-09-01); zip 8.6.0
- Settings: timeout 3600 s; open-check timeout 600 s; memory 2048 MiB
- Tool arguments: --validate-sample

## Files by outcome

| label | files | ok | error | timeout | panic | crash | leftover | lo ok | lo fail | lo error | lo not-run | lo_in ok | lo_in fail | lo_in error | lo_in not-run |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| host | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 1 |
| i686 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 1 |

## Entries by part family: host

| family | entries | identical | recompressed | changed | dropped | added | flagged | mtime | extra | gpflags | descriptor | attrs | comment | local |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| content-types | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| rels | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| workbook | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| worksheet | 2 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| sharedStrings | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| styles | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| docProps | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| total | 9 | 8 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

## Entries by part family: i686

| family | entries | identical | recompressed | changed | dropped | added | flagged | mtime | extra | gpflags | descriptor | attrs | comment | local |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| content-types | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| rels | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| workbook | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| worksheet | 2 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| sharedStrings | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| styles | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| docProps | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| total | 9 | 8 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

## XML comparison of changed entries: host

| family | compared | parse_errors | names_equal | values_equal | in_only | out_only |
| --- | --- | --- | --- | --- | --- | --- |
| worksheet | 1 | 0 | 0 | 0 | 0 | 20 |
| total | 1 | 0 | 0 | 0 | 0 | 20 |

## XML comparison of changed entries: i686

| family | compared | parse_errors | names_equal | values_equal | in_only | out_only |
| --- | --- | --- | --- | --- | --- | --- |
| worksheet | 1 | 0 | 0 | 0 | 0 | 20 |
| total | 1 | 0 | 0 | 0 | 0 | 20 |

## Files not ok, with a diff exit other than 0, with a leftover temporary file, or failing the LibreOffice open check

| id | label | producer | tags | outcome | exit | signal | diff_exit | leftover | lo | lo_in | differs from the expectation |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 35ad8624513a | host | scale-gen | scale,tierd | ok | 0 | - | 1 | no | - | - | changed=1, files_identical=no |
| 35ad8624513a | i686 | scale-gen | scale,tierd | ok | 0 | - | 1 | no | - | - | changed=1, files_identical=no |

## Files whose open check could not run

| id | label | lo | lo_in |
| --- | --- | --- | --- |
| none |  |  |  |
