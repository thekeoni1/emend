# Independent check 2, M2 part 4a P4

Written by the implementing session: the checked phase commit is
3e67745011d5a8e02a32de065175d3d0df3eb640, HEAD when the check ran. The
checker was a fresh general-purpose subagent given only the directory
check2 in this phase's folder on the disk, holding these files, by sha256
(check1's files, the report's draft 2 in place of draft 1, check 1's
answer as installed, and P3's committed gate.log):

    c7d3eb141faa2e4fcc326e7927e7689685c270190ac318b96ffa0b5312268a88  commit-messages.txt
    22a69dac0e389646816e3d26f95c89a253e3e0096c8f48cd90d5171441147628  diff-d3bb207-3e67745.patch
    c7b8ce19451a204a1f4b1e15d3a237582c14abea0b3fe346afd2f4582e1ef023  independent-check-1.md
    2ca7d8201d5cfe51e95f17c6182d67c680f78c3ae70836b099ea777cc19b9074  logs/gate.log
    1b12527e34c7ce2862ce1120b86b65c04986b6915eeb63cd782a44d95e8e47bc  logs/negative.log
    1952a8430d81b24338a0900fba4d245ebd610486aafcda9132910efbe7f37d41  logs/run.log
    fa331f7ad6ae499fa0972bccc3fac3f5e3519ba570da6aa26c8f0a2b5bfc7650  logs/scan-precheck.log
    187a44b259ffb6de644e89f054cc52de238320965dcb5d91135b48175b1c3f4c  logs/size.log
    b8c360183c4d6f4ba753c2908a5f2959d6216335ee1dc82b974443690805c3ef  logs/test.log
    7d1493d1c2c43cef6f00462fc20d2841f0aa4e945725e8541533c0156e53d9aa  p3-gate.log
    085b2a78d8a828a8fb2c7dda90c905fc96cc5c42b52ee584a6994b0d8528a7f0  records/m2p4a-open-2026-09-29-3e67745.md
    1fb5e7ded8c6ce3c7997085f9a85cf7c7749c4869eab299466be85d65d2d2eea  records/m2p4a-open-2026-09-29-3e67745.txt
    ac3810b2cd292dca3be5445c5612e784a5cb9731f50e91ec0bc82b5d4fa7a459  records/m2p4a-p4-link-2026-09-29-3e67745.md
    b7d1866d3f61d4e0e39d48eddce1c127b4f7a7c9944d037594bbe86f0d74d97c  records/m2p4a-p4-link-2026-09-29-3e67745.txt
    72e2367c0be324d5e3a53e1114bc1b4b8cb5410a850bc7f478c8a9771dc7fa7a  records/m2p4a-p4-names-2026-09-29-3e67745.md
    64a7ca95dfd72ed1721eb5e2be222d24dfa1faa8576ade6572502966445d777f  records/m2p4a-p4-names-2026-09-29-3e67745.txt
    25c2d6805ba89ed63357e3abe0271579aa24561633cb4b5f9666ea90b5638aa8  records/m2p4a-p4-props-2026-09-29-3e67745.md
    8965d72863f7f8ae470695e7befedf96fcbc0b5219fa78c277466a9f0701c643  records/m2p4a-p4-props-2026-09-29-3e67745.txt
    bf2ef0d468be79664bce6373f852b84b73c45c6307d281feb1107905bfbfc10a  records/m2p4a-p4-protect-2026-09-29-3e67745.md
    2f14bc6691844b9ff662253fd4b411be8f0dc2ec4eeae2c7f0b2b52eac558bb9  records/m2p4a-p4-protect-2026-09-29-3e67745.txt
    a41ebcc36b6c9e2a5369ee3fc9592b368344a7d0d1b00911c4168f98d9d682be  records/size-m2p4a-2026-09-29-3e67745.md
    330915a4aa654c642de78b140fa804d9547e58d57ccac329f9023e496be5b76f  report-draft.md
    401326fe5e290da1552e0d90ba7f7cd3a07166860b995ab615a2c8ca28430234  spec.md

The answer below the rule is the checker's final message, verbatim, taken
from its transcript (sha256 34ce0c0fe97cf9a31dd1a87a62c463ce326d24d9b8fc92658ebe7d09385b0179,
17877 bytes; plain ASCII; one change for the scan's
home-directory form: the answer's machine paths are written with <home>
for this machine's home directory (1 place) and <repo> for the
repository's root (1 place); nothing else is replaced, and the
transcript's hash is of the text before that change).

---

**Independent check 2, M2 part 4a P4 (phase commit 3e67745, base d3bb207)**

**Scope and method.** I read only the files in `<home>/.cache/emend-build/m2p4a-p4/check2/`, plus read-only `git show`/`git diff`/`git log` in `<repo>`. I read no corpus and ran no cargo.

Provenance checks:
- `git diff d3bb207 3e67745` is byte-identical to `diff-d3bb207-3e67745.patch`, confirmed with `cmp`.
- `p3-gate.log` (sha256 7d1493d1...) is the committed blob at `d3bb207:records/logs/m2p4a-p3/gate.log`.
- The spec hashes to 401326fe... in check2, at ab947b0 and at 3e67745.
- ROADMAP.md at ab947b0 hashes to fa374eb5....
- Every log and record in check2 has the same sha256 that check 1's file list gives (independent-check-1.md:10-30). So check 2 examined the same primaries as check 1. Only `report-draft.md` is new (330915a4...).
- The commit messages, the patch and the draft contain no non-ASCII bytes and no U+2014.
- Each commit message ends with the Co-Authored-By line alone, with no session link, as CLAUDE.md requires.

---

### Question 1: does the diff do what the spec and the release say, and nothing else?

**Commit by commit**

- **ab947b0 (doc).** It changes exactly `ROADMAP.md` (the P3 row only, patch 1-12) and the spec (patch 287-605). The resulting hashes are fa374eb5 and 401326fe. The gate pins the spec against "the hash planning's message gave" (gate.log:23-28).

- **4ece0f0 (first code commit).** It touches only `crates/emend/src/package.rs` and `crates/emend/src/package/tests/properties.rs`. Filtering the `+`/`-` lines of package.rs that are not `//!` leaves exactly two changes:
  - **Release item 1.** The validate check sits at patch 116-127: `#[cfg(feature = "metadata")] if !self.edits.properties.is_empty() && let Some(part) = self.core_part()? && self.archive.index_of(&part).is_none() { return Err(NO_SUCH_PART); }`.
  - **Release item 3.** The `.filter(|prefix| !prefix.is_empty())` on `prefixes[3]` sits at patch 134-149.
  - The four release items are covered by four `#[test]` functions:
    - (1) patch 161-173, a loop over `[Some("t"), None]` at 164;
    - (2) patch 176-190, `remove_part` at 179 and `replace_part` at 185;
    - (3) patch 193-218, with its control at 207;
    - (4) patch 221-285, replace, remove and append against expected bytes.
  - Every test reads its message from a literal in the test, as ruling 1 asks.

- **fb34ab0 (catalogue).** Every `+` line is `+//!`; filtering them leaves 0 lines. The added block is patch 49-114. It covers each item the spec's P4 lists (spec.md:763-767) and each statement a ruling assigns to the catalogue:
  - the names read's refusals (P1 ruling 6 G3, spec.md:924-929);
  - `Workbook` not constructible (P1 ruling 8);
  - the added-sheet link rule (P2 ruling 4, spec.md:1004-1009);
  - `sheet=false` (P3 ruling 3);
  - the unprotect rewrite (P3 verification ruling 7);
  - the missing core part.

- **3e67745 (probe).** It changes only `tools/size-probe/Cargo.toml` (patch 606-629: feature `metadata = ["emend", "xml", "emend/metadata"]`, `[[bin]]` with `required-features = ["metadata"]`) and adds the new binary.
  - I diffed the binary against `probe-emend-format-write-xml` at 3e67745. The only differences are:
    - the header doc;
    - `USAGE` naming its own binary;
    - three new arguments;
    - the five calls: `add_defined_name("emend_probe", name_text, None)`, `set_hyperlink(.., LinkTarget::External(name_text), None)`, `set_property(Title, ..)`, `protect_sheet(.., SheetProtection::default())` and `protect_workbook(.., true, false)`;
    - two read-backs, `defined_names().len() as u64` and `properties().get(Title) == Some(..)`, printed as `names` and `title`.
  - That matches "Size" (spec.md:493-514).

**Findings**

**F1 (defect, still standing; check 1's F1).** P3 ruling 1 (spec.md:1069-1071), verification ruling 10 (spec.md:1202-1206) and the release all ask for "two tests, one a set and one a removal". Ruling 5 and the release ask for "one test each" for `remove_part` and `replace_part`. The diff instead has:
- one test function that loops over a set and a removal (patch 161-173);
- one test function that covers both `remove_part` and `replace_part` (patch 176-190).

The primaries agree that there are four new tests, not six:
- gate.log:2072 and 2082 show 560 on, against p3-gate.log:2110 and 2120 at 556;
- test.log:1968 and 2780 show 764;
- run.log:4663 reads "tests added or edited: 4".

This is a defect of form, not of substance: every case is asserted. It stands until planning accepts it or the tests are split.

**F2 (remark; check 1's F2).** 4ece0f0 also edits three `//!` error-catalogue entries (patch 18-47) against the release's "Nothing else in that commit". The commit message says so ("The catalogue names the new tests", commit-messages.txt:51-52), and the draft now says so too (report-draft.md:23-24).

**F3 (remark, new).** Under plant N4 the `remove_part` case does not write. It fails with `Unsupported("two changes for one entry")` (test.log, the N4 planted panic, properties.rs:337), not with a silent write. So the ruling-5 test for the removed core part pins which message wins, not that the edit would otherwise be lost. The later layer already refuses the write. This supports keeping the validate check, but "untested" (ruling 5's word) overstated the risk. The `replace_part` half has no plant of its own (check 1's F3).

**F4 (remark, restating check 1's F4, confirmed from the source).** The new check tests `self.archive.index_of(&part)`, the input archive. A write that `add_part`s the missing core part and sets a property is therefore refused with "no such part". Before the fix, the same case would have lost the edit, so this is not a regression. But no test and no spec sentence covers it.

**F5 (remark, new, small).** The ruling-6 test's control (patch 207-217) is a non-dated `Title` append on the same root. It shows that the refusal is specific to dated appends, but not that a dated append succeeds when `xsi` is prefixed. That positive case is covered elsewhere: the thirteen-property append at patch 280-284 appends `dcterms:created xsi:type=...` under the fixture root that binds `xsi:`. Plant N2 turns the test (test.log, N2 planted exit 101, properties.rs:358). Adequate.

**F6 (remark; check 1's F5).** The probe passes the name text as the external hyperlink's target (patch 998) and as the title. The target is not a URL. That is harmless for size, and the spec leaves the target free.

Apart from F1 and F2 (form), the diff does what the spec and the release ask and nothing else.

---

### Question 2: do the logs contain every line the spec's P4 section and the release require?

**Present, with evidence**

- **HOME and ANTHROPIC_API_KEY on labelled input lines, every script:**
  - gate.log:6-7
  - test.log:7-8
  - negative.log:5-6 (and ":49 ANTHROPIC_API_KEY set in this shell: no")
  - run.log:15-16
  - size.log:11-12
  - scan-precheck.log:10-11
- **Spec pin 401326fe:**
  - gate.log:23-28
  - test.log:24-30
  - negative.log:58
  - run.log:4 and 1570-1573
  - size.log:4
  - scan-precheck.log:8
- **Attempt 1.** RERUN_NOTE and RERUN_FILES are unset in all six logs:
  - gate.log:4-5
  - test.log:4-5
  - negative.log:3
  - run.log:8-9
  - size.log:5-6
  - scan-precheck.log:6-7
- **Standard error 0 bytes:**
  - gate.log:2170
  - negative.log:1626
  - size.log:4825
  - run.log:5311
  - test.log, the stderr section
  - scan-precheck.log, the stderr section
- **Gate:**
  - nine PASS lines (gate.log:2008-2016, 2019);
  - 512 off and 560 on per target (2084);
  - debug emend-copy hashes (2024, 2026);
  - "gate log checks all as expected: yes" (2174).
- **Test:**
  - `--workspace` 764 passed, 0 failed, 2 ignored per target (1968, 2780), with the same test names on both (3226-3227);
  - legacy hashes 83AF, CBEB, DAA7 (2937-2939);
  - N1-N4 each "found once, ... fails planted: yes" (3182, 3193, 3206, and the N4 line);
  - F11: the shift printout runs with `--features rowcol,metadata` (3045) and the keyword printout with `--features metadata` (3074);
  - "test log checks all as expected: yes" (3239).
- **Negative:**
  - twenty-three fixtures (1226);
  - five of the eleven reached (1568-1569);
  - the six at-call or unreachable messages, each with its reason (1561-1567), as the spec's "the log says so" requires;
  - M1 (P3 ruling 1) exits 2 with exactly NO_SUCH_PART's message and no output, against the controls P2 and P0 (1445-1450);
  - the change AB note (1430);
  - P0's created core part compared by sha256 with a control (1548-1552, change Y);
  - "negative log checks all as expected: yes" (1630).
- **Size:**
  - 17 probes built (3135);
  - 16 compared with part 3's record and 1 new (3193-3197);
  - feature cost 50404, 53248, 41648 (3818);
  - core growth 968440 - 968012 = 428, file 0, .text 192 (3820);
  - "above 2000 ...: no" (3821), with a control (3833);
  - whole growth 556 (3823);
  - budget 435224, 23528 under 458752 (3825-3826), against part 3's 434668 (3827);
  - the 128 beside it (3834);
  - release binaries on `hash:` lines (4623-4628);
  - probe output `names 1` and `title 1` (4544-4545, 4575-4576);
  - the record installed with sha256 a41ebcc3 (4810);
  - "size log checks all as expected: yes" (4830).
- **Run:**
  - four run directories named (66-69);
  - the binaries checked against size.log's hash lines (1581-1591);
  - each of the four phase directories is "the one planning named" (3527-3528, 3752-3753, 3981-3982, 4204-4205);
  - 27 of 27 byte-identical per sample and label (3570, 3646, 3795, 3871, 4024, 4100, 4247, 4325);
  - lo and lo_in ok 27 of 27 (3565 and following);
  - outputs the same on both targets (3716, 3945, 4168, 4395);
  - plants 10/10, 12/12, 9/9, 8/8 (3711, 3940, 4163, 4388);
  - F9 `hash_sheet` (4390-4391);
  - F10 retag, strip and strict-relationship controls (4474-4489);
  - keep 435 pairs, 431 unchanged, 4 new, 0 gone (4604-4621);
  - open check: crate 378, tool 57, 0 refused, 0 not run (5256-5260);
  - five record pairs installed (3970-3973, 4193-4196, 4420-4423, 5281-5286, and the names pair);
  - "run log checks all as expected: yes" (5315).
- **Scan pre-check:**
  - the G3 fix, change AY (310-321, 1076-1079, 1962);
  - "RESULT: PASS", 0 hits, 0 structural failures (2064-2068).

**Required but not met to the letter**

**F7 (remark; check 1's F6).** The spec's P4 and verification ruling 7 say P0's created part is compared "to the bytes the crate test pins". negative.log compares it instead with bytes a Python builder makes (1477-1552).

Correcting check 1 in part: the builder's element bytes are typed literals (the `body = (b"<dc:title>emend sample</dc:title>...` line), and only the five namespace URIs are read from the source by regex. So it is more independent of `new_core_part` than check 1's "largely self-consistent with the source" suggests. It is still a departure planning must rule on.

**F8 (remark; check 1's F7).** M1 reaches "no such part" through the sample's read before any edit (negative.log:1430). The write-time refusal of ruling 1 is reached only by crate tests, with N1 as its plant.

**F9 (remark; check 1's F9).** The spec is inconsistent with itself: its P4 says "seventeen probes classified", while "Size" says sixteen. size.log follows "Size" (3193-3197).

**F10 (remark; check 1's F8).** The G3 control (scan-precheck.log:1077-1079) gives a two-line listing with count 2, which `only_spec` refuses on the count alone. The control never exercises the content comparison. `only_spec` is also not on this run's path, since the AQ branch ran (1963).

**F11 (remark; check 1's F10).** The release attributes F11 to "the run script", but it is carried by the test script (test.log:3045, 3074).

The final `scan.log` is absent, as expected, because the scan runs last. Apart from F7 and F8, which are departures of wording to rule on, nothing the spec or release requires is missing.

---

### Question 3: does every number and quoted string in report-draft.md appear in a primary?

**Checked and matching**

| Draft line(s) | Claim | Primary |
|---|---|---|
| 5 | 401326fe | the spec's hash |
| 10 | fa374eb5 | ROADMAP.md at ab947b0 |
| 12-24 | 4ece0f0 as one test function per ruled pair; "Four new test functions"; "560 ... from P3's 556"; three catalogue entries | patch 18-47 (three entries: 72-73, 80-82, 374-375 of package.rs) |
| 27 | full sha 3e67745011d5... | git |
| 33-35 | 2ca7d820; "9 PASS"; 41aae565, f324def5 | gate.log |
| 36-40 | b8c36018; 764 and 2; 83AF, CBEB, DAA7; the N1-N4 quote; F11 with metadata | test.log |
| 41-49 | 1b12527e; "the twenty-three fixtures"; "five of the eleven reached, as P4 counts them: yes"; "controls P2 and P0"; the change AB and change Y wording | negative.log:1430, 1445-1450, 1548-1550 |
| 50-54 | 187a44b2; a41ebcc3; 7f25fb2a, 89d6f20d, 1445523d, 99d1661d | size.log |
| 55-70 | 1952a843; 27 of 27 with the four directory names; entry totals 816/45/9/0, 822/48/0/6, 843/27/0/0, 816/54/0/0; plants; r_bound_in and w_strict; 435, 431, 4; 378 and 57, 0 and 0 | run.log |
| 71-76 | all ten record hashes | records |
| 77-78 | "RESULT: PASS", 0, 0 | scan-precheck.log |
| 83-95 | 50404, 53248, 41648; 968440, 968012, 428, 0, 192; 556; 967884; 128; 435224, 23528, 458752, 434668 | size.log:3818-3834 |
| 127 | 973348 and 753171 | size.log:3337 |

I also checked the draft's inference at lines 91-93. `git log b728fdf..bcaa31d -- crates/emend tools/size-probe Cargo.toml Cargo.lock` lists bcaa31d alone, and its package.rs change is `//!` lines only. The 128 therefore comes from bcaa31d's change to the probe's own source. The reading is sound and is correctly labelled as the report's own.

**Findings**

**F12 (remark).** Draft line 99 lists "Check 1's F1 and F11 (defect of form)" as still open. F11 was a defect in the report's wording, and draft 2 has corrected it (lines 12-21 now say one test function each). The line should say that F11 is resolved in the report and that only F1 stays for planning. As written it slightly misstates the state of check 1's F11.

**F13 (remark).** Draft lines 127-129 say the whole growth's file and .text cells "are taken from part 3's record row ... (973348 and 753171)". The record's cells are 0 and 288 (size record line 99). They are computed against those two figures, not the figures themselves. "Computed against" would be accurate.

**F14 (remark, unsupported for now).** Draft lines 77-79 ("the final stage ... is the committed scan.log") and line 138 ("STOPPED after the record commit") describe artifacts that no primary yet holds. The final report must add the final scan.log's hash and result and the record commit's sha.

**F15 (remark, omission).** The draft never states the LibreOffice results: lo and lo_in ok 27 of 27 for every label and target (run.log:3565 and following; the records' "Files by outcome"). CLAUDE.md makes these part of acceptance. One line would carry them.

**F16 (remark, planning-relevant omission).** The feature's cost of 50404 section bytes is about 2.5 times the top of planning's estimate of 15000 to 20000 (spec.md:76-83). The spec says nothing hangs on the estimate, but planning would want the ratio stated beside the figure.

**F17 (remark, how check 1 is carried).**
- **Carried fairly:** F1 (lines 99-108), F2 (lines 23-24), F3 (line 106), F4 (lines 117-119), F6 (lines 109-113), F7 (lines 114-116), F9 (lines 120-122). F12 is handled at lines 91-93, and F13 is fixed at line 43.
- **Not carried:** F5, F8, F10 and F14.
  - F8 is planning-relevant, because it bears on whether planning's G3 ruling is exercised as meant (see F10 above).
  - F10 corrects the release's own attribution.
  - F14 is a wording flaw in a record that size.log has already hashed.
  - These should appear under "For planning", at least as one-line remarks.
- **F6 caveat dropped.** The draft omits check 1's caveat that only the dcmitype control shows the comparison can fail. Given F7 above, the draft could say instead that the element bytes are typed literals.
- **Nothing contradicts check 1.** I found no line in the draft that misstates check 1 beyond F12.

**F18 (remark, process).** CLAUDE.md requires each check's answer in the report verbatim. The draft cites check 1 by finding number only. The final report must carry independent-check-1.md's answer and this one verbatim.

No number or quoted string in the draft differs from its primary.

---

### Verdict

The four commits touch what the release names:
- the catalogue commit is doc lines only;
- the probe commit is the probe and its Cargo wiring only;
- the first code commit carries release items 1 to 4, plus three catalogue lines that name the new tests (F2).

The one standing defect is of form: one two-case test for the missing core part and one two-case test for removed and replaced, not the two tests and one-test-each that were ruled (F1). The report now states this correctly.

Every log carries the labelled HOME and ANTHROPIC_API_KEY lines, the spec pin, attempt 1, 0-byte standard error and its "all as expected: yes". Also present:
- the F9, F10, F11 and G3 fixes;
- the binary's M1 refusal;
- all four samples on both targets, 27 of 27 byte-identical to the four named phase directories, run with the size log's hashed release binaries;
- every figure the spec asks for.

Two things depart from the spec's wording and need planning's ruling: the P0 comparison against rebuilt bytes rather than the crate test's pin (F7), and M1 reaching the read's refusal rather than the write's (F8). Every number and quoted string in the draft matches a primary.

Before the final report, the draft should:
- mark F11 as resolved (F12);
- reword the cell provenance (F13);
- fill in the scan and record-commit facts (F14);
- add the LibreOffice results (F15) and the cost against the estimate (F16);
- carry check 1's F8 and F10 (F17);
- include both checks verbatim (F18).
