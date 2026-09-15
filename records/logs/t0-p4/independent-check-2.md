Checked phase: T0 P4 (specs/0001-bootstrap.md)
Checked commit: a713c93b080b415aa9c39d51c327b930445ad2e7 (P4 phase commit); logs through 795a151de876e2c743821bf0724b443ad6b17f41
HEAD when the check ran: 795a151de876e2c743821bf0724b443ad6b17f41
Checker: a fresh general-purpose subagent, not a fork
Inputs (sha256):
- specs/0001-bootstrap.md: ccc54ed25414850cd471a6ecc70772ea41989e8cd30677dfe378b81853ef0800
- <scratch>/p4-diff.txt (git diff 53b99d8626775d71a5c608b9639646544793d189..a713c93b080b415aa9c39d51c327b930445ad2e7): 5fbb071b752cb2a7d2cf12b6cf5fb4e3262cf90e6df336a02f508fc6791285b6
- records/logs/t0-p4/gate.log: 8159e2a57caa2bc835beb8091e7b731a5ded0004056be72b31fee8ada22b64fb
- records/logs/t0-p4/negative.log: fa52b5ef16a1b5fd83f283f19ff941690d98619f38ed43dd9b88f30854fbef85
- records/logs/t0-p4/negative-attempt1.log: c8e9b4ad01fd3ad8cfdd2d297c446e5b8722ed2aa326952efb9a54eae6e0ddd4
- records/logs/t0-p4/scan.log: 9ef0734a495d398c26979c513fa38f695e80a609fdb65074068176da9c682b46
- records/logs/t0-p4/verify.log: ecbfe52209a75b3a1622c9cd156acec2cc230a366afc6eba9150bd09048251f7
- records/logs/t0-p4/verify-attempt1.log: cb05b3d9a9cb1f9c4c0ad7b8a165a1f9db440f232a71b77c2abc73c319287326
- records/logs/t0-p4/independent-check-1.md: 61e8980a00c7221ef04a8028caa1454a2315b3d1420eaac71cfa692025e6faf4
- <scratch>/p4-report-draft.md (draft 2): 8951395c990b11404d4919b22910fd3aeba55ad758e73ef73d7007730f976280
---
# T0 P4 independent check 2

Harness material: the harness put more into my context than the ten inputs. It added CLAUDE.md, an auto-memory index, user identity details, a git status snapshot with recent commit hashes and subjects, and a commit attribution reminder. I did not use any of it as evidence, and I do not repeat the identity details. I read only the ten inputs and ran no commands. What I say about how code behaves comes from reading it.

Labels: "the diff" is the phase diff and "the draft report" is the draft. Logs are cited as G (records/logs/t0-p4/gate.log), N (negative.log), N1 (negative-attempt1.log), S (scan.log), V (verify.log), V1 (verify-attempt1.log) and I1 (independent-check-1.md), all under records/logs/t0-p4/.

## Q1. Does the diff do what the P4 spec says and nothing else?

PARTLY. Steps 5 and 6, the empty allowlist, colour off and the doc commit are all present, and I found no defect that lets either step pass when it should fail within the spec's own command and rules. The diff departs from the spec text in two places and adds three small things the spec does not ask for. Several realistic routes are still not exercised by any primary.

### Doc-commit ruling

- S lines 247-253: 977635b is the first commit after 53b99d8. It touches only ROADMAP.md and specs/0001-bootstrap.md. a713c93 touches only scripts/check.sh and scripts/sys-allowlist.txt (S lines 255-258), so every ROADMAP and spec hunk in the diff (lines 1-22, 180-279) comes from the doc commit.
- S scans both files (S lines 266-273, 290-297) and the commit's message (S lines 310-318). The message ends with the Co-Authored-By line (S line 311). That meets "that phase's scan.log covers them".
- The spec hunks agree with the spec text I was given: the status line, the control-byte rule, "Planning rulings on P3", the P3 ignore.log line and the P4 colour paragraph. I cannot hash the spec I read. S line 290 and I1 line 6 give the same sha256 for it (ccc54ed2...), but that ties I1's input to the commit, not my copy.
- The primaries cannot show that the files were unedited. They also cannot show that the commit holds all of planning's pending changes, because a pending file left out would leave no trace. The draft report says "unedited" is not shown (lines 32-36). It does not mention completeness.

### Colour off

- The diff line 34 exports `CARGO_TERM_COLOR=never` and line 46 adds `-- --color never` to fmt. Both match the spec.
- V v7 (V lines 644-669) runs fmt on the same misformatted file with no flag and CARGO_TERM_COLOR unset, and gets `lines=3 bytes=6` (V line 648). With the flag it gets `lines=0 bytes=0` (V line 651). Through check.sh the captured output has 0 bytes (V line 666).
- Caveat: the "before" run changes two things at once, the env var and the flag. It shows that the pair removes colour, not which of the two does it. The spec asks for both, so this is enough.
- Not shown: colour from the libtest harness on a failing test. No P4 primary has a FAIL host run.

### Allowlist file

The diff lines 177-179 show a new file, mode 100644, blob e69de29 (the empty blob). S line 282 and G line 80 give sha256 e3b0c442... with `bytes 0`. It was created empty, as the spec says.

### Step 5 native (the diff lines 55-75)

- **Departure 1.** `--prefix none` is added to the spec's command (the diff line 58). The draft report flags it. The first field of each output line is the crate name (G line 107, N lines 303-306, V lines 426-429), and the `(*)` dedup suffix comes after the name (V1 line 430), so the awk at the diff line 61 reads names correctly.
- **Banned names.** cc, cmake and pkg-config are matched as exact names (the diff line 62). V v1 shows this firing: `FAIL native: C build tooling in the dependency tree: cc cmake pkg-config` (V line 425). V v2 shows it firing through a non-member wrapper (V lines 462-465).
- **-sys crates.**
  - Names matching `/-sys$/` are checked against the allowlist by first field, skipping blank and `#` lines (the diff lines 67-71).
  - N case e fails as expected (N line 302). N line 306 shows fake-sys under xlsx-diff's own tree, so it would be caught even if it were not also a member root.
  - V v3 shows an allowlisted fake-sys passing with a comment and a blank line (V lines 481-496). V v4 shows a missing allowlist failing (V line 506).
- **Additions the spec does not ask for:**
  - the missing-allowlist check (the diff line 57);
  - comment and blank-line handling in the allowlist;
  - the `readonly SYS_ALLOWLIST` constant.

  All three are harmless.
- **Departure 2.** The spec says "List every crate whose name ends in -sys". The step names only unlisted ones, and only on failure. The draft report's Finding 7 flags this.
- **Could it pass when it should fail?** Not with the spec's command as written. These routes remain:
  1. **Build-dependency edges are never exercised.** cc nearly always arrives as a `[build-dependencies]` entry. Every fixture in N, V and V1 uses `[dependencies]` only. `-e normal,build` should include build edges, but no primary shows it. The draft report does not mention this.
  2. **`--target all` with target-specific dependency tables is never exercised.** The draft report does not mention it.
  3. **Features of workspace members that are off by default.** The spec's command has no `--all-features`. V v6 tests only a feature of a non-member that nothing enables. The draft report's Finding 4 covers this accurately.
  4. **Underscore spellings.** `pkg_config` or `foo_sys` do not match. I1 raised this. V does not test it, and the draft report no longer mentions it.
  5. **The allowlist matches by name only**, not by version or source. The spec does not say otherwise, but a ruling on one `-sys` crate then covers any crate of that name.
- **Could it fail when it should pass?** I found no case.

### Step 6 licences (the diff lines 77-176)

I read the embedded Python closely.

- **Tokens.** The tokenizer `\(|\)|[^\s()]+` (the diff line 102) gives parentheses and runs of non-space characters.
- **Grammar.**
  - `any_of := all_of (OR all_of)*` and `all_of := term (AND term)*`, so AND binds tighter than OR.
  - A term is `( any_of )`, or an identifier with an optional `WITH exception`.
  - Both sides are parsed before `or`/`and` combine them (the diff lines 119-120, 127-128), so short-circuiting skips no tokens.
  - Trailing tokens raise an error (the diff lines 149-150).
- **Identifiers and WITH.**
  - `tok + " WITH " + exception in ALLOWED` groups as `(tok + ...) in ALLOWED` (the diff line 145). That is correct.
  - A bare token cannot contain a space, so it can never equal the allowed `Apache-2.0 WITH LLVM-exception` string by itself.
  - Operators are case-sensitive, so `MIT or ISC` ends with "unexpected or", which fails closed.
- **Bad input.**
  - A null or empty licence fails (the diff line 159).
  - An all-whitespace licence fails as "unexpected end".
  - A Python exception or a missing python3 exits non-zero and fails through the diff line 82. stderr is merged into `$OUT/licences`, so the FAIL line carries the last line of the traceback.
  - A newline inside a licence string only shortens the FAIL reason (`tail -n 1`, the diff line 174). The step still fails.
- **What the logs exercise.**
  - V v5 runs WITH, parentheses, an AND with every part allowed, a syntax error, lowercase and the slash form through real cargo metadata. Every verdict is as expected (V lines 576-596).
  - N cases a-d cover MPL-2.0, no licence, `MIT AND ISC` and `ISC OR MIT` (N lines 190, 219, 248, 277).
  - Workspace members are checked: xlsx-diff appears in N line 192.
- **Could it pass when it should fail?** I found no path.
- **Could it fail when it should pass?** Lowercase identifiers and `MIT/Apache-2.0` fail. Both fail closed, which fits "reported for a planning ruling", and the draft report says so.
- **Dev-dependencies.** No primary shows whether `cargo metadata` includes them. The draft report's Finding 8 says so.

### What the primaries cannot show

- They cannot show `quiet()` or `fail()`, which are outside the diff hunks. N line 320 shows only that `fail()` indents the tool output.
- They cannot show that "unedited" holds.
- The fixtures in N cases a-e, V v3 and V v5 sit under `neg/` inside the clone. By the reasoning in V's header (V lines 15-18), that makes them workspace members. It does not change any N verdict, but no primary shows a non-member `-sys` crate being detected. The draft report does not say that N's fixtures were members.

## Q2. Do the logs contain every line the P4 spec requires?

PARTLY. Every line in the P4 "Logs" and "Acceptance" lists is present. The rules for every phase are met for G, N, N1 and S. V and V1, the two logs written later, are covered by no control-byte, non-ASCII, pattern, hostname or denylist scan in any primary. Neither V nor the draft report says so.

### P4 Logs and Acceptance

- **gate.log:** a full run, not `--quick`, on a cold cache, from `/` (G lines 85-87). The six PASS lines are at G lines 89-94, `exit: 0 (expect 0)` at line 96, and the exact-order check says yes at line 97. Met.
- **negative.log**, run in fresh clones (N line 3, N script lines 58-62), with the clone removed at the end (N line 335):
  - (a) `FAIL licences: fake-mpl 0.1.0 (MPL-2.0)` (N line 190), exit 1 (N line 194);
  - (b) `FAIL licences: fake-nolicence 0.1.0 (no licence)` (N line 219), exit 1 (N line 223);
  - (c) `FAIL licences: fake-mit-and-isc 0.1.0 (MIT AND ISC)` (N line 248), exit 1 (N line 252);
  - (d) `PASS licences` (N line 277), exit 0 (N line 278);
  - (e) `FAIL native: -sys crate not in scripts/sys-allowlist.txt: fake-sys` (N line 302), exit 1 (N line 307).

  Met. The "outside the repo" location is shown only by `mktemp -d` in the script, because the log hides the path.
- **fmt rerun with zero control bytes:** N lines 319 and 330 show a zero line count. V line 666 shows 0 bytes for the same case through check.sh. Met.
- **scan.log:** present, with `RESULT: PASS` (S line 351).
- **Acceptance:** all six PASS in G, and every negative case as expected (N line 337). Met.

### Rules for every phase

- **Toolchain rule.** `RUSTUP_AUTO_INSTALL=0` appears in check.sh (diff context line 32), G script line 16, N line 24, N1 line 17, V line 35 and V1 line 31. G line 82 prints it. S runs no cargo. Ad-hoc session commands are not shown, and the draft report says so (lines 182-185).
- **Control-byte rule:**
  - S counts G, N and N1 (S lines 322-345), with its own controls at S lines 198-199, and expects 0.
  - S counts lines (`grep -c`), not bytes. V recounts G, N, N1 and S as committed, in bytes, and all are 0 (V lines 672-682).
  - V counts I1 in lines only (V line 694). Zero lines implies zero bytes, so the result holds.
  - **Gap:** no primary counts control bytes in V or V1. A log cannot scan itself (P1 ruling), but V could have scanned V1 and did not.
- **Scan rules:**
  - S and V both start with `#!/usr/bin/env bash`, call `/usr/bin/grep` and use `LC_ALL=C` with `-P '[^\x00-\x7F]'`.
  - S matches U+2014 as bytes, and S's `run()` counts exit 2 or more as an error.
  - Both scripts are printed.
  - Weaknesses in V:
    - V's `f_p6` control uses one pattern of nine (V line 210). S controlled all nine.
    - V's `f_host` control covers the short hostname only, not the FQDN (V line 211).
    - `ctrl_bytes` (V line 47) is `tr | wc -c` inside `$(...)`. A read error would print 0 rather than an error. In every V result, grep's line count printed 0 from the same file, so no zero in V is actually false. It is still the "error reported as zero" shape the rules forbid.
- **Log privacy.**
  - S reports 0 hostname, pattern and denylist hits in G, N and N1 (S lines 322-345).
  - V scans I1 and the 980b9e3 message (V lines 685-697).
  - No primary scans V or V1. Reading them, I saw only `<scratch>`, `<repo>` and repo-relative paths. Reading is not a denylist or hostname scan.
  - No primary scans the messages of the later record commits, which the draft report names as 729ab0e, c83e398 and 795a151.
- **First-line rule:**
  - G, N, N1, V and V1 name the commit and the HEAD at run on line 1.
  - S line 1 names the phase diff range, which ends at a713c93, and the HEAD. I accept that.
  - I1 names the phase on line 1 and the commit on line 2, which the P1R Question 3 ruling accepts.

### Gaps raised by I1: closed by V, and reported accurately?

1. **Colour before and after.** V v7 closes it, with the two-variable caveat above. The draft report item 1 is accurate.
2. **Line counts labelled as bytes.** V closes it for G, N, N1 and S as committed, but not for V or V1. The draft report's Finding 5 says "G, N and S ... label the result 'control bytes'". That is true of G (line 99) and N (line 195). S's output says only `form=f_ctrl count=`. Finding 9's "The committed logs have no control bytes (V)" goes further than V shows: V covers four logs, not V1 or V itself.
3. **Case f has no absent check.** V v7 closes it (V line 669). The draft report (lines 103-104) is accurate.
4. **N1 calls itself negative.log.** No later log can fix this. V line 19 and the draft report's Finding 1 both say so.
   - **New:** V1 repeats the same flaw. V1 line 2 says `verify.log`, its script comment (V1 line 21) says the output is moved to verify.log, and nothing in V1 says it is attempt 1. Neither V nor the draft report mentions this.
5. **Fix 3 in N was not a response to a failure, and it narrows the tree check.** This stays open. N's check still ignores records/logs/t0-p4/. V's own final check has no filter (V lines 358, 699). The draft report's Finding 1 is accurate.
6. **Fix 1's cause is a reading.** The draft report marks it "My reading". Closed.
7. **G and N1 started at the same timestamp.** The draft report states it, cites G line 2 and N1 line 2, and marks "same start command" as session fact. Accurate.
8. **G line 3 is fixed text.** Not addressed, and V line 3 has the same pattern. Acceptable, and the draft report does not mention it.
9. **Feature-gated dependencies.** V v6 partly closes this. It shows a non-member's optional dependency missing from the tree and from metadata even with `--all-features` (V lines 634-642). The member-feature case is untested. The draft report's Finding 4 is accurate.
   - **New mislabel:** V line 631 says "(0 means both steps miss a dependency behind a non-default feature)". V's own lines 634-642 show the dependency is in no resolve at all, so there is nothing to miss.
10. **The banned-name branch never fired.** V v1 and v2 close this for normal dependencies. The build-dependency edge is still unexercised, and the draft report does not say so.
11. **An allowlisted `-sys` crate passing, and a missing allowlist failing.** V v3 and v4 close both. Accurate.
12. **WITH, parentheses, syntax error, and an AND whose parts all pass.** V v5 closes all four. Accurate.
13. **Lowercase identifiers.** Now in the draft report (lines 81-83), shown by V line 578. Closed.
14. **Underscore spelling.** V does not test it, and the draft report drops it. Open and unreported.
15. **"A renamed wrapper would not be caught".** V v2 tests a wrapper that depends on cc under its own name. It does not test a renamed dependency (`package = "cc"`). The draft report's Finding 3 claims more than V v2 shows (see Q3).
16. **`quiet()`/`fail()` not in the diff.** Still not visible. Not mentioned.
17. **S's header comment says "the other P3 logs".** Cannot be fixed. The draft report's Finding 6 says so (S line 8). Accurate.

### How the attempt logs are recorded

- **N1.** The failure is kept as its own file, and N lines 4-10 name the cause and each change. I compared the N and N1 scripts. The only differences are:
  - the header echoes;
  - lib.rs changed from empty to a doc comment (N line 77 against N1 line 63);
  - the Diff pattern and its label (N line 157 against N1 line 143);
  - the status pathspec (N lines 162-163 against N1 lines 148-149).

  All are named. check.sh is the same commit in both (line 1 of each). This meets "record, fix, re-run with the fix named". Weaknesses: the failure is described in the re-run's log rather than in N1 itself, N1 is mislabelled, and one listed "fix" answered no failure.
- **V1.** V1 ended `RESULT: PASS` (V1 line 678), but its v1, v2 and v6 fixtures were workspace members. V1 v2 shows cc as a member root (V1 lines 424-430). V1 v6 went the opposite way from V v6: FAIL native, and both PASS lines absent (V1 lines 591-602). V lines 15-18 name the flaw and the fix. My script comparison finds only `add_ext_crate`, `depend_ext`, `members`, the `rm -rf "$BASE/ext"` lines and the header echoes added. Nothing else changed.
  - V1 is a passing run whose design was wrong, not a failing check, so the spec's rule applies by analogy. It is recorded honestly apart from V1's self-label.
  - The draft report's Finding 2 is accurate. It does not note that the fix reversed V1's v6 observation.

## Q3. Does every number and quoted string in the draft report appear in a primary?

PARTLY. Almost every value matches a primary exactly. One hash is wrongly marked as absent from the primaries. One commit role is unmarked. Three statements claim more than their primary shows. Two external claims are not marked.

### Values found in a primary

| Draft report item (line) | Primary |
|---|---|
| Range 53b99d86...a713c93b... (13) | S lines 1, 246; I1 line 7 |
| "The diff file does not state its range" (14) | the diff line 1 starts with `diff --git` |
| 977635b63e89..., its subject, parent 53b99d8, `M ROADMAP.md`, `M specs/0001-bootstrap.md` (18-19) | S lines 248-253 |
| a713c93b080b..., its subject, parent 977635b, `M scripts/check.sh`, `A scripts/sys-allowlist.txt` (20-21) | S lines 247, 255-258 |
| 980b9e3397bb... as the record commit for G, N, N1 and S (22-23) | V lines 1, 671-682; I1 line 2 |
| c83e398755c8... as HEAD at run of V (25-26) | V line 1 |
| `lines=3 bytes=6` and `lines=0 bytes=0` (45-46) | V lines 648, 651 |
| cargo tree command (48) | the diff line 58 |
| v1 FAIL line and `members: xlsx-diff` (53-54) | V lines 419, 425 |
| v2 FAIL line (56) | V line 462 |
| v3 all six PASS, exit 0 (59-60) | V lines 486-492 |
| Case e FAIL line (62) | N line 302 |
| v4 FAIL line (64) | V line 506 |
| Allowlist sha256 e3b0c442... and `bytes 0` (66-67) | S line 282; G line 80 |
| Licence step description (69-73) | the diff lines 80, 100-171 |
| v5 allowed and not-allowed lists and the FAIL line (76-80) | V lines 576-585 |
| G items (88-93) | G lines 85-103 |
| N items (a)-(f) (95-101) | N lines 190, 219, 248, 272-278, 302, 319, 330 (prefix) |
| Absent checks, and `PASS fmt: expect absent` (103-104) | N lines 197-199, 226-227, 255-256, 281-282, 310-311, 332-333; V line 669 (prefix) |
| `all negative cases as expected: yes` (105) | N line 337 |
| `controls: all as expected`, and 46 marked as not printed (107-108) | S line 243; I count 46 control lines at S lines 197-242 |
| S counts and RESULT (109-112) | S lines 298, 319, 346, 348, 351 |
| V results (113-114) | V lines 391, 700-702 |
| G, N and N1 sha256 and `equal to the hash scan.log recorded: yes` (118-122) | S lines 322, 330, 338; V lines 672-679 |
| S sha256 9ef0734a... (123) | V line 681; I1 line 11 |
| `lines=0 bytes=0` for G, N, N1 and S (124) | V lines 674, 677, 680, 682 |
| I1 sha256 61e8980a... (125) | V line 692; V1 line 668 |
| N1 `Diff in <scratch>/emend/neg/fake-mpl/src/lib.rs:1:` and two `+` lines (134-135) | N1 lines 174-176 |
| Fixes named (136-143) | N lines 5-9; N1 line 310 (empty tree check) |
| N1 calls itself negative.log (144-145) | N1 line 2 |
| 2026-09-15T00:44:36Z (147) | G line 2; N1 line 2 |
| V1 flaw and fix, `workspace members: xlsx-diff` (148-152) | V lines 15-18, 419, 456, 621; V1 lines 424-430 |
| V v6 facts (156-158) | V lines 600-642 |
| S line 8 "the other P3 logs" (167-168) | S line 8 |
| Guard in D, G, N, N1, V and V1; S does not set it (182-185) | diff context line 32; G line 16; N line 24; N1 line 17; V line 35; V1 line 31; S lines 6-188 |

### Correctly marked as not in a primary

- 795a151de876e2c743821bf0724b443ad6b17f41 (line 27).
- The stat `2 files changed, 61 insertions(+), 6 deletions(-)` (line 33). It appears in no primary. I recount it from the diff hunks as +3/-3 for ROADMAP.md and +58/-3 for the spec, which gives +61/-6, in agreement with I1 line 208.
- The pre-commit scan and the review before committing (lines 34-36).
- The 18-expression test (line 84).
- The V1 sha256 cb05b3d9... and the V sha256 ecbfe522... (lines 126-128). They appear in no input. I cannot check them, and the check 2 header does not exist yet.
- No toolchain installed; clean tree, no remote, nothing pushed (lines 185-186).
- `--prefix none` output without the flag (line 51).
- Dev-dependencies in metadata (lines 173-174).
- Test-harness colour (lines 175-176).

### Wrong, overstated or unmarked

1. **Wrongly marked absent.** Line 24 says 729ab0ecd297ccc9a10977acca2cfe77fc928b06 is "NOT in a primary". It is in V1 line 1, as HEAD at run. Its role, record commit for I1, is not stated by a primary. V1 line 668 does read I1 at that HEAD.
2. **Unmarked role.** Line 25 calls c83e398 the "record commit for V1". V line 1 shows the hash only as HEAD at run. The role is session knowledge and is not marked.
3. **Overstated: Finding 3** (lines 153-154). V v2 shows an unrenamed wrapper dependency caught. It does not test a renamed dependency, which is how I1 read draft 1's sentence. "Draft 1 was wrong" is supported only for the wrapper reading.
4. **Overstated: Finding 5** (lines 164-166). S does not label its result "control bytes". Only G (line 99) and N (line 195) do.
5. **Overstated: Finding 9** (lines 176-177). "The committed logs have no control bytes (V)" covers G, N, N1 and S only. V1 and V have no count in any primary.
6. **Unmarked external claim.** Lines 81-82 say SPDX "matches case without regard to it". That is outside knowledge, not shown by any primary.
7. **Unmarked inference.** Line 159 says the v6 dependency "is in no build, so missing it is harmless". V lines 634-642 show it absent from the resolve. "No build" is a reasonable inference but is not marked as one.

### Omissions in the draft report

- V and V1 are not scanned by any primary for control bytes, non-ASCII, patterns, hostname or the denylist.
- The messages of 729ab0e, c83e398 and 795a151 are not scanned by any primary.
- V1 mislabels itself as verify.log.
- V line 631 overstates what v6 shows.
- No fixture exercises a build-dependency or a target-specific dependency.
- The underscore-spelling gap from I1 is dropped.
- N's fixtures were workspace members.
- V v7's before-and-after changes two variables at once.
- Whether the doc commit holds all pending changes cannot be shown.
