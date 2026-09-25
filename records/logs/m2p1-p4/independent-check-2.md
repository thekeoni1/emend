# Independent check 2, M2 part 1 P4 (the ruling 11 round)

- Checked commit (the phase commit): 3bd425730cd1e95d892fd8d44aca94cfda6b50f7
- HEAD when the check ran: 3bd425730cd1e95d892fd8d44aca94cfda6b50f7
- Checker: a fresh general-purpose subagent, given only the directory <scratch>/p4/check2 and the three questions of CLAUDE.md; the last check of the phase
- Answer: 8815 bytes, sha256 ec105049052408a6e912dec86f61bba4a2328376c8a0fede2f1393a6f3e02378, committed below verbatim; it is plain ASCII and names no machine path, so nothing was replaced
- Inputs, each with its sha256 (sha256sum over <scratch>/p4/check2):

    d7dd0b711cd40adfea48e5d61cb40717c89a0f48c5b9b7af2adcd4d7db3a808f  diff-3bd4257.patch
    1eb14acc3226a89887aadf9be039d0cb5dbdc3d8aadcca3fe456d92cdb156dba  diff-8abc666.patch
    d00d277ebd8b5bdbbc2a775e0bc2da57feb0a19ec299695cbc8ac2f96501909b  diff-b61fde5.patch
    67d771e1c6314bfcc9f3cb8ef628d90416ee8fc32e6b1d6b6400b3798305deb8  diff-commands.txt
    832796c91dc25c621316aa2cc5166c30098c9b29e978b3bf0b0cf0dc12611bbf  diff-doc-d6396bb.patch
    71df118924befea28ca6847f88a12c454d4fe1f5f0c97768b8a950b58720d016  diff-phase.patch
    0917754c92f32b997c69de91997e7f8d22ed89882f077ec261bf9411be93ace3  independent-check-1.md
    c687c0d2b107d9c6dc08a678c78a7143926732417cfe34ac973e3af66f64c9cd  log-gate.log
    2c59250777ea1969ac578ee4773ca45332605c66a8ecd72af9d77daec7cba3c1  log-negative.log
    5a447edf4ad51903370beb486ec46b91d52da875e6270ad33ebda27409af60c8  log-run.log
    14a740968a9c10b0fa0f017c97373eac907373572ad425281f14aca880adcc72  log-scan-pre-check.log
    929f8152ccabebdbf9c54d20d6efc7ee0fdbec060c88f8d17ff25409b49b7031  log-size-attribution.log
    e17979a14bf8096c0abf289378bb6051db07f53e04c8c0d78e5ec68c1bd762ec  log-size.log
    5096fd67870b8b13b8cc87ec748b05515748870843fc18114cebd64fbc3dbed8  log-test.log
    531f07a79585b6df350b666de372b028a27f560d37b290dc9a26ec31435934ad  record-m2p1-open-2026-09-25-3bd4257.md
    f8876f9e6e46d254f3a587d6318f728afff283dcdba3f9d7d25409a925f81934  record-m2p1-open-2026-09-25-3bd4257.txt
    8ffc4112853f3fad4f0d1d28e41d14092b08ac39895126ecbc8b21c93954afb8  record-m2p1-p4-2026-09-25-3bd4257.md
    0c152dd3f6c851a2a083e4e438b6a5205ba84aa5d92ce3470f58c208d543c60c  record-m2p1-p4-2026-09-25-3bd4257.txt
    7cf001167c0b8b89efdc4ade17d23e907c80284eaaa126142a8f8ac5356d1dd9  record-size-m2p1-2026-09-25-3bd4257.md
    1a6784c94c370dcdc3751c7b9e3dbfd05d77a0344cf1be7c06e1bf9f54c3e404  report-draft.md
    84c149d39875db7cd6ba6eff3e185ff4e2be56e474ee9ba151983cf2243ea31f  specs-0001-bootstrap.md
    b8bf30f3b380d2d0af151a1dae39502a94148787e54c418d99131d1ddf56bc22  specs-0016-new-workbook-and-sheets.md
    0d526141847aad1da91274a7a9041daafde9958859896d047574a9e061e64a3a  superseded-size-attempt-3.log
    562bc30393de8f29d9ff7edb8b72e96421e94f07bf6f93d32d57e0859cdb3d02  superseded-size-attempt-4.log
    844f48c86e8618ae89cdf3f6b9b062fc04dad68c4cc8ae9f5236037c46f6739c  superseded-size-attempt-5.log
    cd09ae6ca93cbe31813292b476eb00670d882d96b25bd94c5ffa57bb4fe66699  superseded-size-record-attempt-3.md

---

Independent check 2, M2 part 1 P4 (the ruling 11 round). Directory checked: <scratch>/p4/check2.

Overall, the diffs, logs and records hold up. The report has a few small gaps (G14 to G19), and test.log's heading (G9) still waits on the ruling the report asks for.

The harness put <repo>/CLAUDE.md and the user's auto-memory index into my context. I did not use either as evidence. I modified no file in <scratch>/p4/check2. My extractions went to a sibling scratch directory, which I deleted afterwards.

Every sha256 the report cites matches sha256sum of the file in the directory: e17979a1 size.log, 14a74096 scan, 844f48c8 attempt 5, and all the others. The answer inside independent-check-1.md, after its "---" line, is 8391 bytes with sha256 368ff8dd, as its header says.

## Question 1: does the diff do what the spec says, and nothing else?

- G1 (confirmed; diff-phase.patch against diff-doc-d6396bb, diff-8abc666, diff-b61fde5 and diff-3bd4257). The diff files are byte-identical to check 1's inputs (same sha256). Added lines: 175 + 1165 + 725 + 75 = 2140 in the commits, against 2132 in the phase diff. The excess is the 8 lines 8abc666 adds and 3bd4257 replaces, as check 1's F1 says. No file lies outside the eight.
- G2 (confirmed, diff-8abc666.patch lines 1-80 and the tests). The module header carries:
  - the caller's picture;
  - what the walk does not do;
  - the Excel check pending;
  - the two new refusals;
  - the P3 verification rulings' limits (sheetId past u32::MAX).

  24 test functions are added. "A name that is not there" is covered by a remove_sheet assertion added to the existing P3 test (diff-phase.patch lines 513-520), not by a new test.
- G3 (confirmed, diff-b61fde5.patch lines 385 onward). The probe does add, move, rename, remove, write, then `Package::new` written to a third path, as "Size" and rulings 6 and 7 say.
- G4 (cannot confirm from any diff here). The name-rule approximation sentence that "P4" lists (spec line 583) is not in any P4 diff. The report's explanation (line 144-146: "package.rs line 443 ... 770c9dd") cannot be checked: no file here shows that line or that commit's contents.
- G5 (confirmed; no diff covers this round). No crate change: size.log attempt 6's line 3 names 3bd4257 as both the phase commit and HEAD. Its printed script is byte-identical to attempt 5's; I extracted both and diffed them.

## Question 2: do the logs contain every line the spec requires?

- G6 (confirmed, log-size.log lines 1-60). Present:
  - command, invocation, about:;
  - shape greps with controls;
  - commits by role, read from git;
  - the HEAD comparison with its null-commit and short-sha refusal controls;
  - the rerun header (line 41) with sha256 lines for attempts 3, 4 and 5 (lines 43-51);
  - root lines with the apart control;
  - the SERIALIZED line, RUSTUP_AUTO_INSTALL=0 and CARGO_TERM_COLOR=never;
  - the printed script from line 65.

  Placeholders are used throughout. The only /home/ or /tmp/ text is a pattern, at line 1918. There are 0 non-ASCII bytes, 0 control bytes, and 0 hits for each of the three forbidden shapes.
- G7 (check 1's F11 is fixed, log-size.log line 41). The header now says attempt 3's script file was not kept, has no hash, and why (it was edited in place). It names the hashed file as an extraction of the printed script (line 44). I reproduced adc333da, 2b719cc7 and 56ac022d from the printed scripts of attempts 3, 4 and 5. The superseded log files hash to the 0d526141, 562bc303 and 844f48c8 printed in the rerun header, and the attempt-3 record to cd09ae6c.
- G8 (ruling 11's run.log condition is met).
  - The five hash: lines are identical in attempts 3, 4, 5 and 6 (log-size.log lines 3423-3427).
  - run.log lines 1135-1143 compared exactly those four emend-copy and xlsx-diff values, host and i686, read from size.log 0d526141 (attempt 3).
  - Apart from the rerun header, the attempt-5 hashes it adds and the timestamps, attempt 6 is identical to attempt 5 (diff: lines 40-41, 44, 49-51, 3619). Its figures section (line 2514 onward) is identical to attempt 3's.
  - The record is 7cf00116, the same as attempts 4 and 5. It says "above the ceiling by 85776" (record-size line 91), and its "The finding" section is at lines 122-145. size.log checks both (lines 3610-3611).
  - The log ends with "checks not as expected: 1", the ceiling (lines 3620-3622).
- G9 (gap, carried: log-test.log line 2259). The heading "each caught by a test this phase adds" is still wrong for the first-sheet mutation (line 2311). The report states this and asks for a ruling. It is not fixed.
- G10 (minor gap, log-size.log line 41, and superseded-size-attempt-3.log line 41, which has no rerun line). size.log's numbering starts at attempt 3. No size.log line says that attempts 1 and 2 never ran; only gate.log line 49 and test.log line 42 imply it ("every log runs from the gate on"). The report explains the label for run.log (line 103-105) but not for size.log.
- G11 (observation, log-size.log lines 2 and 41-51). The rerun note and the list of superseded files come from the environment (the printed script's lines 258-274). The invocation line does not show them, and the hashed files are named by basename only, with no root. The log prints the note's text and the hashes, so nothing required is missing.
- G12 (observation, log-test.log line 42). The test log attempts 1 and 2 are "stopped while it ran, after the gate's failure". The header gives no start time or last completed check. The specs-0001 rule on stopped runs covers stops by the environment, so this may fall outside it. Check 1 did not raise it.
- G13 (confirmed, log-scan-pre-check.log).
  - Header F (lines 107-111) supersedes B's "five" (line 101), which answers check 1's F15 in the script itself.
  - It hashes the six siblings at their current bytes (size.log e17979a1 at line 1340) and the size record 7cf00116 (line 1418).
  - Totals: 0 hits, 0 errors, 0 structural failures; RESULT: PASS (lines 1527-1531). 0 check answers were scanned, which fits the report's statement that independent-check-1.md was installed after this run.

## Question 3: does every number and quoted string in the report appear in a primary?

Found in a primary:
- all hashes;
- 478992, 479232, 393216, 85776, 135968, 166180, 30212, 0, 8 and 7, 4940, 3 and 9;
- 1012208, 903196, -109012, 986700, -25508, 1012384, 176;
- the quoted check line (log-size.log line 3621), "80404 ... becomes -85776" (line 2990), "checks not as expected: 1";
- 585, 0 and 2 on each target; 24 added and 0 removed;
- 27 of 27, removed 24, parts 93, plants 9, host and i686 the same for 27, crate 158, tool 43, 0 refused (log-run.log lines 1600-1727 and 2433-2437);
- "no supplying yet" (spec line 677), "six sibling logs" (scan line 107).

Not supported by any primary, or only partly:
- G14 (report-draft.md line 91). "I asked planning before the run": no primary shows the question or the answer.
- G15 (report-draft.md lines 144-146, check 1's F3). "package.rs line 443" and "came in with 770c9dd": nothing in this directory shows either.
- G16 (report-draft.md line 156). "Both pre-check runs PASSED with 0 hits": run 2 is primary (scan lines 1527 and 1531). Run 1 (1f5c33cb) is attested only as RESULT: PASS by check 1's F15. No primary here prints run 1's "0 hits".
- G17 (report-draft.md lines 54-57). The unlogged comparison of attempt 6 with attempt 3 is disclosed as unlogged. Check 1's F9 compared attempt 5, not attempt 6, with attempt 3. The claim is true for attempt 6: I confirmed it (G8), but no log prints it.
- G18 (report-draft.md lines 124-131). This session's two unlogged runs (the render and the count_matches call) are disclosed as unlogged and not cited as measurements. That is acceptable, but no primary holds them.
- G19 (report-draft.md lines 37-43 against "Size"). The report gives the part's own cost, 135968, but does not say in words that it is above planning's 30000 (and above the 25000 expectation). "Size" asks the report to say so. size.log (lines 2988-2989) and the record (lines 116-120) do say it.

Check 1's findings:

| Finding | State now |
| --- | --- |
| F1, F2, F4-F10, F14 | Confirmations; nothing to fix |
| F3 | Stated, with an unsupported claim (G15) |
| F11 | Fixed (G7) |
| F12 | Stated, with a ruling asked; not fixed (G9) |
| F13 | Stated (report lines 103-105) |
| F15 | Not named in the report; answered by scan header F (G13) |
| F16 | Fixed; the log no longer says "error", report lines 68-70 |
| F17 | Stated (lines 84-87) |
| F18 | Fixed (line 150) |
| F19 | Stated (lines 54-57) |
| F20 | Stated (lines 133-140), from spec decision 4; still no record or log names the gap |
| F21 | Stated (lines 147-149) |

Disagreements with the report: G14, G15, G16 and G19 in the report. In the logs: G9 (still open), plus the minor gap G10.