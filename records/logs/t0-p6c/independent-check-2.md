# T0 P6c independent check 2

- Checked commit: d8ca1bda47b0ec799687aedf25bc80969779956f (P6c phase commit)
- HEAD when the check ran: 0cfa2dd67a04786e3975e45ff3e9c31dfdb0742d
  (check 1 record commit, local only)
- Checker: a fresh general-purpose subagent (not a fork), given only the
  inputs below
- Inputs, with sha256:
  - specs/0001-bootstrap.md
    3945cfaba0607a1e4610847fd8e377e7c3031a35759ad2cddc42994a34f978bc
  - the phase diff, `git diff 302b1b2..d8ca1bd`, saved as <scratch>/p6c-diff.txt
    5693cd2486f5bc58a1807ed07c65d6e59cd76f0acc2788068db08c478ea53722
  - draft report 2, <scratch>/p6c-report-draft2.md
    f4af40e8bf5ce00ad882e62c0954a0ac0821c550f2f32b72110db30ddfec97f5
  - records/logs/t0-p6c/gate.log
    b08ad192794ef36ae0e02c66c9c58e6340f4b261243c47b70dbf836434b17eb5
  - records/logs/t0-p6c/privacy.log
    5a518dc09c563d2260023875ddf2d269a8468651ba4f43d0f18e2885f7330d56
  - records/logs/t0-p6c/hook.log
    67ff628b68af0224435795bd4fc4af5f76015883c8486170878af3798c80e5f9
  - records/logs/t0-p6c/negative.log
    20c3254616926137404c18d66b1029330982d32304b17998a388337fe212e377
  - records/logs/t0-p6c/scan.log
    e499bb9108cd4c94866afba90d093467423ea95d32e7b4b50d526d5604311e2f
  - records/logs/t0-p6c/independent-check-1.md
    3e069d72c9d3e43ae9429f3cf7906e29b8f6bb66f88af35512eb1515fad49a99
  - scripts/privacy.sh (for the cited line numbers only)
    b6a863ffc489d2726b458336fba61083c42d1487c2e8b23aa34008736c36ec6e

---
Harness context: my context includes a CLAUDE.md and a user memory index from the harness. I did not use either as evidence. The only commands I ran were read-only byte, line and hash counts over the input files. I did not run git, and I read no other repository file.

## 1. Does the diff do what the P6c spec says, and nothing else?

**Yes.** Each spec bullet maps to the diff:
- **Office suffixes:** `OFFICE_RE` ends in `($|[#~.])` in privacy.sh (diff 145) and step 7 (diff 85). `.gitignore` has 45 new lines, three forms for each of 15 extensions (diff 14-58).
- **Eight new types:** present in all three places.
- **Archive signatures:** `signature()` checks the 50 4b 03 04, 1f 8b and 37 7a bc af 27 1c prefixes (diff 200-208). The result is cached per blob, counted per tree entry against the fixtures index (diff 246-251), and printed as a count only (diff 259-260).
- **Full commit header:** every line before the first empty line goes through the pattern, denylist and email checks (diff 222-226). Continuation lines of gpgsig or mergetag start with a space, so they stay in the header.
- **Emails in path names:** `email_lines` takes `-z`, and `scan_file` no longer skips it for NUL-separated input (diff 165-187). A path that hits is added to the private index, so its content findings print as `<private path>`.
- **Patterns 10 and 11:** exactly as the spec gives them (diff 113-114). The count stays 11.
- **Self-test and step 7 control:** these changes support the above. The rest of the diff is the planning doc commit (ROADMAP and spec). Nothing is outside the spec.

**Printing:** every new FAIL line prints only a count, a header line number or a tree entry number. No new line prints matched text or a path.

**Fail-open:**
- **od failure (privacy.sh 255, 332, 61):** `error` runs inside the command substitution, so it cannot exit 2. But `error()` prints to stdout, so the message becomes a non-empty signature and the blob is counted as an archive. This fails closed, with a misleading line. Under tests/fixtures/ the signature is ignored, but those paths are exempt anyway.
- **email_lines pipeline:** it checks only grep's exit status. A failure of the new `tr` stage, or of awk or sort, would drop findings silently. This is very unlikely, and the same pattern was there before P6c.

**Can a new check silently miss what the spec targets?** Not for the spec's literal forms: negatives (a) to (g) and h1 to h6 exercise each one. Two low gaps are within the filter:
- **The self-test now plants only the new forms of patterns 10 and 11** (doubled backslashes with USERS in capitals; backslash separators). The single-backslash, mixed-case and forward-slash forms are proven only by scan.log's own controls (scan.log 285-286). privacy.sh does not check them at run time. A later edit that narrows either pattern would pass the count check and the self-test.
- **Password-protected OOXML workbooks** are, by my own format knowledge (not shown in any primary), OLE2 containers and not zip. A renamed one passes the path and archive checks. This is the same class as draft finding 2, which names only xls, xlt and xla.

**Already accepted:** zip-compressed content under tests/fixtures/ is not readable by the content scan. Spec lines 440-442 already accept this ("Committed fixtures are fictional or generated"), so it is not a new finding.

## 2. Do the logs contain every required line, and follow the log rules?

**Required lines: yes.**
- **gate.log:** eight PASS lines in order (96-103), exit 0, stderr 0 bytes, control bytes 0, overall yes (116).
- **privacy.log:** 57 commits (120), PASS privacy (127), summary (129), overall yes (138).
- **hook.log:** installer exit 0 (119), hook sha equals the heredoc sha (166-167), executable (168), core.hooksPath unset (116), identity matches (175).
- **negative.log:**
  - Spec cases (a) to (g) are all present: (a) check-ignore and status per name; (b) b1 and b2; (c) c1 to c4; (d) 635/639; (e) 651/656; (f) 665-667; (g) g-a, g-d1 to g-d3, g-g1 to g-g3.
  - The overall line is yes (853).
  - I count 28 captured-output result lines, all 0/0/0, and no result line reads UNEXPECTED.
  - One small point: P6b (d) wants the hook itself to exit 1. The log shows git push exiting non-zero, the hook's FAIL line, and 0 remote refs. The push was rejected, so this is not a leak.
- **scan.log:** script printed; `/usr/bin/grep` under `LC_ALL=C`; exit 2 handled as an error; a positive control for every form (257-314); 6 files, 2 messages, 4 P6c logs and 3 P7 logs, each equal to HEAD; 0 hits; RESULT: PASS.

**Log rules: met.**
- **First lines:** four logs name d8ca1bd and HEAD. scan.log names the range ending at d8ca1bd, and HEAD.
- **My counts in all five logs:** 0 control bytes, 0 non-ASCII bytes, 0 U+2014.
- **No private strings:** 0 hostname or FQDN hits, 0 Claude temp-directory paths, 0 /mnt paths, 0 session ids. The single home-style path is scan.log 141, a fictional control string. hook.log 174 is the allowed identity address, and hook.log 124 shows the `ls -l` owner, which the P7 rulings accept.
- **Each check evaluated once:** the helpers print one result per check. The fixed strings (gate.log 3 and 5, "EMEND_PRIVATE_DENYLIST set: no") are true by construction, not evaluated twice.

**Observations, no leak:**
- hook.log prints the pre-fix script copy (hook.log 64). Draft 2 records this.
- The start times (gate 12:22:47, negative 12:22:51, hook 12:23:03) show the runs overlapped with the cold gate build. The main repo's refs and status did not change (negative.log 851-852).
- scan.log and the check files are not covered by any committed P6c scan. The pre-push hook will scan them, and the denylist holds the hostname (spec 434-435). Planning should still scan them.

## 3. Does every number and quoted string in draft report 2 appear in a primary?

**Verified against primaries:**
- **Diff and log hashes:**
  - 6 files, 179 insertions, 33 deletions: I1 line 86, and my recount agrees.
  - Log hash prefixes: scan.log 421-448 and I1 header line 24.
  - Full hashes of a371cff and d8ca1bd: scan.log 318-326. 9a26ba0: I1 line 4.
- **Gate, privacy and hook:**
  - 135 files: G 93.
  - 57 commits and summary: P 120, 129.
  - P line 121 (af4b355 and 5 commits ahead).
  - P lines 64-66 (the named fix).
  - Hook hash prefix 3c5d2d79: H 166.
- **negative.log:**
  - 13 fixture strings (N 8) and 28 outputs (my count).
  - Every cited N line: 516, 533, 550, 564, 572, 576, 586, 590, 601, 609, 613, 621, 625, 635, 639, 651, 665-667, 676-677, 686-688, 718, 732, 740, 747, 758, 769, 782-827, 843, 853.
  - The case (d) header field list, all quoted FAIL formats and the ERROR strings.
- **scan.log:** mode 600, 7 entries, the scan counts.
- **Diff details:** the self-test counts (office 2, archives 3, header line 4), the step 7 control names, and the 15 extensions.
- **privacy.sh 61, 255 and 332:** they say what the report says.

**Not in a primary, and marked as session output or untested:**
- "committed without editing"
- I1's record-commit hash
- the first privacy.log run's false "no"
- the `r.xlsx:Zone.Identifier` gitignore test
- the `b.numbers/Index.zip` gitignore test
- the od-failure test
- the Zone.Identifier content URL
- the OLE2 prefix d0 cf 11 e0
- the xz, bzip2, zstd, rar and tar signatures
- the split-zip prefix 50 4b 07 08

**Not in a primary, and not marked:**
- "That first run's output is not committed" (departure 6): a session claim.
- "Index.zip blob fails the archive check": attributed to I1, but I1 took the fact that Index.zip is a zip from draft 1. It is format knowledge.
- "which still happens before any push" (departure 3): a reasonable inference from the hook, but not evidenced.

**Overstatements:**
- **Line 3, "independent checks are complete":** check 2 is still "(to be added)".
- **Line 7, "Draft 2 corrects the points check 1 raised":** several points are recorded, not corrected (the od failure, Zone.Identifier, the hook.log copy). The detailed list at lines 180-188 says this accurately.
- **Case (e), "no part of the name in the output (N 651)":** the absence check is at N 656, and it tests only the name prefix plus the fixture address.

**Does draft 2 handle check 1 fairly? Yes, all five I1 defects are handled.**
- 1: recorded, with the stdout question settled from privacy.sh 61.
- 2: now finding 1.
- 3: 29 corrected to 28.
- 4: finding 3 withdrawn; findings 2 and 3 marked untested; "(L-P)" replaced by P 121; "is done" removed.
- 5: now departures 6 and 8.

I1's plain-renamed CSV, TSV and FODS gap is credited in finding 2. The summary ("diff within the spec and every required log line present") matches I1's answers.

## Remaining defects, by severity

1. **Low, possible leak (spec gap, new):** a renamed password-protected OOXML workbook is OLE2, not zip, so it passes the path and archive checks. This is my own knowledge, untested. It extends draft finding 2.
2. **Low, could let a later regression through:** the privacy.sh self-test plants only the new forms of patterns 10 and 11. A narrowing edit that drops the single-separator or mixed-case forms would pass the count check and the self-test. Only scan.log's one-off controls cover those forms.
3. **Low, fails closed, already recorded:** an od failure in `signature()` gives a misleading archives FAIL instead of an ERROR.
4. **Report accuracy, no leak:**
   - "independent checks are complete" before check 2 exists;
   - "corrects" should be "corrects or records";
   - case (e) cites N 651 for the absence check, which is at N 656;
   - the Numbers Index.zip premise is unmarked format knowledge now attributed to I1;
   - "first run's output is not committed" is an unmarked session claim.
5. **Log consistency, no leak, recorded:** hook.log shows the pre-fix script copy; the gate, negative and hook runs overlapped in time with no effect on refs or status.
