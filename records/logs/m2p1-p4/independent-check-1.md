# Independent check 1, M2 part 1 P4 (the ruling 11 round)

- Checked commit (the phase commit): 3bd425730cd1e95d892fd8d44aca94cfda6b50f7
- HEAD when the check ran: 3bd425730cd1e95d892fd8d44aca94cfda6b50f7
- Checker: a fresh general-purpose subagent, given only the directory <scratch>/p4/check1 and the three questions of CLAUDE.md
- Answer: 8391 bytes, sha256 368ff8ddd64f0e262109fbfcaa4393288ff3ba66212a4bc64d5d684e952025ac, committed below verbatim; it is plain ASCII and names no machine path, so nothing was replaced
- Inputs, each with its sha256 (sha256sum over <scratch>/p4/check1):

    d7dd0b711cd40adfea48e5d61cb40717c89a0f48c5b9b7af2adcd4d7db3a808f  diff-3bd4257.patch
    1eb14acc3226a89887aadf9be039d0cb5dbdc3d8aadcca3fe456d92cdb156dba  diff-8abc666.patch
    d00d277ebd8b5bdbbc2a775e0bc2da57feb0a19ec299695cbc8ac2f96501909b  diff-b61fde5.patch
    67d771e1c6314bfcc9f3cb8ef628d90416ee8fc32e6b1d6b6400b3798305deb8  diff-commands.txt
    832796c91dc25c621316aa2cc5166c30098c9b29e978b3bf0b0cf0dc12611bbf  diff-doc-d6396bb.patch
    71df118924befea28ca6847f88a12c454d4fe1f5f0c97768b8a950b58720d016  diff-phase.patch
    c687c0d2b107d9c6dc08a678c78a7143926732417cfe34ac973e3af66f64c9cd  log-gate.log
    2c59250777ea1969ac578ee4773ca45332605c66a8ecd72af9d77daec7cba3c1  log-negative.log
    5a447edf4ad51903370beb486ec46b91d52da875e6270ad33ebda27409af60c8  log-run.log
    1f5c33cb1c93e00cc6175c098a008430e9f51d9cef9e1b81ef91fef5202b920e  log-scan-pre-check.log
    929f8152ccabebdbf9c54d20d6efc7ee0fdbec060c88f8d17ff25409b49b7031  log-size-attribution.log
    844f48c86e8618ae89cdf3f6b9b062fc04dad68c4cc8ae9f5236037c46f6739c  log-size.log
    5096fd67870b8b13b8cc87ec748b05515748870843fc18114cebd64fbc3dbed8  log-test.log
    531f07a79585b6df350b666de372b028a27f560d37b290dc9a26ec31435934ad  record-m2p1-open-2026-09-25-3bd4257.md
    f8876f9e6e46d254f3a587d6318f728afff283dcdba3f9d7d25409a925f81934  record-m2p1-open-2026-09-25-3bd4257.txt
    8ffc4112853f3fad4f0d1d28e41d14092b08ac39895126ecbc8b21c93954afb8  record-m2p1-p4-2026-09-25-3bd4257.md
    0c152dd3f6c851a2a083e4e438b6a5205ba84aa5d92ce3470f58c208d543c60c  record-m2p1-p4-2026-09-25-3bd4257.txt
    7cf001167c0b8b89efdc4ade17d23e907c80284eaaa126142a8f8ac5356d1dd9  record-size-m2p1-2026-09-25-3bd4257.md
    ccaf66810994fba11f2fce38520075decad10feb77de5e28e3d72287898462fe  report-draft.md
    84c149d39875db7cd6ba6eff3e185ff4e2be56e474ee9ba151983cf2243ea31f  specs-0001-bootstrap.md
    b8bf30f3b380d2d0af151a1dae39502a94148787e54c418d99131d1ddf56bc22  specs-0016-new-workbook-and-sheets.md
    0d526141847aad1da91274a7a9041daafde9958859896d047574a9e061e64a3a  superseded-size-attempt-3.log
    562bc30393de8f29d9ff7edb8b72e96421e94f07bf6f93d32d57e0859cdb3d02  superseded-size-attempt-4.log
    cd09ae6ca93cbe31813292b476eb00670d882d96b25bd94c5ffa57bb4fe66699  superseded-size-record-attempt-3.md

---

Independent check of M2 part 1 P4, final round (ruling 11). Directory checked: <scratch>/p4/check1.

The harness put <repo>/CLAUDE.md and the user's auto-memory index into my context. I did not use either as evidence. I modified no file in <scratch>/p4/check1. My own intermediate files went to a sibling scratch directory, which I deleted afterwards.

Every sha256 the report cites matches `sha256sum` of the file in the directory. That covers the seven logs, the five records and the superseded attempts (0d526141 log-3, 562bc303 log-4, cd09ae6c record-3). specs-0016 hashes to b8bf30f3, the value size.log line 3615 prints at the end of its run.

## Question 1: does the diff do what the spec says, and nothing else?

Diffs I read: diff-phase.patch, diff-doc-d6396bb.patch, diff-8abc666.patch, diff-b61fde5.patch and diff-3bd4257.patch.

- **F1 (confirmed, all five diffs).** diff-phase.patch is exactly the four commit diffs composed. I compared added and removed lines per file. The only excess is 8 lines in package.rs that 8abc666 adds and 3bd4257 replaces. No file is outside the eight the scan lists (log-scan-pre-check.log line 1490 onward).
- **F2 (confirmed, diff-8abc666.patch).**
  - `is_visible` (line 96 onward) treats `state` absent or exactly `visible` as visible, as ruling 1 says.
  - `SheetPlan.first_visible` now uses it (lines 440-452).
  - The module header adds the caller's picture, what the walk does not do, and the pending Excel check (lines 34-61), plus the two refusal messages (lines 19-21).
  - No pre-existing test line is removed from tests.rs, and nothing in the phase diff adds `unsafe`.
- **F3 (cannot confirm from these diffs, diff-phase.patch).** No P4 diff adds the header sentence on the name rule's approximation that "P4" lists (spec line 583). It may already exist from P3, but nothing in this directory shows it.
- **F4 (confirmed, diff-b61fde5.patch).**
  - `--remove-sample` is added with its assertions.
  - `workbook_counts` gains a parameter; the two `--sheet-sample` call sites pass `None`, a mechanical change.
  - size-probe/Cargo.toml adds only the new `[[bin]]` with `required-features = ["emend", "xml"]` and comment text. No new dependency.
  - The probe performs the steps "Size" and rulings 6 and 7 describe, and prints the twenty lines.
- **F5 (confirmed, diff-3bd4257.patch).** Ruling 9 is carried out: `rels_over` removes a removed rels part's `Override`. The header and doc comments say so, and the test `a_removed_rels_part_loses_its_override_too` is added. Nothing else changes.
- **F6 (confirmed from the logs; no diff covers this round).** "No crate change in this round" holds: every log's `about:` line names 3bd4257 as both the phase commit and HEAD.

## Question 2: do the logs contain every line the spec requires?

- **F7 (confirmed, log-size.log).**
  - Lines 1-3: command, invocation, `about:`.
  - Shape greps with controls.
  - Commits by role read from git, with the comparison against HEAD and two refusing controls.
  - Line 41: the rerun header naming attempts 3 and 4.
  - Lines 42-48: sha256 for each superseded log, script and record.
  - Root lines with an `apart` control; `SERIALIZED` line; `RUSTUP_AUTO_INSTALL=0` and `CARGO_TERM_COLOR=never`.
  - The printed script.
- **F8 (confirmed, script-to-script diffs).**
  - Attempt 5's printed script differs from attempt 4's only at lines 1871 and 1874 (the double dash removed).
  - Attempt 4 differs from attempt 3 only by the ATTEMPT 4 a-d changes. The fix in the formulas block uses `$FORM` in place of `$NEW`.
  - I reproduced adc333da and 2b719cc7 from the printed scripts of attempts 3 and 4 (the extraction minus its final newline).
- **F9 (confirmed, log-size.log).**
  - The 37 probe-figure lines are identical to attempt 3's.
  - The five `hash:` lines at lines 3420-3424 are identical to those of attempts 3 and 4.
  - run.log lines 1135-1143 compared exactly these four values (emend-copy and xlsx-diff, host and i686), read from attempt 3's size.log (0d526141). Ruling 11's condition for run.log attempt 3 to stand is met.
- **F10 (confirmed, log-size.log).**
  - Figures: line 2946 onward. Growth 166180, drift, and 7 of 8 commits: lines 2962-2987. Line 2988 reads "above the ceiling by 85776".
  - Attribution read by command, with its sha256 and a same-binary check: lines 2991-3006.
  - Record checks: lines 3601-3609, including the new wording and no negative "under".
  - Last lines 3617-3619: one check not as expected, the ceiling.
  - The record in the file is byte-identical to the copy printed in the log.
- **F11 (gap, log-size.log line 44).** The attempt-3 script is hashed as if it had been kept. The log does not say it is an extraction from attempt 3's printed log, or that the original file was edited in place and not kept. Only report-draft.md lines 70-74 say so. specs-0001 requires an attempt that was not kept to be named as having no hash, and why.
- **F12 (gap, log-test.log line 2259).** The heading says each planted mutation is "caught by a test this phase adds". The first-sheet mutation is caught at line 2318 by `a_move_leaves_a_first_sheet_past_the_end_unchanged`. That test is not among the 24 added (lines 135-158) and appears in no P4 diff. The report (line 98) does not repeat the claim.
- **F13 (observation, log-run.log lines 26 and 213-225).** run.log prints no `rerun:` line, yet ruling 11 and the report call it "attempt 3". gate.log line 49 suggests run.log never ran in attempts 1-2, which would make "attempt 3" a round label. The log itself does not say so.
- **F14 (confirmed, all seven logs).**
  - No absolute machine path: every `/home/`, `/mnt/` or `/tmp/` hit is pattern text or a placeholder.
  - No non-ASCII byte and no control byte.
  - The three forbidden shapes: 0 in every log.
  - Each log prints its script.
  - size-attribution.log prints each variant's diff, the control rebuild at 1012208, and the three figures (lines 103, 113, 123).
  - negative.log has R1 and R2 (lines 130 and 132 of the filtered output).
- **F15 (observation, log-scan-pre-check.log).** Header F is present. The pre-P6d list now includes size.log, size-attribution.log and the size record. The six sibling logs are hashed at their current bytes. RESULT: PASS. Header B still says "five" logs, but F supersedes it.

## Question 3: does every number and quoted string in the draft report appear in a primary?

Found in a primary:
- All the hashes.
- 478992, 479232, 393216, 85776, 135968, 166180, 30212, 0, 8 and 7, 4940, 3 and 9.
- 1012208, 903196, -109012, 986700, -25508, 1012384, 176.
- 585, 0 and 2 on each target; 24 added and 0 removed.
- 27 of 27, removed 24, parts 93, plants 9, crate 158, tool 43, 0 refused.
- "80404 ... becomes -85776" (size.log line 2987).
- Attempt 5's record hash equal to attempt 4's (size.log line 48 and scan line 1418).

Not supported by any primary:
- **F16 (report-draft.md lines 64-65).** "both counts came back error": the attempt-4 log prints only `no` for those checks (lines 3605-3606). The value `error` is never printed. It is inferred, and restated in size.log line 41.
- **F17 (report-draft.md lines 72-74).** The claims that the script was "edited in place ... before this session" and that "a diff shows the extraction differs ... only on the two lines" are unlogged. No primary holds that diff.
- **F18 (report-draft.md line 85).** "ruling 11, step 2" is the report's own label; ruling 11 is not numbered in steps.
- **F19 (report-draft.md lines 53-54).** "All thirteen probes' sha256 and figures in attempt 5 are those of attempt 3": no log prints this comparison. It is true: I confirmed it by comparing the two logs (F9).

Content the spec requires but the report lacks:
- **F20 (report-draft.md; spec lines 610-611).** "P4" requires the report to name the corpus gap of decision 4: no macro, threaded-comment, chartex or Google Sheets workbook has had a sheet removed. Neither the report nor any record in the directory names it.
- **F21 (ruling 11 wording, record-size lines 122-145).** Ruling 11 asks the record to name "ruling 10's three figures". The record's finding section gives the attribution's three variant figures, which ruling 11 itself states. The tables carry the numbers ruling 10 quotes. I take either reading as satisfied, but it rests on interpretation.

Disagreements with the report: F16, F17 and F20, plus the two log gaps F11 and F12.