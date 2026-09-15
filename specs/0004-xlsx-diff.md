# 0004: T1 xlsx-diff, the entry-level fidelity harness

Status: P1 DONE, verified by planning 2026-09-15 (8356f9a to 6ccae09;
phase commits 2df5f6e and dc22f78). P2 READY. Runs after T0b. T2
(corpus, `specs/0005-corpus.md`) runs alongside it.
Context: `CLAUDE.md`, `specs/0000-step-zero-plan.md` (the T1 outline),
`specs/0001-bootstrap.md` ("How this task runs" and "Rules for every
phase"; the logs directory is `records/logs/t1-<phase>/`).

## Goal

`xlsx-diff` compares two zip archives entry by entry and says, for every
entry, whether it is byte-identical, recompressed, changed, dropped or
added, without ever printing entry content. It is the instrument behind
every fidelity claim emend will make (CLAUDE.md, "The fidelity harness is
the acceptance instrument"). T3 runs it over the corpus; Milestone 1 runs
it on every edit test.

## Facts planning checked (2026-09-15)

- `zip` 8.6.0 (MIT, `rust-version` 1.88) with `default-features = false`
  and the single feature `deflate-flate2-zlib-rs` resolves to nine other
  crates on the host and on `i686-unknown-linux-musl`: cfg-if 1.0.4,
  crc32fast 1.5.2, equivalent 1.0.2, flate2 1.1.10, hashbrown 0.17.1,
  indexmap 2.14.2, memchr 2.8.3, typed-path 0.12.3, zlib-rs 0.6.8. Every
  licence is MIT, Apache-2.0, Zlib or Unlicense OR MIT. No `-sys` crate,
  no build script that needs a C compiler. A trivial consumer built on
  both targets with toolchain 1.98.1. The gate's native and licences
  steps prove this again in P1.
- `deflate-flate2` alone gives flate2 with no backend and is not usable.
  `deflate` (the default) adds zopfli, which only compresses. The feature
  above gives both decompression and compression, and the tests need
  compression to make fixtures.
- The reader API used: `ZipArchive::new`, `len`, `by_index`,
  `by_index_raw`, `offset`, `comment` (bytes), `central_directory_start`;
  on an entry, `name_raw`, `compression().to_u16()`, `size`,
  `compressed_size`, `crc32`, `last_modified`, `extra_data`, `comment`,
  `data_start`, `header_start`, `central_header_start`; and the
  re-exported `HasZipMetadata::get_metadata` for the raw central record
  (`flags`, `using_data_descriptor`, `external_attributes`,
  `version_made_by`, `system`, `central_extra_field`, `large_file`).
  `by_index_raw` reads the compressed bytes without decompressing.

## Design

### Crate layout

- `tools/xlsx-diff` keeps its name and stays `publish = false`.
- `src/lib.rs` holds everything; `src/main.rs` only parses `argv`, calls
  `xlsx_diff::run(input, output, &mut stdout, &mut stderr)` and maps the
  result to the exit status. Tests call `run` with in-memory writers, so
  they can assert on everything the tool prints.
- Dependency: `zip = { version = "=8.6.0", default-features = false,
  features = ["deflate-flate2-zlib-rs"] }`, exact version, as above. No
  other dependency, dev-dependencies included. Fixtures are built with
  the same `zip` crate inside the tests.

### Output

Everything goes to stdout as tab-separated rows. Every row starts with
its kind. There is no header row; consumers select rows by kind.

`entry` rows, one per entry name across both archives, in this order:
the input's entries in the input's index order, then entries only the
output has, in the output's index order.

| Column | Meaning |
|--------|---------|
| 1 | `entry` |
| 2 | name, escaped (below) |
| 3 | status: `identical`, `recompressed`, `changed`, `dropped`, `added` |
| 4, 5 | index in input, index in output (0-based; empty when absent) |
| 6, 7 | compression method id in input, in output (the zip method number, `0` stored, `8` deflate) |
| 8, 9 | uncompressed size in input, in output |
| 10, 11 | compressed size in input, in output |
| 12, 13 | CRC-32 in input, in output, 8 lowercase hex digits |
| 14 | first differing offset in the uncompressed bytes (`changed` only) |
| 15 | count of differing bytes (`changed` with equal uncompressed sizes only) |
| 16 | last differing offset (same condition as column 15) |
| 17 | flags, comma-separated, empty when none (below) |

Columns that do not apply are empty. Sizes, offsets and counts are `u64`
and are printed in decimal.

Statuses for an entry present on both sides:
- `identical`: method, CRC-32, uncompressed size, compressed size and
  the raw compressed bytes are all equal.
- `recompressed`: not `identical`, but the uncompressed bytes are equal.
- `changed`: the uncompressed bytes differ (or their lengths differ).

Flags, set only for entries present on both sides, whatever the status:
- `mtime`: the DOS modification date or time differs;
- `extra`: the local extra field or the central extra field differs;
- `gpflags`: the general purpose bit flags differ;
- `descriptor`: one side uses a data descriptor and the other does not;
- `attrs`: the external attributes, version-made-by or system differ;
- `comment`: the entry comment differs.

`archive` rows, after the entry rows: `archive<TAB>key<TAB>value`, in
this order:

| key | value |
|-----|-------|
| `entries_in`, `entries_out` | entry counts |
| `identical`, `recompressed`, `changed`, `dropped`, `added` | row counts by status |
| `flagged` | count of entries with a non-empty flags column |
| `order_preserved` | `yes` if the names present on both sides appear in the same relative order in both archives, else `no` |
| `comment_equal` | `yes` if the archive comments are byte-equal, else `no` |
| `prefix_in`, `prefix_out` | bytes before the first local header (`ZipArchive::offset`) |
| `central_start_in`, `central_start_out` | offset of the central directory |
| `uncompressed_in`, `uncompressed_out` | sums of uncompressed sizes |
| `compressed_in`, `compressed_out` | sums of compressed sizes |
| `bytes_in`, `bytes_out` | file sizes |
| `files_identical` | `yes` if the two files are byte-equal, else `no` |

Names are printed from the raw name bytes: bytes 0x20 to 0x7E other than
`\` as themselves, `\` as `\\`, and every other byte as `\xNN` with two
lowercase hex digits. Entries are paired by raw name bytes. Directory
entries (names ending in `/`) are compared like any other entry.

Nothing else is ever printed on stdout. Errors go to stderr as one line
starting with `error: `. An error line may name an entry (escaped as
above) but never quotes entry content.

### Exit status

- 0: every entry row is `identical` with an empty flags column,
  `order_preserved` is `yes` and `comment_equal` is `yes`.
- 1: the archives were compared and any difference exists.
- 2: an error. Usage error; a file that cannot be opened or is not a zip;
  a duplicate raw name within one archive; an encrypted entry; a
  decompression or CRC error; an unsupported compression method in an
  entry that must be decompressed. For an error, the rows already
  written stay written; the consumer trusts stdout only when the exit
  status is 0 or 1.

### Streaming

- Uncompressed bytes are compared by reading both entries through
  fixed 64 KiB buffers, side by side, never loading an entry whole. The
  first differing offset, the count and the last differing offset come
  from that single pass. A shorter stream on one side makes the status
  `changed` with column 14 set to the shorter length and columns 15 and
  16 empty.
- Raw compressed bytes are compared the same way through
  `by_index_raw`, only when method, CRC-32 and both sizes are equal.
- `files_identical` is a chunked byte comparison of the two files after
  the sizes are known equal.
- Every offset, size and count is `u64` from the first line of code.

## Phases

Each phase ends as in `specs/0001-bootstrap.md` "How this task runs":
phase commit(s), logs in `records/logs/t1-<phase>/` in record commits,
at most two independent checks, report, STOP. Every phase writes:

- `gate.log`: the full `scripts/check.sh`, all eight PASS.
- `test.log`: `cargo test --workspace` on the host and with
  `--target i686-unknown-linux-musl`, colour off, with the test names and
  the summary lines. The i686 run must execute the same tests as the
  host run, and the log states both counts.
- `negative.log`: the phase's negative cases, run in a scratch copy or
  against temporary files, never committed. No fixture text may appear.
- `scan.log`: per the scan rules, over the phase diff, the commit
  messages and the other logs.

Fixtures are generated in test code from fictional content, with the
`zip` crate, into temporary directories, and never committed
(`.gitignore` already covers office extensions; test zips end in `.zip`
and live only in temp dirs). No real workbook is used in T1.

Nothing is pushed during T1 phases. The T1 close-out push, after P4
verifies, needs planning's word and the maintainer's authorisation, as
in `specs/0003-ci.md` "Close-out push (after P2)", and records its CI run.

### P1: crate layout, dependency, inventory

Deliverables:
- The `zip` dependency exactly as above; `Cargo.lock` updated for it and
  nothing else. `cargo tree -e normal --prefix none` for the workspace
  shows the ten crates listed under "Facts", and `gate.log` shows
  `PASS native` and `PASS licences`.
- `src/lib.rs` with `run`, the name escaping, and the `entry` rows with
  columns 1 to 13 filled and 14 to 17 empty. Statuses in P1: `dropped`,
  `added`, and for entries on both sides `identical` (definition above,
  including the raw byte comparison) or the provisional `differs`. P2
  replaces `differs`.
- `src/main.rs` reduced to argument handling and exit mapping. Usage
  error: the usage line on stderr, exit 2.
- `archive` rows `entries_in`, `entries_out`, `identical`, `dropped`,
  `added` (and, for P1 only, `differs`). The rest come in P3.
- Exit status: 0 when every row is `identical`, 1 when any row is not,
  2 on error.
- Tests in `tools/xlsx-diff/tests/` using generated zips: two equal
  archives (all `identical`, exit 0); an entry dropped; an entry added;
  the same content recompressed at a different level or method (P1
  status `differs`); a non-ASCII and a backslash in a name (escaping);
  a duplicate name (exit 2); a non-zip file (exit 2); a missing file
  (exit 2); a stored entry beside a deflated one.
- Negative cases: run the tool by hand on two generated archives and
  show (a) that flipping one byte in the compressed stream of the output
  file makes that entry `differs`, and (b) that truncating the output
  file gives exit 2 with an `error: ` line and no `entry` rows of
  content. Case (a) is done on a copy of the file with `dd` or a small
  script, and the log shows the byte offset and the rows.
- Acceptance: gate all PASS; all tests pass on both targets; the two
  negative cases behave as stated; the escaping test covers `\x00`,
  `\x7f`, `\xc3` bytes and `\\`.

### Planning rulings on P1 (2026-09-15)

- P1 is verified from primaries by planning's own checks:
  - Doc commit 8356f9a holds exactly planning's five doc files (529
    insertions, 16 deletions), unchanged since. Phase commits 2df5f6e
    and dc22f78 touch only `Cargo.lock` and `tools/xlsx-diff/`; the four
    record commits touch only `records/logs/t1-p1/`. All seven messages
    end with the Co-Authored-By trailer only, under the noreply identity.
  - `Cargo.lock` holds xlsx-diff, zip 8.6.0 and exactly the nine crates
    under "Facts". No tracked zip, office or fixture file.
  - Planning ran the full gate: eight PASS. Planning ran `cargo test` on
    the host and on i686: 12 passed on each, same names.
  - Planning read `lib.rs`, `main.rs` and `tests/p1.rs` against this
    spec: the escaping, the column layout, the `identical` definition
    with the raw comparison through `by_index_raw` and two 64 KiB
    buffers, the provisional `differs`, the exit statuses, the
    duplicate-name detection from the central directory, and error
    lines that carry no path, no content and no crate text.
  - Planning reproduced, with its own generated archives and the built
    binary: equal archives (exit 0, all `identical`); one byte flipped
    inside a compressed stream (that entry `differs`, exit 1, empty
    stderr); a truncated output (exit 2, one `error: ` line, no rows);
    and both archives with a 10-byte prefix (exit 0, all `identical`),
    which confirms the absolute `central_directory_start` in departure
    7. No fixture text appeared in any output.
  - The six record files, the four source files, `Cargo.lock` and the
    seven messages have zero hits on every scan. `scripts/privacy.sh`
    over the 85 local commits: PASS. The superseded first-run logs at
    4d17a68 hold no home path either.
- Departures 1 to 11 are accepted as recorded. On 11: an ad hoc loop
  must never run `chmod` over a file list; the mode change was caught
  by check 1 and reverted in dc22f78.
- Remaining gap 1 (the short-stream error in `raw_equal` is untested)
  is carried into P2 as a required test. Gaps 2 and 3 are accepted:
  the fixture text is fictional, and the label is narrower than its
  check but the exit check beside it closes the hole. Gap 4 is closed
  by planning's scan above.
- The note on the T0b P2 log scripts is accepted: three checks of the
  form `$(yn [ ... ] && grep ...)` captured only their first test, and
  the literal lines they were meant to test are printed in the same
  logs and were verified by planning on GitHub. From now on a compound
  check is evaluated inside one function that returns yes or no (see
  "Rules for every phase" in `specs/0001-bootstrap.md`).

### P2: content comparison

- `recompressed` and `changed` replace `differs`. Columns 14 to 16 are
  filled by the streaming pass; the archive rows `recompressed` and
  `changed` replace `differs`.
- Tests: the short-stream error in `raw_equal` (an output whose local
  entry data is cut short while its central record keeps the declared
  sizes gives exit 2 and one `error: ` line; carried over from P1);
  recompressed entry (equal bytes, different raw stream);
  changed entry with equal length (first, count and last offsets
  asserted exactly, including a difference at offset 0 and at the last
  byte); changed entry with different lengths (column 14 is the shorter
  length, 15 and 16 empty); an entry larger than the 64 KiB buffer with
  differences in the first, a middle and the last chunk; an entry with a
  wrong CRC (exit 2); an encrypted entry (exit 2; the fixture is made
  with the crate's ZipCrypto writer if the feature set allows it,
  otherwise with hand-set general purpose bit 0, and the log says which).
- Negative cases: (a) an output whose entry differs by exactly one byte
  at a known offset reports that offset in column 14 and `1` in column
  15; (b) the peak resident set of a run over an archive holding one
  256 MiB entry of generated bytes stays under 32 MiB on the host
  (`/usr/bin/time -v`, `Maximum resident set size`), showing that no
  entry is loaded whole.
- Acceptance: as P1, plus the offset assertions and the memory bound.

### P3: archive rows, flags, exit status

- All remaining `archive` rows and all six flags.
- Exit status exactly as in "Exit status".
- Tests: reordered entries (`order_preserved` `no`, every entry
  `identical`, exit 1); a changed archive comment; a changed entry
  mtime, extra field, comment and external attributes, each alone; a
  data descriptor on one side; `files_identical` `yes` on a byte copy
  and `no` otherwise; `prefix_in` non-zero for an archive with leading
  junk bytes.
- Negative cases: (a) two files that differ only in the archive comment
  give exit 1 with all entries `identical`; (b) the same file given twice
  gives exit 0 and `files_identical` `yes`.
- Acceptance: as P1, plus every flag and archive row exercised by a
  test.

### P4: no-leak test, zip64

- Sentinel test: fixture entries contain a sentinel string and sentinel
  bytes that are unique to the test. The test runs `run` over every
  combination the tests use (identical, recompressed, changed, dropped,
  added, error paths) and asserts the sentinel is absent from stdout and
  stderr, including in error messages for an encrypted or corrupt entry
  whose name is the sentinel. A name containing the sentinel IS printed
  (names are metadata); the test asserts it appears only escaped in
  column 2 and nowhere else.
- zip64 test, `#[ignore]` by default: generate an archive with one entry
  of 4 GiB + 1 zero bytes, deflated, `large_file(true)`; copy it; change
  one byte at offset 4 GiB in the uncompressed data of the copy by
  regenerating with a one-byte difference; run the comparison and assert
  `changed`, column 14 equal to 4294967296, column 15 `1`, and sizes
  equal to 4294967297 on both sides. Run it explicitly on the host and
  on `i686-unknown-linux-musl` (`cargo test ... -- --ignored zip64`),
  and put both runs' output, wall time and peak RSS in `test.log`. The
  temporary files are deleted by the test.
- Negative case: run the sentinel test with the escaping deliberately
  disabled in a scratch copy, and show it fails.
- Acceptance: as P1, plus the zip64 test passes on both targets with the
  peak RSS under 64 MiB.

## Out of scope for T1

- Parsing any XML. The names-only XML comparison is Milestone 1 part 7.
- Reading real workbooks. T3 does that, in a subprocess.
- Any output format other than the rows above. A `--json` or a summary
  view can come later if a consumer needs it.
- Encrypted archives beyond reporting exit 2.

## Done

T1 is done when P4 verifies, the close-out push is recorded with a green
CI run, and `ROADMAP.md` says so.
