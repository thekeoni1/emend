# M5 part 2 P2: independent check 1

A fresh subagent, given only the directory <home>/.cache/emend-build/m5p2-p2/check1. It held spec.md (specs/0033 at the doc commit), diff.patch (a9354ec^..61d23fc), commit-messages.txt, report-draft.md, the gate, test, negative and run logs, the scan pre-check log and the four record files. Their sha256 values are in <phase>/check1-inputs.sha256. It answered the three questions (CLAUDE.md, "Independent check before reporting"). Its answer follows verbatim; it was ASCII as delivered, so nothing was replaced.

---

1. Does the diff do what the spec says for P2 and nothing else? PARTLY (it does nearly all of it; a few small departures are not disclosed)

- The diff hashes to 0ea613f6..., the same as gate.log:4728, "the phase's combined diff". It touches eight files, the ones listed at gate.log:4731-4759. The doc commit modifies only specs/0033 and ROADMAP.md, at 5067dfcc... and 448a94ba... (gate.log:4672-4700). spec.md here hashes to 5067dfcc. The added "Planning rulings on P1" text is in diff.patch:1470+.
- pivot.rs (diff.patch:58-757) matches spec.md:292-342: the types, their fields and derives, lookup by content type in entry order, the sheet join, the first pivotCacheDefinition relationship, streamed parts with skipped subtrees, and no records part. The message is "a pivot table cannot be read" and "no such part" is used for a missing cache. There are 14 crate tests covering every item in the test list at spec.md:590-605, plus 2 tool tests (gate.log:5337-5361). The ruling 10 comment fix is comments only (gate.log:5318-5331).
- Departures:
  a) A cache with no `cacheSource` element at all is refused (`source.ok_or(PIVOT_UNREADABLE)`, diff.patch, read_cache; tested at diff.patch about line 1417, "// No cacheSource."). The spec's refusal list (spec.md:330-341) does not include it. The report lists only "a cacheSource without type". NOT disclosed.
  b) A `recordCount` or a `location` row or column that does not parse is refused. The spec lists only x, fld, cacheId and connectionId. The report covers this only loosely ("a number that does not parse") and does not list it under departures. PARTLY disclosed.
  c) A boolean other than 1, true, 0 or false is refused. Disclosed (report item 3).
  d) The catalogue entry is deferred to P3, although spec.md:394 puts it "in the catalogue". Disclosed (item 4), and it is consistent with P3's scope at spec.md:612.
  e) The second negative case, N2. Disclosed (item 5).
  f) `digest_fields` and `cache_fields` are computed per table, so a shared cache counts twice. The spec text is ambiguous here. Disclosed only indirectly ("32 against 28").
- Report item 2 (sheet holds the part name) is in line with spec.md:302 (`Option<Vec<u8>>`), so it is not a real departure.

2. Do the logs contain every line the spec requires? PARTLY

- Present:
  - TMPDIR input lines in all logs (gate:10, test:10, negative:7, run:17, scan-pre-check:9).
  - The gate's -B check with its history control (gate:4894-4914).
  - pub fn listing off and on (gate:5038-5304).
  - Tests for four feature sets on both targets (gate and test.log:1867-1920).
  - clippy and doc (gate:5431+).
  - The negative case with its control (negative:3141-3145).
  - Run on both targets with the check and the planted controls (run:4528, 4638, 4692-4706).
  - Kinds pivot_tables equality (run:4786, 4851).
  - with_cache and named_fields (run:4535, 4645).
  - Devices (run:4424, 5109).
  - Named record commit (run:44).
  - Calc chain (run:3348-3349).
  - Keep and open check (run:4969-4983, 6216, 6345).
- Gaps:
  a) Ruling 12 says "every mktemp call of a logged script, file or directory, is checked under TMPDIR". The counting library's `d=$(mktemp -d)` (gate.log:4529, also sourced in run.log:3127+) has no under-TMPDIR check in gate.log or run.log. gate.log:56 and run.log:97 count only the script's own calls. Report item 6 mentions only test and negative.
  b) scan.log is absent. That is expected per the brief.
  c) The dry runs the spec requires (spec.md:540) appear in no log. The report labels this a session statement.

3. Does every number and quoted string in the report appear in a primary? PARTLY

- Verified:
  - The four log sha256s and the four record sha256s match the files here.
  - The scan pre-check hash 94e0193e... matches.
  - The script hashes for gate, test, negative, run and scan all reproduce from the printed script text (4-space indent stripped, trailing newline).
  - pivotcmp.py and kindscmp.py are printed in run.log.
  - The doc hashes appear in gate.log:4672-4706.
  - The commit SHAs and the base appear at gate.log:4597-4612.
  - All quoted gate, test, negative and run strings were found, including: consumer gate:5708; 18/4/0 at gate:4895; 3 mktemp calls and 0 plain files at gate:56-60; test counts 515/608/646/980 and 1184; P1-P8 at test.log:2237-2343; the 27s at run.log:4528, 4716; 17 of 18 at run:4677; 24 of 24 at run:4882; FINDING at run:4665; the open totals at run:6216; the keep figures at run:4969-4983.
  - Every row of the figures table matches run.log:4657-4675, and the unplanned figures match run.log:4683-4695.
- Wrong or inexact:
  a) Gate end time. The report says 20:20:31Z; gate.log:5751 reads "run ended at 2026-10-12T20:20:30Z". The report attributes it to logged.out (not here), but it disagrees with the committed log by 1 s. The test, negative and run times match their logs.
  b) "lo ok 1090 fail 11" and the open line: exact, from run.log:6216. Note that crate 6 plus tool 6 refused inputs at run.log:6361-6364 equal 12, which is the lo_in fail count, not the "inputs refused by LibreOffice 9" figure. The 9 is the refused class and the other 3 are the count class (run.log:6366-6380), so it is consistent.
- Not found in any primary (session claims):
  - The target root sizes, 4499862550 and 4415291340 bytes.
  - The "one worksheet cache shared by two tables" explanation (arithmetically consistent, but never printed).
  - The setsid, nohup and stderr details, and logged.out.
  - The fork rm list.
  - The dry-run outcomes and the superseded/dry-final directory.
  - "each template checked byte-equal to the copy printed in records/logs/m5p2-p1" (P1 logs are not here).
  - "both hashes matched planning's before the commit". Only the prefix match is in gate.log:4675.

---

Session's response: the report was corrected for the gate end time (both stamps now given) and adds items 7 to 10: the counting library's mktemp under ruling 12 (a finding for planning), the refusal of an absent cacheSource, the refusal of every kept numeric attribute that does not parse, and the per-table against per-cache counting. The unsourced items are marked as the session's statements, with their files in <phase>. Nothing in the code or the logs was changed after this check.
