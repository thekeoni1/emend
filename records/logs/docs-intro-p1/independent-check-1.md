# Docs intro P1: independent check 1

The answer of a fresh subagent given only these materials, under
<check1> (<phase>/check1), and the three questions of CLAUDE.md
("Independent check before reporting"). Checked: the doc commit 40ee6b1,
the code commits 4dfd4f4 and 2f7891a; HEAD at the check 2f7891a, with
the record files not yet committed. Inputs, sha256:

- README-repo.md b961cc72117fe3e1d855d1c5278bdf29d208b92bfcbc12ab128d365d3d98fc52
- commit-2f7891a.diff fac3c93ae3cead9ce0b9ddd6cfa242138a78537ce55819bcd535628562191cc6
- commit-40ee6b1.diff 2a784eb74d27b83f1b7352f3f61ed398daa20f07f4923e4ca5dbc3827af36215
- commit-4dfd4f4.diff 21399b7f74840e2d79357d0017518494215cf98089dcd19d7d3e0f1ec33aad5b
- compare-m6p6-2026-10-24-d88f735.md 56673747ab8611f3a8c95dd3ad1e14e80b4f0e6a1100a311ccb09eb22bd1c0f4
- decision-architecture.md 90822d91efc61fae8aad38402b2cd051023b71d104e162adfb389403332c77d1
- lib.rs 8c68120fe003f3cd8f048a34de21902b6cfa6efd25d49e09940192bc32e3f46b
- logs-README.md 3b9e45cfd380ae5c00aaecead2b9262923334058ea24febf375fe1ee0836aab5
- planning-messages.txt d1bcc18140de03df3ccdbc98fb835378ff81052026ffba7f42f2f1df76a8b153
- report-draft.md 5140dd0e2648996e8ad48f9954f61f8079dc86bc07dacd932ca8b8c7e584c8b4
- spec-0050.md a7d109d992dcf679fc2a8ad39cd9a222cd27caae0753f7a0b30779e97033f66d
- verify.log fa264d9464e7c369fcb3aa98f759ae9ec2cd8ec34be1cbb656dd4d5258eedc64

The answer was extracted from the subagent's transcript by script into
<phase>/check1-answer.txt (sha256 7a62a7203917a2197890dbe97690116f9e9bd058f0b44ff64b39c609fba54060). It appears
below verbatim. The logs README's section "After the independent check"
answers its points.

root: <check1> stands for <phase>/check1
root: <phase> stands for <home>/.cache/emend-build/docs-intro-p1, the phase's directory
root: <home> stands for the home directory of the account the runs ran under

---

I have a CLAUDE.md, a memory index and a git status in my context from the harness. I did not use any of them as evidence.

**1. Do the diffs do what the spec (as amended by rulings A to E) says, and nothing else?**

Yes with exceptions. Both exceptions are allowed by the rulings; neither is a defect.

- **Doc commit (commit-40ee6b1.diff).** The message has the three "<path> sha256 <hash>" lines and nothing else (lines 7-9), and the hashes are the brief's (planning-messages.txt lines 4-6). It changes only ROADMAP.md, specs/0049 and specs/0050 (lines 13, 48, 92). verify.log lines 566-575 confirm this.
- **README (commit-4dfd4f4.diff lines 24-92; README-repo.md lines 1-57).**
  - The head is the spec's block verbatim (spec-0050.md lines 33-77), with EXAMPLE replaced by 15 lines. verify.log lines 594-599 confirm this.
  - The example uses only the core, has no `unwrap` and no file name. It opens from a reader, sets one cell on the first sheet and writes to a writer.
  - The tail from "## Dependencies and features" is unchanged (diff line 94; verify.log lines 602-606).
- **lib.rs (commit-4dfd4f4.diff lines 100-161, commit-2f7891a.diff lines 185-191; lib.rs lines 3-10 and 40-71).**
  - Paragraph 1 is the README's first paragraph, rewrapped.
  - Paragraph 2 is the first sentence of the openpyxl paragraph only (ruling C).
  - The example is a running doc test under "# Example" (ruling B), with hidden set-up lines that use `Package::new()` (ruling A).
  - No other line changes (verify.log lines 608-621).
- **Ruling C's commit.** It was made as a second commit on top of 4dfd4f4, not as an amend, and it changes lib.rs alone (commit-2f7891a.diff line 181; verify.log line 565).

Exceptions:
1. Two visible example lines are wider than the file's 78 columns (lib.rs lines 46 and 52). The driver checks width only on the new paragraphs (verify.log lines 619-620). Ruling B accepts this.
2. The hidden lines go beyond the spec's "set-up lines": they read the edited cell back and assert it (lib.rs lines 64-68). The spec allows hidden lines, and the 4dfd4f4 message says what they do (commit-4dfd4f4.diff line 14).

Note, not a diff exception: the umya bullet says umya-spreadsheet reads "a model of the whole file". The record it cites says that with `lazy_read`, unparsed worksheets are written back from their raw bytes (decision-architecture.md lines 30-33). The wording is the spec's own (spec-0050.md lines 56-59), so it is planning's to judge.

**2. Does verify.log contain every required line, and does each required check have a control that could fail?**

Yes with exceptions. Every required line is present:
- **Rule 1:** lines 662-699.
- **Rule 3:** lines 701-706.
- **Rule 2:** versions at lines 713 and 726, sha256 at 715 and 728, descriptions at 719 and 732.
- **Acceptance 1:** lines 554-567, with the stat at 560-562.
- **README block equal to the doc test's visible lines, with the count of 15:** lines 594-596.
- **Acceptance 3, the tail comparison:** lines 601-606.
- **Acceptance 4:** lines 655-658 and 739-749.
- **Acceptance 2:** the gate at 752-765, the doc test on both targets at 766-778, planted defects at 779-793, and the budget at 803-804. The FINDING branch exists in the script (lines 514-518), and the summary shows 0 FINDING lines (line 807).

Exceptions:
1. **The calamine description check cannot fail on that description.**
   - The check (script lines 407 and 410) tests for the substring "read".
   - "Spreadsheet" contains "read", and calamine's description holds "Spreadsheet" (line 719). So the check would pass even on a description that said "Spreadsheet writer".
   - The only control (lines 417 and 736) tests "writ", not "read".
   - The verdict still holds, because the printed description says "reader" (line 719).
2. **No control exercises the budget's FINDING branch.** No mismatching budget is planted. The budget also takes probe-zip-xml 533216 from a committed size record rather than building it in this run (lines 800-803).
3. **The privacy scan has no control** (lines 746-749).
4. **These checks have no control of their own:**
   - "Out of scope by design lies in the tail" (line 606).
   - The base-README figure checks (lines 693-699).
   - The HTTP 200 and sha256-equals-cksum checks (lines 712-730). These lean on the generic `same` control at line 552.
   - "at most 15" lines, "no unwrap" and "opens/sets/writes" (lines 589-591).
5. **The check at line 592 claims more than it tests.** Its label says the example "names no feature-gated item", but it only tests for a quoted file name with a dot (script lines 177-178). The core-only claim actually rests on the doc test passing with no features (lines 766-777).

**3. Does every number and quoted string in report-draft.md and logs-README.md appear in a primary?**

Yes with exceptions. These do appear: every line citation, commit hash, file sha256, figure, version and description in report-draft.md lines 4-6, 11 and 14-21, and in logs-README.md lines 38-106. The verify.log sha256 fa264d94...dc64 does not appear in any primary, but I computed the hash of the verify.log in <check1> and it matches. I found no non-ASCII character in either file.

These do not appear in any primary:
1. In both files, the logged run's "exit 0" and "0 bytes stderr" for the script as a whole (report-draft.md line 11; logs-README.md line 65). verify.log prints only the python run's and the gate's stderr (lines 659 and 762). "0 lines holding the home directory" is true of verify.log (I counted 0 matches for "/home") but is not printed in any primary.
2. The d4 results: "yes", 0 FINDING, "0 lines ending ': no'" (report-draft.md line 9; logs-README.md lines 133-134), and d4's sha256 11eb0ee6...a73c (logs-README.md line 134). No d4 log is in <check1>. d3's "yes" and 0 FINDING are in planning-messages.txt line 22.
3. The sha256 of logged.sh (1c46e632...baf6), dry.sh (99b1517e...be1be) and apply.py (96e13645...f524), and of the superseded d3 script (0c9fee94...c89f) (logs-README.md lines 113-115 and 133).
4. What d1 and d2 found: the difflib opcode split, the anchored-pattern miss, and the stale line (logs-README.md lines 122-129). The string "PASS native (allowlisted -sys: none)" does appear (verify.log line 758), as does 'Error: "no cell"' (line 782).
5. "nothing referred to c58ebec" (logs-README.md line 46). Only the amend itself is in a primary (planning-messages.txt line 28).
6. The record commit's file list {verify.log, README.md, independent-check-1.md} (report-draft.md line 7), the range e9dcaac..HEAD scan (line 25), and "quoted verbatim" (line 23). These describe steps after this evidence and are not in any primary.
7. "verify.log lines 566-574" for the three sha256 (logs-README.md line 41). The hash lines are 569-574; lines 566-568 are the file-list check, its control and a command line. The range is accurate but loose.
