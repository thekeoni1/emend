# Committed fixtures

Every file here is fictional or generated, small, and named below with the
tool and version that produced it (`CLAUDE.md`, "Corpus rules").

## openpyxl-ops.xlsx

The workbook `scripts/openpyxl-ops-test.sh` runs every op of
`scripts/openpyxl-ops.py` on (`specs/0037-measurement-and-close.md` P1,
"Planning rulings during P1", ruling F).

- Tool: `scripts/scale-gen.py` ("scale-gen 1", seed 20261001), tier t: two
  sheets, 2500 cells, 1000 shared strings, all fictional.
- Command: `python3 -I -B scripts/scale-gen.py t <dir>`, which writes
  `<dir>/corpus/12ec7ca04a52.xlsx`; that file, copied here unchanged.
- Bytes: 29191.
- sha256: `12ec7ca04a52d9253e947e8f394d04b6d18ca1b133b90500955603a825805fdd`.

The test regenerates tier t into a temporary directory and asserts this
sha256 before it runs the ops, so the committed bytes are shown to be the
generator's.
