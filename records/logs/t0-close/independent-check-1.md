# T0 close-out independent check 1

- Checked commit: e0a5175ebcd3525f50831491150a1c8a162289f9 (planning doc
  updates; the pushed main; the close-out adds no code)
- HEAD when the check ran: 00c64bb877502d2d4afcb3cc9e7d43ed69e24943
  (push.log record commit, local only)
- Checker: a fresh general-purpose subagent (not a fork), given only the
  inputs below
- Inputs, with sha256:
  - specs/0001-bootstrap.md
    06f0cc115ed38cd71b52474e4cee2251c1a5593e23d6232d54962ae5e6cf6d48
  - the diff, `git diff 8e7e665..e0a5175`, saved as <scratch>/close-diff.txt
    498d4217084c68c5f03d9b539f57c9bd04efca2763e05abaf9c67b0df45e1030
  - draft report 1, <scratch>/close-report-draft.md
    c9a3b5b4788990be6aa86d2d393565e0dfca84d9dc4b2769e94c39c3120e191c
  - records/logs/t0-close/push.log
    8990f6fc5bddedaf22825dbc5d5a812eef7fad363779010508f0636795560c5f

---
I had the harness CLAUDE.md in my context. I did not use it as evidence. My evidence was the spec, the diff, the draft report and push.log. On those four files I also ran byte and pattern scans with `/usr/bin/grep` under `LC_ALL=C`. One scan counted hits for this machine's hostname without printing it.

**1. Does the diff do what the close-out section says, and nothing else?**

Yes, as far as the primaries can show.
- **What changed:** the diff touches three files, all documents: `ROADMAP.md` (+6 -2), `specs/0001-bootstrap.md` (+79 -4) and the new `specs/0002-gate-corpus-additions.md` (+96 -0). There is no code, script, config or log.
- **Content:** it adds the P6c rulings, the close-out section itself, the T2 P0 spec, the ROADMAP rows and backlog item, and the status line updates. All of this matches the P6c rulings in the spec.
- **"Unedited":** step 1 requires the updates to be committed unedited. A diff cannot prove that. It rests only on the implementing session's word.
- **Pattern hits:** diff line 24 holds a home-directory path, but only as an unchanged context line in `ROADMAP.md`, so this diff did not add it. The "password" hits (lines 101 and 200) are the phrase "password-protected". The "session" hits (lines 17 and 127) are ordinary spec wording.

**2. Does push.log contain every required line and follow the log rules?**

Yes on both.

Required lines:
- **Denylist variable unset:** line 160, plus the part 2 guard at line 191.
- **`core.hooksPath` empty:** lines 161-163 show exit 1 and 0 bytes.
- **Hook executable and identical to the installer output:** lines 164-169 show `-rwxr-xr-x`, the executable check and equal sha256 values. One caveat: the script builds the "installer output" by pulling the heredoc out of `scripts/install-hooks.sh` with awk (script line 68). It does not run the installer. The hashes match, so this cannot have let a push through here.
- **Full gate:** lines 170-188 show all eight PASS lines in order, exit 0, empty stderr and 0 control bytes.
- **Push with the hook's PASS:** line 195 is `git push origin main`. Lines 197-198 show the hook's range and `PASS privacy`, line 200 shows 9 commits, 95 blobs and 0 findings, and lines 202-205 show the push landed with exit 0.
- **ls-remote equals local main:** lines 209-217, both e0a5175.
- **Still PRIVATE:** lines 218-224.
- **Each check evaluated once:** the script's `check`/`yn` functions (script lines 31-32) print each result once, and the log has no contradictory summaries.
- **No authentication output:** the push stderr has only git's normal lines, and the ls-remote and gh stderr are empty.

Log rules:
- **First line:** names both the commit and HEAD. The part 2 header (line 190) names HEAD again.
- **Bytes:** 0 control bytes beyond tab, LF and CR; 0 non-ASCII; 0 U+2014.
- **Leak scans:** 0 hits for session words or links, temp or home paths, Windows or `.claude` path forms, token-like strings, email-like strings, `uname -a` style output, and the hostname.
- **Accepted items:** line 165 shows the user and group name `dev`, which the P7 rulings accept.
- **Cosmetic only:** lines 187 and 225 print a double colon when the status is empty.

**3. Does every number and quoted string in the draft report appear in a primary?**

These match a primary:
- 226 lines and the sha256; I recomputed both.
- 3 files, 181 insertions, 6 deletions, and the per-file counts (6/2, 79/4, 96/0); I recounted them from the diff.
- e0a5175, af4b355, 9 commits and the 3c5d2d79 prefix.
- The quoted lines for the denylist variable, `-rwxr-xr-x`, `PASS privacy`, the 9/95/0 privacy line, `af4b355..e0a5175  main -> main`, `{"visibility":"PRIVATE"}`, `still PRIVATE: yes` and both "checks all as expected: yes" lines.
- The fact line `ANTHROPIC_API_KEY set in this shell: no`.
- "7-entry" and "11 patterns", which are in the spec.

These do not match a primary:
- **"25 check lines, all yes" (line 56) contradicts the log.** push.log has 24 lines ending ": yes": 22 individual checks plus the two "part N checks all as expected" summaries. It has one line ending ": no". So 25 is the count of yes and no lines together. The report marks this as session output, but it is still wrong against the primary.
- **Planning's "83 insertions and 6 deletions" (line 23) is in no primary,** and neither is the "83 = 79 + 4" explanation. The sentence is not marked session output; nearby sentences are.
- **The hook range quote (line 50) is not verbatim.** The log line is a two-dot range with full hashes. The report shortens both hashes with "...", so it reads like a three-dot range.
- **The record commit 00c64bb and "ahead of origin/main by 1" (lines 29-30)** are not in any primary. They are marked session output.
- **The pre-commit scans of the three doc files and of the log (lines 26-28 and 62-64)** are marked session output. My own scan of the log agrees on control bytes, non-ASCII, U+2014, temp paths, session, hostname and token-like strings. I did not have the denylist or pattern files, so I could not check those parts.
- **The maintainer's authorisation sentence (lines 35-36)** is marked session output and appears in no primary.

Preconditions that are only partly met in the primaries:
- **Planning's word** is backed only by the spec's "Close-out push READY" status and the P6c ruling. The report's reference to planning's message is session output.
- **Maintainer authorisation** and **"unedited"** are shown by session output only.

Overstatement: line 3 says the push is "complete and verified in the log" and then says "the push waits for your verification". The push has already happened, and the verification is the session's own claim.

The independent check section is still "(to be added)", which is expected for a draft.

**Scope filter (private data or a push let through):** no new finding. push.log and the diff introduce no private data, and the log shows every pre-push guard held.

**Defects, most severe first:**
1. The report's "25 check lines, all yes" contradicts push.log, which has 24 yes lines (22 checks plus 2 summaries) and 1 no fact line.
2. Line 23 cites planning's "83 insertions" figure and a guessed explanation that are in no primary and not marked session output.
3. Line 50 misquotes the hook's range: shortened hashes with "..." make a two-dot range look like a three-dot range.
4. Line 3 is ambiguous: it calls the push verified and then says the push waits for verification.
5. Planning's word, the maintainer's authorisation and "unedited" are evidenced only by session output, not by a primary. This is not a rule breach, but planning should confirm them itself.
