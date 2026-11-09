# M8 part 2 P1: independent check 1

The answer of a fresh subagent given only these materials, under
<scratch>/check1:
- the spec as committed (7585b924...) and as working with ruling A
  (e53fcd31...);
- the doc and code commits (git show);
- gate.log and run.log;
- the pre-check scan's log;
- the logs README as it stood before the check;
- the draft report.

The answer was extracted from the subagent's transcript by script into
<phase>/check1-answer.txt (sha256 5911ed3e26f7c8e935f69912a7653b5b1f9a7f7c6c05a020f19d9b16a92dc3ca). It appears below verbatim, and
nothing in it is edited. The README's section "After independent check
1" answers each exception.

root: <scratch> stands for the implementing session's scratch directory, here <phase>
root: <phase> stands for <home>/.cache/emend-build/m8p2-p1, the phase's directory
root: <home> stands for the home directory of the account the runs ran under

**Independent check 1: M8 part 2 P1 (materials under `<check>`)**

I checked the materials themselves: I read the spec, both diffs, the three logs, the README and the draft report, ran sha256sum and wc on every file, and hashed the diff bodies again.

Hashes of the files in `<check>` match what the README and report claim:
- spec-0048-committed.md is 7585b924...a27f48a1f, the same as the doc commit message (doc-commit.diff line 7) and the pin (run.log 1824).
- spec-0048-working.md is e53fcd31...f654, which is PLANNING_SPEC in gate.log 12 and run.log 6.
- gate.log is ba3e318f...e46e2, 929267 bytes, 12084 lines.
- run.log is 194f6dd4...93e07, 233713 bytes, 3284 lines.
- README.md is cc2e6822...e560, the hash the scan read (scan-pre-check.log 6535).
- I took the part of doc-commit.diff and code-commit.diff that starts at the first "diff --git" and hashed it. The results are 9d0cbda8221e... and 6564082e203d..., the same as gate.log 9702 and 9704.

**1. Does the diff do what the spec says, and nothing else?**

Mostly yes. The places it matches the spec:
- **The unshare** (code-commit.diff 114-197, 212-279):
  - `shifted_formula` returns `Unshare` for a shared group whose interval the shift changes (`Moved::Changed if shared`, line 257). It does the same for a master in a deleted row whose `ref` is not wholly inside the deletion (`Moved::Whole if dropping && shared`, line 259).
  - A group moved whole, or left where it is, keeps its bytes.
  - A `ref` wholly inside the deletion (`Gone`) goes with its cells.
  - Array and data table formulas still get CUTS_FORMULA (line 258).
  - The master loses `t`, `ref` and `si` and keeps its text and every other attribute (`unshared_tag`, lines 299-305). Its unescaped text and input cell are held in `unshared` (line 144).
  - Each later member is written with the translated text (lines 159-189). A member that has text of its own keeps it (lines 181-185). The `<v>` elements are not touched.
  - A member met before its master is recorded in `early`. That master's unshare is then refused with MEMBER_BEFORE_MASTER (lines 129-131, 191-193).
  - A member whose master never appears is written unchanged.
- **The translator** (lines 766-1078) follows "The translator":
  - It reads the four reference forms and skips string literals, quoted sheet names, brackets and error literals.
  - It does not treat a token as a reference when the byte before it is a letter, digit, `_`, `.` or `$`, or when the byte after it is `(`, `!` or a name byte.
  - An input column past XFD or a row past 1048576 is not a reference.
  - A moved component below 1 or past the grid gives UNSHARED_OFF_GRID.
  - The output is the input with each moved reference rewritten in place.
- **Errors:** both new messages are listed in the crate docs with their tests (lines 49-58). The cuts message's entry keeps its text and only its test list changes, which is ruling A (5).
- **The tests listed under "### P1"** are present:
  - cut (417): master, member text, `<v>` and `ca` all kept;
  - grown (434);
  - deleted master (449);
  - moved whole, and inside the deletion (490);
  - array and data table refused (509);
  - member before master (524);
  - translation off the grid (541);
  - column axis (566, 593, 611);
  - the translator: 15 tests (651-760). These cover every case in the spec's list.

Coverage gaps:
- **(a) Data tables on the column axis are not tested.** The column-axis refusal test (611-627) uses only an array formula, while P1 asks for "the column axis for each". The other gaps are smaller: on the column axis, no member carries an attribute of its own, and no member has text of its own.
- **(b) I cannot verify the past-the-grid case.** "Errors" says a shared group whose end is pushed past the grid still gets CUTS_FORMULA, and the new doc comment says "refused for all three" (line 233). But the code checks `Moved::Changed(..) if shared => Unshare` before any refusal. The diff does not show which `Moved` variant `interval()` returns for an end pushed past the grid. No new test asserts CUTS for a *shared* group in that case. The docs cite the existing test `a_ref_pushed_past_the_grid_is_a_grow_and_one_moved_past_it_off_the_grid`, and the gate passes it, but its body is not in the materials, so I cannot tell whether it uses a shared formula.

Behaviour the code adds beyond the spec's wording:
- **Bytes past ASCII:** a byte past ASCII counts as a name character (line 850). Ruling A (4) accepts this.
- **Mixed case:** a mixed-case column is written back in upper case (lines 946-950). Ruling A notes this.
- **Leading zeros:** a row with a leading zero (`A01`) is not read as a reference (line 1012). The spec says "one or more digits" and has no such rule. This is not ruled on.
- **Deleted members:** a member in a deleted row that comes before its master also goes into `early` (lines 154-193), so a later unshare of that group is refused even though the member would not be written. This is the literal reading of "records every member's `si` met before a master".
- **Other refusals:** a master marked for unsharing with no numeric `si` is refused with the existing message "a shared formula has no index" (line 127). An element inside `<f>` is refused with "a text element holds an element" (line 334). Neither message is new.
- **emend-copy tests:** the two emend-copy tests now use array formulas (lines 1079-1107), which ruling A (3) accepts.

Nothing else is changed. Numstat (gate.log 9830-9841) and the changed files (gate.log 9710-9720) match the diff.

**2. Do the logs contain every line P1 requires?**

Yes, with one precision point:
- **Four samples over the 39, outputs opened:** run.log sections 1-4. Each run uses `--libreoffice` (run.log 73, 3013). Each is compared per id with `records/m8p1-<name>-2026-11-09-8816db0.txt` on outcome/lo/lo_in (run.log 3008, 3073, 3139, 3204). The summaries are at 3010, 3076, 3141 and 3207.
  - Precision point: the per-id comparison triple leaves out the exit code. The spec's "ok 0 ok ok" names exit 0, but no per-id exit is printed or compared. The run rows (for example 3043) show outcome ok.
- **The two expected changes:** "expected ... id 90a5b01f9541: error/-/ok then, ok/ok/ok now" (run.log 3074). run.log 3243 reads "the two pairs of the table are met: ... (WG): yes". No other id changed, and new and missing are 0 in every sample.
- **Size:**
  - run.log 3250-3264 hold the figures. The verdict is "outside" (3264).
  - The FINDING is at 3265 and the total count at 3269.
- **Prune:**
  - gate.log 181-355.
  - P1 "0 of 1" (229).
  - P2 6336300051 removed, exit 0 (288-291).
  - Close-out 2255072075 removed, exit 0 (347-350).
  - Kept names confirmed (232, 293, 352).
- **The crate's tests:**
  - gate.log 11067 has the expected figures.
  - The host runs are at 11070-11233 and the i686 runs at 11259-11422, with all = 1229 on both.
  - check.sh shows nine PASS lines (10601-10611).
  - shift passed 44 (10715) and translate 15 (10761).
  - 29 new test names, 3 of them renames (10530-10537).
  - emend-copy's renamed tests appear only through check.sh's PASS host and PASS i686. No per-test result is printed.

**3. Does every number and quoted string in draft-report.md and README.md appear in a primary?**

These all check out:
- The hashes in the commit table and the doc commit message.
- 9d0cbda8 and 6564082e.
- The numstat figures.
- 6336300051 and 2255072075, and lines 289 and 348.
- 539/714/576/756/759/821/1229 (line 11067).
- 1229 at 11233 and 11422.
- 1203 (gate.log 11062).
- 26 (gate.log 8668, 8797).
- shift 44 and translate 15.
- The summaries at 3010, 3076, 3141 and 3207.
- The 3243 quote.
- The four record hash prefixes (run.log 3005, 3070, 3136, 3201).
- The FINDING quote at 3265.
- 1061616, 1067556, 357e53fc..., 984728, 993828, 16850afa... (3259-3260).
- 76888, 58244, 1042972, 18644 (3248, 3264), and the arithmetic holds.
- "FINDING lines so far 0" and "FINDING lines printed above: 1".
- gate.log 5834 and 5843, which are inside the printed script.
- The run times 14:32:40Z to 14:49:13Z (run.log 66, 3283; gate.log 12083).
- The last lines of both logs.
- "structural checks failed: 0" and "RESULT: PASS" (scan-pre-check.log 6704, 6706).
- HEAD 2740ce7, and the status lines (run.log 3274-3275).
- The 11 shift tests and 15 translator tests, counted from the diff (26 = 29 - 3 renames).

Differences, or claims I could not find:
- **README line 18: `tmp.X1KdtRYBQx` appears in no primary.** It is not in run.log, gate.log or scan-pre-check.log. run.log's own root line (79) says only "made by mktemp -d under TMPDIR, <scratch>/tmp", so "as run.log's own root line says" is wrong.
- **README line 19 defines `<scratch>` differently from every log.** It calls it the implementing session's scratch directory holding check 1's materials. gate.log 474, run.log 77 and scan-pre-check.log 76 all define `<scratch>` as `<phase>`.
- **README line 45: the gate start time 14:19:59Z is not in a primary.** gate.log 466 says "run at 2026-11-09T14:20:01Z". The 14:19:59Z figure presumably comes from logged.sh, which is not in the materials.
- **These README claims are not in any primary:**
  - line 48: "0 bytes on its outer ... standard error". Only the scripts' own stderr is logged, at gate.log 12080 and in run.log near its end.
  - lines 49-50 and 116: dry run d1 "ran the same three scripts first" and "printed the same figures". The dry logs are not in the materials.
- **README line 54: "lines 181 to 350".** The prune section runs to gate.log 355, and the close-out "kept" line is at 352.
- **README lines 48-49: "Neither log holds an UNEXPECTED line."** gate.log 9597 is a control line that contains the word UNEXPECTED, and 6981 is inside the printed script. No line reports an UNEXPECTED result, so this is a wording point.
- **The draft report contradicts itself.** Line 4 says "STOPPED after the record commit and the scan". Lines 71-72 and 76-78 say the final scan follows the check and HEAD is still 2740ce7, so there is no record commit yet.
- **Draft line 79: "origin/main is e1fc6e4. No tag."** No log states origin/main. The only support is the ROADMAP row in doc-commit.diff, "origin/main 4316630..e1fc6e4". Nothing in the materials mentions a tag.
- **Draft lines 27-33 list six of ruling A's points.** They leave out (7), the prune, and the mixed-case note. Nothing listed is wrong.
