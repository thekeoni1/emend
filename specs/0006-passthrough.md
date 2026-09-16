# 0006: T3 passthrough baseline

Status: T3 DONE 2026-09-16. P1 to P3 verified by planning 2026-09-15 (P1: d63ad31 to
33c0198, phase commits 9eaee47, 79d17ba and cb94097; P2: 8bbda4d to
b1e7982, phase commits 2845fab and 6a7533e; P3: fd5dcf9 to 2638d21,
phase commits 17ec824 and 2623b70). The close-out push after P2 is
recorded. P4 verified by planning 2026-09-16 (39defb7 to 2222c66, phase
commits 39defb7 and 4a4867b); the baseline record is
`records/step0-2026-09-16-39defb7.md` and `.txt`; the P3 and P4
records go with the next authorised push.
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
`alloc` (allocate and touch 512 MiB before copying), `exit3` (exit 3),
and, from P2, `sleep-copying` (sleep 30 s after the first entry is
copied, while the temporary file exists; for an empty archive, before
the comment). The first five run after the input is open and before the
temporary file exists.
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
   behind is removed after the child ends and noted in a `leftover`
   column. The glob was `.out.<label>.<ext>.passthrough-*.tmp` until
   Milestone 1 part 1 P1 widened it to `.out.<label>.<ext>.*.tmp` (the
   ruling below), so the column is evidence for any tool the runner is
   given.
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
- `records/step0-<utc-date>-<shortsha>.txt` (tab-separated; `.txt`
  because the hygiene gate refuses a tracked spreadsheet extension and
  `.gitignore` ignores `*.tsv`, P4 ruling): every summary row, both
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
where a Rust crate changed, `negative.log`, `scan.log`, and in P4
`run.log` and `record.log`), at most two independent checks in plain ASCII, report, STOP. Fixtures are generated
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
  shows the `sleep-copying` fault under `--timeout 2` on a generated
  archive leaving no temporary file and `leftover` `yes` (P2 ruling: the
  P1 faults all run before the temporary file exists).
- Negative cases: a `probe-zip-xml` build with `quick-xml`'s default
  features on, in a scratch copy, showing the size delta the defaults
  cost (recorded, not committed; `quick-xml` 0.42.0's default feature
  set is empty, so the expected delta is 0 bytes, P2 ruling); the
  record script refusing a run directory with only one label unless
  `--allow-single` is given.
- Acceptance: gate all PASS, including licences with `quick-xml` in the
  tree; the two sizes recorded with the commit.

### Planning rulings on P2 (2026-09-15)

- P2 is verified from primaries by planning's own checks:
  - Doc commit 8bbda4d holds exactly planning's two doc files (81
    insertions, 9 deletions), unchanged since. The two phase commits
    touch only `Cargo.toml`, `Cargo.lock`, `tools/size-probe/`,
    `tools/passthrough/` and the two scripts; the four record commits
    only `records/logs/t3-p2/` and the size record. All seven messages
    end with the Co-Authored-By trailer only, under the noreply
    identity. `Cargo.lock` adds only `quick-xml` 0.42.0 and `size-probe`.
  - Planning ran the full gate (eight PASS, with `quick-xml` in the
    licences listing) and `cargo test` on the host and on i686: 70
    passed and 1 ignored on each, same names. `size-probe` builds and
    passes clippy with the `xml` feature on both targets.
  - Planning built the size probes itself with `--profile size` for
    i686: 473636 and 539172 bytes, both stripped static i386 ELF, and
    `probe-zip-xml` byte-identical to the log's (sha256 52d1932f...).
  - Planning read the two probes, the profile, the runner diff and the
    record script against this spec.
  - Planning reproduced on its scratch corpus: the `sleep-copying`
    fault under `--timeout 2` (`timeout`, `leftover` `yes`, no temporary
    file, no passthrough process alive a second later) and the record
    script refusing a single-label run directory, then writing it with
    `--allow-single`, with no manifest sentinel in any output.
  - The seven record files, the size record, the nine source files and
    the seven messages have zero hits on every scan; the superseded logs
    and the replaced size record at 434e714 hold no home path and no
    non-ASCII byte. `scripts/privacy.sh` over the 141 local commits:
    PASS.
- The size baseline stands: 473636 bytes for `zip` alone, 539172 bytes
  for `zip` plus `quick-xml`, a difference of 65536 bytes. As the report
  says, the difference is page-aligned and bounds the `quick-xml` cost.
  Milestone 1 measures the core against 539172 bytes at this toolchain
  and profile, remeasured at the commit it is compared at.
- Departures 1 to 8 accepted as recorded; the two mid-phase rulings
  (the `sleep-copying` fault, the zero delta) are now in the spec text.
- Remaining gaps 1 to 5 go to the passthrough backlog in `ROADMAP.md`;
  none concerns a data flow this project has.

### Close-out push (after P2)

The maintainer decided to push after P2 and start the implementing
session afresh. The push follows the T0b close-out steps
(`specs/0003-ci.md`, "Close-out push (after P2)"): planning's pending
doc updates committed unedited first; the hook, `core.hooksPath` and
`EMEND_PRIVATE_DENYLIST` checks and the full gate; the base recorded
before the push; `git push origin main`; `ls-remote` equal to local main
and the repository still PRIVATE; the CI run on the pushed commit
recorded. Logs in `records/logs/t3-close/`: `push.log`, `ci-run.log`,
`scan.log`. No independent check. Report and STOP. The record commit
goes with the next push.

Planning ruling (2026-09-15): the close-out push is verified and DONE.
- GitHub `thekeoni1/emend` is PRIVATE, one branch `main` at 29b0bad, 142
  commits, no tags, every author and committer the noreply address, the
  Actions settings unchanged.
- Doc commit 29b0bad holds exactly planning's two doc files (79
  insertions, 10 deletions). Record commit 24d4a16 touches only
  `records/logs/t3-close/`. Every message in 83c9852..29b0bad ends with
  the Co-Authored-By trailer only.
- `push.log` recorded the base before the push (83c9852, 32 commits),
  the eight PASS lines, the hook equal to the installer's, and the
  hook's scan of `83c9852..29b0bad`, 32 commits and 244 blobs, 0
  findings. Run 35032688348 on 29b0bad concluded `success` on
  `ubuntu-24.04`; planning filtered its log itself: seven PASS lines,
  then the SKIP privacy line, `rustc 1.98.1`.
- The three logs have zero hits on every scan. `scripts/privacy.sh` over
  the 143 local commits: PASS.
- The implementing session that built T0 to T3 P2 ends here; the
  maintainer starts a fresh one.

### P3: LibreOffice open check

Scoped 2026-09-15, after the corpus session delivered its image (T2 P2,
verified by planning: image `localhost/emend-lo:25.2.3-2-deb13u6`, id
`3f9baefc6e23...`, LibreOffice 25.2.3.2 from Debian package
`4:25.2.3-2+deb13u6` on a base pinned by digest; the pins are in the
corpus directory's `tools/libreoffice.md`, to be copied into
`records/libreoffice-<date>.md` in this phase).

- The runner gains `--libreoffice <image>`. For every `ok` output it
  runs, with `podman run --rm --network=none --userns=keep-id`, the
  output's directory mounted read-only as `/in` and a scratch directory
  as `/out`, a headless conversion of the copy to ODS under the same
  `timeout`. The image's entrypoint supplies `soffice` and its headless
  flags, so the runner passes only `--convert-to ods --outdir /out
  /in/<file>` (P3 ruling; `records/libreoffice-<date>.md` names the
  flags so the command is reproducible from the record).
- "Opens without repair" is pinned as: the conversion completes within
  the timeout; its stderr holds no line containing `Error` or `Warning`
  other than the exact line `Warning: failed to launch javaldx - java
  may not function correctly`, which the image prints on every run
  because it has no Java by design (P3 ruling; matched as a whole
  line; any other wording fails); the ODS file exists and is a zip
  whose `content.xml` holds as many `<table:table ` elements as the
  input's `xl/workbook.xml` holds `<sheet ` elements (count only; P3
  ruling). Chart sheets count as sheets. The conversion's exit status
  is not a condition: it is 0 even when nothing loads (measured in P3).
- The result is an `lo` column after `leftover` in the summary: `ok`,
  `fail` (LibreOffice refused the file or the ODS is wrong, with the
  reason kept in the run subdirectory, never printed), `error` (the
  check could not run: the directory, podman itself, python3, an
  unreadable ODS; P3 ruling, so a broken tool is never read as a broken
  archive) or `-` (no image given or no `ok` output). The record script
  reads it, counts `fail` and `error` per label, and lists both by id
  in separate sections.
- The same check is run once on the input itself, recorded as
  `lo_in`, so a file LibreOffice cannot open on either side is told
  apart from one the copy broke.
- `records/libreoffice-<date>.md`: the pins and versions copied from the
  corpus directory's record, with the sha256 of that private file.
- Tests: none in Rust (no crate changes). Negative cases in
  `negative.log`, in the scratch corpus: the check passes on a
  generated workbook; fails on a copy with one worksheet entry
  truncated (a deliberately corrupted output); fails on a copy whose
  `xl/workbook.xml` names a sheet that has no part; and `-` when no
  image is given. The container's stderr never reaches the log; only
  counts and `ok`/`fail` do.
- Acceptance: gate all PASS; every negative case as stated; the
  LibreOffice record committed.

### Planning rulings on P3 (2026-09-15)

- P3 is verified from primaries by planning's own checks:
  - Doc commit fd5dcf9 holds exactly planning's two doc files (62
    insertions, 30 deletions), unchanged since. The two phase commits
    touch only the runner, the record script and
    `records/libreoffice-2026-09-15.md`; the six record commits only
    `records/logs/t3-p3/`. All nine messages end with the Co-Authored-By
    trailer only, under the noreply identity.
  - Planning ran the full gate: eight PASS.
  - Planning read the runner's `lo_check`, the record script's changes
    and the LibreOffice record against this spec and the mid-phase
    rulings: the whole-line allowance, the entrypoint tail form, the
    count-only condition, the `error` value with both greps' exit codes
    read, the reasons kept in the run subdirectory, the image inspected
    in the gate log rather than the record quoting itself.
  - Planning ran the runner with `--libreoffice` over two real corpus
    files (one LibreOffice export, one Microsoft template) into a
    scratch directory: both `ok`, every entry `identical`, `lo` and
    `lo_in` `ok` on both, no reason file; the LibreOffice file's twelve
    entries carried `gpflags,descriptor,attrs,local` (data descriptors
    rewritten) and the template's twenty-four `extra,attrs` (extra
    fields dropped), as P1's finding table predicts. The record script
    wrote the record with the new columns.
  - The six record files, the LibreOffice record and the two scripts
    have zero hits on every scan; the superseded logs at 9be61d6 and
    bd8c581 hold no non-ASCII byte and no pattern hit. `scripts/privacy.sh`
    over the 160 local commits: PASS.
- The mid-phase rulings are now in the P3 text above: the javaldx
  allowance, the entrypoint tail form, count only, the `error` value,
  and the exit status carrying no evidence. The two measured findings
  stand in `records/libreoffice-2026-09-15.md`: LibreOffice substitutes
  an empty sheet for a missing or unreferenced part, so a dropped
  worksheet part is caught by `xlsx-diff`'s `dropped` rows, not by this
  check.
- Check 1's grep-error inversion was real and is fixed. Check 1's
  answer carried the bare shape of a privacy pattern; it was committed
  with one documented, reversible replacement (the specs/0001 mechanism
  extended to a pattern hit, P3 ruling), and T0 P6d narrows the path
  patterns so this does not recur.
- Departures accepted: the corrupted-input form of the truncated case,
  the negative script running from the main tree behind its guards, and
  the two record commits added instead of amending.
- Check 2's gaps are P4 work, listed under P4: the stale "ok, fail or
  -" refusal and the case that pins it; the error/fail boundary (a
  timeout or kill is `error`; the conversion's own result is evaluated
  first; a count step that cannot read the input or the ODS afterwards
  is `error`); the unexercised `error` branches; `error` rows listed
  under a heading that says failing; `both_listed` without `-a`; the
  gate asserting the entry-script hash against the record rather than
  printing it.

### P4: the first recorded run

After T0 P6d. Pre-run work first, each with a negative case:
- the P3 carry-overs listed in the P3 rulings (the stale refusal text,
  the error/fail boundary as ruled, the `error` branches exercised, the
  record's headings and `both_listed`, the entry-script hash asserted);
- a separate `--lo-timeout <secs>` (default 600) for each conversion,
  since a LibreOffice cold start on the two `large` files may exceed
  the copy timeout; a conversion that times out is `error`;
- a whitespace-tolerant `<sheet` match (a producer may break the line
  after the tag name), and the same for `<table:table`;
- a chart-sheet workbook generated in the scratch corpus and measured
  (measured in P4: LibreOffice writes a `<table:table` for a chart
  sheet, so the count rule stands unchanged; the conditional this
  bullet first carried did not fire and was struck after the phase).

Then the runner over the whole corpus (27 files as of 2026-09-15, plus
whatever the maintainer adds) on both targets with the LibreOffice
check, the record committed as `records/step0-<utc-date>-<shortsha>.md`
and `.txt`, and planning's ruling on the findings. Raw outputs stay
under the corpus directory's `runs/`. The expectation, after P1's
finding table: every entry `identical` with the writer's flags
catalogued; any `changed`, `dropped`, `added`, error, crash, or `lo`
`fail` on an `lo_in` `ok` file is a finding about real archives; `lo`
`error` is a finding about the tooling, listed apart.

### Planning rulings on P4 (2026-09-16)

- P4 is verified from primaries by planning's own checks and is DONE
  (39defb7 to 2222c66; phase commits 39defb7 and 4a4867b):
  - Doc commit 0c2caf2 holds exactly planning's three doc files (73
    insertions, 13 deletions). The phase commits touch only the runner
    and the record script; record commit 25e553a only the record pair;
    the other five only `records/logs/t3-p4/`. All nine messages end
    with the Co-Authored-By trailer only, under the noreply identity.
  - Planning ran the full gate (eight PASS) and `scripts/privacy.sh`
    over the 35 local commits since the last push (269 blobs, PASS),
    and scanned the eleven new or changed files itself: no non-ASCII
    or control byte, no U+2014, no email beyond the scan's own
    `example.invalid` control, no hit under the live or the pre-P6d
    pattern forms, no denylist hit, no hostname.
  - Planning read the private run directory: 27 summary rows per
    label, all `ok`, diff exit 1, `files_identical` `no`, `lo` and
    `lo_in` `ok`, no leftover; 870 entry rows per label, all
    `identical`; flags `attrs` 870, `extra` 578, `gpflags`,
    `descriptor` and `local` 292 each; no reason file. Planning's own
    row, name and status digest is equal for the two labels. The
    committed `.txt` is byte for byte the two summaries followed by
    the two entries files; the 27 ids are the manifest's; no original
    name, note, producer version or application string appears in the
    record pair.
  - Planning re-ran the two repaired counting functions from the final
    gate and negative logs against an unreadable file: both print
    `error`, and the same forms count 1 on their controls.
  - The negative log holds 57 results, none unexpected: the forced
    timeout is a real conversion returning 124 in 1.0 s, the signal
    kill 137, podman 125, the count-step error, the chart sheet with
    both counts (2 and 2), the fourteen tag shapes, the record's four
    counts and headings, the mutated-copy guard, and the `--lo-timeout`
    refusal.
- The result is the P4 expectation and the step-zero baseline: every
  entry identical on both targets, with the zip writer's flags
  catalogued (`attrs` on every entry; `extra` on the 578 entries of the
  ten Microsoft templates; `gpflags`, `descriptor` and `local` on the
  292 entries of the seventeen LibreOffice files, as planning recounted
  by producer from the run directory), and every workbook opening in
  LibreOffice as input and as copy. No finding about a real archive; nothing listed apart.
  Milestone 1's zip layer is measured against
  `records/step0-2026-09-16-39defb7.txt`: its runs must show every
  untouched entry identical AND unflagged.
- Mid-phase rulings, now in force: the error ladder (124, 125 to 127
  and 128 or more are `error`; 1 to 123 `fail`; 0 decides nothing);
  `--lo-timeout` per conversion, independent of `--timeout`; the
  record named after the commit the run was made at; `error` rows only
  under the could-not-run heading, with four counts per column summing
  to the file count; only the record pair enters the repository; the
  heavy run in the foreground, serialized, builds finished first; the
  negative script in a scratch clone with the gate's binaries copied
  in (no cargo); the record's table as `.txt`; `run.log` never edited
  or rerun, the regeneration recorded in `record.log`.
- The chart-sheet conditional was measured by both sessions and did
  not fire (LibreOffice writes a `<table:table` per chart sheet); the
  P4 text above now says so.
- Accepted residue, log-only, no data flow, all to the passthrough
  backlog: `record.log`'s corpus-name count keeps the bare `grep -c`
  form check 2 named (its inputs are files the script itself wrote
  moments before); `record.log` line 1 is the command that read the id
  on line 2; the gate's corpus-name control asserts the grep form, not
  the function. The status 1 to 123 `fail` branch has no positive
  control on the pinned image (every conversion exits 0). The record's
  four-condition heading lists all 54 rows on the diff-exit condition.
- The implementing session's own findings stand as reported: three
  repairs declared complete after fixing one instance of a class; a
  padded commit id caught before anything ran; check 2's finding that
  the check answers were not committed, repaired at f4e03b0. New rules
  in `specs/0001-bootstrap.md`: commit ids in logs are read from git;
  a committed log is never edited; nothing is called measured unless
  a log prints it.
- T3 is DONE. The P3 and P4 records go with the next authorised push,
  together with T0 P6d and T2 P1.

### Planning ruling for Milestone 1 part 1 P1 (2026-09-16): the leftover glob

The implementing session found, before its first corpus run with
`tools/emend-copy`, that both leftover globs in
`scripts/passthrough-run.sh` (the clearing before the child at line
294 and the accounting after it at line 309) name the passthrough
tool's temporary form, `.out.<label>.<ext>.passthrough-*.tmp`, while
`emend-copy` names its temporary `.<name>.emend-copy-<pid>.tmp`. With
the runner unchanged the `leftover` column would read `no` for every
file whatever happened, and `specs/0008-zip-layer.md` claimed the
runner takes any copy tool unchanged. Planning verified both globs,
both tools' names and the claim.

Ruling, given before any run:

- Departure approved: both globs become `"$dir/.out.$LABEL.$ext."*.tmp`,
  tool-neutral, with no other change to the runner. Naming the new
  tool's temporary after the old tool was refused: a check that
  exists to catch debris must not be satisfied by disguising it.
- The runner change goes into part 1 P1's one code commit, with the
  crate and the tool; the report names the runner hunk. This spec and
  `specs/0008` carry the ruling in a second doc commit, made before
  the code commit and holding only these two files.
- P1's `negative.log` shows the widened check working, in a scratch
  clone with the built binaries copied in, each with its control: a
  stand-in tool given through `--passthrough` that writes the output
  and leaves `.out.<label>.<ext>.stand-in-<pid>.tmp` (`leftover` `yes`
  and the file gone afterwards); the same leaving the old
  `passthrough-` form (`yes`, the regression pin); the same leaving
  nothing (`no`); and a stale `.out.<label>.<ext>.emend-copy-1.tmp`
  planted before the run, removed by the clearing step with `leftover`
  still `no`, since only this child's file counts. The last case
  closes the "stale-temporary-file removal has no negative case" item
  of the passthrough backlog on `ROADMAP.md`. No fault hook is added
  to `emend-copy` for this; the stand-in exercises the runner.

- Any edit to a workbook. T3 copies only.
- Excel checks (maintainer ruling 3).
- Rewriting entries, zip64 output, or fixing what the zip crate's writer
  changes. Those are Milestone 1 part 1's questions; T3 records the facts
  they need.
