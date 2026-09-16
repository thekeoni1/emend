Independent check 2 of T3 P4.

Checked commit: 182be0498ea34cf7717310dbbbc37ae6cffe4792
HEAD when the check ran: 182be0498ea34cf7717310dbbbc37ae6cffe4792
Checker: a fresh subagent, given only the files below and never this
session's conversation, asked to answer in plain ASCII. It was told a
first check had found three defects and that its job was to judge the
current state, not to assume the repairs were sound.

Inputs, as they stood at the checked commit:
  specs/0006-passthrough.md              sha256 c8ec782ba2fbe56e9d8cbb7c0ff7eae50f8999f054a588e5eccb92cdcb377108
  specs/0001-bootstrap.md                sha256 641692f661577f0de19a3586509d050259dd9182fd16ce135f11058f4824370b
  records/logs/t3-p4/gate.log            sha256 d794383aa9bb432e270d1db114d48a272f652786c553ba43f05f98a4b18f4434
  records/logs/t3-p4/scan.log            sha256 71a953e8e36f9c727e56ddc991f36c1d776afc49d952158f00ef3e1b351dc0e1
  the draft report (scratch)             sha256 cd2c083d1a6e34619729e5881f419ffd6d86f2f958002fe60b2b4403c1004436
  the phase diff                         git diff 0c2caf21f2f8e062c13945eac8332a534512e0d7 182be04
  The checker was also told it could read negative.log, run.log,
  record.log and the record pair to check the report's numbers.

ASCII substitutions made before committing: none. The answer as returned
holds 14768 bytes and 0 non-ASCII bytes.

---

I read the spec sections, the diff, the draft report and all five logs, plus the committed record pair. Note: the harness placed CLAUDE.md in my context; I did not use it as evidence.

# Q1. Does the diff do what the spec says for P4, and nothing else?

Substantially yes. Every P4 pre-run item in specs/0006-passthrough.md:450-473 is present in the two script commits, and the run itself is recorded.

Present and traceable:
- Stale refusal text (P3 carry-over): scripts/passthrough-record.py now raises "a %s value other than ok, fail, error or -" (diff hunk at load(); exercised at negative.log:825-826).
- Error/fail boundary as ruled: the 124 / 125-127 / 128-or-more ladder in scripts/passthrough-run.sh lo_check, with the status-0 fall-through (diff; asserted gate.log:372; exercised negative.log:767-772, 777-779, 783-785).
- The error branches exercised: C3 timeout (negative.log:771), C4 signal (778), C5 podman (784), C6 count step (791).
- Record headings and the four counts: LO_COUNTS and lo_tally added, and the "fail"-only condition for the failing heading (diff); measured at negative.log:815, 818-822.
- Entry-script hash asserted against the record rather than printed: gate.log:239-240, result gate.log:403.
- Separate --lo-timeout, default 600, timeout is error: diff in passthrough-run.sh; C12 refusal negative.log:842-844; the value reaches run.<label>.tsv (run.log:97-100).
- Whitespace-tolerant `<sheet` and `<table:table`: diff (lookahead forms); measured against 14 shapes at negative.log:739-752.
- Chart-sheet workbook generated and measured: negative.log:728-731. LibreOffice does write a `<table:table` for the chart sheet (2 and 2), so the spec's conditional does not fire and the record correctly says nothing about it.
- The run: 27 files, both targets, LO check on (run.log:31-100), record pair committed at 25e553a.

Departures and omissions the spec does call out:

1. The machine-readable table is committed as `records/step0-2026-09-16-39defb7.txt`, but specs/0006-passthrough.md:114 says `records/step0-<utc-date>-<shortsha>.tsv`. The underlying reason is verifiable in the repository (.gitignore line 32 is `*.[tT][sS][vV]`; scripts/check.sh:244 OFFICE_RE includes `tsv`), and the report calls it finding 6, but the only authority for the change is "planning's ruling of 2026-09-16" quoted inside the script docstring and the report. Nothing in a file I can read rules it. It is a real spec deviation, correctly flagged by the author but not yet reflected in the spec.

2. `records/logs/t3-p4/independent-check-1.md` is NOT in the diff. specs/0001-bootstrap.md:53-55 requires the check answer to be saved as `independent-check-N.md` with a header and committed; records/logs/t3-p1, t3-p2 and t3-p3 each hold `independent-check-1.md` and `independent-check-2.md`. The current committed state of t3-p4 holds no check answer at all, even though a post-check-1 record commit (182be04) has already been made.

3. `record.log` is not one of the logs the phase list names (specs/0006-passthrough.md:156-158: gate.log, test.log, negative.log, scan.log). Its existence is defensible as the ruled correction for run.log, and run.log is likewise unlisted but is the run's primary. I record it as an addition, not a fault.

Nothing else in the diff is outside P4. No crate, gate script, Cargo file or workflow changed, and gate.log:419 asserts exactly that from git.

# Q2. Do the logs contain every line the "Rules for every phase" requires?

Where the repairs are real:
- The forbidden compound form `$(yn [ ... ] && [ ... ])` has no live instance left. A grep of all five logs finds it only in prose: negative.log:398 (comment), scan.log:3 (rerun header) and scan.log:280 (comment). The replacements exist as named functions: pre_clean (scan.log:283), chart_counts_agree (negative.log:402-407), no_corpus_name (gate.log:212-216), only_expected_changes (record.log:78-89), clean_outputs (record.log:205-211). This repair is genuine.
- Rerun headers: gate.log:3, scan.log:3 and negative.log:3 each name the log they replace, the record commit it was committed at (9d5e060), the defect found and the fix. record.log correctly carries none.
- Negative tests in a scratch clone, never the main tree: negative.log:199-218 clones into <scratch> and cds in; the main tree is asserted clean at :670 and again at :853-854, and still at the phase commit at :854.
- Scripts printed in full: gate.log:12-305, scan.log:6-299, negative.log:9-655, record.log:21-222.
- /usr/bin/grep under LC_ALL=C throughout, with an extensive control block in scan.log:311-375 (all as-expected at :375) and controls in gate.log:184, 294-295, 388-390 and negative.log:847, 850-851.
- Control bytes counted with a positive control and zero found: gate.log:449-452, negative.log:850-852, record.log:281-285, scan.log per-file f_ctrl counts 0.

Defects that remain, all of the class check 1 found:

D1. gate.log:182-184. `n=$(grep -c -E '/home/|/Users/|/root/' file1 file2 "$RECORD" | awk -F: '{n += $2} END {print n + 0}')` then `check ... "$(yn [ "$n" -eq 0 ])"`. grep's exit is never read, and because the count is summed across three files by awk, an unreadable file leaves the readable files' zeros and awk prints 0, so a grep error prints "yes". This is exactly the defect the gate's own rerun header (gate.log:3) says was fixed, in the same script, twelve lines before the fix. Unrepaired and unadmitted.

D2. record.log:213-215. Identical shape, and worse: `n=$(grep -a -c -i -F -e emend-testing "$T/old.stdout" "$T/md.diff" 2>/dev/null | awk -F: '{n += $NF} END {print n + 0}')` then `[ "$n" -eq 0 ]`. The `2>/dev/null` actively discards the error grep would report. This is in record.log, the log added at 182be04 as part of the repair commit. It also has no positive control, which specs/0001-bootstrap.md:108 requires of every scan form.

D3. negative.log:125-126 and 648-649. `ctrl_bytes() { cat "$@" | tr -cd ... | wc -c; }` and `nonascii_bytes()` the same, compared with 0 over 18 captured files at :648-649. If any file cannot be read, cat fails, tr sees an empty stream and wc prints 0, which reads as clean. The author's own comment at negative.log:111-115 and report defect 2 identify precisely this cat-in-front failure for the grep helpers and fix those, then leave the identical shape in the two byte counters. Unadmitted.

D4 (weaker). gate.log:243-244 reads a single-file `grep -c` without its exit status. It fails closed (an empty `n` makes `[ "" -eq 0 ]` error and yn prints no), so it is not a hole, but it does not follow the rule either.

D5. record.log:263-264 print real absolute scratch paths: `/tmp/tmp.JBtjAcd9RX/out/step0-2026-09-16-39defb7.md` and `/tmp/tmp.JBtjAcd9RX/committed.md`, from the diff -u file headers. specs/0001-bootstrap.md:116-118: "Logs and check answers write the repository as `<repo>`, scratch locations as `<scratch>` ... an absolute machine path in a log is a defect (T0 P6d ruling)." The script masks paths in its own echoes (record.log:170) but pipes the diff through nothing. Unadmitted.

D6. Scan coverage is incomplete for the committed state. scan.log ran at 9d5e060 and scans four logs (scan.log:253, result :590) plus the eight diff files (:494). record.log, added at 182be04, is scanned by nothing: `grep -c 'record\.log'` over scan.log and gate.log returns 0 in both. Consequently D5's absolute paths were never subjected to any scan. gate.log's EXPECTED_PATHS (gate.log:169) also names eight paths while the final diff 0c2caf2..182be04 has nine, so no gate or scan run covers HEAD as it now stands.

D7 (minor). record.log:1 is `command: git rev-parse HEAD`; the about/HEAD line is line 2. specs/0001-bootstrap.md:31-32 puts that on line 1. The only ruling permitting a line-2 form (specs/0001-bootstrap.md:180) is about check-answer files, not logs.

D8 (minor). The gate's positive control at gate.log:222 asserts `[ "$CTL_HITS" -eq 1 ]` rather than calling no_corpus_name, so the control exercises the grep form but not the function it controls for. Not the forbidden form, and it fails closed.

# Q3. Does every number, quoted string and line citation appear in a primary at the line cited?

I opened every cited line. The great majority are exact.

Verified correct at the cited line: run.log:111, :112, :113, :114, :117, :118, :119, :120, :121-:130 (all the table's figures), :133-145, :146-152, :155-157 (digest dc9d540d4359c2c85c5429c12de4e49f on both labels); gate.log:387, :403, :424 ("Removed 3613 files, 696.2MiB total"), :444-445 (2b961cce... and a718bb29...), :362-363; scan.log:375, :458, :467, :590, :608, :610-614; negative.log:667-668, :713-714, :728 ("2 1 2 Alpha,Chart"), :729, :730, :762-763 (slow 2.1 s, healthy 1.8 s), :767-768 (exit=124 wall=1.0 s), :770, :777 ("exit status 137"), :783 ("exit 125"), :790, :815, :826, :837-838, :843, :848-849, :856; record.log:16-19, :236, :244, :257, :276. The flag totals (attrs 870, extra 578, gpflags/descriptor/local 292, mtime 0, comment 0) are at records/step0-2026-09-16-39defb7.md:46 and :75. ".gitignore line 32 ignores *.tsv" is exactly right, and scripts/check.sh:244 confirms the hygiene refusal. "run.log is untouched, still as first committed at 4d3cb4d" is true: the blob is cc0de5ab at both 4d3cb4d and 182be04. The two quoted LibreOffice findings are at records/libreoffice-2026-09-15.md:75 and :79.

Citations that point at the wrong line:
1. Report line 110, "count 0 exit 1 against a control counting 1 exit 0 (gate.log:389-390)". Those numbers are on gate.log:388. Lines 389 and 390 are the two yes/no result lines and carry no counts.
2. Report line 104, "single named functions, pre_clean (scan.log:608)". The name pre_clean appears at scan.log:283 and :285. Line 608 is a result line and does not contain the word.
3. Report line 182-183, "the eight PASS lines in order, exit 0 (gate.log:438)". Line 438 is the order check only; "exit: 0" and "exit is 0: yes" are at :436-437.
4. Minor: several cites point at the result line rather than the quoted text, e.g. "the new refusal text (:826)" where the text is on :825, and "(:843)" where the text is on :842. Harmless but not what the convention promises.
5. chart_counts_agree is named with no citation; it is at negative.log:402.

Numbers or claims not in a primary:
6. "57 checks, none unexpected (negative.log:856)". Line 856 says only "negative log checks all as expected: yes". The figure 57 is the report's own count. I counted it independently: 57 lines ending in ": as-expected" and 0 "UNEXPECTED" in the results section (lines 657-857). The number is right, but it is not printed anywhere.
7. The session-output items (the ~54 second gate wall time, binary mtimes, CARGO_TARGET_DIR, run.log's sha256) are correctly marked as session output. The run.log sha256 is in fact in a primary (scan.log:440 and :572), so that one is understated rather than unsupported.

Where a primary contradicts the report:
8. Report lines 16-18: "182be04 record commit: ... No tracked file changed in this one". `git diff --name-status` for 182be04 shows four tracked files changed: gate.log, negative.log and scan.log modified, record.log added. The following clause makes the intent clear (no source file changed; the repairs were in the untracked log scripts), but as written the sentence contradicts the diff.
9. Report defect 2 admits the bare-count defect in "the gate's corpus-path check and three of the negative script's count helpers" and states the repair closed it. Two instances of the same defect survive in files the report presents as repaired: gate.log:182-184 (D1) and negative.log:125-126 with :648-649 (D3), plus a fresh one at record.log:213-215 (D2) in the log added by the repair commit. The admission is accurate as far as it goes and incomplete as a statement of the current state.
10. The report's account of record.log (lines 128-146) is otherwise accurate and checks out line by line, but it does not disclose that record.log prints two absolute /tmp paths (record.log:263-264) or that record.log is covered by no scan.

The three self-admitted repairs are real where claimed: the forbidden compound form is gone everywhere (grep confirms only prose mentions), no_corpus_name reads the exit with a same-form control (gate.log:212-222, results :388-390), and the two previously asserted figures are now printed and measured (negative.log:762-763, :767-768). Finding 3's correction about the status-fail branch is also correct: both refused archives are caught by the stderr scan (negative.log:713-714), and the log itself says the status-fail branch is unexercised (negative.log:719-722).

GAPS

1. gate.log:182-184 still compares a multi-file `grep -c` piped through awk with 0 without reading grep's exit; a read error prints 0 and the check passes. Same defect class the gate's own rerun header claims to have fixed; unadmitted.
2. record.log:213-215 repeats that shape with `2>/dev/null` hiding the error, and has no positive control (specs/0001-bootstrap.md:108). It is in the log created by the repair commit.
3. negative.log:125-126 ctrl_bytes and nonascii_bytes still use `cat "$@" | tr | wc -c` and are compared with 0 at :648-649, so an unreadable file reads as clean. This is the exact cat-in-front failure the author documents at negative.log:111-115 for the other helpers; unadmitted.
4. record.log:263-264 print absolute machine paths (`/tmp/tmp.JBtjAcd9RX/...`), which specs/0001-bootstrap.md:116-118 calls a defect; unadmitted.
5. record.log is scanned by nothing (scan.log covers four logs; 0 mentions of record.log), and no gate or scan run covers the current HEAD 182be04: gate.log's EXPECTED_PATHS names eight paths while the final diff has nine.
6. `records/logs/t3-p4/independent-check-1.md` is not committed, though specs/0001-bootstrap.md:53-55 requires it and t3-p1, t3-p2 and t3-p3 each hold both check answers.
7. The table is committed as `.txt` where specs/0006-passthrough.md:114 says `.tsv`; the justification rests on a planning message, not on any file in the repository, and the spec is unedited.
8. Report line 16 "No tracked file changed in this one" is contradicted by the diff for 182be04 (four tracked files).
9. Miscitations: gate.log:389-390 for numbers that are on :388; scan.log:608 for the name pre_clean, which is on :283/:285; gate.log:438 for an exit status printed on :436-437.
10. "57 checks" is the report's own count, not a line any log prints (negative.log:856 states only the pass verdict). The figure is correct as counted.
11. Minor: record.log line 1 is a command echo, not the about/HEAD line specs/0001-bootstrap.md:31-32 asks for; gate.log:243-244 reads a single-file `grep -c` without its exit (fails closed); gate.log:222's positive control asserts a count rather than calling no_corpus_name.
