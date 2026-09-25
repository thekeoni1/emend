# Independent check 2, M2 part 1 P1

Checked commit: ade3c49be9071cfab25d42ed94289d0050217c61
HEAD at the check: ade3c49be9071cfab25d42ed94289d0050217c61
Checker: a fresh general-purpose subagent (not a fork), given only the files below in one scratch directory. It is the phase's second and last check.
Inputs, each with its sha256 (as given to the checker):
- 0016-new-workbook-and-sheets.md: e4d03eccde8282c305d8b715233044d4f61d000e698a85d063dd37c419fefa91
- 0001-bootstrap.md: 84c149d39875db7cd6ba6eff3e185ff4e2be56e474ee9ba151983cf2243ea31f
- code-commit-ade3c49.diff: 8875af4960ae7da5f6ae3c1cd7d16dbbb3a22db7c4f1409aacbc4f2499a7a66c
- doc-commit-1450dbd.diff: 76af02a0cf5561fb350f33b3a96088cc14f15af085277eb847186c36b1b0c7b5
- combined.diff: f03b9ea17b08f3f066196646040749c2c9ba5167178f17c17271b5f9cef46c1f
- draft-report.md: adeb33f6c297e925c26a76dab93d63879ffdf0a26464c69b605525898c16a4e7
- gate.log: 07b528c9771568030b7926a045a473f2fe17f7412dcb911b83bd519989a6d6dc
- test.log: 1db1cac369e6da4e577d6d9886459b1ad0c94f2ec1bb79a0eaeed978497cc846
- negative.log: 9d11a0ddb036f6248ceb30666a34926ded0a207e0f0fd9f0c0dca582c6812368
- run.log: 33b6b1dcf39ed4329dbaaaaf95c9d3a8898509c8598e0be3eba4517a44cd47e0
- scan-pre-check.log: f870ede87ea2db0a6be9ac0f5f281c9a28af1d61ac0d81c8794138fea075d9de
- independent-check-1.md: c272a6e28d5ef58db97346d55f81c6bc597ec6f472a7d7c795288515c8005fb6
- m2p1-open-2026-09-25-ade3c49.md: a7767765a14cdbd0d34529dadb15abc159425f383e8ad0ac1508409f06bcfb02
- m2p1-open-2026-09-25-ade3c49.txt: 70684a4ed62ca32dbcb3ad23222fcf31b685c96e631812e2dd302ed5bec88f06
- m2p1-p1-2026-09-25-ade3c49.md: 917260f4e3bb3f496a9404a1441318a0fbd88a9da0ef919711b970eebf5abce8
- m2p1-p1-2026-09-25-ade3c49.txt: c98ec386ecf4e90968f18f12b2a4b1e91792f5070d2b62a325ccaff498300167
The answer is ASCII and holds no absolute machine path; it is reproduced verbatim below (sha256 of the answer text 1cca1f6d2bec25094a5792c068e10e4d807067b2e67780932f3c4a209386b58d, 11934 bytes).

---

I was given a CLAUDE.md in my context. I have not used it as evidence.

Everything below comes from the files in the check directory. I ran no build or test. I hashed the inputs and compared diff sections byte for byte. To check that run.log's script really was unchanged, I extracted the printed scripts from run.log and negative.log into two temporary files one level above <scratch>/p1, hashed them and deleted them.

**Input hashes I computed:**
- negative.log: 9d11a0ddb036f6248ceb30666a34926ded0a207e0f0fd9f0c0dca582c6812368 (attempt 3).
- gate.log, test.log, run.log, scan-pre-check.log, combined.diff, the spec and the four record files have the same hashes check 1's header gives (independent-check-1.md lines 7 to 21). So only negative.log changed since check 1.
- Check 1's answer, from its line 27 to the end, hashes to 5c5bc628... at 10685 bytes, as its header line 23 states.

## Question 1: does the diff do what the spec says for P1, and nothing else?

Yes, apart from G3 and G10 below.

**Doc commit (I read doc-commit-1450dbd.diff).**
- It changes three files: ROADMAP.md, specs/0012-styles.md and a new specs/0016 (lines 14, 78, 98).
- The 0012 hunk is the styles-part precondition (lines 309 to 316 of the hunk, diff lines 84 to 96). The ROADMAP hunk is the part 1 section.
- gate.log lines 1017 to 1056 check each file against planning's hashes and confirm there is no fourth file.
- The spec's working-tree copy has hash e4d03ecc (gate.log 1302 to 1305) and differs only by the uncommitted P1 rulings.
- Nothing else: confirmed.

**Code commit (I read code-commit-ade3c49.diff).** It touches four files.
- `scripts/edit-open-check.sh`: the heading at diff line 151 is exactly the spec text (spec lines 434 to 437).
- `scripts/excel-check-list.py`:
  - umask 077 is set (line 285) and IndexError is caught (line 314).
  - `--run-plain` can be repeated (line 280) and is refused under another corpus root (lines 301 to 303). Its rows are labelled with the directory's basename and have empty references (lines 256, 301).
  - A size different from `out_bytes` is refused for both flags, in ruling 1's words (lines 251 to 252).
  - An input row follows each accepted fixture row, carrying the record's input bytes and hash (lines 272 to 273). The docstring covers all of it (lines 162 to 216).
- `scripts/excel-check-open.ps1` (new):
  - Paths come from `$PSScriptRoot` (lines 366 to 368, 405).
  - It opens with `Open(path, 0, $true)`, Excel visible and alerts on (lines 399 to 400, 411).
  - Values are read through `Worksheets.Item(1)` and `Value2` as ruling 5 says (lines 384 to 396, 422), and a failed read records `read-error` (lines 423 to 426).
  - It writes the eight columns as ASCII (lines 401, 436).
- `records/excel-check-template.md`:
  - The automated procedure (lines 75 to 95) and the by-hand fallback (lines 87 to 92).
  - Label, input and seen columns (lines 120, 127).
  - The rule that an input Excel refuses is allowed and an output refused on an accepted input is a finding (lines 136 to 141).
- The commit message ends with the Co-Authored-By line only (diff line 27; gate.log line 1010).
- Nothing else, apart from the small additions check 1 already named as F5 and F6: confirmed.

**Combined diff (I read combined.diff).**
- Lines 65 to 472 are byte-identical to code diff lines 30 to the end.
- Lines 1 to 64 plus 473 to the end are byte-identical to doc diff lines 14 to the end. I compared both with cmp.
- There are seven files and no crate, tool, lock, manifest or gate file (gate.log 1113 to 1126, 1138 to 1143).
- Nothing else: confirmed.

## Question 2: do the logs contain every line the spec requires?

Yes, with gaps G1 and G2.

**F4 re-examined (negative.log, attempt 3):**
- The rerun header (line 30) names attempts 1 and 2. Lines 32 to 35 print the hashes of both logs and both scripts, including d726b588 and 43544b26.
- The list script under test runs only from the git-show copy `$LS` (script lines 367 to 371 and 425). The copies' hashes are printed (line 945) and checked against the phase commit (line 946).
- The base's list script also runs from a git-show copy (lines 1111 to 1115).
- Family P reads the .ps1 copy (line 1122).
- Residual points are in G1 and G8.

**run.log rerun header:**
- Line 28 names attempts 1 and 2. Lines 29 to 46 hash their logs, scripts, records, lists and zips.
- Lines 47 to 49 hash and remove the superseded run directory.
- I confirmed "the script unchanged since attempt 2": the script printed in run.log lines 72 to 1275, with the indent removed, hashes to 4a5db12c, the attempt 2 script's hash (line 40).

**The P1 test list:**

| What the spec asks for | negative.log lines |
|---|---|
| Earlier negative cases rerun (L1 to L5) | 1015 to 1060 |
| `--run-plain` with no summary (L6) | 1062 to 1069 |
| Size mismatch (L7 on `--run-plain`, L7r on `--run`), per ruling 1 | 1071 to 1086 |
| Another corpus root (L8) | 1088 to 1095 |
| Keep pair with a refused input, no row (L0) | 1008 to 1009 |
| .ps1 ASCII and no-absolute-path grep, a control per form | 1121 to 1141 |
| Mode 600, with the base's 644 as control (L0 and L10) | 988 to 989, 1116 to 1118 |
| Short-row IndexError, with the base's traceback as control (L9) | 1097 to 1115 |

**The run log:**
- Release build, and the tool unchanged since part 6 P3: lines 1480 to 1495.
- Part 5 P2's script extracted and checked (e5380f13), and cmp.py (ea176c10): lines 1497 to 1512.
- The style run:
  - 27 ok, lo and lo_in ok on 27, entries 816, 54 and 34: lines 1546 to 1560.
  - Totals 992, 1046, 27 and 27, each asserted: lines 1622 to 1630.
  - Plants 17 of 17 plus the reorder plant: lines 1632 to 1655.
- Record run with `--allow-single`, and the ten checks with their control: lines 1663 to 1796.
- The keep:
  - Both targets hold 298 files, the same bytes (lines 1904 to 1911).
  - Equal to part 8 P2's record row for row (lines 1913 to 1917).
- The open check:
  - Totals: lines 2072 and 2197.
  - The heading as ruled, with controls: lines 2081 and 2199 to 2201.
  - No verdict moved: lines 2203 to 2207.
  - Acceptance, with 0 outputs refused on an accepted input: lines 2209 to 2215.
- The list:
  - Counts line: line 2256.
  - 330 of 330 hashes: lines 2285 to 2291.
  - References 27 of 27 against the derivation, with a control: lines 2326 to 2330.
- The stage and zip: 332 files and sha 99b3376d (lines 2346 to 2361).
- Status after the run, lines printed: lines 2368 to 2376.

**Every log:**
- Lines 1 to 3 name the commit the log is about and HEAD.
- The phase commit is compared with HEAD, with a control: gate.log 35 to 40, test.log 51 to 54, negative.log 49 to 51, run.log 22 to 24.
- Each log runs the shape greps over its own script first (lines 5 to 15).
- SERIALIZED lines; the four runs do not overlap in time.
- The status checks print the lines: gate.log 1298 to 1308, test.log 1933 to 1939, negative.log 1143 to 1152.

**Missing, and not looked at by check 1:** G2, the pre-check scan's rerun.

## Question 3: does every number, hash and quoted string in draft-report.md appear in a primary?

Nearly all do.

Found in primaries:
- 1450dbd, d4ab367, d45f4932, afb9fee6, a7cf94e1 and e4d03ecc (gate.log).
- 523 on both targets (test.log line 1929).
- The L cases, as listed in Question 2.
- 27/27, 54, 34, 992, 1046, 27, 27, e5380f13, ea176c10, 17 of 17, and ten checks (run.log).
- "Descriptor flag alone": the record's .md line 45 shows flagged 34 and descriptor 34, every other flag 0.
- 149; 138/11; 139/10; findings 0.
- 54, 19, 15, 27, 138, 138, 11 and 330.
- bcda16d9 with mode 600.
- 99b3376d, 33010705 bytes, 332 entries, testzip ok.
- The four record hashes (run.log 2235 to 2246).
- 3e4c5b5e and c94a8152 (run.log 30, 38).
- d726b588 and 43544b26 (negative.log 32, 34).

The exceptions are G4 to G7 below.

**Were check 1's findings stated fairly?**
- F4 is stated (report line 42), though G4 applies.
- F5, F6 and F7 are listed as not acted on (lines 51 to 54).
- F1, F2, F3 and F8 were fixed in the text (lines 22, 46, 39, 50), but the report never names them as findings or says they were fixed (G5).

## Findings

**G1 (gap).** Does attempt 3 meet the scratch-copy rule? specs/0001 "How this task runs" (0001 lines 90 to 94) defines a scratch copy as "a git worktree or a clone outside the repo". Attempt 3 copies two files with git show, and the script still starts in the main tree (negative.log script line 123) and loads scripts/counting.sh from it (line 125). Root line 38 says "nothing in it is run", but its functions are called throughout. I cannot check specs/0005's rule, which the header cites, because it is not among my inputs. No planning ruling on this form is cited, and report line 42 does not put it to planning. Planning should rule on whether per-file git-show copies meet the rule.

**G2 (gap).** Report line 50 says the pre-check scan ran twice and the first run counted 2 non-ASCII bytes. scan-pre-check.log has no rerun header: no rerun line is printed, and only the script text at line 370 mentions one. No primary gives the first run's hash or says it was not kept (0001, the superseded-attempt rule). In addition, scan-pre-check.log line 1283 hashes negative.log attempt 2. The final scan must hash attempt 3 (9d11a0dd...) and should name both earlier scan runs.

**G3 (gap).** F6's Close defect ships in the staged zip. Report line 53 proposes fixing it in P2. But the stage for Excel check 2 already carries the .ps1 at b6f39860 (run.log 2351), and that script writes its results only at the end (code diff lines 430, 435 to 436). A Close that throws on the maintainer's machine would lose every row. The report does not say whether the stage will be rebuilt before Excel check 2.

**G4 (nit).** Report line 42 says check 1's F4 cited "specs/0005's scratch-copy rule". F4 cited specs/0001 "How this task runs" (independent-check-1.md line 122). specs/0005 appears only in negative.log's own header (line 30).

**G5 (nit).** The report does not name F1, F2, F3 and F8 as findings or say how each was fixed. The fixes are only visible by comparing texts: line 22 (F1), line 46 (F2), line 39 (F3), line 50 (F8).

**G6 (nit).** Three phrases have no primary among my inputs:
- Report line 39, "the control's second half, a .txt one summary row short, is unchanged". run.log line 28 names only what changed.
- Report line 41, "every check passed" for negative attempt 1. negative.log line 30 says that only of attempt 2.
- "2 bytes" (line 41) now appears only in check 1's answer (line 113), which cites attempt 2's log.

**G7 (nit).** Report line 10's hash 9d11a0dd... matches negative.log's bytes, which I hashed. No primary prints it yet.

**G8 (nit).** negative.log prints each command as "scripts/excel-check-list.py ..." (lines 983, 1016, 1029, 1038, 1047, 1054, 1063, 1073, 1080, 1089, 1104). The script actually runs the scratch copy `$LS` (script line 425), so the printed command does not match the invocation.

**G9 (nit).** Report line 13 says "only", but negative.log lines 838 to 843 and script line 373 read the main-tree copies to compare hashes. Only reads, never runs, but the word overstates it.

**G10 (nit).** The template (code diff lines 80 to 81) tells the maintainer to record a repair-prompt row as `repair` by hand. The script itself writes `pass` or `fail` only, so the results file will not show a repair unless the maintainer notes it.

**G11 (nit).** negative.log script line 98 says "Nothing else changed" since attempt 2. The attempt 2 script is not among my inputs (35308 bytes, against 36527 bytes printed now), so I cannot verify it. By contrast, run.log's "unchanged" claim is confirmed by hash.

Verdict: the diffs match P1 and rulings 1 to 11, the logs hold every required line, and F4's rerun behaves as described. Planning should rule on three gaps (G1 the scratch-copy form, G2 the pre-check scan's rerun record, G3 the staged .ps1). G4 to G11 are nits in the report and logs.
