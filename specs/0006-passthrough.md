# 0006: T3 passthrough baseline

Status: P1 DONE, verified by planning 2026-09-15 (d63ad31 to 33c0198;
phase commits 9eaee47, 79d17ba and cb94097). P2 READY. P3 and P4 wait
for the corpus (`specs/0005-corpus.md`).
Context: `CLAUDE.md`, `specs/0000-step-zero-plan.md` ("T3: passthrough
baseline (outline)", the part families, the record layout),
`specs/0001-bootstrap.md` ("How this task runs", "Rules for every
phase"; logs in `records/logs/t3-<phase>/`), `specs/0004-xlsx-diff.md`
(the harness's rows, flags and exit status).

## Goal

One measured run showing what a plain zip passthrough does to every
corpus workbook, on the host and on `i686-unknown-linux-musl`, plus the
size baseline the core's budget is measured against. Every entry is
expected `identical` with no flags; anything else is a finding about
real archives, or about the zip crate as a writer, that the greenfield
zip layer has to handle.

## Facts planning checked (2026-09-15)

- `zip` 8.6.0 offers `ZipWriter::raw_copy_file(ZipFile)`, which copies an
  entry's compressed bytes and its options (method, time, permissions,
  extra data, comment) into the output without decompressing, and
  `set_raw_comment` for the archive comment. What it changes in the
  headers is exactly what T3 measures.
- The corpus directory is empty as of this date. P1 and P2 use generated
  archives only; P3 and P4 need the corpus session's files and image.
- `/usr/bin/time -v` (GNU time) is on this machine and prints
  `Maximum resident set size (kbytes)`; `timeout` is coreutils.
  `python3` is available; the T2 scripts already rely on it.

## Design

### The copy tool

`tools/passthrough`, a binary crate `passthrough`, `publish = false`,
depending on `zip` exactly as `xlsx-diff` does and on nothing else.

`passthrough <input> <output>`:
- opens the input with `ZipArchive::new`, creates the output with
  `ZipWriter::new`, and for every index in order calls
  `raw_copy_file(archive.by_index_raw(i))`; then `set_raw_comment` with
  the input's comment bytes, then `finish`;
- writes the output to a temporary name beside the target and renames
  it into place on success, so a failed run leaves no partial output;
- prints nothing on stdout; on error one `error: ` line on stderr that
  never carries entry content or a file path (the same rules as
  `xlsx-diff`); exit 0 on success, 2 on error;
- every count and offset is `u64`; nothing is loaded whole.

Fault injection for the runner's tests, only when the environment
variable `PASSTHROUGH_FAULT` is set: `sleep` (sleep 30 s before
copying), `panic` (panic after opening), `abort` (`std::process::abort`),
`alloc` (allocate and touch 512 MiB before copying), `exit3` (exit 3).
The variable is documented as test-only, and the tool refuses to run
with it set unless `PASSTHROUGH_FAULT_ALLOWED=1` is also set, so a stray
value cannot change a corpus run.

### The runner

`scripts/passthrough-run.sh`, bash, `#!/usr/bin/env bash`, `set -uo
pipefail`, `/usr/bin/grep` explicitly. Arguments: `--label <host|i686>`,
`--passthrough <path>`, `--xlsx-diff <path>`, `--corpus <dir>` (default
`EMEND_CORPUS_DIR`, then `$HOME/emend-testing`), `--out <dir>`
(default `<corpus>/runs/<utc-date>-<shortsha>`), `--timeout <secs>`
(default 300), `--memory <MiB>` (default 2048, as a `ulimit -v` on the
child), `--ids <file>` (optional; one id per line, for a subset).

For every manifest row (or every listed id), in manifest order:
1. `mkdir -p <out>/<id>`; the input is `<corpus>/corpus/<id>.<ext>`.
2. Runs `timeout --signal=KILL <secs> /usr/bin/time -v -o <out>/<id>/time.<label>.txt <passthrough> <input> <out>/<id>/out.<label>.<ext>` with
   the memory limit, stdin closed, stdout and stderr captured to files.
3. Classifies the outcome from GNU time's report, with `time` outside
   `timeout` so the timeout's kill reaches the tool (P1 departure 4):
   `ok` (exit 0), `error` (exit 2), `timeout` (exit 124, recorded with
   signal 9), `panic` (exit 101), `crash` (a `Command terminated by
   signal` line, any other status, or no report), with the exit status
   and signal recorded as well. From P2, a temporary file the tool left
   behind (`.out.<label>.<ext>.passthrough-*.tmp`) is removed after the
   child ends and noted in a `leftover` column.
4. On `ok`, runs `<xlsx-diff> <input> <out>/<id>/out.<label>.<ext>`
   with stdout to `<out>/<id>/diff.<label>.tsv` and stderr captured,
   and records its exit status.
5. Appends one row to `<out>/summary.<label>.tsv` (below), and appends
   every `entry` row of the diff, prefixed by the id and the label, to
   `<out>/entries.<label>.tsv`.

`summary.<label>.tsv` columns, one header row: `id`, `label`,
`producer`, `tags`, `outcome`, `exit`, `signal`, `wall_s` (from
`Elapsed (wall clock)`), `peak_rss_kb`, `in_bytes`, `out_bytes`,
`diff_exit`, then the `archive` row values `identical`, `recompressed`,
`changed`, `dropped`, `added`, `flagged`, `order_preserved`,
`comment_equal`, `files_identical`. Empty where not applicable.

The runner prints one line per file to stdout, `id label outcome
diff_exit wall_s peak_rss_kb`, and a final `files N ok N error N timeout
N panic N crash N` line. It never prints an original name, a path under
the corpus directory, or a byte of content; error text from the child is
kept in the run directory, not printed.

### The record

`scripts/passthrough-record.py` (python3, standard library only) reads
`summary.<label>.tsv` and `entries.<label>.tsv` for both labels and
writes:
- `records/step0-<utc-date>-<shortsha>.tsv`: every summary row, both
  labels, plus every entry row, in a second table after a blank line,
  with the entry name escaped as the harness prints it;
- `records/step0-<utc-date>-<shortsha>.md`: the run's commit, date,
  targets, tool versions (`rustc --version`, the `zip` version), the
  count of files by outcome per label, then a table per label
  aggregating entry rows by part family (`specs/0000-step-zero-plan.md`
  lists the families; the mapping from entry name to family is a fixed
  table in the script, with `other` for the rest): entries, identical,
  recompressed, changed, dropped, added, flagged, and one column per
  flag name with its count; then the list of files whose outcome is not
  `ok` or whose diff exit is not 0, by id, producer and tags, with the
  archive row values that differ from the expectation.

The record cites a workbook only by id, producer and tags. Entry names
appear as the harness prints them.

### Size baseline

`tools/size-probe`, a binary crate with two binaries: `probe-zip`
(opens the archive named on the command line with `zip`, prints the
entry count) and `probe-zip-xml` (the same, then parses one entry named
on the command line with `quick-xml` in streaming mode and prints the
element count). Dependencies: `zip` exactly as `xlsx-diff`, and for the
second binary `quick-xml` at the current release with
`default-features = false`, behind a Cargo feature `xml` that is on for
that binary only. Its licence tree must pass the gate.

A profile `[profile.size]` in the workspace `Cargo.toml`: `inherits =
"release"`, `opt-level = "z"`, `lto = "fat"`, `codegen-units = 1`,
`panic = "abort"`, `strip = true`, `debug = false`. The baseline is the
size in bytes of each binary built with `--profile size --target
i686-unknown-linux-musl`, recorded in
`records/size-baseline-<utc-date>-<shortsha>.md` with the commit, the
toolchain, the `zip` and `quick-xml` versions and features, the two
sizes, and their difference (what `quick-xml` costs). Milestone 1
measures emend's core as its increase over `probe-zip-xml` built the same
way (maintainer ruling 4, `specs/0000-step-zero-plan.md`).

## Phases

Each phase ends as in `specs/0001-bootstrap.md`: phase commit(s), logs
in `records/logs/t3-<phase>/` (`gate.log`, `test.log` on both targets
where a Rust crate changed, `negative.log`, `scan.log`), at most two
independent checks in plain ASCII, report, STOP. Fixtures are generated
in test code or by the negative script into temporary directories and
never committed. No push during T3 phases; the T3 close-out push comes
after P4, or earlier on the maintainer's decision.

### P1: the copy tool and the runner, on the host

Deliverables:
- `tools/passthrough` as designed, with tests in `tools/passthrough/tests/`
  on archives generated with the `zip` crate and with the byte-level
  builder patterns of `xlsx-diff`'s tests (copied into this crate's
  `tests/common`, not shared across crates): stored and deflated
  entries, a directory entry, an entry with an extra field and a
  comment, an archive comment, leading junk bytes, a data descriptor
  entry, a zip64-marked entry of small size, 200 entries, and the empty
  archive. Each test copies the archive and runs `xlsx-diff::run` on
  the pair; it asserts the exit status and rows it gets, not that they
  are `identical`: the point of P1 is to learn what the zip crate's
  writer preserves. Every observed non-`identical` status or flag is
  listed in the report by fixture and flag, as P1's first finding table.
- Fault injection and its guard, tested through the binary.
- `scripts/passthrough-run.sh` and `scripts/passthrough-record.py`,
  exercised in `negative.log` on a scratch corpus directory the script
  builds under `<scratch>` with a manifest of generated archives (the
  `EMEND_CORPUS_DIR` override), never the real one:
  - a normal file: `ok`, diff exit recorded, rows appended;
  - `PASSTHROUGH_FAULT=sleep` with `--timeout 2`: `timeout`;
  - `panic`: `panic`, exit 101; `abort`: `crash` with the signal;
    `exit3`: `crash` with exit 3; `alloc` with `--memory 256`: `error`
    or `crash`, whichever the allocation failure produces, recorded;
  - the fault variable without `PASSTHROUGH_FAULT_ALLOWED`: the tool
    refuses with exit 2 and the runner records `error`;
  - the record script over that run: the `.md` and `.tsv` written under
    `<scratch>`, with the family table and the non-`ok` list, and no
    original name in either (the scratch manifest carries a sentinel in
    `original_name` and `note`).
- Acceptance: gate all PASS; tests pass on both targets; every negative
  case as stated; the P1 finding table in the report.

### Planning rulings on P1 (2026-09-15)

- P1 is verified from primaries by planning's own checks:
  - Doc commit d63ad31 holds exactly planning's four doc files (279
    insertions, 10 deletions), unchanged since. The three phase commits
    touch only `Cargo.toml`, `Cargo.lock`, `tools/passthrough/` and the
    two scripts; the five record commits only `records/logs/t3-p1/`. All
    ten messages end with the Co-Authored-By trailer only, under the
    noreply identity. No mode change beyond the two new executable
    scripts, no tracked archive. `Cargo.lock` adds only `passthrough`.
  - Planning ran the full gate (eight PASS) and `cargo test` on the host
    and on i686: 69 passed and 1 ignored on each, same names.
  - Planning read `main.rs`, the runner and the record script's outline
    against this spec: the raw copy in order with the comment, the
    temporary file and rename, the fault guard, `time` outside `timeout`,
    the manifest's private columns never read, the runner's stderr
    silenced after the argument checks, and the record script never
    reading the manifest.
  - Planning reproduced with its own archives: a Python-built deflated
    archive with a comment copied with exit 0, then `attrs` on every
    entry, because the writer turned external attributes 0o600 into
    0o100600 (the regular-file type bits added), a row to add to the
    finding table; a data-descriptor archive copied with `gpflags,
    descriptor,attrs,local`; the runner on a scratch corpus of two
    generated files through the `EMEND_CORPUS_DIR` override (both `ok`,
    diff exit 1); the sleep fault with `--timeout 2` (`timeout`, and no
    passthrough process alive a second later); the record script over
    that run (a family table of ten entries, no sentinel from the
    manifest's private columns in any run output or record file). It
    refuses a run directory whose name is not `<utc-date>-<shortsha>`
    without `--stem`, and an output directory that does not exist.
  - The seven record files, the nine source files and the ten messages
    have zero hits on every scan; the superseded logs at ba79abf and
    8d202cd hold no home path and no non-ASCII byte. `scripts/privacy.sh`
    over the 134 local commits: PASS.
- The finding table is accepted as P1's result and carries into
  Milestone 1: the zip crate's raw copy is not a byte-preserving writer.
  It drops unknown extra fields, zip64 fields and leading bytes, rewrites
  data descriptors into the local header, normalises version-made-by,
  version-needed and external attributes, and keeps only the last of
  duplicate names. Milestone 1 part 1 therefore writes local headers and
  central records itself, from the input's own header bytes for
  untouched entries, and uses the crate for reading at most. The
  expectation for the corpus run changes accordingly: every entry
  `identical` (data preserved) with the writer's flags catalogued; any
  `changed`, `dropped`, `added`, error or crash is a finding about real
  archives.
- Departure 4 is accepted: GNU `timeout` exits 124 unless asked to
  preserve the status, and a signal-9 report is a kill from outside, so
  `crash` is right for it. The runner text above now says 124.
- Departures 1 to 3 and 5 to 9 accepted as recorded.
- Remaining gaps: 1 is the ruling above; 2, 3 and 4 are P2 work (below);
  the rest of 4 accepted.

### P2: i686 runs and the size baseline

- The runner used with the i686 builds of `passthrough` and
  `xlsx-diff` on the same scratch corpus, `--label i686`; the record
  script merges both labels.
- `tools/size-probe`, the `size` profile, the size record, and the
  `zip` feature and licence tree cited from the gate's own listing
  (`cargo tree -e normal --prefix none` and the licences step).
- Runner fixes from the P1 rulings: a leftover temporary file removed
  after the child ends and noted in a `leftover` column (the record
  script reads it); every command that can fail guarded with `|| die`
  now that the runner's stderr is silent (the copy to the selected
  list, the `--ids` scratch file, the removals, a new header's write);
  the saved stderr descriptor closed for the children. A negative case
  shows the sleep fault under `--timeout 2` during a real copy (a large
  generated archive) leaving no temporary file and `leftover` `yes`.
- Negative cases: a `probe-zip-xml` build with `quick-xml`'s default
  features on, in a scratch copy, showing the size delta the defaults
  cost (recorded, not committed); the record script refusing a run
  directory with only one label unless `--allow-single` is given.
- Acceptance: gate all PASS, including licences with `quick-xml` in the
  tree; the two sizes recorded with the commit.

### P3: LibreOffice open check

Scoped when the corpus session has delivered its image (T2 P2). Outline:
the runner gains `--libreoffice <image>` and, for every `ok` output,
converts it headless to ODS in the container; "opens without repair"
means exit 0, no load error on stderr, and the sheet count of the
conversion equal to the input's `xl/workbook.xml` sheet count. The
result is a `lo` column in the summary.

### P4: the first recorded run

After the corpus session's first report and P3: the runner over the
whole corpus on both targets, the record committed as
`records/step0-<utc-date>-<shortsha>.md` and `.tsv`, and planning's
ruling on the findings. Raw outputs stay under the corpus directory's
`runs/`.

## Out of scope

- Any edit to a workbook. T3 copies only.
- Excel checks (maintainer ruling 3).
- Rewriting entries, zip64 output, or fixing what the zip crate's writer
  changes. Those are Milestone 1 part 1's questions; T3 records the facts
  they need.
