# M4 part 1 P1 independent check 1

Checked commit (the phase commit): 37ab66bf068074d223c1441966a7d3bcb0aa124d
HEAD when the check ran: 37ab66bf068074d223c1441966a7d3bcb0aa124d
Checker: a fresh general-purpose subagent (not a fork), given only the files below in <check>/in, where <check> stands for <home>/.cache/emend-build/m4p1-p1/check1.
Inputs (sha256, name):
  45981b7e27158d8d37f513ebdf79445ee04ee98d076ef875d651afffd59131c0  diff-267e689.diff
  fbe3e262f9cf0b23840b2b0f72f29f76f053b16ea405cddabf16b580250529ab  diff-37ab66b.diff
  580b577bb63ffc626b6e3ab896725e4808de7615aabd004251ae14dc90496da9  diff-6d5df98.diff
  4e4eb2c79b783e9cffbb66cfc432d2adebc6a18bdefd802f453e6df25b2be921  diff-a1f5479.diff
  3e920f6a7ce366d7cc801eea795fa4939f8812402a10a1b1c00f2b5df6d05b76  diff-combined.diff
  946194727d34c481f4938fcc31c307e8628fc22cf29a609bccfcba607d067f83  gate.log
  5aefa4a2d197a18c16c7f72dd56cdd6bb92360146f5dc31c58f48fb69d689cc2  msg-267e689.txt
  5c61c286338a33d9b745ba3b1c0fa1278b9e13e50c3550ab8ca2302de9238831  msg-37ab66b.txt
  7aa108225c7c1a5300ca54330926297e5537eba147ce36fb69a58d53f03f11bf  msg-6d5df98.txt
  cd8729ad650859b340c6137a9bc149a03a92ec5068f211b5bfe85cd02277e6bf  msg-a1f5479.txt
  d11c7e24a0116030e1d906a7071e6358c311275921398043b6250182c7bd0d37  negative.log
  488e7472f32402ca458bce579a763082b5a663cd808a5d7abba6d19de7010151  report-draft.md
  99824926bdf8c8660c5de6f6ac3c8ca048c2d6fe2f49ccd53b9c45ac2144c227  run.log
  27878fcecbc9364b7bc83c2b85da0e9cb5ab9daa588a04d72cc5b110c332f3c2  scan-pre-check.log
  88a72d93a4dfd064f11581354bf877c24f85ff91d9c7d09741fc6f147ec2de73  spec-0001.md
  7bc47caecab30ad1de299cea49f967aea018f10c3d0599b8fbd4d77a191332f3  spec-0030-committed.md
  ff147a83f00ab2a908fdc6ec27da4efc80a547c656873d11b7f326818564e456  spec-0030-working.md
  4a35ee2a850d2d31430ad54a9a41f9c4b51611247a1b885dfa5b1b3da73c3158  test.log
The answer below is the checker's final text, extracted from its transcript and written verbatim: 7596 bytes, sha256 b43cd76596c5a29fc6ea3153eea44f4f5f8c4a32bc76ec3efef670f48e72ed3c; it holds no non-ASCII character and no machine path, so nothing was replaced.

---

I read only the files in <check>/in. The harness also put CLAUDE.md, the project memory index and git status into my prompt. I did not use any of that as evidence.

**Q1. Does the diff do what the spec and the P1 rulings say, and nothing else?**

Yes, with one point for planning to confirm and one I could not verify.

- **Files in the phase.** The file headers in diff-combined.diff (lines 1, 70, 158, 197, 228, 334) list six files. Each index hash matches the per-commit diffs. Only a1f5479 touches ROADMAP.md, and its index is the same c17d657..c01f374 in diff-a1f5479.diff:2 and diff-combined.diff:2. So "Unscoped" is not changed after the doc commit, as decision 2 A needs. I confirm "nothing else" for the phase as a whole from diff-combined.diff.
- **a1f5479 (doc commit).** spec-0030-committed.md matches the added text of diff-a1f5479.diff:181-393 exactly. Its sha256 is 7bc47cae...91332f3, which equals msg-a1f5479.txt:6, and its git blob is e9ab926, which equals diff-a1f5479.diff:178. I cannot check the ROADMAP.md and specs/0029 hashes in msg-a1f5479.txt:4-5, because I have no copy of either file. gate.log:3800 says the gate checked them.
- **6d5df98 (specs/0001), diff-6d5df98.diff.** The rm -rf bullet's list of standing forms drops passthrough-run.sh lines 121 and 205 (lines 9-16 out, 17-28 in). It gains the dated sentence (2026-10-09, decision 1 A, rulings 2 and 3), the exact refusal message, and "The rule has no exception." Nothing outside the bullet changes. This is rulings 1 to 3. "Nothing else" confirmed from this diff and gate.log:3990-4000.
- **267e689 (runner), diff-267e689.diff.**
  - The trap becomes `"${WORK:?}"` (line 41).
  - The refusal is a `find -mindepth 2 -maxdepth 2` for `lo.$LABEL` or `lo-in.$LABEL`. It looks at every subdirectory, selected or not, and covers only this label. It dies with the exact ruled message and sits after the summary guard and before the loop (lines 49-57).
  - The loop's removal becomes `"${dir:?}/..."` (line 67).

  This matches rulings 1, 2 and 3. "Nothing else" confirmed from this diff. The gate shows the guard at line 173, the refusal at 183 and the loop at 207 (gate.log:4011-4012), and no unruled rm -rf left (gate.log:4015-4020).
- **37ab66b (list script), diff-37ab66b.diff.**
  - Error rows are skipped (lines 112-113), and a directory with every row an error is refused (lines 102-104).
  - The note is printed only when the count is above 0 (lines 153-155), which is ruling 6.
  - The docstring and the return shape change. Any other non-ok outcome is still refused (line 114).

  "Nothing else" confirmed from this diff.
- **For planning to confirm (not a defect against the text):** the note for the `--run` directory names the corpus label "host", not the directory (diff-37ab66b.diff:135; test.log:1912). Item 1 says "<label>", so this fits the text. But for a `--run` directory the note does not say which directory it means.

**Q2. Do the logs contain every line the spec and rulings require for P1?**

Yes.

- **test.log**
  - T1, the fixture with no planted row, has an empty stderr (lines 1876-1887), as ruling 6 says.
  - T2 has a planted error row in `--run-plain`. It shows 3 corpus rows against T1's 4 and the note alone on stderr (lines 1892-1901).
  - T2b plants the row in `--run` (lines 1906-1916).
  - T3a to T3e cover the runner (lines 1918-1955):
    - T3b and T3c refuse, for lo.host and lo-in.host.
    - T3d is not refused (only the other label, lo.i686, is planted).
    - T3e refuses (lo.host under a subdirectory --ids does not select).
  - The log ends "test log checks all as expected: yes".
- **negative.log**
  - N1 and N1b refuse a directory of error rows only, with "error: a run directory holds no ok row" (lines 3116-3131).
  - N2 leaves out a planted row on a copied directory and counts it, with control N2c (lines 3132-3150).
  - N3, L1 and L2 cover the other refusals (lines 3151-3174).
  - N4 and N4c cover the runner refusal, the summary is unchanged, and N4c is the control (lines 3175-3197).
  - N5 is ruling 3's :? stop: "dir: parameter null or not set" and "WORK: parameter null or not set" (lines 3198-3228).
  - **Deviation from ruling 3's wording:** the ruling says "plants the old form on a scratch copy with the variable unset and shows the `:?` stop". The log shows the stop on the new forms (N5a, N5b) and shows the old forms widening only as printf plants of the extracted line, not as a scratch copy of the runner (N5c and N5d, lines 3214-3226). report-draft.md:53-56 discloses this. Planning should accept or reject that reading.
- **run.log**
  - It uses the formula directory as `--run`, printarea as `--run-plain`, `<keep>` = `<home>/.cache/emend-build/m3p7-p5/keep-host` (line 52), and the committed record read through git show (line 1881).
  - Shares are 27 and 21 (lines 1871-1872). The six error ids equal item 1's (lines 1875-1877).
  - Totals: 2210 rows, fixture 1081, input 1081 (line 1902).
  - Stderr is exactly the one ruled note (lines 1891-1892).
  - No row is printed, and the list's removal is logged with "the list is gone: yes" (lines 1914-1916).

  All of ruling 4 is present.
- **gate.log**
  - The environment line is the first section (lines 11-25), as spec-0001.md:253-261 requires.
  - The ruling 5 prune comes before any build: 8 names, du -sb, 4270182350 bytes, `rm -r` without -f, keep-host kept (lines 39-97).
  - m3p7-p1 to p4 and m3p7-close each assert they hold none of the prune-set names (lines 117-206).
  - Nine PASS lines (lines 4455-4463). The log ends "yes".
- **Scan.** scan-pre-check.log ended at 02:00:56, after the other four logs (the last, run.log, ended 02:00:22), with RESULT: PASS. The final scan is not among my inputs.

**Q3. Does every number and quoted string in report-draft.md appear in a primary?**

Yes, except the dry-run section.

- **Supported:**
  - All four log hashes. I recomputed them over the input files and they match report lines 32-35.
  - The commit subjects and the base subject (msg-*.txt:2; gate.log:231).
  - "eight names" and 4270182350 (gate.log:41, 84).
  - "nine PASS lines" (gate.log:4455-4463).
  - The case names (test.log and negative.log section headers).
  - The run figures and the note string (run.log:1871-1902).
  - "RESULT: PASS" and "structural checks failed: 0" (end of scan-pre-check.log).
  - "Unscoped" unchanged (diff-combined.diff:2).
  - The N5 line-number-and-sha256 claim (negative.log:3204-3205).
- **Not in any primary:** report-draft.md:40-49 ("The gate's dry run passed first time", the first dry runs ending "no", and "Second dry runs passed"). The dry-run logs are not among the inputs. Two lines only partly support the descriptions of defects (a) and (b): run.log:1910 (6 host rows naming the error ids) and test.log:1863 (the grep reads the format string, not the docstring).
- **Paraphrase:** "at the sha256 the body names by command substitution" (line 12) loosely restates spec-0030 line 134, "command-substituted hashes". It is not a string from a log.

**Disagreements**

1. Ruling 3's negative case is met by showing the stop on the new forms and the widening of the old forms as printf plants, not literally "the old form on a scratch copy". It is disclosed, but planning has to rule on it.
2. report-draft.md:40-49 makes claims about the dry runs that none of my inputs can verify.
3. The `--run` note is labelled "host", not the directory name (minor; matches the spec's "<label>").
4. Not verified: the ROADMAP.md and specs/0029 hashes in msg-a1f5479.txt (no copies of those files among the inputs), and the final scan (not run yet).
