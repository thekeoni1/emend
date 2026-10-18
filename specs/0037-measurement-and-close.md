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

## Planning rulings during P1 (2026-10-16)

The doc commit 2e36426 verified by planning: the three documents
at their full hashes (931b7e5c, 8f85b6ca, a86114da), one line
each, nothing else, ASCII, the tree clean. The image build as
reported (id 894b239f, 211283030 bytes, openpyxl 3.1.5,
`python3-openpyxl 3.1.5+dfsg-2`, `python3-pil 11.1.0-5+deb13u4`)
is accepted on the report pending the image record and planning's
own `podman image inspect`.

A. FINDING, the runner's timeout: `timeout --signal=KILL` ends the
   podman client and leaves the container running. Ruled as the
   implementing session recommended: every container is named
   (`emend-openpyxl-<runner pid>-<id>`); after exit 124 the runner
   runs `podman rm -f` on that name with its exit written to the
   run directory; at the end of every run the runner counts the
   image's containers and ends with a failure line if any remain;
   the negative log's 1 s timeout shows the container gone (a
   `podman ps` count before and after, with a control that counts
   a container it starts itself). `scripts/lo-check.sh` has the
   same shape; it is not touched in P1 and is carried to P3 as
   engineering-close item 7: the same named form and removal,
   shown by P3's negative log on a planted slow conversion.
B. Exit classes: 0 with an output file `ok`; 1 `error`; 124
   `timeout`; 137 `cap`; 0 without an output file `no-output`;
   every other exit (2, 125 to 127, any) `error` with the exit in
   the exit column. The unknown-op exit 2 is reachable only from
   the negative log.
C. `lo_in` in the openpyxl summary is `-`; the comparison record
   takes the input's verdict from the emend run's `lo_in` and
   names the run directory it read it from. No value is copied
   from one run directory into another's summary.
D. The output keeps the input's extension (`/in/<id>.<ext>`,
   `/out/<id>.<ext>`), as the passthrough runner does; the
   spec's table reads as a shape, corrected here, not rewritten.
E. The record script's `openpyxl` label as proposed: its own
   outcome set (ruling B), its own `run.openpyxl.tsv` header
   (commit, started, ended, image_id, openpyxl, timeout_s,
   lo_timeout_s, memory_mib, ops_sha256, xlsx_diff_sha256, files,
   op), the summary and entries headers exactly the runner's,
   `leftover` yes when the output directory holds anything beside
   the output, the Binaries line naming the image id, the ops
   script's sha256 and `xlsx-diff`'s, a single-label record
   written with `--allow-single`.
F. The fixture: `tests/fixtures/openpyxl-ops.xlsx` generated by
   `scripts/scale-gen.py` tier t, documented in
   `tests/fixtures/README.md` with the tool, the command and the
   sha256; `scripts/openpyxl-ops-test.sh` regenerates it into a
   temporary directory and asserts the sha256 before running the
   ops, so the committed bytes are shown to be the generator's.
   No fixture written by openpyxl. The 64 MiB cap case of the
   negative log uses a workbook the log generates with the same
   generator at a row count the log names, large enough that the
   `roundtrip` op ends as `cap`; the log prints the count and the
   outcome, never the content.

G. FINDING, two chained `[ ... ] && [ ... ]` shapes inside `yn` in
   `scripts/openpyxl-ops-test.sh` (the generator's reported sha256
   and the unknown op's empty stderr never asserted), found by the
   scan's pre-check at 71aaaee after the gate, test and negative
   logs had passed (their shape greps cover their own sources
   only); and the README's `<dir>` placeholder printed by the test
   log without a root line. Ruled as the implementing session
   recommended, on part 5 P3's precedent (`specs/0036` ruling P)
   and `specs/0001`'s rule that local, unreported commits may be
   rewritten before the report: the ops commit rewritten with the
   same message, the two checks as named helpers; the runner
   commit cherry-picked onto it with the same message; one root
   line declaring `<dir>`; the three logs and the image record
   moved to `<phase>/superseded/logged-1`, the failing scan log
   kept beside them unedited, and the gate, test and negative
   logs rerun at the new phase commit, then the scan. The image
   is not rebuilt: the rerun test log must show the same image id
   894b239f, and the regenerated image record differs from the
   superseded one in its commit and date lines only (the report
   prints the diff's line count). The report discloses the
   rewrite, the finding and the superseded directory with its
   hashes. No separate fix commit.

## Planning verification of P1 (2026-10-16)

Verified from primaries: the four logs at the record commit 1cdd3ed
(gate 57a828bd, test b1d36689, negative 047a453d, scan ece13e00
RESULT PASS, 0 hits, 0 structural failures, PLANNING_SPEC 7a2bffe1
the working copy with rulings A to G; each of the three "all as
expected: yes"); the record commit's 6 files and
independent-check-1.md (9bbe5b9d); the image record (a28d721d),
its superseded first form (d476e532) and the superseded gate, test
and negative logs (20832166, f255e2ec, d93ff08a) and failing scan
(7f908a60) under `<phase>/superseded/logged-1`; the raw check
answer (3c886362); the report (10468b31); `git diff 71aaaee
1e29c4e` one file, 4 insertions, 2 deletions; the four messages
ASCII with the Co-Authored-By line only; planning's own privacy
scan of 543c418..1cdd3ed: 4 commits, 1805 blobs, 0 findings; the
image by planning's own `podman image inspect`: id 894b239f,
211283030 bytes, created 2026-10-16 15:06:42 UTC, no container of
it present; the ops script, the test script, the Containerfile,
the fixture README, the runner and the record script read in full
at 1e29c4e against "The openpyxl image", "The operations", "The
comparison runner" and rulings A to F; the fixture's sha256
12ec7ca0 as the README names; planning's own run of
`scripts/openpyxl-ops-test.sh` on the image: 76 ok lines, 0 FAIL,
PASS; planning's own run of `scripts/compare-run.sh --op
roundtrip` over the 27 corpus workbooks with the phase's
`xlsx-diff` and `--memory 4096`: 27 ok, 0 containers remaining,
wall median 0.81 s and maximum 24.94 s, peak RSS median 38300 kB
and maximum 674876 kB, entries over the 27: 28 identical, 309
changed, 533 dropped, 242 added, 0 recompressed (counts only; the
run directory under planning's scratch). The gate's crate figures
515/617/551/657/720/1096 are P4's, the charts probe 1075632 moved
0.

Rulings:

H. The runner writes openpyxl's stderr to the run directory and
   prints no byte count; "the log prints their byte counts" in
   "The comparison runner" is the P2 run log's duty, as the
   implementing session read it. The runner is unchanged; P2's
   run log prints, per run directory, the count of non-empty
   stderr files and their total bytes.
I. A `die` inside the runner's loop ends the run before the
   container count (ruling A says "at the end of every run").
   Carried to P2's first code commit: an EXIT trap in
   `scripts/compare-run.sh` that counts the image's containers and
   prints the line whatever ended the script, with the loop's
   count line kept; P2's negative log plants a mid-loop `die` (an
   unwritable summary file) and shows the count line printed.
J. Check 1's notes: the record title "openpyxl comparison run"
   (its note (c)) is accepted as the label's title; its
   disagreements 1 to 3 were met in the report's wording (the
   shapes attributed to the test log that prints the script, the
   two statements and the record subject labelled); its two
   omitted points are rulings H and I. The path scrub of the
   check's answer (one absolute home path replaced by `<phase>/`,
   the raw answer kept at 3c886362, the first final scan's FAIL
   kept as superseded) is accepted as disclosed; from P2 on, the
   check's brief tells it to name its inputs by `<phase>/` and the
   answer is scanned before the report quotes it.
K. The runner's wall time wraps `podman run`, so it includes the
   container's start. P2's run log times an empty run of the image
   once per run directory (`python3 -I -B -c pass`, the same
   mounts and flags, three runs, median printed), and the
   comparison record and the README paragraph say wall time
   includes the container start and name that figure beside it.
   emend's wall time is the process's own, as the runner measures
   it; the record says that too.

P1 is DONE. P2's doc commit carries this spec (rulings A to K and
this section) and ROADMAP's P1 row.

## Planning rulings during P2 (2026-10-16)

The doc commit 09d82ac verified by planning: specs/0037 (160c2984)
and ROADMAP.md (d1f095bc) unedited, nothing else, the tree clean.
The first code commit 8593ba0 (ruling I's EXIT trap) read: the
count line printed whatever ends the script, exit 2 if any
remain, a non-zero status kept; the argument refusals before it.

L. The scale layout: one run directory per tier per op per tool,
   `runs/<utc-date>-<shortsha>-openpyxl-scale-<tier>-<op>` and
   `runs/<utc-date>-<shortsha>-scale-<tier>-<op>` under the scale
   directory's `runs/`, as `specs/0023`'s runs were laid out per
   sample per tier. "The scale tiers" read as a shape; neither
   runner changes. The record pairs follow: `records/m5p6-
   openpyxl-scale-<tier>-<op>-...` and `records/m5p6-scale-<tier>-
   <op>-...`.
M. emend on the scale tiers exactly as `specs/0023` ran them: both
   labels on every tier it reaches, `--memory 2048`, `--timeout
   3600`, the host `xlsx-diff` for both labels on tier c; tier d
   both labels. The open check on tier a only for both tools
   (`specs/0023` decision 2); tiers b to d record `lo not-run`.
N. openpyxl's settings on the corpus runs: `--memory 4096
   --timeout 600`, so the corpus measures openpyxl and not the
   cap (planning's own roundtrip run peaked at 674876 kB under
   4096), named in `run.openpyxl.tsv` and in every record. emend
   keeps the passthrough runner's defaults on the corpus, as every
   sample record did. A cap or timeout under these settings is
   still an outcome to record, not a reason to raise them.
O. emend's op mapping as proposed: `roundtrip` is `emend-copy
   <in> <out>` with no tool argument (the passthrough), not
   `--roundtrip-sample`; the other ten are `--<op>-sample` as the
   table names them.
P. Ruling K's empty-image timing is per openpyxl run directory,
   corpus and scale alike: three empty runs, the median printed
   in the run log and carried into that directory's record pair
   and the comparison record. The planned order (the 22 corpus
   runs, the records, the keep and the open record, then the
   scale runs last, tier a to d, openpyxl then emend per op, then
   the comparison record) is accepted.

## Planning rulings during P2, second set (2026-10-18)

The logged run (attempt 3, phase commit 5e52758) ended 2026-10-16
23:11:51Z with exit 1 and "run log checks all as expected: no".
Planning read the log, the script, the script's standard error and
the openpyxl image run directory. Every run and every record was
written, the outer stderr is empty, and the three failing checks
are defects in the run script's checks, not in the measurements:

- `keep_pairs_ok` is called (script line 3582) and never defined;
  the P5 scripts defined it and the P2 generator dropped the
  definition. The keep is whole by the log's own counts (1435
  pairs, 2870 files, only whole pairs, every row matching the
  committed open record).
- The script's 105-byte standard error is that one "command not
  found" line.
- "openpyxl: the xml families are the changed entries' families"
  is no for the openpyxl image op alone (yes in its other 104
  places): openpyxl re-encodes the images, so the media family's
  changed entries are four PNGs with no XML comparison row.

The dry run exercised neither check: it ran with `RUN_SKIP_KEEP=yes`
over a two-workbook subset without media.

Q. The fix as proposed. Regenerate the run script with
   `keep_pairs_ok` restored and its planted controls as in P5.
   Narrow the family check to "the xml families are the changed
   entries' xml families": a changed family counts when at least
   one of its changed entries has an XML comparison row in the
   run's diff table, so a family whose changed entries are all
   binary (media) is not expected in the xml table. Two planted
   controls: an entries table with a changed media entry and no
   xml row reads yes; an xml table naming a family with no changed
   entry reads no. The generator also prints, for every script it
   writes, that every helper a `yn` or `ynnot` call names is
   defined in that script (the shape `bash -n` cannot see), with a
   planted undefined name as its control.
R. No full rerun. The 114 run directories, the keep and the records
   are the measurement, written by the phase commit's binaries, and
   nothing in them is in question; a second 12.5 h run would
   measure nothing new and would be the re-run the rules forbid.
   What runs again with the fix named is the checks: a check-only
   log, `logs/check.log` from a generated `m5p6-p2-check.sh`, that
   (1) names the failing run log by sha256 and quotes its end line,
   (2) recomputes the sha256 of every installed record and of the
   host keep's listing and matches each against the line the run
   log printed, counting matches against the record count, (3) runs
   the restored keep check and the corrected 12-check block over
   every record pair, corpus and scale, with ruling Q's controls,
   (4) prints the tree state as the run log does, and (5) ends
   "check log checks all as expected: yes" with an empty standard
   error. The failing run log stays as `logs/run.log` and is
   disclosed in the report, the logs README and the record commit
   message; the independent checks receive both logs. The run
   script that produced the run log is kept beside the regenerated
   one under `<phase>/superseded/run-script-3` and both are named
   by sha256 in the README; the regenerated script is the one the
   close-out's generator starts from.

## Planning verification of P2 (2026-10-18)

Verified from primaries, not from the report: the five local
commits on 1cdd3ed (09d82ac, 8593ba0, 89ef6eb, 5e52758, the record
d5e73cb of 151 files, 143 records and 8 logs, its message
disclosing the failed run log); the tree clean but for this
working spec; the eight committed logs and the comparison record
rehashed to the sha256 the report and the logs README cite
(gate da7b322f, test 1c3bdc6b, negative b74a1500, run 66e39a92,
check 41f34b40, README 15d42a1a, the check's answer 837a0a2f,
scan f093875b, the record 2faf8ff1); gate, test, negative and
check ending "yes", run ending "no" with exactly the three ruled
lines (plus the 12-check block that reports one of them), the
scan "total hits: 0", "structural checks failed: 0", "RESULT:
PASS" under PLANNING_SPEC ce89ba9c; planning's own privacy scan
over 1cdd3ed..d5e73cb (5 commits, 1958 blobs, 0 findings, PASS);
planning's own re-derivation of every comparison-record row from
the run directories' summary and empty-run tables (105 rows agree,
0 disagree); the record's figures (27 of 27 outputs per op and
tool, roundtrip 870 identical and 0 dropped for emend against 28
and 533 for openpyxl, medians 0.10 s and 0.81 s, the run log's
ratio 8.10 on all eleven ops, tier c roundtrip openpyxl cap 137 at
320.13 s and 16576384 kB); ruling N's settings in the 11 corpus
openpyxl records (timeout 600 s, memory 4096 MiB) and the scale
tiers' (3600 s, 16384 MiB) in the 24; ruling P's container-start
line in all 35 openpyxl records and in the comparison record;
ruling H's stderr count line in the run log once per openpyxl run
directory (24 at 0 files, 11 at 13 files and 4489 bytes); the
check log's 143 of 143 record matches, 70 of 70 record pairs, the
restored keep check with its controls and the family controls; the
superseded run script 04ce3671, its stderr ffae65ef and generator
c81d5833, the regenerated 7c32179e, the check script 84064a18, the
scan 568cd01f, the first scan 3a44144b, the first check log
b6dd7cee, the raw answer 41182690 and the first final scan
82ae55f7, each rehashed under `<phase>`; the generator's
defined-helper output f60e8ca3 read (yes with its control for all
three scripts, `keep_pairs_ok` named undefined in the old one, 56
names called). The independent check's PARTLY and its four fixes
are answered in the final report; the stderr's line 2850 is `yn`'s
definition, where bash reports the call, as planning read it too.

S. Ruling Q's defined-helper lines are generator output, printed
   by no P2 log; accepted for P2 as the session's statement with
   the kept output's sha256 named in the report. From P3 on, the
   gate log prints them for every script the phase generates, each
   with its planted control.
T. Ruling P's "record pair" reads as the pair's `.md`; the `.txt`
   holds the run tables only, as every pair's `.txt` has. The `.md`
   alone carries the median. No change.
U. `scripts/compare-record.py` carries two items into P3's first
   code commit, with tests: the scale section's settings sentence
   read from each scale run's `run.openpyxl.tsv` rather than fixed
   text; the op check on `run.openpyxl.tsv` applied to the scale
   directories as to the corpus ones. The committed comparison
   record is unchanged: its fixed text matches the tables planning
   read (3600 s, 16384 MiB in all 24).
V. The first check log, the first final scan (4 non-ASCII hits from
   the checker's two multiplication signs, the committed answer
   writing them as the letter x and saying so) and the never-logged
   first scan script are accepted as disclosed.

## Planning rulings during P3 (2026-10-18)

The doc commit c4b2598 verified by planning: specs/0037 (88c0fa8b)
and ROADMAP.md (e7e741a8) unedited, nothing else, the tree clean.
The code commits e1c4f87 (decision 3 A, ruling U) and 6a2c1bb
(item 7) read by subject and shape before the run log.

W. The reruns of "Harness runs" P3 are the seven distinct samples:
   part 5 P4's chartlabels, removechart, chartmore, chart3d and
   charts, and part 4 P3's addchart and chartkinds; `--charts-
   sample` runs once and is checked against both of its records
   (`m5p4-p3-charts-...-574a77d` and `m5p5-p4-charts-...-a8bfe71`),
   byte-identical by sha256 on both targets. The record names are
   `records/m5p6-p3-<sample>-<utc-date>-<shortsha>.md` and `.txt`
   as part 5 P4 named them, with the open record `m5p6-open-...`.
   The six logs follow part 5 P4's form: gate (with ruling S's
   defined-helper lines), test (with the API record), negative
   (P2's plus item 7's planted slow conversion), size, run, scan.

X. The two code commits' messages state numbers without the
   count sentence the scan requires (change AD); found by the scan
   generator before any log ran. Rewritten in ruling G's form: the
   two commits rebuilt on c4b2598 with identical trees and
   subjects, each message gaining "No log prints a count for this
   commit; the diff alone carries what it changes." before the
   Co-Authored-By line, nothing else; the generators' full-sha pins
   updated; the old pair kept as the local branch
   `p3-before-reword`, never pushed; the report gives the rewrite,
   the new hashes and an empty `git diff` old against new for both.
   Local, unreported commits (`specs/0001`).

Y. FINDING, from P3's dry negative log: N4 (tier a roundtrip under
   `--memory 64`) once ended `error 1` with a Python traceback
   ending in `MemoryError` at the script's last line, 1 of 6 runs;
   the other five read cap 137. Planning read the script: the
   `MemoryError` handler prints two lines and returns 137, and a
   second `MemoryError` raised inside the handler escapes `main`,
   so Python exits 1. Not a P3 regression (P1 and P2 read cap each
   time) and no recorded figure is affected (no P2 row is error);
   a defect of the ops script near the cap. Ruled as option (a): a
   third P3 code commit, `scripts/openpyxl-ops.py` alone with its
   test, where the `MemoryError` path writes its two lines with
   preallocated bytes through `os.write` and leaves by
   `os._exit(137)`, and a `MemoryError` raised during the handler
   also ends 137; the ops test gains a planted handler-time
   `MemoryError` read as 137, with a control that the unplanted
   path still prints its two lines. N4 unchanged. The commit's
   message carries the count sentence (ruling X). The dry negative
   is not rerun for a cap; the logged negative runs once with the
   fix named.

## Planning verification of P3 (2026-10-18)

Verified from primaries: the five commits c4b2598, ab00735,
940a1ef, ef9da37 and c5a529b on d5e73cb; `git diff` e1c4f87
against ab00735 and 6a2c1bb against 940a1ef empty, the old pair on
`p3-before-reword` (ruling X); the three code commits' messages
carry the count sentence (gate.log 6556-6563); ef9da37 holds the
ops script and its test alone; the record commit adds 26 files
under `records/` and nothing else; the six logs at the README's
hashes, each ending "checks all as expected: yes", the scan
"RESULT: PASS" under PLANNING_SPEC d94a5f62; planning's own
privacy scan over d5e73cb..c5a529b (5 commits, 1991 blobs, 0
findings); the size record's figures (budget 436992, core 0,
images 556, charts 492, 29 probes reproduced, verdicts 6, 21 and
2, findings none); the API record (756 keep, 0 rows differ); the
open record (1437 pairs, two more than P2's for decision 3 A's two
new tests, 0 findings); the independent check's inputs limited to
the thirteen files of `<phase>/check1`. Planning's own checks: the
eight record pairs (the seven samples, charts against both
records) compared to their predecessors over every column but
`wall_s` and `peak_rss_kb`, every line equal; the gate's
defined-helper program run by planning over the regenerated
negative script f5687867: 0 undefined of 49 names called, and the
control finds a planted name. The code read: the self-closing root
read with `end` at its `/>` and the root's end tag built from the
start tag's name, the write putting `>`, the anchors and that tag
in place of the two bytes; the open check's named container and
its removal after exit 124; the ops script's cap exit by `os.write`
and `os._exit(137)` with the outer catch.

Z. FINDING 1 (change PZ8) accepted: a check defect at the script's
   entry, nothing measured; the regenerated negative script's
   defined-helper line stands on planning's own run above. From P4
   on, each script's "shape greps over this script's own source"
   block also runs the defined-helper check over the script's own
   source, with its planted control, so a script regenerated after
   the gate log carries the line in the log it writes.
AA. The negative log's refused attempt is disclosed in the README
   only (RERUN_NOTE unset). Accepted for P3. From P4 on, every
   attempt refused at entry is named by RERUN_NOTE and hashed by
   RERUN_FILES in the log of the rerun, the run log's form, with
   no exception for an attempt that measured nothing.
AB. N8 ran the real conversion of the fixture under a 1 s timeout,
   not a planted slow conversion; what was planted is the timeout.
   The effect ruling A asked for is shown (return 2 with the
   reason, the container removed by name, 0 before and 0 after).
   The heading and 940a1ef's message stand as written, read as
   this. N8k's control container, created and never started, is
   accepted: `podman ps -a` counts it, which is what N8 counts.
AC. The independent check's unfixed notes: (a) the cap's RSS
   digits formatted at handler time with "-" as the fallback is
   accepted, the runner reads the outcome from the exit and the
   RSS from the last line's digits only; (b) a third stdout line
   when a MemoryError hits `last_line` after the class line leaves
   the exit 137 and the last line as the runner reads it, no
   change; (c) the string sort of the scale settings is cosmetic
   on one value per setting, corrected when the record script is
   next touched; (d) size.log's "M5 part 4's headroom" and "with
   its ruling W" labels are stale text in a log that passed,
   corrected in the next size log's generator (Milestone 6).
AD. `gen/defined.py` does not check that `yn` and `ynnot` are
   themselves defined. P4's gate adds that check with a control
   (a planted script calling `yn` and defining neither).
AE. P3 is DONE. Planning's ROADMAP edits for P4's doc commit, made
   in the working tree now: the P3 row DONE, the Milestone 6
   section inserted before "Unscoped" from the approved outline,
   and the Unscoped charts bullet rewritten to what Milestone 5
   delivered and what Milestone 6 carries. P4 is briefed as the
   spec's "P4: the README" with rulings Z, AA and AD.

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
