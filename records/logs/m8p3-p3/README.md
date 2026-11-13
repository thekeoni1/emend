# M8 part 3 P3 logs

specs/0049-excel-check-and-release.md, "The release texts and the
manifest (P3)", "Phases" and "### P3", with specs/0045's P3 sections and
rulings, planning's rulings A to J committed in the doc commit (specs/0049
at sha256 3abf5ed71013adc544f2fd119d47ce6d9b6e1e25f49b59feee3a046a77622e61),
and rulings K and L written in the working spec during this phase. The
gate, the negative and the run took the working spec at PLANNING_SPEC
12fe9f11c20ba40465477670a26dd5241c911fbec127ed46515676fd25c91a26 (each log
prints it); PLANNING_ROADMAP was not given, the working ROADMAP.md being
the blob (001f992523bd0c898975db19e10ad7e0ee6a9ad0199bf68529cd50aaeee4393b). `<phase>` is the
phase's directory, `<home>/.cache/emend-build/m8p3-p3`.

root: <home> stands for the home directory of the account the runs ran under
root: <phase> stands for <home>/.cache/emend-build/m8p3-p3, the phase's directory
root: <work> stands for run.log's working directory, as run.log's root line 72 defines it
root: <scratch> stands for <phase>, as in the gate and run logs; the independent check's materials are under <scratch>/check1

| Role | Commit | Subject |
|---|---|---|
| base | f8bd007 | M8 part 3 P2 record: the Excel record, the logs, the README, the check |
| doc | 43b8851 | M8 part 3 P3: commit planning docs (P2's verification and rulings H to J) |
| code | be6cdb1 | M8 part 3 P3: the crate's version is 0.1.1 |
| docs 1 | 41203c5 | M8 part 3 P3: the CHANGELOG and the README carry 0.1.1's texts |
| docs 2 (phase) | 7ea64ba | M8 part 3 P3: the size texts cite the budget at the 0.1.1 manifest |

The code commit changes the version line of crates/emend/Cargo.toml and
the emend entry's version line of Cargo.lock, written by cargo (`cargo
update --workspace --offline`). Docs 1 writes the 0.1.1 texts; docs 2
is ruling K's: the size sentences cite the budget at the 0.1.1 manifest
from this directory's gate.log and part 2's size record's figure before
the version changed. The crate-level doc comment is unchanged.

No file name, sheet or table name, note or cell content of any private
workbook is in these logs or in this README. No registry token is in any
session, file or log. The texts quoted below are the logs' own lines.

## The logs

| File | Bytes | Lines | sha256 | Last line |
|---|---|---|---|---|
| gate.log | 982410 | 12772 | 773a1698735e9b76bfc4a9e5b61549dfcf8f711623d57a9e7de20e8ab4d729a9 | gate log checks all as expected: yes |
| negative.log | 422385 | 5802 | 98b213f71fd015500d9e493ed95195dc7f4e137df1dfde46a8e3ba95e30aca8d | negative log checks all as expected: yes |
| run.log | 221093 | 3028 | e0c107da4eeab603c9064167653e300aa231162ddbafc4d25cbf9cc4c9c3b4d6 | run log checks all as expected: yes |
| scan.log | | | | the final scan, written after the independent check and before the record commit, which holds it |

No record file is made in this phase.

The scripts are <phase>/scripts/m8p3-p3-gate.sh, -negative.sh, -run.sh
and -scan.sh. Each log prints the script that wrote it. Dry run d2, under
<phase>/dry/d2, ran the same four scripts; the logged driver,
<phase>/logged.sh, refuses to start unless each is cmp-equal to d2's.

Each log's own start and end: gate.log line 385 "run at
2026-11-13T01:29:02Z", line 12771 "run ended at 2026-11-13T01:41:53Z";
negative.log line 105 "run at 2026-11-13T01:41:54Z", line 5801 "run ended
at 2026-11-13T01:42:06Z"; run.log line 62 "run at 2026-11-13T01:42:07Z",
line 3027 "run ended at 2026-11-13T01:42:16Z".

## The gate

P2's gate with changes A107 to W107.
- The prune (gate.log lines 183 to 236): P2's build target alone, "their
  bytes in all: 6475256473" (line 232), removed by
  `rm -r "${PREV_DIR:?}/${n:?}"`, "target: rm -r exit 0" (line 234);
  "the kept names, 26" (line 193), "every kept name is still there:
  as-expected" (line 236). The prune form's control ran on a directory
  the gate made (lines 185 to 190).
- The crate's tests: "the figures this run must give: off 544, ch 719, im
  581, ci 761, cc 768, cr 826, all 1244" (line 11724), the phase adding
  no test.
- The probes (specs/0049 "Size" and ruling K): the format probe "sections
  989464" (line 12158); "the format probe's sections at this phase:
  989464; expected (part 2's size record): 989480; the difference: -16"
  (line 12165); the FINDING of line 12167 below; and "the format probe's
  sections are 989464 and the budget 456248, the figures the texts cite
  (specs/0049 ruling K; change W107): as-expected" (line 12172), with its
  controls (lines 12173 and 12174).

## The negative

P2's negative as the frame with Milestone 7 part 2 P3's cases, changes
NA68 to NJ68.
- The roles: the second docs commit is the phase commit, the first docs
  commit refused in its place (lines 97 and 98).
- The packaging cases over a `git archive` of the phase commit outside
  any checkout: (a) the manifest of 612986a, no include list, cargo
  package exit 101 naming `_rels/.rels` (line 5677); (c) the code
  commit's manifest, exit 0, the file listed (line 5678); (b) information,
  the manifest less its `template/_rels/.rels` line still lists the file,
  75 files (line 5681).
- The Excel sentence's figure check refusing a planted copy (line 5765)
  and a swapped copy (line 5770); the feature-list check refusing a copy
  lacking one feature (line 5778).
- Ruling H (2)'s stale line is not in this frame, so nothing is reworded
  (ruling L).

## The run

Milestone 7 part 2 P3's run with changes XU to XZ and YL to YN.
- The gate log given as its second argument: records/logs/m8p3-p3/gate.log
  (line 12).
- The packaging dry run: the checkout's list "entries 76" (line 2746),
  the archive's "entries 75" (line 2833), `.cargo_vcs_info.json` the
  difference; "the aborting-upload warning: 1; warning lines: 1; error
  lines: 0; \"Packaged N files\": 75" (line 2868); one "Packaging emend
  v0.1.1" (line 2874); the archive's manifest names version 0.1.1 (line
  2841); CARGO_REGISTRY_TOKEN and CARGO_REGISTRIES_CRATES_IO_TOKEN absent
  (lines 2735, 2736); `<home>/.cargo/credentials.toml` absent by
  existence alone (line 2741). The archive holding no repository, the
  dry run there does not pass the dirty flag (ruling L).
- The Excel sentence: the record's totals line (line 2882); README's and
  the 0.1.1 entry's sentences each hold the seven figures in that order
  (lines 2887 and 2891).
- The size sentence (ruling K): "the run's own gate log prints the format
  probe at 989464 and the budget at 456248" (line 2905); "458752 - 456248
  = 2504; part 2's size record's budget: 456264" (line 2908); README and
  the 0.1.1 entry each hold one size sentence citing the gate log and
  part 2's size record with the four figures in order (lines 2913 and
  2917).
- The CHANGELOG heading "## 0.1.1 (unreleased)" (line 2922), its form
  (ruling J) "yes" (line 2923).
- Ruling F's sentence in both places (line 2933); the 0.1.0 entry and all
  below it the base's, byte for byte (line 2944).
- The citation rule: the gate log path resolved to this run's own gate
  log, which prints ruling K's figures (line 2950); "sentences holding a
  number 29; uncited 0; missing 0" for README (line 2959) and "5; uncited
  0; missing 0" for the 0.1.1 entry (line 2967). Sentences citing a spec
  alone, and the version heading, are listed and not refused (ruling L).
- The doc comment: lib.rs is the base's (line 2975); the manifest's 12
  features each named in its list; its opening is the README's first
  paragraphs word for word.

## FINDING lines

One: gate.log line 12167, "FINDING: the format probe's sections (989464)
or the budget (456248) differ from part 2's size record's (989480,
456264); recorded, not a rerun (specs/0049 "Size"; change Q107)". Ruling
K keeps it, recorded and not fixed: the texts cite the gate's figures
and part 2's beside them. gate.log line 12138 holds the word in a
heading; run.log lines 3012 and 3013 are its section heading and
"FINDING lines printed above: 0". The negative prints none.

## Figures from files outside this directory

Each is named with its source; none is in a committed log:
- <phase>/logged.sh (sha256 12cc6ff9f8c4dc2b20440fe9619f2b16ca322a3d94c95817602d60da12dc7468) wrote
  <phase>/logged/driver.out: "gate started 2026-11-13T01:29:00Z",
  "negative started 2026-11-13T01:41:53Z", "run started
  2026-11-13T01:42:06Z"; "outer stderr 0 bytes" for each step; the gate's
  and the run's own standard error 0 bytes each; "m8p3-p2's target after
  the gate: gone; its names: 26".
- The run's working directory is <phase>/tmp/tmp.4HXaSszNhA, the one new
  name in <phase>/tmp (the driver's before and after listings), so the
  driver prints "<phase>/tmp names unchanged: no".
- Dry run d1 (at docs 1, before ruling K) printed the same FINDING with
  989464 and 456248 (<phase>/dry/d1/logs/gate.log line 12080), the cause
  of ruling K; dry run d2 (at docs 2) read yes on the gate, the negative
  and the run, its scan pre-check failing on its three dry-layout lines
  alone.
- Before the logged run, <phase>/scripts/out (the dry runs' scan stderr
  files) was moved to <phase>/superseded/dry-d1-scripts-out and
  dry-d2-scripts-out.
- Fork N disclosed one regenerated negative written with stale roles by a
  failed generator edit, never run, moved to
  <phase>/tmp/N/m8p3-p3-negative.sh-8dbbb446-stale-roles (planning's
  word on d2). Every superseded script is moved under <phase>/tmp/<letter>,
  none removed.

## Leftovers (left in place for the maintainer's deletion list)

- <phase>/dry/d1 and <phase>/dry/d2 with their build roots.
- <phase>/tmp/G, N, R and S, the forks' files; <phase>/tmp/tmp.4HXaSszNhA,
  the run's working directory.
- <phase>/target, <phase>/gate and <phase>/negative, this phase's build
  roots; the next phase's gate prunes the target.
- <home>/.cache/emend-build/m8p3-p1/keep-host, P1's keep, read by nothing
  since P2.

## After the independent check

The body above is kept as the check read it (sha256 6fd70921c74537942cbd1c7f1f44244108f56a1aca25e5bd0e16d9dac18f3b87).
independent-check-1.md holds the answer verbatim: yes to all three
questions, every figure and cited line it checked matching, no required
line missing, nothing sensitive. Its point on this README, answered from
the primaries:

- The root line for <scratch>: it holds for gate.log and run.log; in
  negative.log, <scratch>/tmp stands for what that log's own root line
  118 says (<build>/tmp, the negative's build root's TMPDIR).
- The names it could not verify (the run's working directory's name,
  dry run d1's gate.log line 12080, d2's pre-check lines, and
  <phase>/scan/pre-check-1.out) come from the files named under
  "Figures from files outside this directory", none committed.
