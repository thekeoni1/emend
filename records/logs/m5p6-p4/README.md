# M5 part 6 P4 logs

specs/0037-measurement-and-close.md, "P4: the README" and "The README",
and planning's rulings to AI, held uncommitted in the working spec
(sha256 8019cc257ec22efd399b4ef095719d36fd3e9e6df7b3c9ff07b6b34c55dff55b).
The phase commit is 9e171da717740f589f0c85b8e20db65dca019b15, the second
README commit. `<phase>`
is the phase's directory, `<home>/.cache/emend-build/m5p6-p4`.

| Log | Last line | sha256 |
|---|---|---|
| gate.log | gate log checks all as expected: yes | 6b80688727c3b972aa2fad1059d963f0280ac060012d2f28f54de60cb2ce226e |
| test.log | test log checks all as expected: yes | 744bc54a209c6b282002d34c1d67ea62b78f4b64a865e0b73dd96d10353c7e42 |
| negative.log | negative log checks all as expected: yes | 2523186bd01c3fd7fee9e9c80b2bc72170708f98f0788944189c1fe971ffb7aa |
| run.log | run log checks all as expected: yes | b95091210393b1940858f124dc48cdf5ed8a85679d58d530a625ba491f913d05 |
| scan.log | written last, by the scan | |

The logs ran serially at the phase commit and ended at
2026-10-18T14:23:57Z, 14:25:13Z, 14:25:15Z and 14:26:32Z. The gate, test and
negative logs are the second of their kind and the run log the third: the first four ran at the first README commit,
7355d98, ended "checks all as expected: yes", and are superseded by
ruling AI (below). Each log's RERUN_NOTE names its first and RERUN_FILES
hashes it. No logged attempt was refused or stopped.

**FINDING, change RRO: the second run log failed the scan's pre-check.**
It ended "run log checks all as expected: yes", but the scan found it
printing `<phase>/`, from the driver's RERUN_NOTE, with no root line
declaring it. The run generator now declares `<phase>`, as the gate, test
and negative scripts do, and the run log was run a third time. The second
run log and that pre-check are kept in `<phase>/superseded/run-rro-1`; the
third run log's RERUN_NOTE names both earlier run logs and RERUN_FILES
hashes them. There is no size log: the phase changes
README.md alone, and the README cites P3's size record.

## Before the logs (the session's statements; no log prints them)

1. **Ruling AG.** The doc commit's first message named the next milestone
   by its number, which the count rule reads as a count. It was reworded
   by `git commit-tree`, and the README commit rebuilt on it, trees
   unchanged: doc aad0b35 (was ee2564b), README a1c60f6 (was 324c8ce).
   The old pair is on branch p4-before-reword.
2. **Ruling AH, a FINDING.** The run generator's README check found three
   sentences of the comparison paragraph holding digits and citing no
   record. Each now cites the comparison record; the README commit was
   amended to 7355d98 (was a1c60f6, kept on branch p4-before-cite), its
   message byte-identical.
3. **The first dry negative log failed one check**, "N2: a MISSING line
   names the absent path": its `count_matches` call was given two mode
   words, so the second was taken as the pattern. Change QZ9 gives one
   mode word, and the generator now refuses the form. The dry log is kept
   in `<phase>/superseded/dry-negative-1` (sha256
   514940886d2e5e271683d1234287cbf685e9ef75b86f5f1d826a26477f938804), the
   script as it was in `<phase>/superseded/negative-script-1` (sha256
   d9d4c5f468e71fc8e64a538476923246ba0fe0a47888475325178133ae2cc15f).
4. **Ruling AI.** The independent check of the first four logs (Q1, Q2
   and Q3 YES) noted the `images` line still citing M3 part 7's size
   record. Planning ruled that every feature line cites
   records/size-m5p6-2026-10-18-ef9da37.md, which carries all twelve
   costs (images 57620, the others unchanged), and that the fidelity
   sentence names the samples as part 5's. The second README commit,
   9e171da on 7355d98, makes both edits; the generators were repinned
   to it and the four logs rerun. The first four logs, as installed and
   as written, the first scan pre-check and the first check's inputs and
   answer are kept in `<phase>/superseded/logs-1`.
5. A first dry gate, started with planning's previous working-spec hash,
   was stopped 14 s in and kept in `<phase>/superseded/dry-stopped-1`.

## Gaps the second independent check found (not fixed; left for planning)

- The gate's own ruling Z line covers 0 names: the gate calls its helpers
  through `is` and `isnot`, which the defined-helper program does not read
  (gate.log, "0 names called").
- Only run.log asserts its RERUN_FILES hashing with a check line; gate.log,
  test.log and negative.log print the hashes without one.
- The final scan names the two superseded scan pre-checks by RERUN_NOTE and
  hashes them by RERUN_FILES (`<phase>/superseded/logs-1` and
  `<phase>/superseded/run-rro-1`).
