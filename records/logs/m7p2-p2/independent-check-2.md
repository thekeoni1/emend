# M7 part 2 P2, independent check 2 (the record run; its answer verbatim, but the home directory's path written <home>, once, and its non-ASCII characters written in ASCII: U+2013 as "-" 20 times, U+2026 as "..." 14 times, U+2212 as "-" once; the answer as given is sha256 a64694a8d448a18430e0201116a4fb220fb5e6aba4fc584d9d501ad79df328ed in <phase>/check2-answer.md; its brief, equal to the prompt the checker was given, is <phase>/check2-brief.txt, sha256 42353d5c6d8e5f55122744d03229df803bcf01b5a69d353417aec448777df791)

I read all the primaries myself. Overall: no blocking disagreement with the record's figures. There are seven wording or timing points, listed under the questions below and repeated at the end.

**Frozen stage logs**: all three files hash to the README's values. gate.log is 6ce0b431..., negative.log is d760f1c7..., run.log is 7c7ab5a2..., each matching in full.

## 1. Does the diff do what the spec says P2 does, and nothing else?
**Verdict: yes.**
- `git diff 58c4f51 5ae8367` touches two files only: `ROADMAP.md` (+2/-2) and `specs/0045-publish.md` (+346, no deleted lines).
- ROADMAP: the P1 row reads DONE 2026-10-28 and the P2 row reads IN PROGRESS (`specs/0045` ruling P). That is what ruling P asks for.
- The message gives each file with its blob sha256. Both match `git show 5ae8367:<file> | sha256sum` (b3121df7..., 466fc0da...). It ends with the Co-Authored-By line only and is ASCII (scan pre-check, "commit messages in the phase range").
- There is no code commit (record-gate.log 8856 section; pre-check "the range holds no code commit: yes").
- The record and the six logs are untracked, as they should be before the record commit.
- One point, already recorded by planning in ruling T (a): the subject says "rulings M to P", but the diff also adds rulings A to L and the P1 verification.

## 2. Do the logs contain every line the spec requires?
**Verdict: yes, with exceptions (E1 and E2).**

| Required line | Where it is |
|---|---|
| Record copied byte for byte, sha256 at the pin | record-run.log 2322-2327: copy sha 6ca1f6e8... = pin, cmp-equal, control |
| Assembly rerun, exit 0, output equal to the record | record-run.log 2329-2334: exit 0, stderr 0 bytes, sha and cmp equal, control. Script pinned 35f29a5f at 2290-2291 |
| 2401 of 2401 rows found, 2239 of 2239 input pairs | record-run.log 2353-2354, with a planted-copy control at 2355-2357 |
| Totals recomputed by awk equal the record's | record-run.log 2373-2377: `\| 4640 \| 159 \| 3 \| 0 \| 1878 \| 0 \| 361 \| 1881 \| 0 \| 358 \|`, with three controls |
| By-hand file sha256 and row count | record-run.log 2292-2293 (73b8e06d..., 4 lines) and 2302-2304 (3 rows) |
| Corpus counts 159 / 3 / 0 (decision 6 A, change TS) | record-run.log 2381-2393: 159 3 0, the results-alone control 159 0 3 refused, by-hand cells 3 of 3, rows naming a fail row 3 of 3, each with a control |
| Two assembly stops on scratch copies of the real results file | record-negative.log 6191-6218: AC exit 0 with sha equal to the record; A1 and A2 each print exit 2, line counts, and the STOP: line with the key cut (trailing ": " only). No row or value is printed. The fictional controls RC, R1 and R2 are at 6163-6189 |
| Two older stages and zips equal at start and end | record-gate.log (start at A93, end at 10741-10747) and record-run.log (2312-2318 and 2400-2406) |
| P1's keep unchanged | record-gate.log 239-244 and 10735-10739: 4502 files, listing 41afdbbf at both ends |
| Scan pre-check | EXPECT_RECORDS=1 with its control (6177-6179); six sibling logs listed and scanned (6072, 6133); RESULT: PASS with 0 hits, 0 errors, 0 structural failures (6333-6337) |
| FINDING lines | 0 emitted in each log. record-gate and record-negative contain the word 0 times. record-run contains it 17 times: 15 inside the printed script (lines 89-2082), plus the heading and summary at 2408-2409 |

The record itself: I recounted its own tables and got 162 corpus rows (27 per label), 2239 fixture rows with no repeated id, and the same totals and pair counts as the log. Its sha256 is 6ca1f6e8..., it has 2461 lines and 637291 bytes, and it is ASCII.

Exceptions:
- **E1. Copy step not logged.** Ruling V (2) says the driver step copying the stage logs "prints its sha256 equal to the frozen hash". No log in the repo prints that line; it is in the driver output, which is not a primary. The equality is only shown indirectly: the pre-check scanned the three copies at 6ce0b431, d760f1c7 and 7c7ab5a2, and my own sha256sum agrees.
- **E2. Pre-check scanned an older README.** It read README.md at 16185 bytes, sha256 7b3e07f9.... The current README is 17033 bytes, sha256 67ac45d9..., modified at 06:24:53, ten seconds after the pre-check ended at 13:24:43Z. The pre-check's PASS therefore does not cover the README as it stands; the final scan has to.

Minor wording problems in the logs (no check depends on them):
- **W1. Wrong part label.** record-run.log line 57 reads "phase: Milestone 7 part 1 P2 ... run.log". It should say part 2. The record logs' phase lines also name themselves `gate.log`, `negative.log` and `run.log` rather than the `record-` names (record-gate 337, record-negative 92).
- **W2. Stale text in the pre-check.** Line 18 reads "record files expected: 1 (P2 commits no record, change LT)", which contradicts itself. A control line says "none, the phase having no record".
- **Strictness note.** pairs.awk tests membership in a set, so a repeated row would not be caught on its own. The equal per-source counts and my recount (no repeated fixture id) cover this.

## 3. Does every number and quoted string in the report appear in a primary?
**Verdict: yes, with exceptions.** Every hash, count, totals line, machine argument, STOP string (including the trailing ": ") and FINDING count (17, 15, lines 2408 and 2409) matches a primary.

The exceptions:
- **R1. Unmarked string found in no primary.** Report line 27 quotes "every copy equals its source (and the stage logs their frozen hashes): yes". That string appears in no primary. It sits under "Logged record run", not under the session's statements, so it is unmarked. It is the same gap as E1.
- **R2. Check 1 described as committed when it is not.** Report line 43 says check 1's answer is "committed as `independent-check-1.md`". The README says the same in its "After check 1" section, which is not in the session-statements block. The file is actually held at `<phase>/logged/record/held/independent-check-1.md` (sha 017fb1cc..., which matches the README). It is not under `records/logs/m7p2-p2/`, and the pre-check reads "check answers scanned: 0". Both texts state as done something still pending.
- **R3. README time differs from the log.** The README's record-run section, which is not marked as the session's, says the record gate was "run from 2026-10-29T13:15:29Z". The log says "run at 2026-10-29T13:15:30Z" (record-gate.log 337). The report gives 13:15:29Z as the driver's time and marks it as the session's, which is acceptable.
- **R4. Scan generator hash differs from the gate's reading.** Under the session's statements, the README and report give the scan (record) generator as e3bb9c6a.... record-gate.log line ~187 hashed `gen/scan-record.py` as b8a430de... at gate time. This is consistent with change LX being made after the logged run (report line 42), but neither text says the generator's hash changed.
- The report does not mention E2 or W1.

## Beyond reading
- Ran sha256sum, wc, cmp-free comparisons, grep, sed, awk, ls and git show/diff/log, all read-only. I wrote, moved or deleted nothing.
- Recounted the record's own tables with awk: results by source and label, input/output pairs, repeated fixture ids, and the names listed in findings 3 and the "Listed" line (24 and 21).
- Read the held check-1 file's sha256 and checked file times.
- My tool output showed one corpus row of the record's table, including its seen columns. That is permitted (the record's own table), and I have not reproduced the values here.
- I did not read the list, the results file, the by-hand file or anything under `<home>/emend-testing`.
