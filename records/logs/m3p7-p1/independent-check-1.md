# Independent check 1, M3 part 7 P1

Written by the implementing session. Checked commit (the phase commit): bcef9c3433d25a9f6583db1547407b886d9fa057; HEAD when the check ran: bcef9c3433d25a9f6583db1547407b886d9fa057. The checker was a fresh general-purpose subagent given only the directory <phase>/check1/inputs (<phase> stands for <home>/.cache/emend-build/m3p7-p1), read-only; its inputs, by sha256 (sha256sum over each file, paths relative to that directory):

    25b4423dee915611bed371619920611ddc09ce6b86d017d7dc17164ebd0b4d0a  ./commit-271fd30.diff
    1139fdde28a2b2cf3216d70fc33ff477da0e38d9972f3bc815a60a288c02ea14  ./commit-34eea64.diff
    ccdcb3d835a2a070bd67cec6cbc0c0302fc8c2d2a5a6735abfdefd203173f3e1  ./commit-6387e9b.diff
    43d220ae66ca74d6278dad8001a85d142d8d87d980882d00805dd3f28c02301d  ./commit-bcef9c3.diff
    907c2f77c484f5fa4a4d939f99a370411dd0b8a8d511fbaf99c5f0f187d5a8c7  ./gate.log
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./gate.stderr
    6b6ee302f1a3185d1a57d38814e40509de9275fb089d29cdc3370db962894252  ./logged.out
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./logged.stderr
    24ed7c501369e2adec584b611b6d71f4ff1de224b89a41bfc157b350f9ffd5fd  ./messages.txt
    fa141d41de6a9a7e3653c9e3a7c48a3c2f5ceba4934df07f64f8d8b4d409f551  ./negative.log
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./negative.stderr
    c7f0aa9cfd122e85147b5a7f14f32f6cbcf2183375a66bfdbf932ebad8b5ad23  ./phase.diff
    10de30e9e77268565b3bf707481fd1881412a1b62c6ba45ec2f83028fbdd0a1d  ./report-draft.md
    07b6df06054a55bc40c1b63cfeb9cdc47fe3a8861f82121a75103580f0684e40  ./scan-pre-check.log
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./scan-pre-check.stderr
    cc08451dbf02d94eee665c1c8841da28be181ba6f6b430c412fb6160d5172543  ./self-test.log
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./self-test.stderr
    af4487f67633fb5174aa5f79c315370ef717cc141ff24d549b3796b4b4288a62  ./spec-0029-working.md
    c8cb5ac995bbdda91ba7dac1688d05214cd176d4ffc05c539a2f30b52e8ff371  ./specs-0001-at-phase.md
    df5320180b41d9ddcda9c6290a8909ca62045f89acec63d0f669f90c9300317a  ./superseded/negative.log
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./superseded/negative.stderr
    b55bf3c4076e9e003e74c3254113cb83e631b9e2c16cab694feac0f2fa402267  ./superseded/scan.log
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./superseded/scan.stderr
    04e5e11890ed76d83936c766b39e55a99d894be78e74f173b525ea1740941128  ./test.log
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./test.stderr

The answer below is the checker's final message, extracted from its transcript, verbatim (sha256 99ae37b817e6d347854872783dd596a9bc31ec71ae5910589d4675094b889a91); it holds no non-ASCII byte and no absolute machine path, so nothing in it is replaced.

---

I was given a CLAUDE.md, a memory index and git status in my prompt. I did not use any of them as evidence.

## Q1. Does the diff do what the spec says and nothing else?

**Yes with reservations.**

I read each commit's diff separately, then confirmed that phase.diff is exactly their union. These byte comparisons all matched:
- phase.diff 49-61 against commit-34eea64.diff
- phase.diff 62-154 against commit-bcef9c3.diff 1-93
- phase.diff 155-212 against commit-271fd30.diff
- phase.diff 1-48 and 213-678 against commit-6387e9b.diff

**6387e9b, the doc commit** (read in commit-6387e9b.diff):
- It touches only ROADMAP.md, specs/0028 and specs/0029.
- I rebuilt specs/0029 from the added lines of commit-6387e9b.diff (141-514). It hashes to sha256 76265c0b...375be5, which is the hash in messages.txt line 46, and its git blob id is 0fc4bfb, matching commit-6387e9b.diff line 137.
- `diff` between that rebuild and spec-0029-working.md shows only the inserted section "Planning rulings during P1".
- I could not hash ROADMAP and specs/0028 from partial hunks. For those I rely on gate.log 3093-3143, which compares them against planning's brief and against the message.
- Nothing else: confirmed from commit-6387e9b.diff.

**271fd30, the rules commit** (read in commit-271fd30.diff):
- It adds four bullets (lines 9-55) and removes nothing. The removal count is also printed in gate.log, "removed lines 0".
- Item 1 matches spec item 1 as read through ruling 4 (i): "first section after its header and input lines".
- Item 2 matches spec item 2 almost word for word.
- Item 3 matches item 3 as amended by ruling 1. It lists the same eleven forms by file and line, says "stand until a part touches them", and includes the line-205 sentence.
- Item 4 matches spec item 4.
- Nothing else: confirmed from commit-271fd30.diff.

**34eea64** (read in commit-34eea64.diff):
- The only change is line 158, from the unguarded form to `rm -rf "${d:?}"` (diff lines 9-10).
- Nothing else: confirmed from commit-34eea64.diff.

**bcef9c3** (read in commit-bcef9c3.diff):
- passthrough-run.sh:
  - The header comment changes (lines 46-48).
  - `ctmp` is made with `mktemp -d "$WORK/tmp.XXXXXX"` (line 64).
  - `TMPDIR` is exported only inside the child's subshell (line 68).
  - `leftover=yes` is set when the glob or `ctmp` held anything (lines 86-89).
  - The directory is removed as `rm -rf "${ctmp:?}"` (line 90).
  - These match ruling 2 and ruling 3, which keeps the column as yes/no.
- passthrough-record.py: `binaries_line` (lines 9-20) and its header insertion (lines 31-33) match ruling 4 (ii). Nothing else in the record script changes, as ruling 3 requires.
- Nothing else: confirmed from commit-bcef9c3.diff.

**Reservations:**
1. **Citation mismatch.** commit-bcef9c3.diff line 62 cites "specs/0023-scale.md P1 verification ruling 4", and report-draft.md repeats it. spec-0029-working.md ruling 2 says "specs/0023 ruling 4". specs/0023 is not among the inputs, so I cannot tell which is right.
2. **Line numbers stale at the phase commit.** The specs/0001 bullet gives "line numbers at 6387e9b" (commit-271fd30.diff line 43). bcef9c3's first hunk (@@ -6,8 +6,9) adds one line, so the passthrough-run.sh forms at 121 and 205 sit at 122 and 206 at the phase commit. The text is accurate as worded, but it is already out of date at P1's own end.
3. **Old run directories untested.** `binaries_line` indexes `r["passthrough_sha256"]` and `r["xlsx_diff_sha256"]` without a fallback (commit-bcef9c3.diff lines 14-15). A run.<label>.tsv without these columns would raise KeyError. test.log T1 uses only a fresh fixture run. No input shows that every existing run directory carries the columns.
4. **`$WORK` not shown in the diff.** The diff assumes `$WORK` is a `mktemp -d` under the caller's `TMPDIR`, but its definition is outside the hunks. The only evidence is indirect: test.log 2004-2005 shows the child's `TMPDIR` "two tmp. levels" under the runner's.

## Q2. Do the logs contain every line the spec requires for P1?

**Yes with reservations.**

- **Environment line:** gate.log lines 1-10 are the header and input lines. Line 12 is the environment section:
  - propagation `shared`, with controls for private and empty (14-17)
  - keepalive count 1, with controls for 0, 2 and error, plus three keep_count controls (18-26)
- **Prune:**
  - The ten names are listed with `du -sb` (gate.log 44-55). Their total is 9460754243, and I recomputed the sum.
  - `rm -r` exit 0 is logged for each name (56-66).
  - The log asserts every pruned name gone and every kept name present, with controls (67-70). The 17 kept names are on line 46.
  - The script uses `rm -r "${PREV_DIR:?}/${n:?}"` with no -f (gate.log 1566).
- **Self-test asserted:**
  - gate.log 3223-3246 checks that the commit is in range, that RESULT: PASS appears, that no gate file changed afterwards, the phase line, a 0-byte stderr and an exit file of 0.
  - self-test.log 107-108 reads "self-test: 50 cases, 0 unexpected, 0 errors" and "RESULT: PASS".
- **Test log, three tests:**
  - T1 (1938-1990): the header holds the binaries lines, with a control where the rows disagree.
  - T2 (1992-2024): leftover is no with nothing planted, yes with the `TMPDIR` plant, and yes with the run-directory plant.
  - T3 (2026-2048): an unset d gives exit 127 with the ":?" message; a set d is removed.
- **Negative log:** N1 (negative.log 1572-1597) shows the planted unset stopped: exit 1, the message once, no closing line. It has an unplanted control. N2 goes beyond what the spec asks.
- **Stderr:**
  - gate.log 3933-3936, test.log 2057-2060 and negative.log 1648-1651 each print their own stderr as 0 bytes.
  - The self-test's stderr is printed in gate.log 3244.
  - scan-pre-check.log's tail prints its own.
  - All *.stderr inputs are 0 bytes.

**Gaps:**
1. **"Untouched" is checked as existence only.** `kept_all` tests `-e` on each kept name (gate.log 1562). No log hashes or sizes the kept names before and after the prune. Ruling 4 says `out` is "empty", and no log checks that.
2. **`$WORK` is never printed by name.** Ruling 2 asks for the child's `TMPDIR` shown "under `$WORK`". test.log shows the child two levels under `<scratch>/w/rt`, for example line 2003, but never prints `$WORK` itself.
3. **The logged runner's stderr is printed by no log.** logged.stderr is 0 bytes, but only the file and logged.out show it.

## Q3. Does every number, hash and quoted string in report-draft.md appear in a primary?

**Yes with reservations.**

**Found in a primary:**
- The four full commit shas and bafd780's subject: gate.log 85-108 and messages.txt.
- af4487f6...: gate.log 76 and scan-pre-check.log 8.
- The specs/0023 citation: commit-bcef9c3.diff line 62. See Q1 reservation 1 for the mismatch with the spec.
- The self-test sha cc08451d...: gate.log 3223. Its times 00:28:40Z and 01:31:12Z and its summary strings: self-test.log 3 and 106-108.
- 9460754243, "10" and "17": gate.log 45-55.
- "exit 127" and "two tmp. levels": test.log 2005 and 2038.
- N1's "exit 1": negative.log 1580.
- records/logs/m3p6-p4/scan.log: negative.log 1600.
- RESULT: PASS and "structural checks failed: 0" for the pre-check: scan-pre-check.log tail.
- Every "... checks all as expected: yes" line: the three log tails.
- "(the run rows disagree)": commit-bcef9c3.diff line 19.
- The standing forms and the "(line numbers at 6387e9b)" note: commit-271fd30.diff 38-43 and spec ruling 1.
- 2026-10-08: commit-271fd30.diff.
- The dry-scan finding: superseded/scan.log 2895 ("change N") and 2909-2910 (undeclared: 2), and superseded/negative.log against negative.log 46-47 and 136-139 ("change VE").

**Not printed in any primary:**
1. The hashes report-draft.md gives for gate.log (907c2f77...), test.log (04e5e118...), negative.log (fa141d41...) and the scan pre-check (07b6df06...). No log, diff, message or spec prints them. I verified them by sha256sum over the input files, and all four match.
2. The header form `- Binaries (<label>): passthrough sha256 X; xlsx-diff sha256 Y` is a paraphrase with placeholders, not a verbatim string. The primary form is in commit-bcef9c3.diff line 16.
3. No input shows that "the first script and generator are kept under <phase>/superseded/". superseded/ holds only the two logs and their stderr. The report gives no hashes for any superseded file, while specs-0001-at-phase.md (around lines 202-207) asks that a superseded attempt be hashed and kept. superseded/negative.log hashes to df532018...300317a.
4. The claim that "no logged run was affected" has no timing evidence. The dry scan ended at 01:39:01 (superseded/scan.log) and the logged gate started at 01:39:52 (gate.log 109). No primary shows when the regenerated negative script was written relative to the start of the logged sequence, or that the fixed script had a dry rerun before its logged run at 01:43:38.

**Side observation:** superseded/scan.log prints unscrubbed absolute home paths 28 times (for example lines 54 and 2735-2746), from its `LOGDIR` input. That breaks specs/0001's log-privacy rule if it is ever committed. No committed-to-be log has the problem: the one hit in scan-pre-check.log, line 1673, is the split control fixture.
