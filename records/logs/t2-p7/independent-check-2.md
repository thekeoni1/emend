Independent check 2 of T2 P7.

Checked commit: 8f4388242d866b93a6da90dcd876a622561d0a2f
HEAD when the check ran: 8f4388242d866b93a6da90dcd876a622561d0a2f
Checker: a fresh subagent, given only the files below and never this
session's conversation, asked to answer in plain ASCII.

Inputs, as they stood at the checked commit:
  specs/0005-corpus.md                   sha256 63bc947067d96e142ce21149e9b7bfe45d3aaad5417c4881bd174472f67ec85c
  specs/0001-bootstrap.md                sha256 5d7ef86b6939c58aa4fda4b87360fe6e6cc1eb12087ee7ed9078ce47a8626a7e
  records/logs/t2-p7/gate.log            sha256 366e4f9b66c45e5d3e0cc61eb8c6871d1eb50e01674e3f0810729cf9ed92265b
  records/logs/t2-p7/scan.log            sha256 ed71f3352aca0dead3610b533ab116812b7dd54d88b3eac22db20d75c5b64a94
  the phase diff                         git diff 854dbaa 8f43882
  the draft report (scratch)             sha256 UNRECOVERABLE, overwritten by the next draft
  The checker could also read positive.log, negative.log and the record.

ASCII substitutions made before committing: none. The answer as returned
holds 18759 bytes and 0 non-ASCII bytes.

---

I have everything I need; no further tool calls. Here is the check.

# Independent check 2, T2 P7

Note: the harness placed CLAUDE.md in my context. I say so and do not use it as evidence. Everything below is from the spec, the diff and messages, the four logs, the record, the two scripts at HEAD, and the draft report.

## Q1. Does the diff do what the spec says for P7, and nothing else?

The three P1 follow-ups (specs/0005-corpus.md:244-251) and the record are present:

- Follow-up 1, the docstring. scripts/corpus-coverage.py:19-24 now says "A failure found while checking a manifest row names that row's line number; the failures that are not about a row do not..." and lists them. Present as code (gate.log:378), exercised in negative.log:523-526.
- Follow-up 2, the staged order. scripts/corpus-add.py:147 (`tmp = os.path.join(dest_dir, f".{fid}.{ext}.corpus-add.tmp")`), copy, append, `os.replace(tmp, dest)`, `os.unlink(tmp)` under `except BaseException`. Present (gate.log:376-377), and the logged failed-append case the spec asks for is N2 (negative.log:461-469).
- Follow-up 3, UTF-8 cases for file name and producer_version. No code needed; the validation at scripts/corpus-add.py:111-120 covers `pver`, `note` and `os.path.basename(src)`. Cases N4 and N5 at negative.log:500-512.
- The record: records/corpus-coverage-2026-09-16.md, derived from the run in positive.log:366-419.

Nothing in the diff is outside the spec's seven paths (gate.log:368-369, scan.log:461-467): the two scripts, the record, the four logs.

Two things the spec calls for that are missing or unevidenced:

1. **The second docstring change (0c3c6a3) is evidenced by nothing.** It added "A bad argument prints the usage line to stderr and exits 2, with no FAIL line" (scripts/corpus-coverage.py:17-18). Its commit message says: "Measured: `corpus-coverage.py --bogus` exits 2, prints nothing on stdout, and prints `usage: corpus-coverage.py [--verify]` on stderr." No log contains that run. A grep of all four logs for "bogus" returns only the two echoes of the commit subject (gate.log:335, 359; scan.log:455). negative.log was rerun *after* 0c3c6a3 and still has no usage-failure case: N7 covers only a header mismatch and a missing manifest (negative.log:523, 525). gate.log's `docstring_narrowed` greps only the two per-row phrases (gate.log:78-79), not the usage sentence. So a committed message calls something measured that no log prints, which is the rule at specs/0001-bootstrap.md:124-126 and is exactly the class of defect this round was convened to repair. The report does not admit it.
2. **specs/0005-corpus.md:309-310** ("The `large` tag is covered (two files over 60 MB uncompressed)") is unproven. The report admits this at lines 61-64. Accurate: the record shows only `tag large 2 libreoffice` and 8283997 / 8284163 byte file sizes.

Also in the diff but not called for by the spec, and worth planning's eye: the record's header attests `Commit: 58063fbf...` and `Script: sha256 2b8be584...`. That sha is corpus-coverage.py at 58063fb; the only log that now derives the record ran the script at 0c3c6a3, sha ff02af14 (positive.log:356-357, 428). No current primary shows 2b8be584 producing that stdout. The superseded positive.log did; the rerun silently re-attests a body against a different blob. Not mentioned anywhere.

## Q2. Do the logs contain every line "Rules for every phase" requires?

Satisfied:

- HEAD on line 1 of each log: gate.log:1, positive.log:1, negative.log:1, scan.log:1, all with the resolving command on line 2.
- The exact script printed: gate.log:13-198, positive.log:9-217, negative.log:8-320, scan.log:8-264, plus the shared library printed into each (gate.log:200-312 and the same in the other three).
- /usr/bin/grep with LC_ALL=C: `G=/usr/bin/grep` (gate.log:224) and every form is `LC_ALL=C "$G" ...` (gate.log:234, 244; scan.log:79). Positive controls in the same command form: the 15 library controls (gate.log:315-329) and scan.log's 23 form controls (scan.log:407-429), including `f_p6` on a bare shape expecting 0 and `f_pre` on the same expecting 1 (scan.log:415-416).
- No control bytes: gate.log:422-423, positive.log:492-494, negative.log:540, and scan.log's `f_ctrl` count=0 on every diff file, message and sibling log.
- Compound checks inside one named function: `staged_order`, `record_shape`, `docstring_narrowed` (gate.log:60-88); `body_is_stdout`, `clean_capture`, `differs` (positive.log:57, 201, 183); `refused_with`, `failed_with`, `captured_clean` (negative.log:93, 97, 308). I found no `$(yn [ ... ] && [ ... ])` or `$(is [ ... ] && ...)` form anywhere. The "label prints two numbers, assertion checks one" defect is specifically avoided at negative.log:308-313 and positive.log:199-207.
- Commit ids read from git: gate.log:46 resolves the doc commit by subject, and gate.log:5-6 prints the command; positive.log:42 and negative.log:46 assert HEAD equals the argument; scan.log:110-113 now does too (scan.log:3).

Not satisfied:

1. **A counting site inside an assertion that is outside the library and does not read an exit status: negative.log:83.**
   `rows=$(awk -F'\t' 'NR > 1 && NF == 12' "$dir/manifest.tsv" 2>/dev/null | wc -l)`, guarded only by a separate `[ -r "$dir/manifest.tsv" ]` on the next line (negative.log:84), with `rows=0` in the `else` branch when `[ -f ]` is false (negative.log:85-86). `state()` feeds the assertions at negative.log:158, 173, 207, 219, 233, 250, 293. This is a permission test, not the pipeline's status; the pipeline's exit is discarded despite `pipefail`. Concretely: a `manifest.tsv` that is a dangling symlink, or a directory, makes `[ -f ]` false and yields the clean string `rows=0`, and N4/N5 assert exactly `files=0 rows=0 temporaries=0`. There is no positive control for this form.
   This also makes the shared library's own header false again: "Every counting site that feeds an ASSERTION goes through these functions" (negative.log:335-337, identical in all four logs) with an exceptions list (negative.log:337-341) that names only positive.log's manifest-row and corpus-file lines, `git show ... | wc -c`, and scan.log's `git diff --name-only ... | wc -l`. It omits negative.log:83, which is asserted, and omits positive.log:108, which is also asserted.
2. **positive.log:108** `check "the run writes nothing to stderr" "$(yn [ "$(wc -c <"$T/stderr")" -eq 0 ])"`. A counting site feeding an assertion, outside the library, no control. It does fail closed (an unreadable file makes the substitution empty and `[ "" -eq 0 ]` errors, so `yn` prints no), but it contradicts the header's claim and is not in its exceptions list. gate.log:179 (`wc -c < "$T/err"`) is likewise unnamed, though only informational.
3. **A vacuous-pass path in the phase's central privacy check: positive.log:138-160.** If the manifest cannot be read, the four awk extractions at positive.log:133-136 leave empty pattern files, `count_lines` returns 0, and each iteration prints "no patterns to test (the manifest holds none)" and `continue`s (positive.log:140-143). The control loop `continue`s too (positive.log:154). Nothing asserts the manifest was readable: the row count at positive.log:91 is informational and would print 0. The log would end "positive log checks all as expected: yes" with all four privacy assertions skipped. It did not happen in this run (positive.log:452 shows 18 / 27 / 2 / 2 patterns), but the form is not fail-closed.
4. **positive.log:92-94**: `ls ... || printf 'the corpus directory could not be listed\n' >"$T/corpus-listing"`, then `count_lines` on that file. A failed listing reports "corpus files present: 1", a plausible number, not `error`. Informational only, but it is the same shape.
5. **Negative tests are not in a scratch copy.** specs/0001-bootstrap.md:69-71 requires a worktree or clone outside the repo. negative.log:34 does `cd "$(git rev-parse --show-toplevel)"` and runs `./scripts/corpus-add.py` from the checkout; only the corpora are under `mktemp -d`. The report admits this at lines 110-115 and asks for a ruling. Accurate, and still a breach as the rule is written.
6. **scan.log's base commit is typed.** `BASE=${1:?...}` (scan.log:28) is never resolved or verified by a printed git command; only `C` is asserted against HEAD (scan.log:110-113). specs/0001-bootstrap.md:119-120 says every commit id in a log is read from git at run time by a command the log prints. Minor, and it is the one commit id in the phase that is not.

## Q3. Does every number, quoted string and citation in the report appear in a primary at the line cited?

I opened every cited line. Correct: report:24-25 (gate.log:376-377, negative.log:458-459, 466-467); report:31-33 (corpus-add.py:111-120, negative.log:504-505, 511-512); report:37-38 (positive.log:361-365); report:43 (positive.log:428); report:45 (positive.log:454, 456, 458, 460 and :470); report:47 (positive.log:462, 464, 466, 468 and :472); report:48-49 (positive.log:477, 478); report:84 (negative.log:457, 464, 494, 503, 510, 533); report:90-92 (negative.log:493 contains the quoted `exit 2, stderr: error: PermissionError`); report:106 (gate.log:369); report:136-138 (negative.log:517-518); report:142-144 (gate.log:418-419, 369, 383-387; the "fifteen counting controls" are gate.log:315-329, exactly fifteen, with :331 the summary); report:148-152 (scan.log:622-626, 430, 404, 531, 574, 604, 620, each saying what the report says). The gap figures (27 files, 23 tags, 1 uncovered, 7 single-producer) match the record and positive.log:418-419. records/libreoffice-2026-09-15.md does exist and says it was copied from the corpus directory's `tools/libreoffice.md`, as report:65-68 claims.

Wrong or unsupported:

- **report:122, "positive.log:10"**, cited for the 2026-09-16 rulings. positive.log:10 is `#!/usr/bin/env bash`. The ruling text is at positive.log:12. Off by two, and it is echoed script comment rather than a result, though for an attribution that is unavoidable.
- **report:93, "(:495-496)"** for "I verified separately that it fails in the staging copy and not in the duplicate guard". negative.log:495 and :496 are the two assertion result lines. No primary shows any separate verification. See (b).
- **report:116-118** is contradicted by a primary in the safe direction: it says scan.log "covers neither 8f43882's message nor the four logs it commits". scan.log:577, 586, 595 record sha256 366e4f9b..., d8f98ec2..., f9f028b6... for gate, positive and negative in the working tree, and those are exactly the blobs committed at 8f43882 (I recomputed them). So three of the four final logs were scanned; only scan.log itself and 8f43882's message are uncovered. The diff-section scans at scan.log:477, 495, 486, 504 are of the superseded ae51d0a blobs (69e2f0..., 6494dd..., 70a21e..., d8b420...).

Nothing else in the report is a number or quoted string I could not find.

## The four specific questions

**(a) state().** The `ls` half is genuinely repaired: negative.log:69-74 returns `files=error rows=error temporaries=error` on a failed `mktemp` and on a failed listing, and `files`/`temps` come from `count_lines`/`count_matches`, which the controls prove report `error` on an unreadable file (negative.log:441, 451). The `rows` half is not. negative.log:83-86 counts with `awk | wc -l` and then tests `[ -r ]`; nothing reads the pipeline's status, and the `else` branch sets `rows=0` whenever `[ -f ]` is false. A `manifest.tsv` that is a dangling symlink, a directory, or a path whose stat fails for any reason other than the corpus listing failing first, yields the clean string `files=N rows=0 temporaries=0`. For N4 and N5, which assert `files=0 rows=0 temporaries=0`, that is still a clean-looking string on a manifest that could not be read. The claim in the header at negative.log:59-60, "Every count here reports the word error rather than a number when it cannot read", is therefore not true of `rows`. Separately, 8f43882's message says state() was "measured on four cases, including an unreadable corpus and an unreadable manifest": no log prints a single state() result other than plain numbers (every `files=` line in negative.log is at 457, 464, 465, 494, 503, 510, 533, all numeric). That measurement exists nowhere in the repo, which is the same rule breach as the defect it is repairing.

**(b) N3 attempt 2.** It is genuinely run now: negative.log:211-219 does the chmod, the invocation and the assertions, and negative.log:493-496 prints the result. "Not the duplicate guard" is demonstrated: the guard exits 1 with `refused: duplicate id` (negative.log:490, 532), and attempt 2 exits 2 with `error: PermissionError`. "Before any append" is demonstrated: rows unchanged at negative.log:494. But "it tested the staging copy" is still an inference. corpus-add.py prints only the exception class, so no log line says which call raised; `os.makedirs`, `os.chmod` and `shutil.copyfile` are all inside the same try. The inference is sound on my reading of the code (makedirs with `exist_ok=True` on an existing directory does not raise, so the EACCES is the copy), but the log does not show it, and the assertion's label at negative.log:217 asserts the conclusion while the condition tested is only the exit status and the message - the same "label claims more than the check" shape check 1 found elsewhere. The report's "I verified separately" is unmarked session output with no primary.

**(c) The four privacy controls.** All four are real and none is vacuous in this run. positive.log:152-160 salts a copy of the record with `head -n 1` of each pattern file and requires exactly 1; positive.log:461-468 shows all four at 1. All four pattern files are non-empty (positive.log:452: 18 stems, 27 notes, 2 versions, 2 app strings), so no iteration hit the `continue`. Two residual limits: each control exercises only the first pattern in its file, not all of them; and the whole block is skipped without failure if a pattern file is empty, which is gap 3 in Q2. I confirmed the earlier state: the superseded positive.log at ae51d0a has exactly one such control, "one stem appended" (its line 442).

**(d) The correction of ae51d0a.** Accurate. ae51d0a's message does contain "each counted with a control proving the count can report a hit", and the log it committed had only the stem control. 8f43882's message item 3 names the message, quotes the sentence, says why it was false, and says what now stands, and the repair is real at positive.log:461-468. The correction is complete for that claim. It is incomplete for the round as a whole, because the same message asserts the unrun state() measurement described in (a), and the intervening commit 0c3c6a3 asserts the unrun `--bogus` measurement described in Q1.

## The report's own admissions

Accurate as far as they go: the three check-1 items (report:75-99) each match a primary; "THIS IS THE THIRD TIME I have declared this class closed while instances stood" is an honest statement; the two self-found errors at report:156-163 are real (`yn ! cmp` is replaced by `differs` at positive.log:183, and the "$: no$" filter is session output, correctly marked). The open items at report:110-118 and the unverifiable rulings at report:120-127 are correctly flagged.

They are not complete. The class in question, a clean-looking result where a read or a run did not happen, has at least three further live instances that the report does not admit: negative.log:83 (asserted, outside the library, no exit status read, no control), positive.log:138-160 (four assertions silently skipped if the manifest cannot be read), and the two commit messages that state measurements no log prints. On the last, the count is now four declarations of closure with instances standing, not three.

## GAPS

1. 0c3c6a3's committed message states a measurement (`corpus-coverage.py --bogus` exits 2, prints the usage line on stderr) that no log prints, and the docstring sentence it added is evidenced by no log or gate check. Same class as the defect this round repaired; unadmitted. specs/0001-bootstrap.md:124-126.
2. 8f43882's committed message says state() was "measured on four cases, including an unreadable corpus and an unreadable manifest". No log prints any state() result other than plain numbers. Unadmitted; same rule.
3. negative.log:83-86: the `rows` count is a counting site inside assertions (negative.log:158, 173, 207, 219, 233, 250, 293), outside the shared library, whose exit status is never read and which has no positive control. A manifest.tsv that is a dangling symlink or a directory returns `rows=0` clean. Fourth instance of the class; unadmitted.
4. The shared library's header (negative.log:335-341, identical in all four logs) says every asserted counting site goes through the library and lists the exceptions. It is false: negative.log:83 and positive.log:108 are asserted and are not in the list, and gate.log:179 is unnamed.
5. positive.log:138-160: if the manifest cannot be read, all four privacy assertions and all four hit controls are skipped and the log still reports "all as expected". The phase's central privacy claim can pass vacuously.
6. records/corpus-coverage-2026-09-16.md attests `Commit: 58063fb` and script sha256 2b8be584, but the only log deriving the body ran ff02af14 at 0c3c6a3 (positive.log:356, 428). No current primary shows the attested script producing that output; neither logs nor report mention it.
7. report:93 cites negative.log:495-496 for "I verified separately that it fails in the staging copy"; those lines are assertions, not a verification, and no primary shows one. The stage that raised is still an inference, and negative.log:217's label asserts more than its condition tests.
8. report:116-118 is contradicted by scan.log:577, 586, 595: three of the four finally committed logs were scanned (hashes match the blobs at 8f43882); only scan.log itself and 8f43882's message are uncovered.
9. report:122 cites positive.log:10, which is the shebang; the ruling text is at positive.log:12.
10. positive.log:92-94: a failed corpus listing reports "corpus files present: 1" rather than error. Informational, same shape.
11. Negative tests run in the main working tree (negative.log:34), against specs/0001-bootstrap.md:69-71. Admitted in the report; still a breach until planning rules.
12. scan.log's `BASE` is a typed argument, never resolved or checked by a printed git command (scan.log:28), against specs/0001-bootstrap.md:119-120.
13. 0c3c6a3's message cites scripts/corpus-coverage.py:143-145 for the usage failure; at HEAD it is 144-146 (143 is `def main(argv):`).
14. specs/0005-corpus.md:309-310, "two files over 60 MB uncompressed", remains unproven. Admitted in the report at lines 61-64; listed so planning can rule.
