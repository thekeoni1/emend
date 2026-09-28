# Independent check 2, M2 part 4a P3

Written by the implementing session: the checked phase commit is
372bc214503fd4fdca25fe69b5b6efd409b29ff5, HEAD when the check ran. The
checker was a fresh general-purpose subagent given only the directory
check2 in this phase's folder on the disk, holding these files, by sha256
(the logs there are the four committed logs, each attempt 1, and the scan
pre-check's attempt 2 with attempt 1 and its script under superseded/;
the spec as committed at the phase commit and planning's uncommitted
working copy; no log was rerun after check 1):

    089a678456ea5b2a448037213683b06300208233bd4624317124fd3153acfd0d  commit-messages.txt
    8c68e039c9d1ee539a0fb2c232ab9026294820e571d7e49f56ee357f637b767b  diff-372bc21.patch
    941b484ebc412bcf24e7a2fa0b4138e9fc9377c35f0fb38371e71ea7fcb2ee05  diff-51acd32.patch
    9ee3b5f020779af18b22a202f7bd97455df133fed134307415deab480e117a09  diff-bc46388.patch
    d5a18e8e46cbc5c37fc3ba671950a66c2b0ab0fa030b15afc4f40fb03080fc22  diff-commands.txt
    030aa7ef5686b5137e0078e9439109d600227859984ffd3b0022c6992425fcce  diff-doc-c9b313f.patch
    07afd76830b42c6f0399621feda737b936bc06d1aae2da2bad1e99c2cef483ab  diff-phase.patch
    7c112f5886aa2e3d348a7dbb93e211268f93393b75d280457dbbefdfe13c65e5  independent-check-1.md
    7d1493d1c2c43cef6f00462fc20d2841f0aa4e945725e8541533c0156e53d9aa  log-gate.log
    77c9168ddd4281788a06542a8085e16fd0116786b9455d629a9205586d7fa1dc  log-negative.log
    9b4e75f491e1aa92176d0434f3cd6359ab2c2bb208739444f3c353cade6fbca0  log-run.log
    6a9541e07c364ed0b2e4eea0d629deb9f89406369eb2291c95bdf339c2b308c1  log-scan-precheck.log
    2846d9f8c6cc056296677396371f0b7f4651cc313b9698c0206c0d27c8d9619a  log-test.log
    cce49be67eb2801601bedc9520f5a6c6d8ca0844811aeb04284e4461974809ec  m2p4a-open-2026-09-28-372bc21.md
    956a27ab0cd1ccece6fe43c10d43e31b31d9f1542b76673d00555fb8f57008e1  m2p4a-open-2026-09-28-372bc21.txt
    3327e6347b676fae80c0568e354add380f5540098af747d3b7623f0485e681a6  m2p4a-p3-props-2026-09-28-372bc21.md
    1deb3d3236be5a3b775552f42b308788079a0559ad10dadf0a07c231bcb234af  m2p4a-p3-props-2026-09-28-372bc21.txt
    7e0026f3eba2fd8430f0e332365b83351cb78b8d4a7533c562b2ab1e503ed688  m2p4a-p3-protect-2026-09-28-372bc21.md
    3980432cd1d0f5f5b600126733ee9ddbd2fea37e1b71d73636fd8519fb14dd08  m2p4a-p3-protect-2026-09-28-372bc21.txt
    ecaeeb75e28f2665b181d6786c34c8a433b731bd9c116a9641e3131c0cb4d0a7  report-draft-2.md
    79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c  specs-0001-bootstrap.md
    541003c1280e93fad39ec97767d539f1263b2799b9917156fd3c55f92bb9b4e1  specs-0018-rows-and-columns.md
    e276356647ce82d213b96be8c852ba715892696b69cc6b2331bedd454387bcb1  specs-0019-names-links-properties-protection.md
    1caf5a9698cc16f25264008a2a0f1bfedbb4fddc128a8e65bf0d227b28a2d8b1  specs-0019-names-links-properties-protection.working.md
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-gate.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-negative.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-run.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-scan-pre-check.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-test.txt
    60e3bda4949ce9e05989941016f5d92c1e91417a16ddaaab4fd26e935dbf81e7  superseded/m2p4a-p3-scan.attempt1.sh
    a2a6d979f4ffa821efc9865163585f856c4b5300515a7db0481dfd86df6215b4  superseded/scan-precheck.attempt1.log

The answer below the rule is the checker's final message, verbatim, taken
from its transcript (sha256 f0deb0801ff09f1d497e6564459a0c296a983e99cb5c80bff58a28f1788d3eb7,
11868 bytes; plain ASCII, nothing replaced).

---

**Independent check 2, M2 part 4a P3 (document properties and protection)**

Context disclosure: the harness put a CLAUDE.md, a memory index, a git status snapshot, attribution text and environment details into my prompt. I used none of it as evidence. I read only the files in `<check>`. Every input matches `../check2-inputs.sha256`. The committed spec's git blob hash is d1b3779, the post-image blob in diff-doc-c9b313f.patch line 15, and its sha256 is e2763566.... The working copy's sha256 is 1caf5a96..., the PLANNING_SPEC value in log-scan-precheck.log line 9.

## Short answers

**Q1.** Yes, apart from one defect that is ruled and two gaps in the rulings.
- The diff does what P3 and the P2 rulings that bind it (2, 4, 6, 7) say, and nothing else beyond what check 1 found.
- I agree with check 1's F1 defect, confirmed from diff-51acd32.patch, and with rulings 1 to 9 as they apply.
- Gap: check 1's F6 is neither ruled on by planning nor carried by the report (G1).
- Gap: ruling 4's correction leaves the working spec's "Crate" paragraph naming one `package` constant (G2).
- The "nothing else" diffs I read:
  - diff-doc-c9b313f.patch: exactly ROADMAP.md and the spec.
  - diff-bc46388.patch: the P2 carries, plus the kept-link attribute resolution that ruling 7 accepts.
  - diff-51acd32.patch: the crate.
  - diff-372bc21.patch: the tool and four usage-test lines.
  - diff-phase.patch: its index blobs chain from one commit diff to the next.

**Q2.** Yes: every line P3 requires is present, and the logs' checks test what they say, apart from check 1's F9 to F11, which ruling 9 covers.
- Change AV is what the rerun note says and nothing else. Diffing the attempt 1 log against attempt 2 shows only:
  - the input lines;
  - the AV comment;
  - the hash-check branch;
  - the planning-files branch;
  - the status-pattern alternative;
  - the rerun lines;
  - the timestamps.
- The printed attempt 1 script in superseded/scan-precheck.attempt1.log equals superseded/m2p4a-p3-scan.attempt1.sh, apart from one trailing blank line. The rerun lines hash both superseded files correctly (a2a6d979..., 121493 bytes; 60e3bda4..., 70909 bytes).
- AV's hash check and its control test what they say.
- AV's other two checks have no control, although the AV comment says "each check" has one, and two AQ labels are now stale (G3).

**Q3.** Almost every number, hash and quoted string in report-draft-2.md is in a primary. Missing or wrong:
- F6 is absent from the report's list (G1).
- The record-side statement of the F1 defect that ruling 1 asks for is in no primary yet (G4).
- "Nothing pushed" is still in no primary (G5).
- Line 18 puts the seventh negative fixture case in P4's first code commit, where ruling 1 puts it in P4's negative log (G6).
- The final-stage sentence on lines 85-87 describes a run that has not happened (G5).
- The report carries F1 to F5 and F7 to F15 as ruled.

## Details

### Q1: what I read, and my own checks of the diff

**diff-51acd32.patch** (crate):
- `legacy_password_hash` (lines 297-307): the rotation matches the restated rule, including shift 0 at position 15. The test log's M1 plant (log-test.log lines 3265-3277) shows 33711 = 0x83AF.
- `password_ok` counts bytes. That equals characters because only 0x20-0x7E is accepted.
- `SheetProtection::default()` and `options()` give the schema defaults. That is ruling 3, as corrected in the working copy at lines 364-368 and 392-398.
- `set_property` refuses a bad date form, then a forbidden character, at the call (ruling 5).
- `record_core_creation`, the `write_to` rewrite guard and the `validate` guard confirm F1:
  - lines 573-576: creation returns because `core_part()` is Some;
  - lines 676-686: the rewrite needs `index_of` to be Some;
  - lines 744-755: `validate` needs `index_of(&part).is_some()`.
  - So a property edit on a package whose core relationship names an absent part is silently dropped. I agree it is a defect and agree with ruling 1's refuse-not-create reading.
- The workbook protection path: pending creation only when the element is absent; no rewrite for an unprotect with none; TWO_BOOK_ELEMENTS both in `validate` and in the rewrite. It matches "Protection".
- `ProtectPass` creates the element before the first later root child. The tests pin protectedRanges, mergeCells, pageMargins and a created mergeCells (lines 1929-1956 of the protection tests). `sheet_is_strict` and the kept ids in bc46388 match P2 rulings 2 and 6.
- One remark of mine, not in check 1: `rewrite_core` opens up an empty-element start tag (`opened_up`) when it gives the element content. The spec's "start tag kept byte for byte" cannot hold for an empty-element tag given content, so this is the only possible reading. It needs no action.

**diff-372bc21.patch** (tool):
- The two samples match "Harness runs": W found as the first sheet of kind Worksheet, the three values, the counts, the assertions, and the output removed on a difference.
- The usage string and four usage-test lines are added, and nothing else.

### Findings

**G1 (gap).** Check 1's F6 is neither ruled on nor carried forward.
- independent-check-1.md line 138 is F6: `no_link_edits` is widened to cover properties and both protections (diff-51acd32.patch lines 386-403), and its callers lie outside every diff in `<check>`.
- The working spec's rulings (lines 1058-1118) rule on F1 to F5 and F7 to F15 but never on F6.
- report-draft-2.md lines 121-131 list every finding except F6.
- I cannot see the callers either: grep over diff-phase.patch finds only the two definitions (lines 411 and 423).
- The widening therefore changes behaviour at call sites no primary shows. Planning should rule on it, and the report should carry it.

**G2 (gap).** Ruling 4's correction is incomplete.
- Working spec lines 149-163 ("Crate") still say "one `package` constant, `HYPERLINK`".
- Ruling 4 (line 1086) corrects only "Document properties" (lines 308-315).
- `CORE_PROPERTIES` is a second public constant (diff-51acd32.patch lines 67-75), which is exactly check 1's F3 first point. This is where the ruling's preamble (lines 1052-1056) says planning revisits a statement that differs.

**G3 (remark, Q2).** Change AV's checks and labels in log-scan-precheck.log.
- (a) The AV comment (line 303) says "A control shows each check refuses another hash". Only the hash comparison has a control (lines 639-641, 68). Two AV checks have none:
  - `only_spec` (lines 1054-1055, 1942);
  - the widened OURS_RE (line 1060).
  - The existing control (line 1067, output line 1945) uses a listing with ` M crates/emend/src/lib.rs`. It never shows that the new alternative accepts only specs/0019, for example that ` M ROADMAP.md` is still refused. `only_spec` covers that case in practice through nplan (1 in this run).
- (b) Two labels are now inaccurate:
  - line 1943, "of them this phase's log directory or records: 8", counts the modified spec among the 8;
  - line 1944, "the working tree holds nothing but this phase's log directory and records (change AQ): yes", is printed while the spec stands modified.
  - The comment at lines 1050-1052 still states AQ's premise.
- None of this changes the result. The change otherwise matches the rerun note (lines 6, 53), and the pin check on the phase commit's copy stays (line 645).

**G4 (gap, Q3).** Ruling 1 (working spec line 1058) asks that the defect be stated "in the report and the record".
- The report states it (lines 8-19).
- None of the six records mentions it: grep for "defect" and "no such part" finds nothing, and their hashes are fixed by the scan.
- The report does not say where the record-side statement will go, presumably the record commit's message. No primary in `<check>` shows it yet, so planning should verify it at the record commit.

**G5 (remark, Q3).** Two statements are in no primary.
- report-draft-2.md line 133, "Nothing pushed". Check 1's F13 flagged it and it remains. Ruling 10 orders no push but does not measure one.
- Lines 85-87: "The final stage follows the checks, its rerun lines hashing both attempts, and is the committed scan.log." This describes a future run that is not in `<check>`.

**G6 (remark, Q3).** report-draft-2.md lines 17-19 say the refusal "with two tests and a seventh negative fixture case, lands in P4's first code commit".
- Ruling 1 (lines 1058-1071) puts the two tests in P4's first code commit and the seventh case in P4's negative log through the binary.
- Working spec line 772 (P4) agrees with the ruling.

**G7 (remark, for planning).** The P4 count in the working copy does not include the two-title case.
- Working spec lines 772-777: "the six fixture cases of P1 to P3 and a seventh".
- P3's negative log also reached CORE_TWO through the binary with the two-title fixture p3 (log-negative.log lines 996-999, 1019-1020), a case beyond P3's two required ones.
- If P4 runs "every refusal of this part reachable through the binary", that case makes eight.

**G8 (remark, Q3).** report-draft-2.md line 51 says the eight plants each fail "with its message".
- Each plant check in log-test.log (for example line 3277) asserts only "passes unplanted and fails planted".
- The panic message is printed (lines 3273-3276) but not checked.

**G9 (remark).** The feature-on crate count of 556 (report line 44) is 21 over P2's 535, while the phase adds 22 crate tests (log-test.log lines 2877-2898).
- The difference is `a_protection_on_a_part_with_a_pending_shift_is_refused`, gated on `rowcol` (diff-51acd32.patch, protection tests, `#[cfg(feature = "rowcol")]`).
- It runs only in the workspace run (760), the same arrangement P2 ruling 8 accepted for the link test.
- The report does not say this.

### Q3: items checked and found in a primary

- **Hashes and commits:**
  - the spec hash e2763566 (log-gate.log);
  - the four commit ids and the base (log-gate.log lines 30-44);
  - f5c74f60 (log-scan-precheck.log line 1590);
  - the four log hashes and six record hashes (log-scan-precheck.log lines 1797-1904, and the file hashes).
- **Gate and tests:**
  - 9 PASS (log-gate.log lines 2058-2069);
  - 512 and 556 on both targets (log-gate.log lines 2103-2118);
  - the pub fn and const lines and the consumer crate (log-gate.log lines 1836, 2130-2179);
  - 760, 0 and 2: the sum of log-test.log lines 1779-1965 on each target;
  - 83AF, CBEB and DAA7 (log-test.log lines 3026-3028);
  - the sixteen elements and the template parts (log-test.log lines 3034, 3059);
  - the eight plants (log-test.log lines 3265-3366).
- **Negative log:**
  - the three messages (log-negative.log lines 990-1001);
  - the gate binary checked by hash (log-negative.log lines 802-812);
  - P0's props_after=3 (log-negative.log line 948 area).
- **Corpus run:**
  - props 843, 27, 0, 0 and 9 of 9 (log-run.log lines 2902, 3087-3088);
  - protect 816, 54, 0, 0 and 8 of 8 (log-run.log lines 3170, 3353-3354);
  - outputs identical across targets (log-run.log lines 3093, 3359);
  - the retag lines (log-run.log lines 3433-3442);
  - the keep figures 431 and 50, with none gone (log-run.log lines 3566-3567);
  - open-check 0 and 0 for crate and tool (log-run.log lines 4321-4325).
- **Scan pre-check:**
  - attempt 1 "RESULT: PASS" with 0 and 0 (superseded/scan-precheck.attempt1.log tail);
  - attempt 2 "RESULT: PASS" (log-scan-precheck.log last line).
- **Design choices:**
  - `<sheetProtection sheet="1"/>` and `<sheetProtection/>` (spec; the protection tests);
  - the quoted spec phrase (committed spec lines 355-356);
  - "a text holds a character XML cannot carry" and REMOVED_PART_EDITED (diff-51acd32.patch);
  - the verifier-form remark (independent-check-1.md line 97);
  - the unprotect behaviour (independent-check-1.md F15; the protection test).
- **Rulings:** the ruling numbers against check 1's findings F1 to F5 and F7 to F15 match working spec lines 1058-1118. F6 is the exception (G1).