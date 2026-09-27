# Independent check 2, M2 part 3 P4

Written by the implementing session: the checked phase commit is
b728fdf4ea00b7752e2f6d54540a24e0e586887b, HEAD when the check ran. The
checker was a fresh general-purpose subagent given only the directory
<scratch>/p4/check2, holding these files, by sha256:

    9cc5cd27a03a1d1c34efb2beb7806eb844035acc438d2e56b674e1aab16dbc77  diff-24fb07a.patch
    fb882b95329209d1afcb652662a0ba2c3f7331096bbf74d0c3885554c8b65dfc  diff-af1e585.patch
    32049d182952065665913c9c217bc28ad8ffd93c97963952c3b07c25c447311d  diff-b728fdf.patch
    6fbcfa511b10baa299125a58f2fc5fec33ab26b807949412b1dfe4a691d28d26  diff-commands.txt
    7b9f0eebce996231bbf94a814921b41b5e42113c0ef4638467b649d35f36dc43  diff-doc-e7a6f72.patch
    b6873579667232bd5c498f09a7cdc67910f7a411625a918e802c6d0a6b43d84c  diff-phase-from-3cd9696.patch
    589e736b8c4467f7dcac4fdc4b31925e440ca4a37fd2439475a0584d74978d60  diff-phase.patch
    d10d73c36ad914c8e198bed56c731f5d9e2f29765003103ed434fa92f60ec2c0  independent-check-1.md
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
    8981e7d055cb291aab5e0a30c7461a9e959bbb2b19728bef210705c81e556ec8  report-draft-2.md
    85375314e5b6586eb6a6eac23497bd692240dd3c218aa8b413c03bca0cf40c6b  size-m2p3-2026-09-27-b728fdf.md
    79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c  specs-0001-bootstrap.md
    fc6acfedc54be1560db90e53b57fc82bbecce360e77e1743c8d0dc4a1587ea2a  specs-0018-rows-and-columns.md
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-scan-precheck.txt

The checker was told that the attempt-1 logs and scripts hashed in gate.log's
and test.log's rerun lines, and the five logs' stderr files, are gone with
the tmpfs scratchpad (specs/0018 P4 ruling 7). This is the phase's second
and last check. The answer below the rule is the checker's final message,
verbatim, taken from its transcript
(sha256 ff19acdd51822bf352cb42ba673e6638b545f5de2543b88e2d1e3974c867be03, 12732 bytes).

---

Short answer:
Q1: Yes, apart from one defect. The diff does what P4 asks and nothing else, but both new probes' USAGE strings name the wrong probe (G1). The lines P4 adds to the header leave out some refusals and the formula-change rule, and whether the earlier header parts state them cannot be seen in any diff given (G2).
Q2: Yes. Every P4 item, rulings 1 and 3 to 7 and the P3 verification ruling 2 carried items is in a log, with two limits. The final scan.log is not in the set, and the attempt-1 files that gate.log and test.log hash are gone (G7, G15).
Q3: Yes, with one qualification. Every number and hash in report-draft-2.md is in a primary or follows from one. The "4 tokens" reasoning at report lines 38 to 43 is weaker than it reads (G3), and a few statements have no primary in the set (G8, G9).

The harness put the project's CLAUDE.md and a memory index into my context. I did not use either as evidence.

Inputs: I read only <scratch>/p4/check2. I recomputed the sha256 of every file there. Each one checked by independent check 1 matches the value in its header (independent-check-1.md lines 8 to 36). report-draft-2.md, independent-check-1.md, diff-commands.txt and diff-phase-from-3cd9696.patch are new or changed for this check.

Under P4 ruling 7, the attempt-1 logs and scripts that gate.log lines 49 to 52 and test.log lines 42 to 45 hash are gone, and so are the five logs' stderr files. I did not have them and could not. I judged those rerun lines as the record of the attempt-1 files, and each log's own stderr byte count as the record of its stderr file:
- gate.log 2027 to 2030
- test.log 2829 to 2832
- negative.log 1244 to 1247
- run.log 2800 to 2803
- size.log 4256 to 4259
- scan pre-check 1880 to 1882 (its stderr file, stderr-scan-precheck.txt, is present and 0 bytes)

Which diff I read for each "nothing else":
- **Doc commit:** diff-doc-e7a6f72.patch.
- **C1:** diff-24fb07a.patch.
- **C2:** diff-af1e585.patch.
- **C3:** diff-b728fdf.patch.
- **The phase as specs/0001 lines 48 to 50 define it:** diff-phase-from-3cd9696.patch.
  - Its sha256 is b6873579667232bd5c498f09a7cdc67910f7a411625a918e802c6d0a6b43d84c, the value gate.log line 1421 prints.
  - I split it per file and compared each section with diff-doc-e7a6f72.patch and diff-phase.patch. All 8 sections are equal.

## Q1: the diff against P4

What P4 asks for is at spec lines 746 to 772. P4 rulings 1 to 3 are at 1305 to 1332. P3 verification ruling 2 (the code items) is at 1274 to 1278.

- **Doc commit:** ROADMAP.md and specs/0018 only (diff-doc-e7a6f72.patch 13 and 26). gate.log 1382 to 1409 matches both files to the hashes planning gave.
- **C1:** the two carried tests (diff-24fb07a.patch 45 to 77) and the held-whole sentence extended to the column axis (26 to 29).
  - I checked both tests' expectations against "The shift rule" (spec 302 to 320). They agree: dropping column 3 moves the 5-6 interval to 4-5, and the autoFilter moves, is cut and is dropped.
- **C2:** module-header lines only.
  - The cells header is at diff-af1e585.patch 26 to 37.
  - The package header is at 49 to 107: what moves (77 to 85), what stays (86 to 96), the one-shift rule and the two-writes pattern (97 to 101), what is not read back (102 to 104), the Excel check still to come (105 to 107).
  - gate.log 1498 to 1560 holds each commit to its area.
- **C3:** the probe crate's `rowcol` feature and two [[bin]] entries (diff-b728fdf.patch 21 to 66), and the two probe sources.
  - Ruling 2, merges counted through the crate's own `Stream`: `count_named` at diff-b728fdf.patch 1217 onward.
  - Ruling 3, the number cell found by its value and read back after the fourth and fifth writes: `shifts` and `number_cell` at 1162 to 1214.
  - Indices are 1-based, as spec line 194 says. size.log 3860 passes column 2 and row 2, and 3946 passes shift row 1 and column 1. The moves printed at size.log 3976, 3978, 4039 and 4040 agree with that.
  - size.log 2821 builds the rowcol probe alone with `--features rowcol`, as "Size" asks.

Nothing else is in the diff: gate.log 1426 to 1434 lists 8 files, the same 8 as the combined diff.

- **G1 (defect; confirms check 1 F1).** Both new probes' USAGE constants name probe-emend-fullstyles-write-xml:
  - diff-b728fdf.patch 104 and 755
  - diff-phase-from-3cd9696.patch 376 and 1027

  The report's "line 32" and "line 34" (report line 75) follow from the hunk headers at diff-b728fdf.patch 72 and 721.

- **G2 (gap; extends check 1 F4, which check 1 left partly open).**
  - The new "what moves" bullet lists "the shared, array and data table formulas' `ref`" and then says "Each is moved, grown, cut, clipped to the grid or dropped as "The shift rule" gives" (diff-af1e585.patch 81 to 85).
  - The spec refuses such a `ref` when the rule would grow, cut or remove it (spec 330 to 334).
  - The lines P4 adds state neither that refusal nor the off-grid refusal (spec 323 to 325). They also leave out three things the spec says the module header states:
    - the formula-change rule (spec 382 to 386; check 1 F4 and report lines 79 to 82)
    - P2 ruling 3's kept hyperlink relationship (spec 1005 to 1007, "the limits say so")
    - P3 ruling 1's malformed `<col>` under a shift (spec 1193 to 1200, "The module header says so")
  - P3 verification ruling 4 (spec 1289 to 1290) says parts of the catalogue were written ahead of P4 in b675f3d and c3b5bbd. Those parts are in no diff given, so I cannot confirm or deny that they carry these rules.
  - The report's "verified by me in the tree" (line 71) covers only the formula-change rule. The word "grown" read against the formula `ref` is at best ambiguous.
- **G3 (remark; confirms check 1 F2).** `edits.column + 1` and `edits.row + 1` are unchecked u32 additions (diff-phase-from-3cd9696.patch 651, 657, 1331 and 1337). The size log passes 2 and 2 (size.log 3860).
- **G4 (remark; confirms check 1 F3).** "a column shift streams `<cols>`" (diff-24fb07a.patch 27, and C1's message at line 12) is still not measured by any log in the set.

## Q2: each required item and the log that carries it

- **P4, the size record:**
  - Sixteen probes measured, fourteen classified, two new (ruling 1): size.log 2859 to 2865, and 3264 to 3330 for the classifications.
  - The figures "Size" asks for: 3331 to 3345.
  - The judgment of the ceiling as a fact: 3334, and 4079. The record states it at line 105.
  - The whole growth split into the part's own cost and the drift: 3373 to 3377.
  - The record installed and hashed: 4240 to 4245.
  - Release binaries hashed: 4062 to 4073.
  - Ruling 3's moves: 4039 to 4041, with the output check at 4012 and its control after it.
- **P4, the corpus run:**
  - Release binaries matched to size.log: run.log 1289 to 1305.
  - Five run directories: run.log 46 and 47 to 51.
  - 27 of 27 outputs byte-identical on host and i686 for each sample: run.log 1478 and 1523 for format, 1616 and 1661 for insert-rows, 1754 onward for the rest.
  - Six record pairs installed and hashed: for example run.log 2115 to 2118, and 2768 to 2776.
  - The open check on the host keep: run.log 2743 to 2748.
- **P4 rulings 4 and 5 (negative.log):**
  - The source block: 14 constants (negative.log 850).
  - F8 refused with the index message and F9 as its control: 1033 to 1045 and 1161 to 1163.
  - The doubled-element message three ways (F2, F3, F4), cols disorder (F1) and merge overlap (F6): checks at 1143 to 1160.
  - The table, cuts and off-grid messages on rows and columns (R1, R3, R5, C1, C3, C5): 1164 to 1191.
  - The range-unparsed message through a dimension (R7): 1174.
  - The sets: 1195 to 1216. 8 reached and 6 unreachable with reasons, 3 of them at-call.
  - The spec pinned at 52fcf673: 51.
- **P4 ruling 6:** scan pre-check 1859 to 1866. It prints the sha256 key and the line count, with three controls.
- **P4 ruling 7:** gate.log 49 to 52 and test.log 42 to 45, the reasons and the attempt-1 hashes. Test attempt 1 did fail one check (test.log 42). Gate attempt 1 did not (gate.log 49).
- **P3 verification ruling 2, carried to the scripts:**
  - The pairs-times-two control, run every time: run.log 28 to 31. The real check is at 2231 and 2232 (336 plus 336).
  - The moved run-directory listing control, run every time: run.log 32 to 34. Line 27 shows nothing was superseded.
  - Clippy and doc with the feature off in the gate: gate.log 1913 to 1924.
  - The count rule on every commit, the doc commit included: gate.log 1366 to 1373, and scan pre-check 1528 to 1537.
  - The workspace's crate-binary count beside the gate's, with the difference put down to dates: test.log 2516 to 2520, and gate.log 1945.
  - One `set -uo pipefail` line per printed script: gate.log 144, test.log 110, negative.log 144, run.log 129, size.log 160, scan pre-check 164.
- **P3 verification ruling 2, the code items:** C1 above. The test.log plants U1 and U2 show both new tests fail when planted (test.log 2774 to 2804).

Findings for Q2:

- **G5 (remark; confirms check 1 F7).** test.log 2516 calls 545 "the gate's figure", but the script measures it itself (test.log 431) rather than reading gate.log. gate.log 1945 prints the same 545.
- **G6 (remark; check 1 F9 stands).** test.log pins no spec hash.
- **G7 (remark, ruling 7).** The attempt-1 files named at gate.log 51 and 52 and test.log 44 and 45 are gone. Planning cannot recompute those four sha256s, and neither could I; the lines are the only record.
- **G15 (gap; check 1 F8 stands).** The final scan.log is not in the set. Whether it repeats pre-check lines 1859 to 1866 and scans the two check answers (pre-check 1601 and 1602 read 0 at this stage) cannot be judged here.

## Q3: the report's numbers and strings

- **Log hashes:** the sha256s at report lines 17 to 21 match the files and scan pre-check 1551, 1561, 1571, 1581 and 1591. The attempts match gate.log 49 and test.log 42.
- **Spec pins:**
  - fc6acfed: scan pre-check 1762 and 1774.
  - e2a69b86: gate.log 1385 and 1395.
  - 52fcf673: gate.log 2020, negative.log 51, size.log 54 and 4254, run.log 1282 and 2796.
  - "before rulings 6 and 7": scan pre-check 143 and 163, and size.log 103.
  - Check 1 F10 is fixed.
- **b6873579:** gate.log 1421, and my own hash of the file.
- **Figures (report lines 47 to 56):** size.log 3331 to 3377, and record lines 95 to 119. The arithmetic holds:
  - 967884 - 533216 = 434668
  - 458752 - 434668 = 24084
  - 434668 - 399176 = 35492, which is 4296 + 31196
- **Run and test counts (report lines 60 to 69):** run.log 1478 to 2075 and 2743 to 2746; test.log 1702 and 2456; negative.log 850 and 1212.
- **Quoted strings:**
  - "expected count 2": spec 1370 and 1371.
  - "changes AC to AF": scan pre-check 125 to 163.
  - "usage: probe-emend-fullstyles-write-xml": diff-b728fdf.patch 104 and 755.

Findings for Q3:

- **G3a (remark; disagrees in part with the report at lines 38 to 43 and with check 1 F12).**
  - size.log carries the ruling 6 token 4 times on 2 lines (968 and 3351). On each line the first occurrence is followed by a space and a pipe, not a slash. Only the second is in the placeholder form the scan defines (scan pre-check 1022 and 1023).
  - Counted as the placeholder block counts, tokens in the placeholder form (pre-check 1853 to 1855), the figure is 2, which is ruling 6's count.
  - The exemption's line count (pre-check 1040 to 1049) happens to give the same 2. The departure the report describes is one of method, not of number. The "4 tokens" figure counts two occurrences that are not in the placeholder form.
- **G8 (remark).** Some statements have no primary in the set:
  - "my message to planning" (line 42), "as planning's message set it" (line 85) and "planning's message named..." (lines 96 and 97).
  - Line 95 says both checks were given "this draft". Check 1 was given report-draft.md (independent-check-1.md line 32), whose sha256 differs from report-draft-2.md's (8981e7d0...).
- **G9 (remark; check 1 F13 stands).** "Nothing pushed" (line 6) and the future items (lines 22, 98 and 99) have no primary.
- **G10 (remark).** Report lines 80 and 81 paraphrase spec 382 to 386 but leave out "when the workbook has one". It is not presented as a quote.
- **G11 (remark, for planning).** Scan pre-check 1873 prints the ruling 6 token itself in size.log's list of placeholders, not before a slash. The final scan.log will do the same. It is not in the placeholder form, so ruling 6's carried rule is not engaged, but a committed log will spell the token.

On check 1:
- I agree with F1 to F4, F7 and F9 as stated.
- F5 and F11 are resolved by diff-phase-from-3cd9696.patch (my Inputs paragraph).
- F10 is fixed in draft 2.
- F12 is qualified by G3a.
- Check 1 missed G2's refusal and limits items and G3a's placeholder-form count.