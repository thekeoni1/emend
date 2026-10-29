# M7 part 2 P3, independent check 1 (its answer verbatim, but the home directory's path written <home>, once; the answer as given is sha256 34acdcf43da61d351feda5c2417fb879cc2898603df7074a39a8272744a8c881 in <phase>/check1-answer.md; its brief, equal to the prompt the checker was given, is <phase>/check1-brief.txt, sha256 ceca55d7d48d2af5e991a6e44f7c538d8fe867376578ed4c55945160c20e8d20)

I answer yes to all three questions, with exceptions on questions 2 and 3. I found no disagreement on the diff, the record figures or any log verdict. The exceptions are small: two imprecise counts in the draft report, one exit-code column with no stated source, and three items no log tests or states.

## 1. Does the diff do what the spec and rulings say P3's four commits hold, and nothing else?

**Verdict: yes.**

- **Files touched.** `git diff 16c083f f7cabba` touches six files. The gate confirms that list (gate.log 9140-9160).
  - 612986a (doc): the spec and ROADMAP. Their hashes match the message: 8a515384 and 884c6e33, recomputed by `git show | sha256sum`. ROADMAP marks P2 DONE and P3 IN PROGRESS, as ruling AD says.
  - 790c488 (code): `crates/emend/Cargo.toml` only.
  - b016818 (docs): CHANGELOG.md and README.md only.
  - f7cabba (docs 2): CHANGELOG.md, README.md and lib.rs.
- **Manifest (790c488).** `publish = false` is gone. The seven-entry `include` list is `src/**`, `template/**`, `template/_rels/.rels`, `Cargo.toml`, `LICENSE-APACHE`, `LICENSE-MIT`, `README.md`. The comment cites specs/0045 and specs/0031 P2 ruling 1.
- **Fixed entries.** CHANGELOG has five, in P1's order, fix 1 to fix 5. Each cites `specs/0045-publish.md` with its fix number and `records/excel-check-m7-2026-10-26.md` (run.log 2546-2551).
  - I checked three claims in the text against the code:
    - fix 2's "`workbookProtection`, `definedNames` or `calcPr`" (package.rs 14202-14211, 14164-14174);
    - fix 3's message (chart_write.rs 88) and the `add_chartsheet` mention (chart_write.rs 918);
    - fix 5's message (chart_write.rs 93).
  - Fix 5 leaves out Polynomial, which is fine. The spec says "if it is not already", and the 2 to 6 check already existed (chart_write.rs 1595).
- **Excel sentences.** Both are in CHANGELOG "Measured" and README "What has been measured", with ruling AG's wording ("fixed five of the causes and documented the rest as limits", "every sample's outputs on the private test corpus", "six samples' outputs over the 27 workbooks"). The three repairs are named. "was asked to open" appears in all four places (f7cabba).
  - The figures equal the two records' Totals rows. Part 1's record (47e6f360) gives 6018/1528/32/1864/365/1874/355. Part 2's record (6ca1f6e8) gives 4640/159/3/1878/361/1881/358.
- **Heading.** It reads "## 0.1.0 (unreleased)" at both 16c083f and f7cabba (ruling AB).
- **Doc comment.** lib.rs gains exactly one line after `copy`: "- `charts`: charts read, added, formatted and removed, and chartsheets." (ruling AG). Its paragraphs match the README's (run.log 2593-2608).
- **Limits.** No limit text was added (ruling AC). Both limits already stand in README "Out of scope by design" and CHANGELOG "Not supported by design"; I grepped each section.
- **ASCII.** Nothing outside ASCII in the four files.
- **Observation, not a disagreement:**
  - The `include` entry `"README.md"` matches no file under `crates/emend/`. The README is packaged through the `readme` key, which cargo resolves from `../../README.md`. The entry does no harm, but no case tests it.
  - b016818's message says "the crate-level doc comment is unchanged". f7cabba then changes it, as ruling AG directs. Commit order explains this.

## 2. Do the logs contain every line the spec and rulings require?

**Verdict: yes with exceptions.**

Present:
- **Ruling AF (negative.log 5361-5401).**
  - (a) The doc commit's manifest lists 71 files without the `.rels` file. `cargo package --offline` exits 101 with "couldn't read `src/../template/_rels/.rels`".
  - (b) The list without the by-name `.rels` line gives 72 files, with the file listed once.
  - (c) The code commit's manifest gives 72 files, the file listed, exit 0.
  - Each extraction prints "not a git repository".
- **Archive (run.log).** It is outside any checkout, with a control. 2639 files are extracted. `--allow-dirty` is passed in the checkout (1 status line) and not in the archive.
- **Package lists.** 73 entries in the checkout and 72 in the archive. They are equal once `.cargo_vcs_info.json` is removed, and `template/_rels/.rels` appears once in each.
- **Publish dry run.** Exit 0, "Packaged 72 files", one "aborting upload due to dry run" warning, 0 errors. The warning count has a control.
- **Tokens and credentials.** Both token variables are absent, with two controls. The credentials file line reads "test -e ...: no" (run.log 2345-2350).
- **The four sentences (run.log 2470-2505).** Each is printed beside the record's Totals row, read by `totals_of`, with "figures missing 0" and "in order: yes".
  - Controls: a planted figure (+1) and a swap (1528/32 and 159/3). The negative log repeats both on the README.
- **Heading form (run.log 2539-2544).** Three controls: a dated heading accepted, "0.2.0" refused, two words refused.
- **Limits.** Both read in both sections, with a control.
- **Features.** 12 of 12 named, side by side, with a control lacking `charts`.
- **Gate.**
  - "nothing is removed: the prune set is empty" (gate.log 235); P1's keep unchanged at the end (11066-11071).
  - Nine PASS lines from check.sh.
  - Messages: ASCII with the T95 control.
- **FINDING lines.** 0 in each log. The only occurrences are in the run log's printed script and its summary.

Exceptions:
- **E1.** The spec asks that the gate tell the hygiene step "if it reads `publish`". No log line states whether it does. I grepped `scripts/check.sh` myself and found no `publish`, and the gate shows "PASS hygiene", so the gap does not matter.
- **E2.** Every control on the figure check and the swap check runs on the README sentences only. The CHANGELOG sentences go through the same function, so this is a narrow gap, not a hole.
- **E3.** No log prints an exit code for the gate, negative or run as a whole. Only the driver's `logged.out` does (see Q3).

## 3. Does every number and quoted string in the draft report and the README appear in a primary?

**Verdict: yes with exceptions.**

Matches:
- The three log hashes (sha256sum), the scan pre-check hash fc109aac, and "total hits 0", "structural checks failed 0" and "RESULT: PASS".
- PLANNING_SPEC 64044f95 and "1294 lines" (wc). The gate's phase-line time 14:28:09Z.
- Every package, warning and figure number, the heading, "12", and "1529".
- The session-statement hashes. I checked all of them with sha256sum and all match. Each of the three logs is cmp-equal to its copy in `<phase>/logged`.
- The driver times in `logged.out` (14:28:08Z to 14:35:32Z).

Disagreements:
- **D1.** The report says "In run.log it occurs 16 times: 14 in its printed script...". Those are 16 *lines*. The word itself occurs 18 times, because lines 2106 and 2612 each hold it twice (`grep -o FINDING run.log | wc -l` = 18). Ruling AH's d3 figure of "13 times" appears to use the same line count.
- **D2.** The report says "each scan pre-check FAIL on its two dry-layout lines alone, hits 0" for d1 to d4. d1 ran no scan: the driver stopped after the gate, and `dry-d1.out` shows the gate's exit 1 only. The claim holds for d2 to d4.
- **D3 (unmarked).** The report's "Exit 0" column for each log has no stated source. The note under the table names `logged.out` for the times and the 0 bytes of standard error only, not the exits. The exits are in `logged.out`, not in the logs.
- **D4 (wording).** The report and b016818's subject say the README "carries the five fixes". The README carries only the part-1 sentence's "fixed five of the causes". The fixes themselves are only in the CHANGELOG.
- **D5 (minor omission).** The README lists `<superseded>` contents but leaves out `gate-gen1` and `run-gen1`, which exist there.

Statements are otherwise marked as the session's where no log prints them.

## Beyond reading

I ran only read-only commands: `cat`/`sed`/`grep`/`awk`/`head`/`tail`/`wc`/`ls`, `git show`/`diff`/`log`, `sha256sum` and `cmp`. That covered the repo, the phase directory's scripts, generators, drivers and dry-run outputs, and `logged.out`. I modified nothing. I ran no cargo or script. I read nothing under `<home>/emend-testing` or any credentials file.
