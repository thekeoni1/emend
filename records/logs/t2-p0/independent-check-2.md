# T2 P0 independent check 2

- Checked commit: d97b9ef2ec5e3deb45e5dc68ed63dd453f7a0f0b (T2 P0 phase commit)
- HEAD when the check ran: 9a726b959b1b86e547f62fabef1e8213f4abed51
  (check 1 record commit, local only)
- Checker: a fresh general-purpose subagent (not a fork), given only the
  inputs below
- Inputs, with sha256:
  - specs/0002-gate-corpus-additions.md
    9fac44a85e155ea248137974806bda68b00e4803864fa26feac4ef56b6af1c3a
  - specs/0001-bootstrap.md
    ebb2545a358b6c38b14dd00c177a3c47e6f36a332c79dfa14fcd4f380b231a37
  - the phase diff, `git diff 41e2919..d97b9ef`, saved as <scratch>/t2p0-diff.txt
    547f4d9cb4ff1dc71361700cda3597e387c8b17a898901b3bbae14ec48fbe599
  - draft report 2, <scratch>/t2p0-report-draft2.md
    ba2b85585d80f787e88451c9de7c2e85e148b079995201de338f4a9fef6cfb94
  - records/logs/t2-p0/gate.log
    43326ed211bf5ea961742156ba221278832754f979621deff2a1ca4899bf6aba
  - records/logs/t2-p0/privacy.log
    06b2a4e4c24e25750c759b057b375b4b15d59bfd945c31452c7ee20181d9c69f
  - records/logs/t2-p0/negative.log
    9cfccddffee31269791055699e08cdc129e08c737e058796f4b8d4cd6cb2dfe1
  - records/logs/t2-p0/scan.log
    f97bddb51b1aec0f9b7dce6a2a8ebc89cd43c4204b8927f0410b858ae4786ec4
  - records/logs/t2-p0/independent-check-1.md
    302258b4c90e2e639815011fc8666621c623107e99a7f46fe1e21c95877ed7e3
  - scripts/privacy.sh (for the cited line numbers only)
    3ecb98800165c0a7605d04b1b650e567258d676eb393ce8f9e56a0042230e465

---
**T2 P0 independent check 2 (on draft report 2)**

My context holds a CLAUDE.md from the harness, and I did not use it as evidence. My evidence was the seven inputs named in the request: specs/0002; the named sections of specs/0001; the phase diff; draft report 2; the four logs; check 1's file; and only lines 161-164, 177-179 and 362-371 of scripts/privacy.sh. I ran no git command and changed nothing. My only commands were reads, byte counts and pattern counts over those same inputs. They printed counts and line numbers, never matched text.

## 1. Does the diff do what specs/0002 says, and nothing else?

Yes. Every change in specs/0002 is present. I found no fail-open path and nothing that prints a private path or matched text.

- **Change 1, Zone.Identifier files.**
  - In privacy.sh, the pattern matches in any case, anchored at the end of the name (diff 186). It is counted over the path list with no tests/fixtures/ exclusion, and only the count is printed (diff 249-251).
  - The office suffix class includes `:` in both scripts (diff 106, 183).
  - check.sh step 7 has a count-only check with a positive control; the control expects two of its four names to match (diff 124-128, 138-142).
  - .gitignore widens the form to `[.:]*` for all 15 extensions. The Zone.Identifier rule comes after the tests/fixtures/ exception, so it applies there too (diff 77).
  - A content finding inside a Zone.Identifier file prints a placeholder. It is checked after the private, corpus and office placeholders, so a private path still takes precedence (diff 280-281).
- **Change 2, signatures.**
  - The hex values at diff 236-241 match the spec bytes, including tar at offset 257 for 5 bytes (diff 228-230).
  - The finding label is the spec's "archive or compound-file signature", with no path (diff 290).
- **Change 3, self-test.**
  - The older forms of patterns 10 and 11 are planted in w.txt and each is required under its own number (diff 312-313, 337-340).
  - There is one planted blob per new signature, plus the two Zone.Identifier files (diff 321-329).
  - The exact counts required are office 3, Zone.Identifier 2 and archives 10 (diff 349-352). Because the counts are exact, removing any one detection is caught; N f1 to f4 show this.
- **Change 4, exit statuses.**
  - All four email pipeline statuses are saved at once and checked (diff 205-208).
  - `signature()` returns 1 when a read fails, and its caller runs in the main shell with `|| error` (diff 226-230, 271). A read failure therefore cannot become a signature or a finding. The earlier version called `error` inside a command substitution, where it could not stop the script.
- **Fail-open cases I considered.**
  - If `wc` printed something that is not a number, the size test would be false and tar detection would be skipped silently. The self-test plants a tar blob and requires the exact archive count of 10 on every run, so that would fail closed.
  - Replacing `tr` with `false` fails closed whichever stage trips first.
- **Nothing else in the diff.** The ROADMAP.md and specs/0001 hunks belong to the doc commit, and the report marks "unedited" as session output. The other additions are the reported departures and comment updates.

## 2. Do the logs contain every required line and follow the log rules?

Yes.

- **gate.log:** line 1 names the commit and HEAD. Lines 96-103 are the eight PASS lines in order, then stderr 0 bytes, exit 0, control bytes 0, and a final yes.
- **privacy.log:** line 1 names the commit and HEAD. It scans 65 of the 65 commits on main: `PASS privacy`, 0 findings.
- **negative.log:** line 1 names the commit and HEAD. Every case in specs/0002 is present:
  - (a) a1, a2 and a3;
  - (b) b1 to b8;
  - (c) c1 and c2;
  - (d) d1 and d2;
  - (e) the three lock and backup files, e-c1 to e-c4, and e-d. These cover P6c negatives (a), (c) and (d).
  - All 30 captured outputs show 0 fixture strings, 0 control bytes and 0 non-ASCII bytes. I counted the run_out calls in the printed script and got 30.
- **scan.log:** line 1 names the diff range and HEAD. It covers the 5 files, 2 messages, 3 logs and both files under records/logs/t0-close/, with controls for every form and `RESULT: PASS`.
- **Byte and text checks, my own count over all four logs, check 1's file and draft 2:**
  - 0 control bytes, 0 non-ASCII bytes, 0 em dashes and 0 hostname hits.
  - No temporary session paths and no Windows paths.
  - The only path-shaped or address-shaped strings are in scan.log's printed script: pattern definitions and split control fixtures (lines 40, 130, 132, 141, 142), and allowlisted or control addresses (lines 22, 58, 67, 146).
  - gate.log line 5 and the script lines that set the denylist location show the unexpanded home variable, never an expanded path.
- **Evaluate-once rule:** every verdict comes from a helper that evaluates once and prints that result.
- **Cosmetic:** the empty-status lines print a double colon (gate.log 115, privacy.log 134, negative.log 846), which was accepted before.

## 3. Is every number and quoted string in draft 2 in a primary?

I checked every number and quoted string, and nearly all hold:

- **Counts and hashes:** 5 files, 130 insertions and 49 deletions (my own count of the diff agrees); the four log hash prefixes (S 410, 419, 428 and I1 line 24); both full commit hashes (S 318-326); 384231f (I1 line 4); origin/main at e0a5175 with 4 commits ahead (P 118).
- **Quoted strings from the diff:** ZONE_RE, the OFFICE_RE class, all signature hex values, and the FAIL, ERROR and self-test strings.
- **Self-test counts:** 3, 2 and 10.
- **check.sh:** the g.xlsx control and "two of four".
- **negative.log line numbers:** every one cited (473, 487, 488, 500, 508-515, the seven b lines, 608, 621, 630, 639, 648, 660, 670-678, 679, 695, 710, 725, 737, 748, 758, 769, 781, 794, 803, 812, 821, 837, 847).
- **Other log facts:** c1 and c2's narrowed patterns (N 292, 294), and 10 fixture strings (N 8).
- **scan.log:** mode 600, 7 entries, 0 with whitespace, and the coverage counts.
- **privacy.sh line references:** 161-164 is grep, sed and tr with only grep's status checked; 177-179 is grep and cut, the same; 362-371 is the placeholder order ending in the path-list fallback. All are accurate.

Not supported by a primary:

| # | Claim | Problem | Marked as session output? |
|---|---|---|---|
| a | Departure 1: the probe result | No primary | Yes |
| b | Departure 5: uutils 0.8.0 and od's skip-past-end message | No primary (N 670-678 is consistent with that tool's style) | Yes |
| c | Departure 9: six runs, the permission error, 126, the re-test | Only the script comment about `mv` at N 164-168 supports it | Yes |
| d | 9324e64 "committed without editing" | No primary | Yes |
| e | Finding 1: a sed, tr or cut failure would drop "pattern, denylist, corpus, office or Zone.Identifier hits" | The cited lines show the function bodies, not their callers. Only the Zone.Identifier caller is in the diff (249), and diff 246 suggests office paths use their own grep status check. The list of affected checks is partly unsupported. It overstates the exposure rather than hiding any, so it is safe. | No |
| f | Finding 2: "git-quoted path" | The fallback reads an array named QUOTED, but how that array is filled is in no primary. Wording only. | No |
| g | "The P6c-form fakes are unchanged" | Supported only by their absence from the diff. N c1 and c2 show the original fakes are not the older form, which is what the spec's negative needs. That they hold both doubled backslashes and capital USERS is not visible. | No |
| h | "(N 679)" attached to "with no FAIL line" | N 679 is the ERROR line; the no-FAIL check is N 684 | No |

Overstatement: none material. Item e is the closest.

**Treatment of check 1.** Draft 2 corrects or records all six of check 1's defects. It cites finding 1 to privacy.sh lines, adds finding 2, marks departures 9 and 5 as session output, fixes the range to 670 to 678, records the d1 and d2 test strength in (d), and rewords "S lists its message". It also replaces the "raw NUL-separated" wording. Its summary of check 1 is fair. It leaves out check 1's minor log notes, which check 1 itself did not count as defects.

## Remaining defects, ranked by severity

1. **Low (report; affects how planning weighs finding 1).** Finding 1's list of affected checks goes beyond the cited lines. Only the Zone.Identifier caller is shown to use the unchecked pipeline. Planning should confirm the callers before sizing the fix. None of this weakens the gate.
2. **Very low (report).** "(N 679)" should be N 684 for the absence of a FAIL line.
3. **Very low (report wording).** "git-quoted path" and "The P6c-form fakes are unchanged" rest on code that is not in a primary.

I found no defect in the diff or the logs that could leak private data or let a push through.
