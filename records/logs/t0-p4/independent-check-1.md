Checked phase: T0 P4 (specs/0001-bootstrap.md)
Checked commit: a713c93b080b415aa9c39d51c327b930445ad2e7 (P4 phase commit); logs in 980b9e3397bbb85e00189948dc4dc8c993833fe3
HEAD when the check ran: 980b9e3397bbb85e00189948dc4dc8c993833fe3
Checker: a fresh general-purpose subagent, not a fork
Inputs (sha256):
- specs/0001-bootstrap.md: ccc54ed25414850cd471a6ecc70772ea41989e8cd30677dfe378b81853ef0800
- <scratch>/p4-diff.txt (git diff 53b99d8626775d71a5c608b9639646544793d189..a713c93b080b415aa9c39d51c327b930445ad2e7): 5fbb071b752cb2a7d2cf12b6cf5fb4e3262cf90e6df336a02f508fc6791285b6
- records/logs/t0-p4/gate.log: 8159e2a57caa2bc835beb8091e7b731a5ded0004056be72b31fee8ada22b64fb
- records/logs/t0-p4/negative.log: fa52b5ef16a1b5fd83f283f19ff941690d98619f38ed43dd9b88f30854fbef85
- records/logs/t0-p4/negative-attempt1.log: c8e9b4ad01fd3ad8cfdd2d297c446e5b8722ed2aa326952efb9a54eae6e0ddd4
- records/logs/t0-p4/scan.log: 9ef0734a495d398c26979c513fa38f695e80a609fdb65074068176da9c682b46
- <scratch>/p4-report-draft.md (draft 1): d333af06e420c3e975a1866ae62c653e9562544f30083d6a3742c8a51b35d0f9
---
# T0 P4 independent check 1

Harness material: the harness put more into my context than the seven inputs. That material was CLAUDE.md, an auto-memory index, a git status snapshot with recent commits, user identity details and a commit attribution reminder. I did not use any of it as evidence, and I do not repeat the identity details. I read only the seven inputs and ran no commands. Everything I say about how the code behaves comes from reading it, not from running it.

Labels: "the diff" is the phase diff. "The draft report" is the draft. The logs are cited by path: gate.log, negative.log, negative-attempt1.log and scan.log, all under records/logs/t0-p4/.

## Q1. Does the diff do what the P4 spec says and nothing else?

PARTLY. The diff has the steps, the empty allowlist, colour off and the doc commit. I found no defect in the licence parser against the spec's rules. There are two small departures from the spec text, one gap the spec does not name, and several branches that no primary exercises.

### Doc-commit ruling

- scan.log lines 247-253 show 977635b as the first commit after 53b99d8, touching only ROADMAP.md and specs/0001-bootstrap.md.
- scan.log scans both files (lines 266, 290) and the commit's message (line 310).
- The spec hunks in the diff (lines 184-274) match the spec text I was given: the status line, the control-byte rule, "Planning rulings on P3", the ignore.log line and the colour-off paragraph.
- The primaries cannot show "unedited". Nothing records the working tree before the commit. The draft report says so.

### Colour off

- The diff line 34 exports `CARGO_TERM_COLOR=never`.
- The diff line 46 runs `cargo fmt --all --check -- --color never`.
- Both match the spec. That covers cargo, clippy and rustfmt output. python and awk emit no colour.
- The test harness inside `cargo test` gets no explicit colour flag. `CARGO_TERM_COLOR` does not reach it, so it depends on whether its output is a terminal. No primary shows test-failure output after this change: there is no FAIL host rerun. I call this a weakness, not a spec violation.
- The diff does not contain `quiet()` or `fail()`, so how they capture output cannot be seen from the diff.

### Step 5 native (the diff lines 55-75)

- **Command.** The spec's command gets `--prefix none` added (the diff line 58). This departs from the spec text. The draft report flags it. It looks harmless: gate.log line 107 and negative.log lines 303-306 show one crate name first on each line.
- **Banned crates.**
  - cc, cmake and pkg-config are matched as exact package names (the diff line 62). The spec says the tree "contains" them. Exact names are the sensible reading, and the draft report flags it.
  - The draft report's Finding 2 says "a renamed wrapper would not be caught". I disagree with that as written. `-e normal,build` applies at every depth, so a wrapper crate that depends on cc still puts cc in the tree. A dependency renamed with `package = "cc"` is printed by its package name. Only a crate that vendors cc's code, or is named `pkg_config` with an underscore, gets past. That is my reading of cargo, not something a primary shows.
- **-sys crates.**
  - A crate counts as `-sys` if its name ends in `-sys` (the diff line 67), and it is checked against the allowlist by its first field (the diff line 68).
  - Blank and `#` lines in the allowlist are ignored. That is an addition the spec does not ask for. It is harmless.
  - A missing allowlist fails (the diff line 57), which is also an addition.
  - An empty allowlist makes awk exit 1 (`exit !found`), so every `-sys` crate is reported. That is correct.
- **Listing.** The spec says "List every crate whose name ends in -sys". The step prints only the unlisted ones, and only on failure. This is a departure, and the draft report's Finding 4 flags it.
- **Can it pass when it should fail?** A non-zero exit from cargo tree fails, and stderr is kept apart. I found no bypass within the spec's scope, with two exceptions:
  - A dependency behind a non-default Cargo feature. cargo tree resolves default features only unless `--all-features` is given, so such a dependency never appears in the tree.
  - An underscore spelling, as above.

  The spec does not ask for all features. Given that heavier dependencies are meant to sit behind features that are off by default, planning may want to rule.
- **Untested branches.** No primary exercises:
  - the cc/cmake/pkg-config branch;
  - a `-sys` crate on the allowlist passing;
  - a missing allowlist failing.

  The spec's negative list does not require them, but the banned-name branch is a core part of the step and has never been seen to fire.

### Step 6 licences (the diff lines 77-176)

- **Metadata and packages.**
  - `cargo metadata --format-version 1` has stdout and stderr kept apart (the diff line 80), and every entry in `packages` is checked (the diff lines 154-169).
  - Workspace members are included: negative.log line 192 shows xlsx-diff.
  - The metadata docs, as I recall them, say `packages` holds only dependencies enabled by the selected features. That is the same non-default-feature gap as step 5. No primary shows it either way.
- **Allowed identifiers.** The eight are exactly the spec's (the diff lines 87-96). `Apache-2.0 WITH LLVM-exception` is matched as a whole string (the diff line 145; Python's `+` binds before `in`, so it is correct).
- **Parser.** The grammar is `any_of := all_of (OR all_of)*`, `all_of := term (AND term)*`, and a term is a parenthesised expression or an identifier with an optional `WITH` exception. That gives AND tighter binding than OR, which is SPDX precedence.
  - Both sides are parsed before `or`/`and` combine them (the diff lines 119-120, 127-128). Short-circuiting therefore cannot skip tokens, and trailing garbage is caught at the diff lines 149-150.
- **Expressions I traced by hand.** Each gets the right verdict under the spec's rules:
  - Fail:
    - `MIT AND ISC`
    - `ISC OR MIT AND ISC`, which parses as ISC OR (MIT AND ISC)
    - `OR MIT`, `MIT OR`, `MIT MIT`, `()`, `((MIT)`, `MIT)`
    - `(MIT) WITH X`, which ends with "unexpected WITH"
    - `MIT WITH LLVM-exception`, `MIT WITH (`
    - `MIT/Apache-2.0`, `mit`, `MIT or ISC`
    - an all-whitespace string, which ends with "unexpected end"
  - Pass:
    - `ISC OR MIT`
    - `MIT AND (ISC OR MIT)`
    - `Apache-2.0 WITH LLVM-exception OR MIT`
    - `MIT OR Apache-2.0 AND ISC`, which passes through MIT and is correct under SPDX precedence
- **Missing licence.** An absent, null or empty licence fails (the diff line 159). A crate with only `license-file` also fails, which the spec wording supports.
- **Possible "should pass but fails".** Two cases reject real SPDX expressions:
  - SPDX matches licence identifiers without regard to case, so `mit` or `apache-2.0` is valid SPDX but fails here.
  - The old slash form fails too.

  Both fail closed, which fits "any other identifier fails and is reported for a planning ruling". The draft report mentions lowercase operators and the slash form, but not lowercase identifiers.
- **Can it pass when it should fail?**
  - A missing python3 or a Python exception exits non-zero and so fails (the diff line 82).
  - A licence string containing a newline would shorten the FAIL reason, because it takes `tail -n 1` (the diff line 174), but the step still fails.
  - I found no pass-when-should-fail path.
- **What the primaries exercise.** Only MPL-2.0, no licence, `MIT AND ISC`, `ISC OR MIT` and `MIT OR Apache-2.0`. No primary exercises WITH, parentheses, the syntax-error path, or an AND whose parts are all allowed passing. The draft report's 18-expression test is not in a primary.

### Other files in the diff

Nothing. scan.log lines 262-265 list exactly four paths.

A small mislabel outside the diff: the scan.log script's header comment (scan.log line 8) says "the other P3 logs". It scans P4 logs, as line 168 shows.

## Q2. Do the logs contain every line the P4 spec requires?

YES. Every required line is there. The weaknesses and mislabels below do not remove a required line.

### Required lines

- **gate.log** has the six PASS lines (lines 89-94), `exit: 0 (expect 0)` (line 96) and the six-PASS-in-order check (line 97).
  - The run is a full one with no `--quick`, from `/` by absolute path, after `cargo clean` (lines 85-87).
  - `RUSTUP_AUTO_INSTALL=0` is exported in the script and shown at line 82.
- **negative.log case a:** PASS native (line 189), `FAIL licences: fake-mpl 0.1.0 (MPL-2.0)` (line 190), exit 1 (line 194).
- **Case b:** `FAIL licences: fake-nolicence 0.1.0 (no licence)` (line 219), exit 1.
- **Case c:** `FAIL licences: fake-mit-and-isc 0.1.0 (MIT AND ISC)` (line 248), exit 1.
- **Case d:** `PASS licences` (line 277), exit 0 (line 278), no FAIL line (line 282).
- **Case e:** `FAIL native: -sys crate not in scripts/sys-allowlist.txt: fake-sys` (line 302), exit 1.
- **fmt rerun (case f):** `FAIL fmt` naming `--color never` (line 319), with `count=0 grep exit=1` for control bytes in the captured output (line 330). The ESC-and-SI and tab-and-CR controls are at lines 168-169.
- **scan.log:**
  - `#!/usr/bin/env bash`, `/usr/bin/grep`, `LC_ALL=C`, `-P '[^\x00-\x7F]'`, and U+2014 as printf bytes `\342\200\224` with `-F`;
  - errors (exit 2 or more) are counted separately (script line 57);
  - every form has a control using the same command form (lines 197-242);
  - the script is printed;
  - the four files, both messages and the other three logs are scanned, with `RESULT: PASS` (line 351).

### Rules for every phase

- **Toolchain rule:** gate.log script line 16, negative.log script line 24, negative-attempt1.log script line 17. scan.log runs no cargo.
- **Control-byte rule:** scan.log counts control bytes in gate.log, negative.log and negative-attempt1.log (lines 322-345), with controls at lines 198-199, and finds 0. scan.log does not scan itself. The P1 ruling accepts that.
- **Log privacy:**
  - paths are written `<repo>` and `<scratch>`;
  - hostname, P6 patterns and denylist all have 0 hits across the three logs (scan.log);
  - no hostname or `uname` output is visible.
- **First-line rule:** all four logs name the commit and the HEAD at run on line 1.

### Weaknesses and inconsistencies

1. **The colour-off rerun cannot tell before from after.** No P4 primary runs the same fmt case without `--color never` in the same environment. So "zero control bytes" does not show the flag is what removed them. The "before" is only the P3 record.
2. **Line counts are labelled as byte counts.**
   - `f_ctrl` uses `grep -c`, which counts lines, not bytes. The ESC-and-SI control holds three control bytes on one line and prints `count=1` (gate.log line 100, negative.log line 168, scan.log line 198).
   - A zero is still correct. But the logs label these numbers "control bytes" and "count", and the spec says "counts control bytes".
3. **Case f has no absent check.** Both of its checks are present-checks (negative.log lines 332-333); cases a to e each check a wrong outcome is absent.
4. **negative-attempt1.log is mislabelled.** Its line 2 calls itself `negative.log`, and its script comment says the output is moved to negative.log. Only negative.log line 4 identifies it as attempt 1.
5. **Recording the failure.** The spec says to record the failure in the phase log, fix it, and re-run with the fix named. negative.log lines 4-10 do that, and every script change between the two runs is either named or cosmetic:
   - lib.rs goes from empty to one doc-comment line (negative.log script line 77 against negative-attempt1.log line 63);
   - the Diff pattern `^Diff in ` becomes `^ +Diff in ` (line 157 against 143);
   - the status pathspec changes (lines 162-163 against 148-149);
   - header echoes and labels change.

   Two caveats:
   - Fix 3 did not respond to an observed failure. negative-attempt1.log line 310 shows the tree check empty, yet negative.log lists it under "Cause and fixes". It also weakens the tree check, which now ignores anything under records/logs/t0-p4/.
   - Fix 1's cause ("wants a trailing newline") is the implementer's reading. negative-attempt1.log lines 174-176 show only a diff at lib.rs:1 with two `+` lines.
6. **gate.log and the first negative run were apparently concurrent.** gate.log line 2 and negative-attempt1.log line 2 carry the same timestamp, 2026-09-15T00:44:36Z, even though gate.log did a cold full build. No required line depends on it, but no log mentions it.
7. **gate.log line 3 is fixed text.** "working tree clean before the run: yes" is a constant echo, backed only by the exit-2 guard at script line 22. That is acceptable because the script would have exited.
8. **No log checks feature-gated dependencies.** No required line covers them either (see Q1).

## Q3. Does every number and quoted string in the draft report appear in a primary?

PARTLY. Most values match a primary exactly. One claim is false, one is inaccurate, and several claims are not marked as coming from outside the primaries.

### Values that match a primary

| Draft report item | Primary |
|---|---|
| Range `53b99d86...a713c93b...` | scan.log lines 1, 246 |
| 977635b63e8947060bb99136eed36b901143825f and its subject "T0 P4: commit planning doc updates (P3 rulings)"; parent 53b99d8; M ROADMAP.md, M specs/0001-bootstrap.md | scan.log lines 248-253 |
| a713c93b080b415aa9c39d51c327b930445ad2e7 and its subject "T0 P4: gate steps native and licences, colour off"; parent 977635b; M scripts/check.sh, A scripts/sys-allowlist.txt | scan.log lines 247, 255-258 |
| Allowlist sha256 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855, and zero bytes | scan.log line 282; gate.log line 80 (`bytes 0`); the diff line 177-179 |
| gate.log sha256 8159e2a57caa2bc835beb8091e7b731a5ded0004056be72b31fee8ada22b64fb | scan.log line 322 |
| negative.log sha256 fa52b5ef16a1b5fd83f283f19ff941690d98619f38ed43dd9b88f30854fbef85 | scan.log line 330 |
| negative-attempt1.log sha256 c8e9b4ad01fd3ad8cfdd2d297c446e5b8722ed2aa326952efb9a54eae6e0ddd4 | scan.log line 338 |
| Colour-off lines | the diff lines 34, 46 |
| cargo tree command | the diff line 58 |
| Exact names | the diff line 62 |
| `-sys` crates and allowlist | the diff lines 67-74 |
| Missing allowlist | the diff line 57 |
| Allowlist created empty | scan.log lines 258, 282; gate.log line 80 |
| Eight identifiers | the diff lines 87-96 |
| OR, AND, parentheses, WITH | the diff lines 115-146 |
| Empty licence | the diff line 159 |
| FAIL reason format | the diff lines 160-171; negative.log line 190 |
| Cold cache, from `/` | gate.log lines 85-87 |
| Six PASS lines | gate.log lines 89-94 |
| `exit: 0 (expect 0)` | gate.log line 96 |
| Six PASS in order | gate.log line 97 |
| `stderr bytes: 0` | gate.log line 95 |
| Controls and stdout/stderr counts | gate.log lines 100-103 |
| sha256 on disk and at a713c93 | gate.log lines 79-80 |
| Clean before and after | gate.log lines 3, 111 |
| Tree and metadata lines | gate.log lines 107, 109 |
| Case a | negative.log lines 189-190, 194 |
| Case b | negative.log lines 219, 223 |
| Case c | negative.log lines 248, 252 |
| Case d | negative.log lines 272-278, 282 |
| Case e | negative.log lines 284, 291, 302, 307 |
| Case f FAIL line | negative.log line 319 |
| Case f control-byte string | negative.log line 330 (a prefix of that line) |
| `all negative cases as expected: yes` | negative.log line 337 |
| "46 controls" | Not printed anywhere. I counted the control lines at scan.log lines 197-242 and got 46. |
| `controls: all as expected` | scan.log line 243 |
| `files in diff: 4; files scanned: 4; files read: 4` | scan.log line 298 |
| `messages scanned: 2; commits in range: 2` | scan.log line 319 |
| Both messages end with the Co-Authored-By line | scan.log lines 302, 311 |
| `logs scanned: 3 (expected 3)` | scan.log line 346 |
| Totals and RESULT | scan.log lines 348-351 |
| Findings 1a to 1d, including `Diff in <scratch>/emend/neg/fake-mpl/src/lib.rs:1:` | negative.log lines 4-10; negative-attempt1.log line 174; both runs at a713c93 per line 1 of each |

- **Diff stat "2 files changed, 61 insertions(+), 6 deletions(-)".** This string is in no primary, and the draft report marks it as session output. The numbers do agree with the diff's hunks for the two doc files. ROADMAP.md is +3/-3. specs/0001-bootstrap.md is +4+6+42+1+5 = +58 and -3. The total is +61/-6.

### Correctly marked as not in a primary

- record commit 980b9e3397bbb85e00189948dc4dc8c993833fe3;
- scan.log sha256 9ef0734a495d398c26979c513fa38f695e80a609fdb65074068176da9c682b46;
- "unedited";
- the pre-commit scan;
- the 18 synthetic expressions;
- committed log hashes equal to the working-tree hashes;
- no install;
- clean tree, no remote, nothing pushed.

### Wrong or inaccurate

1. **False: the guard in "the printed G, N and S scripts".** The draft report's line 150 says the primaries show the `RUSTUP_AUTO_INSTALL=0` guard in the printed G, N and S scripts. The scan.log script (scan.log lines 6-188) never sets `RUSTUP_AUTO_INSTALL`. It runs no cargo, so it needs none, but the claim is false for S. The guard is in gate.log line 16, negative.log line 24 and negative-attempt1.log line 17.
2. **Inaccurate: "each" case checks a wrong outcome absent.** The draft report's lines 84-85 say this of every case. Case f has no absent check (negative.log lines 332-333). "the indented `Diff in` lines" is also plural where there is one line (negative.log lines 320, 333).

### Claims not marked as outside the primaries

- "I reviewed both files before committing and found nothing wrong" (the draft report's line 26).
- `--prefix none` "keeps the output ASCII (no tree-drawing characters)". No primary shows output without it.
- Finding 5, "cargo metadata ... includes dev-dependencies". No primary has a dev-dependency.
- Finding 1's cause, "rustfmt's check wants a trailing newline". negative-attempt1.log lines 174-176 show two `+` lines, not a proven cause.
- Finding 2's "a renamed wrapper would not be caught". This is unsupported and, as argued in Q1, probably wrong.

### Surprises the draft report leaves out

- gate.log and negative-attempt1.log carry the same start timestamp.
- Fix 3 was not prompted by an observed failure.
- The colour-off rerun has no before/after comparison in P4.
- The line-count versus byte-count labelling.
- negative-attempt1.log's line 2 calls itself negative.log.
- No primary exercises the cc/cmake/pkg-config branch or the WITH and syntax-error paths.
- Dependencies behind non-default features escape both steps.
