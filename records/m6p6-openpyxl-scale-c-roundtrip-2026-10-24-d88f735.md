# openpyxl comparison run 2026-10-24-d88f735-openpyxl-scale-c-roundtrip

Written by `scripts/passthrough-record.py` from the run directory `2026-10-24-d88f735-openpyxl-scale-c-roundtrip` (raw outputs stay under the corpus directory's `runs/`). Workbooks are cited by id, producer and tags only (`specs/0005-corpus.md`). The machine-readable companion to this file is `m6p6-openpyxl-scale-c-roundtrip-2026-10-24-d88f735.txt`, the same tab-separated tables and the tool's xml rows (the kind's columns in column_1 onward, `specs/0014-xml-comparison.md` "Output"); it is written as `.txt` because the hygiene gate refuses a tracked spreadsheet extension.

- Record written: 2026-10-24T12:11:24Z
- Commit: d88f7359f323da40da287d8f16ad9e1a8b8b9bd1
- Runs: 1, from 2026-10-24T12:06:22Z to 2026-10-24T12:11:14Z
- Targets: openpyxl
- Settings: timeout 3600 s; open-check timeout 600 s; memory 16384 MiB
- openpyxl: version 3.1.5; op roundtrip
- Container start: an empty run of the image (python3 -I -B -c pass, the same mounts and flags), three runs 0.59, 0.53, 0.50 s, median 0.53 s; the openpyxl wall times include it (specs/0037 rulings K and P)
- Binaries (openpyxl): image id 894b239fc0d1cd90df5fd3e86d89d662adc9562504da38d1a4ea32666985cf63; ops script sha256 741b65937554743d09366e10c6ca9346f66aaffbc785003b9373b8cd63befbc8; xlsx-diff sha256 1445523df9522bff56d4f0e03d7bde2ab333bac294020da256de42c01ef71f6b

## Files by outcome

| label | files | ok | error | timeout | cap | no-output | leftover | lo ok | lo fail | lo error | lo not-run | lo_in ok | lo_in fail | lo_in error | lo_in not-run |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| openpyxl | 1 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 1 |

## Entries by part family: openpyxl

| family | entries | identical | recompressed | changed | dropped | added | flagged | mtime | extra | gpflags | descriptor | attrs | comment | local |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| total | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

## XML comparison of changed entries: openpyxl

| family | compared | parse_errors | names_equal | values_equal | in_only | out_only |
| --- | --- | --- | --- | --- | --- | --- |
| total | 0 | 0 | 0 | 0 | 0 | 0 |

## Files not ok, with a diff exit other than 0, with a leftover temporary file, or failing the LibreOffice open check

| id | label | producer | tags | outcome | exit | signal | diff_exit | leftover | lo | lo_in | differs from the expectation |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 456ab96bc553 | openpyxl | scale-gen | scale,tierc | cap | 137 | - | - | no | - | - | not compared |

## Files whose open check could not run

| id | label | lo | lo_in |
| --- | --- | --- | --- |
| none |  |  |  |
