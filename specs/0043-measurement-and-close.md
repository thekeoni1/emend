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
are `specs/0037`'s. Nothing in `scripts/openpyxl-ops.py` changes.
The eleven pairings are the docstring's own; the five below are
planning's reading off the sample lists its docstring cites for
each op, the one sample whose request the op mirrors (ruling A).

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

## Planning rulings during P1 (2026-10-23)

A. (The ops table's pairing, R1 and R2.) The docstring of
`scripts/openpyxl-ops.py` names the eleven ops' samples one to one
and, for the five of this milestone, lists the samples of the
"Harness runs" each op's fixture served; it states no one-to-one
pairing for the five. "The operations" is corrected to say so: the
five pairings are planning's reading off those lists, each the one
sample whose request the op mirrors, and the table is the
authority for the comparison's record. The table's openpyxl column
is a summary of the docstring, which governs the op's content
(the trendline's equation, the x axis's `tickLblSkip`, the
chartsheet's `absoluteAnchor` of ext 0 by 0 are the op's, left out
of the table); "the first sheet" in the chart-analytic entry is W,
`wb.worksheets[0]` of kind `Worksheet`, as the docstring defines
it. No code change; `scripts/openpyxl-ops.py` stays unchanged.

B. (P1's four readings, Q1 to Q4.) Q1: the runner's refusal
message "one of the eleven ops" reads "sixteen"; that line and
`OPS_LIST` are the runner's only changes, and "nothing else in it
changes" is read with this correction. Q2: the record script's
header is its module docstring (citing this spec beside
`specs/0037`, the default stem `compare-m6p6-`), the record's title
("Milestone 6 part 6 P2") and the record's intro paragraph (this
spec beside `specs/0037`); all three change. Q3: the record script
takes no op, so its half of the first negative case is a
`run.openpyxl.tsv` whose op lies outside the sixteen, refused as
"names another op"; the second case is a row naming another of the
sixteen; no new refusal code. Q4: the gate, test, negative and scan
generators take part 5 P3's as the templates for shape; the test
log carries the ops test block from part 5 P2 and the record
script's test from Milestone 5 part 6 P2 with its expected lines
moved to sixteen ops, and drops the crate-test and API blocks; the
negative log holds the four cases the spec lists and no binary
case, since P1 touches no crate source or tool; the scan covers the
four logs, no record, and code paths under `scripts/` only.
`scripts/compare-record-test.py` changes its `OPS`, stem, counts
and its docstring's "eleven"; no new assertion, the Q3 refusal
being the negative log's. The gate's change dropping the two stale
constants is numbered as the generator numbers its changes.

C. (G1, the prune; the stray `rm` line.) P1's gate prunes
Milestone 6 part 5's two remaining target roots, `m6p5-p3/target`
and `m6p5-close/target`, in part 5 P1's multi-root form (each
listed with `du -sb` before, removed with `rm -r "${var:?}"`, the
kept names read at generation), as the standing rule that each
part's first gate prunes the previous part's roots; the spec's
"Phases" is read with it. The code commit 99b671d is verified: the
three scripts alone, the diff ruling B's and nothing else. The
session's stray `rm -rf /dev/null 2>/dev/null; true` in a model-copy
command, which failed without root and removed nothing, is recorded
here and in the phase's README as the session reported it: an `rm`
is written only inside a generated, logged script, only on a
directory that script created, in the `"${var:?}"` form; a dev
command copies with `cp` and removes nothing. Nothing in the
repository or the phase directory was touched by it.

D. (P1's scan, S1 and S2.) S1 (a): the scan's range starts at
part 5's close-out record f15a11b and holds three commits; the
part's first doc commit 9c62247, whose subject carries the part's
prefix and no phase's, is classed a doc commit, as every part's
first doc commit has been. S2: the scan generator dropped the
template's code-path controls that named older phases' commits as
redundant, keeping the verdicts it computes from git and asserts
(5e52758 and 8593ba0 accepted; 1e29c4e, ab00735 and part 5 P3's
test and CHANGELOG commits refused); accepted as a departure the
phase README records. For the record, with no ruling needed: block
P hashes the two scripts it does not print; T1's expected lines are
fixed at generation; the gate generator removes nothing. Written to
planning's scratch while the dry run d1 ran, and moved into the
spec before the logged run started.

E. (Dry run d1's finding F1.) The gate's check that the script runs
neither test nor the container tool counts three lines of its own
that write `scripts/compare-record-test.py` as fixture data (the
code commit's area list, the Python scripts in the diff, the test
file fixture); no earlier gate wrote that name, so the carried check
found it. The gate does not run either test or podman; the check is
right on the script as written. Fix (a): the generator writes the
three lines with the name built from the script's existing piece
variable, as the check builds its own words; the check and its
control stay. A numbered generator change, a fresh dry run d2 of
the three scripts, d1 kept and named in the README. Verified by
planning: d1's gate.log holds one UNEXPECTED verdict at that check,
and the gate script's only non-comment lines naming the two tests
or podman are the three fixture lines.

F. (The scan pre-check's finding F2.) The negative log prints
`scripts/compare-record.py` whole for the record-script cases, and
with it the docstring's `<corpus-runs>` and `<scale-runs>`, which
no root line of that log declares; Milestone 5 part 6 P2's test
log declared both (its change RB) and the negative fork carried the
cases without the two lines. Verified by planning: the pre-check's
scan.log reads structural checks failed 1, total hits 0, and the
installed negative.log holds the two placeholders twice each and no
root line for either. Fix (b): a numbered negative-generator change
adds the two root lines in RB's wording, the case code unchanged; a
dry rerun of the negative first; then the logged rerun in the
template's carried form (`RERUN_NOTE` naming F2, `RERUN_FILES`
naming the superseded log and script, both moved to the phase's
superseded directory, every value given to the script written with
placeholders and no absolute path); gate and test stand as logged;
then the scan pre-check again, and on PASS the draft, the check,
the final scan and the record commit. d1's and d2's outcomes and
this rerun are named in the README.

## Planning verification of P1 (2026-10-23)

Verified from primaries after the implementing session's report:

- Commits: doc 55e6363 (this spec with ruling A, blob ceed814e),
  code 99b671d (the three scripts alone, the diff ruling B's),
  record 65d4c5a (parent 99b671d, six files under
  `records/logs/m6p6-p1/`, ASCII, the diff-alone sentence, the
  Co-Authored-By line alone). origin/main read by `git ls-remote`
  is ee78ffa; nothing pushed.
- Logs, hashes equal to the report's: gate 3a242a7b, test
  be7ac536, negative 530f3630 (the ruling F rerun, its "rerun:" line
  naming F2, three superseded files hashed, the two root lines
  present), scan 0dd83b52 (final; check answers present 1, hits 0,
  structural 0, RESULT: PASS); each log ending "checks all as
  expected: yes"; gate and test at PLANNING_SPEC 17adfe77, the
  negative rerun and the scan at 38bd78f4; no "/home/" line but the
  scan's planted control; no non-ASCII byte.
- The gate: the prune removed part 5 P3's and close-out's target
  roots (9583979959 and 2143316010 bytes) and found P1's and P2's
  not there (ruling C); the self-check "lines: 0" (ruling E); the
  charts probe 1151328 against part 5's record's 1151328,
  difference 0, the budget 451448, no stale constant printed as a
  figure (ruling BI, item 5).
- The test: the ops test block 109 ok lines and PASS; the record
  script's test 34 ok lines and PASS.
- The negative: C0, R0, N1 (runner and record script), N2, N3, N4
  as ruling B set them.
- Check 1: the brief a4b702a3 with the frozen draft f3a9d08c inline,
  no absolute path; the answer 67e4ccfd verbatim in
  `independent-check-1.md` 6169768a and quoted in the final report
  a9460cce; verdicts yes, yes, yes with exceptions (ruling G).
- Planning's own privacy scan of 55e6363..65d4c5a: 2 commits, 2216
  blobs, 0 findings, PASS.

P1 is DONE.

G. (Check 1's exceptions.) Accepted. (a) The second pre-check's
path is the session's; planning read its hash as the check did.
(b) The report's shortened quote of the stray `rm` line is a quote
of the spec's full one. (c) The timing holds: rulings E and F were
given in planning's messages before each fix, as this spec's
rulings record them. (d) The dry runs, forks, models and generator
hashes are the session's statements, as every phase's are. (e)
origin/main is ee78ffa by `git ls-remote`, read by planning. (f)
The local date is the session's; the logs' UTC times are the
primaries. (g) As worded. The prune's visit to part 5 P1's and
P2's directories is part 5 P1's form. No change.

## Planning rulings during P2 (2026-10-23)

H. (The generators' readings R1 to R9.) R1 (a): five logs, gate,
test, negative, run and scan, the test and negative P1's carried
unchanged but for names and roles; roles base 65d4c5a and doc
d88f735, the doc commit the phase commit since P2 expects no code
commit; no check log. R2 (a): the run log's template is Milestone 5
part 6 P2's regenerated run script, pinned by hash 7c32179e, its
generator showing it equal to that log's printed run-script-3 plus
the RK and RL replacements of `specs/0037` rulings Q and R; the
part's numbered changes then make the names, the sixteen ops, the
m6p6 stems and the settings. R3, each confirmed against `specs/0037`
rulings M, N and O and the Milestone 5 records: corpus openpyxl
`--memory 4096 --timeout 600` with the open check; corpus emend by
`scripts/passthrough-run.sh`, the release `emend-copy`, both labels,
the open check, runner defaults; scale openpyxl `--memory 16384
--timeout 3600`; scale emend as `specs/0023`, both labels on every
tier, `--memory 2048 --timeout 3600`, the host `xlsx-diff` for both
labels on tier c; the open check on tier a only for both tools; the
order corpus runs, records, keep and open record, then the tiers a
to d one at a time, openpyxl then emend per op, then the comparison
record. R4 (a): the ops table governs; `chart-analytic` runs
`--chart-axis-sample` and `named-style` runs `--namedstyle-sample`,
the other three by `specs/0037` ruling O's rule; the run log prints
the pairing. R5 confirmed from the spec's names: 32 corpus pairs, 34
emend and 34 openpyxl scale pairs, the open pair, the comparison
record, 203 files. R6: the keep against part 5 P3's open record,
0 new pairs expected, any new or gone pair a finding. R7: the gate
prunes P1's target root in the single-root form. R8 (a): the
restricted dry run as Milestone 5 part 6 P2's, two ids, roundtrip
and three of the five new ops, tier a only, no keep, stand-in
directories. R9: the wall-time estimate is the session's; a cap or
timeout is an outcome and nothing is rerun for it.

I. (P2's generators, G-R1 and the departures.) G-R1 (a): the gate's
structure check takes Milestone 4 part 1 P3's form for a phase of
doc commits only (its change A44), with its four controls, and the
carried check that requires a code commit is kept as a control that
refuses this range. Departure NC59 accepted: the control that read
the runner's "eleven" at the base now reads the runner at P1's doc
commit 55e6363, the base naming "sixteen". The run fork added no
exemption for the new ops; a record-check trip in the dry run is a
finding. RUN-R1 noted: the comparison record's row check first runs
at the end of the logged run; a failure there is a finding, the runs
and records standing and the check alone redone with the fix named,
in Milestone 5 part 6 P2's check-log form (`specs/0037` rulings Q
and R). Written to planning's scratch while dry run d1 ran, moved
into the spec before the logged run.

J. (The logged run's refusal, finding F3.) The run script stages
each record pair under its scripts directory before installing it
into the records destination it is given; dry run d1 staged its 32
files there and left them, and the logged run, with the same date
and sha in its stems, refused at its pre-flight as it is built to,
writing no run log and measuring nothing. Verified by planning: the
run log empty, the script's own standard error one line naming the
staged roundtrip record, 32 staged files, no run directory and no
record naming d88f735, the tree holding the working spec alone, no
script running. Fix (a), no script or generator change: d1's staged
files move to the dry run's directory; the refused attempt's three
files (the empty run log, the empty outer stderr, the script's
stderr) move to the phase's superseded directory; the run step
alone reruns into the phase's logs at the same PLANNING_SPEC with
`RERUN_NOTE` naming F3 and `RERUN_FILES` naming the three, in the
carried form; gate, test and negative stand as logged (the prune
ran once); the README names d1's staged records and the refused
attempt. A dry run's staged files are cleared before a logged run
from here on, and the next part's run generator makes the staging
directory per invocation.

K. (The scan pre-check's finding F4.) The run log prints
`<superseded>` twice, in the `RERUN_NOTE` the session wrote for the
ruling J rerun (its input line and its "rerun:" line), and the run
script declares no root line for that token; the session's authoring
slip in the note, not a script or generator defect. Verified by
planning: the two lines, no root line, the pre-check's structural
checks failed 1 with total hits 0, the run log ending yes, 203
records installed, the open record 2237 pairs with none new, gone or
changed. The measurements stand; the run log is not rerun for a
word. Fix: a numbered scan-generator change by which a placeholder a
log prints may be declared by a root line in the logs' README as
well as in the log, with a control planting a token declared in
neither and shown refused; the README declares `<superseded>` as the
phase's superseded directory; the scan regenerated and the pre-check
rerun; the README names F4 and this ruling. Standing from K: a
`RERUN_NOTE` or `RERUN_FILES` value uses only tokens the script it
is given to declares.

## Planning verification of P2 (2026-10-24)

Verified from primaries after the implementing session's report:

- Commits: doc d88f735 (this spec with rulings B to G and the P1
  verification, `ROADMAP.md`'s P1 row), record 3102354 (parent
  d88f735, 210 files: the seven under `records/logs/m6p6-p2/` and
  the 203 records ruling H R5 counted; ASCII; the diff-alone
  sentence; the Co-Authored-By line alone). origin/main by `git
  ls-remote` is ee78ffa; nothing pushed.
- Logs, hashes equal to the report's: gate 64cb2e15 (the prune of
  P1's target root, 6167760653 bytes, one directory; the docs-only
  structure form of ruling I), test 6c23f337, negative f9cbfb2a,
  run af820041 (the ruling J rerun; 203 records installed; the open
  record 2237 pairs, none new, gone or changed; every row of the
  comparison record derived again), scan 0643057e (final; check
  answers present 1, hits 0, structural 0, RESULT: PASS); each
  ending "checks all as expected: yes"; the README 39fe654a with its
  root line for `<superseded>` (ruling K); no "/home/" line but the
  scripts' pattern text and the scan's planted control; no non-ASCII
  byte. PLANNING_SPEC d761db90 for gate, test and negative, 4a971987
  for the run, ad36059f for the scan.
- The comparison record `records/compare-m6p6-2026-10-24-d88f735.md`
  (56673747): 48 corpus rows, every one 27 files and 27 ok for both
  tools on every op; 102 scale rows; the one outcome not ok the tier
  c openpyxl roundtrip, `cap` at exit 137, as Milestone 5's was.
- Check 1: the brief be64f4d6 with the frozen draft 79ff8dc5 inline,
  no absolute path; the answer 75e72493 verbatim in
  `independent-check-1.md` a8df0c39 and quoted in the final report
  ad01c372; verdicts yes with exceptions, yes, yes with exceptions
  (ruling L).
- Planning's own privacy scan of d88f735..3102354: 1 commit, 2425
  blobs, 0 findings, PASS.

P2 is DONE.

L. (Check 1's exceptions and the prune's form.) The gate pruned
P1's one target root through P1's multi-root helper rather than the
single-root form ruling H R7 named; the same checks (bytes listed
before, `rm -r` without `-f`, kept and gone), one directory:
accepted, the README's departure now ruled. Change LC's second
control, an acceptance case, is of the same change and accepted.
The self-source line's label naming the README's root lines, where
the code checks the source against its own alone, is a label and
stands. The F4 quotes live in the superseded pre-check; the run's
exit 2 at its pre-flight and the timing of rulings J and K are the
session's statements, the timing confirmed by planning's own
messages; the README's "H to J" in its first paragraph predates K,
which the same paragraph names. No change.

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
