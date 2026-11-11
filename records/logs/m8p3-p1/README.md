# M8 part 3 P1 logs

specs/0049-excel-check-and-release.md, "The reruns, the keep, the list
and the stage (P1)", "Phases" and "### P1", with specs/0044's P1 sections
and rulings, and planning's rulings A to C of "Planning rulings during
P1", written in the working spec before the logged run. The gate, the
negative and the run took the working spec at PLANNING_SPEC
014bffadbf7d3f70832e1946f70143712427b97d1afe381e0e7f1916e7b52ed5 (each log
prints it); PLANNING_ROADMAP was not given, the working ROADMAP.md being
the phase commit's blob (ruling A). `<phase>` is the phase's directory,
`<home>/.cache/emend-build/m8p3-p1`.

root: <home> stands for the home directory of the account the runs ran under
root: <phase> stands for <home>/.cache/emend-build/m8p3-p1, the phase's directory
root: <corpus> stands for the private corpus directory the run reads
root: <work> stands for run.log's working directory, as run.log's root line 79 defines it
root: <stage> stands for the list's stage, as run.log's root line 84 defines it
root: <scratch> stands for <phase>, as in the logs; the independent check's materials are under <scratch>/check1

| Role | Commit | Subject |
|---|---|---|
| base | f44debd | M8 part 2 close-out record: the push, the CI run and the scan |
| doc (phase) | 4f8fdfc | M8 part 3 P1: commit planning docs (the part 3 spec, part 2's close-out verification) |

There is no code commit in this phase: the doc commit is the phase
commit, and the code is part 2 P4's (f625b99).

No file name, sheet or table name, note or cell content of any private
workbook is in these logs or in this README. Workbooks are named by id,
the first 12 hex digits of the sha256. No list row is printed. The texts
quoted below are the logs' own lines.

## The logs and the records

| File | Bytes | Lines | sha256 | Last line |
|---|---|---|---|---|
| gate.log | 929497 | 12144 | 7fc19b191b20794a6845c664b36f8d804dc3dc277b993fe8a9c00a06e6d0e60f | gate log checks all as expected: yes |
| negative.log | 438212 | 6353 | daf02a4d97017930ba3fc7c0839157dd09f097185e3d8db3548fcd2c4db7fe4e | negative log checks all as expected: yes |
| run.log | 320622 | 4407 | d48ede4c220cf2c57e9d5ab75895c122616204d146cc64832025c8b114f139a7 | run log checks all as expected: yes |
| scan.log | | | | the final scan, written after the independent check and before the record commit, which holds it |

The records are the 18 files `records/m8p3-<name>-2026-11-11-4f8fdfc.md`
and `.txt`, a pair for each of the eight samples (formula, sheet, remove,
delete-rows, delete-columns, comment, richcomment, commentbox, the usage
text's order) and the open record `m8p3-open-2026-11-11-4f8fdfc`, 2755548
bytes in all. run.log wrote each in <work>/records, hashed it, and
installed it into records/ in section 6: "every record file was
installed, byte for byte: yes" (line 4277).

The scripts are <phase>/scripts/m8p3-p1-gate.sh, -negative.sh, -run.sh
and -scan.sh. Each log prints the script that wrote it. Dry run d1, under
<phase>/dry/d1, ran the same four scripts; the logged driver,
<phase>/logged.sh, checked them cmp-equal before starting.

Each log's own start and end: gate.log line 406 "run at
2026-11-11T05:05:30Z", line 12143 "run ended at 2026-11-11T05:17:41Z";
negative.log line 95 "run at 2026-11-11T05:17:42Z", line 6352 "run ended
at 2026-11-11T05:17:44Z"; run.log line 66 "run at 2026-11-11T05:17:44Z",
line 4406 "run ended at 2026-11-11T08:10:49Z".

## The gate

- The prune (gate.log lines 186 to 304, specs/0048 ruling O): part 2 P4's
  build target, 6475256385 bytes (line 237), and part 2's close-out build
  target, 2310161037 bytes (line 297), each removed by
  `rm -r "${PREV_DIR:?}/${n:?}"`, "target: rm -r exit 0" for each; the
  kept names, 28 (line 196) and 34 (line 245), "every kept name is still
  there: as-expected" for each. The prune form's control ran on a
  directory the gate made (lines 188 to 192).
- The crate's tests: "the figures this run must give: off 544, ch 719, im
  581, ci 761, cc 768, cr 826, all 1244" (line 11126), part 2 P4's
  figures, the phase adding no test; each set passed them with 0 failed
  (from line 11132).

## The negative

The list script refusing a run directory whose summary is missing and a
`--run-plain` directory under another corpus root; the run script's
formula check (specs/0044 ruling H, extracted from run.log's script by
line range) refusing another sample's directory; the stage assertion
refusing a planted byte change on a copy; each with a control. Fictional
fixtures alone.

In dry run d1 the negative read "no" on one control (change NA48), which
presumes the working spec differs from the committed blob; the dry driver
gave PLANNING_SPEC equal to the blob. Ruling A: a driver input, no script
change; the logged driver gives PLANNING_SPEC only when the working spec
differs from the phase commit's blob, as it did here.

## The run

- The eight samples, each over the 39 on the host into
  `<runs>/2026-11-11-4f8fdfc-<name>`, each compared per id on outcome, lo
  and lo_in with part 2 P4's record of the same name: "earlier 39 (same
  39, changed 0, missing 0); new 0" for each (lines 3678 to 4133); "the
  eight runs each read the same as part 2 P4's, no id changed, new or
  missing (ZE): yes".
- The keep: the workspace host tests with EMEND_KEEP_DIR on that command
  alone; the open check's totals line "pairs 2280; lo_in ok 2268 fail 12
  error 0; lo ok 2269 fail 11 error 0; inputs refused by LibreOffice 9;
  the sheet count did not apply 3; counts differ 0; findings 0".
- Against records/m7p2-open-2026-10-27-43471ed: "pairs 2251 lo_in_ok 2239
  lo_ok 2240"; "the difference ...: pairs +29 lo_in_ok +29 lo_ok +29"
  (line 4200), printed and no FINDING (specs/0049 P1). By name: "pairs
  then 2251; now 2280; gone 3; new 32; in both 2248"; no pair in both
  leaves ok.
- The formula check before the list: the --run directory is the formula
  sample's by its run.host.tsv (ruling H).
- The list: "corpus rows 312 (with A 28, with F 20; 273 from
  --run-plain); fixture rows 2268 and input rows 2268 of 2280 kept pairs
  (12 left out: input not accepted)"; the list script's stderr 0 lines;
  "rows: 4848; by source: corpus 312; fixture 2268; input 2268" (line
  4312); by label: - 4536, host 39, and each of the seven --run-plain
  directories 39 (lines 4314 to 4322); the list sha256 9307b03b...,
  1285959 bytes, mode 600 (line 4311).
- The stage `<corpus>/work/excel-check-m8-2026-11-11/`, mode 700: "files
  staged: 4850; under files/: 4848; list rows: 4848" (line 4336); "staged
  files differing from their list rows: 0" (line 4350); the opener's
  sha256 equal to the phase commit's.
- The zip `<corpus>/work/excel-check-m8-2026-11-11.zip`: "sha256
  269a84859d909119794c3f88a3ea52b415954ead2375bb06f45fa130d62af412,
  161525935 bytes, mode 600" (line 4356); "entries 4850; entries whose
  bytes are the staged file's: 4850; stored: 4850; testzip: ok" (line
  4357). Neither the stage nor the zip is committed.
- The six earlier stages and zips (Milestone 2, Milestone 7 parts 1 and
  2) read as at entry by presence, type, bytes and modification time;
  none read, moved or removed (lines 4361 to 4368).

## FINDING lines

run.log prints three, lines 4208 to 4210, each
"FINDING: package.tests.shift.a_shared_formula_before_after_straddling_inside_and_deleted_whole.N:
a pair of records/m7p2-open-2026-10-27-43471ed missing from this keep
(specs/0049 P1); recorded, not fixed", N being 1, 2 and 3. Cause (ruling
B): part 2 P1's code commit 2740ce7 renamed that test function to
`a_shared_formula_before_after_and_deleted_whole`; the old name has three
keys in the m7p2 open record and none in the tree, and the new name's
three pairs are among the 32 new. They are recorded, not fixed, and gate
nothing. Dry run d1 printed the same three. The gate and the negative
print none.

## Figures from files outside this directory

Each is named with its source; none is in a committed log:
- <phase>/logged.sh (sha256
  afe3c373ed6f2c2e907ea2ad4be4f47bf907ce084f14522670304fe44fe369e6)
  wrote <phase>/logged/driver.out:
  - "gate started 2026-11-11T05:05:28Z", "negative started
    2026-11-11T05:17:42Z", "run started 2026-11-11T05:17:44Z";
  - "outer stderr 0 bytes" for each step; the gate's and the run's own
    standard error 0 bytes each;
  - "m8p2-p4's target after the gate: gone; its names: 28" and
    "m8p2-close's target after the gate: gone; its names: 34";
  - "free space 725G" at the start;
  - "files under records/ matching m8p3-: 18".
- The run's working directory is <phase>/tmp/tmp.Ea7F6fTV9v, the one new
  name in <phase>/tmp, from the driver's before and after listings
  (<phase>/logged/phase-tmp-before.txt and -after.txt). The driver
  prints "<phase>/tmp names unchanged: no" for it; the run keeps <work>
  by design (run.log line 79).
- Before the logged run, <phase>/scripts/out (the forks' trial and dry
  d1's scan stderr files, 0 bytes and the pre-check's) was moved to
  <phase>/superseded/pre-logged/scripts-out, so the logged run made its
  own.

## Leftovers (left in place for the maintainer's deletion list)

- <phase>/dry/d1 with its build roots, its keep, its dry corpus, stage
  and zip (6697047345 bytes by du -sb before the logged run).
- <phase>/tmp/G, R, N and S, the forks' temporary files; <phase>/tmp/
  removed-fns.txt, the session's list of test functions removed since
  43471ed (the grep behind ruling B's cause), and progress.md.tmp, an
  empty file of the session's.
- <phase>/tmp/tmp.Ea7F6fTV9v, the run's working directory.
- <phase>/target, <phase>/gate and <phase>/negative, this phase's build
  roots. The next phase's gate prunes the target.
- <phase>/keep-host, which the stage's fixture and input files were
  copied from; it stays until the maintainer's check is recorded.
- The six earlier stages and zips under <corpus>/work, the maintainer's
  by a method they name (specs/0049 "Out of scope").

## After the independent check

The body above is kept as the check read it (sha256 feee35146587a37976186e0a9b26f79529245ca4dc61f5577fe590255c47e1a9).
independent-check-1.md holds the answer verbatim: yes to all three
questions, with every figure it checked matching and its own per-id,
by-name and FINDING-key recomputations equal to the logs'. Its points on
this README, answered from the primaries:

- "logged.sh checked the scripts cmp-equal to d1's": driver.out prints
  no cmp line because the check is a refusal before any step.
  <phase>/logged.sh (sha256 afe3c373ed6f2c2e907ea2ad4be4f47bf907ce084f14522670304fe44fe369e6) line 39 exits 2 unless each of
  the four scripts is cmp-equal to <phase>/dry/d1/scripts' copy; the
  driver went on to the gate (driver.out "gate started").
- "Dry run d1 printed the same three": <phase>/dry/d1/logs/run.log
  (sha256 58483dc6e089ddc8227ec0dcf4b9d1efd799ab79a680e42c1ab0a632e4f3e3ae), lines 4198 to 4200, the three FINDING
  lines with the same keys. It was not given to the check.
- The names and figures under "Figures from files outside this
  directory" and "Leftovers" come from the files those sections name,
  none committed, as the check says.
