# M7 part 2 P1 logs

specs/0045-publish.md, "P1", and planning's P1 brief. P1's doc commit
bb7c35f committed the spec at sha256
f2adb17a7421940ca97270dcc9cbc65586d74c0b14559e240bfb0137d7f25678,
ROADMAP.md at sha256
3106f10d4bf6f7ad3fa356cb5b8460ab62b24ced9a8cbb32767eb83f364c25bb and
specs/0044-excel-check.md at sha256
4522b5aaa2c1d446b210d50f82e683880553819b05b6e254aec4c1c8c5b66ffe.
Planning's rulings A to L, written during P1, are in the working
specs/0045, uncommitted. The gate and run logs took it at PLANNING_SPEC:
sha256 cf085c08a528ea3d53e24eb379c14bbe751011368af52eb2a511a751cbafc300
(606 lines). The negative and size logs, run again under ruling L,
took it at sha256
866c16cee699f81ed08b2a6a3c76589a246c0c649247b03466fa190214a59e93
(645 lines). It took the working ROADMAP.md at PLANNING_ROADMAP: sha256
0af809f25d889e588fc0ed763f8dcbb2a2d1eca15c49fbe22bafca3aeff33ac1. The
phase commit is 43471ed8d83a79c057532fb5c60e253941bc7137. `<phase>` is
the phase's directory, `<home>/.cache/emend-build/m7p2-p1`.

root: <dry> stands for <phase>/dry, the phase's dry runs' directory (specs/0043 ruling Q)
root: <superseded> stands for <phase>/superseded, the phase's superseded directory
root: <session> stands for the harness's session directory under <home>, where the checks' tool outputs were saved; not named here

The phase's commits sit on part 1's close-out record commit 658dbc0:

| Role | Commit | Subject |
|---|---|---|
| doc | bb7c35f | M7 part 2 P1: commit planning docs (the part 2 spec, part 1's close-out verification) |
| fix 1 | 4a2582b | M7 part 2 P1: an of-pie with no split writes no splitType |
| fix 2 | f79e447 | M7 part 2 P1: a created workbook element goes before the next main child |
| fix 3 | 64f9062 | M7 part 2 P1: a new scatter plot of the style none is refused |
| fix 4 | 35e3d1d | M7 part 2 P1: a chartsheet keeps its chart |
| fix 5 | 5d14c89 | M7 part 2 P1: a moving-average period is 2 to 255 |
| docs (phase) | 43471ed | M7 part 2 P1: the README and the CHANGELOG name the three limits |

| Log | Last line | sha256 |
|---|---|---|
| gate.log | gate log checks all as expected: yes | f5d7ee389533377babbcc005cbaa0ca77adbd1a1307c0dbcb8f44d53340880ce |
| negative.log | negative log checks all as expected: yes | e805474576bfecce43a3da0941d6d2cae8d12a95a924d58c801870fff821c456 |
| run.log | run log checks all as expected: yes | 30be6eadc25f34d24422135e576e459c2ba0c7e3630f069bdb689b4fc7756102 |
| size.log | size log checks all as expected: yes | 872d5eda949ad73d8ef1cef6b50f92d63cae1f6e535626c388aa9d72043d529c |

The scan's log, scan.log, is written last. It covers these four logs,
this README, the two checks' answers and the 15 record files the run and
the size log installed.

## Rulings in the working spec

- A: on a chartsheet alone, a removal plus an addition in one write
  replace the chart; the two chartsheet tests renamed to what they now
  show.
- B: the moving-average period's message.
- C: 8 of part 1's finding 5 pairs re-kept; the other 24 allowed under
  the documented precondition.
- D: the plot checks refuse "at the call".
- E: the workbook insertion rule.
- F: the ROADMAP row's slip.
- G: new pairs from this part's tests expected; the formula and protect
  outputs may change bytes, their counts may not.
- H: the gate's environment check stands; d2's seven readings accepted.
  Among them: the scatter style test's .6 gone; PLANNING_ROADMAP; the
  size readings; the docs commit classed as code; the gate fork's cargo
  runs, disclosed below.
- I: after d3. The refused trendline field test's new keys .46, .47 and
  .48 are expected by key when open ok (change TJ). Change TI accepted.
- J: after d4. Change SF accepted. The core's 16 byte shrink is
  attributed to fix 2 by planning's reading of the package.rs changes,
  not by a measurement. The eight size FINDING lines are recorded, not
  fixed, and closed by this ruling.
- K: planning's word for the logged run on d5's conditions.
- L: after the logged run's first independent check. Its three gaps
  hold:
  - (a) the negative log showed fixes 3 to 5's refusals only as the
    tests passing by name;
  - (b) K3 had no control;
  - (c) the size record lacked ruling J's attribution, and one of its
    sentences named four commits as the five fix commits.

  Option B: changes NJ64 (negative) and SG (size); the negative and
  size logs run again, logged; attempt 1 moved to
  `<phase>/logged/superseded/`; a second and last independent check.

## What the logs show

- The gate (part 1 P2's gate, its changes continuing to the *91
  labels), 2026-10-27T22:02:20Z to 22:09:20Z:
  - the root's mount propagation shared, one keepalive process;
  - the roles by full subject, in order, the docs commit the phase
    commit;
  - the prune (ruling K, specs/0044 ruling AD form, with its control):
    `<home>/.cache/emend-build/m7p1-p2/target`, 6215682904 bytes, and
    `<home>/.cache/emend-build/m7p1-close/target`, 2144181980 bytes,
    removed by `rm -r`;
  - check.sh nine PASS lines;
  - the release builds the run reads: emend-copy sha256
    9950593b375f5d33f1304c73efbe186119e4185f53e27420ecabb2377a8e88d7,
    3957792 bytes; xlsx-diff sha256
    1445523df9522bff56d4f0e03d7bde2ab333bac294020da256de42c01ef71f6b,
    897296 bytes.
- The negative (attempt 2, ruling L), 2026-10-28T01:37:28Z to
  01:37:52Z:
  - K1 to K5, one per fix, each with its control; K3's control (change
    NJ64) is the scatter style test run alone with its own keep, whose
    marker output holds xl/charts/chart1.xml;
  - R3 to R5 (change NJ64), each refusal read from its test at the
    phase commit: the message, the refusing and accepting calls by
    their exact lines in source order, the exact test's pass line, and
    a control on a planted copy of the body:
    - R3: None refused at add_chart and add_chartsheet, Marker taken;
    - R4: the chartsheet removal refused alone, taken with an addition;
    - R5: the periods 0, 1, 256, 1000 and 65535 refused, 2 and 255
      taken;
  - all yes; FINDING lines 0.
- The run, 22:09:42Z to 2026-10-28T00:56:48Z:
  - cargo test --workspace on the host and on i686-unknown-linux-musl:
    passed 1452, failed 0, ignored 2 each;
  - the six samples rerun (formula, chart-surface, protect, edit,
    remove, chartsheet): "reruns done 6 of 6; record pairs written 6;
    FINDING lines so far: 0". Each host row and identical count equals
    its part 1 record's;
  - the keep: inputs 2251, outputs 2251; the open record
    `records/m7p2-open-2026-10-27-43471ed`: pairs 2251, lo_in ok 2239,
    lo ok 2240, findings 0;
  - against part 1's open record: pairs then 2241, now 2251; "gone as
    expected 7, gone otherwise 0; new as expected 17, new otherwise 0";
    open results leaving ok 0, becoming ok 0;
  - ruling C: the 6 re-kept pairs still named each differ from part 1's
    output; the 2 renamed are gone, and the one-write test's 3 pairs
    replace them; the pairs allowed are 24;
  - "FINDING lines printed above: 0";
  - 14 record files installed (the samples' 12 and the open record's
    pair).
- The size log (attempt 2, ruling L), 01:37:52Z to 01:43:25Z:
  - the budget figure 451512 section bytes, 7240 under the ceiling
    458752; M6 part 6's record's 451528, this one less it -16;
  - the core's movement: sections -16, file 0, .text -16;
  - the attribution beside it (change SG), size.log 11069; in the
    record, line 142: "This movement is attributed by planning's
    reading to fix 2 (f79e447), the one change to code the core links
    (`specs/0045` ruling J), not by a measurement.";
  - the record's cause sentence, line 340, keeps the two lists apart:
    "5d14c89, 35e3d1d, 64f9062, f79e447; the part's five fix commits
    are 4a2582b f79e447 64f9062 35e3d1d 5d14c89.";
  - the eight FINDING lines, size.log 11195 to 11202 (the record's
    copy at 13618 to 13625): seven probes linking the core -16 (rowcol
    -32), and the core movement -16. All are ruling J's;
  - the record `records/size-m7p2-2026-10-28-43471ed.md`. It is dated
    2026-10-28 because the size step started after midnight UTC.

## The session's statements (no log prints them)

- Generators and scripts, sha256:

  | Log | Generator | Script |
  |---|---|---|
  | gate | 0f96ebf9c56ae5702d5588ccf0fbc45ed71c34a0b56449b3ff6c992e90a38cd5 | 31f99755a2c710b4e6defa30951014dd9da781c7719c67da13ab996c505cd45d |
  | negative | 4c8afa0bac693e6f5b5fccfb563115bf1ebdfecc6135ef203daa969eecc0197f | b3e84f4c17e0e2e0ffec85d487379ae791516c9720d265454ee63cc58bc2d05d |
  | run | f9463a7b1a3f9d4111ed26f7d52ab44ba10b1297d2dd71d1adae39facbc3ace4 | 57493ed825a7e3f969edb4150e1280286139bad9b16ce74514b9f57929ca7e88 |
  | size | 7cd2e497a2413e9634bac804d61a2dafabbaa54b265068ca0ab37ad96aae2921 | e6a4b83e73050f7d93dc9c442a27c2b078723820580918c8aa2271b7ad4f7a8e |
  | scan | 0686352fa53383adc88279260564f6b57a518c89451bdca3d5d1663115d6169f | 8153c44dbbd2e01e28b044f4d04d05932f4f00f1b7086f3dcbdb20ee539890df |

  No generator pins PLANNING_SPEC; the drivers pass it in, so the
  scripts did not change for rulings I to K. Ruling L changed the
  negative (NJ64) and the size log (SG); attempt 1's scripts were
  negative 77f2e612177e6e7aa3e18402ea18a999ddacbf1ca79a7cd1d679c810eb6b3214
  and size 0bd27600b57af4e0b275dbbfb6ee51477e8d33d9b0ba2b2844ec803cd70cefc0,
  kept under `<superseded>/negative-gen2` and `size-gen3`. The drivers:
  - `<phase>/logged.sh`, sha256
    04cc7f40472045115a7f2a468bb88c6be47fec9fea0c16cfd3e3b11ae471b8c5;
  - `<phase>/logged2.sh` (ruling L's rerun), sha256
    8bb25d9bc0937f0ab2073703171e488c3aca75d2ce5816f4cb16670df23c4d6f.

- Disclosures (ruling H):
  - The environment. Before d1 the WSL instance had restarted: the
    root's mount propagation read private and no keepalive was running
    (d1's gate.log lines 19 to 25). The session restarted the keepalive
    by starting the Windows scheduled task 'WSL keepalive'. The
    session's `sudo mount --make-rshared /` was refused, and the
    session did not work around it. The maintainer ran the mount as
    root from Windows. From d2 on, every gate reads propagation shared
    and one keepalive.
  - The gate fork's cargo runs (ruling H (7)). While writing the gate
    generator, the fork confirmed its expected per-set test counts with
    one `cargo test -p emend` per feature set on the host into
    `<home>/.cache/emend-build/dev-target`. That was against the phase's
    "forks do not run cargo" rule. No phase target and no log was
    touched.

- The dry runs, each from the start under `<dry>/d<n>`, each kept:
  - d1 (PLANNING_SPEC 42763519...): the gate exited 1 on the
    environment lines alone (propagation private, keepalive 0). The
    negative and run did not start.
  - d2 (the same spec): gate yes; the negative exited 1 on NC64, "each
    of the three messages is at the phase commit once, absent at the
    base, and in the committed spec's words": the spec wraps the
    messages across lines. Change NI64 compares with whitespace
    collapsed; the earlier negative script is kept under
    `<superseded>/negative-gen1`.
  - d3 (the same spec, NI64): gate and negative yes; the run exited 1
    on two checks, "each of the six samples is named once by the
    binary's usage text" and "the 8 re-kept pairs of ruling C are each
    one of finding 5's names", plus three FINDING lines (the refused
    trendline field test's .46 to .48). Cause of the two: the calls
    `count_matches -xF -- <key> <file>`, where `--` was read as the
    pattern. Change TI drops the `--` and adds two controls that admit;
    ruling I and change TJ cover the three keys. The earlier run
    scripts are kept under `<superseded>/run-gen2` and `run-gen3`.
  - d4 (PLANNING_SPEC 4c2780e7..., TI and TJ): gate, negative and run
    yes. The size log exited 1 on "TMPDIR is exported as <scratch>/tmp
    (change NX)": change SE set TMPDIR from SIZE_TMP but left this
    check comparing with the default. Change SF compares with the
    directory in effect. The earlier size script is kept under
    `<superseded>/size-gen2`. Its eight FINDING lines are ruling J's.
  - d5 (PLANNING_SPEC 6ae521c3..., SF): gate, negative, run and size
    yes. The scan pre-check reads RESULT: FAIL on its four dry-layout
    checks alone (LOGDIR and RECDIR not the defaults; 2 of 6 run
    directories; records 7 of 15; 52 output lines holding the home
    directory, every one under `<dry>/d5`), with total hits 0.

- The logged run (PLANNING_SPEC cf085c08..., under ruling K) started
  2026-10-27T22:02:20Z by the driver; gate.log's own start line reads
  22:02:22Z. Gate, negative, run and size each exited 0 and read "yes",
  with 0 bytes of outer standard error each. Its scan pre-check read
  RESULT: PASS (hits 0, errors 0, structural failures 0).
- Attempt 1 (ruling L (3)): attempt 1's files were moved to
  `<phase>/logged/superseded/` before the rerun, with their outer
  standard error and the negative's build root
  (`<phase>/logged/superseded/negative-build`):
  - negative.log, sha256
    6727ff5b9e978b2853745904a93cfb2cf0f9565752ca2f92cf8227f214c4879d;
  - size.log, sha256
    627f5141b75922b4aa824a56d2ed060a572126b1defa0dc50bd7e6519e177a59;
  - size-m7p2-2026-10-28-43471ed.md, sha256
    56c7ae20539f0f0104debe08f1e52f95485130a8088e46e0f74ea534863bf916.

  The cause is check 1's gaps (a) to (c), ruled on in ruling L; the
  changes are NJ64 and SG.
- Dry run d6 (PLANNING_SPEC 866c16ce...), the regenerated negative
  and size alone, under `<dry>/d6`: both exit 0 and "yes".
- The rerun (`<phase>/logged2.sh`, PLANNING_SPEC 866c16ce...):
  negative and size each exited 0 and read "yes", with 0 bytes of outer
  standard error each.

## The independent checks

- Check 1 (`independent-check-1.md`) read the spec, the diff, the draft
  report, the four attempt-1 logs, this README as it stood then and the
  scan pre-check. Verdicts: 1 yes; 2 yes with exceptions; 3 yes with
  exceptions.
  - Its gaps (a) to (c) went to planning, became ruling L, and were
    fixed by the rerun above.
  - The report takes its minor points: both gate times; "M6 part 6's
    record's 451528"; d5's "RESULT: FAIL on its four dry-layout checks
    alone"; the driver's figures marked as the session's statements.
- Check 2 (`independent-check-2.md`), the phase's second and last,
  read the regenerated negative and size logs, the size record and the
  revised report alone. Its brief is `<phase>/check2-brief.txt`, sha256
  fd9afbbeffa51b075ca861d444644dc6f4175047d8c6ce6c37766a7bc3e40482. The
  report it read is `<phase>/logged/report-for-check2.md`. Verdicts:
  1 yes with exceptions; 2 yes; 3 yes with exceptions. It found check 1's
  gaps closed, every check in both logs yes, and no wrong number or
  misquote. What was done with its points:
  - E1 to E5 (how strict the new readings are: K3's control uses
    another reading than K3's own; R3's add_chartsheet refusal and R5's
    `refused(` call read through other lines; R4's accepted half and
    its `refused_after` line without a control of their own; the pass
    line printed as a count): recorded, not fixed. Ruling L makes check
    2 the last, and nothing was rerun. The report's line on R3 to R5
    names E2 and E3.
  - Its side note on the size record's template wording (the
    `metadata` feature named in its opening): template text, recorded.
  - Its report points i to iii: the report gives negative.log's own
    start, 01:37:29Z, beside the driver's 01:37:28Z. It marks the
    script hashes, attempt 1's hashes and d6 as the session's
    statements, and narrows the R3 to R5 wording.

  No code or log changed after check 2; the final report is
  `<phase>/report.md`.
- The check files. Each is a heading line, then its answer verbatim,
  with two edits named in its heading: the home directory's path is
  written <home>, and the harness session directory under it is
  written <session>. The heading gives the sha256 of the answer as
  given, kept under `<phase>/logged/`.
- The final scan's first run read RESULT: FAIL, total hits 2, one
  structural check failed. Its two hits were the harness session path
  that each check's answer quoted, matched by the privacy patterns
  (form f_p6). The <session> placeholder above was then written into
  both check files, and the final scan ran again; scan.log is that
  second run. The first run's log is kept under `<phase>/logged/`
  (scan-final-1.log).
