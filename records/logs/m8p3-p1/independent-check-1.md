# M8 part 3 P1: independent check 1

The answer of a fresh subagent given only these materials, under
<scratch>/check1, and the three questions of CLAUDE.md ("Independent
check before reporting") with specs/0044 P1's check (the run log's
directory list against the usage text's flags, the record pairs' count
against the directories):
- the working spec specs/0049 (014bffadbf7d3f70832e1946f70143712427b97d1afe381e0e7f1916e7b52ed5), with rulings A to C, and specs/0044 at the phase commit;
- the doc commit's diff (git show 4f8fdfc) and the tool's source at the phase commit (its USAGE constant);
- gate.log, negative.log, run.log, the scan's pre-check output (49515dbadc0d2d1465b2d3b121821c5f0da08306633284a2825d94890969042f) and <phase>/logged/driver.out;
- the 18 new records, the eight part 2 P4 records and the m7p2 open record they were compared with;
- the logs README as it stood before the check (sha256 feee35146587a37976186e0a9b26f79529245ca4dc61f5577fe590255c47e1a9) and the draft report (89223b670b826ccecd42ceac9998cce2857c2feab448b1529d5a57ef0c416313).

The answer was extracted from the subagent's transcript by script into
<phase>/check1-answer.txt (sha256 45f8457ba72b1ef0ce317b9c620e1c7a7a3b2dcb7b46878dd37edc1f26933b75). It appears below verbatim. The
README's section "After the independent check" answers each exception.

root: <scratch> stands for <phase>, as in the logs
root: <phase> stands for <home>/.cache/emend-build/m8p3-p1, the phase's directory
root: <home> stands for the home directory of the account the runs ran under
root: <runs> stands for <corpus>/runs, as in run.log
root: <corpus> stands for the private corpus directory the run reads

1. Yes. The doc commit and the phase's work do what specs/0049 P1 says and nothing else.
- **Doc commit.** doc-commit.diff touches three files and nothing else: ROADMAP.md, specs/0048 and specs/0049 (new file).
  - ROADMAP.md: part 2's heading gains "DONE and PUSHED 2026-11-10 at 9f862ca", part 2's Close row reads DONE and PUSHED, and part 3's heading, paragraph and five rows are added.
  - specs/0048: the status paragraph, ruling O and "Planning verification of the close-out".
  - This is what spec-0049 "### P1" asks for. There is no code change.
  - The commit message carries the three sha256 lines (2fd9630f..., 28b639a8..., 36231e99...) and ends with the Co-Authored-By line. The scan pre-check, lines 6680 to 6700, reads the message forms at count 0 for em dash, session line and non-ASCII.
  - The base is f44debd (gate.log lines 319, 358, 9837).
- **Working spec.** I stripped the diff's "+" prefixes and compared the result with spec-0049-working.md. The only difference is the added "Planning rulings during P1" section, 35 lines. `sha256sum spec-0049-working.md` gives 014bffad...52ed5, which is the PLANNING_SPEC in driver.out line 1. Gate, negative and run logs and the scan pre-check each print that hash.
- **USAGE order.** In emend-copy-main.rs lines 1187 to 1203 the eight flags come in this order: formula, sheet, remove, delete-rows, delete-columns, comment, richcomment, commentbox.
- **Directory list.** run.log lines 3583 to 3594 list the eight flags in that order. Lines 3580 and 3594 read yes for "the binary's flags, in order, are the USAGE constant's at the phase commit" and "keep the usage text's order". The directory list at lines 4138 to 4147 has the eight `<runs>/2026-11-11-4f8fdfc-<name>` directories in the same order, "run directories there: 8".
- **List command.** run.log lines 4290 to 4300: `--run` is the formula directory, then seven `--run-plain` in the usage order (sheet, remove, delete-rows, delete-columns, comment, richcomment, commentbox). The checks "in the usage text's order: yes" and "the seven others are each a --run-plain directory: yes" pass. `--record` is the m8p3 open record .txt.
- **Record count.** `ls records/m8p3* | wc -l` gives 18: 8 sample pairs plus the open pair. run.log line 4254 reads "the record files are 18 and the run directories 8 ... yes". All 18 installed sha256 values at run.log lines 4259 to 4276 equal my `sha256sum` of the files in records/. `cat records/m8p3* | wc -c` gives 2755548.
- **Prune.** gate.log lines 186 to 304 remove only "target" from each previous directory (prune set "1 names: target"; kept 28 and 34), as ruling C says. No script, stage or earlier zip is changed: run.log lines 4361 to 4368 read yes.

2. Yes. The logs contain every line specs/0049 P1 requires, including specs/0044's P1 sections as 0049 adapts them.
- **Per-id comparison.** Each sample prints "compared with records/m8p2-<name>-2026-11-10-f625b99.txt per id (outcome/lo/lo_in ...)". The summaries at run.log lines 3678, 3743, ..., 4133 read "earlier 39 (same 39, changed 0, missing 0); new 0". Each also prints its outcome counts from summary.host.tsv ("outcomes: ok 39"), and asserts that run.host.tsv names the phase commit and the flag.
- **Open record totals.** The totals line is at run.log line 4180. Both totals lines and the difference are at lines 4194 to 4200: 2280/2268/2269 against 2251/2239/2240, a difference of +29 each, "no FINDING".
- **By-name comparison.** run.log lines 4202 to 4245: then 2251, now 2280, gone 3, new 32, in both 2248. Pairs leaving ok 0, becoming ok 0.
- **The list.**
  - The command by placeholder is at run.log line 4300.
  - Stderr notes: "stderr lines 0; of them notes 0" and "error rows ... 0; the notes' sum: 0" (lines 4304 to 4309).
  - sha256, bytes and mode 600 at line 4311.
  - Rows and counts by source at line 4312.
  - Counts by label at lines 4314 to 4322, with "sum to the rows: yes".
- **The stage.** Counts per source (312, 2268, 2268) with yes at lines 4336 to 4344. The opener's sha256 equals the phase commit's and the repository's. "staged files differing from their list rows: 0" with the assertion's command printed. Modes 600 and 700 read yes.
- **The zip.** sha256, bytes and mode at line 4356. Entries, byte-equality, stored and testzip at line 4357.
- **The negative.** negative.log lines 6185 to 6330:
  - case (a) L1 (missing summary), refused, exit 2, no list;
  - case (b) L2 (`--run-plain` under another corpus root), refused;
  - plus L3, with control LC;
  - case (c) F1 (another sample's directory), refused, with control FC and F2;
  - case (d) S1 (one byte changed, size kept), refused, with control SC and S2;
  - the cross-controls all read yes.
  - The NA48 control reads yes at lines 143 to 145. The last line is "negative log checks all as expected: yes".
- **The gate's prune.** Exactly two targets: 6475256385 bytes (gate.log line 237) and 2310161037 bytes (line 297), each "rm -r exit 0". The kept names are still there, and the rm -r form's control ran at lines 188 to 192.
- **Gate tests.** Off 544, ch 719, im 581, ci 761, cc 768, cr 826, all 1244, 0 failed on host and i686 (gate.log lines 11126 to 11483).
- **FINDING lines.** Exactly three, run.log lines 4208 to 4210, with the count line at 4372. gate.log and negative.log have 0 lines starting with FINDING.

3. Yes, with exceptions. Every number and quoted string I checked in report-draft.md and README.md appears in a primary, and I found none wrong. A few statements cannot be checked from the primaries here, and one is worded imprecisely.
- **Checked and matching:**
  - the three log sha256 values, byte counts, line counts and last lines (`sha256sum`, `wc -c`, `wc -l`);
  - the prune bytes and kept counts;
  - the test figures;
  - the open record totals and by-name counts;
  - list rows 4848 and sha 9307b03b...;
  - stage 4850/4848 and opener 0f2bed0b...;
  - zip 161525935 and 269a8485...;
  - records 2755548 bytes;
  - driver.out's stderr, "gone" and 28/34 lines, free space 725G, records count 18, and the logged.sh hash afe3c373...;
  - script hashes e6739e5c, 5a036ef8, da45f8af, b4fdee87, which match ruling A's d1 hashes;
  - every README line citation I tried: gate 186, 188 to 192, 196, 237, 245, 297, 406, 11126, 11132, 12143; negative 95, 6352; run 66, 79, 84, 3678, 4133, 4200, 4208 to 4210, 4277, 4311, 4312, 4314 to 4322, 4336, 4350, 4356, 4357, 4361 to 4368, 4406;
  - the timestamps.
- **My own recomputations, all matching:**
  - Per-id comparison with python3 -I -B for all eight sample pairs (the task asked for at least two). Each m8p2 .txt and m8p3 .txt has 39 host ids: same 39, changed 0, missing 0, new 0. Every triple is ok/ok/ok. The files are not byte-identical (cmp shows differences, which come from the timing and memory columns), but they are equal on outcome, lo and lo_in.
  - Open record by name against m7p2-open: then 2251, now 2280, gone 3, new 32, in both 2248. lo_in ok 2239 to 2268, lo ok 2240 to 2269, leaving ok 0, becoming ok 0. The headers are equal, with lo_in and lo in columns 6 and 7.
  - The three gone keys are `package.tests.shift.a_shared_formula_before_after_straddling_inside_and_deleted_whole.1`, `.2` and `.3`. These equal the FINDING keys at run.log lines 4208 to 4210. The old name has 3 rows in m7p2-open and 0 in m8p3-open. The new name's .1, .2 and .3 are among the 32 new (run.log lines 4243 to 4245).
- **Not checkable from the primaries here:**
  - Report: "<phase>/scan/pre-check-1.out". The scan pre-check names its stderr as `<phase>/scripts/out/scan-pre-check.stderr`; "pre-check-1" appears in no primary.
  - Report: "the four dry-layout lines of d1 read as expected". No primary labels four lines as the dry layout. The likely candidates, scan-pre-check.log lines 96 to 99, read yes, but d1's scan is not supplied.
  - README: the claim that logged.sh checked the scripts cmp-equal to d1's. driver.out prints the hashes but no cmp line.
  - README: `<phase>/tmp/tmp.Ea7F6fTV9v`, the dry/d1 size 6697047345, `superseded/pre-logged/scripts-out` and `removed-fns.txt`. They come from files not supplied; the README itself places them under "Figures from files outside this directory".
  - README: "Dry run d1 printed the same three". This is consistent with ruling B ("expected again") but d1's log is not supplied.
- **Imprecise:** the report says "The one 'no' line in each is the information line 'the diff-alone sentence: no'". scan-pre-check.log has two more lines ending in ": no":
  - line 44, an indented planted-fixture output ("c-defined.sh: control ...: no"), whose wrapping check reads as expected;
  - line 6573, "EMEND_PRIVATE_DENYLIST set: no (unset by this script)".
  Neither is a failed check, so the claim holds if read as "the one check-level no line".
- **Other:** no non-ASCII characters and no U+2014 in report-draft.md, README.md or doc-commit.diff (`grep -P '[^\x00-\x7f]'`).

Overall verdict: yes to all three. The diff and the logs meet the spec; the report and README figures match the primaries, apart from four unverifiable statements (two in report-draft.md, two in README.md) and one imprecise "one 'no' line" sentence in report-draft.md.