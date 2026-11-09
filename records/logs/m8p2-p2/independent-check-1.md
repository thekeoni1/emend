# M8 part 2 P2: independent check 1

The answer of a fresh subagent given only these materials, under
<scratch>/check1:
- the spec as committed (1328dd1a...) and as working with ruling C
  (25b65c82...);
- the doc, test and code commits (git show);
- gate.log and run.log;
- the pre-check scan's log;
- the logs README as it stood before the check;
- the draft report.

The answer was extracted from the subagent's transcript by script into
<phase>/check1-answer.txt (sha256 411cf216082a93564e4555a7d46204523c0b95c4add1a686f68c1a6f1b61f764). It appears below verbatim, and
nothing in it is edited. The README's section "After independent check
1" answers each exception.

root: <scratch> stands for <phase>, as in gate.log and run.log
root: <phase> stands for <home>/.cache/emend-build/m8p2-p2, the phase's directory
root: <home> stands for the home directory of the account the runs ran under

**Independent check 1: M8 part 2 P2 (materials under `<check>`)**

I read only the files under `<check>`. I ran sha256sum, wc, grep, sed and awk over them and nothing else. Hashes I computed: spec-0048-committed.md 1328dd1a14ea...a7f; spec-0048-working.md 25b65c82ff78...c8ef; gate.log d0027553...8646 (930651 bytes, 12072 lines); run.log 0646a0c7...81a0 (241897 bytes, 3393 lines); README.md 329258df...8dff8b. The diff hashes, each taken from the first "diff --git" line, are: 94fc5a4 b94067420fb9...2cb6, 8586040 178ffe3326a0...7367, c609bf3 9ada8e855012...a8a.

## 1. Does the diff do what the spec says for P2, and nothing else?

**Answer: yes, with the coverage notes below.**

**The doc commit (commit-94fc5a4.diff).**
- The message names both files as "<path> sha256 <hash>" with no count sentence (lines 7-8).
- The spec hash 1328dd1a... equals my sha256 of spec-0048-committed.md.
- The ROADMAP change moves P1 to DONE and P2 to IN PROGRESS (lines 20-23).
- The spec change adds the P1 rulings and verification. spec-0048-working.md differs from the committed copy only by adding "Planning rulings during P2", ruling C (working lines 558-597).

**The test-only commit (commit-8586040.diff).** It touches only crates/emend/src/package/tests/shift.rs, adding 53 lines and removing none. It holds exactly the three tests of ruling B (1) (working spec lines 520-526):
- a data table cut on the column axis, refused with CUTS (lines 27-35);
- a column-axis member keeping its own `ca="1"` and its own text `X9` (lines 37-56);
- a shared group pushed past the grid by an insertion at or before its master, refused with CUTS on both axes (lines 58-78).

It changes no code. Its message carries the diff-alone sentence (line 15).

**The code commit (commit-c609bf3.diff), against "Threaded comments" (spec lines 238-271).**
- `threaded_cells` (diff lines 87-116) reads each threadedComment relationship's target.
- `threaded_refs` (lines 173-223) checks the root `ThreadedComments` in THREADED_NS and parses each depth-2 `threadedComment` `ref` with `Area::parse(..).first`. A missing `ref` or one that does not parse refuses with THREADED_UNREADABLE. A root of another name or namespace refuses, and so does a part with no root.
- `record_comments` (lines 121-144) replaces the sheet-level check with a per-cell check over additions, removals and replaced, raising THREADED_CELL.
- `record_copies` is not touched. THREADED_COMMENTS_HELD keeps its message, with its cfg narrowed to `all(feature = "comments", feature = "copy")` (lines 152-156). Ruling C (4) accepts that.
- `Package::comments` is not touched.

**Against "Errors" (spec lines 313-326).** Both new messages are in the crate docs' list with their tests (lines 42-45). The "a worksheet has threaded comments" entry now says the copy's write alone raises it (lines 39-41). The `add_comment` docs and the Limits paragraph state the per-cell rule (lines 57-63, 74-79).

**Tests (comments.rs, diff lines 236-543), against the spec's test list (lines 260-271).**
- An addition, a box and a removal beside the threaded cell (lines 328-380). Each asserts the fallback and its `tc=` author byte for byte through a whole comments-part equality, and asserts every part outside comments1.xml and the VML untouched (`assert_untouched`).
- A rich addition (lines 382-413).
- The four calls on C3 refused, plus an area ref (lines 415-468).
- Another root name, another namespace, a child without `ref`, `ref="C"` and a missing target, each refused with the read message (lines 470-489).
- The relationship with no legacy part: the created parts equal those created on a plain sheet, and the threaded part and its relationship are kept (lines 491-543).

**Coverage notes and things beyond the spec:**
- (a) **The box on the threaded cell is never exercised alone.** The refused-box case at lines 423-437 calls `add(p, "C3", ...)` first, and that addition alone already triggers THREADED_CELL. The check at lines 133-140 covers additions, removals and `replaced`, not a box set. So the spec's "a box ... on the threaded cell refused with the new message" depends on a box always being tied to an addition. The existing test name `boxes::a_box_on_a_cell_this_write_adds_no_comment_on_is_refused_at_the_write` (gate.log 10700-10725 area) suggests it is, but nothing in the diff shows which message a lone box on C3 gets.
- (b) **The "nothing recorded" part of the four-calls test does not show what its comment says.** The comment at line 464 reads "Nothing is recorded: the package still writes the next edit". The code then adds on C3 again and asserts `write_to(...).is_err()` (lines 465-467), which shows only that the refusal repeats. Any stronger proof is in `refused_with`, which is not in the diff.
- (c) **The spec's "the copy of such a sheet still refused" has no test in this diff.** The docs point to the existing `copy::a_copy_of_a_sheet_with_threaded_comments_or_a_comment_edit_is_refused` (line 41). I cannot check its fixture from `<check>`.
- (d) **Beyond the spec text, but named in ruling C:** an external target refuses with the read message (lines 99-100), and an XML error is mapped to it (lines 109-112). Errors other than `Error::Xml` from reading the part pass through unmapped (line 111, `e => e`). The spec's "a part that cannot be read refuses with" the read message (spec line 249) could be read as covering those too.
- (e) **The external-target and XML-error refusals have no tests.**
- (f) Diff line 79 is an over-long doc-comment line. Ruling C notes it and lets it stand.

Nothing else is changed: the code commit touches two files (package.rs +128/-15, comments.rs +292/-8).

## 2. Do the logs contain every line the spec and rulings require?

**Answer: yes, with one observation.**

**The four samples over the 39.** Each was run with `--libreoffice localhost/emend-lo:25.2.3-2-deb13u6` (run.log lines 3057, 3123, 3189 and 3253 regions), with "files 39" for each. Each was compared per id on outcome/lo/lo_in with `records/m8p1-<name>-2026-11-09-8816db0.txt`:

| Sample | Comparison line | Summary line | Same | Changed | Missing | New |
|---|---|---|---|---|---|---|
| comment | 3115 | 3118 | 38 | 1 | 0 | 0 |
| richcomment | 3181 | 3184 | 38 | 1 | 0 | 0 |
| commentbox | 3247 | 3250 | 38 | 1 | 0 | 0 |
| copy | 3313 | 3316 | 39 | 0 | 0 | 0 |

**15205770ae5c.**
- comment, richcomment and commentbox go from error/-/ok to ok/ok/ok (lines 3116, 3182 and 3248; the rows at 3087, 3153 and 3219).
- copy is kept at error/-/ok (lines 3285 and 3314).
- Line 3352 reads "the three pairs of the table are met ... : yes".
- The spec sentence is matched at the phase commit, "found: 1" (lines 3039-3041).

**Every output opened.** The evidence is the `--libreoffice` argument and the lo/lo_in columns of the compared triples. **No line prints a per-sample count of LibreOffice outcomes.** "Opened" is shown through the per-id comparison only.

**The comments probe's figure.** Run.log lines 3355-3375: section 1051304 against 984728, cost 66576 against the record's 63440, growth 3136. It is judged against "expected 500 to 3000, accepted up to 5000: accepted" (line 3373), which is the band in spec "Size" (lines 68-69). There are 0 FINDING lines (lines 3375 and 3378).

**The prune of P1's target** (gate.log 181-238).
- The prune set is "target"; 26 names are kept (line 191).
- du gives 6511593713 (lines 229-230).
- `rm -r "${PREV_DIR:?}/${n:?}"` exits 0 (lines 231-232), with the gone and kept checks and their controls (lines 233-238).
- The R90 control of the `rm -r` form is at lines 183-188.

**The crate's tests on both targets.**
- The required figures are at gate.log 11055.
- Results are at 11059-11410: cc 762 at 11167 (host) and 11356 (i686); all 1236 at 11221 and 11410.
- The summary line is at 11436.

**By name.**
- shift: 47 (command 10538, result 10589/10595, check 10598).
- comments with `comments,richtext,rowcol`: 60 (command 10661, result 10725/10731, check 10734). Ruling C (3) accepts this.

**Other required lines.**
- PLANNING_SPEC is 25b65c82... (gate.log line 12), which matches the working copy.
- The working-tree status check is at gate.log 12058 and run.log 3380-3386.
- gate.log ends "gate log checks all as expected: yes"; run.log ends "run log checks all as expected: yes".
- The pre-check scan ends "RESULT: PASS", "structural checks failed: 0" (scan-pre-check.log 6734-6736). It scanned README.md at 329258df... (line 6569), which is the README in `<check>`.

## 3. Does every number and quoted string in draft-report.md and README.md appear in a primary?

**Answer: yes, with the differences below.**

**Verified:**
- The commit ids and subjects (the diffs' headers).
- The per-file line counts: 53 0; 128 15; 292 8.
- The three diff hashes, the two log hashes, and their bytes, lines and last lines.
- The committed spec hash and PLANNING_SPEC.
- ROADMAP e957658c..., but only from the 94fc5a4 message. No ROADMAP copy is in `<check>`, so I could not sha256sum it.
- The gate and run start times: gate.log 354 and run.log 69.
- The test figures. 759/1229 are P1's figures (gate.log 11050-11053 region), and the arithmetic holds: 759 + 4 - 1 = 762 for cc; 1229 + 3 + 5 - 1 = 1236 for all.
- shift 47 at line 10598 and comments 60 at line 10734.
- The 26 kept names (line 191) and the rm form and exit 0 (line 232).
- The four record-pair hash prefixes (run.log 3112, 3178, 3244 and 3310).
- The summaries (3118, 3184, 3250 and 3316), the quote at 3352, and line 3378.
- The probe figures, file sizes and both sha256s (3368-3373).
- 63440 and the band, 66576, 3136.
- gate.log FINDING lines 5721 and 5730 inside the printed script.
- `<work>`'s root line 82.
- The script names (line 2 of each log).
- The scan's RESULT: PASS.

**Differences:**
- README line 55: "6511593713 bytes (line 230)". The du line itself is gate.log 229 ("target: 6511593713 bytes"). Line 230 holds the same figure as the total, "their bytes in all: 6511593713". The number is right; the line is off by one if the du reading is meant.
- **README lines 102-103 are incomplete**: "The other run lines holding the word are the summaries' and sections' counts, each 0". run.log has 41 lines holding "FINDING". Besides the count lines, 33 of them are inside the printed script (run.log 141 to 2699), and line 3377 is the section heading. The gate.log sentence after it names the carried history; the run.log sentence does not.
- draft-report.md lines 29-38 differ from ruling C on two counts:
  - It says ruling C "accepted the stop report's eight points" and labels the seventh "status". Ruling C (7) reads "Planning edits specs/0048 alone before the record commit" (working spec lines 592-593).
  - Points (7) and (8) are statements, not acceptances.

  The substance agrees with ruling C, but these labels are the implementer's, not quotes.
- draft-report.md line 59, "Nothing else changed (run.log 3352)". Line 3352 states the pairs met and kept. "Nothing else changed" is carried by the summaries' same/changed counts (3118, 3184, 3250 and 3316), not by line 3352 alone.

**Unverifiable (README lines 122-132, sourced outside `<check>`):** "gate started 2026-11-09T15:44:25Z", "outer stderr 0 bytes", "m8p2-p1's target after the gate: gone", tmp.Zk89GDSXIA, and the dry-run cmp-equal claim (lines 45-47).

**ASCII:** both files are ASCII, with no em dash.
