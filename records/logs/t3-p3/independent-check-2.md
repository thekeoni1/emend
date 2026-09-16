# T3 P3 independent check 2

- Checked commits: 17ec824d227887d0b4f21e4f104ec54e97a6f0de (phase commit 1)
  and 2623b70d92dbc1c33d15783749297435178244e6 (phase commit 2, the fixes
  independent check 1 forced)
- HEAD when the check ran: bd8c581578de25a85b7e08f5ea1caae576d145db (the record
  commit with the three logs rerun at phase commit 2, local only)
- This is the second and last check of the phase (specs/0001-bootstrap.md,
  "How this task runs": at most two checks per phase)
- Checker: a fresh general-purpose subagent (not a fork), given only the inputs
  below plus planning's rulings of 2026-09-15 quoted in its prompt (the spec
  text is corrected after this report, so the rulings govern), asked to answer
  in plain ASCII, and told to run read-only commands only, never to run the
  scripts, cargo or podman, and never to read the private corpus directory
- Inputs, with sha256:
  - specs/0006-passthrough.md
    7d4b6482eb66a8ac0aff58f00d36429fa41acd0e7be01a503a00ab2cee15f985
  - specs/0001-bootstrap.md
    5b7ed9aa84e89ec895f3cbd4065ef38c5d2d1cf2a1d7c2a0b00be4c7608e6f4b
  - phase diff 1, `git diff 869796b..17ec824`, saved as <scratch>/t3p3-diff.txt
    1ca18687cab26a5cea858d785120ee792a5362782988aaf8eb025efdf67377cc
  - phase diff 2, `git diff 8a7c862..2623b70`, saved as <scratch>/t3p3-diff2.txt
    dcf99b454572212793dcc75bec5d69040fac447c8fee4764d744e123c74fe48a
  - records/logs/t3-p3/independent-check-1.md
    85dc71c2bf6475668d542f8da91b5f6242df6e4275a884b58e4fed024af8e9c1
  - draft report 2, <scratch>/t3p3-report-draft2.md
    3b544719cb8864c559cfb7992845b9987f69f89836241fb9de4fc4562b77c870
  - records/logs/t3-p3/gate.log
    aad7302a35e4552b2ad842ba6a345562b684ba4c3f28b2213d36b3cbd5be7212
  - records/logs/t3-p3/negative.log
    41cbd2f91845a37e3e530f4f017f5489b2a1424b7071e7b96793d5bb757831a1
  - records/logs/t3-p3/scan.log
    e275d08fbf17366cc674795f4d52ccc4c7552b0c2c3f38d4676b2c3537f8ed20
  - records/libreoffice-2026-09-15.md
    ef2177180704b1a7ecfc5bdd215410498d2c9b868055f52e35042131735dceeb
- Answer: the checker's final message, extracted verbatim from its transcript
  with python, without printing the transcript, and concatenated to this header
  by the same script. It is 17920 bytes, 78 lines, sha256
  edd5487b7466591aacd9b30b23f6d42c3810dc7a1d1edd4ecab8f3a592822757, and holds
  0 non-ASCII bytes and no HTML escapes, so no character replacement was
  needed. Unlike check 1, no privacy-pattern replacement was needed either:
  the checker was told to name privacy pattern 10's bare shape rather than
  write it, and it did.
- Privacy-scanned before the commit with the same command forms as scan.log
  (the patterns from scripts/privacy-patterns.txt, the hostname, the private
  denylist, addresses outside the allowlist, a Claude-Session line, U+2014,
  control bytes): every count 0, and 0 lines with a scratch path of any form.
  The controls fired on a file built for them (session output).

---
**Note on my inputs.** The harness also put CLAUDE.md, a memory index, a git status snapshot, attribution text and the user's email address into my context. I say so here and used none of it as evidence. Everything below cites the spec, `<scratch>/t3p3-diff.txt`, `<scratch>/t3p3-diff2.txt`, `<scratch>/t3p3-report-draft2.md`, the four files under `records/logs/t3-p3/`, `records/libreoffice-2026-09-15.md`, or git objects I read read-only.

## 1. Does the work do what the spec and the rulings say, and nothing else?

**Scope of diff 2.** It touches two files only, `scripts/passthrough-run.sh` and `scripts/passthrough-record.py` (`<scratch>/t3p3-diff2.txt`; gate.log:236-238 shows 2 files, 74 insertions, 23 deletions). Nothing outside the rulings is added. I agree with check 1 on diff 1's scope and on the "-" semantics, and I do not repeat those findings.

**Gap 4 is closed, and closed correctly.** The piped form is gone. Each grep now runs on its own with its exit code read, an exit of 2 or more ends the check as `error`, and only exit 0 from the second grep produces the `fail` (`<scratch>/t3p3-diff2.txt`, the new lines at `passthrough-run.sh` 49-64). I checked the logic exhaustively against the three reachable states of the second grep: exit 0 (a disallowed line) gives fail, exit 1 (none) falls through, exit 2 or more gives error. Empty `flagged.txt` gives exit 1 from `grep -v -x -F`, which is the pass path, so the common case is right. `flagged.txt` lives inside `$work`, which is removed per iteration (`passthrough-run.sh` line 102 at 2623b70), so no stale file survives. No new hole here.

**The error/fail split is right in outline but not complete, and the boundary differs from the ruling in two places.**

- Correctly `error`: the conversion directory, podman exits 125 to 127, either grep erroring, python3 failing to run, and an unparsable count.
- Still `fail` for something that is not LibreOffice refusing: a timeout. `timeout --signal=KILL` returns 124 (or a 137-style status if the container is killed by the OOM killer); neither is in 125 to 127, so it lands on `the conversion exited %s` and returns 1. The spec's first condition does say "exits 0 within the timeout", so reading a timeout as `fail` is defensible, but a 300-second cold start on a `large` workbook, or a host OOM kill, will read in the P4 record as an archive finding. The report does not name it.
- Still `fail` for an environment fault: if `/out` is unwritable, LibreOffice writes an error line and exits 0 (M1 shows every conversion exits 0), so the run reads `fail`, not `error`.
- `error` for something that may be the archive: the python3 helper opens both zips in one `try`, so a *reference workbook* that python's `zipfile` cannot open, or that has no `xl/workbook.xml`, produces `error ...` and returns 2 (`passthrough-run.sh` 70-84 at 2623b70). The ruling's list of error causes is "conversion directory, podman failing to start, python3, unreadable ODS" - an unreadable *input* is not in it, and before diff 2 that case was `fail`. Over 27 real archives in P4 this is the plausible one: a zip the Rust tool copies fine but python's `zipfile` rejects will now read `error` and be filed under "the check could not run". The report does not name this widening.
- A container process that legitimately exits 125, 126 or 127 would read as `error`. Unlikely for soffice; worth one line in the record.

**A defect in the record script that diff 2 left behind.** `LO_VALUES` gained `"error"` (line 30 at 2623b70) but the refusal message did not: `passthrough-record.py` line 163 still raises `"summary.%s.tsv has a %s value other than ok, fail or -"`. The message now misstates the accepted set, and negative.log:355 pins that exact stale string as C10's expected stderr, so the log locks the wrong wording in. Nothing in the report mentions it.

**A smaller beyond-spec effect of the new skip condition.** The skip test became `row["lo"] in ("ok", "-") and row["lo_in"] in ("ok", "-")`, so an `error` row is now listed both in the table headed "Files not ok, with a diff exit other than 0, with a leftover temporary file, or failing the LibreOffice open check" and again in the new "Files whose open check could not run" section. An `error` row is not "failing", and the heading was not widened. Harmless, unnamed.

**Leak paths.** I found none, and diff 2 adds none. `flagged.txt` holds container stderr lines but stays in the run subdirectory; the reason files hold only this project's wording; nothing from the container reaches stdout or the error channel (the child still gets `3>&-`). C9 at negative.log:504-508 is a real check with a working control in both directions. The image name is regex-constrained and C12 (negative.log:540-542) now exercises the refusal.

**Where I disagree with check 1.** Check 1's item "the reason files are not cleared at the top of each iteration" is now fixed (`passthrough-run.sh` 103-105). Its reading of the piped grep was right and the fix is right. Its gap 8 (circularity) is only partly closed - see question 2. I do not share its concern about `EXPECTED` not being extended; that remains coherent.

## 2. Do the logs contain every line the spec and the rulings require?

**The one hard miss: scan.log has no rerun header.** `records/logs/t3-p3/scan.log` lines 1-3 are `about:`, `phase:` and `temporary files live in <scratch>` - there is no `rerun:` line, and `grep -c '^rerun:'` returns 1 for gate.log, 1 for negative.log and 0 for scan.log. This scan.log replaces the scan.log committed at record commit 9be61d6, so specs/0001-bootstrap.md ("A rerun that replaces logs committed at an earlier phase commit names that record commit in its header (T1 P3 gap 3)") requires it to name 9be61d6. The script supports it (it prints `RERUN_NOTE` when set); the variable was simply not set for this run. The draft report's heading "Logs (all rerun at 2623b70; each header names record commit 9be61d6)" is therefore false as written.

**Does C11 prove what it claims?** Yes, and it is well constructed. The wrapper directory holds only `python3` (negative.log:359-366), so C8's podman wrapper cannot contaminate it - the fault check 1 caused the session to find. Two assertions pin the branch: `lo_is ... healthy error error` (negative.log:535) and the reason file being exactly `cannot count sheets and tables` (negative.log:536-537), which is reachable only from the `counts=$(... python3 ...) ||` branch. `lo` is only computed when `outcome = ok`, so `lo=error` also tells us the copy was made. It cannot pass for another reason. Its limits: it exercises exactly one of the five error branches. The podman 125-127 branch, the two grep-error branches and the directory branch have no case, and no case shows an `error` row surviving into the record.

**That last point weakens the report's C11 paragraph.** C10 runs the record script over `run-lo` only (negative.log:332), where no row is `error`. The printed outcome table (negative.log:519-522) is a `grep -A3`, so it shows the heading, the column row and the separator but not the counts row, and the "Files whose open check could not run" section's contents are never printed. So the primaries support "the columns exist" (negative.log:523) and "the section heading exists" (negative.log:524), but nothing in any log shows an `error` row being counted or listed. The record script was never run over `run-error`.

**Do M1 to M4 make the record's claims primaries?** Largely yes, and this is the strongest part of the rerun.

- M1 (negative.log:546-551) runs the conversion directly on five fixtures and prints each exit status, `0 0 0 0 0`, with the loop assertion in one named function `all_zero_exits`. "Every conversion exits 0, including the three that load nothing" now has a primary, closing check 1's gap 3 for the five measured fixtures.
- M2 and M3 (negative.log:552-553) give the record's "cut central directory" case its primary: cutdir wrote no ODS and exactly one `Error: source file could not be loaded` line.
- M4 (negative.log:554-555) gives "three table elements, the third empty" its primary: `3 0`.
- Not covered: the record's second ghost-finding sentence, that the nopart workbook "converted with the full table count", is only inferable from C5 passing the count comparison (negative.log:486), not measured directly. And the pins table itself (base digest, package versions, `soffice --version`) still has no primary in this repository, as check 1 said.

**Is the gate's image inspection genuinely non-circular?** Partly, and more than before. It reads the image, not the record: `podman image inspect --format '{{json .Config.Entrypoint}}'` returns `["/usr/local/bin/lo-entry.sh"]` (gate.log:281-282), and a container runs `cat` over that script, which is then grepped for the flags and hashed (gate.log:283-289). Three reservations:

- gate.log:284 prints `sha256 2c09931b...`, which is indeed the `tools/image/lo-entry.sh` build-input hash in `records/libreoffice-2026-09-15.md`, but **no check line compares them**. The equality is left to the reader; a mismatch would not have failed the gate. The report calls this "the strongest line", which overstates what the script asserts.
- `entry_has_flags` greps each flag independently anywhere in the file (gate.log:67-73), so it proves the tokens are present, not that they sit on the soffice invocation, and `'"$@"'` likewise.
- gate.log:270-273 (the record "states" its own strings) remains circular exactly as check 1 said; the inline caveat at gate.log:272 is honest about it.

**Everything else in "Rules for every phase" holds.** HEAD and the phase commit on line 1 of all three logs; the full script printed in each (gate.log:11-199, negative.log:8-437, scan.log:5-241); `/usr/bin/grep` with `LC_ALL=C` throughout, with check 1's one bare-grep site now routed through `grep_q` (negative.log:91, 343-344); the scan's control battery covers every one of the eleven patterns in both directions plus the "bare names must not match" negative (scan.log:253-310); 0 hits, 0 errors, 0 structural failures, RESULT: PASS (scan.log:514-518); zero control bytes, and the byte-count check now has the positive control check 1 asked for (negative.log:558-560). The negative script's five-versus-six comment is fixed (negative.log:16-17). The scratch corpus is under a `mktemp -d` and `--corpus` always names it.

Two leftovers from check 1's gap 10 that are neither fixed nor recorded: `both_listed` still omits `-a` (negative.log:124), and the negative script still `cd`s to the work tree and runs `cargo build` and the repo scripts in the **main working tree** (negative.log:62, 158, 253), against specs/0001's "never in the main working tree". The report's "Recorded, not fixed" list does not mention the second one at all, so a live check-1 finding has silently dropped out of the report.

**The committed independent-check-1.md.** The header names the checked commit, the HEAD, every input with its sha256, the replacement's line and columns (74; 344 to 360), the original written in pieces, the replacement token, and both sha256 values, and says the pre-replacement hash is the one listed above. I verified from the file itself: the extracted answer is 92 lines, hashes to `49f6c6d9da88661ff72507f9c64b6dc9c63fa17f3368f81b2746d40223198d78` exactly as the header claims for the post-replacement text, holds 0 non-ASCII bytes, and matches privacy pattern 10's regex 0 times; scan.log:384-392 independently shows all eight scan forms at count 0 for this file, at sha256 `85dc71c2...`, which equals the file on disk. The change is reversible from the description. One nit: the header's "22219 bytes" is the pre-replacement count and the file now extracts 22220 bytes (the 17-character original became an 18-character token); the header gives no post-replacement byte count, so a reader checking the number against the file sees a one-byte discrepancy with nothing saying why.

## 3. Does every number and quoted string in the draft report appear in a primary?

I checked every citation in `<scratch>/t3p3-report-draft2.md`. These hold: the 62/13 insertion counts (gate.log:211-213); `fc4f3bdd...`/17310 and `61b34ae1...`/14926 with mode 100755 (gate.log:252-253); the parse and compile lines (gate.log:258, 260); `541945cc...` cited as it is now (gate.log:264-266); the image id (gate.log:268-269); the record's stated strings and the no-corpus-path check (gate.log:270-275); the whole image-inspection block (gate.log:277-289); no gate or Cargo file changed (gate.log:302); the eight PASS lines, exit 0, 0 stderr bytes (gate.log:305-321); empty status and the closing line (gate.log:329, 331); every negative.log citation for the fixtures (451-457), C1 to C6 (475-487), C7 (492), C8a and C8b (497, 500), C9 (504-508), C10 (516-530), C11 (535-537), C12 (541-542), M1 to M4 (546-555), the new controls (558-560) and the closing line (563); and every scan.log citation (250, 310, 438, 493-512, 514-518). The three log line counts (331, 563, 518) are right, the quoted refusals `error: summary.host.tsv has a lo value other than ok, fail or -` and `error: --libreoffice must name an image` are verbatim from negative.log:529 and 540, and bd8c581's "339 insertions, 56 deletions" matches `git show --stat`. The draft holds 0 non-ASCII bytes, 0 U+2014, no machine path and no privacy-pattern match, so check 1's item about a literal temporary path is gone.

Wrong or unsupported:

1. **"Logs (all rerun at 2623b70; each header names record commit 9be61d6)"** is contradicted by scan.log:1-3, which has no rerun line. This is the report's one false statement of fact.
2. **"The record script counts `lo error` and `lo_in error` and lists those files in a section of their own (negative.log:523-524)"** overstates its primary. Lines 523-524 prove the column headings and the section heading exist; no log shows a row with `error` being counted or listed, because the record script was never run over the C11 run directory.
3. **"the gate inspects the image itself ... The strongest line is gate.log:284: that script hashes to 2c09931b..., which is exactly the build-input hash the corpus session recorded"** - the hash is printed at gate.log:284 and the same hash appears in `records/libreoffice-2026-09-15.md`, so the claim is true, but no check line asserts the equality; the word "demonstrably" leans on a comparison the script does not make.
4. **"5 commit messages, each ending with the Co-Authored-By line (scan.log:491)"** cites the count line; the per-message assertions are at scan.log:442, 452, 462, 472 and 482. Trivial, and the same mis-citation check 1 raised.
5. **Departure 4, "The committed file has zero pattern matches (session output)"** could now cite a primary: scan.log:384-392 scans `records/logs/t3-p3/independent-check-1.md` at its committed sha256 with all eight forms at count 0.
6. **"Its gap 10, the small ones: ... All fixed"** - four items are fixed; the same gap's "the negative script builds and runs in the main working tree" is not fixed and appears nowhere in the report, and `both_listed`'s missing `-a` likewise.
7. Nothing else in the "(session output)" list could reasonably have been logged, except item 5 above and the scan.log self-check, which the report already explains. One thing no committed scan covers: the record commit bd8c581's own message and the new scan.log blob fall outside the scanned range `869796b..2623b70`; the report's sentence about the pre-push hook covers this, but planning should know the in-phase scan does not.

Also worth planning's eye, though not a report error: scan.log's phase-diff section scanned the **superseded** log blobs (`24b75cdb...`, `062d6bdd...`, `5047a291...` at scan.log:375, 393, 402), because those are what the commit range contains; the current logs are scanned only in the working-tree section (scan.log:494, 503), whose hashes `aad7302a...` and `41cbd2f9...` I confirmed equal the blobs committed in bd8c581.

## Gaps, most important first

1. scan.log carries no rerun header naming record commit 9be61d6, which specs/0001 requires, and the report states the opposite for all three logs.
2. No primary shows an `error` row reaching the record script: C10 runs only over `run-lo`, the counts row of the outcome table is not printed, and the "could not run" section's contents are never shown. The report's claim is stronger than its citation.
3. `passthrough-record.py` line 163 still says "other than ok, fail or -" although `error` is now valid, and negative.log:355 pins that stale wording as the expected refusal.
4. The error/fail boundary departs from the ruling in two directions that matter for P4: a timeout or an OOM kill reads as `fail`, and an input workbook python's `zipfile` cannot read now reads as `error` (the ruling lists an unreadable ODS, not an unreadable input). Neither is named in the report or the record.
5. Only one of five `error` branches is exercised (python3). The podman 125-127 branch, the two grep-error branches and the directory branch have no case.
6. The gate does not assert the entry-script hash against the record's build-input hash, only prints it; `entry_has_flags` matches each flag anywhere in the file; and gate.log:270-273 remain circular. The pins table still has no primary.
7. A live check-1 finding was dropped without record: the negative script still builds and runs in the main working tree (negative.log:62, 158, 253), against specs/0001.
8. `error` rows are listed twice in the record, once under a heading that says "failing".
9. `both_listed` (negative.log:124) still omits `-a`; the independent-check-1.md header gives only the pre-replacement byte count, so the file's 22220 bytes does not match the 22219 stated; the scan.log:491 citation is for the count line, not the Co-Authored-By assertions; and departure 4's "zero pattern matches" could cite scan.log:384-392 instead of being session output.
