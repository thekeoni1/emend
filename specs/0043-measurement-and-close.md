# 0043: Milestone 6 part 6, measurement and close

Status: APPROVED by the maintainer 2026-10-23, decisions 1 to 3
each taken as A, the recommended option, typed in the planning
session. Sent to the implementing session the same day.

Context: `CLAUDE.md` ("What emend is": the comparison point is
openpyxl, with the four exclusions; "The fidelity harness is the
acceptance instrument"; "Dependencies and licences"; "32-bit and
sizes"; "Elevation and system changes"), `ROADMAP.md` ("Milestone
6", the outline approved 2026-10-16, decision 4 A: the milestone
ends with a measurement part that reruns the openpyxl comparison of
Milestone 5 part 6 with the new ops; "Unscoped"),
`specs/0001-bootstrap.md` ("Rules for every phase"),
`specs/0016-new-workbook-and-sheets.md` (the sheet add's namespace
rule), `specs/0023-scale.md` ("The tiers"), `specs/0029-engineering-
close.md` ("The README", decision 4 A), `specs/0037-measurement-and-
close.md` (the image, the ops script, the comparison runner and
record script, the scale reach, the README's form; rulings C, K, L,
N, O, P, Y, AC), `specs/0038-chartsheets.md` (rulings J, L, M, N),
`specs/0039-chart-kinds-and-formatting.md` (ruling P),
`specs/0040-analytic-chart-elements.md` (ruling U),
`specs/0041-chart-text-and-edits.md`, `specs/0042-gradient-fills-
and-named-styles.md` ("Size and packaging"; rulings BB, BI, BJ). No
external text is cited; openpyxl's behaviour is measured here, not
quoted.

## Goal

Milestone 6 ends as Milestone 5 did: the comparison of `specs/0037`
rerun at one commit with the five operations the milestone added,
each tool's output compared with its input by the harness, each
run's wall time and peak memory beside the other's, LibreOffice
asked to open every output; the size record that stands as the
milestone's closing figure, reading the `charts` cost whole as
`specs/0038` ruling M carried; the API review rerun; the README's
feature list, fidelity and comparison sentences brought to
Milestone 6's records; the one carried refusal decided (decision
3); and the Unscoped list pruned of what the milestone delivered.
No feature is added. The core moves only for decision 3, if A.

## Facts planning checked (2026-10-23)

- `scripts/openpyxl-ops.py` already knows sixteen ops: the eleven
  of `specs/0037` and `chartsheet`, `chart-format`,
  `chart-analytic`, `chart-text` and `named-style`, each added by
  a part's P2 as a fixture op with its test in
  `scripts/openpyxl-ops-test.sh`. `scripts/compare-run.sh`
  (`OPS_LIST`), `scripts/compare-record.py` (`OPS`, `SCALE`, the
  default stem `compare-m5p6-`) and `scripts/compare-record-test.py`
  still name the eleven. `specs/0038` ruling L left to this part
  whether the comparison gains the new ops.
- The openpyxl image `localhost/emend-openpyxl:3.1.5-dfsg-2-deb13`
  (id 894b239fc0d1) and the LibreOffice image
  `localhost/emend-lo:25.2.3-2-deb13u6` are on the host. The image
  record `records/openpyxl-image-2026-10-16.md` stands; no rebuild.
- The scale tiers are on disk under `<corpus>/scale` at the ids
  `records/m3p1-gen-2026-10-01-a509ad3.md` names. The comparison
  of Milestone 5 ended openpyxl at its memory cap on the tier c
  roundtrip (`records/compare-m5p6-2026-10-16-5e52758.md`); the
  same cap and timeout stand, and the same outcome, if it recurs,
  is an outcome.
- The budget after part 5 is 451448 section bytes against the
  ceiling 458752 (`records/size-m6p5-2026-10-23-fe280ff.md`): 7304
  of headroom. The `charts` cost is read from that record's charts
  probe over the format probe; part 1's record attributed 1968 of
  the core's growth to the chartsheet kind in the sheet path
  (`specs/0038` ruling M), and this part reads it, not re-measures.
- The gate generator of part 5 P3 prints the charts probe's total
  against two stale constants, Milestone 5 part 4's 1055368 and
  part 5 P4's 1075632, beside part 4's record's figure (`specs/0042`
  ruling BI); this part's gate generator drops the two constants
  and prints against part 5's record's figure alone.
- `README.md`'s feature list cites `records/size-m5p6-2026-10-18-
  ef9da37.md` on every line and its `charts` line names the three
  Milestone 5 specs; its comparison paragraph cites the Milestone 5
  record. Every sentence holding a digit cites a path under
  `records/` that exists at HEAD, checked by P4's run log.
- `ROADMAP.md` "Unscoped" holds the openpyxl `<sheet>` namespace
  refusal (`specs/0038` ruling J) and the charts bullet that names
  Milestone 6's scope as what was left; the second is delivered.

## The operations

The eleven ops of `specs/0037` "The operations" as they stand, and
the five of this milestone, each one of emend's existing samples
and the openpyxl op a part's P2 wrote to mirror it. The two tools
are never compared with each other's output: each output is
compared with the input by `xlsx-diff`. W, M and the placement rule
are `specs/0037`'s. Nothing in `scripts/openpyxl-ops.py` changes;
the pairing below is read from its docstring, and a pairing the
implementing session reads otherwise is a ruling before P1's code.

| op | emend | openpyxl (`scripts/openpyxl-ops.py <op> <in> <out>`, as the docstring says) |
| --- | --- | --- |
| chartsheet | `--chartsheet-sample` | `create_chartsheet()`, then a `BarChart` with one series over W's B2:B(M+1), categories A2:A(M+1), added on it |
| chart-format | `--chart-format-sample` | a `LineChart` over the same with a marker, a dashed line and one formatted `DataPoint`, at (M+2, A); a `PieChart` over the same with one exploded `DataPoint`, at (M+18, A) |
| chart-analytic | `--chart-axis-sample` | a `LineChart` over the same with a linear `Trendline`, fixed `ErrorBars`, drop lines and scaled axes, at (M+2, A); a date column written after the last used column, and a `LineChart` over it with a `DateAxis` and a log y axis, at (M+18, A) |
| chart-text | `--chart-text-sample` | a `BarChart` over the same with a style, a title, an axis text property and data labels with a text property, at (M+2, A) |
| named-style | `--namedstyle-sample` | a `NamedStyle` with a bold font and a linear `GradientFill` of two stops, added with `add_named_style` and set on (M+1, A) through `cell.style` |

Decision 1 sets the reach. Option A: all sixteen ops run at this
part's P2 commit, emend on both labels and openpyxl on the host,
over the 27 corpus workbooks, every output opened; the eleven are
rerun rather than cited, so every figure of the comparison record
comes from one commit of emend. Option B: the five new ops only,
the eleven's figures cited from the Milestone 5 record.

## The comparison runner and the record

`scripts/compare-run.sh`'s `OPS_LIST` gains the five ops; nothing
else in it changes. `scripts/compare-record.py`'s `OPS` gains the
five, `SCALE` keeps its shape (every op on tiers a and b, roundtrip
alone on c and d), the default stem becomes `compare-m6p6-`, and
the record's header names this spec beside `specs/0037`.
`scripts/compare-record-test.py` follows. `scripts/passthrough-
record.py` is unchanged (the label `openpyxl` exists). The run
directories and record pairs are named as Milestone 5 part 6 named
them with `m6p6` in place of `m5p6`: `records/m6p6-<op>-<utc-date>-
<shortsha>.md` and `.txt` for emend's corpus runs, `records/m6p6-
openpyxl-<op>-...` for openpyxl's, `records/m6p6-scale-<tier>-<op>-
...` and `records/m6p6-openpyxl-scale-<tier>-<op>-...` for the
tiers, and `records/compare-m6p6-<utc-date>-<shortsha>.md` for the
comparison record, which cites the image record's hash.

## What is measured and how it is read

As `specs/0037` "What is measured and how it is read": per op and
workbook, each tool's outcome, wall time, peak RSS, the diff's
entry columns and the open verdict; the run log derives fidelity,
speed and memory per op. A workbook openpyxl cannot load is an
outcome with its exception class counted. Nothing is rerun to
change a figure. The five new ops' openpyxl rows are read with
one more eye: an op whose openpyxl output LibreOffice does not open
is a measured fact the record states, not a defect of the runner.

## The scale tiers

As `specs/0037` "The scale tiers" with the sixteen ops: tiers a and
b through the openpyxl runner for every op with `--memory 16384`
and `--timeout 3600`; tiers c and d for `roundtrip` only under the
same cap; emend's side rerun at the P2 commit for the same ops and
tiers, both labels only where `specs/0023` ran both. A tier the cap
or the timeout ends is recorded as `cap` or `timeout`.

## The engineering close

The carried items, each with its source; nothing here is new.

1. The `charts` cost (`specs/0038` ruling M): decision 2. Option A,
   the size record states the cost whole from part 5's record's
   figures, lists each part's growth from the parts' size records
   (parts 1 to 4 under `charts`, part 5 the core), names the 1968
   of part 1's core growth as read by ruling M, and no code moves.
   Option B, the chartsheet kind's sheet-path code is regated under
   `charts` in P3, the core's shrink measured and the `charts`
   cost's growth measured, each against the record.
2. The openpyxl `<sheet>` namespace refusal (`specs/0038` ruling J,
   Unscoped): decision 3. Option A, in P3 the sheet add writes the
   relationships namespace declaration on the `<sheet>` element it
   writes when the workbook root binds no prefix for it, as
   openpyxl writes its own; `add_sheet` and `add_chartsheet`
   succeed on the openpyxl fixture of part 1 P2, one test each on
   a built package and on that fixture, the output opened; the
   refusal's message leaves the catalogue if nothing else reaches
   it; the corpus samples `--sheet-sample`, `--chartsheet-sample`
   and `--copy-sample` rerun byte-identical to their latest records
   by sha256 (no corpus producer writes the form). Option B, the
   bullet stays in Unscoped; no code.
3. The size record `records/size-m6p6-<utc-date>-<shortsha>.md`,
   written in P3, stands as Milestone 6's closing figure: the
   budget against part 5's 451448 and the ceiling 458752; the
   `charts`, `images`, `comments` and `read` costs against part 5's
   record; item 1's attribution; this part's own movement judged
   by "Size" below.
4. The API review rerun (`specs/0029` decision 3's form, as part 5
   P3 ran it): `records/api-review-m6p6-<utc-date>-<shortsha>.md`,
   every row judged; the rows that change against part 5's 844 are
   none unless decision 2 is B.
5. The gate generator's stale charts constants (`specs/0042` ruling
   BI): dropped in P1's gate generator, the charts probe's total
   printed against part 5's record's figure alone.
6. The MSRV policy: the four pins agree at the close; the hygiene
   step proves it. No move. The Milestone 2 Excel check gates no
   push in this milestone (the Milestone 6 heading); untouched.

## The README

`README.md` as `specs/0029` decision 4 A shaped it and Milestone 5
part 6 brought it forward, now brought to Milestone 6: every
feature line's cost cites P3's size record; the `charts` line names
`specs/0038` to `specs/0041` beside the Milestone 5 specs; a new
core line or sentence names gradient fills and named styles with
`specs/0042`; the fidelity paragraph gains Milestone 6's samples
(the chartsheet, chart format, chart axis, chart text, gradient and
named style samples of the parts' P3 reruns, citing their records)
beside Milestone 5's; the size sentence cites P3's record; the
speed sentence keeps the scale record and gains this part's tier
figures; the comparison paragraph cites `records/compare-m6p6-
...md` only, stating per op or for the set what each tool left
byte-identical, dropped and opened, the median wall times and peak
RSS. Every sentence holding a digit cites a path under `records/`
that exists at HEAD; P4's run log checks it as before (the command
printed, a control on a planted sentence; no U+2014, no absolute
path, no corpus content). The sentences state measurements; they
do not say "faster" or "better".

## Size

The budget after part 5 is 451448 against the ceiling 458752: 7304
of headroom. This part's code in the crate is decision 3's, if A,
in the sheet writer of the core: expected 0 to 600 on the core,
accepted up to 1500, and the budget stays under the ceiling, a
figure over it a finding and a ruling before any further commit;
every other probe that links emend expected the core's growth and
accepted up to 2000 over it; the three probes that link no emend
expected 0 and accepted up to 2000. If decision 3 is B and decision
2 is A, no code moves: the core expected 0 and accepted up to 100,
every other probe expected 0 and accepted up to 2000. If decision 2
is B, the core's shrink and the `charts` cost's growth are measured
and stated, expected within 3000 of each other.

## 32-bit

The openpyxl image is amd64 and runs on the host only; the record
says so. emend's corpus runs run on both labels; the scale reruns
on the labels `specs/0023` used per tier. Decision 3's code, if A,
writes an attribute and touches no size or offset.

## Harness runs

P2: the ops of decision 1, 27 workbooks, openpyxl on the host and
emend on both labels, every output opened; the scale tiers as "The
scale tiers". P3: decision 3's reruns, if A, byte-identical to their
records by sha256. P4's run log holds the README check. No other
corpus run.

## Phases

Each phase as `specs/0001` "Rules for every phase": a doc commit
first, the code commits, one record commit, the scan last, at most
two independent checks, the report under `<phase>`, then stop.
Target root `<home>/.cache/emend-build/m6p6-pN/target`. The gate
generator is part 5 P3's with item 5's change and the part's names,
as a numbered change.

### P1: the scripts

`scripts/compare-run.sh`, `scripts/compare-record.py` and
`scripts/compare-record-test.py` gain the five ops and the stem;
the test log runs `scripts/compare-record-test.py` and
`scripts/openpyxl-ops-test.sh` and prints them, never the gate, so
CI is unchanged; the negative log: an op outside the sixteen
refused by the runner and by the record script, a run directory
whose `run.openpyxl.tsv` names another op refused, the record
script refusing runs that do not cover the same workbooks, a
planted path in a stderr file shown to stay there. No corpus run.
Decision 3's code, if A, is not here.

### P2: the runs

The corpus runs of decision 1 and the scale runs at the phase
commit; the record pairs; the comparison record by
`scripts/compare-record.py`; the run log's derivations, asserting
every row of the comparison record against the summaries it was
built from; the open record.

### P3: the close

Decision 3's code and tests (if A) and the catalogue; decision 2's
code (if B); the size record (item 3); the API record (item 4); the
reruns of "Harness runs" P3; the CHANGELOG bullet under Fixed for
decision 3 (if A) and under Changed for decision 2 (if B), nothing
otherwise.

### P4: the README

The README's edits; the run log's README check; `CHANGELOG.md`
unchanged beyond P3; planning's ROADMAP edits (the Unscoped charts
bullet gone, the `<sheet>` bullet gone if decision 3 is A, the part
6 rows) in this phase's doc commit.

## Close-out push (after P4)

As part 5's (`specs/0042` "Close-out push" and rulings BJ and BK):
the generator transforms part 5's scripts (`gen.py` 9e3f469b and
the three it printed, carried cmp-equal), its numbered changes
continuing from 61; the doc commit holds this spec and `ROADMAP.md`;
the base is part 5's close-out doc commit ee78ffa, named by its
subject ("M6 part 5 close-out: commit planning docs (the P3
verification and rulings)"); `PREV_RECORD_PATTERN` is `^M6 part 5
close-out record` (f15a11b), in the range exactly once; the
close-out doc commit's subject is "M6 part 6 close-out: commit
planning docs (the P4 verification and rulings)";
`CARGO_TARGET_DIR` printed and checked as `<home>/.cache/emend-
build/m6p6-close/target`; the visibility read PUBLIC with its
control (ruling BJ); no Excel-record check, the stray-record count
0 kept; the record commit "M6 part 6 close-out record: the push,
the CI run and the scan" holding the three logs only; pre mode on
a clean tree; one independent check before the report. The push
carries f15a11b and this part's commits, on planning's word and
the maintainer's authorisation typed in the implementing session.
`ROADMAP.md`'s Milestone 6 heading then reads DONE and PUSHED, and
the close-out's verification says what the next milestone carries.

## Decisions for the maintainer

1. The comparison's reach: A (recommended), all sixteen ops rerun
   at one commit, so the record's every figure is of this emend;
   B, the five new ops only, the eleven cited from Milestone 5's
   record.
2. The `charts` cost carried by `specs/0038` ruling M: A
   (recommended), stated whole in the size record with each part's
   growth and the 1968 named, no code; B, the chartsheet kind's
   sheet-path code regated under `charts` in P3.
3. The openpyxl `<sheet>` namespace refusal (Unscoped): A
   (recommended), fixed in P3 by the declaration on the `<sheet>`
   element, the core expected 0 to 600; B, left in Unscoped.

## Out of scope

- Any new operation beyond the sixteen, any new emend sample, any
  change to `scripts/openpyxl-ops.py`.
- A rebuild of either image, or a newer openpyxl.
- The publish step (Milestone 4 decision 1, held).
- openpyxl behaviours emend does not mirror beyond this milestone's
  list; a next milestone's theme is the maintainer's.
