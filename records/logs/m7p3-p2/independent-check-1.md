# Independent check 1 (M7 part 3 P2 record)

The answer of the fresh subagent, unedited (ruling L). It was given the
working spec (4fc4371f), the spec at the phase commit, gate.log, run.log,
the README draft before the dispositions (sha256 5571c040), git status, a draft
report, and a script (sha256 56bf68df) and its output (sha256 36260c0e)
that read the quoted texts from the private run directories. Its line
numbers are those files' lines. The README's dispositions answer it.

---

Answer to Q1. Answer: yes, with exceptions.
- The tables do what ruling O asks. I diffed the 71 table rows in README.md (lines 111-169, 178-182, 190-191, 199-202, 211) against the 71 rows in extract-texts.out and they match byte for byte. They are grouped as ruling O's five groups, with counts 59/5/2/4/1, which matches spec lines 832-856. The outcome words are the summary's (ok/error, ok/fail/-), and the quoted texts are word for word what extract-texts.out holds. Every run.log line number given points at the matching FINDING line (run.log 2589 to 7125, the 71 lines).
- No original file name and no cell content appear. Workbooks are named only by id, producer and tags. The id table (README 82-93) matches the producer and tags columns of records/m7p3-anchor-2026-11-05-d7a8196.md for all 12 new ids.
- The disclosures are there: the logs table (34-37), stderr (45-49), slips and superseded versions (213-233), and the check (235-240). I was not given P1's README, so I can't confirm it is "like P1's" beyond those headings.
- Exception (a): README line 240 is still the placeholder "(DISPOSITIONS)". The README is not final yet.
- Exception (b): README 96-98 says the quoted texts come from stderr.host.txt and reason files in the run directories. It does not say those files are private and uncommitted. I found that none of the quoted texts appear in run.log, gate.log or any of the 118 records (0 hits), so a reader of the commit cannot check them. The report draft (42-45) discloses this; the README should say it too.
- Exception (c): README 73 says "all on the 12 new ids". The FINDING lines fall on 7 of the 12 new ids. It should read "all on new ids".

Answer to Q2. Answer: yes, with exceptions.
- The flag list and the exclusion are there: run.log section 2 lists 59 flags, then "--new-sample is in the usage text, and is excluded", then the 58 flags run, then "every flag of the usage text but --new-sample is run (58 of 59): yes".
- There are 59 run headings "into <runs>/2026-11-05-d7a8196-<name>", and the directory listing after line 4618 marks each one "there".
- The outcome counts from summary.host.tsv are there: 51 lines "ok 39", 1 "error 5 ok 34", 1 "error 6 ok 33", 6 "error 1 ok 38", all "rows 39".
- The commit and flag assertions are there: 59 lines "run.host.tsv: run rows 1; commit d7a81968f39f...". There is 1 "naming the phase commit and no tool argument: yes" and 58 "...and the flag alone: yes".
- The comparisons are there: 59 lines "earlier 27 (same 27, changed 0, missing 0)". The passthrough's pick is records/m6p6-roundtrip-2026-10-24-d88f735.md, with ruling M and change TG named. There are 71 FINDING lines.
- The stderr checks are there: 59 "its stderr is empty: yes" and 118 "stderr bytes: 0". Both logs have their own-stderr checks: gate.log 10949-10950 and run.log's last section.
- The final verdicts are there: "runs done 59 of 59 ...; FINDING lines so far: 71", "FINDING lines printed above: 71", and "run log checks all as expected: yes". gate.log has 742 ": as-expected" lines and 0 ending in UNEXPECTED, ends with "gate log checks all as expected: yes", and shows PRUNE_PREVIOUS list with "the prune set is empty".
- Exception: the 118 "stderr bytes: 0" lines are 59 run-script lines ("...; exit 0; stderr bytes: 0") plus 59 record-script lines. They are not "118 record-script lines" as ruling O and README line 48 say. The log is complete; the description of it is wrong.

Answer to Q3. Answer: yes, with exceptions. Items with no primary or that contradict one:
1. README 43, "15:39:50Z": gate.log line 291 reads "run at 2026-11-05T15:39:51Z", and 15:39:50 appears in no primary.
2. README 48, "118 record-script lines": the log shows 59 plus 59 (see Q2).
3. README 46, "and so was each driver capture of it": no primary shows driver captures. Only the scripts' own stderr checks are logged.
4. README 230-233: "68355fc9" (the first K98 script), "run-gen1", "scan-gen1 (scan.py ed360c54, script 0b6c8b34)" and "five k98-test directories" appear in none of the primaries. The spec's only "scan-gen1" is P1's, in ruling L at line 686, which is a different phase directory.
5. README 222-223, "dry.sh d8903793, now in <superseded>": the hash is in ruling M, but no primary says the file is in <superseded>.
6. Report-draft hashes: I recomputed these myself and they match the files in the check directory, but no log records them: README 5571c040, extract-texts.py 56bf68df, extract-texts.out 36260c0e, working spec 4fc4371f (913 lines). The README hash will change once "(DISPOSITIONS)" is filled in, so report-draft line 14 has to be updated after that.

Everything else I checked is in a primary:
- spec e41e5b45 (run.log, gate.log); ROADMAP 6de20108 (gate.log); PLANNING_SPEC 061dc48b
- gate.log 6a3e7ed3 at 835285 bytes; run.log 4e633756 at 477964 bytes; both last lines
- 15:52:04Z and 20:08:42Z
- gen/gate.py 6e5b325f, gen/run.py 8fdb538c (gate.log 170-175)
- 46fa607b, 352f55f1, a19d89ce, 4f10daa0, e9bacd1b, defb9e5c, cd4ca8ac (ruling N/M)
- the image localhost/emend-lo:25.2.3-2-deb13u6, 39 workbooks, 59/58/71, the outcome counts, the five-group counts
- all quoted texts and exit 2 (extract-texts.out); 118 untracked records (git-status.txt, 118 files on disk)
