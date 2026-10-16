# 0037: Milestone 5 part 6, measurement and close

Status: APPROVED by the maintainer 2026-10-16, decisions 1 to 3
each taken as A, the recommended option, typed in the planning
session. Sent to the implementing session the same day.

Context: `CLAUDE.md` ("What emend is": the comparison point is
openpyxl, with the four exclusions; "The fidelity harness is the
acceptance instrument": a README sentence about fidelity, speed or
size cites a harness run in the records; "Dependencies and
licences"; "32-bit and sizes"; "Elevation and system changes"),
`ROADMAP.md` ("Milestone 5", the outline approved 2026-10-11, part
6: the openpyxl comparison, the size record, the engineering close,
the README's comparison sentence citing the record; "Unscoped"),
`specs/0001-bootstrap.md` ("Rules for every phase"),
`specs/0006-passthrough.md` ("The runner"),
`specs/0015-open-checks.md` ("The open-check library"),
`specs/0023-scale.md` ("The tiers", "The runs", "What is measured
and how it is read"), `specs/0029-engineering-close.md` ("The
README", decision 4 A; "MSRV"), `specs/0030-carries-and-api-
changes.md` (decision 2 A, the per-file attribution),
`specs/0033-read-breadth-survey.md` (P3 verification ruling 1, the
`read` feature's finding), `specs/0034-charts-read.md` (ruling AC),
`specs/0035-charts-write-first-set.md` (ruling O), `specs/0036-
charts-write-second-set.md` ("Planning verification of P1", the
by-symbol attribution; rulings Q, R, S; close-out ruling W). No
external text is cited; openpyxl's behaviour is measured here, not
quoted.

## Goal

Milestone 5 ends with the comparison the maintainer asked for on
2026-10-11 made as a measurement: the same requests put to openpyxl
and to emend over the private corpus and over the scale tiers, each
tool's output compared with its input by the harness, each run's
wall time and peak memory recorded beside the other's, and
LibreOffice asked to open every output. The record that results is
what the README's comparison sentences cite. The part also closes
the milestone as part 7 closed Milestone 3: the one candidate the
parts carried (the self-closing drawing root, decision 3), the size
record that stands as Milestone 5's closing figure with the
attributions the parts promised, the API review rerun, and the
README's feature list and measured sentences brought to Milestone
5's records. No feature is added; the core does not move.

## Facts planning checked (2026-10-16)

- openpyxl is installed nowhere this project can run it: not on
  the host (`python3 -I -B -c 'import openpyxl'` fails with
  `ModuleNotFoundError`) and not in the pinned LibreOffice image
  `localhost/emend-lo:25.2.3-2-deb13u6`, which holds no `python3`
  at all (run with `--entrypoint sh`). ROADMAP's "in the pinned
  container" therefore means a second pinned image, built as the
  first was.
- Debian trixie, the suite the LibreOffice image's base digest
  pins, offers `python3-openpyxl` at `3.1.5+dfsg-2`, depending on
  `python3-et-xmlfile`, `python3-lxml (>= 5.0.1~)` and `python3`,
  recommending `python3-pil` (needed for an image to be added).
  Read with `apt-get update` and `apt-cache show` in a throwaway
  container from the same base digest.
- The host has 21 GiB of memory, 16 processors and over 700 GiB
  free on the corpus disk. The scale tiers are on disk at the ids
  `records/m3p1-gen-2026-10-01-a509ad3.md` names: a (11025994
  bytes), b (176372258), c (1159829654), d (201769244).
- `scripts/passthrough-run.sh` caps the child with `ulimit -v`
  and measures its peak RSS and wall time; the open check
  (`scripts/lo-check.sh`) runs the image with `--network=none
  --userns=keep-id`, the input mounted read-only. Both forms carry
  to the openpyxl runner.
- The emend operations the comparison needs exist as
  `tools/emend-copy` samples with records in `records/`; what
  they do is fixed by their specs and is not changed here.
- `ROADMAP.md` "Unscoped" still lists charts as a later
  milestone's crate; planning edits that bullet in P4's doc
  commit (charts are Milestone 5's feature).

## The openpyxl image

`tools/openpyxl-image/Containerfile`, committed: `FROM` the
LibreOffice image's base digest
(`docker.io/library/debian@sha256:abc9cb88...`, trixie-slim,
amd64), `apt-get install -y --no-install-recommends
python3-openpyxl=3.1.5+dfsg-2 python3-pil`, the installed `python3*`
packages listed to `/py-packages.txt` by `dpkg-query`, the apt
lists removed, `ENV HOME=/tmp`, `WORKDIR /work`, no entrypoint (the
runner names the command). The tag is
`localhost/emend-openpyxl:3.1.5-dfsg-2-deb13`. The build is a
container operation under `CLAUDE.md` "Elevation and system
changes": the implementing session prints what it will run, runs
it rootless as the LibreOffice image was built (no elevation; if
podman asks for any, stop and ask), and records in
`records/openpyxl-image-<utc-date>.md`: the base digest, the
package pins, every installed package with its version (from
`/py-packages.txt`), `python3 -c 'import openpyxl; print(
openpyxl.__version__)'`, the image id and byte count, the
Containerfile's sha256, podman's version. If the suite no longer
offers `3.1.5+dfsg-2`, the build is pinned to snapshot.debian.org
at the date the record names rather than moved to a newer version;
that is a mid-phase ruling, not a silent change. Decision 1.

The image never sees the network at run time (`--network=none`),
never sees the repository, and sees one corpus file at a time,
read-only.

## The operations

Eleven requests, each one of emend's existing samples and the
openpyxl call that makes the same request. The two tools are never
compared with each other's output: each output is compared with
the input by `xlsx-diff`, and what each tool rewrote, dropped or
added is what the record reports. W is the first worksheet of kind
`Worksheet` (openpyxl: `wb.worksheets[0]`), M its last used row
(openpyxl: `ws.max_row`); every addition goes below M as the emend
samples place theirs. The script's docstring names every constant;
this spec fixes the kind of each request, not its cell.

| op | emend | openpyxl (`scripts/openpyxl-ops.py <op> <in> <out>`) |
| --- | --- | --- |
| roundtrip | `emend-copy <in> <out>` (the passthrough, no edit) | `load_workbook(in)`; `save(out)` |
| read | `--read-sample` | load; every cell of every worksheet visited by `iter_rows`, counted; `save(out)` |
| edit | `--edit-sample` | load; a number, a text and a boolean set in row M+1, columns A to C; save |
| format | `--format-sample` | load; a number in (M+1, A) with `number_format` set; save |
| insert-rows | `--insert-rows-sample` | load; `insert_rows(1)`; save |
| names | `--names-sample` | load; one defined name over W's A1; save |
| table | `--table-sample` | load; two header cells and one data row written at M+2, a `Table` over them; save |
| comment | `--comment-sample` | load; a `Comment` on (M+1, A); save |
| image | `--image-sample` | load; a 2 by 2 PNG the script holds as bytes added at (M+2, A); save |
| add-chart | `--add-chart-sample` | load; a `BarChart` with one series over B2:B(M+1), categories A2:A(M+1), added at (M+2, A); save |
| copy | `--copy-sample` | load; `copy_worksheet(W)`; save |

The `read` op saves because openpyxl has no other output to hand
the harness; its figures are reported as measured and the record
says so. The emend side runs with the release `emend-copy` at the
phase commit, through `scripts/passthrough-run.sh` as every sample
record was made, on both labels.

`scripts/openpyxl-ops.py` is committed, fictional, holds no
corpus content, runs under `python3 -I -B`, refuses an op it does
not know with exit 2 and one line, prints at exit one line
`openpyxl=<version>\tpeak_rss_kb=<ru_maxrss>` from
`resource.getrusage(RUSAGE_SELF)`, maps an exception to one line
holding the exception's class name only (never its message, which
may quote content) and exit 1, and maps `MemoryError` to exit 137
so the runner classes it as the cap did. Nothing it prints names
a path or a cell value.

## The comparison runner

`scripts/compare-run.sh`, modelled on `scripts/passthrough-run.sh`
and sharing `scripts/lo-check.sh`: `--op <op> --image <image>
--xlsx-diff <path> [--corpus <dir>] [--out <dir>] [--timeout
<secs>] [--memory <MiB>] [--ids <file>] [--libreoffice <image>]
[--lo-timeout <secs>]`. For every manifest row, or every id in
`--ids`, in manifest order: the file is mounted read-only at `/in`
and the output directory at `/out`, the script at `/ops.py`
read-only, and the container runs `sh -c 'ulimit -v <MiB*1024> &&
exec python3 -I -B /ops.py <op> /in/<id>.xlsx /out/<id>.xlsx'`
under `timeout --signal=KILL`, `--network=none --userns=keep-id`,
stdout and stderr to files in the run directory (openpyxl's own
warnings go there and are never printed; the log prints their byte
counts). Wall time is the runner's; peak RSS is the script's last
line; the outcome classes are the runner's (`ok`, `error`,
`timeout`, `cap` for exit 137, `no-output`). Then `xlsx-diff` on
the host between input and output, the same columns as the
runner's `summary.<label>.tsv` and `entries.<label>.tsv`, the
label `openpyxl`; `run.openpyxl.tsv` records the commit, the image
id, the openpyxl version, the settings. `--libreoffice` opens
every output as the runner does (`lo` column; `lo_in` is the emend
run's). It prints one line per file (id, outcome, exit, wall
seconds, peak RSS kB) and a count line, never a name or a path
under the corpus directory. The run directory is
`runs/<utc-date>-<shortsha>-openpyxl-<op>` beside the emend run
`runs/<utc-date>-<shortsha>-<op>`.

`scripts/passthrough-record.py` accepts the label `openpyxl`
(today it takes `host` and `i686`), so each openpyxl run has the
same record pair as every emend run: `records/m5p6-openpyxl-<op>-
<utc-date>-<shortsha>.md` and `.txt`.

`scripts/compare-record.py`, committed, writes
`records/compare-m5p6-<utc-date>-<shortsha>.md` from the run
directories' summary and entries files (never from a log): per
op, one row per tool and label with files run, outcomes, the
entries identical, changed, dropped and added (totals over the
27), LibreOffice verdicts, wall time (median and sum, seconds) and
peak RSS (median and maximum, kB); then the scale table; then the
openpyxl image record's hash. Counts, times and bytes only; no
entry name is listed that is not an entry name (entry names are
fine, as every record says).

## What is measured and how it is read

For every op and every corpus workbook, each tool: outcome, wall
time, peak RSS, the diff's entry columns, the open verdict. The
run log derives per op:

- fidelity: how many of the 27 outputs each tool wrote, how many
  entries each left byte-identical, how many it dropped, how many
  LibreOffice opened (openpyxl's drops are the point of the
  measurement; nothing is "fixed" on its side);
- speed: the median wall time per tool per label, and the ratio
  openpyxl over emend on the host (one figure per op);
- memory: the median and the maximum peak RSS per tool per label.

A workbook openpyxl cannot load (`error`) is an outcome, not a
failure of the run; the record names the exception class and the
count. Nothing is rerun to change a figure.

## The scale tiers

The scale manifest and the four tiers of `specs/0023` as they
stand on disk, by id. Decision 2 sets the reach; option A: for
every op, tiers a and b through the openpyxl runner with `--memory
16384` and `--timeout 3600`; tiers c and d for `roundtrip` only
under the same cap; emend's side rerun at the phase commit on the
host for the same ops and tiers (tiers c and d on the host with the
host `xlsx-diff`, as `specs/0023` decision 4 ran them), both labels
only where `specs/0023` ran both. A tier the cap or the timeout
ends is a finding to record (`cap`, `timeout`), never a reason to
raise the cap. One run directory per tier and tool, as the scale
runs were laid out: `runs/<utc-date>-<shortsha>-openpyxl-scale-
<tier>` with every op's rows in it.

## The engineering close

The carried items, each with its source; nothing here is new.

1. The self-closing `<xdr:wsDr/>` drawing root refused on an
   append (`specs/0036` ruling Q): decision 3. Option A, the
   shared drawing writer accepts the self-closing root as an empty
   drawing and appends to it, one test under `images` and one
   under `charts` on a generated fixture, the message that refused
   it gone from the catalogue; the five part 5 samples and part 4's
   three rerun byte-identical (no corpus producer writes the
   form). Option B, the form named in the catalogue's Limits as a
   refusal, no code.
2. The size record cites the by-symbol attribution of the chart
   writer's cost (`specs/0036` "Planning verification of P1": the
   emission's literals about half, the regated drawing writer a
   quarter) and the `images` feature's attributed shrink
   (`specs/0036` ruling S: 784e3c0, measured at 1bb8e02 and
   4836d14); it names the milestone's two estimate misses
   (`specs/0034` ruling AC, `specs/0035` ruling O) and the `read`
   feature's finding (`specs/0033` P3 verification ruling 1,
   112600) as history, with the figures the records hold.
3. The API review rerun (`specs/0029` decision 3's form, as part 5
   P4 ran it): `records/api-review-m5p6-<utc-date>-<shortsha>.md`,
   every row judged; the rows that change against part 5's are
   decision 3's, if A, and none otherwise.
4. The MSRV policy (`specs/0029` decision 1): the four pins agree
   at the close; the hygiene step proves it as it does every gate.
   No move.
5. The Milestone 2 Excel check: gates no push in this milestone
   (ROADMAP's Milestone 5 heading); untouched, its stage on disk.
6. The probe `probe-emend-charts-write-xml`'s per-kind form
   (`specs/0036` ruling R) is the probe's documented form; the
   size record describes it as part 5's did. No action.

## The README

`README.md` as `specs/0029` decision 4 A shaped it, brought to
Milestone 5: the feature list gains `charts` (its spec and cost
from the size record P3 writes) and the `read` line cites that
record's figure; the fidelity sentence names Milestone 5's samples
(the five of part 5 P4's reruns, citing their records) beside
Milestone 3's; the size sentence cites P3's record; the speed
sentence keeps the scale record and gains the tier rerun's
figures; and a comparison paragraph states, per op or for the set,
what each tool left byte-identical, dropped and opened, the median
wall times and peak RSS, citing `records/compare-m5p6-...md` only.
Every sentence holding a digit cites a path under `records/` that
exists at HEAD; P4's run log checks it as `specs/0029` P4 did
(the command printed, a control on a planted sentence; no U+2014,
no absolute path, no corpus content). The sentences state
measurements; they do not say "faster" or "better".

## Size

The budget figure after part 5 is 436992 section bytes against
the ceiling 458752 (`records/size-m5p5-2026-10-15-a8bfe71.md`):
21760 of headroom. This part's only code in the crate is decision
3's (if A), under the shared drawing writer that `images` and
`charts` link: expected 0 to 600 on each of those two probes,
accepted up to 1500; every other probe expected 0 and accepted up
to 2000 as the Milestone 5 records accept; the core expected 0 and
accepted up to 100. The size record `records/size-m5p6-<utc-date>-
<shortsha>.md` is written in P3 with item 2's citations and stands
as Milestone 5's closing figure.

## 32-bit

The openpyxl image is amd64 and runs on the host only; the record
says so. emend's corpus reruns run on both labels as every sample
record did; the scale reruns on the labels `specs/0023` used per
tier. Nothing in decision 3's code touches a size or an offset.

## Harness runs

P2: eleven ops, 27 workbooks, openpyxl on the host and emend on
both labels, every output opened; the scale tiers per decision 2.
P3: decision 3's reruns (part 5's five samples, part 4's three)
byte-identical to their records by sha256, if A. P4's run log holds
the README check. No other corpus run.

## Phases

Each phase as `specs/0001` "Rules for every phase": a doc commit
first, the code commits, one record commit, the scan last, at most
two independent checks, the report under `<phase>`, then stop.
Target root `<home>/.cache/emend-build/m5p6-pN/target`.

### P1: the image and the scripts

The Containerfile, the build (the plan printed first), the image
record; `scripts/openpyxl-ops.py` with its docstring and a test
that runs every op on a committed fictional fixture inside the
image (`scripts/openpyxl-ops-test.sh`, run and printed by the
phase's test log as `scripts/scale-gen-test.py` is run, never by
the gate, so CI is unchanged); `scripts/compare-run.sh` and the
record script's `openpyxl` label with their tests; the negative
log: an unknown op refused, a planted exception's class printed
and no message, a planted `MemoryError` as exit 137, a cap of 64
MiB ending a run as `cap`, a timeout of 1 s as `timeout`, a run
printing no path (the control plants one in a stderr file and
shows it stays there). No corpus run.

### P2: the runs

The 22 corpus runs and the scale runs of decision 2 at the phase
commit; the 22 record pairs and the scale pairs; the comparison
record by `scripts/compare-record.py` with its test; the run log's
derivations; the open record. The run log asserts every row of the
comparison record against the summaries it was built from.

### P3: the close

Decision 3's code and tests (if A) and the catalogue; the size
record (item 2); the API record (item 3); the reruns of "Harness
runs" P3; the CHANGELOG bullet for decision 3 under Fixed (if A),
nothing otherwise.

### P4: the README

The README's edits; the run log's README check; `CHANGELOG.md`
unchanged beyond P3; planning's ROADMAP edit (the Unscoped charts
bullet, the part 6 rows) in this phase's doc commit.

## Close-out push (after P4)

As part 5's (`specs/0036` "Close-out push (after P4)" and its
close-out rulings U to W): the generator transforms part 5's
scripts (`gen.py` 933219df and the three it printed, carried
cmp-equal), its numbered changes continuing from 43; the doc
commit holds this spec and `ROADMAP.md`; the base is part 5's
close-out doc commit c4a01cf, named by its subject ("M5 part 5
close-out: commit planning docs (the P4 verification and
rulings)"); `PREV_RECORD_PATTERN` is `^M5 part 5 close-out record`
(543c418), in the range exactly once; the close-out doc commit's
subject is "M5 part 6 close-out: commit planning docs (the P4
verification and rulings)"; `CARGO_TARGET_DIR` printed and checked
as `<home>/.cache/emend-build/m5p6-close/target`; no Excel-record
check, the stray-record count 0 kept; the scan's denylist form
whole-word; the record commit "M5 part 6 close-out record: the
push, the CI run and the scan" holding the three logs only; pre
mode on a clean tree; one independent check before the report.
The push carries 543c418 and this part's commits, on planning's
word and the maintainer's authorisation typed in the implementing
session. `ROADMAP.md`'s Milestone 5 heading then reads DONE and
PUSHED, and the close-out's verification says what the next
milestone carries.

## Decisions for the maintainer

Decided by the maintainer 2026-10-16: 1, 2 and 3 as recommended (A
in each), typed in the planning session.

1. The openpyxl source ("The openpyxl image"): A (recommended), a
   second pinned image from the LibreOffice image's base digest
   with Debian trixie's `python3-openpyxl=3.1.5+dfsg-2` and
   `python3-pil`, built rootless, no network at run time, the pins
   in a record; B, openpyxl from PyPI in a host virtual
   environment, pinned to 3.1.5 by hash, the network at install
   time and the interpreter's version unpinned by any record.
2. The scale reach ("The scale tiers"): A (recommended), every op
   on tiers a and b and `roundtrip` on tiers c and d, under
   `--memory 16384` and `--timeout 3600`, with emend rerun beside
   it; B, every op on every tier (up to 44 openpyxl runs of up to
   an hour each, most of them expected to end at the cap).
3. The self-closing drawing root ("The engineering close", item
   1): A (recommended), accepted as an empty drawing by the shared
   writer, one test per feature, 0 to 600 section bytes on each of
   two probes; B, named in the Limits as a refusal, no code.

## Out of scope

Comparing one tool's output with the other's; any change to what
emend's samples do; openpyxl's `read_only` and `write_only` modes
(a request put the ordinary way is the comparison asked for); a
rendering comparison; openpyxl on the 32-bit target; the publish
step (Milestone 4 decision 1, held); the Milestone 2 Excel check's
run; any crate change beyond decision 3; a new milestone's outline
(the close-out's verification names candidates only).
