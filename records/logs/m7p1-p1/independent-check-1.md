Check 1, M7 part 1 P1 (specs/0044 "P1: the reruns, the keep, the list, the stage", rulings A to P)

## Independent check 1: M7 part 1 P1

**Inputs.** I checked the primaries under `<phase>/check1/`. Their hashes are as given: spec-0044-working.md is 7f767447... with 572 lines, and the logs hash as the draft's table says. I also read the repository with git, the 118 record files, `<phase>/scan-pre/scan.log` (9c7a3458..., equal to the copy) and the byte sizes of `<phase>/dry/d1` and `<phase>/dry/d2`.

**Departures from my instructions.** There are two, both in the session's scratch directory or read-only:
- I wrote one temporary file in my own scratch directory. It holds the 118 "installed" lines from run.log in checksum form, so that `sha256sum -c` could check the records against them.
- I ran `du -sb` on the two dry directories. It reads file sizes only.

I wrote nothing in the repository or the phase directory, ran no script, cargo, podman or network command, and read nothing under the corpus.

### Q1. Does the diff do what the spec says and nothing else?

**Verdict: yes.**

- **The patch.** diff.patch is byte-equal to `git diff 3f68f3b 82c07fb`. The numstat is ROADMAP.md 74/7, specs/0043-measurement-and-close.md 39/0 and specs/0044-excel-check.md 377/0. These are the three files the spec's P1 names, and nothing else changes.
- **specs/0043.** It adds "Planning verification of the close-out (2026-10-24)" with rulings W and X.
- **ROADMAP.md.** It adds the Milestone 7 section, with part 1's P1 row "IN PROGRESS (doc commit pending)". It also carries Milestone 6's close-out edits: the heading "DONE and PUSHED 2026-10-24", the part 6 Close row DONE, and T2 P5 and P6 "scoped into Milestone 7 part 3".
- **specs/0044.** The committed spec is the approved 377-line text. The working file adds the rulings and removes no line; `git diff 82c07fb` shows only additions.
- **The commit.** 82c07fb has tree 60b98e065b66... and parent 3f68f3bf0685.... 31841f74dbe6... has the same tree and the same parent, and its message is the dropped paragraph form. This matches ruling L.
- **The message.** It matches ruling L's form exactly:
  - the briefed subject;
  - three "<path> sha256 <hash>" lines, each equal to `git show 82c07fb:<path> | sha256sum` (5503d966..., 45b841ae..., d14a8fe1...);
  - "No log prints a count for this commit; the diff alone carries what it changes.";
  - the Co-Authored-By line only, with no session line.

  It is ASCII; a grep found no non-ASCII byte, and gate.log says "message lines 10, non-ASCII bytes 0" with the diff-alone sentence found. The gate also checks each blob against its message hash.
- **No code commit.** gate.log:401 reads "no file under crates/ is changed since M6 part 6 P4's phase commit". The gate's section at line 8805 asserts nothing under crates/, tools/ or scripts/ is in the range.
- **Working tree.** `git status` shows only ` M specs/0044-excel-check.md`, `?? records/logs/m7p1-p1/` and the 118 `records/m7p1-*` files.

### Q2. Do the logs contain every line the spec requires?

**Verdict: yes.** One wording point is under (c).

**(a) The run directories against the flag list and the two exclusions.**
- run.log section 2 prints the usage text's flags, numbered 1 to 59, with "the usage text's flags ending in -sample: 59; distinct: 59".
- It checks them against the USAGE constant at the phase commit, with a control.
- It prints "--new-sample is in the usage text, and is excluded ...: yes" and "--new-sample is not among the flags run: yes".
- It prints "--drop-calc-chain in the usage text: 1 line(s); it ends in no -sample ..." and "--drop-calc-chain is in the usage text and is not among the flags run: yes".
- It then lists the 58 flags run, in order. I compared the two lists: the 58 are the 59 less `--new-sample`, in the same order.
- The rerun sections `### 1.` to `### 58.` each name `<runs>/2026-10-25-82c07fb-<name>`.
- run.log:5790 to 5849 lists the 58 directories, each "there", followed by "run directories there: 58".
- Each of the 58 sections has these lines:
  - the run command, with `--timeout 300 --lo-timeout 600`, the pinned image and `--tool-arg --<name>-sample`;
  - outcome counts from summary.host.tsv: 57 directories read "ok 27", and printarea reads "error 6 ok 21", with 0 other outcomes;
  - "run.host.tsv holds one run row naming the phase commit and the flag alone: yes";
  - a "compared with records/..." line naming the record used. No chosen record is a `-scale-<tier>-` record, and printarea's is `records/m4p1-p3-printarea-2026-10-10-669fa91.md`, as ruling M says.
- The gate's release usage section repeats the 59-flag list and both exclusions.

**(b) The record pairs against the directories.**
- run.log section 6 prints "record files written in <work>/records: 118 (of them the samples' 116); expected 118".
- It then prints "the samples' record files are two per run directory: yes" and "the samples' record files are 116 and the run directories 58 ...: yes".
- It lists 118 "installed records/... sha256 ..., N bytes" lines, ending "every record file was installed, byte for byte: yes".
- In records/ I counted 118 files matching `m7p1-*-2026-10-25-82c07fb.*`: 59 .md, which is the 58 sample names plus open. All 118 match the logged sha256s (`sha256sum -c`).
- The 58 names are the 58 directory names. scan-pre.log reads "records scanned: 118" and "files in the records directory whose name ends in -82c07fb.md or -82c07fb.txt: 118".

**(c) Every FINDING line printed and listed (specs/0043 ruling R).**
- The run log prints one line beginning FINDING, at run.log:5898, ruling K's keep growth. It counts it at line 6206: "FINDING lines printed above: 1".
- gate.log, negative.log and scan-pre.log contain the word 0 times.
- The full count in run.log is 31 matches:
  - 14 in the printed script's source (lines 102 to 2331);
  - 13 information lines of ruling J, each ending "not a FINDING";
  - line 5788, "... FINDING lines so far: 0";
  - line 5898, the FINDING;
  - lines 6205 and 6206, the ruling R section header and its count.
- logged-driver.out's grep lists the 14 source lines and the 13 information lines, plus lines 5788, 5898 and 6206.
- Wording point: the report lists the one FINDING line and explains the rest, rather than listing every grep hit. That meets ruling R as the report frames it, a "line beginning FINDING". If planning reads ruling R as "every line the grep finds", the report would need the full list.

**(d) The report's stage and zip lines.**
- run.log:80 defines `<stage>` as `<corpus>/work/excel-check-m7-2026-10-25`, with `<stage>.zip` beside it.
- Section 8 prints the list's path by placeholder, "rows: 6018; by source: corpus 1560; fixture 2229; input 2229", the sha256 46cb9453..., 1496245 bytes and mode 600.
- Section 9 prints "files staged: 6020", the three per-source counts against the rows, the opener's sha256 equal to the repository's, and "staged files differing from their list rows: 0".
- It then prints the zip at `<stage>.zip` with sha256 d32483dd..., 1075262198 bytes, mode 600, "entries 6020; ... stored: 6020; testzip: ok".
- The report prints all of these by placeholder.

**Other required lines are present:**
- **Keep and open record.** The keep test run has EMEND_KEEP_DIR set on that command alone. The keep holds 2241 inputs and 2241 outputs. The open check exits 0 and prints its totals line. The comparison with m6p6-open is printed.
- **The list.** The command is printed with placeholders. The stderr note reads "note: 6 error row(s) left out of 2026-10-25-82c07fb-printarea". The counts by label sum to the rows. No row is printed.
- **Negative cases.**
  - (a) is L1, "error: the run directory has no summary.host.tsv".
  - (b) is L2, "error: a --run-plain directory is under another corpus root".
  - (c) is F1, the run script's `formula_run_ok`. It is extracted from lines 2033 to 2036, its extract's sha256 d51a7bba... equals those lines, and it refuses another sample's directory. That is ruling H's form.
  - (d) is S1, a byte flipped in one staged file on a copy and refused. S2 is the extra appended-byte case.
  - L3, F2 and the controls LC, FC and SC are also present.
- **Endings and exits.** All three logs end "checks all as expected: yes". logged-driver.out gives exit 0 and outer stderr 0 bytes for each. scan-pre.log ends "RESULT: PASS" with "total hits: 0" and "structural checks failed: 0".

A minor inconsistency in the spec, not in the logs: its section header at negative.log:6051 says "specs/0044 P1 case (d)" for the extraction of both functions. Case (c) sits under that same header. This is labelling only.

### Q3. Does every number and quoted string in the report appear in a primary?

**Verdict: yes, with exceptions.** All of the following check out against a primary:
- PLANNING_SPEC 7f767447... and its 572 lines;
- the list's rows, counts, sha256, bytes and mode;
- the stage's 6020 files and 0 differing;
- the zip's 1075262198 bytes, sha256 d32483dd..., 6020 entries, stored, testzip ok;
- the FINDING line, quoted exactly at run.log:5898;
- the "reruns done 58 of 58 ..." line, at run.log:5788;
- the commit hashes, trees, parents and numstat;
- the four log sha256s;
- the prune sizes 11237987271 and 2144181986 (gate.log:240 and 289);
- the nine PASS lines and the test sets 539, 713, 576, 755, 758, 820 and 1201 on host and i686;
- emend-copy 35814fb2... at 3954088 bytes and xlsx-diff 1445523d... at 897296 bytes;
- the 59 and 58 flag counts, the 58 directories, the 118 records and the 2241 keep pairs;
- the scan sha256 9c7a3458... with 118, "total hits: 0", "structural checks failed: 0" and "RESULT: PASS";
- the d1 PLANNING_SPEC 0ca0630c... and d1's two failing controls (dry-d1-run.log:2627 and 2628);
- the logged start time 2026-10-25T17:27:20Z (logged-driver.out).

The exceptions, in order of weight:

1. **The FINDING section's last sentence is incomplete.** "The other matches of the word in run.log are the printed script's source and the 13 information lines of ruling J" leaves out three output lines that contain the word: 5788 (which the report quotes just before), 6205 (the section header) and 6206 ("FINDING lines printed above: 1"). The sentence should name them.
2. **"2223 lines to 2230" is printed in no primary.** It is consistent with the script listings: d1 runs from line 101 to the library header at 2325, and the logged run from 101 to 2333. It is also consistent with run-TA.diff's +7 lines. The old script's sha256 c1ad9291... appears only in the session-written README. The new one, 689dc7f9..., is printed in negative.log:12. Both the counts and the old hash are the session's statement.
3. **"Dry run d2 met ruling P's conditions" rests partly on the README.** dry-d2-run.log ends "yes", and its only FINDING is the ruling K line with d1's totals. dry-d2-scan.log ends RESULT: FAIL with exactly ruling O's four "no" lines: change AK, change LK, "8 (expected 118)", and the home-directory line. But d2's gate and negative logs and d2's exits are not among the primaries. "Gate and negative pass as in d1" is the session's statement here, from the README. d1's gate and negative exit 0 rests on ruling N and the README.
4. **The dry directories' sizes appear in no log.** `<phase>/dry/d1` (160334494 bytes) and `<phase>/dry/d2` (159893566 bytes) equal my `du -sb` readings now. The report labels these as the session's statements, which is correct.
5. **The Milestone 2 stage was "not read, moved or removed": the logs show only part of this.** run.log:90 and 91 prove the run's stage is excel-check-m7-2026-10-25 and not the Milestone 2 stage's name, with a control. No log shows that nothing read, moved or removed the old stage, so that part is the session's statement.
6. **The run's start time differs by one second.** The report gives 17:34:17Z, from logged-driver.out. run.log's own header (line 65) says "run at 2026-10-25T17:34:18Z". 23:43:56Z is in both.
7. **The following are the session's statements only, and the report mostly says so:**
   - "Nothing pushed". It is consistent with origin/main resolving to 42ddf3b48ff0 locally and with no remote branch containing 82c07fb.
   - What the future record commit will hold.
   - "ANTHROPIC_API_KEY was not set in any step". It is supported for every log given: the gate, the negative, the run, the scan pre-check and the d1, d2 and scan logs. It is not shown for d1's gate and negative.
   - The "outer standard error 0 bytes" for the scan pre-check. logged-driver.out covers only the gate, the negative and the run. The scan log prints only its own script's stderr as 0 bytes.
8. **"Every log ends 'checks all as expected: yes'" is slightly loose.** It holds for the three logs. README.md, also listed in the logs table, has no such line.

No quoted string in the report is contradicted by a primary. No workbook content appears in the report or in this answer.
