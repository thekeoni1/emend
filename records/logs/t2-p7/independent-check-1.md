Independent check 1 of T2 P7.

Checked commit: ae51d0a4a41a0671e4df712dc6c7bfcc4082c9db
HEAD when the check ran: ae51d0a4a41a0671e4df712dc6c7bfcc4082c9db
Checker: a fresh subagent, given only the files below and never this
session's conversation, asked to answer in plain ASCII.

Inputs, as they stood at the checked commit:
  specs/0005-corpus.md                   sha256 63bc947067d96e142ce21149e9b7bfe45d3aaad5417c4881bd174472f67ec85c
  specs/0001-bootstrap.md                sha256 5d7ef86b6939c58aa4fda4b87360fe6e6cc1eb12087ee7ed9078ce47a8626a7e
  records/logs/t2-p7/gate.log            sha256 69e2f0e005454897baeb55d11ade14cbde4568240c1f292671a5ebcb027065a1
  records/logs/t2-p7/scan.log            sha256 d8b42079690508286d96231b1569d3b69c111de226c03873e61b65010667c44a
  the phase diff                         git diff 854dbaa ae51d0a
  the draft report (scratch)             sha256 UNRECOVERABLE, overwritten by the next draft
  The checker could also read positive.log, negative.log and the record.

ASCII substitutions made before committing: none. The answer as returned
holds 17010 bytes and 0 non-ASCII bytes.

---

I have everything I need; no further reads are required.

Note on inputs: the harness placed CLAUDE.md in my context. I did not use it as evidence. Everything below comes from the spec files, the diff, the four logs, the record and the two scripts at HEAD.

# Q1. Does the diff do what the spec says for P7, and nothing else?

Scope of the diff is exactly right. `git diff --stat 854dbaa ae51d0a` is `scripts/corpus-add.py`, `scripts/corpus-coverage.py`, `records/corpus-coverage-2026-09-16.md` and the four logs; `git log --oneline 854dbaa..ae51d0a` is 58063fb (phase), 8f5553c (record), ae51d0a (logs), matching gate.log:321-333 and scan.log:420-434. Nothing in the diff is outside what P7 calls for (specs/0005-corpus.md:299-310).

Follow-up 1, the docstring. scripts/corpus-coverage.py:17-24 now narrows the claim. I checked it against every failure site in the file rather than against the log: the per-row failures at :78, :82, :84, :86, :88, :90, :92, :95, :98, :102, :112, :118, :120 all carry `line {n}`, and the ones with no line are exactly the six the docstring lists plus the catch-all - :52 (HOME), :61, :63, :67, :69, :72, and :161. The narrowed claim is true and its list is complete. Gate check at gate.log:349; exercised at negative.log:457 (`FAIL line 2: corpus file missing`), :464, :466.

Residual defect in the sentence the phase edited: scripts/corpus-coverage.py:17-18 still says "Any failure prints one `FAIL <reason>` line and exits 1". The usage failure at :143-145 prints USAGE to stderr and returns 2, with no FAIL line. The phase rewrote this exact sentence and did not correct that half.

Follow-up 2, the staged order. scripts/corpus-add.py:147-186: copy to `.{fid}.{ext}.corpus-add.tmp`, chmod, append, `os.replace(tmp, dest)`, unlink on `BaseException`, re-raise. The append is still an append. Gate proves the code is present (gate.log:347-348); N1 (negative.log:405-408) and N2 (negative.log:411-416) exercise the happy path and the failed append the spec asks for. Present and correct.

Follow-up 3, the UTF-8 cases. No code change; the validation is at scripts/corpus-add.py:111-120 and covers `pver`, `note` and `os.path.basename(src)`. N4 (negative.log:441-446) exercises producer_version, N5 (negative.log:448-453) the file name. Both refuse with `refused: a text field is not valid UTF-8`, exit 1, `files=0 rows=0 temporaries=0`. Delivered.

The record. records/corpus-coverage-2026-09-16.md is committed, body byte-identical to stdout (positive.log:409), header as ruled (gate.log:354).

Called for but not done or not reported:

- specs/0005-corpus.md:306-309 also asks for the LibreOffice pins in `records/libreoffice-<date>.md` "if T3 P3 has not done so". records/libreoffice-2026-09-15.md exists and its own first lines say it was copied from `tools/libreoffice.md`, so the clause is satisfied - but the report never says so, leaving planning to find out. Reporting gap, not a diff gap.
- specs/0005-corpus.md:310: "The `large` tag is covered (two files over 60 MB uncompressed)." The record shows `tag large 2 libreoffice` (record:67) but no primary in this phase evidences the 60 MB uncompressed half; the only byte counts printed are the compressed 8283997 and 8284163 (record:25, :34). `grep -rn "60 MB\|uncompressed"` over the phase's logs and record hits only positive.log:176 and :455, which are about the baseline's column layout. The spec's parenthetical is unproven here.

Nothing in the diff is unasked for.

# Q2. Do the logs contain every line "Rules for every phase" requires?

What holds:

- HEAD on line 1 of every log: gate.log:1, positive.log:1, negative.log:1, scan.log:1, each with the reading command on line 2.
- The exact script printed: gate.log:12-193, positive.log:8-207, negative.log:7-278, scan.log:6-257; the shared library printed beside it in each: gate.log:195-298, positive.log:209-312, negative.log:280-383, scan.log:259-362.
- `/usr/bin/grep` explicitly (`G=/usr/bin/grep`, gate.log:210) under `LC_ALL=C` at every library call site (gate.log:220, :230, :243), with 15 controls in the same command form run before any use (gate.log:301-316, positive.log:315-330, negative.log:386-401, scan.log:365-380), including an unreadable-file case for all four forms (gate.log:305, :309, :313, :315).
- No control bytes: each log checks its own captures (gate.log:393-394, positive.log:467-468, negative.log:481); scan.log checks the other three (scan.log:490, :499, :508) and declares it cannot scan itself (scan.log:223, :515).
- The forbidden `$(yn [ ... ] && [ ... ])` form appears nowhere. I grepped all four logs for `$(yn ...&&` and `$(is ...&&`: none. Compound checks are inside named functions - `staged_order` gate.log:59, `record_shape` gate.log:82, `body_is_stdout` positive.log:56, `clean_capture` positive.log:191, `refused_with`/`failed_with`/`fail_names_no_line` negative.log:67/:70/:79, `captured_clean` negative.log:266, `files_complete`/`messages_complete`/`pre_clean`/`scan_passed` scan.log:195/:211/:243/:250.
- The one label that prints two numbers (negative.log:270) asserts `captured_clean`, which checks both, and says so at negative.log:264-265. Correct.
- Negative tests never touch the real corpus: `unset EMEND_CORPUS_DIR` at negative.log:38, a per-case `EMEND_CORPUS_DIR=$W` under `mktemp -d` at :126, :139, :170, :185, :202, :213, :230, :244.

What fails:

1. Counting sites outside the library, one of them load-bearing. The library comment printed into all four logs says "no log script in this phase counts any other way" (gate.log:204-205). That is untrue.
   - negative.log:60-62, `state()`: `ls -1A "$1/corpus" 2>/dev/null | LC_ALL=C "$G" -a -c -v -e ... || true` and `rows=$(awk ... | wc -l)`. If the corpus directory were missing or unreadable, `ls` fails, its stderr is discarded, grep counts 0 over an empty stream, and `|| true` swallows the status, so `state` returns `files=0 rows=0 temporaries=0`. N4 (negative.log:191) and N5 (negative.log:208) assert exactly that string, so both would pass vacuously on a read failure. This is precisely the class the phase existed to close, inside an assertion.
   - positive.log:90 (`awk ... "$MANIFEST" | wc -l` -> "manifest rows: 27", printed at :335) and positive.log:91 (`ls -1 "$CORPUS/corpus" | wc -l` -> "corpus files present: 27", :336). Same pipeline hazard; informational, not asserted, but they print numbers into the log.
   - gate.log:125, gate.log:148, positive.log:92, positive.log:114 (`git show ... | wc -c`): informational.
   - scan.log:194 (`git diff ... | wc -l`) is the one non-library site that fails closed, because `files_complete` (scan.log:195) requires `want > 0`.
2. An unrun attempt reported as a result. negative.log:165 and :433 state attempt 2's outcome ("the staging copy into the same directory fails first, with exit 2 and PermissionError"). The script never runs it: the only `chmod` calls in negative.log are :84 (700 on the scratch dirs) and :138 (400 on the manifest, which is N2). This breaks the rule this phase itself committed at 854dbaa, now specs/0001-bootstrap.md:124-126: "Nothing is called measured unless the log prints the measurement; a figure in a script's prose or comments is not one."
3. Same class, lesser. negative.log:411 says "the append fails after the copy is staged", but no log ever observes the temporary existing - `grep -n 'corpus-add.tmp' negative.log` returns nothing outside the script text. That the copy was staged is inferred from the code, not measured. N2 does measure that nothing survives.
4. Negative tests run in the main working tree. negative.log:33 is `cd "$(git rev-parse --show-toplevel)"` and the cases invoke `./"$ADD"` and `./"$COV"` from the repo checkout. specs/0001-bootstrap.md "How this task runs" requires every negative test to run in a scratch copy, a git worktree or a clone outside the repo. Mitigation: all writes go to `mktemp -d` and negative.log:483 asserts the tree is still clean. Planning should rule whether this is the settled practice from T2 P1; nothing in the evidence I was given says it is.
5. Commit ids. gate.log resolves the base from git by a printed command (gate.log:45, :94-95) and asserts `HEAD == C` (gate.log:51); positive.log:41 and negative.log:45 assert it too. scan.log takes both `BASE` and `C` as typed arguments, resolves neither, and never asserts `HEAD == C`; it only prints `git rev-parse HEAD` separately (scan.log:108-109).
6. gate.log's `staged_order`, `copy_is_staged`, `docstring_narrowed` and `record_shape` (gate.log:59-87) use direct `grep -q` with no positive control in the same command form. They fail closed (a grep error returns non-zero, so the check prints "no"), so the exposure is a false FAIL, not a false PASS. Listed for completeness against "every scan form has a positive control".
7. ae51d0a's own commit message and the four log files are covered by no scan in this phase: scan.log ran at 8f5553c and scanned two messages (scan.log:485). Previously accepted practice, but planning's own scan has to cover them.

# Q3. Does every number, quoted string and citation in the report appear in a primary, at the line cited?

Verified correct, at the exact line: report:39 ":409" = positive.log:409; report:42 ":443" and ":447" = positive.log:443, :447; report:44 ":452" and ":453" = positive.log:452, :453; report:42 range 435-445 covers the four verdicts at :435, :437, :439, :441 and the path verdict at :445 (the count lines themselves begin at :434, just outside the range, which is harmless); report:28 "corpus-add.py:111-120" is exactly the field-validation loop; report:38 "27 files, 23 tags" matches record:17 and the 23 `tag` lines at record:45-67; report:50-54's gaps match positive.log:399-400 and record:68-69; report:71 "the rule committed at 854dbaa" matches specs/0001-bootstrap.md:124-126 as added by that commit; report:8-13's commit descriptions match `git show --stat` for all four (854dbaa is exactly 3 files: ROADMAP.md, specs/0001, specs/0006, and sets P7 to "READY (after T3 P4)"). "No push" is right: main is ahead 39 of origin/main.

Wrong citation:

- report:33-34: "exit 0, no stderr (positive.log:340-341)". positive.log:340 is blank and :341 is the heading `## the run`. The evidence is at positive.log:343-346. The citation points at a heading, not a result.

Claims a primary contradicts:

- report:42, "each counted with a control proving the count can report a hit (:443, :447)". False as written. The loop at positive.log:135-144 counts four things - stems, notes, producer_version values, app strings - and only the stem count gets a hit control (positive.log:146-150, a salted copy of the record with one stem appended). The notes, producer_version and app-string counts have no control at all. `:447` is the separate corpus-path control (positive.log:154-157), not a control for those three. So three of the five zero counts rest on a form never shown able to report a hit against those pattern files. The same overclaim is committed in a primary: ae51d0a's commit message says "each counted with a control proving the count can report a hit".
- report:77-84, "Every counting site in all four logs goes through one shared library". False - see Q2 item 1. Also contradicted by the library comment printed in every log (gate.log:204-205).
- report:68-69 restates unrun attempt 2 as fact; see Q2 item 2.

Unsupported by any primary I was given:

- report:62-63 cites "ruled on 2026-09-16 that corpus-add.py is not to be changed to make it testable"; positive.log:10 cites "planning's ruling of 2026-09-16, option (a)"; ae51d0a's message repeats it. specs/0005-corpus.md carries rulings dated 2026-09-15 only (specs/0005-corpus.md:160, :213). Planning must confirm it made those rulings; as evidence they stand on nothing.
- report:98-100, "the fourth too-narrow extraction this session" and the three earlier ones: session output, in no primary. Unverifiable, not contradicted.

On your vacuity question (a): the privacy counts are not vacuous in the ways you named. The pattern files are non-empty - positive.log:433 records 18 stems, 27 notes, 2 versions, 2 app strings. The columns are right: `$11`, `$12`, `$6`, `$7` (positive.log:130-133) against HEADER `id, sha256, bytes, ext, producer, producer_version, app, source, tags, added, original_name, note` (scripts/corpus-coverage.py:39-40). `count_from_file -F` cannot silently miss a literal, and a read failure prints "error", not 0, with a control for that at positive.log:323. A wrong column would have produced hits, not zeros. The genuine weakness is the missing hit control for three of the four, above. Separately, the path control (positive.log:154) writes the real corpus path, which contains both `emend-testing` and `/home/`, so it proves only that some alternative of `emend-testing|/home/|/Users/|/root/` can match, not all four.

On (b): the branch description is accurate in outline but worded favourably and incomplete. "Present and unexercised" is honest, negative.log:436 says "Recorded as unproven, not as passing", and N6 really does exercise the catching failure (negative.log:457). But the report presents both attempts as if both were run; only attempt 1 was (negative.log:437-439). Attempt 2's exit code and exception are asserted, never measured. Neither the report nor the code comment (scripts/corpus-add.py:171-179) notes that the comment's justification - "any permission that blocks os.replace also blocks the staging copy in the same directory" - is itself an untested claim about the one case nobody ran.

On (c): the library's controls do prove what is claimed for the four library functions - each of `count_matches`, `count_from_file`, `count_bytes`, `count_lines` is shown returning the right number on a match, 0 on a miss, and `error` on an unreadable file (gate.log:301-316). What they do not prove is the report's wider sentence, since several counting sites never call them.

On the two admitted errors: admission 1 is accurate and supported - the fix and its reasoning are at positive.log:173-174 and the control verdict at positive.log:453. Admission 2 is session output and cannot be checked, but is self-consistent. Both admissions are incomplete in the same direction: three further errors of the same kind (the "each counted with a control" overclaim, the "every counting site" overclaim with a load-bearing vacuous site in `state()`, and attempt 2 reported as measured) are unadmitted, and the second of those is the third time this exact class has been declared closed after repairing most but not all instances.

GAPS

1. negative.log:60-62 `state()` counts through `ls | grep -c` and `awk | wc -l` with `2>/dev/null` and `|| true`, so N4 (negative.log:191) and N5 (negative.log:208) would pass vacuously if the scratch corpus could not be read - a counting site that reports zero on a read failure, inside an assertion, in the phase whose purpose was to close that class.
2. report:77-84 and the library comment in all four logs (gate.log:204-205) claim every counting site uses the shared library. It is false: positive.log:90, :91, negative.log:60-62, gate.log:125, :148, positive.log:92, :114, scan.log:194.
3. report:42 and ae51d0a's commit message claim "each counted with a control proving the count can report a hit". Only the stem count and the path count have controls; notes, producer_version and app strings have none (positive.log:135-150). One of the two overclaims is already committed.
4. negative.log:165/:433 and report:68-69 state attempt 2's result without running it, breaking specs/0001-bootstrap.md:124-126 as committed by this phase at 854dbaa. Lesser instance: negative.log:411's "after the copy is staged" is never observed.
5. report:33-34 cites positive.log:340-341, a blank line and a heading. The result is at positive.log:343-346.
6. report:62-63, positive.log:10 and ae51d0a's message cite planning rulings of 2026-09-16 that appear in no primary; specs/0005-corpus.md carries only 2026-09-15 rulings.
7. Negative tests run from the main working tree (negative.log:33), not a worktree or clone outside the repo, contrary to specs/0001-bootstrap.md "How this task runs". Writes are confined to scratch and the tree is asserted clean (negative.log:483); planning should rule.
8. specs/0005-corpus.md:310's "two files over 60 MB uncompressed" for `large` is unevidenced in this phase's primaries, and the P7 LibreOffice-pins clause (specs/0005-corpus.md:306-309) is unmentioned in the report though already satisfied by records/libreoffice-2026-09-15.md.
9. scripts/corpus-coverage.py:17-18 still says every failure prints a FAIL line and exits 1; the usage failure at :143-145 exits 2 to stderr with no FAIL line. The phase rewrote that sentence.
10. scan.log takes BASE and C as typed arguments, resolves neither from git, and never asserts HEAD == C (contrast gate.log:45, :51). gate.log:59-87's four `grep -q` forms have no positive control (they fail closed). ae51d0a's message and the four logs are scanned by nothing in this phase.
