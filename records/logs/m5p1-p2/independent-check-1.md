# M5 part 1 P2 independent check 1

Checked commit (the phase commit): 70b3e002f7de4bce70c035279c95ba4e51ef81d4
HEAD when the check ran: 70b3e002f7de4bce70c035279c95ba4e51ef81d4
Checker: a fresh general-purpose subagent (not a fork), given only the files below in <check>, where <check> stands for <home>/.cache/emend-build/m5p1-p2/check1. It read scan-pre.log, the scan's pre-check; the final scan.log runs after this answer.
Inputs (sha256, name):
  1ffec0dcd627b3f8c727598175fb9c56ac49372ef44f59942befcaa332890833  diff-70b3e00.diff
  e82149939fae36b599846ab544d24a3cab1a0c8913a01645b97bd41647d8795b  diff-76f489f.diff
  8ac4d1d3181c493bdb1619ef30dacc679b59bd529f6f600d2881e2ccf6a73257  diff-85bb33f.diff
  6bad9b027633cfc3e3959588316aed73bbf724362cb71b8da54202f35fba67c7  diff-phase.diff
  8eded48f5946f4d14e4b3d34fc29069c302a42dba342ca7b44df18d096277cc6  gate.log
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  gate.outer.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  gate.stderr
  bb0fdf20c79fd0b8dba08815a6f653e09e925875500849a60459bc2d6c6bb313  m5p1-open-2026-10-12-70b3e00.md
  c89d8e9b166e0c0b725fb5b289b7fd565b323790d4e1b789e618054cb1a0f8f2  m5p1-p2-2026-10-12-70b3e00.md
  6fb87668c9a3064b2c7240506a634ca19b18b72181766cb8203f42ed4344f3df  negative.log
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  negative.outer.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  negative.stderr
  47a377883ddd00371a86e99661c3e4ef8a38d502327426ed32195c8e911b2dba  report-draft.md
  e81d9d306bacde49f552fe0690fa8a8ad48b5e508828a5c55e3128a8eb6fc2b8  run.log
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  run.outer.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  run.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  scan-pre-check.inner.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  scan-pre-check.stderr
  53b4deeca70b16b009d85b9429ae4856f6c426519de137a664fcc4fbdd74fc92  scan-pre.log
  01c1d1947bff5902e17cb3eb3221725500f12472add28d091efb0835766fb053  spec-0032-at-76f489f.md
  5265b795dd70280d213a65115db86cc20b2ad93650167016e6b40e1a643a6102  spec-0032-working.md
  88a72d93a4dfd064f11581354bf877c24f85ff91d9c7d09741fc6f147ec2de73  specs-0001.md
  ba4b6db50bb15431ad520911a3fff057146fadcef3bc752d9ae4b41b0a0f582f  test.log
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  test.outer.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  test.stderr
Answer: 9760 bytes, sha256 c4094dc29ceb1615f17d864551eff3c20a97728ad0cd50ed7d3c1b019eb072eb; plain ASCII, no machine path, committed verbatim (its angle brackets written as the characters, which the delivery escaped as HTML entities).

---

Independent check, M5 part 1 P2 (specs/0032 "P2: colours", working copy at sha256 5265b795, with P2 rulings 1 to 10 and P1 verification rulings 3 and 4)

I read only the files in check1/. I did not build anything. I recomputed things by hand where a primary allowed it: the sha256 of every diff, log and record; the insertion and deletion counts of each diff; and the seven FNV-1a digests that the tool tests assert, recomputed in Python.

## 1. Does the diff do what the spec says and nothing else?

Yes. There are four minor points below; none of them is something the spec forbids. I found no change outside P2's scope.

**Doc commit 76f489f** (diff-76f489f.diff)
- It touches ROADMAP.md and specs/0032 only.
- The ROADMAP rows read P1 DONE and P2 IN PROGRESS (lines 9-12).
- specs/0032 gains the corrected P1 facts, the P1 rulings and the P1 verification section, as P1 verification ruling 8 requires.
- spec-0032-at-76f489f.md differs from the working copy only by the added "Planning rulings during P2" section (working lines 658-766).

**Crate commit 85bb33f** (diff-85bb33f.diff)
- **The palette.**
  - `PALETTE` (lines 868-877) holds 64 entries with the top byte FF, per ruling 8.
  - I checked all 64 against my own reading of the 18.8.27 table and they agree.
  - The run log's independently typed palette (run.log, the colorcmp.py print) holds the same values.
- **`indexed_colors`.**
  - It is read under the `Colors` typed container (lines 793-806, 830-841, 1325-1375).
  - The refusals are: a second `<colors>` (via `seen_typed` [4]), a second `<indexedColors>`, an `rgbColor` with no `rgb`, an `rgb` that `color_from` refuses, and a main-namespace child other than `rgbColor`.
  - `mruColors` and foreign elements are walked past. All of this is gated on `read`. This matches ruling 1.
- **`theme_colors` and `read_theme`.**
  - Only `a:theme/a:themeElements/a:clrScheme` is read. A second scheme there, or none at all, is refused; `extraClrSchemeLst` is skipped (lines 1110-1162). This matches ruling 2.
  - A scheme child is refused when it is missing, met twice, foreign or unknown. `a:extLst` is skipped. A colour element with children is refused (the `childless` function), and so is any form other than `srgbClr` or `sysClr`. Six hex digits are required in either case (lines 1163-1281). This matches ruling 3.
  - The cache is `Option<Option<[u32;12]>>` on both `Package` and `Parts` (lines 64-69, 158-174). `DRAWINGML` is gated `any(images, read)` (lines 50-51). This matches ruling 7.
- **`resolve`, `ResolvedColor` and `tinted`.** These follow "Colours" and ruling 4: the swap is applied through `THEME_ORDER`; the f64 HLS arithmetic uses HLSMAX 255; rounding is `f64::round`; the alpha byte is kept.
- **The accessors.** `typed_fonts`, `typed_fills` and `typed_borders` are present (lines 1303-1321), per ruling 9.
- **The two messages** are in styles/read.rs (lines 1290-1300).
- **The carry, P1 verification ruling 3.** The off-feature test is at lines 1397-1428; the gate confirms it is `cfg(not(feature="read"))` (gate.log 4979-4981).
- **The carry, P1 verification ruling 4.** The null_rewrite.rs wrap keeps its text word for word (gate.log 4368-4375).
- **Tests.** There are 22 tests in tests/colors.rs. Together they cover every case in the P2 test list, ruling 1's refusals plus the `mruColors` test, ruling 4's two known cases, and ruling 9's test.

**Tool commit 70b3e00** (diff-70b3e00.diff)
- `--colors-sample` writes the input back unedited.
- The four groups and the entry order follow ruling 10, with the `rgb:tinted` entry form and FNV-1a.
- The swap proof uses `rgb` before the tint, over cell and dxf fonts and fills, per ruling 5.
- No colour value is printed.
- It adds 3 tool tests. All seven asserted digests (lines 375-377, 393-395) match my independent FNV-1a computation.

**Points for planning**

- **(a) The package.rs catalogue wrap changed its text.** P1 verification ruling 4 says "wrapped ..., text unchanged". In diff-85bb33f.diff lines 14-18, "and P1 ruling 3" became "P1 ruling 3 and P2 ruling 1", because the sentence was extended with ruling 1's refusals. The wrap also leaves a short line, `//! ruling 1). The`. The gate checks the null_rewrite.rs wrap word for word (gate.log 4372-4373) but runs no equivalent check on package.rs. The report discloses this only as "a reworded refusal sentence". Planning should rule on it.
- **(b) A long `indexedColors` override.** `resolve` consults the override first, even past 63 (diff-85bb33f.diff line 1006). An override longer than 64 entries would therefore resolve index 64 or above to a value rather than `None`. The spec's "None for an index past 63" does not say which way this case goes. No test covers it, and the run log's comparison has the same precedence, so the run cannot detect a difference.
- **(c) How the digests are asserted.** Ruling 10 says the tool test asserts a digest "computed by hand in the test". The tests assert hand-derived constants, with a comment saying they were "worked out by hand beforehand" (line 370). They do not compute the digest inside the test. The values are correct.
- **(d) The catalogue entries arrive in P2.** The two new messages are catalogued in package.rs now, although P3 also has catalogue work. This is consistent with "Errors" ("in the catalogue as the earlier parts' are") and with how P1 handled its messages. It is not a departure.

## 2. Do the logs contain every line the spec requires?

Yes.

**gate.log ("as P1's")**
- The pinned spec hash and the working-copy hash are printed (lines 45-58).
- The commits are listed by role, with ASCII messages ending in the Co-Authored-By line only (4110-4166).
- The diff hashes are printed (4221-4231).
- The phase touches only its own files; the base gains exactly the two new messages; the ten catalogue pins are present (4332-4446).
- The public fn lines are listed with their cfg lines: the base's plus exactly six, all under `read` (4633-4917).
- check.sh shows nine PASS lines (4998-5015).
- The debug binary hashes are printed (5017-5028).
- Clippy and doc are clean for off, read, read,rules and all features (5030-5137).
- The crate tests pass with the four feature sets on both targets, with the colors and carry counts (5139-5216).
- The consumer crate check is present (5218-5266).
- stderr is 0 bytes, and the log ends "gate log checks all as expected: yes" (5283).

**test.log**
- The workspace passes 1156 tests on both targets.
- F gives 515, 584, 622 and 956 on both targets.
- E states each figure as P1's figure plus the colors tests and the carry test. The off figure is shown growing by 1, as P1 verification ruling 3 requires (1609-1678).
- Ten plants are run, each failing exactly its expected set (1854-2054).
- The log ends "yes" (2070).

**negative.log** contains the spec-required case: N1, a `sysClr` with no `lastClr`, exits 2 with "a theme cannot be read", and control B0 is accepted (2716-2780). N2 to N6 are extras. The log ends "yes" (2839).

**run.log**
- Both targets ran: 27 of 27 ok; lo and lo_in ok 27; files_identical yes 27; 870 identical, 0 changed; every entry byte-identical.
- The comparison agrees with the tool on counts and digests in 27 of 27 workbooks, and the swap proof holds in 27 of 27 (3405-3424, 3449-3466).
- `theme_part` and `palette_override` are checked against the parts present.
- The devices of TMPDIR and of the corpus directory are printed (3305-3312), and the calcChain workbooks are counted with their rule (2845-2853).
- The keep and the open check are named against P1's record commit (3612-5111), and the record pair is installed (3574-3610).
- The log ends "yes" (5135).

**Leakage.** No colour value from a real workbook appears in any log or record. Only ids, counts and 16-digit digests appear. My grep for `rgb:tinted`-form entries found none outside the fictional fixtures.

## 3. Does every number and quoted string in the report appear in a primary?

All of the following match a primary:
- The commit SHAs and subjects.
- The four diff hashes (my own sha256 agrees), and the per-commit file, insertion and deletion counts (my own count agrees).
- The hashes of specs/0032, ROADMAP.md and the working spec.
- The log line counts and sha256s.
- "checks all as expected: yes".
- The test figures (515, 584, 622, 956; 514, 562, 600, 934; 0, 22, 22, 22).
- The debug binary hash ee265129.
- Ten plants; 22 of 22; 3 of 3.
- The descriptions of N1 to N6 and B0.
- 27 ok; 870, 0, 0, 0, 0, 0; tints on other forms 1.
- All twelve totals; 498 and 32.
- 1099 pairs and 2 new; 1087 and 12; 1088 and 11; 995 and 104; 0.
- The four record hashes.
- The scan's 3473 lines, 53b4deec, RESULT: PASS, 0 hits and 0 structural failures.

Not in any primary:
- **logged.sh's sha256 b6d09c07..., "exit 0", and the superseded commit 8469e5a.** These do not appear in any primary. The report discloses them itself in item 9.
- **"First attempt" for the scan pre-check.** The scan log only shows "RERUN_NOTE is not set" (line 5).
- **Disclosures 1, 2, 5, 6 and 7** (the Q1 to Q9 exchange, the amend, the stash, the development builds in target/, and the `__pycache__` removal). They have no primary, and the report says so in item 9.

Wording to correct:
- **Disclosure 3**, "the spec's tint-out-of-range case". P2's negative-log requirement names only the `sysClr` case (spec line 470). The tint of 1.5 appears in the spec only as a crate-test refusal. The dropped binary case came from the implementer's own plan (negative.log, change MI, line 150).
- **Disclosure 4**, "the negative N5 above (change MI)". The dry-run finding was about the first draft's N5, the tint of 1.5 (negative.log lines 150-155). The current N5 is the second `<colors>`. The sentence reads as if it means the current N5.
