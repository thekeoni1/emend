# 0004: T1 xlsx-diff, the entry-level fidelity harness

Status: P1 to P4 DONE, verified by planning 2026-09-15 (P1: 8356f9a to
6ccae09, phase commits 2df5f6e and dc22f78; P2: a3166a5 to 24f4ef0,
phase commits 8d3c2aa, ac10ff6 and ed5f1f7; P3: f0adb45 to 827766a,
phase commits 9f124c7 and 401bb98; P4: 564af3b to b5180be, phase
commits 74b2bf0 and c35112b; P5: b58c086 to 4f6f347, phase commits
1c8eca1 and b3534cc). The T1 close-out push went after P4 (maintainer
decision, 2026-09-15). P6 READY, the last T1 phase. Runs after T0b. T2
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
| 14 | first differing offset in the uncompressed bytes (`changed` only); when the streams agree up to the shorter length, the shorter length |
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
- `attrs`: the external attributes, internal attributes, disk number
  start, version-made-by (whose high byte is the system) or
  version-needed-to-extract in the central record differ (internal
  attributes and disk number start from P5); or, from P6, the raw bytes
  of the central record's method, CRC-32, compressed size or
  uncompressed size field differ while the decoded values are equal
  (a zip64 placeholder on one side and a plain value on the other);
- `comment`: the entry comment differs;
- `local`: a local-only difference. A field of the local file header
  (version needed, general purpose flags, method, DOS time, DOS date,
  CRC-32, compressed size, uncompressed size) differs between the sides
  while the same field of the central records does not, or the name
  bytes in the local headers differ. A difference that the central
  records mirror is reported by the status or by the central flag
  alone (P4 added the flag over all local fields; P5 narrows it to
  local-only differences).

`descriptor` also covers, from P5, the bytes between the end of an
entry's compressed data and the next local header, or the central
directory for the last entry: a data descriptor's bytes, and any padding.

The flags `mtime`, `extra`, `gpflags`, `descriptor`, `attrs` and
`comment` read the central directory record's own bytes, plus the local
header's extra field for `extra`. With all seven flags as defined after
P6, exit status 0 means that every common entry is byte-equal from its
local header to the next header, that its central record is byte-equal
apart from the local header offset field, that the entries stand in the
same order in both central directories and in both files, and that the
archive comment is equal. A `files_identical` of `no` beside exit 0 can
then come only from a prefix or from end-of-central-directory and zip64
records other than the comment.

`archive` rows, after the entry rows: `archive<TAB>key<TAB>value`, in
this order:

| key | value |
|-----|-------|
| `entries_in`, `entries_out` | entry counts |
| `identical`, `recompressed`, `changed`, `dropped`, `added` | row counts by status |
| `flagged` | count of entries with a non-empty flags column |
| `order_preserved` | `yes` if the names present on both sides appear in the same relative order in both central directories and, from P6, in the same relative order by local header offset in both files; else `no` |
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
  `changed`; column 14 is the first differing offset, which is the
  shorter length when the bytes agree up to there; columns 15 and 16
  are empty (ruled on P2 departure 4; P3 implements it).
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
  against temporary files, never committed. The tool under test may be
  built and run from the main working tree behind a HEAD and clean-tree
  guard, with its archives in temporary directories (ruled on P2 gap 6,
  P3 gap 2). No fixture text may appear in any captured tool output;
  the printed generator may hold fictional content templates, and
  sentinel values are built at run time and never printed (ruled on P1
  gap 2 and P2 gap 3).
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

### Planning rulings on P2 (2026-09-15)

- P2 is verified from primaries by planning's own checks:
  - Doc commit a3166a5 holds exactly planning's four doc files (105
    insertions, 35 deletions), unchanged since. The three phase commits
    touch only `tools/xlsx-diff/`; the six record commits only
    `records/logs/t1-p2/`. All ten messages end with the Co-Authored-By
    trailer only, under the noreply identity. No mode changes, no
    tracked archive.
  - Planning ran the full gate (eight PASS) and `cargo test` on the host
    and on i686: 21 passed on each, same names.
  - Planning read the `lib.rs` diff and `tests/p2.rs` against this
    spec: one streaming pass through two 64 KiB buffers, the longer
    stream drained so both CRC-32 values are checked, declared-size
    checks, the encrypted check at open time, `u64` throughout, and
    error lines with no path, content or crate text.
  - Planning reproduced with its own archives and the built binary: a
    one-byte difference at offset 123457 (columns 14 to 16 `123457`,
    `1`, `123457`); the same bytes at deflate levels 1 and 9
    (`recompressed`); a 600-byte output against a 200000-byte input
    with byte 5 also differing (column 14 `600`, the P2 behaviour ruled
    on below); a CRC-32 field changed in both headers (exit 2, the
    decompress line); and a 256 MiB entry at two levels (`recompressed`,
    peak resident set 3540 kB, wall time about 5 s).
  - The seven record files, the five source files, `Cargo.lock` and
    the ten messages have zero hits on every scan; the `/home/u/` string
    in the scan logs is the scan's own control. The superseded logs in
    history hold no home path; the first I1 record (e472c39) holds the
    two lines with U+2192 named in its header and nothing else
    non-ASCII. `scripts/privacy.sh` over the 95 local commits: PASS.
- Departure 4 is not accepted as the final behaviour. Column 14 is the
  first differing offset in every `changed` case; with different
  lengths it equals the shorter length only when the bytes agree up to
  there. The column table and the Streaming section now say so, and P3
  implements it and changes the `earlier-byte` case of
  `changed_entry_with_different_lengths` to expect `5`. The P2 code and
  test were a fair reading of the old wording and are accepted as the
  P2 state.
- Departure 12 is accepted as committed: the check answer with four
  U+2192 characters written as `->`, with the positions, byte count and
  sha256 of the extracted bytes in the header. From now on the checker
  is asked to answer in plain ASCII, and any remaining non-ASCII
  character is replaced the same documented, reversible way (see "How
  this task runs" in `specs/0001-bootstrap.md`). The scan stays strict.
- Departures 1 to 3 and 5 to 11 are accepted as recorded. On 1: the
  wrong statement in 8d3c2aa's message stays in history and is
  corrected by ac10ff6 and by T's fixture section. On 5: the debug
  profile is the right one to bound; a release build uses less.
- Remaining gaps: 1 accepted, with a new rule that a rerun log's header
  names the committed log of the failed run it replaces; 2 ruled above;
  3 ruled above (the "Every phase writes" wording is clarified); 4, 5
  and 6 accepted. P3 tightens the three loose assertions in gap 5 while
  it touches those tests.

### P3: archive rows, flags, exit status

- All remaining `archive` rows and all six flags.
- Exit status exactly as in "Exit status".
- Column 14 for different lengths becomes the first differing offset
  (P2 departure 4 ruling): `changed_entry_with_different_lengths`
  expects `5` for its `earlier-byte` case and `600` for the other two.
- The three loose assertions from P2 gap 5 are tightened:
  `entry_of_exactly_two_buffers` asserts stderr and the status word in
  its changed section; the different-lengths test checks both sizes on
  every row; the large-entry test asserts columns 4 to 13.
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

### Planning rulings on P3 (2026-09-15)

- P3 is verified from primaries by planning's own checks:
  - Doc commit f0adb45 holds exactly planning's three doc files (82
    insertions, 10 deletions), unchanged since. The two phase commits
    touch only `tools/xlsx-diff/`; the four record commits only
    `records/logs/t1-p3/`. All seven messages end with the
    Co-Authored-By trailer only, under the noreply identity. No mode
    changes, no tracked archive.
  - Planning ran the full gate (eight PASS) and `cargo test` on the host
    and on i686: 31 passed on each, same names.
  - Planning read the `lib.rs` diff and `tests/p3.rs` against this
    spec: the central records read from their own bytes, the local
    extra field read at `header_start`, the six flags in order, every
    `archive` row in order, `order_preserved` from rising output
    indices, checked sums, `files_identical` through two buffers, the
    exit status rule, and column 14 as the first differing offset.
  - Planning reproduced with its own archives and the built binary:
    equal archives (exit 0, `files_identical` `yes`); a comment-only
    difference (exit 1, all `identical`, `comment_equal` `no`);
    reordered entries (`order_preserved` `no`); a changed DOS time
    (`mtime` on every entry, `flagged` 3); an extra field, external
    attributes and an entry comment each alone (`extra`, `attrs`,
    `comment`); a 10-byte prefix (`prefix_out` 10, exit 0); a 600-byte
    output with byte 5 differing (column 14 `5`); and the same file
    twice (exit 0). A first extra-field attempt used header id 0x9901,
    the AES id, and got the unsupported-feature error, which is right.
  - The six record files, the six source files, `Cargo.lock` and the
    seven messages have zero hits on every scan; the superseded logs at
    171f980 hold no home path and no non-ASCII byte. `scripts/privacy.sh`
    over the 102 local commits: PASS.
- The spec observation (local header time, flags and version needed,
  and the central version needed, are compared nowhere) is taken up: P4
  adds the flag `local` and extends `attrs`, as the Output section now
  says.
- Departures 1 to 8 are accepted as recorded. On 1: reading the central
  record's own bytes is the right choice; the Facts section's list of
  crate fields was a pointer, not a requirement. On 2: the `attrs` flag
  on the stored-against-deflated entry is real, since the writer's
  version-made-by differs.
- Remaining gaps: 1 is carried into P4 as a `files_identical` test whose
  only difference lies in a prefix longer than two buffers; 2 is
  confirmed, and the "Every phase writes" wording now says so; 3 is
  accepted, and the rerun rule in `specs/0001-bootstrap.md` now covers
  superseded as well as failed runs; 4 accepted.

### P4: no-leak test, zip64

- Sentinel test: fixture entries contain a sentinel string and sentinel
  bytes that are unique to the test, in their data, and the string also
  in entry and archive comments and in extra fields. The test runs
  `run` over every combination the tests use (identical, recompressed,
  changed, dropped, added, error paths) and asserts the string and the
  bytes are absent from stdout and stderr, verbatim, escaped and as
  hex. A sentinel entry name of non-printable bytes may appear only
  escaped: as the whole of column 2 of an `entry` row, and at most once
  in the one error line that names the entry. The check is a pattern
  check against known values, not a proof that nothing else can leak
  (ruled on P4 departure 2 and gap 3).
- zip64 test, `#[ignore]` by default: generate an archive with one entry
  of 4 GiB + 1 zero bytes, deflated, `large_file(true)`; copy it; change
  one byte at offset 4 GiB in the uncompressed data of the copy by
  regenerating with a one-byte difference; run the comparison and assert
  `changed`, column 14 equal to 4294967296, column 15 `1`, and sizes
  equal to 4294967297 on both sides. Run it explicitly on the host and
  on `i686-unknown-linux-musl` (`cargo test ... -- --ignored zip64`),
  and put both runs' output, wall time and peak RSS in `test.log`. The
  temporary files are deleted by the test.
- The seventh flag `local` and the `attrs` extension (P3 ruling), with
  tests on the byte-level builder: each of the local header's version
  needed, flags, time and date alone gives `local` and nothing else,
  except that a local flags change with the central flags equal is
  `local` alone; a central version-needed change alone gives `attrs`;
  a local name that differs from the central name is `local`.
- A `files_identical` test whose only difference is one byte inside a
  prefix longer than 131072 bytes on both sides (P3 gap 1): every
  entry `identical` with no flags, exit 0, `prefix_in` equal to
  `prefix_out`, `files_identical` `no`.
- The zip64 test writes its entry in fixed-size chunks and never holds
  the entry in memory, so it runs on i686.
- Negative case: run the sentinel test with the escaping deliberately
  disabled in a scratch copy, and show it fails.
- Acceptance: as P1, plus the zip64 test passes on both targets with the
  peak RSS under 64 MiB.

### Planning rulings on P4 (2026-09-15)

- P4 is verified from primaries by planning's own checks:
  - Doc commit 564af3b holds exactly planning's three doc files (96
    insertions, 11 deletions), unchanged since. The two phase commits
    touch only `tools/xlsx-diff/`; the four record commits only
    `records/logs/t1-p4/`. All seven messages end with the
    Co-Authored-By trailer only, under the noreply identity. No mode
    changes, no tracked archive.
  - Planning ran the full gate (eight PASS) and `cargo test` on the host
    and on i686: 35 passed and 1 ignored on each, same names.
  - Planning read the `lib.rs` diff and the outline of `tests/p4.rs`
    against this spec: the local header read with checked lengths, the
    `local` flag over bytes 4 to 25 and the name, `attrs` with the
    central version needed, the sentinel test's byte-level checks, and
    the zip64 test written in 1 MiB chunks.
  - Planning reproduced with its own archives and the built binary: a
    sentinel string in entry data, entry comments, extra fields and the
    archive comment, plus file names holding it, across identical,
    dropped, changed, comment-only, truncated and non-zip runs, with
    zero occurrences in any output; a local-header-only DOS time change
    (`local` alone); and a one-byte difference inside a 200000-byte
    prefix on both sides (exit 0, `files_identical` `no`).
  - The zip64 test: `test.log` records both explicit runs, 4294967297
    bytes on both sides and the difference at 4294967296, host 11 min
    11 s with a 5752 kB peak and i686 14 min 25 s with a 3344 kB peak.
    Planning ran the test itself on i686 from the built test executable
    under `/usr/bin/time -v`: passed, 14 min 36 s, peak 3344 kB.
  - The six record files, the seven source files, `Cargo.lock` and the
    seven messages have zero hits on every scan; the superseded logs at
    b11c221 hold no home path and no non-ASCII byte. `scripts/privacy.sh`
    over the 109 local commits: PASS.
- Departure 1 (`local` set beside every `changed` or `recompressed`
  entry, and beside `mtime` or `gpflags` whenever a writer repeats the
  field) is accepted as the P4 state and replaced in P5: `local` becomes
  a local-only difference, as the Output section now says.
- Departure 2 (the sentinel test's reading) is accepted; the P4 text
  now says what the test does. Departures 3 to 5 accepted.
- Gap 1 is the P5 work: `attrs` gains the internal attributes and disk
  number start, `descriptor` gains the bytes after the data, and the
  exit-0 paragraph is restated. Gaps 2 and 3 accepted, with the limit
  written into the P4 text. Gaps 4 and 5 accepted.

### P5: flag completeness (added on the P4 rulings)

- `local` narrowed to local-only differences, `attrs` and `descriptor`
  extended, exactly as the Output section says. Flag order unchanged.
- The P2, P3 and P4 tests updated to the narrowed `local`: a `changed`
  or `recompressed` entry whose local header mirrors its central record
  carries no `local`; a central and local time change is `mtime` alone.
- Tests on the byte-level builder: a local-only time change (`local`
  alone); a local-only flags change (`local` alone); internal attributes
  alone and disk number start alone (`attrs`); two entries with bit 3
  set on both sides whose descriptors differ only in the optional
  signature (`descriptor` alone, `gpflags` absent); padding bytes
  between two entries on one side (`descriptor` on the entry before
  the padding, every entry `identical`, exit 1); and the same padding
  case with the padding removed (exit 0).
- Negative cases: (a) an archive with padding between entries against
  its unpadded twin gives exit 1 with `descriptor` on one row; (b) the
  unpadded twin against a byte copy gives exit 0.
- Acceptance: as P1, plus every new flag condition exercised by a test,
  and the exit-0 paragraph true by construction: no byte from a local
  header to the next header, and no central record byte other than the
  offset field, is outside the status and the flags.

### Planning rulings on P5 (2026-09-15)

- P5 is verified from primaries by planning's own checks:
  - Doc commit b58c086 holds exactly planning's two doc files (22
    insertions, 2 deletions), unchanged since. The two phase commits
    touch only `tools/xlsx-diff/`; the four record commits only
    `records/logs/t1-p5/`. All seven messages end with the
    Co-Authored-By trailer only, under the noreply identity. No mode
    changes, no tracked archive.
  - Planning ran the full gate (eight PASS) and `cargo test` on the host
    and on i686: 38 passed and 1 ignored on each, same names.
  - Planning read the `lib.rs` diff and `tests/p5.rs` against this
    spec: the narrowed `local` over the eight shared fields against the
    raw central bytes, `attrs` with internal attributes and disk number
    start, the gap from each data end to the next local header or the
    central directory, compared through two buffers, and the P5 tests.
  - Planning reproduced with its own archives and the built binary: 37
    padding bytes inserted before the second local header with the
    offsets fixed (`descriptor` on the first row alone, exit 1); the
    padded archive against its byte copy (exit 0); a local-only DOS
    time change (`local` alone); the same change in both headers
    (`mtime` alone); internal attributes changed (`attrs` alone).
  - The six record files, the eight source files, `Cargo.lock` and the
    seven messages have zero hits on every scan; the superseded logs at
    97cb52d hold no home path and no non-ASCII byte. `scripts/privacy.sh`
    over the 118 local commits: PASS.
- Findings 1 and 2 are accepted as real and are the P6 work: `attrs`
  gains the raw-bytes case and `order_preserved` gains the file order,
  as the Output section now says. After P6, T1 is DONE, and further
  harness findings go to a backlog in `ROADMAP.md` unless they concern
  a real data flow: a corpus file in T3 or an emend output later. The
  same proportion rule that closed the privacy gate after T2 P0.
- Departures 1 to 4 accepted as recorded. On 1: the gap definition
  (empty past the central directory, measured to the first header at
  or after the data end otherwise) is right; P6 tests the overrun case.
- Remaining gaps: 1 and 2 are the P6 work; 3 is partly P6 work (the
  overrun case) and otherwise accepted (the zip64 test is rerun at the
  T1 backlog's discretion, not per phase); 4 accepted.

### P6: closing the exit-0 guarantee (added on the P5 rulings)

- `attrs` and `order_preserved` extended exactly as the Output section
  says. Flag order unchanged.
- Tests on the byte-level builder:
  - a zip64 pair: both sides carry a byte-equal zip64 extra field with
    the sizes, one side with 0xffffffff placeholders in the central
    size fields and the other with the plain values; the entry is
    `identical` with `attrs` alone, exit 1; the same placeholders on
    both sides give exit 0;
  - the local entries permuted in the file with the central order kept:
    every entry `identical` with no flags, `order_preserved` `no`,
    exit 1, `files_identical` `no`;
  - an entry whose data runs over the next local header on both sides
    alike (a stored entry whose declared compressed size overlaps the
    following header): no error, the overlapped bytes compared as data,
    the gap starting where the data ends, exit 0.
- Negative cases: (a) the placeholder pair gives exit 1 with `attrs`
  on one row; (b) the permuted pair gives exit 1 with `order_preserved`
  `no` and no flags.
- Acceptance: as P1, plus the exit-0 paragraph true by construction:
  every central record byte other than the offset field, every byte
  from a local header to the next header, and the order in both the
  central directory and the file, are inside the status, the flags and
  the archive rows.

### Close-out push (after P4)

The maintainer authorised the T1 close-out push after P4 verified, so
it goes before P5. It follows the T0b close-out steps
(`specs/0003-ci.md`, "Close-out push (after P2)"): planning's pending
doc updates committed unedited first; the hook, `core.hooksPath` and
`EMEND_PRIVATE_DENYLIST` checks and the full gate; the base recorded
before the push; `git push origin main`; `ls-remote` equal to local
main and the repository still PRIVATE; the CI run on the pushed commit
recorded, filtered as in T0b P1. Logs in `records/logs/t1-close/`:
`push.log`, `ci-run.log`, `scan.log`. No independent check, as for the
other close-out pushes. Report and STOP. P5's records go with the next
authorised push.

Planning ruling (2026-09-15): the close-out push is verified and DONE.
- GitHub `thekeoni1/emend` is PRIVATE, with one branch, `main` at
  83c9852, 110 commits and no tags. All 110 author and committer pairs
  are the noreply address. The Actions settings are unchanged.
- Doc commit 83c9852 holds exactly planning's two doc files (118
  insertions, 29 deletions), byte-identical to planning's versions.
  Record commit 343e91f touches only `records/logs/t1-close/`. Every
  message in 199b468..main ends with the Co-Authored-By trailer only.
- `push.log` recorded the base before the push (199b468, 33 commits),
  the eight PASS lines, the hook equal to the installer's, and then the
  hook's scan of `199b468..83c9852`, 33 commits and 205 blobs, 0
  findings, with git reporting `199b468..83c9852  main -> main`.
- Run 35017518231 on 83c9852 concluded `success` on `ubuntu-24.04`.
  Planning fetched and filtered its log itself: the seven PASS lines,
  then the SKIP privacy line, and `rustc 1.98.1 (48a229cea 2026-09-01)`.
- The three logs have zero hits on every scan. `scripts/privacy.sh`
  over the 111 local commits: PASS.
- Departures 1 and 2 accepted. The next phase is P5.

## Out of scope for T1

- Parsing any XML. The names-only XML comparison is Milestone 1 part 7.
- Reading real workbooks. T3 does that, in a subprocess.
- Any output format other than the rows above. A `--json` or a summary
  view can come later if a consumer needs it.
- Encrypted archives beyond reporting exit 2.

## Done

T1 is done when P6 verifies and `ROADMAP.md` says so; the close-out
push after P4 is recorded with a green CI run, and the P5 and P6 records
go with the next authorised push. From then on, harness findings go to
the T1 backlog in `ROADMAP.md` unless they concern a real data flow.
