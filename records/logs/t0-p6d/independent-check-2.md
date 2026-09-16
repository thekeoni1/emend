# T0 P6d independent check 2

- Checked commits: 721e86323e41fe6090b95ce3e26876e81e27b464 (phase commit 1)
  and 14c669f0aa5f8dca2e5e1fe2e66f1aa767379093 (phase commit 2, the class
  widened after independent check 1)
- HEAD when the check ran: 91775fe09b3f1f6c653a0454bf26a7f360516795 (the record
  commit with the four logs rerun at phase commit 2, local only)
- This is the second and last check of the phase (specs/0001-bootstrap.md,
  "How this task runs": at most two checks per phase)
- Checker: a fresh general-purpose subagent (not a fork), given only the inputs
  below plus planning's rulings of 2026-09-16 quoted in its prompt, asked to
  answer in plain ASCII, told to run read-only commands only, and told to name
  the three patterns by their line number rather than write their shapes. It
  followed that rule: the answer holds 0 matches of any pattern in the file and
  0 matches of the three forms as they read before this phase.
- Inputs, with sha256:
  - specs/0002-gate-corpus-additions.md
    05c8478dbb5d1ff54371b98d9a5059fd5c37a458dda61fea8598b695f6619c4d
  - specs/0001-bootstrap.md
    5b7ed9aa84e89ec895f3cbd4065ef38c5d2d1cf2a1d7c2a0b00be4c7608e6f4b
  - phase diff 1, `git diff 2638d21..721e8632`, saved as <scratch>/t0p6d-diff.txt
    38ce503fe616ba5e18760546dad909b4d00ddd685f2d774908c75068873436a2
  - phase diff 2, `git diff ad1683c..14c669f`, saved as <scratch>/t0p6d-diff2.txt
    b264269b20a1a77c795e79cde9efdf326a02624b46a54bd55c02724edb951160
  - records/logs/t0-p6d/independent-check-1.md
    485d1f1bd52ad596c41de29027b6063375db5dae81995f5dfa09965a6bf2df7f
  - draft report 2, <scratch>/t0p6d-report-draft2.md
    99f8800b89ee76c19a63adfefe122b05653d14504fc7fc7f350b176f226f529e
  - records/logs/t0-p6d/gate.log
    1ed319e7c1dcd5fba6775cc95f1969fc6b48fa3290c5458f32db44ec93391916
  - records/logs/t0-p6d/privacy.log
    f0c828cfa740e700d8bb40b2aee544e62c272f50ab7937666ea38d3bffe3ebf6
  - records/logs/t0-p6d/negative.log
    1fa3cb1345a78546cc766889a86b949bf946fe2f5ab874b0bbf851584ddb8626
  - records/logs/t0-p6d/scan.log
    0e5d2ffa701ce1ea23b8f287910108b25b1d88c8c868d871eaad2a6413e1c491
- Answer: the checker's final message, extracted verbatim from its transcript
  with python, without printing the transcript, and concatenated to this header
  by the same script. It is 22142 bytes, 112 lines, sha256
  50e86a33dea60541e1b0ae05affa3b5cc40bf695fc962cd43106cf05b56dd5b8, and holds 0
  non-ASCII bytes and no HTML escapes, so no replacement of any kind was needed.
- Privacy-scanned before the commit with the same command forms as scan.log,
  and additionally with the three patterns as they read BEFORE this phase,
  because the narrowed patterns can no longer see a bare shape and so cannot
  prove the answer holds none: every count 0, with a positive control of 1 on a
  built fixture. Also 0 hostname, home-path and temporary-path matches (session
  output).
- Its two headline findings were reproduced by the implementing session with
  its own commands before the report repeated them: the gate's home-path
  pipeline returns 0 where the raw grep returns 1, and the report's drift
  figures do not reproduce. Both are stated as fact in the report, not as the
  checker's assertion.

---
**Note on my inputs.** The harness also placed CLAUDE.md, a memory index, a git-status snapshot, attribution text and the user's email address into my context. I say so here and used none of it as evidence. Everything below rests on the two spec sections, `<scratch>/t0p6d-diff.txt`, `<scratch>/t0p6d-diff2.txt`, `records/logs/t0-p6d/independent-check-1.md`, `<scratch>/t0p6d-report-draft2.md`, the four logs at 91775fe, and read-only git/grep commands I ran myself.

**Wording.** No path shape is written below in any form. The three narrowed patterns are named only by their line number in `scripts/privacy-patterns.txt`: pattern 8, pattern 10, pattern 11.

# 1. Does the work do what the spec and the rulings say, and nothing else?

**Yes on substance. One correctness defect found, in the gate script rather than in the patterns, and one class of testing that was never done.**

## Diff 2: is the class as ruled?

`<scratch>/t0p6d-diff2.txt` changes three lines of `scripts/privacy-patterns.txt` and nothing else. Each narrowed pattern's required trailing character becomes an alternation of the path-character class with `[^[:print:][:space:]]` - precisely planning's ruling of 2026-09-16 (a path character, or any byte that is not printable ASCII and not whitespace). Confirmed independently: `gate.log:194-195` (3 insertions, 3 deletions, one file), `gate.log:224-225` (exactly three pattern lines changed), `scan.log:354` (14c669f touches only the pattern file), `gate.log:219-223` (count still 11, all three end with the class), `gate.log:240` (no crate, Cargo file or workflow changed), `scan.log:257` (the scan script's copy equals the file at the phase commit). The gate's suffix test now uses the class itself (`gate.log:52`, `:57-62`) instead of the fixed byte count that went stale.

## Does it behave as claimed?

I tested the three live patterns directly with `LC_ALL=C /usr/bin/grep -a -c -E`, the same form `scripts/privacy.sh:137-139` uses (GNU grep 3.12, `scan.log:251`):

- **Every real form still matches.** Pattern 10 matches a single backslash separator with mixed-case letters, a forward-slash lower-case form, and doubled backslashes with capitals - all count 1. Pattern 11 matches single backslash, forward slash and doubled backslash forms - all count 1. Pattern 8 matches a following letter, digit or dot. This agrees with the gate's own evidence: `negative.log:298-306` (each matches its real fixture, none matches its bare shape, each pre-P6d form does match the bare shape), `negative.log:309-317` (case a, each reported under its own number), `negative.log:327-332` (case c, both older forms of pattern 10), `scan.log:280-289` (each pattern and both P6b forms), and the self-test's own plants (`scripts/privacy.sh:470-483`).
- **A non-ASCII first byte in the segment matches on all three.** Logged for pattern 8 only (`negative.log:362`, `:365`); I confirmed by hand that patterns 10 and 11 also match a segment whose first byte is non-ASCII (count 1 each).
- **A bare shape does not match.** `negative.log:299`, `:301`, `:303`; `scan.log:291-293`; case b passes clean (`negative.log:320-324`); case d, the T3 P3 answer as extracted, passes (`negative.log:336-344`) with the discriminating controls at `:338-339`.

## The residues, verified

I measured each character the report accepts. After the trailing separator, on all three patterns: space, `%`, `~`, `$`, a single quote and a double quote all count **0** (not matched); a full stop counts **1**, so a bare shape followed by a full stop is still a hit, exactly as the report and the ruling say. A doubled trailing separator on pattern 8 counts **0** (check 1 named this; pattern 8 kept a single literal separator).

**Characters neither the report nor check 1 names**, all behaving like the accepted residues: `+`, `(`, `&`, `#`, `@`, `!`, `,`, `=`, `[` - a segment starting with any of these is now missed on all three patterns. None is a realistic first character of a user-folder name, so I judge the risk acceptable, but the record should say the residue is "anything outside the path-character class", not a list of five characters.

**One unremarked consequence in the other direction.** `[^[:print:][:space:]]` under `LC_ALL=C` matches every control byte (0x00-0x08, 0x0e-0x1f, 0x7f) as well as every non-ASCII byte. So a bare shape followed immediately by a control byte, or by any non-ASCII punctuation - a typographic quote, an em dash - is a hit. The gate bans both in authored files, so the practical impact is near zero, but it is worth one line in the record: a future checker's answer that placed a curly quote straight after a shape would be refused, which is the failure mode this phase exists to remove.

**No other realistic real-path form is newly missed** beyond what check 1 listed (a segment starting outside the class, a path that legitimately ends at its separator, a doubled trailing separator on pattern 8). I looked for others and found none.

## The defect: the gate log's home-path check

`gate.log:113` reads:

```
n=$(LC_ALL=C /usr/bin/grep -c -E '/home/|/root/' scripts/privacy.sh | awk -F: '{ n += $2 } END { print n + 0 }')
```

With a **single** file argument GNU grep prints a bare count and no colon, so `$2` is empty and the sum is always 0. I ran the exact pipeline: the raw `grep -c` returns **1**, the pipeline returns **0**, and the match is at `scripts/privacy.sh:422`. Therefore `gate.log:227` - "lines with a literal home path in scripts/privacy.sh: 0 (expect 0; the control file counts 1): yes" - is **false**, and the check cannot fail for any input.

Two further points:

- The control on that same line calls `grep -c` **directly**, not through the `awk` stage, so it does not use the same command form as the check and is structurally incapable of catching this. That is the specs/0001 scan rule ("Every scan form has a positive control that uses the same command form") broken in the one place it mattered.
- The bug is new in this phase. `records/logs/t3-p3/gate.log:122` uses the same `awk` form but passes **three** file arguments, where grep does print `file:count` and the form is correct; `records/logs/t2-p1/gate.log:132-137` uses a per-file `grep -c` with no awk. P6d copied the multi-file idiom to a one-file call.
- Fixing it will make the check fail, because `scripts/privacy.sh:422` genuinely holds a home-shaped fixture - which the report's own drift section lists as deliberate test data. So planning has to rule on whether that line is exempt, not just on the pipeline.

Check 1 accepted this line twice (in its positive-controls list and in its Q3 citation list). **I disagree with check 1 here**; this is the one substantive correctness finding of my check.

## Diff 1, re-examined where the rulings bear

- **Pattern 11's `+`.** I confirm check 1 and the report's correction: pattern 11's planted fake (`scripts/privacy.sh:411`) and its P6b plant (`scripts/privacy.sh:422`) use single separators, and the scan control `scan.log:139` likewise; only pattern 10's fake uses doubled separators. Nothing requires the `+` on pattern 11. I add that it is not inert: I verified pattern 11 now matches a doubled separator, and **nothing anywhere tests that**, so a real widening went in untested with a stated reason that is false. The report corrects the reason (ruling 3) but does not say the widening is untested.
- **Case e's isolation.** Check 1 doubted whether the self-test's bare plants for patterns 10 and 11 were live. Case e now widens each in turn (`negative.log:242-258`, results `:348-357`). I verified the isolation by hand: the widened form of pattern 8 matches only the bare line of pattern 8, widened 10 only bare 10, widened 11 only bare 11. So the generic refusal message cannot have come from another plant. Check 1's doubt is closed.
- **Check 1's gap 4** (the spec's own numbering) stands: `specs/0002-gate-corpus-additions.md:144` and `:147` are 0-based, while `specs/0001-bootstrap.md:1151-1152`, `scripts/privacy.sh` and every P6d log are 1-based. The implementation uses the right one. Planning's to fix, as the report says.

## The one thing never tested

Diff 2 widened three patterns; **case f exercises the new alternative on pattern 8 alone** (`negative.log:359-367`). Patterns 10 and 11's new alternative is exercised by no case, no control in `scan.log` (its controls `c-p6-9`, `c-p6-10` are ASCII, `scan.log:137`, `:139`) and no self-test plant. I verified by hand that both do match, so this is a test gap rather than a defect - but the phase's instrument does not prove two thirds of its own change.

# 2. Do the logs contain every line the spec and the rulings require?

**Yes, every enumerated requirement is present.** The qualifications are about what is proved, not what is missing.

- **Six cases.** a `negative.log:309-317`; b `:320-324`; c `:327-332`; d `:336-344`; e `:348-357`, all three patterns; f `:359-367`. Each committing case asserts its commit adds only its own fixture (`:310`, `:321`, `:328`, `:341`, `:364`). Acceptance met: eight PASS (`gate.log:248-259`), `privacy.log` PASS (`privacy.log:63-70`), every case as stated.
- **Controls.** Fixture controls `negative.log:298-303` plus the new pre-P6d controls `:304-306`; ESC/SI control 3 (`gate.log:262`, `privacy.log:71`, `negative.log:370`); non-ASCII control 2 (`negative.log:371`); the full battery `scan.log:260-319` with every pattern individually and combined, both older forms, the bare-name file and the three bare shapes, ending `controls: all as expected` (`:320`).
- **Rules for every phase.** HEAD on line 1 of all four logs. Printed script in all four (`gate.log:11-156`, `privacy.log:9-59`, `negative.log:7-289`, `scan.log:6-248`). `/usr/bin/grep` with `LC_ALL=C` throughout, with exit codes read and `>= 2` treated as an error (`scan.log:73-86`, `scripts/privacy.sh:69-75`). No control bytes: 0 in every captured output with controls; I re-counted all four committed logs myself - 0 control bytes, 0 non-ASCII bytes each. Negative tests in a scratch clone on throwaway branches, never in the tree, status clean (`negative.log:145`, `:374`). Compound checks inside one named function: `ends_with_added_class`, `pattern_count_is`, `clean_pass`, `both_older_reported`, `selftest_refused`, `only_added`, `files_complete`, `messages_complete`, `scan_passed`. I found no `$(yn [ ... ] && cmd ...)` construction.
- **Rerun headers.** All four name record commit 4eb5a98 as the one they replace, and say what changed.

## Does case f prove what it claims?

**Yes.** The fixture's byte immediately after the trailing separator is a UTF-8 lead byte (`negative.log:361` records 2 non-ASCII bytes in the fixture), which is outside the path-character class, so only the new alternative can satisfy the pattern; the live-pattern control counts 1 (`:362`) and the gate reports it under pattern 8 (`:365`, `:367`). It cannot pass for another reason: the pattern is anchored on its literal prefix and the class is the only thing that follows. What it does **not** prove: anything about patterns 10 or 11, and it is not contrasted against the phase-commit-1 form (a control showing that form counting 0 on this fixture would have isolated the change to 14c669f; the conclusion currently follows by inspection of the class, not from the log).

## Does case e now prove all three plants live?

**Yes.** `negative.log:348-357`, exit 2 three times with the self-test's own refusal each time, and `selftest_refused` (`:130-132`) evaluates the exit status and the message in one function, so a generic error cannot pass it. The widening script (`:244-252`) deletes the path-character class and leaves an empty alternative, which GNU grep accepts and which matches the empty string - I confirmed the widened forms match their own bare lines and no other. One weakness: the refusal message is generic and the log does not print which plant fired, so the isolation rests on the pattern text rather than on the log itself.

## Is the account of the four gate runs complete and honest?

The header (`gate.log:3`) contains everything the rule asks: the stale-check failure and what was changed ("it now tests for the class as ruled, as a suffix rather than a fixed byte count"), both kill start times, the last checks each completed, and the reason given. It names the `CARGO_BUILD_JOBS=4` departure explicitly and explains why a third identical retry was not attempted. The killed runs were gate runs, so noting them in `gate.log` alone is right.

**But two claims in that header leave no trace in the log's own evidence.** The printed script exports only `RUSTUP_AUTO_INSTALL` and `CARGO_TERM_COLOR` (`gate.log:28`) and prints only those two (`gate.log:241`); it never sets or echoes `CARGO_BUILD_JOBS`. The free-memory figure likewise comes from the caller: the whole header line arrives through `RERUN_NOTE` (`gate.log:70`). So a declared departure from a recorded run's environment is, in this log, session-authored text inside an otherwise self-evidencing document. One `echo "CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-unset}"` beside line 241 would have closed it. Honest, but not evidenced.

## Does any log write a shape?

**No - and I proved it the only way that now works.** The scan battery uses the **narrowed** patterns (`scan.log:41-44`), which by construction no longer fire on a bare shape, so `scan.log`'s 0 hits do not establish the "no shape" claim. I ran the three **pre-P6d** forms over all four logs, over `records/logs/t0-p6d/independent-check-1.md` and over `<scratch>/t0p6d-report-draft2.md`: **0 matches in every file**, with a positive control on a built fixture counting 1. That is the missing control - a bare-shape scan using the pre-P6d forms belongs in the phase's scan script from now on, since the gate itself can no longer see them.

Two near-misses worth naming: `negative.log:166-168` and `:229` print the pre-P6d patterns as regex text, and `:221` builds the shape from separate concatenated pieces inside the printed python. Both are the sanctioned forms and neither matches anything, so the log's own claim (`negative.log:5`) stands - but the bracketed regex at `:166-168` is close enough that a reader recognises the shape, which is worth knowing before these logs are pushed.

Also verified: `scan.log` scanned the **final** versions of the other three logs - the sha256 values at `:514`, `:523` and `:532` equal the committed files on disk. The final `scan.log` itself is scanned by nothing, which the report states and marks as session output; I re-ran those counts and confirm 0 control bytes, 0 non-ASCII, 0 pattern hits.

**What the logs do not contain:** no case for patterns 10/11 with a non-ASCII segment, and no case pinning the boundary that actually forced the ruling (a shape followed by a backtick). The second is now checkable from a committed primary - see Q3, item 4.

# 3. Does every number and quoted string in the draft report appear in a primary?

**Correct and landing on the cited line:** 140 insertions / 23 deletions for 4641ae3 (`gate.log:171`); `scripts/privacy.sh` parses (`:216`); 11 patterns (`:219-220`); all three end with the class (`:221-223`); exactly three pattern lines changed (`:225`); identity file and hook installer unchanged (`:226`); the cold `cargo clean` and the eight PASS lines with exit 0 (`:243-259`); empty status (`:267`); all as expected (`:269`); log lengths 269, 74, 376 and 547 (I ran `wc -l`); privacy exit 0 (`privacy.log:63`), `PASS privacy` (`:65`), the quoted `"165 commit(s), 405 blob(s) scanned, 0 finding(s)"` (`:67`), the three checks (`:68-70`), control bytes against a control of 3 (`:71`); every `negative.log` citation (`:298-303`, `:304-306`, `:309-317`, `:320-324`, `:327-332`, `:337-344`, `:348-357`, `:359-367`, `:310`, `:321`, `:328`, `:341`, `:364`, `:370-372`); `762fc434...` (`negative.log:336`, and independently the T3 P3 header); `scan.log:257`, `:320`, `:458`, `:513-541`, `:543-547`. The 285 insertions / 99 deletions for 91775fe is right (I ran `git show --stat`) and is marked session output.

**Problems, in order of weight:**

1. **The drift counts do not reproduce.** Method: `LC_ALL=C /usr/bin/grep -a -o -E` for an absolute home-or-root path over every tracked file. I count **86 occurrences on 78 lines in 56 files**, not 59. Of those: **7** are corpus-directory paths - exactly the seven files and lines the report lists, verified individually; **10** are repository paths - exactly the ten it lists, verified individually; but **31** occurrences in total sit under the account's home directory, so **14 account-home occurrences are in neither of those buckets and appear nowhere in the report**: `records/logs/t0-p1r/verify.log` (7), `records/logs/t0-p1r/independent-check-1.md` (2), `records/logs/t0-p3/independent-check-1.md` (2), `records/logs/t0-p2/independent-check-2.md` (1), `specs/0001-bootstrap.md` (2). The remaining 55 split into generic placeholder paths in fixtures (29) and bare mentions of the directory prefix with nothing after it (26); neither total is 42. **Consequence for planning's ruling:** the report puts `specs/0001-bootstrap.md:168` and `:885` in the "deliberate test data" bucket, but both are account-named absolute paths, not placeholders. So the report's conclusion - "the only disclosure beyond the account name is the corpus directory name" - understates it: fourteen occurrences disclose locations under the account home outside the repository, seven of them in one committed log. Every individual citation in the report's kind-2 and kind-3 lists is correct; it is the kind-1 count and the classification that are wrong.
2. **"no literal home path in scripts/privacy.sh against a control that counts 1 (227)".** The citation is accurate, the fact is not: the check always prints 0 (measured, see Q1) and the file holds one at `scripts/privacy.sh:422` - which the report's own drift section names two pages later. This is the one number in the report contradicted by a primary, and by the report itself.
3. **"5 commit messages, each ending with the Co-Authored-By line (511)".** `scan.log:511` supports only the count; the per-message assertions are at `:462`, `:472`, `:482`, `:492` and `:502`. This is check 1's gap-7 mis-citation carried forward with a new line number, while the report's "What check 1 changed" section lists gap 7's other two items as fixed and drops this one silently.
4. **The central measurement is uncited, and needn't be.** "measured against the T3 P3 check 1 answer as extracted, that matches it again, because the answer writes the shapes inside backticks" is the justification for the whole ruling and carries no citation. It is **true and verifiable from a committed primary**: `records/logs/t3-p3/independent-check-1.md` holds the replacement token wrapped in backticks in both of its two occurrences, so under planning's first wording the reconstruction would match again. It should be cited, and a two-line case in `negative.log` (the reconstruction against a pattern built with the rejected wording) would have made the phase's own reason a logged fact rather than a remembered one. The companion claims about a shape in double quotes and one followed by a comma are logically right - I confirmed both characters fail the ruled class - but have no primary either.
5. **Correctly marked session output**, and nothing to add: 4641ae3 holding planning's bytes unedited, the 91775fe stat, and `scan.log` not scanning itself. Of these, only the last could now have been logged (a second scan pass over the finished `scan.log`, written to a fifth file).

# Gaps, most important first

1. **`gate.log:113`'s home-path check is broken and its control cannot catch it.** Single-file `grep -c` piped into `awk -F:` always yields 0; the real count is 1 (`scripts/privacy.sh:422`). `gate.log:227` reports a pass that is false, and the report repeats it. New in this phase (the t3-p3 original passed three files, where the idiom is correct). Fixing the pipeline will make the check fail, so planning must also rule on whether that fixture line is exempt. Check 1 accepted this line; I disagree with check 1.
2. **The drift figures are wrong.** 59 and 42 do not reproduce: 86 occurrences on 78 lines. The 7 corpus and 10 repository citations are exact; 14 further account-home occurrences are unlisted, and `specs/0001-bootstrap.md:168` and `:885` are mis-classified as fixtures when they name the account home.
3. **Two thirds of diff 2 is untested.** The new alternative is exercised only on pattern 8 (case f); patterns 10 and 11 have no case, no control and no plant for it. Case f also lacks a control against the phase-commit-1 form, which would have isolated the change to 14c669f.
4. **The measurement that produced the ruling is neither cited nor logged**, although the primary that proves it is committed (`records/logs/t3-p3/independent-check-1.md`).
5. **"No log writes a shape" is not proved by this phase's instruments**, because the scan battery uses the narrowed patterns, which by design no longer see a bare shape. I proved it by hand against the pre-P6d forms (0 in all four logs, in check 1's answer and in the draft report, with a positive control of 1). A pre-P6d bare-shape control belongs in every future scan script.
6. **`gate.log:3` declares a `CARGO_BUILD_JOBS` departure that the log's own printed script neither sets nor prints** (`:28`, `:241`); the same is true of the free-memory figure. Disclosed as the rule requires, but unevidenced.
7. **Minor:** the `scan.log:511` mis-citation (check 1's gap 7, unfixed); the residue list should read "anything outside the path-character class" rather than five characters (`+`, `(`, `&`, `#`, `@`, `!`, `,`, `=`, `[` behave identically); the class also admits control bytes and non-ASCII punctuation, so a bare shape followed by a typographic quote is a hit - the very failure this phase removes, worth one line in the record; pattern 11's `+` is a real, untested widening, not only a false reason; and `negative.log:166-168` prints the pre-P6d patterns as regex text, which is permitted and matches nothing but is recognisable to a reader.
