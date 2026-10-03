# M3 part 3 P4, independent check 1

The answer of a fresh subagent (not a fork) given only <phase>/check1 (the committed spec specs/0025, the committed specs/0024, specs/0001, the phase diff, the doc commit's and each code commit's diffs, the commit messages, gate.log, test.log, negative.log, size.log, run.log (attempt 3), the scan's pre-check, each one's stderr, the superseded run.log attempts 1 and 2 and attempt 1's failed scan pre-check, the phase's records with part 2's size record, and the draft report), recorded verbatim below. <phase> stands for <home>/.cache/emend-build/m3p3-p4. Checked commit (the phase commit): 01b14c0e9b5a9a1b72fb497d8dc6a6dd85a03ae1; HEAD when the check ran: 01b14c0e9b5a9a1b72fb497d8dc6a6dd85a03ae1. The answer is ASCII as written; it was extracted from the subagent's final message and hashes to 35bd1e9b868aa5da2df4052d661152fca42a498998d9ee538f9070e56a070803. Its items on the report are corrected in the report; its observations on the code and the logs are reported to planning. No second check was run.

Inputs and their sha256:

- e2221f2aee8df0c38c858030cefcc0db6feccd10a1881a9b5e4e7f1b584c233a code-commit-1-carry.diff
- 8985d504d9b65ec847376f3176e97ce5b315c8febc2e2b5e183e78e60c12f6b4 code-commit-2-catalogue.diff
- 8c4c185c3c92654a955ed420afd44aedf63727a8d333a9954c70160899435320 code-commit-3-probes.diff
- 7bb35979cf8a1c07bba0912bdb87de910f243566c005123ceb5e130cc88f86ed commit-messages.txt
- c81f027c5a4f64fd5a83d5eeda767f6ea0f2c7174cfb1fc52918f85ae4aa2aa3 doc-commit.diff
- 953a93d6dcc7b27c86bf19d8b0967f5648e577f9f0da12f2c4864bbe19d8d780 gate.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 gate.stderr
- 658613c0a2853f490a9c8bbd965ecc060e87aa7b242d758798d3c4e5cba93f39 negative.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 negative.stderr
- da16aaf0b2291741db8508c4c57fdf7cc48710832e88ee048f3b35a13b549ebf phase.diff
- 9d9103d7d2432935b713911d135f76d78637a8402f5b7e953c0de6cc2ffc8f71 records/m3p3-open-2026-10-03-01b14c0.md
- 0b207b2bf61203114a23764d0954d0ef3222fa468c9a9dcff1748deb9e3d0556 records/m3p3-open-2026-10-03-01b14c0.txt
- abfd8616196b4f295ebfe9869a2ed00a0355dbaf27640f56e22db6ff6d803dfe records/m3p3-p4-pagesetup-2026-10-03-01b14c0.md
- 11f92c85754dd7885854047f102313e0ac5fb16aeab0f317d293707222d47ace records/m3p3-p4-pagesetup-2026-10-03-01b14c0.txt
- b87b87a2ba9e0b3dcdf07a8abb578271a31d66749cdee5027819c532ee24fd93 records/m3p3-p4-printarea-2026-10-03-01b14c0.md
- 16b39d8013f38e885337c0d8d822da52dba4a43cd156728c553dfb8bae3226da records/m3p3-p4-printarea-2026-10-03-01b14c0.txt
- ba2afe4b57a5254b523c14aed29348567ebc4eafec2d188b6f3596f0addadfbe records/m3p3-p4-sheetview-2026-10-03-01b14c0.md
- d63249a1258705f6f05c0773fc07c6a68c46e91e5b04fe91728c020a6ce846cd records/m3p3-p4-sheetview-2026-10-03-01b14c0.txt
- a8bd470844e32ec5f2b68757c84623d9945253459fcd928ce68433da348e1eb8 records/size-m3p2-2026-10-03-0c3e4fa.md
- b847515ab9c7bafe7c8d2c4f11cf5194a89fb30d243c0e378019d0bb1f7f966f records/size-m3p3-2026-10-03-01b14c0.md
- 7444b58cb9a082a117c16cfe38a3f843a7dbded9fba3a9646d18cb18d9b3b2da report-draft.md
- 93445a576583cbd5d0ab505143bf72816285f6838ac77fdeb54e288715479aa4 run.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 run.stderr
- ebfaec6b7c4e863ddc1f6a50209aa5130729f07088616475cb3ab802b75dc320 scan-pre-check.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 scan-pre-check.stderr
- f0961333912ef5c13a54ec55cb621c5f8305edf13b91427f06c389c20695a93c size.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 size.stderr
- 79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c spec-0001.md
- ae1ab201b5c7071c36235f2c5d8c253c6badfab95a9f163575ef7daa0e78863a spec-0024-committed.md
- efdacd5c7d337e3f51e35e7ae626d78300f2cce65cb429e4950c1487a4d7fe47 spec-0025-committed.md
- 5545eacf5e2bb95312f9c2f195cb68c99a35ab8065248b276a71d2716129aacb superseded/run-attempt-1.log
- 8b0a9b3c0e546c273c871a3ca55e6d8b1d86933f5a2d578daf79471e2489c298 superseded/run-attempt-2-aborted.log
- 008adb0f99174cbd08a12196506be84b6a72cbe2c7cfe89db1276276051148ea superseded/scan-pre-check-attempt-1.log
- 812d010f79b6cfca0e5515044fc14ced5ffa90b0d0bf76d381ee2086e3411211 test.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 test.stderr

---

# Independent check 1: M3 part 3 P4

Inputs verified first: `sha256sum -c ../inputs.sha256` gave every file OK and 0 lines not OK. Every .stderr file is 0 bytes. I read only files under the inputs directory and ran no git command.

## 1. Does the diff do what the spec and the P3 verification rulings say for P4, and nothing else?

**Verdict: YES with reservations.**

### Evidence

**Doc commit**
- The doc commit holds the spec at efdacd5c and ROADMAP.md at 8e1658d3 (commit-messages.txt 71-72).
- gate.log 2123-2150 checks both against the hashes planning's message gave. The doc commit carries exactly those two files.
- sha256 of spec-0025-committed.md is efdacd5c..., equal to the pin.

**Phase diff**
- phase.diff equals the union of the three code-commit diffs. I compared the sorted +/- lines of the three diffs against phase.diff: identical.
- None of the four commit messages or the diffs holds a non-ASCII byte (grep -P over all *.diff and commit-messages.txt: 0).
- Each message ends with the Co-Authored-By line only (commit-messages.txt 23, 48, 66, 76; gate.log 2090-2100).

**Carry (f59c78c): test code only, the four ruled tests**
- It touches crates/emend/src/package/tests/names/print.rs alone, status M, 130 lines added, 0 removed (code-commit-1-carry.diff 1-138; gate.log 2241-2290).
- The four tests are:
  - **Replaced part, P3 verification ruling 1:** the three calls on a replaced part are refused with part 1's message (carry diff 10-41).
  - **Shift, ruling 1:** a print area and titles beside insert_rows are written unadjusted, and the autofilter beside a shift is refused, under rowcol (43-76). The expected texts quote "S1" because S1 names a cell, as P3 ruling 6 requires.
  - **Missing table part, ruling 2:** an autofilter whose table part is missing is refused with TABLE_OVERLAP (78-107).
  - **Added table, ruling 2:** an autofilter overlapping a table the same write adds is refused, under tables, with a control (109-138).
- This matches P3 verification rulings 1 and 2 (spec-0025 878-895).

**Catalogue (b0c6bd8): //! lines only**
- package.rs alone, 82 lines changed, 0 of them other than //! (gate.log 2290-2300; code-commit-2-catalogue.diff 9-90).
- Ruling by ruling:
  - **Attribute merge (P2 ruling 3; P2 verification ruling 1):** stated once for every element: set first in the corpus order, kept after in their order (catalogue diff 26-33).
  - **Separators not kept (P1 verification ruling 3, R3; P2 verification ruling 2, C4):** catalogue diff 29-30.
  - **Prefixed same-name attribute kept (R2):** catalogue diff 30-32.
  - **What Excel refuses above the bounds is Excel's (P2 ruling 1):** catalogue diff 52-56. The bounds match P2 ruling 1 and P1 verification ruling 1 (the tint).
  - **Built-in names:** exempt from the formula-change rule, no chain drop and no fullCalcOnLoad (P3 ruling 2); never adjusted on a shift; the autofilter is refused beside one (P3 verification ruling 1). Catalogue diff 70-75.
  - **Titles:** columns then rows, "openpyxl's order" (P3 ruling 3), catalogue diff 68-69.
  - **Sheet-name rule and two corners (P3 ruling 6):** catalogue diff 64-68.
  - **Never written:** every item in the P4 list is there: tabSelected, workbookViewId, gridLinesSet, autoPageBreaks, outlinePr, cellComments, errors, useFirstPageNumber without a number, the printer settings part, a filter's columns or sort state, and an even or first text without its flag (catalogue diff 44-51).
  - **Created-view path proven by tests (P1 verification ruling 3, L4):** catalogue diff 34-43.
  - **What part 5 will give:** catalogue diff 81-86.

**Probes (01b14c0)**
- The manifest gains `sheetprops = ["emend", "xml", "emend/sheetprops"]` with no dependency of its own. There are two [[bin]] blocks: sheetprops-write-xml (required-features ["sheetprops"]) and printnames-write-xml (["metadata", "sheetprops"]) (code-commit-3-probes.diff 1-31). gate.log 2300-2340 confirms these are the only non-comment manifest changes and that no Cargo.lock changed.
- **sheetprops probe:** after the format probe's work it calls:
  - set_tab_color;
  - set_sheet_view with the zoom from the command line and gridlines off;
  - set_print_options;
  - set_page_margins;
  - set_page_setup (landscape, fit 1 by 0, fit_to_page);
  - set_header_footer with an odd header from HeaderFooter::section.

  It then counts sheetPr, printOptions, pageMargins, pageSetup and headerFooter (probes diff 1137-1188, 1236-1247, 1271-1288).
- **printnames probe:** after the metadata probe's work it calls set_print_area, then set_print_titles of the area's first row, then set_auto_filter. It reads the built-in names back through workbook() (probes diff 431-441, 492-498).

### Reservations

- **R1, messages by feature.** P4 asks the catalogue for "the messages marked by feature". The new //! block names only the overlap message and "the settings message" (catalogue diff 76-80). It does not list the four new constants with their feature marks. The constants do sit under their cfg lines (negative.log 1302-1338). Whether an earlier part of the package header already lists them by feature cannot be seen from these inputs.
- **R2, two catalogue claims with no primary here.** "a `<headerFooter>` left with no attribute and no child is dropped" (catalogue diff 51) and "an apostrophe as &apos;" (69-70) describe P1-P3 code that is not among the inputs, so I could not confirm them.
- **R3, how the sheetprops probe counts elements.** "Size" says the elements are counted by "the probe's own quick-xml count". The probe counts quick-xml events delivered by the crate's own Stream (probes diff 1271-1288), and its doc comment says so (probes diff 793-794). The count is the probe's own; the reader is the crate's. This is a wording difference, not a functional one.
- **R4, no control on the missing-table test.** That test (carry diff 78-107) has no clear-of-table control. The rulings did not require one, and test.log's plant V3 shows that the test can fail (test.log 4930-4937).

## 2. Do the logs contain every line the spec and binding rulings require?

**Verdict: YES with reservations.**

### Spec pin
Every log pins efdacd5c:
- gate.log 21-25
- test.log 22-28
- negative.log 70 and 378
- size.log 4, 68 and 7639
- run.log 4 and 2051-2052
- scan-pre-check.log 7 and 53

### gate.log
- Crate tests on both targets:
  - host: 513 off, 540 with sheetprops, 601 with sheetprops and metadata (gate.log 2761-2771);
  - i686: the same three figures (2776-2786).
- clippy and doc sections: 2696.
- The public fn and const lines: 2463.
- Ends "gate log checks all as expected: yes" (2857).

### test.log
- The workspace runs 977 tests on both targets with the same names (4318-4339, 4969-4970).
- Four tests added, which accounts for 973 + 4 (4353, 4389-4410).
- Crate runs with features off, sheetprops, and sheetprops plus metadata on both targets: 4426-4497.
- One EMEND_KEEP_DIR run: 4498-4512 (800 passed with and without it; 1390 files kept).
- Each of the four plants fails at its own assertion line: lines 429, 450, 496 and 518, "plants: 4; distinct assertion lines they failed at: 4" (4900-4955). Failing test names are printed. This follows spec-0024 P2 verification ruling 6.

### negative.log
- 21 fictional fixtures (1475).
- TWO_SETTINGS is refused six ways (S1, S2, P1, P2, P3, A2). TWO_ELEMENTS, TABLE_OVERLAP and INDEX_OUT_OF_RANGE are refused once each (S3, T1, F3), each with a control that writes output (1665, and the checks list before it).
- SETTING_OUT_OF_RANGE, HEADER_FOOTER_INVALID and PRINT_INVALID are stated unreachable, each with its reason and covering tests, and each test body checked for the message (1686-1707).
- "the messages accounted for 4; reached 1; stated unreachable 3" (1690).
- The other shared messages are printed as left to earlier parts' logs.

### size.log
- **Probe list:** 25 probes measured; 23 compared with part 2's record; the two new ones listed as new (5163).
- **Classification:** every earlier probe is classified against part 2's record (5301 to about 6000). There are 6 identical and 17 code; each code verdict names a commit touching its source causes.
- **Cause per probe:** each probe's causes are listed from the diff's paths: its own bin file, the manifests, and for emend probes the five emend files the diff changed (5422-5476, 5437, 5465). The cause control and the manifest cause listing are kept (5429-5434).
- **Feature cost:** 28148, with "above 35000 ...: no" (6026-6027).
- **Pair's cost:** 34468 (6028).
- **Core growth:** "969672 - 968936 = 736", with "above 2000 ...: no" (6029-6031). The 0c3e4fa rebuild reproduces part 2's hash (6001-6010).
- **Budget:** 436456, "22296 under the ceiling" of 458752 (6034-6036).

### run.log, attempt 3
- **Earlier run directories:** P1, P2 and P3 run directories are named by record commits 138fe80, 25f9bba and 38d4008 (31-36, 2174-2186).
- **Binaries:** size.log's release binaries, matched by hash (2059-2075).
- **Each sample, each target:** outputs, lines and diff tables equal to the earlier run's:
  - sheetview, 27 workbooks: host 2422-2423, i686 2527-2528;
  - pagesetup, 27: host 2671-2672, i686 2776-2777;
  - printarea, 21: host 2914-2915, i686 3013-3014.
- **Refusals:** the six overlap refusals, with ids, on both labels (2856-2861, 2955-2960). They are the same six P3 verification named.
- **Calc chain:** "9 of 27" with its rule in one line (3072), with the per-sample counts at 3066-3071.
- **Devices:** the runs' TMPDIR and the corpus directory, device 2096 against /proc's 61 (2190); the open check's TMPDIR, also 2096 (3255).
- **Keep against P3's record:** 776 pairs regenerated against P3's record of 774. The 2 new pairs come from tests this phase added, and none changed (3194-3240).
- **Open check:** "findings 0" (4037). No verdict of P3's pairs moved (4172-4180).

### Rerun lines
- run.log 54 names attempts 1 and 2. Lines 55-58 hash all three superseded files, and the hashes and byte counts match the files in superseded/:
  - 5545eacf..., 325659 bytes;
  - 008adb0f..., 176265 bytes;
  - 8b0a9b3c..., 159744 bytes.

### rm -rf
Every `rm -rf` is either:
- the `"${var:?}"` form on a mktemp directory the script made (gate.log 685/689/705, 750/763; test.log 742-743, 803/816; size.log 751-752, 830/843), or
- counting.sh's own on its controls' directory (gate.log 2019, test.log 2177, negative.log 1209, size.log 4029, run.log 2013).

run.log's change JI (315-322) correctly says the script itself has none.

### Attempts 1 and 2: sound
- **Attempt 1:** the logged script differs from attempt 3's only by change OK, a five-line comment and one `root: <scale>` echo (diff of the printed scripts). Attempt 1 ended "all as expected" (superseded/run-attempt-1.log, last line). Its pre-check failed solely on the undeclared `<scale>` (superseded/scan-pre-check-attempt-1.log 2746, 2758-2760). Attempt 3 declares `<scale>` at run.log 79, and its pre-check passes (scan-pre-check.log, last lines).
- **Attempt 2:** its printed script is identical to attempt 3's. Its rerun note used `<phase>` with no root line (superseded/run-attempt-2-aborted.log 8, 54, 76-85).
- **Nothing lost:** attempt 1's and attempt 3's outputs differ only in timings, peak memory, run-directory names, temp-dir names, record hashes (from the directory names) and the status count. The per-workbook sha256 comparisons hold in both. Attempt 1's run directories and attempt 2's partial directory are left in place. Attempt 1's keep was moved, not removed.

### Reservations

- **L1, attempt 2's stopping point.** The claim that attempt 2 was stopped "during its first corpus run" (run.log 54) is not shown by the partial log. That log ends inside the printed record-check script, before any "## host run" header. Its size, 159744 bytes, is exactly 39 x 4096, which fits buffered output cut short but proves nothing either way. The superseded run directories and attempt 1's moved keep are named but not hashed; only the logs are, and run.log 59 prints "superseded run directory named: none; superseded keep named: none".
- **L2, identical cause lists.** The cause per probe is read off the diff's paths, but every emend-linking probe gets the same five emend files (size.log 5440-5476). It is per probe in form, but no narrower than what any emend probe links. This is acceptable under spec-0024 P3 verification ruling 5 as I read it, but planning may want it noted.

## 3. Does every number, hash and quoted string in report-draft.md appear in a primary?

**Verdict: YES with reservations.**

### Checked and found
- **Base and doc commit:** 38d4008 and its subject (run.log 31-33); efdacd5c and 8e1658d3 (commit-messages.txt 71-72; gate.log 2127-2134).
- **Commit hashes:** 2b19bb6, f59c78c, b0c6bd8 and 01b14c0 (commit-messages.txt).
- **Log hashes:** gate 953a93d6, test 812d010f, negative 658613c0, size f0961333 and run 93445a57 (the sha256 of each input file; also negative.log 1244, run.log 2062 and 3076).
- **Crate test counts:** 513, 540 and 601 on both targets (gate.log 2761-2786).
- **Workspace count and plants:** 977 (test.log 4969); four plants at distinct lines (test.log 4954).
- **Unreachable messages:** the three quoted messages match negative.log 1342-1345 and 1686-1689 by constant.
- **Size figures:**
  - 28148, and "no" above 35000 (size.log 6026-6027);
  - 34468 (6028);
  - 969672, 968936 and 736, and "no" above 2000 (6030-6031);
  - 436456, 22296 and 458752 (6034-6036).
- **Size record name:** size-m3p3-2026-10-03-01b14c0.md; its hash b847515a appears in size.log.
- **Run figures:** 27, 27 and 21 per label (run.log 2422, 2671, 2914); six refused (2856-2861); 9 of 27 (3072); 2096 (2190, 3255); 776 pairs and "findings 0" (4037).
- **Superseded attempts:** 5545eacf, 008adb0f and 8b0a9b3c (run.log 55-58 and the superseded files). "failed one structural check" (superseded/scan-pre-check-attempt-1.log, line with "structural checks failed: 1"). Change OK (run.log 79 and the printed script). The placeholder note (superseded/run-attempt-2-aborted.log 8).
- **Scan pre-check:** RESULT: PASS (scan-pre-check.log, last line), hash ebfaec6b (the sha256 of the input).
- **Record file names:** all match records/, and each record's hash appears in run.log or size.log.
- **Disclosure 1:** the five file names (size.log 5422-5428); "expected near 0" (spec-0025 76 and 357).

### Without a primary (working state)
- "nothing pushed" (report line 3).
- "the final stage runs after the check" (line 21).
- Disclosure 2, the two stray empty files and the plain rm (line 27).
- The reading in disclosure 1 (line 26) that "parts P1 to P3 changed files the core builds with no feature on" is the cause of the 736. size.log lists the files but does not attribute the bytes to them.
- "during its first corpus run" (line 20) appears only as the script's own input note (run.log 54), which the partial log does not confirm (see L1).
