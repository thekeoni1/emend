# Independent check 1, M2 part 4a P4

Written by the implementing session: the checked phase commit is
3e67745011d5a8e02a32de065175d3d0df3eb640, HEAD when the check ran. The
checker was a fresh general-purpose subagent given only the directory
check1 in this phase's folder on the disk, holding these files, by sha256
(the logs there are the five committed logs, each attempt 1, and the scan
pre-check's attempt 1, which the final scan.log's rerun lines hash):

    c7d3eb141faa2e4fcc326e7927e7689685c270190ac318b96ffa0b5312268a88  commit-messages.txt
    22a69dac0e389646816e3d26f95c89a253e3e0096c8f48cd90d5171441147628  diff-d3bb207-3e67745.patch
    2ca7d8201d5cfe51e95f17c6182d67c680f78c3ae70836b099ea777cc19b9074  logs/gate.log
    1b12527e34c7ce2862ce1120b86b65c04986b6915eeb63cd782a44d95e8e47bc  logs/negative.log
    1952a8430d81b24338a0900fba4d245ebd610486aafcda9132910efbe7f37d41  logs/run.log
    fa331f7ad6ae499fa0972bccc3fac3f5e3519ba570da6aa26c8f0a2b5bfc7650  logs/scan-precheck.log
    187a44b259ffb6de644e89f054cc52de238320965dcb5d91135b48175b1c3f4c  logs/size.log
    b8c360183c4d6f4ba753c2908a5f2959d6216335ee1dc82b974443690805c3ef  logs/test.log
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
    1cf2f3e69efbc96c2ccadc6a4c5f9f72ddee443ccb069cb3167f552240d5ba15  report-draft.md
    401326fe5e290da1552e0d90ba7f7cd3a07166860b995ab615a2c8ca28430234  spec.md

The answer below the rule is the checker's final message, verbatim, taken
from its transcript (sha256 eef9ea67ce19cfc730e8dd568db4ee4f36d6e94b24b6d3745c8a010b7364a8d9,
13874 bytes; plain ASCII; one change for the scan's
home-directory form: the answer's machine paths are written with <home>
for this machine's home directory (1 place) and <repo> for the
repository's root (1 place); nothing else is replaced, and the
transcript's hash is of the text before that change).

---

**Independent check 1, M2 part 4a P4 (phase commit 3e67745, base d3bb207)**

I worked only from `<home>/.cache/emend-build/m2p4a-p4/check1/` and from read-only `git show`/`git diff` in `<repo>`. I read no corpus. `git diff d3bb207 3e67745` is byte-identical to `diff-d3bb207-3e67745.patch`. Every sha256 of a log and record in check1 matches the prefix the report gives. The spec hashes to 401326fe..., both in check1 and at ab947b0 and 3e67745.

### Question 1: does the diff do what the spec and the release say, and nothing else?

**Commit by commit**

- **ab947b0 (doc).** Two files only: `ROADMAP.md` (sha256 fa374eb5...) and the spec (401326fe...). The message is ASCII and ends with the Co-Authored-By line alone.
- **4ece0f0 (first code commit).** Touches only `crates/emend/src/package.rs` and `crates/emend/src/package/tests/properties.rs`.
  - Item (1): `validate` returns `NO_SUCH_PART` when `properties` is non-empty, the core relationship names a part, and `archive.index_of(&part)` is none (patch 116-133). It sits before the existing removed/replaced check. Test at patch 161-173.
  - Item (2): test at patch 176-190.
  - Item (3): `.filter(|prefix| !prefix.is_empty())` on the `xsi` prefix, only in the append closure for dated properties (patch 134-149). Test with a control at patch 193-218.
  - Item (4): the thirteen-property test at patch 221-285, covering replace, remove and append against expected bytes.
- **fb34ab0 (catalogue).** Every added line is `+//!`; I filtered the commit's `+`/`-` lines and none remained. It adds 60 lines at package.rs 788+ (patch 49-114). They cover each ruled statement: names, links, properties, protection, `app.xml`/`custom.xml` untouched, the legacy hash is not encryption, what is read back, `Workbook` not constructible, `sheet=false`, the unprotect rewrite, the missing core part, the added-sheet link rule, and the Excel check pending.
- **3e67745 (probe).** Only `tools/size-probe/Cargo.toml` (feature `metadata = ["emend", "xml", "emend/metadata"]` plus a `[[bin]]` with `required-features = ["metadata"]`, patch 606-629) and the new binary (patch 631+).
  - Diffed against `probe-emend-format-write-xml`, the only changes are:
    - `USAGE`, which names its own binary;
    - three new arguments;
    - the five metadata calls: `add_defined_name` workbook-level, `set_hyperlink` External, `set_property(Title)`, `protect_sheet(.., SheetProtection::default())` and `protect_workbook(.., true, false)`;
    - two read-backs, `defined_names().len()` and `properties().get(Title)`, printed as `names` and `title`.
  - That matches "Size".

**Findings**

**F1 (defect, against the letter of the rulings).**
- Item (1): P3 ruling 1 and the release both say "two tests, one a set and one a removal". The diff has one `#[test]`, `a_property_edit_on_a_missing_core_part_is_refused_at_the_write`, which loops `for value in [Some("t"), None]` (patch 161-164).
- Item (2): ruling 5 and the release say "one test each". The diff has one `#[test]`, `a_core_part_removed_or_replaced_refuses_a_property_edit`, covering both `remove_part` (patch 179) and `replace_part` (patch 185).
- The primaries agree there are four new test functions, not six:
  - `gate.log:2072` shows 556 becoming 560 with the feature on;
  - `test.log` has 764 against P3's 760;
  - `run.log:4663` reads "tests added or edited: 4".
- The substance is covered. The count and form are not what was ruled, so planning should rule on whether a two-case test counts as two tests.

**F2 (remark).** 4ece0f0 also edits three `//!` error-catalogue entries to name the new tests (patch 18-47). The release says "Nothing else in that commit". This is the catalogue's standing convention and the commit message says so ("The catalogue names the new tests"). The report's line for 4ece0f0 (lines 12-19) does not mention it.

**F3 (remark).** The plants cover only one half of each two-case test:
- N4 fails at `properties.rs:337` (`test.log`, the N4 panic line), the `remove_part` half. The `replace_part` half has no plant of its own.
- N1 fails at `properties.rs:325` on the `Some("t")` iteration. The removal case is not shown failing separately, though it shares the same code path.

**F4 (remark, behaviour nobody tested or ruled on).** The new check tests `archive.index_of(&part).is_none()`. A caller who `add_part`s the missing core part and sets a property in the same write is now refused with "no such part". That is conservative and consistent with the read, but no test and no spec sentence covers it.

**F5 (remark).** The probe passes the name text as the external hyperlink's target and as the title too; `size.log` shows `target=Sheets!$A$1 mode=External`. Stored as given, as the spec allows, and never opened in LibreOffice. It is harmless for size, but the target is not a URL.

### Question 2: do the logs contain every line the spec's P4 section and the release require?

**Present and verified**

- **HOME and ANTHROPIC_API_KEY on labelled input lines, in every log:**
  - gate.log:6-7
  - test.log:7-8
  - negative.log:5-6
  - run.log:15-16
  - size.log:11-12
  - scan-precheck.log:10-11
- **Spec hash pin 401326fe..., in every log:**
  - gate.log:23-25
  - test.log:24-26
  - negative.log:58
  - run.log:4
  - size.log:4 and 67
  - scan-precheck.log:8 and 51
- **Attempt 1 everywhere.** RERUN_NOTE is unset in every log.
- **Gate:** nine PASS lines (gate.log:2008-2016, 2019). The debug `emend-copy` hashes are at 2024 and 2026. Ends "all as expected: yes" (2174).
- **Test:** 764 passed and 2 ignored on host and i686. N1-N4 each "found once, the test passes unplanted and fails planted: yes". The legacy hashes 83AF, CBEB and DAA7 are printed.
- **F11:** the shift and keyword printouts run with `--features rowcol,metadata` and `--features metadata`.
- **Negative:**
  - 23 fixtures written (negative.log:1226).
  - Five of the eleven messages reached (1569), and the six unreachable ones each carry a reason (1561-1567).
  - M1 exits 2 with exactly `NO_SUCH_PART`'s message and no output, against two controls (1445-1450).
  - The change AB note is at 1430.
  - P0's core part matches the built bytes by sha256, with a control (1548-1552).
- **Size:**
  - 17 probes built, and the 16 compared against part 3's record.
  - Feature cost 50404 / 53248 / 41648.
  - Core growth 968440 - 968012 = 428 (file 0, .text 192), "above 2000: no".
  - Whole growth 556.
  - Budget 435224, 23528 under 458752, against part 3's 434668.
  - Release binaries hashed on `hash:` lines.
  - The record is installed at sha256 a41ebcc3... and checked byte for byte.
  - The probe's run prints `names 1` and `title 1`, checked against the expected 30 lines.
- **Run:**
  - The binaries are checked against size.log's hash lines (run.log:1580-1596).
  - The four phase directories are the ones planning named: e981810-c (3528), cced544, 372bc21-props and 372bc21-protect.
  - All eight label runs say "byte-identical ... 27 of 27".
  - Host and i686 outputs are the same for all four samples.
  - The plants turn their flags: 10/10, 12/12, 9/9 and 8/8.
  - F9: the `hash` plant changes the workbook's attribute and `hash_sheet` the worksheet's (run.log:4390-4391, BF at 151-155).
  - F10: `w_strict`, the retag plant, the stripped control and the strict-relationship control (run.log:4474-4489, BG at 156-161).
  - Keep 435 pairs: 431 unchanged, 4 new, 0 gone.
  - Open check: crate 378, tool 57, with 0 refused and 0 not run.
  - Five record pairs installed.
- **Scan pre-check:** "RESULT: PASS", hits 0, structural failures 0 (2064-2068). The G3 change AY is at 317-321 and its control line at 1962.
- **Standard error:** every script's is 0 bytes.

**Findings**

**F6 (remark, a departure planning should rule on).** P3 verification ruling 7, and the spec's P4, say P0's created core part is compared by sha256 "to the bytes the crate test pins". The crate test pins a part with different values (`<dc:title>emend</dc:title><dcterms:created ...>`, test.log's template core-part printout). So `negative.log` instead compares with bytes a Python builder makes to mirror `new_core_part`, using namespace constants read from the source (change Y, negative.log:149-157 and 1477-1552).
- That is a reasonable reading, but a different check: it is largely self-consistent with the source, and only the dropped-dcmitype control shows it can fail.
- The report states it inline (lines 41-42), right after the "Change AB" sentence, so a reader may attribute it to change AB. The log calls it change Y. It is not listed under "For planning".

**F7 (remark).** M1 reaches "no such part" through the sample's properties read before any edit (negative.log:1430). So the binary never exercises the write-time refusal that P3 ruling 1 added. The log and the report both say so, which is the honest statement. But ruling 1's "the seventh fixture case through the binary" reaches the read's refusal, not the fix; only test.log's N1 reaches the fix. Planning should confirm this satisfies ruling 1.

**F8 (remark).** The G3 control (scan-precheck.log:1075-1079) offers a two-line listing with count 2. `only_spec` refuses that on `[ "$2" = 1 ]` alone, so the control does not exercise the content comparison. A one-line listing naming `ROADMAP.md` would. `only_spec` is also not on this run's path: PLANNING_SPEC is unset, and the AQ branch at 1963 ran instead.

**F9 (remark).** The spec's P4 says "the seventeen probes classified against part 3's record"; "Size" says sixteen. The log classifies 16 and labels the 17th new (size.log:2666-2669). That follows "Size", and is the only possible reading, since part 3 has no figure for the new probe. The spec is inconsistent with itself.

**F10 (remark).** The release lists F11 among "the run script's" fixes, but F11 lives in the test script (test.log, the shift and keyword runs with `metadata`). It is present there. The run log carries F9 and F10.

Nothing required is missing from the logs. The final `scan.log` is not in check1, as expected, because the scan runs last.

### Question 3: does every number and quoted string in `report-draft.md` appear in a primary?

**Checked and found matching**

- **Hashes:** 401326fe and fa374eb5; the full 3e67745011d5... sha; the log hashes 2ca7d820, b8c36018, 1b12527e, 187a44b2 and 1952a843; the debug binaries 41aae565 and f324def5; the release binaries 7f25fb2a, 89d6f20d, 1445523d and 99d1661d; all ten record hashes; a41ebcc3.
- **Gate and test:** 9 PASS; 764 passed and 2 ignored; 83AF, CBEB and DAA7; N1-N4.
- **Negative:** twenty-three fixtures; "five of the eleven reached, as P4 counts them: yes".
- **Run:**
  - 27 of 27 and the four directory names.
  - Entry totals:
    - names: 816 identical, 45 changed, 9 dropped, 0 added;
    - link: 822, 48, 0, 6;
    - props: 843, 27, 0, 0;
    - protect: 816, 54, 0, 0.
  - Plants 10, 12, 9 and 8 plus hash_sheet.
  - r_bound_in=0 and w_strict=0/1.
  - Keep 435 / 431 / 4 / none gone; 378 and 57, with 0 and 0.
- **Size figures:** 50404, 53248, 41648; 968440 - 968012 = 428; file 0, .text 192; 556 and 967884; 128; 435224, 23528, 458752 and 434668; 973348 and 753171.
- **Scan pre-check:** "RESULT: PASS", 0 and 0.
- **Quoted "yes" lines:** each is verbatim in its log.

**Findings**

**F11 (defect, report).** Report lines 12-16 say "(1) ... two tests; (2) ... one test each". The diff has one test function for each item (F1), and the primaries show four new tests in all (`run.log:4663`, "tests added or edited: 4"; 760 to 764). The line should say one test covering a set and a removal, and one test covering a `remove_part` and a `replace_part`, or F1 should be fixed first.

**F12 (remark, report).** Report line 84 says "of which 128 is bcaa31d's own change to the probe source". The primary prints "the format probe at bcaa31d... over part 3's figure, 968012 - 967884 = 128" (size.log, the figures block) and does not attribute it to the probe source. `git show bcaa31d` does show that commit's `package.rs` change is `//!` lines only, so the attribution is a sound inference. But it is not in a primary as worded; it should be labelled as the report's own reading.

**F13 (remark, report).** Report line 38 says "with its control"; the log shows two controls for M1 (P2 and P0, negative.log:1445-1450).

**F14 (remark, record wording, not the report).** `records/size-m2p4a-2026-09-29-3e67745.md` has the ungrammatical sentence "It is not above the 2000 section bytes at which the spec has planning told." It is already committed-bound, since size.log hashed it, so it is only worth noting.

No other number or quoted string in the report is missing from or different from a primary.

### Verdict

The four commits touch what the release names. The catalogue commit is doc lines only, and the probe commit is the probe plus its Cargo wiring only. The first code commit carries items (1) to (4) and three catalogue entries naming the new tests (F2).

There is one real defect and it is one of form, not substance:
- The rulings ask for two tests for the missing core part and one test each for removed and replaced. The commit has one two-case test for each, four new tests in all rather than six (F1).
- The report repeats the ruled counts instead of what the diff holds (F11).

The logs carry every required line: labelled HOME and ANTHROPIC_API_KEY lines, the spec pin, attempt 1, F9 with both elements planted, F10 with the strict control, F11, G3, byte identity 27 of 27 against the four named phase directories on both targets, the release binaries checked against size.log's hash lines, and all figures. Every other figure in the report matches its primary.

Planning should rule on:
- F1 and F11;
- whether the rebuilt-bytes comparison satisfies ruling 7 (F6);
- whether M1's read-side refusal satisfies ruling 1's binary case (F7).

The remaining findings (F3-F5, F8-F10, F12-F14) are remarks.
