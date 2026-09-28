# Independent check 1, M2 part 4a P3

Written by the implementing session: the checked phase commit is
372bc214503fd4fdca25fe69b5b6efd409b29ff5, HEAD when the check ran. The
checker was a fresh general-purpose subagent given only the directory
check1 in this phase's folder on the disk, holding these files, by sha256
(the logs there are the four committed logs, each attempt 1, and the scan
pre-check's attempt 1, which the final scan.log's rerun lines hash):

    089a678456ea5b2a448037213683b06300208233bd4624317124fd3153acfd0d  commit-messages.txt
    8c68e039c9d1ee539a0fb2c232ab9026294820e571d7e49f56ee357f637b767b  diff-372bc21.patch
    941b484ebc412bcf24e7a2fa0b4138e9fc9377c35f0fb38371e71ea7fcb2ee05  diff-51acd32.patch
    9ee3b5f020779af18b22a202f7bd97455df133fed134307415deab480e117a09  diff-bc46388.patch
    d5a18e8e46cbc5c37fc3ba671950a66c2b0ab0fa030b15afc4f40fb03080fc22  diff-commands.txt
    030aa7ef5686b5137e0078e9439109d600227859984ffd3b0022c6992425fcce  diff-doc-c9b313f.patch
    07afd76830b42c6f0399621feda737b936bc06d1aae2da2bad1e99c2cef483ab  diff-phase.patch
    7d1493d1c2c43cef6f00462fc20d2841f0aa4e945725e8541533c0156e53d9aa  log-gate.log
    77c9168ddd4281788a06542a8085e16fd0116786b9455d629a9205586d7fa1dc  log-negative.log
    9b4e75f491e1aa92176d0434f3cd6359ab2c2bb208739444f3c353cade6fbca0  log-run.log
    a2a6d979f4ffa821efc9865163585f856c4b5300515a7db0481dfd86df6215b4  log-scan-precheck.log
    2846d9f8c6cc056296677396371f0b7f4651cc313b9698c0206c0d27c8d9619a  log-test.log
    cce49be67eb2801601bedc9520f5a6c6d8ca0844811aeb04284e4461974809ec  m2p4a-open-2026-09-28-372bc21.md
    956a27ab0cd1ccece6fe43c10d43e31b31d9f1542b76673d00555fb8f57008e1  m2p4a-open-2026-09-28-372bc21.txt
    3327e6347b676fae80c0568e354add380f5540098af747d3b7623f0485e681a6  m2p4a-p3-props-2026-09-28-372bc21.md
    1deb3d3236be5a3b775552f42b308788079a0559ad10dadf0a07c231bcb234af  m2p4a-p3-props-2026-09-28-372bc21.txt
    7e0026f3eba2fd8430f0e332365b83351cb78b8d4a7533c562b2ab1e503ed688  m2p4a-p3-protect-2026-09-28-372bc21.md
    3980432cd1d0f5f5b600126733ee9ddbd2fea37e1b71d73636fd8519fb14dd08  m2p4a-p3-protect-2026-09-28-372bc21.txt
    1c61c69f8239e9bb29909f35cc26f7e4afbb3bf2299c43513ff8654e2f6b9ba5  report-draft.md
    79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c  specs-0001-bootstrap.md
    541003c1280e93fad39ec97767d539f1263b2799b9917156fd3c55f92bb9b4e1  specs-0018-rows-and-columns.md
    e276356647ce82d213b96be8c852ba715892696b69cc6b2331bedd454387bcb1  specs-0019-names-links-properties-protection.md
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-gate.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-negative.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-run.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-scan-pre-check.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-test.txt

The answer below the rule is the checker's final message, verbatim, taken
from its transcript (sha256 09a1754753fad2d60ece29c60e8745f448c076ade17cf3859a26d963cf7a38e9,
12462 bytes; plain ASCII, nothing replaced).

---

Independent check 1, M2 part 4a P3 (document properties and protection)

Context disclosure: the harness put a CLAUDE.md, a memory index, a git status snapshot and attribution text into my prompt. I did not use any of it as evidence. I read only the files in `<check>`. The sha256 of each input matches `../check1-inputs.sha256`. The git blob hash of `specs-0019-names-links-properties-protection.md`, computed locally with `git hash-object` on the file, is d1b3779, the post-image blob in `diff-doc-c9b313f.patch` line 15. Its sha256 is e2763566..., the value every log pins.

## Short answers

**Q1.** Mostly yes, with one defect and several spec departures planning must rule on.
- `diff-bc46388.patch` does what P2 rulings 2, 4, 6 and 7 require, and I found nothing else in it apart from one behaviour widening (F7).
- `diff-51acd32.patch` implements the "Document properties" and "Protection" sections.
- Defect: a property edit on a package whose core relationship names a missing part is silently dropped at the write (F1).
- Departures, most of them disclosed in the report: the default and `sheet=false` bytes, a second relationship-type form in `CORE_PROPERTIES`, and extra refusals (F2 to F6).
- `diff-372bc21.patch` matches "Harness runs".
- `diff-doc-c9b313f.patch` is exactly the two planning files.
- `diff-phase.patch` is consistent with the four per-commit diffs: each file's index blob chains from one commit diff to the next.

**Q2.** Every line P3 requires is present:
- the two negative fixture cases with same-form controls;
- both samples on both targets in two run directories, three record pairs and the open check;
- the retag plant on a byte-built workbook;
- HOME and ANTHROPIC_API_KEY on labelled input lines in all five logs;
- the "committed" spec label;
- the spec hash pinned in every log;
- figures from other logs read from those logs.

Three checks do not test what they say, or test a superseded rule:
- the protect comparison's `hash` flag cannot fail on any output (F9);
- the link comparison the retag plant runs through still encodes the rules P2 ruling 2 replaced (F10);
- two printout test runs go without the feature (F11).

**Q3.** Almost every number, hash and quoted string is in a primary. Three are not: the "15 characters or fewer ... beyond it differs" sentence, "which all 27 corpus packages write", and "nothing pushed" (F13). One quoted message appears in the primaries only as doc text (F14).

## Details

### Q1: which diff I read for each "nothing else"

- **Doc commit.** `diff-doc-c9b313f.patch`: ROADMAP's P2 row, and the spec corrections in place that the P2 rulings name. The post-image blob equals the spec in `<check>`.
- **P2 carries.** `diff-bc46388.patch`, lines 1-381:
  - Ruling 2: `sheet_is_strict` (lines 95-119) supplies both the type form (line 87) and the retag form (line 130), with the strict-form test (lines 285-323). Plant M7 in `log-test.log` bites it.
  - Ruling 4: two tests (lines 325-368). The external case asserts "no such part" (line 367).
  - Ruling 6: `kept_ids` and the filter in `finish` (lines 134-220), with a test (lines 232-246). Plant M6 bites it.
  - Ruling 7: lower-case `r` and `c` (lines 377-378), and the two-sets test (lines 248-283).
  - The `no_links` doc comment is corrected (lines 33-36).
  - Nothing else, except F7.
- **Crate.** `diff-51acd32.patch`:
  - The read (lines 1032-1070).
  - The in-place replace keeping the start tag, opened up when empty (lines 1257-1287).
  - The removal (skip whole).
  - The append with the root's prefixes (lines 1203-1230).
  - The refusal when no prefix is bound (lines 1222-1225).
  - The creation with relationship and Override (lines 573-588).
  - Nothing touches `app.xml` or `custom.xml`.
  - Protection:
    - Sixteen options in schema order (lines 258-273).
    - The hash as restated (lines 297-307). I independently computed 83AF, CBEB and DAA7, and C678 for "emend"; the code's rotation also equals the MS verifier form for long passwords.
    - The password rule (lines 309-313).
    - Replace, drop and create for `sheetProtection` (lines 896-1003) and for `workbookProtection` (lines 777-829).
    - A second element refused (worksheet: `TWO_SHEET_ELEMENTS`; workbook: `validate` and rewrite).
    - `workbookProtection`'s forms (lines 864-879).
    - Neither is a formula change.
    - The one-shift refusal via `no_links` (lines 381-384), with a test.
- **Tool.** `diff-372bc21.patch`: the two samples and four usage-test lines, as "Harness runs" gives them; nothing else.

### Findings

**F1 (defect), `diff-51acd32.patch` lines 573-576, 676-686, 744-755.**
- Setup: a `set_property` on a package whose core-properties relationship exists but names an absent part.
- What happens:
  - `record_core_creation` returns because `core_part()` is `Some`.
  - The write's rewrite is skipped because `index_of` is `None`.
  - `validate` only refuses when the part is present.
  - The write succeeds and the recorded edits are silently lost.
- Why it is a defect: "Errors" reuses "the missing-part refusal of a relationship whose target is absent", and `properties()` refuses this case (test at lines 1510-1515). The write should refuse too.
- No test covers the write side.

**F2 (gap, needs a ruling), spec lines 353-356 against 381-392; `diff-51acd32.patch` lines 199-247.**
- The spec says `SheetProtection::default()` is "Excel's dialog default (everything locked but selecting locked and unlocked cells)". That locks `objects` and `scenarios`, which contradicts the exact bytes `<sheetProtection sheet="1"/>`.
- The code follows the bytes, and its doc comment says so.
- With `sheet` false the element is written bare, `<sheetProtection/>` (`log-test.log` line 3036). The spec says the element is written "as `sheet="1"` followed by ...".
- Both points are disclosed in `report-draft.md` lines 72-78.

**F3 (remark), `diff-51acd32.patch` lines 67-75.**
- `CORE_PROPERTIES` is a second public `package` constant; the spec's "Crate" (line 156) names one, `HYPERLINK`. "Document properties" does say "the type constant joins the catalogue's eight", so the spec is internally loose here.
- The constant also accepts the older officeDocument form, which is beyond the spec. Disclosed in the report.

**F4 (remark), `diff-51acd32.patch` lines 582-585 and 744-755.**
- Two refusals are not in "Errors":
  - "a text holds a character XML cannot carry" at `set_property`;
  - `REMOVED_PART_EDITED` for a core part the write removes or replaces.
- The text-at-call refusal has a test (lines 1729-1734). The core-part refusal has none, and the catalogue lines of `diff-51acd32.patch` lines 355-362 name no test for it.

**F5 (remark), `diff-51acd32.patch` lines 1075-1093 and 1150-1158.**
- `declared_prefix` returns an empty prefix for a default-namespace binding.
- If a root bound the xsi namespace as its default, an appended date would get an unprefixed `type=` attribute, which is not in the xsi namespace. No corpus shape does this.

**F6 (remark), `diff-51acd32.patch` lines 386-403.**
- `no_link_edits` now also covers properties and both protections. Its callers are outside the diff context, so I cannot confirm every caller wants that wider meaning.

**F7 (remark, low confidence), `diff-bc46388.patch` lines 166-198.**
- Attribute resolution now runs on every `<hyperlink>` in an edited container, kept ones included.
- A kept link carrying an attribute with an unbound prefix may now be refused where it was copied raw before. The earlier handling of kept links is outside the diff context, so I cannot confirm the previous behaviour.
- This is consistent with part 2's rule, but ruling 6 does not ask for it and the commit message does not state it.

**F8 (gap), `diff-51acd32.patch` lines 1541-1579 against spec lines 713-715 ("each replaced in place ..., removed, appended").**
- The write test replaces title, keywords and modified, removes creator, and appends category, created, language and revision.
- Subject, Description, LastModifiedBy, ContentStatus and Version are never written. The read test covers all thirteen.

**F9 (gap, Q2), `log-run.log` lines 2107-2116 and 3337-3354; `report-draft.md` lines 53-54.**
- `protectcmp.py`'s `hash` flag compares the log's own `legacy_hash("emend")` with a typed "C678". It does not depend on the output, so it could never fail on any workbook.
- Its "plant" changes the log's own password, not the output. "8 of 8 turned by a plant, the hash among them" therefore counts a self-test.
- The substance is covered elsewhere:
  - `line_protect` (line 466) checks the tool's printed `hash=C678`, with a C679 control;
  - `sheet_elem` and `book_elem` build the password bytes from the log's own hash.

**F10 (gap, Q2), `log-run.log` lines 2160, 2587, 2602.**
- The retag plant required by P2 ruling 8 runs through `linkscmp.py`. That comparison still takes the retag's declaration form from the root's namespace and the relationship type from the part's existing hyperlink relationships.
- These are the two signals P2 ruling 2 replaced with the worksheet's own relationship in the workbook's rels part.
- On the template both signals are transitional, so the plant passes. The check nevertheless encodes the superseded rule: on a strict-relationship workbook it would expect the wrong forms.

**F11 (remark, Q2), `log-test.log` lines 3134 and 3163.**
- The shift-table printout runs with `--features rowcol` only (546 tests), and the keyword-table printout with no features (513).
- P1 ruling 6 says every later log that runs the crate's tests runs them with the feature on. These are single-test printouts; planning should say whether the rule covers them.

**F12 (remark, Q2), `log-negative.log` lines 988-996.**
- The P1 and P3 checks are printed twice, once per control. This is cosmetic.
- P0, the template with no core part, creates the core part through the binary (`props_after=3`, lines 948-952), but its output bytes are not checked there.
- `report-draft.md` lines 94-95 say creation "is exercised by the byte-built tests alone"; the negative log's P0 also exercises it, unchecked.

**F13 (Q3, not found in any primary), `report-draft.md`:**
- Lines 79-81: "for passwords of 15 characters or fewer that equals the shift-and-fold form other writers use, and beyond it differs". No log measures this, and "other writers" is unnamed.
- Lines 86-87: "which all 27 corpus packages write". The run log shows only "a core part in 27 of 27" (`log-run.log` line 2735). Neither form of the type string appears in `m2p4a-p3-props-2026-09-28-372bc21.txt`.
- Line 8: "nothing pushed", and line 97. No log reads the remote.

**F14 (remark, Q3), `report-draft.md` line 85.**
- "an edit to a removed part" appears in the primaries only as doc text (`diff-51acd32.patch` catalogue and `set_property` doc). The value of the `REMOVED_PART_EDITED` constant itself is in no primary, and no test pins it.

**F15 (remark), `report-draft.md` lines 88-90; `diff-51acd32.patch` lines 1929-1934.**
- An unprotect of a sheet with no `<sheetProtection>` rewrites the worksheet part to its own bytes. The harness would list that entry as changed or recompressed.
- The corpus run does not exercise this case.

### Q3: everything else checked against a primary

All of the following were found:
- the four full commit ids and the base (`log-gate.log` lines 30-44);
- e2763566 and f5c74f60 (`log-gate.log` lines 1488-1516);
- the four log hashes (`log-scan-precheck.log` lines 1768-1798, and the file hashes in `<check>`);
- the six record hashes (`log-run.log` install lines; `log-scan-precheck.log`);
- 9 PASS, and 512/556 on both targets (`log-gate.log` lines 2059-2127);
- 760 passed, 0 failed, 2 ignored (`log-test.log`);
- 83AF, CBEB and DAA7 (`log-test.log` lines 3025-3027), the sixteen elements, the template parts, and the eight plants;
- the negative messages (`log-negative.log` lines 961-987);
- props totals 843/27/0/0 and 9 of 9; protect totals 816/54/0/0 and 8 of 8 (`log-run.log` lines 2904 and 3169; the plant blocks);
- the retag lines (`log-run.log` lines 3398-3442);
- the keep figures 431 and 50, none gone (`log-run.log` lines 3570-3572);
- the open-check figures 0 and 0 (`log-run.log` acceptance block);
- pre-check "RESULT: PASS", 0 hits, 0 structural failures (`log-scan-precheck.log` lines 1993-1998);
- the quoted spec phrase (spec line 355), and `<sheetProtection sheet="1"/>` and `<sheetProtection/>` (spec line 385; `log-test.log` line 3036).