# M4 part 2 P2 independent check 2

Checked commit (the phase commit): 2c31aad90d5729ec6c37d41e8cc55f436f3fa69e
HEAD when the check ran: 2c31aad90d5729ec6c37d41e8cc55f436f3fa69e
Checker: a fresh general-purpose subagent (not a fork), given only the files below in <check>, where <check> stands for <home>/.cache/emend-build/m4p2-p2/check2: attempt 2 of run.log and negative.log (specs/0031 "Planning rulings during P2", rulings 6 to 8), the kept gate, test and size logs, check 1's answer and the superseded attempt 1 logs.
Inputs (sha256, name):
  63948b10cda1e9c28a56a4115b27aac85b0d3d5257844920acc2a86e6af0986d  diff-2c31aad.diff
  816acdc341891fc7c1230ac9f46f8d8a7fd5e77596e12ff7ac2a35c87f9c471b  diff-phase.diff
  9e11d94263cd917d5cc6cd650574cdb446ff1185e8a996fdff3c8206e875f14b  gate.log
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  gate.outer.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  gate.stderr
  f2d08eb137686abe8b73f3d4b01c239a3d0426c7e183cf4826848bc43c709820  independent-check-1-answer.md
  f37cdd615cba61906f9f272b4d952c4d54ca19f525c395f161897abedf1f231f  negative.log
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  negative.outer.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  negative.stderr
  9512eef4d046a8bde04c6a0788caf8a3fd4f7e632db4e465ebf8879d1fc96481  report-draft.md
  3af3524cef4928e83ee89cd0c4f8a189ed27e488e146845deca6e8e6e4e5cef5  run.log
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  run.outer.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  run.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  scan-pre-check.stderr
  86df0849780e229c5c60a9e6a89174bac19b50b2daa0dbf488e42ab687676f50  scan-pre.log
  80e18c0a5f11dcc28233ad62398447597b76a73fccca740b370a314690fa2be8  size-m4p1-2026-10-10-669fa91.md
  ce2340fe8496bddede28a6e16509ac0afcdf9876fe6564a36e08d5d610cf5bf6  size-m4p2-2026-10-10-2c31aad.md
  a445c58d2d188baadf583179cd0d43354d541149aa8e204c5f9acc8d698e120c  size.log
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  size.outer.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  size.stderr
  f00739a49a66bf183dbc95e0c32169ab38cf46173557207660aef7c0bc3ee546  spec-0031-at-66fd35f.md
  e6b80fc15201e901423230a7e2768e7900cba9b60163c3726bb1a5f9f6db9d24  spec-0031-working.md
  88a72d93a4dfd064f11581354bf877c24f85ff91d9c7d09741fc6f147ec2de73  specs-0001.md
  bfeb6e51035b1e7afbbfd2b9ee483dace0f366d0ca7be0e5bec19a5537c333a7  superseded-negative-attempt1.log
  cd3feb2f9ed574a0a5b9e5f5588f465c058287c2b207b53a43c1880e700a9f3b  superseded-run-attempt1.log
  bc8c052b78d98190172b58095a8923ddc378278e75de0837a93adf4b80cab6c8  test.log
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  test.outer.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  test.stderr
Answer: 6934 bytes, sha256 ffbc75f3e2bad25c0ec3677290582b4186b250f64e24816c041705d23e7542e9; plain ASCII, no machine path, committed verbatim (a final newline added).

---

I read only the files in `<home>/.cache/emend-build/m4p2-p2/check2/`. CLAUDE.md was in my context from the harness, and I did not use it as evidence. I used only cat, sed, grep, diff, wc and sha256sum. I did not run git, cargo or any script, and I wrote nothing.

Every hash the report states for the inputs matches the files: the six logs, the size record, both spec versions, and the two attempt-1 logs (cd3feb2f and bfeb6e51). The line counts also match: 4590, 1224, 9551, 3435, 2285 and 3248. All eleven stderr files are empty.

**1. Does the diff do what the spec says for P2, and nothing else?**

Yes. I read both diffs.

- **diff-2c31aad.diff (the code commit):** one file, `crates/emend/Cargo.toml`, 7 lines added and 0 removed.
  - The order is `publish = false`, then `description`, then the existing `readme` line, then `repository`, `keywords` (five), `categories` (two slugs), a blank line, and `[package.metadata.docs.rs]` with `all-features = true`.
  - The `description` is byte-equal to the spec's quoted sentence, as ruling 10 settles.
  - Nothing else is added: no `homepage`, `documentation` or `doc_cfg`. `publish = false` is kept.
- **diff-phase.diff (5640bfa..2c31aad):** besides the manifest, it changes `ROADMAP.md` (the P1 row set to DONE, the P2 row to IN PROGRESS) and `specs/0031`.
  - The spec changes are the "Phases" sentence, "Planning rulings during P1" and "Planning verification and rulings on P1". That is what P1 verification ruling 8 says the doc commit holds.
  - Running diff between spec-0031-at-66fd35f.md and spec-0031-working.md shows only the added "Planning rulings during P2" section (rulings 1 to 11).
- **One limit:** the full manifest is not among my inputs, so I cannot see keys outside the hunk.

**2. Do the logs carry the rulings and specs-0001's rules?**

Mostly yes, with one defect (D1 below).

- **Ruling 6 (labels):** done in both logs.
  - The labels now read "(this phase's logs, its size record, planning's modified specs/0031)": in run.log at lines 61 (entry), 3427 and 3428 (end), and in negative.log at lines 59, 2272 and 2273.
  - No output line in either rerun still says "planning's two files", "planning's uncommitted spec" or "planning's spec". The old wording survives only in the printed carried history (run.log 233, negative.log 299 and 367), which is acceptable.
- **Ruling 7 (the archive copy's verify build):** done, in run.log at lines 2480 to 2498.
  - `cargo package -p emend --offline` on the git archive copy: exit 101, 12 lines on standard error, the line "error: couldn't read `src/../template/_rels/.rels`" counted as 1, and a control.
  - The worktree control (lines 2521 to 2529) exits 0.
  - The FINDING (line 2498) says "Measured here" and names both missing entries: `template/_rels/.rels` as the finding and `.cargo_vcs_info.json` as expected.
- **Ruling 8 (the network sentence):** done. Line 2533 reads "no other logged command of this phase reached the network". negative.log (2132), test.log (8), size.log (61) and gate.log (39 to 41) each print CARGO_NET_OFFLINE=true.
- **Naming attempt 1 by hash:** done, as specs-0001 requires of a rerun.
  - run.log names it at lines 3 and 37 to 41: cd3feb2f, script 0be2c514, re-hashed by sha256sum at run time.
  - negative.log names it at lines 3 and 37 to 41: bfeb6e51, script fab2ca67.
- **Check 1's other points:** B is ruled by ruling 9, E by ruling 10, and F (the git archive copy in the negative log, the HEAD-guard control forms, the timestamps) by ruling 11. A, C and D are fixed.
- **specs-0001 rules I can check from the text:**
  - No non-ASCII byte, no control byte and no absolute path in run.log, negative.log or the report.
  - The phase commit is compared with HEAD, with a control that refuses a fabricated commit.
  - The scripts are printed, the shape greps run before each log, and `rm -rf` is written as `"${PUB:?}"` and `"${B:?}"`.
  - The scan pre-check hashes both reruns at the right hashes and ends "RESULT: PASS" with "structural checks failed: 0".

**3. Does every number, hash and quoted string in the report appear in a primary?**

Every count, hash and quoted log string I checked appears in a primary, apart from the items below.

- **Not found in any primary:**
  - The attempt-1 scan pre-check hash 8746f6ed... (report line 17).
  - The superseded script hashes and paths bd565766... and fda251ee... (line 26).
  - D1's claim that the *publish dry run* on a git archive copy failed. The logs measure `cargo package` on that copy, not `cargo publish --dry-run`.
  - The disclosures in D5 and D6. These have no primary by their nature.
- **Where the report differs from the files:** D1, D2 and D3 below.

**Defects and doubts, most serious first**

- **D1. negative.log carries a script change that its rerun note and change list do not name.**
  - Attempt 1's script has `for f in "$B"/*.out; do ALL+=("$f"); done`. Attempt 2's (negative.log line 1823) has `for f in "$B"/*.out "$B"/*.err; do ALL+=("$f"); done`.
  - As a result, "captured outputs" goes from 4 (superseded-negative-attempt1.log) to 8 (negative.log line 2265).
  - The rerun note (lines 3 and 37) says the attempt only corrects the labels (change LE) and that "its cases are unchanged". Change LE describes the labels alone.
  - specs-0001 says to "re-run with the fix named in the log". The fix itself is right, since attempt 1 never checked the captured standard error for control bytes, but it is undisclosed. The report says nothing about it either.
  - The run script's code changes, by contrast, all match its change KM.
- **D2. Report line 12 still says the gate had "no prune (P1 pruned everything due)".** Ruling 9 says the P1 target *was* due and calls this planning's error. The sentence contradicts the ruling the report cites in its own D0, and it still rests only on a script comment (gate.log line 118).
- **D3. Report D0 says check 1's answer was "committed as independent-check-1.md".** Nothing is committed yet: the report's own line 10 says "not yet committed", and the scan pre-check says "check answers present: 0" (lines 3099 and 3100). It should read "to be committed" or "saved".
- **D4. Report D4 says "No logged run was repeated."** run.log and negative.log were both repeated as attempt 2. The sentence needs scoping to the scripts superseded before any logged run.
- **D5. Ruling 7's control is not exactly "the same command".**
  - It adds `--allow-dirty` and runs after `publish = false` was removed (run.log line 2521).
  - The FINDING's "packages the file" rests on "Packaged 56 files" and exit 0. The worktree's own package list is not listed.
  - This is minor.
- **D6. gate.log, test.log and size.log cite the spec at 01fef33d (rulings 1 to 5), not e6b80fc1.** Ruling 6 ("gate, test and size stand") covers this and the report discloses it. Noted only.
- **D7. The final scan must cover both check answers.** The pre-check scanned none.

I found no other defect.
