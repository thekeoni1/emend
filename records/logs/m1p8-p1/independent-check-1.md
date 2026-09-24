# M1 part 8 P1 independent check 1

Implementer's note: the checker's answer follows as returned, with one change: the repository's absolute path, on the line naming the read-only git commands, is written as <repo>, as the logs write it. The final scan's first attempt found that path (form f_home, count 1).

## Scope and method

The checker was given only what is in `scratchpad/p1/check1/`: `spec-0015.md` (sha256 f5ca180a...af214, the working-tree spec with rulings 1 to 8), `phase.diff`, `gate.log`, `test.log`, `negative.log`, `run.log`, `scan-pre-check.log`, the record pair and `report-draft.md`. Commit subjects, messages and file contents at the listed commits were confirmed with read-only `git -C <repo> show/log/grep/diff/cat-file`. Nothing was run or modified.

Hashes computed by the checker over the supplied files:

- gate.log 11fdfd59...6a483
- test.log a52dcecb...f4f6f4b
- negative.log 6c5f3a3a...346e8
- run.log deeab3a0...dee72c
- record .md 29c469a2...bb713f9, 36753 bytes
- record .txt 6d37834e...c1a042b, 32082 bytes

All of these match the report and run.log line 1168.

## Question 1: does the diff do what the spec says, and nothing else?

### What the spec asks, and where it is met

- **Doc commits.** e566de1 carries ROADMAP.md, specs/0014 and specs/0015 at planning's hashes (gate.log 1164-1211). f25dd1b carries specs/0015 alone at ce0232d3... (gate.log 1182-1186). HEAD carries the same blobs (gate.log 1187-1201). The working-tree spec is f5ca180a... and is uncommitted (gate.log 1538-1543), as ruling 8's tree-status form requires. `git show e2b8a81:specs/0015-open-checks.md | diff - spec-0015.md` shows that only rulings 7 and 8 are added.
- **The keep in `testing.rs`** (diff 254-459).
  - `keep(input, output)` writes `<test>.<n>.in.xlsx` and `<test>.<n>.out.xlsx`, with `::` replaced by `.` and `n` counted per thread from 1 (ruling 1).
  - It refuses before writing either file when a name is already there.
  - It does nothing when no directory is named.
  - It reads a thread-local override first and the variable only when there is no override (ruling 3).
  - It panics when the named directory is not a directory (ruling 3).
  - The module is `#[cfg(test)]` (negative.log 810-813).
- **`open_fixture` and `written`** (diff 62-253, ruling 2).
  - `open_fixture` records the bytes.
  - `written` takes the fixture. With a keep on, it panics if no fixture was recorded or if the recorded entry names and CRC-32s differ from the package's.
  - The only call site changed is the no-chain round trip (diff hunk `@@ -1992,7 +2031,7 @@`).
- **The cells test.** It calls `keep(&input, &out)` after its one write (diff 48-61).
- **The tool keep** (diff 1250-1576, ruling 4).
  - It has its own variable, override, counter, refusal and the `emend-copy.` prefix.
  - It keeps only when the run exits 0 and both paths are files afterwards.
- **`scripts/lo-check.sh`** (diff 656-781).
  - It is a move from the runner. The only rewrite is `$G` becoming `$LO_GREP`, twice (gate.log 1331-1341).
  - The runner sources it once (gate.log 1338).
- **`scripts/edit-open-check.sh`** (diff 460-655).
  - It has the options the spec names.
  - It writes seven-column rows, totals, the refused-inputs list and the refused-on-accepted-input list.
  - Reasons stay in a scratch directory. Only that directory's path is printed.
  - Exit codes: 0, 1 when an output is refused on an accepted input, 2 on any error. An empty keep is refused.
- **Ruling 7 (d9315b8).** Nineteen made-up types became six real-type constants (gate.log 1343-1353). No part was added.
- **Core code.** No file changed outside the three crate test files, and no lock file or manifest changed (gate.log 1324-1327).
- **Ruling 8.** No code change, as the ruling says.

### Findings

**F1 (note).** Ruling 7 says "no test assertion changes, since none asserts on the type strings". d9315b8 does change an assertion's expected value: `types_expected` in `the_recipe_drops_the_chain_and_leaves_every_other_entry_alone` (diff hunk `@@ -879,10 +981,10 @@`, three attributes). Planning's own count of 19 includes those three: the builders account for 3+4+4+5 = 16, and this expected string for 3 more. Ruling 8 repeats the count, and d9315b8's message discloses the change. So the change is within the ruling's intent, but the ruling's sentence is literally false and the report does not mention it.

**F2 (note).** The diff goes a little beyond the spec, all of it harmless and disclosed in commit messages:

- e2b8a81 renames the scratch rows file from `.tsv` to `.txt`.
- The record has a fourth list, "Checks that did not run (error)".
- The script refuses half a pair, a file that is not part of a pair, and a record that already exists.
- The script prints one stdout line per pair and the path of the reasons directory.
- The crate keep tests `written_with_a_keep_refuses_*` and the "directory not there" tests implement rulings 2 and 3.

**F3 (should-fix: record wording and report disclosure).** The record's heading "Inputs LibreOffice refuses (allowed: a fixture that is not a workbook)" is wrong for all seven crate entries under it:

- run.log 1243-1250 shows each of the seven crate inputs holds `xl/workbook.xml`.
- For five of them (rows 36, 37, 102, 105 and 106 of the .txt), the reason at run.log 1170-1192 is "the ods holds 1 table elements and the workbook 0 sheet elements". LibreOffice converted those files. The check's `<sheet(?=[\s/>])` count found no sheet: the workbook is either prefixed (`a_calc_pr_in_a_prefixed_workbook_takes_the_prefix`) or its root has no children.
- For those seven inputs the output check tells us nothing, including one row (109) whose output passes on a refused input.

Ruling 7 names these seven as allowed, so this does not block. Two things still need attention:

- Ruling 8 asks the record to name the refused tool inputs "as allowed zip-level fixtures". The heading does not say zip-level, and it groups the crate's workbook-shaped fixtures under "not a workbook".
- The report should state that seven crate edit tests are, in effect, not open-checked, and why.

**F4 (note).** Mutation K3 (removing the tool keep's exit-0 condition) does not bite (test.log 2442, table line K3 `mode=measure`). No test covers a run that exits non-zero and leaves an output file. The tool keep is a separate copy of the crate keep, and no mutation targets its own refusal, counter or override; K1, K2, K4 and K5 all mutate crate files. The report states K3 correctly.

**F5 (note).** In the "unset, nothing is written" tests, the directory being watched is never a keep target. The `listed(&dir)` assertion in `with_no_keep_directory_nothing_is_written` and the `dir.names()` assertion in `with_no_tool_keep_directory_nothing_is_kept` would pass even if the keep wrote somewhere else. The effective assertion is `keep_dir() == None` under an override set to None. The case with no override and the variable unset is covered only by test.log's unset run passing, and no log checks that that run left no stray files.

## Question 2: do the logs contain every line the spec requires?

| Requirement | Where | Control |
|---|---|---|
| The keep writes the pair named after the test | test.log case map (crate, written, tool), each passed on both targets; source lines 2250-2277 | K4 and K5 bite (test.log mutation table) |
| A second write of the name is refused | the same case map, crate and tool | K1 bites |
| Unset, nothing is written | case map lines for the crate, `written` and the tool; test.log line 49 "unset for the whole log" | none bites directly (F5) |
| Only the override is set; no test touches the variable | test.log 2278-2285 | control finds a `set_var` line |
| Ruling 2 refusals | case map; K2 bites on both tests | mutation-free worktree control |
| Negative log: truncated output gives `lo` fail and exit 1 | negative.log 958 and following | `fixture.sound` ok/ok row |
| Negative log: unopenable input is listed and exit is 0 | negative.log 1014 and following | sound row |
| Negative log: missing image gives `error` and exit 2 | negative.log 1070 and following | "the pinned image is there, through the same command" |
| Negative log: empty directory is refused | negative.log 1082-1086 | S1's reasons directory is found |
| Negative log: T3 P3 cases rerun | negative.log 1103-1189: C1-C9, C11, C12 on both runners, 5 of 5 runs equal, 7 of 7 reason files byte-identical | "tells run-lo from run-extra" |
| Run log: both targets run with the variable set, into two keeps | run.log 854-948 | |
| Run log: same counts and names as without the variable | run.log 899-900 and 946-947 (523 passed, 2 ignored) | doubled-name control, 949 |
| Run log: the two keep directories compared | run.log 952-959 (298 = 298, byte-identical) | cmp control |
| Run log: the script run on the host keep | run.log 1010-1165, exit 0 | |
| Run log: record stem `m1p8-open-<utc-date>-<shortsha>` | run.log 57; the script derives it with `date -u` and `rev-parse --short=7` | |
| Run log: no output refused on an accepted input | run.log 1165, 1217 and 1220 | none in run.log itself; negative S1 is the functional control |
| Run log: every kept test named in the record | run.log 1197-1201, 149 rows compared one by one | one-digit-changed hash control |
| Run log: kept pairs printed beside the source count | run.log 998: 149 pairs, 114 distinct tests, 114 from source; per-file set equality at 973, 977 and 997 | reach.py controls at 1004 and 1007 |
| Ruling 8's assertions | run.log 1223-1242, sets of 6 equal, non-empty | zip-entry read control |
| Phases rule: rerun lines name only hashes printed by the command beneath them | gate.log 61 names no hash; the command under it prints 55d91c... (gate.log 63) | |
| Phases rule: the record check compares content and each row | run.log 1195-1213 | .txt cut to one row |
| Phases rule: root lines say "sourced" where a library is sourced | gate.log 65, test.log 55 and run.log 44 comply; negative.log 44 does not (F6) | |
| Each log ends "all as expected" | gate.log 1547, test.log 2553, negative.log 1198, run.log 1262 | |

**F6 (should-fix; planning may rate it blocking, since it is one of the three required Phases rules).** negative.log line 44 says "the phase runner, scripts/lo-check.sh and scripts/edit-open-check.sh are run from it". scripts/lo-check.sh is never run. It is sourced by the runner and by the script, and the gate asserts "scripts/lo-check.sh is not executable: it is sourced, never run" (gate.log 1248). run.log 44 words the same fact correctly ("which sources scripts/lo-check.sh").

**F7 (note).** P1 asks for "the same test counts as the gate's run without it". run.log compares against test.log's unset run (run.log 843-853), because gate.log prints only PASS lines (gate.log 1506-1514). The Design section places that comparison in the test log. The reading is reasonable, but the spec's wording is not literally met.

**F8 (note).** C10 and M1-M4 of T3 P3 are not rerun. negative.log 90-91 gives the reason: they do not run `lo_check`.

**F9 (note).** The run writes the record to `<scratch>/p1/out/records`. No log shows it being moved into `records/`. The supplied pair hashes to run.log 1168's values, so the copy is the run's output.

## Question 3: does every number and quoted string in report-draft.md appear in a primary?

These report items match their primaries:

- **Commits:** the eight commits and their roles and subjects (gate.log 1097-1115, git).
- **"Eight new tests" / "Five new tests":** test.log 2245-2246.
- **The amend disclosure.**
  - 27afb8c exists as a commit object with parent f25dd1b.
  - `git diff 27afb8c 67ad8c1` is empty.
  - Its message says "Seven new tests". Confirmed from git.
- **gate.log:**
  - its hash;
  - nine PASS lines (gate.log 1506-1517);
  - 55d91c... (gate.log 63);
  - `<home>/.cache/emend-build/` (gate.log 68).
- **test.log:**
  - 523 passed, 0 failed, 2 ignored on both targets (1537, 2108);
  - 13 tests added and 510 before (2139-2143);
  - the K-mutation results.
- **negative.log:** S1-S6, and C1-C9, C11, C12 with equal columns and reason files.
- **run.log:**
  - 149 pairs, 114 distinct tests, and a source count of 114 split as package 85, cells 1 and tool 28 (963-998);
  - exit 0;
  - rows compared;
  - ruling 8's 6 = 6.
- **Record:**
  - both hashes;
  - 149; lo_in 136/13/0; lo 137/12/0;
  - inputs refused 13, split as crate 7 and tool 6 (run.log 1216 and 1219);
  - 0 outputs refused.
- **Spec hash:** f5ca180a... (gate.log 1541, run.log 841).

**F10 (should-fix).** "/tmp is an 11G tmpfs, about 9G of it held by other sessions' scratch directories" does not appear in any primary. None of the logs gives a size figure for /tmp.

**F11 (note).**

- The report says attempt 1 is "hashed on its rerun line". The rerun line (gate.log 61) says "hashed below"; the hash is on the command output at gate.log 63.
- The report gives "/tmp ... was full" as the cause without the log's own qualifier, "measured by rerunning the step's build by hand, not by a log".
- gate-attempt-1.log was not supplied, so the checker could not recompute its hash (55d91c...) or its size (99922 bytes).

**F12 (should-fix, omission).** The report does not mention:

- F1: an assertion's expected string changed, contrary to ruling 7's sentence.
- F3: the seven crate "refused" inputs are workbook packages, and five of them fail only the sheet count.

## Verdict

Nothing here blocks the code. The diff implements P1 and rulings 1 to 8 and nothing outside them. The logs hold every required line except that negative.log's root line says the sourced library scripts/lo-check.sh is "run" (F6). Planning should rule on F6 and on the record heading (F3), and the report should drop or source its /tmp figures (F10) and disclose F1 and F3.
