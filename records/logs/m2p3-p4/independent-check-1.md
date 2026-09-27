# Independent check 1, M2 part 3 P4

Written by the implementing session: the checked phase commit is
b728fdf4ea00b7752e2f6d54540a24e0e586887b, HEAD when the check ran. The
checker was a fresh general-purpose subagent given only the directory
<scratch>/p4/check1, holding these files, by sha256:

    9cc5cd27a03a1d1c34efb2beb7806eb844035acc438d2e56b674e1aab16dbc77  diff-24fb07a.patch
    fb882b95329209d1afcb652662a0ba2c3f7331096bbf74d0c3885554c8b65dfc  diff-af1e585.patch
    32049d182952065665913c9c217bc28ad8ffd93c97963952c3b07c25c447311d  diff-b728fdf.patch
    31bc38e5c7f09f93d9d240f07522f9197ea6692853b4abcac73453c3a0731727  diff-commands.txt
    7b9f0eebce996231bbf94a814921b41b5e42113c0ef4638467b649d35f36dc43  diff-doc-e7a6f72.patch
    589e736b8c4467f7dcac4fdc4b31925e440ca4a37fd2439475a0584d74978d60  diff-phase.patch
    9eb2f2145886c6607377e89b44d5f24405da885a870e064b08b9841c7c209f6b  log-gate.log
    fa45f27bb3027c6ceb83a932b0473a2ec65850b19ae5a03299492c3183030484  log-negative.log
    0842d14bf4d14853128062d5770ab9003225d57f80a4854e55122d5c7a16a35f  log-run.log
    8a2a7a82156e148c552155eecc68cb6c38ce21b5f95183f18199f20205d9d483  log-scan-precheck.log
    dd209c5103eb90aa53c76798b9b734f78d3bfe38680443ae9e6be7977dfa3587  log-size.log
    038bcc01031f22d26c8ce5e9ab1a152e0dce2bc4c1481aa687691a96a58336be  log-test.log
    fd2d7fbd7aec5e170cff8cac440822eaa317f0b2635c1c58da7626e8eb382985  m2p3-open-2026-09-27-b728fdf.md
    0809ae662b9ce648b7ffd241066a31c5d210363e806b37f93b9eae78fb6724ca  m2p3-open-2026-09-27-b728fdf.txt
    5c0eb23e79324950ef364e76b4ea9c870e2f91ea45656d59d4005312af12f147  m2p3-p4-delete-columns-2026-09-27-b728fdf.md
    85b14155d28e3b8b7e5fdae7c07d07735e56a9c7f5e5545f7791c69d4bc11fab  m2p3-p4-delete-columns-2026-09-27-b728fdf.txt
    311f9dc9db1034c61cdf812a7564c13e3969af04c5353142258b587f2424d972  m2p3-p4-delete-rows-2026-09-27-b728fdf.md
    78c9cd90723a0a7f631302601a80b164f841cc19539d3bac1c1aa0e096d7d749  m2p3-p4-delete-rows-2026-09-27-b728fdf.txt
    5502365de0fc801811f49ec7498a10245aa958804a924d35dce9bc2f688a9c00  m2p3-p4-format-2026-09-27-b728fdf.md
    400bbb36755456cd90104a3318de99c906bf6758248d9f27131a9b6e4789c6bd  m2p3-p4-format-2026-09-27-b728fdf.txt
    e1ee726b3152a1b29035874a8731ba32f0ad7de6d0263469218c8100a9b9a0e7  m2p3-p4-insert-columns-2026-09-27-b728fdf.md
    f8e6916c53a0fe6e229ba8e305a9864da62db92c2ce8d1e7eec48bc030d14cea  m2p3-p4-insert-columns-2026-09-27-b728fdf.txt
    77ffc6eeb9cc8eb5bebe153b89ad35bb934ec24648891016664924e20ed8773a  m2p3-p4-insert-rows-2026-09-27-b728fdf.md
    e58b1d4fe376fae068833bf6f01e57528b562f9259cf6db72085cd3616050276  m2p3-p4-insert-rows-2026-09-27-b728fdf.txt
    741b9826db5c4eff5007e8cb4b397a5e070b5d40695ca4787c03b7907d234521  report-draft.md
    85375314e5b6586eb6a6eac23497bd692240dd3c218aa8b413c03bca0cf40c6b  size-m2p3-2026-09-27-b728fdf.md
    79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c  specs-0001-bootstrap.md
    fc6acfedc54be1560db90e53b57fc82bbecce360e77e1743c8d0dc4a1587ea2a  specs-0018-rows-and-columns.md
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-scan-precheck.txt

The checker was told that the attempt-1 logs and scripts hashed in gate.log's
and test.log's rerun lines, and the five logs' stderr files, are gone with
the tmpfs scratchpad (specs/0018 P4 ruling 7). The answer below the rule is
the checker's final message, verbatim, taken from its transcript
(sha256 81c613eb9e129ad8aa34f834de69286c4e6f1eda8106691dffaa888a7dd74e95, 9766 bytes).

---

Short answer:
Q1: Yes, mostly. The diff does what P4 asks for and nothing outside it. One small defect: both new probes' usage strings name the wrong probe (F1). A few header claims can't be confirmed from what I was given (F3, F4).
Q2: Yes. Every carried item and ruling 4 to 7 is in a log. The gate's crate figure beside the workspace's is measured again inside test.log, on the host only, not read from gate.log (F7). The final scan.log was not given.
Q3: Every number and quoted hash in report-draft.md appears in a primary. Two statements need fixing: the spec-pin sentence at report lines 7 to 9 is incomplete (F10), and the wording about how ruling 6's count is taken (F12).

The harness put the project's CLAUDE.md and a memory index into my context. I did not use either as evidence.

Inputs: I read only <scratch>/p4/check1. The gate and test attempt-1 logs and scripts are gone, and so are the per-log stderr files; I did not have them and could not. I judged gate.log lines 49 to 52 and test.log lines 42 to 45 (the rerun lines and their hashes) as the record of those attempts. I judged the stderr byte counts each log prints (gate.log 2027 to 2030, test.log 2829 to 2832, negative.log 1244 to 1247, run.log 2800 to 2803, size.log 4256 to 4259) as the record of those stderr files. The scan pre-check's stderr file, stderr-scan-precheck.txt, is present and is 0 bytes.

Which diff I read for each "nothing else":
- **Doc commit:** diff-doc-e7a6f72.patch. It touches exactly ROADMAP.md (the P3 row) and specs/0018 (the P3 rulings and verification added).
- **C1:** diff-24fb07a.patch.
- **C2:** diff-af1e585.patch.
- **C3:** diff-b728fdf.patch.
- **Combined:** diff-phase.patch (e7a6f72..b728fdf, diff-commands.txt line 1).

## Q1: the diff against P4

What P4 asks for (spec lines 746 to 772, rulings 1 to 3 at 1310 to 1332, P3 verification ruling 2 at 1280 to 1291), and where the diff does it:
- **Carried tests, C1:** diff-24fb07a.patch lines 45 to 77 add `a_col_dropped_while_other_cols_survive` and `an_auto_filter_on_the_column_axis_moves_is_cut_and_dropped`. Their expectations follow "The shift rule" (spec 302 to 333).
- **Held-whole sentence on the column axis, C1:** diff-24fb07a.patch lines 26 to 29.
- **Catalogue and limits in the package and cells headers, C2:** diff-af1e585.patch lines 26 to 37 (cells) and 49 to 107 (package). They cover what moves (77 to 85), what stays (86 to 97), the one-shift rule and the two-writes pattern (98 to 102), what is not read back (103 to 104) and the Excel check still to come (105 to 107). gate.log 1566 to 1569 measures that C2 changes only //! lines.
- **Two probes, C3:** the Cargo.toml feature `rowcol = ["emend", "xml", "emend/rowcol"]` and the two [[bin]] entries (diff-b728fdf.patch 21 to 66). gate.log 1574 to 1583 confirms these are the manifest's only non-comment changes, with no lock change.
  - The rowcol probe is the format probe plus additions only. I compared the two sources; the only differences are the header comment, the four extra arguments, `shifts`, `write_package`, `number_cell` and four more output lines.
  - Ruling 2 (merges counted through `Stream`): diff-b728fdf.patch 442 and 466 to 480.
  - Ruling 3 (number cell found by its value): diff-b728fdf.patch 1146 to 1214.

Nothing else is in the diff. gate.log 1425 to 1432 lists 8 changed files at the phase commit. Each code commit stays in its area (gate.log 1498 to 1560). No public first line is added or removed (gate.log 1840 to 1845).

Findings:

- **F1 (defect).** Both new probes' `USAGE` constants begin "usage: probe-emend-fullstyles-write-xml". See diff-b728fdf.patch 104 and 755 (diff-phase.patch 234 and 885). It's a copy slip, printed only on a bad command line, and the report doesn't mention it.
- **F2 (remark).** `edits.column + 1` and `edits.row + 1` (diff-phase.patch 509, 515, 1189, 1195) are unchecked u32 additions on command-line values. At u32::MAX a debug build panics and a release build wraps to 0. The size log never passes such a value.
- **F3 (remark).** Two claims in the diff can't be checked against any primary I have:
  - "a column shift streams `<cols>`" (diff-24fb07a.patch 27; C1's commit message says the same). No log I was given measures it, and the code is not in this diff.
  - "[`Area::cells`] saturates at `u64::MAX`" (diff-af1e585.patch 33). No test in the diff pins it.
- **F4 (gap).** The C2 bullets on insertions and deletions (diff-af1e585.patch 77 to 97) don't state the formula-change rule for a shift: the calc chain is dropped and fullCalcOnLoad is set when an `<f>` moves (spec 382 to 387). The merge bullet at 71 states it for merges. It may already sit in the parts of the header written in P3 (ruling 4 of the P3 verification, spec 1293 to 1294), but the diffs I was given don't show that.
- **F5 (remark).** diff-phase.patch is e7a6f72..b728fdf (diff-commands.txt line 1). specs/0001 lines 48 to 50 define the phase diff from the previous record commit, 3cd9696. Together with diff-doc-e7a6f72.patch the content is the same. But gate.log 1421 to 1422 hashes the 3cd9696..b728fdf diff, so the given file can't be compared with that hash.

## Q2: each required item and the log that carries it

- **Pairs-times-two control that runs every time:** run.log lines 28 to 31 (planted listings). The real check is at 2231 to 2232: 336 .in files, 336 .out files, 672 files.
- **Moved run directory listing control that runs every time:** run.log 32 to 34. Line 27 shows nothing was superseded this run, so only the controls ran.
- **Clippy and doc with the feature off in the gate:** gate.log 1913 to 1924, off and on. The probe crate's three feature sets follow at 1925 to 1934.
- **Commit-message count rule on every commit, doc commit included:** gate.log 1364 to 1379 (e7a6f72 at 1364 to 1365). Also scan pre-check 1523 to 1537 ("4 of 4").
- **Test log's crate-binary count beside the gate's, the difference put down to dates:** test.log 2507 to 2522. The workspace run is 559 and the crate with rowcol 545; test names under dates number 14, and 559 less 545 is 14. gate.log 1945 shows 545.
  - **F7 (remark).** The figure test.log labels "the gate's figure" is its own re-measurement, not a value read from gate.log (script at test.log 427 to 431). It is taken on the host only. The two values agree.
- **Repeated `set -uo pipefail` lines cleaned:** one line in each printed script: gate.log 144, test.log 110, negative.log 144, run.log 129, size.log 160, scan pre-check 164.
- **Ruling 1:** size.log 2859 to 2865 (16 measured, 14 compared, 2 new). Record lines 53 to 79.
- **Ruling 3:** size.log 3974 to 3978 and 4039 to 4040. The number cell moves one row down and then one column left and is not deleted.
- **Rulings 4 and 5:** negative.log 850 (14 constants) and 1033 to 1047 (F8 refused, F9 as its control).
  - Table, cuts and off-grid on rows and columns: R1, R3, R5, C1, C3, C5, in the checks at 1145 to 1185.
  - The sets: 1195 to 1216 (8 reached, 6 unreachable with reasons, 3 at-call).
  - The spec pin: 51.
- **Ruling 6:** scan pre-check 1859 to 1864. It prints the sha256 key and the count, with three controls.
  - **F8 (gap).** Only the pre-check was given. Whether the final scan.log repeats this, and scans the check answers, can't be judged here.
- **Ruling 7:** gate.log 49 to 52 and test.log 42 to 45 give the reason and the attempt-1 hashes. Those files are gone (see Inputs).
- **Records and figures:**
  - Size record installed and hashed: size.log 4240 to 4244.
  - Release binaries hashed: size.log 4062 to 4067, then matched in run.log 1290 to 1305.
  - All five samples, 27 of 27 byte-identical on each target: run.log 1478 and 1523, and the matching lines for the other four samples up to 2075.
  - Open check: run.log 2743 to 2748.
- **F9 (remark).** test.log prints no spec hash (neither 52fcf673 nor e2a69b86 appears in it). The per-phase rules don't name one for the test log, so this is only an inconsistency with the other four logs.

## Q3: the report's numbers and strings

Every sha256 in report lines 15 to 19 matches the file (I computed them) and scan pre-check 1551 to 1555. The attempts match gate.log 49 and test.log 42. Every figure in lines 41 to 50 is in size.log 3334 to 3374 and the record, lines 95 to 126. Lines 54 to 63 match run.log 1478 to 2075 and 2743 to 2746, test.log 1702 and 2456, and negative.log 850 and 1212. fc6acfed is at scan pre-check 1762 and 1774.

- **F10 (defect).** Report lines 7 to 9 say gate.log pins the committed blob e2a69b86 and that "negative, size and run pin 52fcf673". gate.log also pins 52fcf673 for the uncommitted spec (gate.log 2016 to 2022). test.log pins neither hash. The sentence should name both of the gate's pins and state that test.log pins none.
- **F11 (remark).** Report line 69 says the checks get "the phase diff e7a6f72..b728fdf". This matches diff-commands.txt, but see F5: the gate's phase diff hash is over 3cd9696..b728fdf.
- **F12 (remark).** Report lines 36 and 37 say the scan counts lines "as count_matches does". That is true (scan pre-check 1037 to 1042). But the same log's placeholder rule says "Tokens are counted, never their lines" (scan pre-check 1854 to 1855). The exemption counts lines instead.
  - Size.log carries the token on 2 lines (968 and 3351), twice on each, so 4 occurrences against ruling 6's "expected count 2".
  - The sha256 key already blocks any added occurrence, so nothing escapes the check.
  - The report should say that the line count departs from the log's own rule and why, rather than calling it the same form.
- **F13 (remark).** "Nothing pushed" (report line 6) and the future items (scan.log "in the final report" at line 20, the check files at 69) have no primary in the given set. They are statements, not numbers.