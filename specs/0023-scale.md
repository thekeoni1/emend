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
tags `scale,tier-<letter>`, original_name the tier's file name,
note empty) so the runner takes the directory as a corpus.

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

| Tier | Rows | Columns | Strings | Sheet XML, about | Purpose |
| --- | --- | --- | --- | --- | --- |
| A | 50000 | 24 | vocabulary | 60 MB | LibreOffice opens it; the baseline every sample is compared against |
| B | 800000 | 24 | vocabulary | 1 GiB | past any sensible buffer, under the cap |
| C | 1048576 | 48 | vocabulary | 4.5 GiB | one entry over 4 GiB: zip64 sizes, over the 32-bit address space |
| D | 800000 | 24 | every string unique | 1 GiB, shared strings about 150 MB | the shared strings table loaded whole, measured |

Tier C's row count is Excel's maximum. The archive sizes are what
deflate makes of them, recorded, not designed. Each tier's sha256
and byte count go in the P1 record; the runs cite them.

### The runs

`scripts/passthrough-run.sh` unchanged, `--corpus` the scale
directory, `--memory 2048` on both labels (so the host and the
32-bit target are measured alike), `--timeout 3600`, one run per
sample per label per tier: 25 samples, two labels, four tiers,
200 runs. `--libreoffice` on tier A only (decision 2); tiers B to
D record `lo not-run`. The diff runs with the host `xlsx-diff` for
both labels on tier C (decision 4), since the tool reads an entry
whole and the entry is larger than a 32-bit address space; the
record names the binary's sha256 as the runner prints it. The
run directory is `runs/<utc-date>-<shortsha>-scale-<tier>`, one
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
the runner reads is a byte count; the tool's `in_bytes` and
`out_bytes` for tier C exceed `u32`, and the run log checks that
they are printed whole (a control: a planted 32-bit truncation of
the figure is caught).

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
   here. Recorded, not asked.

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
