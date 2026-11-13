# Docs intro P1 logs

specs/0050-newcomer-intro.md, P1, with planning's rulings A to E on the
P1 stop report (given by message; planning writes them into the spec
after this record commit). `<phase>` is the phase's directory,
`<home>/.cache/emend-build/docs-intro-p1`; `<run>` in verify.log is
`<phase>/logged/run`.

## The commits

- Doc commit 40ee6b1b803ae9e9881a7ba74bf1056fa8176226, parent e9dcaac: specs/0050 at sha256
  a7d109d992dcf679fc2a8ad39cd9a222cd27caae0753f7a0b30779e97033f66d, specs/0049 at
  921473851f14db07ff6bbf4d86332ec294bf0f10a2e76a6d94dc4215a1e1b82c and ROADMAP.md at
  0e1fad895e3e2348ae401e388016c6b1f0971e0739eabc44054eb3fa809e4f07 (verify.log lines 569-574).
- First code commit 4dfd4f47c5fe4ea69aac818559063c7d1cf63292, parent the doc commit: README.md's opening
  replaced by the spec's text with the example, and lib.rs's first two
  doc paragraphs replaced and the example added as a doc test that runs.
  It was first made as c58ebec and amended at once for a wrap in its
  message alone, same tree; nothing referred to c58ebec (ruling E).
- Second code commit 2f7891a4048e195debebc8d97613d37956d1404c, parent the first, change 1 (ruling C):
  lib.rs's second paragraph is the README's openpyxl paragraph's first
  sentence alone, because the next sentence names README sections the
  crate docs do not have. README.md unchanged.

Rulings A and B: the hidden lines use `Package::new()` (the spec named
`Workbook::new()`, which does not exist); the example stands at the end
of the module doc under "# Example", its two over-width lines accepted.
Ruling D: no environment line and no prune in this phase; nothing was
removed.

## The logged run

| Log | Last line | sha256 |
|---|---|---|
| verify.log | verify log checks all as expected: yes | 0f2913fe443c887bb3cef3f4ce93b4594fe3d54f584c072475cc8ee67d89c4d8 |
| driver.out | logged run ended 2026-11-13T22:32:49Z | 76dd358b139eed16b683a187a5de1293982540b64027b8dccd856e17ab1658d1 |

Run 2026-11-13T22:24:12Z to 2026-11-13T22:32:49Z (lines 2 and 809);
FINDING lines 0 (line 807). driver.out is the logged driver's own
output: the script's exit 0, its standard error 0 bytes, no line of
verify.log holding the home directory and none ending ": no" (driver.out
line 2). verify.log prints the script that wrote it.

- Acceptance 1: both code commits together name README.md and
  crates/emend/src/lib.rs and nothing else (line 563); change 1 names
  lib.rs alone (line 565).
- The README block equals the doc test's visible lines byte for byte,
  15 lines (lines 594-595); the README head is the spec's text with the
  example (line 599); lib.rs is the base with the two paragraphs
  replaced and the example block inserted, nothing else (line 612), the
  second paragraph the openpyxl paragraph's first sentence (line 616).
- Acceptance 3: the README from "## Dependencies and features" on is
  10351 bytes with sha256
  3c14bdd673a02238ae21bf138d364907d9c82ed1bbed6075fec7782ed9d6435e at
  the base and at the code commit (lines 602-604).
- Rule 1: 870, 456248, 1159829654 and 7872 each found by grep -n -F in
  the record its sentence cites (lines 669, 677, 683, 689); every
  sentence of the new opening that holds a digit cites a records/ path
  (line 627).
- Rule 2: calamine 0.38.0 (crate sha256
  f431b67cba6122a1f7d26b7adafc71bbdf07026095fe811f5efc025d8748e1d8, the
  index's cksum) describes itself as "An Excel/OpenDocument Spreadsheet
  reader and deserializer in pure Rust" (line 719); rust_xlsxwriter
  0.102.0 (sha256
  6ca3d0e879a578c1313c2800ab0c6c029d38eed18098675e50341a30d32372ed) as
  "A Rust library for writing Excel 2007 xlsx files" (line 732). Both
  support the README lines (lines 722 and 735). The crate files stayed
  in `<run>/crates`, read-only; nothing from them is in the repository.
- Rule 3: records/decision-architecture.md names umya-spreadsheet
  3.1.0 (line 705); the README bullet cites it and says emend has not
  measured it.
- Acceptance 2: scripts/check.sh exits 0 with nine PASS lines (lines
  753-764); the doc test passes on the host and on
  i686-unknown-linux-musl (lines 771 and 777); with its set_cell line
  removed it fails with 'Error: "no cell"', and with the value changed
  it fails on its assertion (controls, lines 786 and 793). The format
  probe reads 989464 section bytes; less probe-zip-xml 533216 the
  budget is 456248, 2504 under the ceiling of 458752 (lines 803-804).
- Acceptance 4: no U+2014 in the spec, README or lib.rs (lines
  655-657); the three commit messages ASCII (lines 739, 741, 743);
  scripts/privacy.sh over the range: 3 commits, 3114 blobs, 0 findings
  (lines 747-749).

## Ruling G: the log's U+2014 written as escapes

This record was first committed as 25ceb42, with change 2's doc commit
0aba0fb and code commit 00fae94 on top; all three were local and are
superseded. Change 2's dry run d5 failed the gate's hygiene step on
this verify.log: the script's Python block, written through a tool
whose JSON input decoded the text `\u2014` into the character, held
four U+2014 characters on two lines, and the script prints itself.
Nothing caught it before: the earlier dry runs and the logged run ran
the gate while the log was untracked, and the pre-commit hygiene run
used `git add -N`, which stages no content. By planning's ruling G the
history was rebuilt from 2f7891a, and the committed verify.log is the
logged run's file with each U+2014 written as the six ASCII characters
of its escape, by this command (run with the repository as the working
directory; `<orig>` is the logged run's file, kept in `<phase>`):

    python3 -I -B -c 'import sys; d = open(sys.argv[1], "rb").read(); open(sys.argv[2], "wb").write(d.replace(b"\xe2\x80\x94", b"\\u2014"))' <orig> records/logs/docs-intro-p1/verify.log

- The logged run's file: sha256
  fa264d9464e7c369fcb3aa98f759ae9ec2cd8ec34be1cbb656dd4d5258eedc64.
- The committed file: sha256
  0f2913fe443c887bb3cef3f4ce93b4594fe3d54f584c072475cc8ee67d89c4d8.
- Both have 810 lines. `diff` of the two prints one change, lines
  319-320 against 319-320, four lines marked `<` or `>`, and nothing
  else; both lines are the printed script's U+2014 checks. The
  committed file holds no byte outside printable ASCII.

The line numbers this README and independent-check-1.md cite are the
same in both files. independent-check-1.md names the file the check
read, the logged run's, by its sha256 fa264d94... above.

## Scripts

| File | sha256 |
|---|---|
| script scripts/docs-intro-p1-verify.sh | cec8aefdede6808e53aa5e186674be57c40d33b41bc6c33ab4b4b89a12154c1e |
| logged driver logged.sh | 1c46e63257dbc8f3e2b6cd2c8d371e433b13ac5b1f1712ecae8d0f60d385baf6 |
| dry driver dry.sh | 99b1517e1279a5e09059b4996c7301add8f1b653ff4ec1f14387d7de932be1be |
| the edit script apply.py, for the first code commit | 96e13645edbab7f7e4fee77278661f16ce2546730e0c34848e4fa978e544f524 |

The script is new for this phase, not carried from an earlier phase's
gate; the phase changes no gate file.

## The dry runs

Four, under `<phase>/dry/dN`, each a full run of the script at the time
(0 bytes of standard error, no FINDING line in any). d1 and d2 ended
"no" on defects of the script, not of the texts: the paragraph check
counted difflib opcodes, which split one replacement in two; the gate's
"PASS native (allowlisted -sys: none)" line missed an anchored pattern;
the planted removal was expected to fail on the assertion but fails on
"no cell" first, so two plants now assert their own texts; and a stale
line reassigned the inserted block. d3 ended "yes" and went to planning
in the stop report. After change 1 the script took the two code commits
and the one-sentence paragraph (the d3 script kept as
`<phase>/superseded/docs-intro-p1-verify-d3.sh`, sha256
0c9fee94fe59d2d11843f185ed72e10f91b0696316a86177bdf283eb0f78c89f); d4 ended "yes" with 0 FINDING
lines (sha256 11eb0ee67b2220f12161cb8cc4444874fdb6d18fa88d28d308ef638287d9a73c), and the logged run
followed on planning's word.

## The independent check

independent-check-1.md: one check, "yes with exceptions" on each of the
three questions.

## After the independent check

The logged run is not repeated (it is made once); these points are
recorded, not fixed by a rerun.

1. Question 2, exception 1, CONFIRMED: the calamine test looks for the
   substring "read", which "Spreadsheet" also holds, so that test could
   not fail on this description. The verdict rests on the printed line
   instead: calamine 0.38.0's description says "reader" (verify.log line
   719). A later script should match whole words.
2. Question 2, exceptions 2 to 5: the budget's FINDING branch, the
   privacy scan and the listed checks have no control of their own in
   this log, and the check at line 592 is labelled more widely than it
   tests (it looks only for a quoted file name with a dot; the core-only
   claim rests on the doc test passing with no feature, lines 766-777).
   Recorded as found.
3. Question 3, point 1: the script's exit, its standard error and the
   home-directory count are in driver.out, now committed beside the log
   (it was not among the check's inputs).
4. Question 3, points 2 to 4: the dry runs, the scripts' sha256 and what
   d1 and d2 found are in `<phase>` (dry/dN/verify.raw, the scripts),
   which this record names and does not commit; d3's result is in
   planning's message of the rulings.
5. Question 3, point 5: "nothing referred to c58ebec" is the
   implementing session's statement; the chain from e9dcaac to 2f7891a
   that verify.log lines 541-544 print, each commit with its parent,
   does not hold c58ebec.
6. Question 3, point 7: the range for the three planning files' sha256
   is now lines 569-574.
7. The note on the umya bullet (with lazy_read, unparsed worksheets are
   written back from their raw bytes, decision-architecture.md lines
   30-33) concerns the spec's own wording and goes to planning.

## Addendum: change 2 (ruling F) and its logged run

Planning verified the first logged run and judged the independent
check's note on the umya line (ruling F): the cited record does not
support "a model of the whole file". The spec's text for that bullet
now reads "reads a workbook into its own model and writes a new file
from it", and the README bullet says so verbatim. The commits, on the
rebuilt record commit a46fe52 (ruling G, above):

- 3a90317 doc commit: specs/0050 alone, sha256
  a00e7d6a0806a50411754a33779fb35ea26a7146823f6df5c946e500c0a902ce,
  the same patch and message as the superseded 0aba0fb.
- 4b2a23e code commit: README.md alone, the umya bullet, the same patch
  and message as the superseded 00fae94.

The script's change 2: the commit chain of seven, the spec pinned to
that hash, Acceptance 1 over the three code commits, the calamine test
on the whole word "reader" (the check's point 1) and the xlsxwriter
test on the whole word "writing", each with a control. With ruling G:
the script spells U+2014 as chr(0x2014), holds no byte outside
printable ASCII and checks that of itself, and checks that no file of
this directory at the code commit holds U+2014, its control the
superseded 25ceb42's verify.log, read by the same command form.

Dry run d5 (sha256 50650063281b995f54611c19697eb31313b67598cd19669224ad75897abfca87) ended "no": the gate's hygiene step failed on
the U+2014 in the first verify.log, the finding ruling G answers. Dry
run d6 (sha256 7f947fd71799dae92f845902b074a3a44383a9f758208071131d7c5de5a33a77) ended "yes" with no FINDING line.

| Log | Last line | sha256 |
|---|---|---|
| verify-change2.log | verify log checks all as expected: yes | a5809b5e06ece067dbd269df415d675ebb2877a206309ac1b5fa0a7cede80785 |
| driver-change2.out | logged run ended 2026-11-13T23:10:45Z | aa8a456dafac2548b8195f6a7f4102c11f410a8e85cb73a901bf9b34c228db2d |

The run, once, 2026-11-13T23:02:10Z to 2026-11-13T23:10:45Z
(verify-change2.log lines 2 and 901), at 4b2a23e (line 593), FINDING
lines 0 (line 899); driver-change2.out line 2: exit 0, standard error
0 bytes, no line holding the home directory, none ending ": no". The
script as run: sha256 cd43f489cca488790547d771015f18f12360a1989b7d7d5bbda6dea799d2a769 (line 4); the
logged driver logged-change2.sh: sha256 40a10842f23db4a9e41388009ecda81209fcb9beefed3f3c4fd6ec7d3bdea291.

- The spec at the code commit is the pinned one (line 605).
- Change 2 names README.md alone; the three code commits name
  README.md and crates/emend/src/lib.rs and nothing else (lines 617
  and 629).
- The README head is the spec's text with the example; the README block
  equals the doc test's visible lines (lines 661 and 665); the tail is
  still 10351 bytes with the same sha256 at e9dcaac and at the code
  commit (lines 668-670).
- calamine 0.38.0's description holds the word "reader" and
  rust_xlsxwriter 0.102.0's the word "writing" (lines 788 and 801); the
  whole-word test refuses "A Spreadsheet writer" and accepts "A
  Spreadsheet reader" (lines 803-804).
- The script holds no byte outside printable ASCII (line 809); no file
  of this directory at the code commit holds U+2014, and the same form
  finds it in 25ceb42's verify.log (lines 813-822).
- The privacy scan over the range: 6 commits, 3120 blobs, 0 findings
  (line 839). The gate passes with nine PASS lines; the doc test
  passes on the host and on i686 (lines 855-869); the budget is 456248,
  2504 under 458752 (lines 895-896).

No second independent check (ruling F).
