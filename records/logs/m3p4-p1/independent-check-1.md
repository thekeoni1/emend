# M3 part 4 P1, independent check 1

The answer of a fresh subagent (not a fork) given only <phase>/check1 (the committed spec specs/0026, planning's working specs/0026 with "Planning rulings during P1" 1 to 6, the committed specs/0025, specs/0001, the phase diff, the doc commit's and each code commit's diffs, the commit messages, gate.log, test.log, negative.log, run.log, the scan's pre-check, the stderr files and the runner's summary, the phase's records, and the draft report), recorded verbatim below. <phase> stands for <home>/.cache/emend-build/m3p4-p1. Checked commit (the phase commit): 93a92d588515fe5b35188803678872029c9b7bab; HEAD when the check ran: 93a92d588515fe5b35188803678872029c9b7bab. The answer was extracted from the subagent's final message and hashes to 27295233146e1fb75d00eadf3f2e32b91fb1f8a4cb50a6b50a4e5ca8e1ee332b; it held one non-ASCII character, a U+2013 (an en dash in a line range), recorded here as an ASCII hyphen so the committed file is ASCII (the final scan's first attempt, kept with its sha256 outside the tree, failed on its three bytes); no other character is changed, and it held no U+2014. Its items on the report are corrected in the report; its observations on the code and the logs are reported to planning. No second check was run.

Inputs and their sha256:

- 7c840ceeae8ddf702d60fa7c3732926094b07fcb90e4a30581fa0318903989d6 code-commit-1-crate.diff
- af143d6e1cce7100d37f94567c0edcf8d1a96082dc2767624bcffd28148626a1 code-commit-2-tool.diff
- 87a2b84c9b264ce546e644e00fddebb0b3e4182db04dd0034951f3428ef55532 code-commit-3-rows-present.diff
- 3fdceae12a8734cff67ea3fd449667e7cd79f89106b1c9acc213b788f0a9e265 commit-messages.txt
- d8e9f89baac239e4c24cc5ab869a354415c56fd4110d74e8d8a341f530baf1c0 doc-commit.diff
- c21d77b98c25350bfec430680bdd6e59146a62e3b9b013c95d5bc0b7ad789dcc gate.log
- 55712c3414c219d647442494068a41aece0d43e07e8f28ff13957273e6451638 logged.out
- c6511c96bb4b1e8828abc16eb2d82666cbc0d5bbad836470b080c28e980be1b8 negative.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 negative.stderr
- 7223555149a1f72296a2e474efbc66e1f70a2addac3afe21b34679dc16c02cde phase.diff
- d76c076cdaaff920af97bd389ee3ad1699eca55b79d9f32d21f4b55e03e63367 records/m3p4-open-2026-10-04-93a92d5.md
- 5aa106c25e96eaca35bd56036e7cccaa038008f6aca5a4a173209e7a0398c8a9 records/m3p4-open-2026-10-04-93a92d5.txt
- e0ac095703142eaebc593370d0b4afc369ed09609ad906c4b3835e0638e35b43 records/m3p4-p1-2026-10-04-93a92d5.md
- f2767444cf652ddeeef98f87445c8e5b4dc6e76086fe9b29632d28d05891dc50 records/m3p4-p1-2026-10-04-93a92d5.txt
- 12680e1b532779618061ba4eca14cd7e7e19a2b53bf961e517cc2e645769ff48 report-draft.md
- b1810fa3f6cb415b95a902779e8438b1540d8cd4bb09fae080cf6e7ec2d57c03 run.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 run.stderr
- 195eb2315e2f541a2e4d58c97d357243eddc24741b23599abae10d01224fea4e scan-pre-check.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 scan-pre-check.stderr
- 79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c spec-0001.md
- ef2fe2221752fd654240a4ebe3f65f07e2b7d1fa37d247493f2dbf7873f713be spec-0025-committed.md
- 36fa82a55fe222509d62107c9601392c64df7a0c60cc4c43d04b36b131b28199 spec-0026-committed.md
- c62113dcec0823e289c96d2e1742b807054fea193b84e5899af6bb48c547e31b spec-0026-working.md
- 49793bacd74a546be59b5246c3562f37f1151262338b08c3310909b4ffaec0da test.log

## The answer

I checked all 24 inputs against `inputs.sha256` and every one printed OK. Two further checks: the `specs/0026` I rebuilt from `doc-commit.diff` hashes to 36fa82a5..., the same as `spec-0026-committed.md`, and `spec-0026-working.md` hashes to c62113dc..., the same as the run log's "planning's uncommitted spec" line. I found no corpus content in the inputs. The logs and records print only workbook ids, producer and tag words, counts and schema names.

## 1. Does the diff do what the spec (with the P1 rulings) says for P1, and nothing else? Verdict: partly

**What the spec asks for and the diff does:**
- **The five calls.** All five are added, each under `#[cfg(feature = "rowcol")]` (`code-commit-1-crate.diff`). The gate confirms the public list grows by exactly these five. Nothing public is lost.
- **The call checks.** The range and level checks are in `Parts::group`: `first == 0 || first > last || last > limit` gives the index message, and `level > 7` gives `LEVEL_OUT_OF_RANGE`. Nothing is recorded on failure.
- **The settings pass.** It is now gated `cfg(any(sheetprops, rowcol))`, with one definition. `<sheetFormatPr>` is added to `SETTINGS`. The maximums are raised and never lowered: `maximums()` writes a level only `if level > have`, and a created element gets `defaultRowHeight="15"` first (rulings 1, 4 and 5). `<outlinePr>` is merged or created through `merged_child`.
- **The row attributes.** `line_formatted` appends `outlineLevel`, then `collapsed`, after the size and hidden flag (ruling 5). A group with `Hidden(false)` removes `hidden` (ruling 3).
- **The coalescing.** `ColsOut` merges only split pieces and created elements. A kept element ends any merge. Coalescing is switched on only when a column `Level` or `Collapsed` field is recorded for the part (ruling 2).
- **The message** is catalogued in the `//!` header.
- **The tests.** `grouping.rs` has 22 tests and they cover every item in the spec's P1 test list. That includes level 8, `first > last`, a row past the grid, two `<sheetPr>`, a shift in the same write, a removed part, and a group kept through `insert_rows`.
- **The tool.** `--group-sample` is added. Ruling 6's `rows_present` and its assertion are in commit 93a92d5.

**Things beyond the spec, or that differ from it:**
1. **Code built without `rowcol` changed.** `cols_content` now writes through `ColsOut` in every build, and `Recorded` gains a `no_outline_edits()` that always returns true without the feature. Without `rowcol` the byte behaviour is equivalent: `kept(start, true)` and `kept(rest, true)` set `any` as the old code did. The run log backs this up: "pairs of M3 part 3 P4's record whose bytes differ in the keep: 0". The report discloses the change, but the spec expected the core to grow by what the feature gates add, "near 0". That will only be measured at P3.
2. **Builds with `sheetprops` alone changed.** `SETTINGS` now has 7 entries including `SheetFormatPr`, and `met` is sized by `SETTINGS.len()`. The `fitToPage` branch was refactored into `merged_child`; that one is byte-for-byte the same logic. The risk is in `observe()`, which is not in the diff. I cannot tell from the inputs whether it now treats a second `<sheetFormatPr>` differently in a `sheetprops`-only write. Two things limit the risk: `touches(SheetFormatPr)` is false without `rowcol`, and the gate runs clippy and doc with `sheetprops` alone. But the crate's tests are never run with `sheetprops` alone. The gate runs them off, with `rowcol`, and with `sheetprops,rowcol`.
3. **An extra assertion in the tool.** Commit 93a92d5 asserts `after.in_window != 3` in addition to ruling 6's `rows_after = rows_before + 3 - rows_present`. It is not in the ruling, and the tool's own "Asserted:" doc comment does not mention it. It is harmless, but it is extra.
4. **`kept_cols` does not match ruling 3's wording.** Ruling 3 says `kept_cols` compares every attribute but `outlineLevel`, `hidden` and `collapsed`, "with `min` and `max` as the split rule gives them". The tool's `GROUP_SET` drops `min` and `max` from the comparison entirely. The spans are checked only indirectly, through `leveled == expected`. The run log's `cols_group` and `cols_bytes` checks do cover them.
5. **A latent mismatch with the spec's `level_cols` rule.** `ColItem::Other` (whitespace or a comment between `<col>` elements) calls `kept()`, which ends any merge. Suppose a `<cols>` is indented and has two equal-attribute elements for columns 2 and 3. The crate writes two elements, but the spec ("1 when the input covered 2 and 3 with equal attributes") and the tool's `grouped_col_attributes` expect one, so the sample would exit 2. The corpus never hit this (27 ok) and no test covers it.
6. **A small test gap.** Ruling 2 says `set_column_width` and `set_column_hidden` alone write today's bytes. The test `widths_alone_write_today_s_split_without_the_coalescing` covers widths only. Hidden-alone is covered only indirectly, by the unchanged 776 keep pairs.

## 2. Do the logs contain every line the spec requires for P1? Verdict: yes, with minor notes

**Gate** (`gate.log`):
- Off, `rowcol` and pair runs on both targets, each with its expected outcome: "host off 513, host on 568, host pair 596, i686 off 513, i686 on 568, i686 pair 596".
- The public list at the base and the phase commit: "adds to the base's exactly the five calls (change E28): as-expected".
- A consumer crate checked with the features off, `sheetprops` alone, `rowcol` alone and both.
- Note: the spec says "the `pub fn` lines listed off and on". The log lists them with their cfg attribute, plus the consumer check, rather than once per feature build. This is the same form as earlier parts.

**Test** (`test.log`):
- Workspace "passed 1002" on both targets.
- The crate runs off, with `rowcol`, and with the pair, on both targets. "every grouping test ran and passed" on each.
- Seven plants: "each plant failed at a line of its own ... yes".

**Negative** (`negative.log`):
- N1 has last row 1048574 and is refused with exactly the index message. N1C is its control, with exit 0.
- N2, N3 and N4 cover two `<sheetFormatPr>` (ruling 1), one out of order, and two `<sheetPr>`, each with a control.
- `LEVEL_OUT_OF_RANGE` is stated unreachable, with its covering test.

**Run** (`run.log`):
- Both labels: "files 27 ok 27".
- lo and lo_in ok 27; `order_preserved` yes 27; `files_identical` no 27.
- "identical 843, recompressed 0, changed 27, dropped 0, added 0, flagged 17".
- calcChain: "9 of 27 ... identical and unflagged".
- The comparison's 18 flags, each with a plant on both the created and the kept case: "18 of 18".
- Ruling 6: "rows numbered M + 1 to M + 3 already present ... 5, in 2 workbooks", plus "the rows edited in place are the survey's present rows: yes".
- Ruling 3: "hidden="1" ... 0, in 0 workbooks".
- Devices: TMPDIR, the open check's TMPDIR and the corpus are all 2096, against /proc's 61 as a control.
- The comparison is named by record commit 6d576894 ("M3 part 3 P4's pairs whose lo_in, lo or either class changed: 0").
- The record pairs are installed.

**Requirements with no matching line:**
- `scan.log`: only the pre-check exists. That is expected, since the final scan runs after this check.
- Ruling 3's "the run log reports the count" of hidden corpus columns: the count is there, but the survey (`gpsurvey.py`) treats only the value `"1"` as hidden. A column with `hidden="true"`, which the schema also treats as hidden, would be counted only in `hidden_any`. That case is not split out.

## 3. Does every number and quoted string in `report-draft.md` appear in a primary? Verdict: partly

**Matched:**
- **Commits and base:** the four commits and their roles; base 0caa390 and its subject (gate.log line 782); the doc hashes 36fa82a5, ef2fe222 and 86d03fb9.
- **Test counts:** "(22)"; 513/568/596 on both targets; 1002 on both; seven plants.
- **Log hashes:** gate c21d77b9, test 49793bac, negative c6511c96 and run b1810fa3 (scan-pre-check.log lines 2477-2507). Scan 195eb231 matches the input file itself, though no log prints it.
- **Run figures:** 843/27/17/0/0/0, 5 in 2, 9 of 27, 2096, 814 pairs, 6d57689, 28 in 17, and the 0.
- **Other claims:** "RESULT: PASS"; each log at its first attempt (`RERUN_NOTE` unset in every log; `SUPERSEDED_*` unset in run.log); the five calls under `rowcol`.

**Disagreements, or claims with no primary:**
1. **"28 in 17 workbooks carry hidden="false""** is more specific than the primary. The run log counts "with a hidden attribute of any value". The word "false" appears only in the script's parenthetical "(LibreOffice writes hidden="false")". No log measures the value.
2. **"814 pairs, no finding"** is the report's own summary, not a line in the log. The log shows "lo_in ... fail 12 ... lo ... fail 11". There are 0 outputs refused on an accepted input and 0 verdicts moved, so the summary is defensible, but it should say "no output refused on an accepted input".
3. **The "Script history before the logs" paragraph has no primary.** That covers `gen/test.py`, the negative dry run's failed control, "dry-run twice", and the scan fork's `rmdir`. It is working state but is not labelled so. The report labels only the scan's final stage that way.
4. **"<sheetFormatPr> its fourth held element"** is the spec's wording from ruling 1. In the code, `SETTINGS` has it as the second of seven entries. This is not an error, just a quotation of the spec.

## Summary
- **Q1, partly.** P1 is implemented in full, and each ruling's behaviour is in the code and tests. Beyond the spec: code built without `rowcol` changed (equivalent bytes, disclosed); `sheetprops`-only builds changed in ways whose `observe()` effect cannot be seen from the diff; there is an undocumented extra tool assertion; `kept_cols` drops `min`/`max` against ruling 3's wording; and the whitespace-between-`<col>` case would break the spec's 1-or-2 element rule.
- **Q2, yes.** Every required gate, test, negative and run line is present and says what is expected. The final scan is pending, and the ruling 3 count does not separate `hidden="true"`.
- **Q3, partly.** Every figure matches a primary. "hidden="false"" is unmeasured, "no finding" is loose, and the script-history disclosures have no primary and are not labelled as working state.
