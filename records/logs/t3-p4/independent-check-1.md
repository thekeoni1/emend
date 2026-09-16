Independent check 1 of T3 P4.

Checked commit: 9d5e06094624cf5fb0e8c93d7ad0c53692d66832
HEAD when the check ran: 9d5e06094624cf5fb0e8c93d7ad0c53692d66832
Checker: a fresh subagent, given only the files below and never this
session's conversation, asked to answer in plain ASCII.

Inputs, as they stood at the checked commit:
  specs/0006-passthrough.md              sha256 c8ec782ba2fbe56e9d8cbb7c0ff7eae50f8999f054a588e5eccb92cdcb377108
  specs/0001-bootstrap.md                sha256 641692f661577f0de19a3586509d050259dd9182fd16ce135f11058f4824370b
  records/logs/t3-p4/gate.log            sha256 5b81b4dfdbd78c96d3da2ee987817129db97a91ccd47e9713b098ea66b13a4a3
  records/logs/t3-p4/scan.log            sha256 53eb75701cf09b94021dbb21d60712d0e3961271d0c15baed7cbe1e078d94171
  the phase diff                         git diff 0c2caf21f2f8e062c13945eac8332a534512e0d7 9d5e060
  the draft report (scratch)             sha256 UNRECOVERABLE, see the note below
  The checker was also told it could read negative.log, run.log and the
  record pair to check the report's numbers.

NOTE on the draft report's sha256: the draft the checker read was a
scratch file that this session later overwrote to produce draft 2, so its
hash cannot now be recovered. It is recorded as unrecoverable rather than
reconstructed: a plausible-looking hash in a header planning verifies from
primaries would be worse than an honest gap.

ASCII substitutions made before committing: none. The answer as returned
holds 17444 bytes and 0 non-ASCII bytes.

---

I checked only the files named in the task. CLAUDE.md was placed in my context by the harness; I did not use it as evidence.

# Q1. Does the diff do what the spec says for P4, and nothing else?

Substantially yes. The code diff is two files only (`git diff --stat`: `scripts/passthrough-record.py` +81/-, `scripts/passthrough-run.sh` +63/-), plus four logs and the record pair. gate.log:396 confirms no gate file, Cargo file, crate or workflow changed in the range.

Spec P4 pre-run items, each found in the diff:
- Stale refusal text (spec 0006:445): `passthrough-record.py` load() now raises "a %s value other than ok, fail, error or -".
- Error/fail boundary as ruled (spec 0006:446-447): `lo_check` gains `rc -eq 124` -> 2, `rc -ge 128` -> 2, keeps 125-127 -> 2, and 1-123 -> 1 falls through only after those.
- Error branches exercised (spec 0006:447): negative.log C3-C6 (negative.log:697-728).
- Headings (spec 0006:447-448): the skip condition becomes `row["lo"] != "fail" and row["lo_in"] != "fail"`, so an `error` row leaves the failing heading.
- `both_listed` without `-a` (spec 0006:446): fixed in the negative script, negative.log:139-144.
- Entry-script hash asserted rather than printed (spec 0006:448): gate.log:98-100 `entry_hash_agrees`, asserted at gate.log:225-226, result at gate.log:380.
- `--lo-timeout <secs>` default 600, own budget (spec 0006:458-460): parsed, validated, used at the `timeout --signal=KILL "$LO_TIMEOUT" podman run` site, and recorded in a new `lo_timeout_s` column.
- Whitespace-tolerant `<sheet` and `<table:table` (spec 0006:461-462): `r"<sheet(?=[\s/>])"` and `r"<table:table(?=[\s/>])"`.
- Chart-sheet workbook generated and measured (spec 0006:463-466): negative.log C7, measured "2 1 2 Alpha,Chart" (negative.log:672).
- The run itself and the record committed (spec 0006:468-470).

Two divergences from the spec text:

1. The spec asks for the record as `.md` and `.tsv` (spec 0006:470, and the Design section at 0006:113). The diff writes `.txt`. That is a real departure from the spec as written; the report names it (finding 6) and the primary supports the reason (`.gitignore` line 32 `*.[tT][sS][vV]`; `scripts/check.sh:244` `OFFICE_RE` includes `tsv`, with the `tests/fixtures/` exception at :248). The spec was not edited, so a spec-against-diff reading shows a mismatch.
2. The chart-sheet conditional the spec calls for (exclude `<chartsheet` parts if LibreOffice writes no `<table:table`) is not implemented, because the measurement showed it does not apply. The report states this openly (finding 4). I agree the measurement answers the conditional; the spec text remains unamended.

Not called for by anything in the evidence I was given: a large share of the diff is justified by "planning's ruling 1/2/4/5/6 of 2026-09-16", "option (c)" and "obligation 2", cited in code comments (e.g. `passthrough-run.sh` lines around the ladder and `--lo-timeout`, `lo_tally`'s docstring) and throughout the logs. The spec's P4 section carries no 2026-09-16 rulings; it stops at the P3 rulings of 2026-09-15. The widening of the outcome table from four columns ("lo fail", "lo_in fail", "lo error", "lo_in error") to eight (`LO_COUNTS`) and the `lo_tally` sum guard rest entirely on "ruling 4", which is in no document I can read. Judged against the spec alone, those are additions. Judged against the logs, they are consistently attributed. I flag it as unverifiable from the named evidence, not as wrong.

Nothing else in the diff is outside the spec's reach: `write_md` taking `stem` is a direct consequence of the `.txt` change, and the docstring rewrite documents it.

# Q2. Do the logs contain every line the rules require?

- HEAD on line 1: yes, all four. gate.log:1, scan.log:1, negative.log:1, run.log:1 each name the commit and the HEAD at run.
- Script printed into the log: gate.log:12-291, scan.log:6-294, negative.log:9-599. run.log prints no script; it prints the commands it ran (run.log:29, 59) and the runner is a committed file. The "print the exact script" bullet sits under "Scans" in specs/0001:97-114, so run.log is arguably out of its scope. Worth planning's attention rather than a defect.
- `/usr/bin/grep` with `LC_ALL=C`: satisfied everywhere I could find. scan.log:27 `G=/usr/bin/grep` with `LC_ALL=C` in every `f_*`; gate.log uses `LC_ALL=C /usr/bin/grep` at :59, :66, :84, :92, :99, :105-132, :144, :182, :198, :207; negative.log wraps it in `grep_q` (:101) and uses `LC_ALL=C "$G"` at :104-107, :143, :508, :583. The T3 P3 check-1 finding (a grep without `LC_ALL=C`) does not recur.
- Positive controls in the same command form: scan.log is exemplary (scan.log:141-201 build controls and run them through the same `f_*` functions; results at :306-369, "controls: all as expected" at :370). gate.log has controls only for the home-path form (:183-184) and the control-byte forms (:278-281); its seven other grep forms (`record_states`, `entry_has_flags`, `record_cites_private`, `lo_timeout_wired`, `ladder_as_ruled`, `tolerant_counts`, `record_p4_changes`) have none. negative.log has controls for the container-line and byte counts (:783-788) and none for its code-presence greps. These are assertions rather than privacy scans, so this is a weakness, not a rule breach.
- No grep exit of 2 or more reported as zero hits: scan.log is correct (`run()` at :92-97 and `check_ctl` at :98-105 both read rc, `rc >= 2` sets `err`; `f_email` returns 2 explicitly at :82). gate.log is NOT: at :207-208 the check "the record names no corpus path" compares `grep -c` output with `-eq 0` and never reads the exit status, so a read error would print `0` and the check would read `yes`. Same shape at :144, :182-184, :198. negative.log has the same weakness at :104-106 and :583-588 (`count_path`, `count_container_words`, `count_container_line` all pipe into `grep -c` and are compared to 0). This is the same class of defect T3 P3 check 1 raised, in new places. Neither the logs nor the report mention it.
- No control bytes: satisfied and evidenced. gate.log:426-429 (ESC/SI positive control gives 3 bytes, stdout and stderr give 0); scan.log f_ctrl count=0 on all eight diff files, all four logs and all four messages, with controls at :307-308; negative.log:786-788 (3-byte and 2-byte positive controls, 0 across every captured output).
- Compound checks evaluated inside one named function: mostly yes, but there are TWO instances of the exact form specs/0001 forbids ("The form `$(yn [ ... ] && cmd ...)` captures only the first test"):
  - scan.log:280 `"$(yn [ "$pre_hits" -eq 0 ] && [ "$pre_err" -eq 0 ])"`. Only the `pre_hits` test reaches the printed yes/no; the grep-error condition is silently dropped from the structural result at scan.log:586.
  - negative.log:380 `"$(is [ "${CS_COUNTS%% *}" = 2 ] && [ "$(echo "$CS_COUNTS" | awk '{ print $3 }')" = 2 ])"`. Only the input `<sheet` count is asserted; the ODS `<table:table` count is dropped. That dropped half is precisely the second leg of the chart-sheet finding the report's finding 4 rests on.
  Both happen to be true in fact (scan.log:586 shows errors 0; negative.log:672 prints "2 1 2 Alpha,Chart"), so no conclusion is wrong, but the two checks are weaker than their labels claim, and the rule is explicit. Neither log nor the report notes it.
- Negative tests in scratch copies, never the main working tree: satisfied, and this is the phase's clearest improvement. negative.log:5 (the departure), :180-198 (clone, checkout, binaries copied in, `cd "$CLONE"`), :614 (main tree untouched), :789-790 (main tree still clean and still at the commit). T3 P3 check 2's finding that the P3 script built in the main tree is fixed.
- Rerun headers naming the record commit they replace: satisfied. gate.log:3, scan.log:3 and negative.log:3 each name record commit 4d3cb4dcb5bfb29e9706e9908d416937996b9c53 and say why. Each also states that run.log is not rerun.

One thing unmentioned anywhere: gate.log:44 prints `CORPUS=${EMEND_CORPUS_DIR:-$HOME/emend-testing}` and gate.log:207-208 print the literal `emend-testing` inside the script text, while the header at gate.log:7 says "the corpus directory prints as <corpus dir>" and the check at :208 exists to keep that name out of the LibreOffice record. It is not an absolute path and the scan found 0 denylist hits (scan.log:417), so it is not a leak, but the header slightly overstates what the log does.

# Q3. Does every number and quoted string in the report appear in a primary?

Verified against primaries, correct:
- The run table in full: 27 files, ok 27, error/timeout/panic/crash 0, lo and lo_in 27/0/0/0, entry rows 870, identical 870, recompressed/changed/dropped/added 0, diff_exit 1 on 27, files_identical no on 27, leftover no on 27 (run.log:100-130 and the record's "Files by outcome" table).
- Flags "attrs 870, extra 578, gpflags 292, descriptor 292, local 292, mtime 0, comment 0" - both "total" rows of the record.
- "the name and status digest over all 870 entries is identical" - run.log:151-155.
- "lists all 54 rows" and "every lo and lo_in in that table reads ok" - I counted the failing section: exactly 54 data rows, all `ok | ok`.
- "The record's 'Files whose open check could not run' section reads none" - record's last table, `| none |`.
- "56 checks" - exactly 56 result lines in negative.log.
- C4 "the conversion was killed, exit status 137" (negative.log:713), C5 "podman could not run the container, exit 125" (:719), C6 "cannot count sheets and tables" (:726), C10 "error: summary.host.tsv has a lo value other than ok, fail, error or -" (:761), C11 "error: summary.host.tsv: the lo counts total 6, not the 7 files" (:772), C12 "error: --lo-timeout must be a positive whole number of seconds" (:778), C3 reason "the conversion timed out after 1 s" (:706), C2 reasons "stderr holds a line with Error or Warning other than the allowed javaldx line" (:657-658) - all verbatim.
- "14 shapes" - negative.log:682-695, 8 sheet forms plus 6 table forms.
- Chart-sheet numbers "2 1 2 Alpha,Chart" - negative.log:672.
- Entry script sha256 `2c09931b...` - gate.log:376, 379, 380. Eight PASS lines and exit 0 - gate.log:404-415. Binary hashes `2b961cce...` and `a718bb29...` - gate.log:421-422, negative.log:611-612, and run.log's settings block, so "unchanged across the rebuild" holds.
- "records/libreoffice-2026-09-15.md, 'The exit status is not evidence of a good load'" - gate.log:206 asserts that exact string is in the record.
- ".gitignore line 32 ignores *.tsv" - line 32 is `*.[tT][sS][vV]`. "check.sh's hygiene step refuses any tracked spreadsheet extension outside tests/fixtures/" - scripts/check.sh:244 and :248.
- Scan log claims "RESULT PASS, 0 hits, 0 errors, 0 structural failures, controls all as expected", four logs plus the record pair, pre-P6d forms 0 with a positive control - scan.log:370, 482, 568, 586, 588-592, 339-341.
- "the real 0c2caf21f2f8e062c13945eac8332a534512e0d7" in the committed gate script - gate.log:43. The committed value is the real full hash.

Not found in any primary, or contradicted:

1. NOT MENTIONED AND MATERIAL: "run.log is not rerun and is unchanged" is true but incomplete. run.log:118-121 records the record script printing "wrote step0-2026-09-16-39defb7.tsv and step0-2026-09-16-39defb7.md" and gives `records/step0-2026-09-16-39defb7.md: sha256 88173268ca625b49fe6b81af927f5b68127d88adc3906f3e33f27f7d3f4dc98b, bytes 14595`. The committed .md is `8ee0314e6c0b021131a5c204508040aaadcb5675b03688e9edb3d8c74191a9e1`, 14799 bytes (gate.log:338, scan.log:446). The .txt matches the old .tsv byte for byte (`8d0c78f8...`, gate.log:339), so only the .md changed. Consequence: the committed .md was produced by a later run of the record script that no log records, and run.log, the phase's primary for the record pair, names a file that was not committed and a hash for the .md that is not the committed one. The report does not mention this anywhere. A reader verifying the .md from run.log will find a mismatch.
2. "You have it on the passthrough backlog as 'no positive control exists on the pinned image'." That exact string appears nowhere in the repository. ROADMAP.md:177-181's passthrough backlog lists the T3 P2 gaps and says nothing about a positive control or the pinned image. Unsupported quoted string.
3. "You measured the same independently on your own 1946-byte fixture." "1946" appears in no primary; the negative script's chartsheet fixture is 1900 bytes (negative.log:623). Unverifiable.
4. "I doubted the 53-second cold run as too fast and checked the primaries rather than assert it: cargo clean reported 3613 files removed, the binaries' mtimes fall inside the run window, CARGO_TARGET_DIR is unset, and the rebuilt hashes are byte-identical." Only two of those four are in a primary (3613 files at gate.log:401; the hashes at :421-422). "53-second" appears nowhere; gate.log's own timestamps (:2 and :432) give 12:21:24 to 12:22:18, i.e. 54 s for the whole script, not for check.sh alone. mtimes and `CARGO_TARGET_DIR` appear nowhere in gate.log. The report's own preamble says anything not from a log is marked "(session output)"; these are not marked, and they are presented as the grounds for calling the run genuine.
5. "C3 a real forced timeout: --lo-timeout 1 against the pinned image, status 124". "124" appears in negative.log only inside the script's comments (:42, :49). The measured output at :704-708 shows "run exit: 0" and the reason text. The inference is sound, because only the `rc -eq 124` branch writes that reason, but the report states a measured status that the log never prints.
6. "MEASURED and worth stating plainly: a 40000-row and a one-row workbook both convert in about 2.1 s on this box". The 40000 rows are real (negative.log:272, :698). The 2.1 s is asserted in the script's own prose (negative.log:36-44, :699-703) and in no timing output anywhere. Calling it MEASURED overstates what the log holds; the log holds a claim, not a measurement.
7. "Cold cargo clean (3613 files, 696.2 MiB)". gate.log:401 reads "696.2MiB". Trivial transcription difference.
8. "0c2caf2 your three doc files, unedited." Three files confirmed (ROADMAP.md, specs/0001-bootstrap.md, specs/0002-gate-corpus-additions.md). "unedited" is not checkable from the named evidence.
9. The report opens "Citations are `<log>:<line>` under records/logs/t3-p4/ as committed." The report contains no line citations at all. Every claim above I had to locate myself.
10. Finding 3's "An earlier draft of that case asserted the status-fail branch; it was wrong" is uncheckable: negative.log:3 states that the cases are unchanged since 4d3cb4d, so the earlier draft predates the first commit and leaves no primary. Not a defect, but it is a self-report, not a finding from a log.

The report's central substantive claim - that diff_exit 1 and files_identical no on all 54 rows is zip metadata only and not a fidelity finding - is fully supported by the record's family tables (870 identical, 0 recompressed/changed/dropped/added, flags confined to attrs/extra/gpflags/descriptor/local) and run.log:131-149.

# GAPS

1. run.log, the phase's primary for the record pair, names `step0-2026-09-16-39defb7.tsv` and gives the .md as sha256 88173268..., 14595 bytes, while the committed .md is 8ee0314e..., 14799 bytes (gate.log:338). The committed .md came from an unlogged rerun of the record script. The report says only that run.log "is unchanged" and does not mention the mismatch. Either rerun run.log or add a note in the report and in the record commit naming the .md hash gate.log:338 proves, so a verifier from primaries is not left with a contradiction.
2. Two uses of the exact banned form `$(yn [ ... ] && [ ... ])`: scan.log:280 drops the `pre_err` condition from the bare-shape structural result, and negative.log:380 drops the ODS `<table:table` count from the chart-sheet assertion that finding 4 rests on. specs/0001 "Rules for every phase" forbids this form by name. Neither log nor the report mentions it.
3. Grep exit statuses of 2 or more read as zero hits: gate.log:207-208 ("the record names no corpus path"), and gate.log:144, :182-184, :198; negative.log:104-106 with :583-588. Same class as the T3 P3 grep-error finding, in new places, unmentioned.
4. Two quoted strings and one number in the report appear in no primary: the backlog wording "no positive control exists on the pinned image" (nowhere in the repo; ROADMAP.md:177-181 says nothing of the kind), the "1946-byte fixture", and the "53-second cold run" with its mtime and CARGO_TARGET_DIR grounds.
5. "MEASURED" is overstated twice: the 2.1 s conversion time (negative.log prose only, no timing output) and C3's "status 124" (script comments only, never printed as a measured value).
6. The spec still asks for the record as `.md` and `.tsv` (spec 0006:470, :113) and still carries the chart-sheet conditional (:463-466). The diff answers both differently and correctly, but the spec amendment is owed, otherwise the next reader sees a diff that does not match its spec.
7. Lesser: gate.log's seven code-presence grep forms and negative.log's code-presence greps have no positive controls, unlike scan.log; run.log prints no script; gate.log:44 and :207-208 put the corpus directory name in a log whose header says it prints as `<corpus dir>`; the report's stated citation convention (`<log>:<line>`) is used nowhere in the report; "696.2 MiB" is written "696.2MiB" in gate.log:401.
