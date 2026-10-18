Independent check 1

Implementer's header (not the checker's): a fresh general-purpose subagent, given only the files under <phase>/check1/, answered the three CLAUDE.md questions. Its inputs:

| File | sha256 |
|---|---|
| README.md | fdbfe4ffed24b3f409f723805cc2f0bd96785cdadedc2f97da1900b40d0e4d94 |
| api-review-m5p6-2026-10-18-ef9da37.md | f207c1e983127ce54e4138d7c9a8ce6d73412e70986c7ac93337127ac5780d00 |
| gate.log | d9f79c5e7218502bc6db12f107edc4be44c942ce269597ff22f6a4ff7cec260e |
| m5p6-open-2026-10-18-ef9da37.md | efaad250713644c49c212fbd34cbabdec1b728a605249f4c82d2b051f3d602e1 |
| negative.log | cc2401e33d1c97dbaaaf3712c1b9b6b7a9c1aae4d66f43177951c7d875dcaafe |
| phase.diff | a8d4fc7cbaa98503f48cfc5174386d0d91e9c23a5cc06d581850d4e901175db6 |
| report-draft.md | 789fd3c0a04479fa3c52318cb7f86a68be81784180445cee0b7863c4f24980f7 |
| run.log | 11cf162e8a35e53f4ea691aef6a6c821c0d86f2a1f9d2d964de4ecf7cb286d20 |
| scan-pre-check.log | 51f917a8c0130cee45bef238ba0a1afd9a3091ba733cba8b7f3d1035faa35718 |
| size-m5p6-2026-10-18-ef9da37.md | 8d7e65ca3b01742345e083ca77c6d59e6ba674b3878a098f54959bd388415611 |
| size.log | 17e29e556b68b66e512bef2224022e1fa01cc068e95023c38c984f79592ee6e5 |
| spec-0037-working.md | d94a5f62a0b23e3eeb4025903a9087e70736cc26ef57c0175c0c260b82fe5244 |
| test.log | deae0b5b73731de6479d8b128259234049ea485c72169d7f76bcffd7ddcae521 |

The answer, verbatim (sha256 90d28f7b07cad46b3219644efd9ea094215964bdb72d81d50472130fff4ff9cc):

No workbook cell content in the inputs. Corpus workbooks appear only as ids, entry names, byte counts and sha256s. The only cell-like strings are fictional generated or fixture data: size.log 3835-3843, the size script's generated package, and test.log's O block, which reads the committed fictional fixture `openpyxl-ops.xlsx`.

# Independent check, M5 part 6 P3 (inputs: `<phase>/check1`)

Before answering I recomputed three things against the inputs:
- **Log hashes:** the sha256 of gate.log, test.log, negative.log, size.log, run.log and README.md equal the hashes in report-draft.md lines 44-49.
- **Spec hash:** spec-0037-working.md hashes to d94a5f62...5244, the PLANNING_SPEC value.
- **Per-file counts:** my count of added and deleted lines per file in phase.diff equals the table at report-draft.md lines 28-38.

## Q1. Does the diff do what the spec says for P3, and nothing else?

**What matches the spec:**
- **Doc commit (c4b2598).** It touches only specs/0037 and ROADMAP.md (gate.log 6609-6610). The 160 lines it adds to the spec (phase.diff 580-746) equal spec-0037-working.md lines 522-681 exactly; I compared them. The ROADMAP edit is planning's P2 row only (phase.diff 29-30; gate.log 6618).
- **Decision 3 A (ab00735).**
  - `read_drawing` no longer refuses a self-closing root. It records `end` at the `/>` and a `close` tag, and the new `DrawingRead::appended` writes `>`, the anchors and the end tag, keeping the bytes after the root (phase.diff 87-177; the call site is at phase.diff 61-86).
  - The catalogue no longer lists "an empty `wsDr` root on an append" (phase.diff 37-51), and the `Package` doc no longer says "or is empty" (phase.diff 52-60).
  - There is one test under images and one under charts (phase.diff 183-259), and gate.log 7407-7421 shows exactly these two names added.
  - The set of `Error::Package` messages is unchanged at 150 (gate.log 6810-6812). That is correct: the message still serves the other refusals.
  - The CHANGELOG gains a "### Fixed" heading and one bullet, nothing else (phase.diff 1-18; gate.log 6795-6798; run.log 6165ff).
- **Ruling U (same commit).** The scale settings sentence is built from `run.openpyxl.tsv`, and the op check now covers the scale openpyxl directories (phase.diff 308-376). The tests are at phase.diff 261-307, with plants P4 and P5 at test.log 5710ff. The committed comparison record is not touched.
- **Item 7 / ruling A (940a1ef).** `lo-check.sh` alone (gate.log 6750-6752): a named container, then `podman rm -f --time 0` after exit 124, with its output and exit kept in the work directory (phase.diff 377-425).
- **Ruling Y (ef9da37).**
  - The commit holds only `openpyxl-ops.py` and its test (gate.log 6753-6756).
  - `cap_exit` writes bytes built before the op through `os.write`, then leaves by `os._exit(137)`.
  - A `MemoryError` that escapes `main` is also caught (phase.diff 494-575).
  - The test adds M2 (a `MemoryError` raised inside the handler) and the control M1 (phase.diff 426-493).
- **No out-of-scope change.** Nothing else changed: no manifest, no lock file, README.md and passthrough-run.sh untouched, the tree equal to the base's outside these 11 files (gate.log 6741-6810). No `pub` item was added (api-review lines 13-20).

**Points against, all minor:**
- **(a) RSS digits not prebuilt.** Ruling Y says the cap's two lines use preallocated bytes. The RSS digits are formatted at handler time, falling back to "-" (phase.diff 154-166 of the ops hunk, around diff lines 527-545). The runner would then read a last line with `peak_rss_kb=-`. Defensible, but not literally "preallocated".
- **(b) Possible extra stdout line.** If a `MemoryError` hits `last_line()` after `print(type(err).__name__)` has succeeded, stdout gets three lines (class, MemoryError, last line). The exit is still 137, so the outcome is unaffected. M2 does not cover this case.
- **(c) M1 is planted.** Ruling Y asks for "a control that the unplanted path still prints its two lines". M1 is itself planted: the op is replaced by `raise MemoryError`. I read it as covering the ordinary handler path; the real unplanted path is N4 in negative.log.
- **(d) String sort.** compare-record.py sorts the memory and timeout values as strings (phase.diff 368-370), so a mixed set would be ordered lexicographically ("3600, 600"). Cosmetic. The committed record is unaffected.

**Verdict Q1: YES**, with the minor notes (a) to (d).

## Q2. Do the logs contain every line the spec and the rulings in force require for P3?

**Present:**
- **Gate log:**
  - Ruling S's defined-helper lines for all six scripts, each with its control (gate.log 94-107), and a planted-script control (108-111).
  - P2's target root alone pruned: "target: 5220494886 bytes" and "target: rm -r exit 0" (gate.log 113-175).
  - The commit messages carry the ruling X sentence; the three code commits read "the diff-alone sentence: yes" (gate.log 6556-6563).
  - check.sh gives nine PASS lines, including hygiene, which is the MSRV proof for item 4 (gate.log 7458-7470).
- **Test log:**
  - The comparison record's test: 34 ok lines and PASS (test.log 4449-4490).
  - The ops test: 79 ok lines with M1 and M2, containers 0 before and 0 after (test.log 4492-4578).
  - The API record: T1, T2 and R2 (test.log 4584-5709), recording "rows 756; {'keep': 756}; ... 0 gone, 0 new" (test.log 5682-5683).
  - Six plants, each failing exactly its expected tests (test.log 5710ff).
- **Negative log:** P2's cases N1 to N7, plus N8 for item 7 with controls N8c and N8k (negative.log 3137-3321). N4 reads cap 137 (negative.log 3205-3215).
- **Size log:**
  - All 29 probes rebuilt at a8bfe71 reproduce that log's sha256 (29 "yes" lines from size.log 8914 on).
  - Every probe judged in its band, the core band 0, 0 findings (size.log 9393-9525).
  - The size record cites everything item 2 asks for: the by-symbol attribution, the images shrink (784e3c0 at 1bb8e02 and 4836d14), the two estimate misses and the read finding of 112600. It also describes the per-kind form for item 6 (size record lines 77 and 290-325).
- **Run log:**
  - Ruling W's seven samples, with charts checked against both its records: 16 lines of "27 of 27: yes" (run.log 4895-6126; the records are named at run.log 67-117).
  - The CHANGELOG check (run.log 6165ff).
  - The open record: 1437 pairs, 0 findings (run.log 8033).
  - The rerun disclosure (run.log 7-8 and 148-152).
- **Scan pre-check:** PASS, 0 hits, 0 structural failures, under PLANNING_SPEC (scan-pre-check.log 4928-4932).

**Missing or weak:**
1. **No defined-helper line for the logged negative script.** Ruling S: "From P3 on, the gate log prints them for every script the phase generates". The gate log's line for m5p6-p3-negative.sh (gate.log 98) reads the script as it was before PZ8. The regenerated script, the one that wrote negative.log, has no defined-helper line in any log; the README's last paragraph discloses this as the session's statement only.
2. **The negative log does not disclose its refused first attempt.** negative.log line 3 reads "RERUN_NOTE: unset", and no log hashes the refused attempt's files. run.log does both for its own refused attempt (run.log 7-8, 148-152). For the negative attempt, the only trace in a log is PZ8 in the printed script comments (negative.log 155-166); the stderr hash 35ef8de8 appears only in README.md.
3. **Ruling A asked for a "planted slow conversion".** N8 instead runs the real conversion under LO_TIMEOUT=1 (negative.log 3298-3305). The effect is shown (return 2, container `emend-lo-...-1` removed, 0 before and 0 after), but the heading "a planted slow conversion" (negative.log 3287) overstates what was planted.
4. **N8k's control container is created, never started.** Ruling A's P1 form says the control counts "a container it starts itself", as N5c does (negative.log 3315-3321).
5. **Stale labels in size.log, cosmetic:**
   - Lines 9409-9411 say "M5 part 4's headroom" and "M5 part 4's budget figure" for figures that come from M5 part 5's record.
   - Line 101 describes PLANNING_SPEC as "planning's copy with its ruling W during P3", but that hash is the copy holding W to Y.
6. **Ruling X's evidence is not in any log.** The empty old-against-new `git diff` and the branch `p3-before-reword` are the session's statement only. The ruling asks only that the report give them, which it does, marked.

**Verdict Q2: PARTLY.** Items 1 and 2 are real gaps against ruling S and against the run log's own disclosure form; items 3 to 6 are minor.

## Q3. Does every number and quoted string in report-draft.md appear in a primary?

**Verified:**
- The commit table (lines 18-21; gate.log 6507-6511) and the per-file counts (lines 28-38; recomputed).
- The log hashes and last lines (lines 44-49) and the end times (lines 52-53, the log tails).
- The spec hashes (lines 4-8; gate.log 12 and 6575; size.log 5).
- Item 2: the test names, and the quote at lines 64-66, which is test.log 4485 verbatim.
- Item 3: the quotes (negative.log 3304-3305).
- Item 4: 29 reproduced; verdicts 6, 21 and 2; growths 0, 556 and 492 with their from and to figures; 436992; "None." (size record lines 118, 150, 158, 166, 320; size.log 9420-9500).
- Item 5: 756 items, 0 and 0 differing rows.
- Item 6: 16 lines; 1437, 1425 and 12, 1426 and 11, 0, 9, 3 (run.log 8033; open record lines 14-19, 41-49).
- Item 7: the quote (gate.log 107), 5220494886, rm exit 0.
- Item 8: the N4 quote (negative.log 3214); 79 ok lines with M1 and M2.
- Findings: 35ef8de8 and 1e03865f (README.md); e1c4f87 and 6a2c1bb (spec line 685); TEST_API_DIR (test.log); change PR and "--" (size.log 202-204); "49 names called" (gate.log 98); dry2 and run-refused-1 (run.log 7-8).

**Unmarked and not found in any primary:**
1. **Line 111:** "gen-negative-1.py (2d4ab0f7...)". The hash appears in no input, and only the next sentence carries the "(session's statement)" mark.
2. **Lines 117-121, finding 2:**
   - "the dry run's 16 staged records": the 16 appears only in README.md, not in a log.
   - "They equal the dry run's records byte for byte": README.md only; no log hashes them.
   - "the same refusal as P2's first run": not in any input. The spec mentions only P2's "attempt 3" (spec line 564).
3. **Lines 136-138, finding 6:** "finding 5's ynnot slipped past it and was caught by the dry run" is unsourced and unmarked. The gap itself can be read from the program printed at gate.log 66-93.

**Differs or overstated:**
4. **Line 6:** "the size log and the scan received it as PLANNING_SPEC". The gate log received it too (gate.log 12). This understates; it is not wrong.
5. **Lines 127-128, finding 4 (marked):** it places the dry negative log in `<phase>/superseded/dry-negative-n4` and `<phase>/dry-logs`. negative.log's PZ7 names `<phase>/dry-logs/negative.log` only (negative.log 149-150). The phrasing should agree with the log.
6. **Line 71:** "Every twenty-nine probe" is wrong as English; it should read "Each of the twenty-nine probes".
7. **Lines 10-12:** the draft says nothing about Q2 items 1 and 2: the logged negative script has no defined-helper line, and negative.log carries no RERUN_NOTE. Finding 1's sub-bullet partly covers item 1, but only as the session's statement.

The statements marked "(session's statement)" or "no log prints it" are marked: lines 3, 24, 54, 112-113, 116, 125, 128, 129.

**Verdict Q3: PARTLY.**

## Overall verdicts

- **Q1: YES** (minor notes (a) to (d))
- **Q2: PARTLY**
- **Q3: PARTLY**

## Fixes

1. Run the gate's `defined.py` check on the regenerated negative script and record it in a log, or state in the report and README that ruling S's line for the logged negative script is the session's statement only.
2. In README.md and the report, hash the refused negative attempt's files from a log, or state plainly that no log hashes them (negative.log's RERUN_NOTE is unset), matching run.log's form.
3. Mark "gen-negative-1.py (2d4ab0f7...)" as the session's statement, or cite where it is printed.
4. In finding 2, mark "16", "byte for byte" and "the same refusal as P2's first run" as the session's statement (or README-only), or cite a primary.
5. Mark finding 6's last clause as the session's statement.
6. Make finding 4's locations match negative.log PZ7 (`<phase>/dry-logs/negative.log`).
7. Optional wording: in the report, say N8 uses a 1 s LO_TIMEOUT on a real conversion rather than a planted slow one, and say N8k's control container is created, not started. Note the stale size.log labels ("M5 part 4's headroom", "with its ruling W") for the next generator.
8. Line 71: "Each of the twenty-nine probes".