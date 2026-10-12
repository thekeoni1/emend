# M5 part 1 P3 independent check 1

Checked commit (the phase commit): 6f518994bc2d396dc0f1fab98cde9f2ca9274a3e
HEAD when the check ran: 6f518994bc2d396dc0f1fab98cde9f2ca9274a3e
Checker: a fresh general-purpose subagent (not a fork), given only the files below in <check>, where <check> stands for <home>/.cache/emend-build/m5p1-p3/check1. It read scan-pre.log, the scan's pre-check; the final scan.log runs after this answer.
Inputs (sha256, name):
  e3b7f27c4a48d300b7fd5ee5a09ffa46d8a08f0099088eefac09672d0d0d6115  api-review-m5p1-2026-10-12-6f51899.md
  252ca5c498840bcdb064d4eedfafdbb11faed9059f82d0d86aca5d9f22793024  commit-messages.txt
  5f1b744a84b4081f3c893f24cf50e874c3d45df565f6bda8a7f76d29cae59139  diff-08070f7.diff
  b4f95e5a375ab04ea9a117daddd65d880db99d9a74be2a1abbaf0756b6ff399d  diff-6f51899.diff
  77542d4ab92b9d7559be80e471bf5dab362fb069c7659170fc933a44a9f42a06  diff-c0ee63d.diff
  5169828d3d3e52efca12bcb6f87ebb46fb38cd5ea9c2ed4b972b865ef79ea248  diff-f84f905.diff
  5cf0f3fe7447b626ae34df5a18f423f76ca556e6c0e5cd0fb568b1cc2a637fcc  diff-phase.diff
  629ec669091770c559ae2348b5f4552d1cc1676d3031ae1db53816871b5c3378  gate.log
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  gate.outer.stderr
  04a4d97bb9a6d60ea8652205ecc53a0a8129377fbf9335db76e15e1fe9591257  m5p1-open-2026-10-12-6f51899.md
  60dbf6225852206f46cf70e9d0abc1ec5d33cd48fa3c5155c820223764f997c3  m5p1-p3-colors-2026-10-12-6f51899.md
  aa120792054d9bccdbf085a9f42cd3aebabf9303c2595b789a2ffe1e2a092e0c  m5p1-p3-namedstyles-2026-10-12-6f51899.md
  a15a5e9e7753435423075bb6eedd75806a520fe6ab913a719d25829418358b57  negative.log
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  negative.outer.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  negative.stderr
  ff5224471b49ec9d8b1bd57aec5e47fbb831aaba2bc6f3bc0e3e8381998f686f  report-draft.md
  190bbe46c66b351c7393a7f9be7908d982ae8f8e3923571ff2ed7fd06449c836  run.log
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  run.outer.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  run.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  scan-pre-check.stderr
  31356ab967e8a12d2d4684cad8eb92f5e30eacd8a5a502782f651e25630bec96  scan-pre.log
  8e4cbdc011eb3a7a1070d188b486c1ae372572074131598fe2c5be34b012fdab  size-m5p1-2026-10-12-6f51899.md
  3ab98393896dfba48874bf066556421523e2e0c8539fb3139200a970564dfb9d  size.log
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  size.outer.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  size.stderr
  07bcd9f38034557dde7f3fc30a130ace3ec3ebc07b6e704a196b4d8752e5f961  spec-0032-at-f84f905.md
  e92d9be242a0316d44fd14e659bd4c26fd72a1b561166f2e80a1b76643f6f9b8  spec-0032-working.md
  88a72d93a4dfd064f11581354bf877c24f85ff91d9c7d09741fc6f147ec2de73  specs-0001.md
  aca204b313ec19b75b93ee409831115e4f46757bfb640abbebf5b978a0899464  test.log
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  test.outer.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  test.stderr
Answer: 13900 bytes, sha256 4ef48e80279658d27ef3418148e9148f33da3a9b85cc5a382f52d5a956ec5b4f as delivered; plain ASCII; committed verbatim but for its one machine path, the checker's own <home>/.cache/emend-build/m5p1-p3/check1 in its first line, where the home directory is written <home> (13897 bytes, sha256 84c76487dfac0d1a10bf4f8aadcb101bb9916154c08ba35e145cb17a039be849); its angle brackets written as the characters, which the delivery escaped as HTML entities.

---

**Independent check: M5 part 1 P3** (inputs: `<check>` = <home>/.cache/emend-build/m5p1-p3/check1; working spec sha256 e92d9be242a0316d44fd14e659bd4c26fd72a1b561166f2e80a1b76643f6f9b8 recomputed and matching)

## 1. Does the diff do what the spec says for P3 and nothing else?

**Verdict: yes, with points.**

I read each commit's own diff and checked `diff-phase.diff` against them. All five diff sha256s match the report and gate.log (5169828d, 77542d4a, 5f1b744a, b4f95e5a, 5cf0f3fe). By my count the file, insertion and deletion totals are 2/208/2, 4/82/14, 1/61/2, 1/4/0 and 8/355/18.

**diff-f84f905.diff (the doc commit).** It holds the spec and ROADMAP only. It adds the sections "Planning rulings during P2" and "Planning verification and rulings on P2". ROADMAP moves P2 to DONE and P3 to IN PROGRESS. That meets P2 verification ruling 8. `spec-0032-at-f84f905.md` hashes to 07bcd9f3.

**diff-c0ee63d.diff (the crate commit).** Against Q2(1) and the carried rulings:
- **P2 verification ruling 2 (test and doc clause):** the new test `an_override_of_65_entries_resolves_64_to_its_entry_and_65_to_none` is in package/tests/colors.rs. Its asserts are 64 to `plain(0xFF000040)` and 65 to `None`. Both `resolve_color`'s doc (package.rs hunk @@5560) and `resolve`'s doc (styles/colors.rs) gain the clause.
- **P1 verification rulings 1 and 2:** the new "What the typed view drops" bullet names the dxf `vertical` and `horizontal` sides, the `outline`, and `numFmtId` not read.
- **Spec P3 list:** the dxf pattern rule, `Auto` and the system indexes as `None`, and the theme's 48-byte hold are each stated. The stale "dxfs, cellStyles and cellStyleXfs ... not read" clause is corrected. The styles.rs header sentence is corrected.
- **P2 verification ruling 1 (reflow):** the short line `//! ruling 1). The` is joined to the next one. The result is exactly 80 columns.
- **Nothing else:** gate.log shows comment lines alone changed outside the one new test, and 141 distinct messages at the base and at the phase commit. It also shows the pub-first-lines list equal (191 and 191), test names 1159 to 1160 with the one addition named by ruling, and 125 constants unchanged (negative.log).

**diff-08070f7.diff (the probe commit).** It follows Q1 exactly:
- It calls `named_style` over `0..cell_style_xfs.len()`, then `differential_styles`.
- It calls `resolve_color` over the same `fonts` vector that `read_fonts` counts. It counts `Some` results and counts `tinted() != rgb`.
- It calls `default_font` with the size defaulting to `0.0`.
- It prints five lines in the ruled order. Every call uses `?`, so a refusal fails the probe.

**diff-6f51899.diff (the phase commit).** It is CHANGELOG.md alone: one "Added" bullet naming named styles, differential styles, the default font and colours resolved, citing specs/0032. That is Q2(3).

**Commit order and messages.** The order is doc, crate, probe, CHANGELOG. gate.log's "every commit message in the range" section shows each message is ASCII and ends with the Co-Authored-By line only.

**Points:**
- a. Spec "P3" asks for "the messages by feature" in the catalogue. The diff adds no message entries; it relies on the existing Errors paragraph (P2 verification ruling 4: "edits the entries as it needs"). The diff's context alone cannot show that paragraph is grouped by feature. negative.log section ML does show the catalogue entries exist and name their tests.
- b. The reflow is a single line join (diff-c0ee63d.diff, hunk @@687). The following lines keep their old breaks. That is acceptable under ruling 1, but it is not a reflow of the paragraph.
- c. The "What resolves to `None`" bullet includes a clause about what does not resolve to `None` ("an override longer than 64 entries resolving an index within it"). The substance matches ruling 2; the placement reads oddly.
- d. The probe's runtime figures are `read_dxfs 0`, `read_tinted 0` and `read_default_size 0` (size.log lines 3423-3427 of the non-script output). The calls are linked and executed, and `tinted` runs on the one resolved colour. But on this fixture no dxf, no tint difference and no default size is exercised. Q1 only says "linked and used", so this is not a breach. It is a note on how much the probe proves.

## 2. Do the logs contain every line the spec, its rulings and specs-0001 require for P3?

**Verdict: yes, with points.**

**Present:**
- **gate.log:**
  - Environment line with its controls.
  - PYTHONDONTWRITEBYTECODE printed and checked (P2 verification ruling 6).
  - The phase commit given and compared with HEAD, with a fabricated-commit control.
  - The gate files listed against the diff.
  - No prune, per ruling 7.
  - Nine PASS lines.
  - Debug binaries 8c9787ff (host) and 225c362d (i686).
  - "gate log checks all as expected: yes".
- **test.log:**
  - Off 515, read 585, read,rules 623 and all 957 on both targets; workspace 1157 on both (Q6).
  - E lines tie each figure to P2's plus the one new test.
  - Plants P1 to P11, each "failed tests are exactly the table's"; P11 is the ruling 2 plant (Q6).
  - The T1 listing: 567 rows, "1 gone, 24 new", including ruling 9's three accessors and `theme_colors`, `resolve_color`, `ResolvedColor` and `tinted` (P2 verification ruling 8).
  - The formatter line, the spec map extended (`"M5 part 1": "0032"`), and the record installed at e3b7f27c (Q3).
- **negative.log:**
  - N1 to N12, each naming its source and original number.
  - N13 "no such part".
  - N14 as `default_font`'s case.
  - Both unreachable cases stated with their covering tests.
  - The one line on `Error::Xml` and the unbound prefix.
  - "negative log checks all as expected: yes".
- **size.log:**
  - All 28 probes classified, with a cause per probe and its control, and the manifest commits listed.
  - Core growth 0, with both figures printed.
  - Budget 436992, 21760 under the ceiling.
  - Feature cost 70044, and "above 80000 ... : no".
  - The five probe lines, checked against an independent Python reader.
  - The theme-relationship check for Q1: "the input names no theme part it lacks ... : yes".
  - Release hash lines for both targets.
  - The record installed at 8e4cbdc0.
- **run.log:**
  - The binaries checked against size.log's lines.
  - Both samples on both targets, "rows compared 27; identical ... 27" for each sample and label, with the per-row lo, lo_in and entry checks kept (Q4).
  - namedcmp and colorcmp stated as not rerun.
  - The citation check.
  - TMPDIR and corpus devices (2096 and 2096).
  - calcChain 9 of 27, with its rule.
  - Earlier runs named by record commit.
  - Keep 1099 pairs, none changed.
  - The open check.
  - All six records installed.

**Missing or weak:**
- e. **The run step's superseded attempt is not in run.log.** run.log line 7 reads `input: RERUN_NOTE=unset`, and SUPERSEDED_RUN is unset too. Attempt 1 appears only in scan-pre.log, which is uncommitted (lines 5-6 and 57-63, hashed there at 05:52:31Z). That is after attempt 2 ended (05:50:22Z). specs-0001 says "A superseded attempt is hashed and kept before the next one starts". Nothing I was given shows attempt 1 hashed before attempt 2. P1 verification ruling 6 (RERUN_NOTE on a rerun) is met only in the scan.
- f. **Q5 says "the one shared control B0".** negative.log runs two controls: "B0 (namedstyles) ... (the control for N1 to N6 and N14)" and "B0 (colors) ... (the control for N7 to N13)". With two fixture families this is reasonable. It is still a departure from the ruling's wording, and the report's departures section does not name it.
- g. **N14's cause is argued, not measured.** The refusal is the shared "a style cannot be read". The claim that `default_font` is the refusing call rests on a script comment and an echo line: "N14 counted among the cases: ... the one call left to meet the missing <fonts> is default_font". No control isolates the cause; for example, the same part with `<fonts>` kept would be expected to be accepted. N14 also drops `<cellStyleXfs>` and `<cellStyles>`, not only `<fonts>`. Ruling 5 left the case's form to the dry run, which no primary I have shows.
- h. **Dry runs.** Spec "Phases" requires a dry run before each logged run, and Q1 says the size fixture "is checked at the dry run". No primary I was given evidences any dry run. The report declares this in its item 8.
- i. **Python `-I` and PYTHONDONTWRITEBYTECODE.** negative.log, size.log and test.log claim no Python they run writes bytecode, because the variable is exported. Yet they invoke `python3 -I` without `-B`: negative.log `python3 -I <scratch>/w/gen-ns.py` and `gen-co.py`, size.log `python3 -I <scratch>/w/sources.py`, run.log `python3 -I <scripts>/changelog_check.py`, and 3 such calls in test.log. Disclosure 4 itself says `-I` ignores that variable. The practical risk is nil: these are single scripts and `-I` drops the script's directory from the import path. run.log alone measures it ("left no scripts/__pycache__ ... : yes").
- j. **The API record does not say why one row is "gone".** The row is `use read::CellStyle`, re-exported now as `use read::{CellStyle, NamedStyle}`. test.log shows both keys; the record lists the gone row without saying it is a rename.

## 3. Does every number, hash and quoted string in report-draft.md appear in a primary?

**Verdict: yes, with points.**

**Confirmed:**
- **Commits:** all commit ids and subjects (gate.log, commit-messages.txt). The base subject (run.log).
- **Diff hashes:** all five (gate.log).
- **Spec and ROADMAP hashes:** spec 07bcd9f3 and ROADMAP 43e4ed35 (gate.log, commit message). Working spec e92d9be2, in every log.
- **Log hashes:** gate 629ec669 (negative.log, scan-pre.log). test aca204b3, negative a15a5e9e, size 3ab98393 and run 190bbe46 (scan-pre.log only; size's also in run.log). I recomputed all of them over the files.
- **Line counts:** 5366, 3409, 3373, 10200, 5698 and 3603 match `wc -l` of the files.
- **Step end times:** 04:19:50Z, 04:24:25Z, 04:24:28Z, 04:29:24Z and 05:50:22Z (each log's "run ended" line).
- **Binaries:** debug 8c9787ff and 225c362d. Release 4861eb6d, df512f91, 1445523d and 99d1661d.
- **Test figures:** 515, 585, 623, 957 and 1157. Eleven plants.
- **API listing:** 567 rows, "1 gone, 24 new".
- **Size figures:** 0, 2000, 436992, 21760, 458752, 70044, 80000, 18056 and 51988. Copy and pair growth 0. The five probe lines.
- **Run figures:** 27 each for the identity checks. Keep 1099. Open check 1087/12 and 1088/11.
- **Records:** all seven record hashes (run.log, size.log, test.log install lines).
- **Scan pre-check:** "RESULT: PASS". RERUN_NOTE and RERUN_FILES (scan-pre.log lines 5-6).

**Not found, or derived rather than printed:**
- **logged.sh dddc78d1 and logged-run2.sh fd37511d:** in no primary. The report declares this (item 8).
- **Superseded commits 8875191, 76aeebc and ef02546, and the "94-column doc line":** in no primary. Declared (item 8, disclosure 2).
- **Attempt 1's "exit 2":** not in any primary I have. scan-pre.log shows only `0 bytes run.log` and `127 bytes run.stderr`. Covered by item 8's "their outputs".
- **"findings 0" (run.log bullet):** no such figure is printed. It summarises the open record's two "(a finding)" sections reading "None." and run.log's "outputs refused on an accepted input 0".
- **Per-commit file, insertion and deletion counts:** no log prints them. They agree with my count over the given diffs.
- **Scan pre-check sha256 31356ab9:** printed in no primary. It matches the file. The report labels it "not committed".
- **"a new 'Read breadth' Limits entry":** the diff has no entry by that name. The paragraph opens "Read breadth adds these, under the same Cargo feature `read`".
- **Disclosure 7:** it lists the ": no" lines of the committed logs but not scan-pre.log's "EMEND_PRIVATE_DENYLIST set: no (unset by this script)". That line is the expected answer.

**Observation (not an error):** the P3 open record's .txt sha256 1ca0a182... is byte-identical to P2's committed `records/m5p1-open-2026-10-12-70b3e00.txt` (run.log: "the record: records/m5p1-open-2026-10-12-70b3e00.txt (sha256 1ca0a182...)" and "installed records/m5p1-open-2026-10-12-6f51899.txt: sha256 1ca0a182..."). That is consistent with the keep being unchanged, but the report does not say so.

**Declared as not in any committed primary (report item 8):** the logged.sh and logged-run2.sh hashes, their outputs, the superseded commits, the dry runs, and disclosures 2 to 6. Its scan pre-check bullet also says "(not committed)".

## Points for the planning session's ruling

1. **The run-step rerun.** run.log carries no RERUN_NOTE or superseded-attempt line. Attempt 1 is hashed only in the uncommitted scan pre-check, after attempt 2 finished. Is the final scan's note enough under specs-0001 and P1 verification ruling 6?
2. **Two B0 controls in negative.log** against Q5's "the one shared control B0". Accept, and should the report name it as a departure?
3. **N14.** Its attribution to `default_font` is reasoning rather than a measured, isolated cause, and the fixture drops three containers. Accept as Q5's dry-run outcome, or ask for a control (the same part with `<fonts>` kept and accepted)?
4. **Two rule breaches repeated.** Disclosure 3 is a hand `rm -rf`, which P2 verification ruling 6 accepted "this once". Disclosure 4 is a generator run that wrote `gen/__pycache__`, against the same ruling's bytecode requirement. Also, the logged `python3 -I` calls lean on PYTHONDONTWRITEBYTECODE, which `-I` ignores. Should `-B` become the rule?
5. **Wording in the report:** "findings 0" (not a printed figure), "exit 2" (not in a primary), and the "Read breadth" entry name.
6. **The catalogue's "messages by feature":** the diff adds none and relies on P2's Errors paragraph. Confirm that this meets spec "P3".
7. **The API record** does not explain its one "gone" row, which is the re-export renamed into `use read::{CellStyle, NamedStyle}`.
