# M7 part 1 P2: independent check 1

I answered all five questions. Questions 1 and 2 are a clean yes, question 3 is a yes, and questions 4 and 5 are yes with one small exception each. The record's own text also has one wording error. My scratch files are only under <phase>/check1/tmp. I wrote nothing else and did not run git writes, cargo or the phase's scripts.

## 1. Do the record's rows and totals come from the results file and the list? Yes

I checked this with my own Python script (<phase>/check1/tmp/chk.py, run with `python3 -I`). It did not reuse planning's code.

- **Inputs:**
  - The list (sha256 46cb9453...) has 6018 rows: 1560 corpus, 2229 fixture and 2229 input.
  - open-results.txt (sha256 9bf8bc30...) has 6018 rows. All 6018 match the list's rows in order, and there are 6018 distinct keys.
  - Results by source: corpus 1528 pass and 32 fail; fixture 1864 pass and 365 fail; input 1874 pass and 355 fail.
  - The by-hand file (32 rows): all 32 are the 32 refused corpus rows. 30 read "repair (by hand)" and 2 read "repair (by hand; repair failed)".
- **Totals:** my line, from the results file with the by-hand file applied, is | 6018 | 1528 | 32 | 0 | 1864 | 0 | 365 | 1874 | 0 | 355 |. It equals the record's line 20.
- **Pairs:** fixture pairs by (input, output) come to pass/pass 1842, refused/refused 333, refused/pass 22 and pass/refused 32. These equal the record's line 22.
- **Seen values:** 53 were compared and 0 differ, as on line 24.
- **Every row:** each of the record's 1560 corpus rows (lines 44 to 1603) and 2229 fixture rows (lines 1611 to 3841) equals, field for field and in the list's order, what the list, the results file and the by-hand file give. There are 0 mismatches.
  - Bytes and sha256 per source: corpus 1560 of 1560 and fixture 2229 of 2229 (3789 of 3789). The fixture rows' input pairs match the list's input rows 2229 of 2229.
- **Finding lines:** the workbook ids in findings 1 to 4 (lines 30 to 33), the 32 test names in finding 5 (line 34) and the 22 names under "Allowed" (line 36) all equal what I computed.
- **Stage files:** all 6018 files under <stage>/files match the list's bytes and sha256. None is missing and none differs.
- **Privacy:**
  - The record has no ref or expect column. Of the list's 24 distinct references, 0 appear anywhere in the record as a whole word.
  - The results file has 752 non-empty message cells with 1 distinct text, and that text appears 0 times in the record.
  - The record has 0 non-ASCII bytes.
  - The seen1 to seen3 values do appear, as the template and the spec allow, and only on rows that have a reference.
- **Disagreement (wording only, planning's text):** findings 1 to 4 (record lines 30 to 33, from line 122 of the assembly script) say "whose host row and every other sample pass". That is not literally true:
  - The chart-surface sample repairs on all 27 workbooks. So for the edit, protect and remove workbooks, another sample (chart-surface) does not pass.
  - Of the 27 chart-surface workbooks, 5 also have a non-pass row from edit, protect or remove.
  - The host row passes on every one.
- **Minor:** line 39's parenthesis "(the list holds them; they are not copied here)" reads as if the seen values were left out. It means the references.

## 2. Does the restored check's extract equal the script's lines? Yes

- negative.log line 5456 gives the range 865 to 888. Line 5460 gives 24 lines, sha256 7c70651d..., equal to the script's lines by sed. Line 5461 reads "yes" and its control (line 5462) also reads "yes".
- I took lines 865 to 888 of <phase>/check1/m7p1close-push.sh myself (sha256 3e85a0b6..., the pin at negative.log 5398). They hash to 7c70651d..., and they are byte-equal (cmp) to the printed extract at negative.log 5464 to 5487 once its 4-space indent is removed.
- XA is accepted (5529) and XB refused (5534). XC to XG are at 5536 to 5560.
- <repo>'s HEAD is unchanged after the clones, and 0 files there match records/excel-check-m7-*.

## 3. Does the diff do what the spec says and nothing else? Yes

- `git show ad139c3` is byte-equal to doc-commit.diff. The parent is a72467d.
- It touches two files only: ROADMAP.md (+2/-2) and specs/0044-excel-check.md (+324).
- The blob hashes 5b32ead8... and 2ebcd1e2... equal the message's lines 7 and 8.
- The message is in rule GN's form: one "<path> sha256" line per file, and it ends with the Co-Authored-By line. There is no Claude-Session line and no non-ASCII byte. It has no count, so it needs no diff-alone sentence; 42ddf3b has the same form.
- **ROADMAP.md:** the P1 row now reads DONE and the P2 row IN PROGRESS, as ruling Y says.
- **Spec:** the committed copy (701 lines) runs through ruling Y. The working spec (757 lines, sha256 9de4dd8e...) differs from it only by adding rulings Z to AD (spec lines 667 to 722). That matches the tree's single " M specs/0044".

## 4. Do the logs contain every line the spec requires for P2? Yes, with one exception

- **Gate (ruling Z):**
  - CACHEDIR.TAG is printed as information (gate.log 242 to 244).
  - `du -sb` is printed before the prune: 6142303428 and 73379815 bytes (245 to 248).
- **Gate (ruling AD):**
  - The prune is `rm -r "${PREV_DIR:?}/${n:?}"`, exit 0 for each name (249 to 251).
  - The names are gone and every kept name is still there (252 to 253).
  - The R90 control reads "target: rm -r exit 0; output bytes 0" (189 to 193).
  - No `-f` is used on the prune and no tag is written (source at 5786 and 5804).
  - The rerun line names attempt 1 (379 to 384).
- **Negative:**
  - A1 and A2: exit 2 with the expected STOP lines; AC: exit 0 with the record's sha256 (5426 to 5433).
  - The extract and the clones, per ruling X: lines 5454 to 5560.
- **Run log (ruling W):**
  - The copy and its sha256 (2214 to 2219).
  - The assembly run again, equal (2221 to 2226).
- **Run log (ruling AB):** pairs per source 1560, 2229 and 3789; inputs 2229 (2228 to 2249).
- **Run log (ruling AA):** totals by awk with the results-alone line as information (2251 to 2275).
- **FINDING lines:** "FINDING lines printed above: 0" (2284).
- **Scan:** scan-pre.log scanned the record (5903) and the README (5891). It ends "RESULT: PASS" (6059).
- **Exception:** ruling V says "the run log and the README print its sha256", meaning the assembly script's. The run log does (around line 2188: 54f344a7...). The README does not: neither 54f344a7 nor 95a69b8a appears in it.

## 5. Does every number and quoted string in the draft report appear in a primary? Yes, with one exception

Every hash, count, byte size, line range, exit code and quoted string I checked against the diff, the logs, the README, the record and the spec matches.

- **Lines 55 to 57 (exception):** the report says the other lines holding the word FINDING are run.log 1838, 1876, 1882, 1934, 1955, 1956 and 2283. run.log actually holds the word on 16 lines. The report also leaves out 116, 118, 152, 186, 191, 250, 259 and 614, which are comment lines in the printed script. The count of 0 emitted FINDING lines is correct: gate.log and negative.log hold the word 0 times.
- **Lines 66 to 67 (imprecise):** "attempt 2 with RERUN_NOTE and RERUN_FILES naming attempt 1" holds only for the gate (gate.log 4 to 5). The negative and run logs have both unset (negative.log 3 to 4, run.log 4 to 5), which fits since they never started in attempt 1. RERUN_FILES also names the gate script, not only the gate log.

## Summary

The record is sound. Every row and total traces exactly to the list, the results file and the by-hand file, and the restored check's extract is byte-equal to lines 865 to 888 of the push script. The doc commit holds only the spec and ROADMAP.md in rule GN's form. The disagreements are:
- The logs README lacks the assembly script's sha256, which ruling V requires.
- The draft report's list of other FINDING-word lines in run.log leaves out eight comment lines.
- The record's findings 1 to 4 say "every other sample pass", which is not literally true where one workbook has two findings.

None of them affects a row, a total or a hash.
