# 0023: Milestone 3 part 1, proof at scale on i686

Status: APPROVED by the maintainer 2026-10-01 ("a on all":
decisions 1 to 4 as planning recommended, 5 and 6 recorded), sent
to the implementing session the same day.

Context: Milestone 3's outline (`ROADMAP.md`, approved 2026-10-01)
puts this part first because it alone can change the design of the
others. `CLAUDE.md` says the core streams where the format allows
it and that a workbook may be bigger than the address space would
like, on a target where `usize` is 32 bits. Every corpus workbook
is under 8.3 MB and every corpus run's peak RSS is under 8 MB, so
nothing measured so far has tested the claim. This part measures
it on a generated workbook up to a zip64 entry, under the memory
cap the harness already applies, and records which paths stream
and which load a part whole, so parts 2 and 5 build on what is
known. No new capability.

## Goal

Four generated workbooks, fictional and deterministic, never
committed; every sample of `emend-copy` run over each on the host
and on `i686-unknown-linux-musl` under a 2048 MiB address-space
cap; the harness records as for the corpus, with peak RSS and wall
time; a decision record naming each code path's memory class as
measured, the findings, and what the later parts must design
around.

## Facts planning checked (2026-10-01)

- `scripts/passthrough-run.sh` already applies `ulimit -v` of
  `--memory` MiB (default 2048) and `timeout` of `--timeout` s
  (default 300) to the tool, records `/usr/bin/time -v`'s peak
  RSS in kB and the wall time per run, takes `--corpus <dir>`
  with its own `manifest.tsv`, `--ids`, `--tool-arg`, and runs the
  LibreOffice check only when `--libreoffice` names an image. The
  diff (`xlsx-diff`) runs outside the cap.
- `scripts/passthrough-record.py` reads the run directory only
  and never the manifest; the record names workbooks by id (the
  input sha256's first 12 hex), producer and tags.
- `Package::cells` streams: a `Cells` iterator over the entry's
  reader. `Package::shared_strings` loads the table whole. The
  comments and drawing parts are read whole (`read_to_end`), which
  is by design for parts of their size. Whether an edited
  worksheet and the shared strings part are rewritten by streaming
  is for the P1 survey to state from the code, function by
  function.
- The archive layer reads and writes zip64 records and writes
  them only when a value exceeds its field (`archive.rs`).
- `emend-copy` has 25 `--<name>-sample` flags; `--scale-sample` is
  taken (the colour-scale rule sample), so this part adds no
  sample and names nothing "scale" in the tool.
- The corpus run records' columns: `wall_s`, `peak_rss_kb`,
  `in_bytes`, `out_bytes` beside the outcome and diff columns.
  Over the part 4d P4 image run the largest peak RSS is 7996 kB.
- Disk: 578 GB free under `/home/dev/emend-testing`. Python
  3.14.4 on the box. 22 GB of RAM on the host.

## Design

### The generator

`scripts/scale-gen.py`, committed, Python 3.14.4, standard library
only (`zipfile` with zip64 allowed), deterministic from a fixed
seed, writing under a directory the caller names outside the
repository (the maintainer's `/home/dev/emend-testing/scale/`).
It writes each tier's workbook by streaming rows into the zip
entry, never holding a sheet in memory, so the generator itself
runs within the cap it tests. Content is fictional: numbers from
the seed, strings drawn from a vocabulary of 1000 made-up words
held in the script, formulas of the form `=A1+B1` with cached
values, dates as serials, booleans; `docProps/core.xml` with
creator and lastModifiedBy "corpus", no company. The script refuses
a target under the repository and a tier it does not know, and
writes a `manifest.tsv` in the corpus manifest's form (producer
`scale-gen`, producer_version the script's own version string,
tags `scale,tier<letter>` (no hyphen: the runner's tag form is
`[a-z0-9,]`, P1 ruling 2), source `generated` (P1 ruling 3),
original_name the tier's file name, note empty) so the runner
takes the directory as a corpus.

The workbook: two worksheets. The first is the tier's large sheet,
24 or 48 columns per row (12 or 24 numbers, 6 or 12 shared
strings, 3 or 6 formulas with cached values, 2 or 4 date serials
with a date style, 1 or 2 booleans), rows from 1. The second is a
10 by 10 sheet of numbers so the sheet and shift samples have a
small target. One `styles.xml` with a date number format, one
`sharedStrings.xml`, no theme beyond the minimal one `Package::new`
embeds (the template of `specs/0016`, read through the crate so the
generator and the crate agree on the parts), no calc chain.

### The tiers

| Tier | Rows | Columns | Strings | Sheet XML, counted | Purpose |
| --- | --- | --- | --- | --- | --- |
| A | 50000 | 24 | vocabulary | 45838595 bytes | LibreOffice opens it; the baseline every sample is compared against |
| B | 800000 | 24 | vocabulary | 760275485 bytes | past any sensible buffer, under the cap |
| C | 1048576 | 120 | vocabulary | about 4.7 GiB (P1 ruling 1; the P1 record prints the count) | one entry over 4 GiB: zip64 sizes, over the 32-bit address space |
| D | 800000 | 24 | every string unique | 778893118 bytes, shared strings 166996261 | the shared strings table loaded whole, measured |

Tier C's row count is Excel's maximum; its 120 columns are five
blocks of the 24-column form, since the form runs about 38 bytes
a cell and 48 columns reach only 1.9 GiB (P1 ruling 1; planning's
draft said 4.5 GiB at 48 columns from a guessed 96 bytes a cell).
The counts are the implementing session's, from the generator's
row function before any zipping, reprinted by the P1 record. The
archive sizes are what deflate makes of them, recorded, not
designed. Each tier's sha256 and byte count go in the P1 record;
the runs cite them.

### The runs

`scripts/passthrough-run.sh` unchanged, `--corpus` the scale
directory, `--memory 2048` on both labels (so the host and the
32-bit target are measured alike), `--timeout 3600`, one run per
sample per label per tier: 24 samples (`--new-sample` never reads
its input and `specs/0016` keeps it off the corpus runs; P1
verification ruling 3), two labels, four tiers, 192 runs, with
`TMPDIR` on the disk (ruling 4). `--libreoffice` on tier A only
(decision 2); tiers B to D record `lo not-run`. The diff runs with
the host `xlsx-diff` for both labels on tier C (decision 4),
since the tool reads an entry whole and the entry is larger than
a 32-bit address space; the run log names the binary's sha256 as
built, and the runner writes it into each run directory's
`run.<label>.tsv` (the record script does not carry it; P2
verification ruling 3). The run directory is
`runs/<utc-date>-<shortsha>-scale-<tier>`, one
per tier, both labels in it, as the corpus runs are laid out.

### What is measured and how it is read

For every run: outcome, exit, wall time, peak RSS, input and output
bytes, the diff columns. The run log then derives, per sample:

- the memory class: `streams` when the peak RSS on tier B is less
  than twice tier A's and under 64 MiB; `loads` otherwise, with the
  part it loads named by the P1 survey;
- the tier C outcome under the cap: `ok`, or `error` with the
  message, or killed by the cap (exit 137 or a panic on
  allocation), each a finding, not a failure of the run;
- the throughput: output bytes over wall time, per label.

The P1 survey predicts the class of each sample from the code
before any run; the run log prints prediction beside measurement
and counts the disagreements. A disagreement is a finding recorded
in P3, never a reason to edit the prediction.

### What a finding leads to

- A defect: a wrong output, a panic, a refused input that the
  format allows (a zip64 entry refused, a `u32` overflow), an
  entry not byte-identical that no sample touched. Fixed in this
  part as a correctness fix in the core, measured by the size log,
  within the 23528 of headroom; its test added; the runs repeated
  with the fix named (`CLAUDE.md`, "Nothing is re-run until it
  passes").
- A design limit: a path that loads a part whole and so fails
  under the cap on tier C, or needs memory in proportion to a
  part. Not changed here. Named in the decision record with the
  function and the part, and carried into the spec of the part
  that touches that path (parts 2 and 5 for the read side; part 6
  for comments and drawings), where a streaming form is designed
  or the limit is stated in the catalogue.
- The shared strings table loaded whole on tier D: expected; its
  peak RSS per unique string is the figure recorded, and the
  decision record says whether a consumer at that scale needs a
  streaming resolver (a design question for part 5).

### The catalogue

The `package` header gains one block, "What streams and what is
held", stating from the decision record which reads and rewrites
run in memory independent of a part's size and which hold a part,
with the tier C figures beside each. The claim in `CLAUDE.md` is
then a measured one.

### Size

No probe changes. The size log at P3 rebuilds the twenty-one
probes and the core's growth is expected 0; a correctness fix
changes that and is accounted for by its commit. The size record
is `records/size-m3p1-<utc-date>-<shortsha>.md`.

### 32-bit

Every size and offset the generator writes into the manifest and
the runner reads is a byte count. Tier C's archive is deflated and
stays under 2^32 bytes (1159829654 in), so the figures over `u32`
are its sheet entry's uncompressed sizes, in and out; the run log
checks that those, and the tool's `in_bytes` and `out_bytes`, are
printed whole (a control: a planted 32-bit truncation of the
figure is caught). The draft named `in_bytes` and `out_bytes` as
the figures over `u32`; P2 verification ruling 4 corrected it.

## Phases

Each phase as `specs/0001`: the doc commit, the code commits, one
record commit, the scan last, at most two independent checks, the
report, STOP.

### P1: the generator and the survey

The doc commit (this spec, the Milestone 3 outline in
`ROADMAP.md`, the part 4d close-out rulings in `specs/0022`, the
two carried script fixes noted in `specs/0022` close-out ruling 8
go into this part's close-out scripts, not here). Then
`scripts/scale-gen.py` with a tiny tier `t` (100 rows) that its
own test generates into a temporary directory and opens through
the crate (`Package::sheets`, `cells` count, `shared_strings`
count), the refusals (a target under the repository, an unknown
tier, an existing file not overwritten without `--force`) in the
negative log; the four tiers generated into the scale directory
with their manifest, each tier's sha256, byte count and generation
time in the P1 record `records/m3p1-gen-<utc-date>-<shortsha>.md`;
the survey, in the same record: for each of the 25 samples, the
crate calls it makes, and for each call whether the part is
streamed or held, with the function names, from the code, the
predicted class per sample and tier C's predicted outcome. Logs:
gate, test, negative, scan.

### P2: the runs

The runs as "The runs" says, tier A first with LibreOffice, then
B, D, C. Records `records/m3p1-scale-<tier>-<utc-date>-<shortsha>
.md` and `.txt` from the record script, one pair per tier. The run
log holds the derivations of "What is measured and how it is
read" and the 32-bit control. A defect found here stops the phase:
it is recorded, fixed in a named commit, and the runs repeated. A
design limit is recorded and the runs go on. Logs: gate, test (the
workspace, unchanged unless a fix), run, scan.

### P3: the decision record, the catalogue, the size record

`records/scale-m3p1-<utc-date>-<shortsha>.md`: the memory class of
every sample on every tier, predicted and measured, the
throughput per label, the findings each with its kind (defect
fixed, design limit carried, expected), and the carries into
parts 2, 5 and 6 by name. The catalogue block. The size log and
record. Both corpus samples that part 4d added run once more on
the corpus with the size log's binaries, byte-identical to part
4d's runs by sha256, so the part's only crate change (the
catalogue, or a fix) is shown harmless. The open record of the
keep.

### Close-out push (after P3)

As part 4d's, with `specs/0022` close-out ruling 8's carries: the
scan script prints the `ANTHROPIC_API_KEY` check; the ci-run
script ends its JSON output with a newline; the push script's
previous-record pattern names part 4d's record commit ab5b3aa
(`^M2 part 4d close-out record`); base part 4d's close-out doc
commit fe584b3 resolved by its subject, tip this part's close-out
doc commit; one independent check before the report. Before the
push, the Excel check for Milestone 2 (every corpus row of every
Milestone 2 sample, on the laptop), deferred by the maintainer
five times, last on 2026-10-01, is run (Milestone 2's acceptance
record then follows Milestone 1's form) or deferred again by a
decision recorded here (decision 6).

## Planning rulings during P1 (2026-10-01)

1. Tier C's size. The implementing session counted the sheet XML
   of every tier with the generator's row function: the form the
   spec gives runs about 38 bytes a cell, so 1048576 rows by 48
   columns make 2002424253 bytes, under 2^31, and no zip64 entry.
   Planning's 4.5 GiB was a guess at 96 bytes a cell, wrong. The
   maintainer's decision 1 is the zip64 case over the 32-bit
   address space, not a column count. Ruling: tier C is 1048576
   rows by 120 columns, five blocks of the 24-column form, the
   content form unchanged (no padding of cells, which would make
   the tier measure a form no workbook has). The P1 record prints
   the counted size, expected about 4.7 GiB, and the run log
   checks that the entry's uncompressed size exceeds 2^32 before
   tier C counts as the zip64 case. The table above is corrected.
2. Tags. The runner's manifest check takes `[a-z0-9,]` only, so
   the tags are `scale,tier<letter>` with no hyphen. The runner is
   not changed for a hyphen.
3. The manifest's source column: `generated`. The runner and the
   record script never read it; `specs/0005`'s fixed list is the
   corpus's and a generated set adds one word to it, recorded here.
4. The tier `t` test: a committed `scripts/scale-gen-test.py` that
   generates tier `t` into a temporary directory and runs the
   gate's `emend-copy` on both targets through the flags that go
   through `Package::sheets`, `cells` and `shared_strings`,
   asserting the counts (2 sheets, 2500 cells, 1000 shared
   strings, every operation byte-identical), run by `test.log`.
   Accepted: through the tool is through the crate, and Python
   stays out of `cargo test` and CI. The negative log holds the
   generator's refusals as "P1" says.
5. These rulings are written into the working spec after the doc
   commit 48c6e0d; P1's logs pin the committed spec, 7b1eb997; the
   rulings go in P2's doc commit.

## Decisions for the maintainer

1. The tiers. A (recommended): the four tiers as drafted, 200
   runs, tier C the zip64 case over the 32-bit address space. B:
   tiers A, B and D only, no zip64 case, which leaves the
   `CLAUDE.md` claim untested where it matters most.
2. LibreOffice. A (recommended): the open check on tier A only;
   tiers B to D are beyond what the pinned image opens in any
   budget worth waiting for, and the check proves the output
   opens, which tier A's does for the same code paths. B: tier A
   and B with a long budget.
3. The cap and the budget. A (recommended): 2048 MiB address space
   on both labels, 3600 s per run. B: no cap on the host, which
   would make the host figures a different measurement.
4. The diff on tier C. A (recommended): the host `xlsx-diff` for
   both labels on tier C, recorded; the tool holds an entry whole
   and this part does not change the tool. B: skip the diff on
   tier C and trust the tool's own read-back, which loses the
   byte-identity proof on the one tier it matters.
5. Corpus P5 and P6. Recorded, not asked: the maintainer agreed
   on 2026-10-01 to supply them before part 1; this part does not
   use them, so they are wanted before part 2's scoping at the
   latest, through the corpus session under its brief.
6. The Excel check for Milestone 2, Milestone 2's acceptance:
   deferred five times, to run before this part's close-out push
   at the latest, or be deferred again by a decision recorded
   here. Asked on 2026-10-02 after P3's verification; the
   maintainer deferred it a sixth time ("Defer again"). This
   part's close-out pushes without it; Milestone 2 stays
   ACCEPTANCE PENDING, and the check (every corpus row of every
   Milestone 2 sample, on the laptop) is due before Milestone 3
   part 2's close-out push at the latest, or deferred again by a
   decision recorded in that part's spec.

## Planning rulings during P2 (2026-10-01)

1. Run directories. The runner takes one row per id per label and
   the record script one run directory per call, so 24 samples
   cannot share a tier's directory. Ruling: one run directory per
   tier per sample, `runs/<utc-date>-<shortsha>-scale-<tier>-
   <sample>`, 96 directories, both labels in each, as part 4d P4
   laid its two samples out; one record pair per directory,
   `records/m3p1-scale-<tier>-<sample>-<utc-date>-<shortsha>.md`
   and `.txt`, 96 pairs, each small since a tier has nine entries;
   `run.log` derives the per-sample tables across them. The runner
   and the record script are unchanged. "The runs" and "P2" read
   with this ruling.
2. The grid check's name is `in_grid` (package.rs:8968); P1
   verification ruling 5 said `on_grid`. The fix uses `in_grid`.
3. The `TMPDIR` proof: the runner driving a stand-in tool that
   records whether `TMPDIR` is set to the export, in a scratch run
   directory, printed by `run.log` as a control before the runs.
   Accepted.
4. These rulings are written into the working spec after the doc
   commit 38ec8c2; P2's logs pin the committed spec, 30b61067; the
   rulings go in P3's doc commit.

## Planning verification and rulings on P1 (2026-10-01)

Verified by planning from primaries: the doc commit 48c6e0d holds
this file at 7b1eb997, `ROADMAP.md` at ed534cce and `specs/0022`
at bfec1e8f, the hashes planning sent; the code commit a509ad3
adds exactly the two scripts and the record commit e2ff972 the
six logs and the record; 3 of 3 messages end with the
Co-Authored-By line only, ASCII; the logs and the record at the
reported hashes, each log ending all as expected, the scan RESULT:
PASS, every log pinning 7b1eb997; `scripts/scale-gen.py` read in
full: standard library, one seed, the vocabulary made of syllables
in the script, the row form as "The generator" gives it, rows
streamed in batches of 1000 into the entry, zip64 forced on tier
c's sheet, core properties "corpus", the refusals, no string or
value ever printed; `scripts/scale-gen-test.py` read in full;
planning's own sha256 of the four workbooks in the scale
directory equal to the manifest's and the record's, its own read
of each archive's central directory (9 entries, the sheet sizes
45838595, 760275485, 5025024826 over 2^32 with the zip64 field,
778893118, the shared strings 166996261 on tier d, creator and
lastModifiedBy "corpus", no company); the survey's cited lines
read at a509ad3 (`new_sample` writing its first path, the five
`Cursor` second writes in the read-backs of validate, cf, table,
comment and names, `set_cell` recording with no check, `on_grid`
and `INDEX_OUT_OF_RANGE` present for other calls, `TempFile` in
`env::temp_dir`); planning's own privacy scan ab5b3aa..e2ff972,
3 commits, 911 blobs, 0 findings; `ANTHROPIC_API_KEY` unset.

1. The independent checks' remaining items: the report's "blobs"
   for content hashes, the "(attempt 2)" labels in negative.log's
   third attempt, the negative log's HEAD check without the
   short-sha control the other logs carry, no negative case for a
   sibling path sharing the repository's prefix. Cosmetic or
   allowed by `specs/0001`; accepted, no change. The next phase's
   negative log carries the short-sha control.
2. The vocabulary is generated syllables; a word that happens to
   spell a name names no one, and no string is ever printed. The
   content is fictional as `CLAUDE.md` requires. Accepted.
3. Finding 1, `--new-sample`: it never reads its input and writes
   `Package::new()` to its first path by `specs/0016`'s design,
   which keeps it off the corpus runs. Ruling: P2 runs 24 samples,
   192 runs; the tool is not changed; "The runs" is corrected.
4. Finding 2, the temporaries: `TempFile` honours `TMPDIR`, and
   `/tmp` is a tmpfs the cap and peak RSS cannot see. Ruling: the
   run script exports `TMPDIR` as a directory on the disk under
   the scale directory, mode 700, printed as a placeholder; the
   run log counts the files and bytes there before and after every
   run (0 after a completed run; a killed run's leftovers counted,
   removed and logged); the peak temporary size on tier c is the
   code's reading, stated as such in the decision record and not
   measured. If the runner does not pass `TMPDIR` through to the
   tool, the implementing session holds and reports.
5. Finding 3, a cell recorded past the grid: a defect. The write
   refuses a recorded cell value, clear or style whose reference is
   off the grid with "a row or column index or count is out of
   range" (`on_grid` and the message exist for other calls), in
   P2's first code commit before any run, with tests on both
   targets and the catalogue sentence; the six samples are then
   refused on tier c at the write, as insert-rows is. No corpus
   sheet reaches the grid, so every corpus output is unchanged by
   construction; P3's rerun of part 4d's two samples shows it on
   two paths and the tests on the rest. Measured by the size log at
   P3 as a correctness fix.
6. Finding 4, tier c's fewer write paths: recorded. The decision
   record lists the rewrite paths tier c measured (the zip64
   entry rewritten by edit-formula, delete-rows, insert-columns,
   delete-columns, link and protect, if the runs bear it out) and
   says the rest were measured on tiers b and d.
7. The names sample's predicted kill on tier c is a hold in the
   tool (the read-back's second write into memory, part 4a's
   sample design), its own class "holds in the tool", not a crate
   limit; recorded in P2 and the decision record; the tool is not
   changed in this part.
8. The amend of the code commit before any log, the superseded
   attempts kept and named by hash, and the first generation's
   identical hashes as an unlogged aside: recorded, no change.
9. P1 is DONE. P2 is released: its doc commit carries this file
   with the P1 rulings and this section and `ROADMAP.md`'s part 1
   section; its first code commit is ruling 5's fix; the runs as
   "The runs" with rulings 3 and 4, in the order a with
   LibreOffice, b, d, c, both labels each, the host `xlsx-diff`
   on tier c for both labels.

## Planning verification and rulings on P2 (2026-10-02)

Verified by planning from primaries: the doc commit 38ec8c2 holds
this file at 30b61067 and `ROADMAP.md` at 75fe5ce7, the hashes
planning sent; the code commit 8d05845 touches `package.rs` and
its tests only (84 lines added, 6 removed): the one check in
`Parts::validate` over the recorded cell and style edits through
`in_grid`, the three doc comments, the catalogue sentence and the
test; the record commit 079db61 adds the five logs, the check and
96 record pairs, 197 files; 3 of 3 messages ASCII, ending with the
Co-Authored-By line only; the logs at the reported hashes, each
ending all as expected, the scan RESULT: PASS at its second final
attempt with the superseded attempt named, every log pinning
30b61067; planning's own build at 8d05845 in a detached worktree:
513 and 727 crate tests passing on both targets with the feature
set off and all on, the new test passing by name on both, 894
workspace tests on both, clippy and doc 0 lines, the i686 release
binaries at the run log's hashes (the host's differ by build path,
as in every phase); planning's own reruns with its own binaries
under the same cap: tier a's 24 samples on both labels, 48 outputs
and 48 stdout lines byte-identical to the run directories'; tier
c's props and sheet on both labels and edit-formula on the host,
identical, the latter's sheet entry 5025024804 bytes with the
zip64 field; tier c's names on the host aborted (exit 134, "memory
allocation of 2198929408 bytes failed", 2 files of 955 bytes left
in `TMPDIR`) and on i686 panicked (exit 101, "capacity overflow",
nothing left), peaks 1078072 and 1075852 kB; planning's own
recount of the 192 rows of the 96 `.txt` records against the run
log's per-tier tables, 192 agreeing, tier a's 48 rows `lo ok ok`;
the run log's 193 `TMPDIR` counts, one non-zero (names, host);
planning's own privacy scan e2ff972..079db61, 3 commits, 1110
blobs, 0 findings; `ANTHROPIC_API_KEY` unset.

1. Finding 1, names on tier c. The host abort and the i686
   "capacity overflow" panic are one kill, P1 verification ruling
   7's class "holds in the tool": the tool's `Cursor<Vec<u8>>`
   (main.rs:4476) asks for 2198929408 bytes, which the 2048 MiB
   address space refuses on the host (abort, signal 6) and the
   `Vec`'s own size check refuses on i686 before any allocation
   (over `isize::MAX`, "capacity overflow"). Not a crate defect:
   the crate writes into the `Vec` the tool gives it, and no crate
   path holds the archive. "A panic on allocation" in "What is
   measured" covers both forms; the derivation's word `killed`
   stands as the log's. The two messages are in no log: P3's log
   prints each of the names run directory's two stderr files (its
   sha256 and bytes, then its lines, after a check with a control
   that no line holds the home directory), so the decision record
   quotes them; planning's own rerun reproduced both.
2. Finding 2, the leftovers. Counted, removed and logged as P1
   verification ruling 4 ordered. The reading (the crate's
   `TempFile`s, `Drop` not run on an abort) is the code's reading,
   stated as such in the decision record; planning's rerun bears
   it out (2 files, 955 bytes after the abort; 0 after the i686
   panic, where unwinding runs `Drop`). The runner's `leftover`
   column counts the run directory, so the record's `no` is right
   by its definition; the runner is not changed. Carried to part
   7: whether the runner counts `TMPDIR` too.
3. Finding 3, the diff binary's hash. Planning's sentence in "The
   runs" was wrong: the record script has never carried the
   runner's binary hashes (no part 4d record does); run.log line
   1047 prints the build's and the runner writes it into each run
   directory's `run.<label>.tsv`. "The runs" is corrected above;
   run.log line 1052's prose is a wrong claim in a committed log,
   recorded here, not re-run; the record script is unchanged; the
   decision record cites run.log line 1047. Carried to part 7: the
   record script's header gains the two hashes.
4. Finding 4, "32-bit". The archive is 1159829654 bytes, under
   2^32, so planning named the wrong figures; the sheet entry's
   uncompressed sizes (5025024826 in; 4985617758 to 5026073402
   out) are the ones over `u32`, and the log checked those whole
   with the planted truncation caught, and `in_bytes` and
   `out_bytes` whole besides. The substitution is accepted;
   "32-bit" is corrected above.
5. The link disagreement. The P1 survey listed link's calls
   without the `set_cell` at `max_row + 1` (main.rs:3034-3038),
   and the prediction carried the omission; the measured grid
   error is ruling 5's fix refusing a seventh sample, as it
   refuses edit, style, font, border, date and formula. A survey
   omission, not a defect; the decision record lists tier c's 14
   samples refused at the grid (7 at the recording call, 7 at the
   write, link among the write's) and insert-rows at the shift.
6. P1 verification ruling 6 bears out at five: the zip64 entry
   was rewritten by edit-formula, delete-rows, insert-columns,
   delete-columns and protect (largest peak 7864 kB); link was
   refused. The decision record says so.
7. The shared strings table. No sample calls
   `Package::shared_strings` (the survey said so; planning missed
   it in its P1 verification), so tier d measured the raw copy
   only and "What a finding leads to"'s third item has no figure.
   Ruling: P3 adds three run directories through the unchanged
   runner, `--tool-arg --shared-strings` and `--tool-arg --cells`
   on tier d and `--tool-arg --cells` on tier c, both labels, the
   same cap and timeout, with record pairs
   `m3p1-scale-d-shared-strings-`, `m3p1-scale-d-cells-` and
   `m3p1-scale-c-cells-`; the P3 log says they are the tool's
   read operations, not samples, and checks each output
   byte-identical as the tier t test asserts. The figure is the
   peak RSS in kB over the `items` count the tool prints, per
   label, and tier c's `--cells` is the streaming read at scale
   (a timeout is a finding, not a failure). No code changes.
8. The independent check's remaining items: "8 MB" read as 8 MiB
   (8004 kB), corrected in the report; "first attempt" supported
   by `RERUN_NOTE` unset in each log; the `--all-features` run a
   scope note; the three U+00D7 replaced and declared; one check
   of the two allowed. Tiers b to d print `-` in the `.txt`
   records' `lo` column and count under `lo not-run` in the `.md`,
   which "The runs" calls `lo not-run`. Accepted, no change.
9. P2 is DONE. P3 is released: its doc commit carries this file
   with the P2 rulings and this section and `ROADMAP.md`'s P2 row;
   P3 as "P3" says, with ruling 1's stderr print, ruling 7's three
   run directories, and the decision record's items from rulings 1
   to 7; the size log and record; part 4d's two samples rerun; the
   open record.

## Planning verification and rulings on P3 (2026-10-02)

Verified by planning from primaries: the doc commit c9a59be holds
this file at 4c46d46c and `ROADMAP.md` at 79769f0d, the hashes
planning sent; the phase commit 6578925 adds 49 lines to
`package.rs`, every one a `//!` line, and nothing else; the record
commit fe3c19e adds the six logs and eight record files; 3 of 3
messages ASCII, ending with the Co-Authored-By line only; the logs
at the reported hashes, each ending all as expected, the scan
RESULT: PASS, every log pinning 4c46d46c; the size record at
f9f2bd5f and the decision record at 0db5d43b read in full;
planning's own build at 6578925 in a detached worktree: 513 and
727 crate tests on both targets, 894 workspace tests on both,
clippy and doc 0 lines; planning's own size probes in the size
profile on i686: `probe-zip-xml` 533216 and the format probe
968568 section bytes at 6578925, 968568 at 8d05845 and 968440 at
c1f6602 in their own worktrees, so the part's growth 128, the
fix's 128, the catalogue's 0 and the budget 435352 (23400 under
458752) are reproduced to the byte; the i686 release binaries at
the size log's hashes (the host's differ by build path, as in
every phase); planning's own read runs with its own binaries
under the cap, each output byte-identical to its input and each
stdout line to the run directory's: tier d `--shared-strings`
266380 / 216572 kB, tier d `--cells` 266312 / 216572 kB, tier c
`--cells` 3904 / 3740 kB in 99.82 and 161.85 s (the runner's
peaks are a few MB above these; the classes are the same);
planning's own reruns of the comment and image samples over the
26 corpus workbooks its glob took, both labels, 104 outputs and
104 stdout lines byte-identical to P3's two run directories;
planning's own privacy scan 079db61..fe3c19e, 3 commits, 1129
blobs, 0 findings; `ANTHROPIC_API_KEY` unset.

1. The catalogue sentence "A whole pass over tier c's sheet runs
   within the runs' peaks below" is supported by P3's run log
   (tier c `--cells`, 7724 and 7432 kB, both under the rewrites'
   7860) and by planning's own run, though the block names P2's
   run log as its source and was committed before P3's run.
   Ruling: accepted as supported; no code commit in the close-out.
   Carried to part 2's first code commit: the block's first
   paragraph names P3's run log beside P2's.
2. The implementing session's two corrections are right: P2
   verification ruling 3's prose is on run.log line 1051, and
   ruling 6's largest rewrite peak is 7860 kB (edit-formula on the
   host; 7864 is remove's raw copy). Both rulings read as corrected
   here; the decision record notes both.
3. The phase commit's amend (message only, the same tree as
   3bd5095, before any log): recorded, no change, as P1
   verification ruling 8.
4. The independent check's remaining items: the report's wording
   on the comment comparison (outputs and lines, no diff tables)
   and on the rerun lines (four files hashed, the build directory
   kept and not hashed) corrected in the report; the two-sheet cell
   count and the "code's reading" labels are in the committed
   decision record. Accepted, no change.
5. The names i686 stderr holds the toolchain's source path
   (`/rustc/<hash>/library/alloc/src/raw_vec`), which the run
   log's check does not count since it counts private paths. Not
   private; the line's wording "holds no path" is loose. Accepted.
6. The size log's first attempt failed its own check (the record's
   rows matched by figures alone, which two rows share), fixed and
   rerun with the attempt named and hashed, the same record
   written by both: accepted; the record's figures are planning's
   own.
7. P3 is DONE. Decision 6 was asked and the maintainer deferred
   the Milestone 2 Excel check a sixth time (recorded under
   decision 6). The close-out is released: its doc commit carries
   this file with this section and decision 6's outcome, and
   `ROADMAP.md`'s P3 row and Milestone 2 heading; the rest as
   "Close-out push (after P3)" says.

## Planning verification and rulings on the close-out (2026-10-02)

Verified by planning from primaries: `origin/main` at the close-out
doc commit 21b89bb, 11 commits over fe584b3 (ab5b3aa and this
part's ten), the doc commit holding this file at 8eee79d0 and
`ROADMAP.md` at 4129f85b, the hashes planning sent; the CI run
37005285780 completed with success on 21b89bb, read by planning
through `gh`; the record commit 3cbd6fd, parent 21b89bb, exactly
the three logs, the one local commit ahead; the logs at the
reported hashes, each ending all as expected, the scan RESULT:
PASS; push.log's push mode showing the authorisation typed by the
maintainer in the implementing session; the two carried fixes of
`specs/0022` close-out ruling 8 present (scan.log line 861 prints
the `ANTHROPIC_API_KEY` check; ci-run's JSON output ends with a
newline); 12 of 12 messages ASCII, ending with the Co-Authored-By
line only; planning's own privacy scan fe584b3..3cbd6fd, 12
commits, 1142 blobs, 0 findings; `ANTHROPIC_API_KEY` unset.

1. The independent check's reservations (working-state facts no
   log prints: the copied scripts' equality with part 4d's, the
   generator's path, the pre-commit hash check, "one local commit
   ahead"; the report's change list omitting the 4c to 4d label
   wording; the headers citing `specs/0022` close-out rulings 5
   and 6 beside 8, which carries them; the check's own
   `sha256 -c` run from the wrong directory then the right one):
   cosmetic or allowed by `specs/0001`; accepted, no change.
2. Milestone 3 part 1 is DONE and PUSHED. The record commit
   3cbd6fd goes with the next push; the next part's push script
   names it by `^M3 part 1 close-out record`, its base this
   close-out's doc commit 21b89bb by its subject.
3. Carried into the next parts, from the decision record and the
   rulings above: part 2's first code commit names P3's run log
   beside P2's in the catalogue block (P3 ruling 1); part 5 the
   shared strings table's hold; part 6 the whole VML and drawing
   reads; part 7 the runner's `leftover` column and the record
   script's hashes. The Milestone 2 Excel check is due before
   part 2's close-out push (decision 6).
4. These rulings are written into the working spec after the
   close-out doc commit; they go in part 2's first doc commit.

## Out of scope

- Any new capability, sample or API; a streaming shared-strings
  resolver (part 5 decides); a streaming comments or drawing read
  (part 6 decides).
- Speed as a target: the throughput is recorded, not judged;
  no optimisation in this part.
- A workbook over 2^32 bytes as an archive (the central directory
  offsets in zip64): the archive layer's tests cover the records;
  tier C covers one entry over 4 GiB, which is the case the
  32-bit address space makes real.
- Multiple large sheets, charts, pivots at scale, macro files:
  the corpus P5 and P6 files, when supplied, go through the
  ordinary corpus runs.
- Committing any generated workbook; the tiny tier `t` lives only
  in the test's temporary directory.
