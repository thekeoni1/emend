# M4 part 2 P2 independent check 1

Checked commit (the phase commit): 2c31aad90d5729ec6c37d41e8cc55f436f3fa69e
HEAD when the check ran: 2c31aad90d5729ec6c37d41e8cc55f436f3fa69e
Checker: a fresh general-purpose subagent (not a fork), given only the files below in <check>, where <check> stands for <home>/.cache/emend-build/m4p2-p2/check1. It read attempt 1 of run.log (cd3feb2f9ed574a0a5b9e5f5588f465c058287c2b207b53a43c1880e700a9f3b) and negative.log (bfeb6e51035b1e7afbbfd2b9ee483dace0f366d0ca7be0e5bec19a5537c333a7), superseded by specs/0031 "Planning rulings during P2" ruling 6 and kept outside the repository; check 2 reads attempt 2.
Inputs (sha256, name):
  63948b10cda1e9c28a56a4115b27aac85b0d3d5257844920acc2a86e6af0986d  diff-2c31aad.diff
  816acdc341891fc7c1230ac9f46f8d8a7fd5e77596e12ff7ac2a35c87f9c471b  diff-phase.diff
  9e11d94263cd917d5cc6cd650574cdb446ff1185e8a996fdff3c8206e875f14b  gate.log
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  gate.outer.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  gate.stderr
  bfeb6e51035b1e7afbbfd2b9ee483dace0f366d0ca7be0e5bec19a5537c333a7  negative.log
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  negative.outer.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  negative.stderr
  019fa0d81de5f66b73d7cee6f0d1631b96a751a568d7cf890f93f68d3139eb78  report-draft.md
  cd3feb2f9ed574a0a5b9e5f5588f465c058287c2b207b53a43c1880e700a9f3b  run.log
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  run.outer.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  run.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  scan-pre-check.stderr
  8746f6edb8d381dc02a73a3fe3dcab2a071353e7ddc25993e733837a6e2c0f79  scan-pre.log
  80e18c0a5f11dcc28233ad62398447597b76a73fccca740b370a314690fa2be8  size-m4p1-2026-10-10-669fa91.md
  ce2340fe8496bddede28a6e16509ac0afcdf9876fe6564a36e08d5d610cf5bf6  size-m4p2-2026-10-10-2c31aad.md
  a445c58d2d188baadf583179cd0d43354d541149aa8e204c5f9acc8d698e120c  size.log
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  size.outer.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  size.stderr
  f00739a49a66bf183dbc95e0c32169ab38cf46173557207660aef7c0bc3ee546  spec-0031-at-66fd35f.md
  01fef33d1271b3717a4502f579d6c5888b93f16c95d7649b931d7890340b85fe  spec-0031-working.md
  88a72d93a4dfd064f11581354bf877c24f85ff91d9c7d09741fc6f147ec2de73  specs-0001.md
  bc8c052b78d98190172b58095a8923ddc378278e75de0837a93adf4b80cab6c8  test.log
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  test.outer.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  test.stderr
Answer: 8830 bytes, sha256 f2d08eb137686abe8b73f3d4b01c239a3d0426c7e183cf4826848bc43c709820; plain ASCII, no machine path, committed verbatim (a final newline added).

---

I'm the independent checker. CLAUDE.md was in my context from the harness; I did not use it as evidence. I read only the files in the check1 directory. I ran sha256sum and grep over them and nothing else: no git, no cargo, no scripts. Every log hash, both diff hashes, both spec hashes and the size record's hash match what the report and the gate log state.

**1. Does the diff do what the spec says, and nothing else?**

Yes, with no defect in the diff itself.

- **diff-2c31aad.diff (the code commit):** one file, `crates/emend/Cargo.toml`. It adds 7 lines and removes 0. The gate log (lines 3878-3925) agrees: 7 added, 0 removed, in order.
- **diff-phase.diff (the phase diff):** besides the manifest, it changes only `ROADMAP.md` (P1 row to DONE, P2 row to IN PROGRESS) and `specs/0031`. The spec changes are the "Phases" sentence (P1 verification ruling 5), "Planning rulings during P1" and "Planning verification and rulings on P1". That is exactly what P1 verification ruling 8 says the doc commit holds.

The manifest, field by field:
- **Order:** `publish = false` comes first, then `description`. The existing `readme` line follows `description`, then `repository`, `keywords`, `categories`, then the `[package.metadata.docs.rs]` table. This is the spec's order plus P1 ruling 3.
- **description:** byte-identical to the spec's quoted sentence, on one line.
- **readme:** kept as a context line, unchanged.
- **repository:** `https://github.com/thekeoni1/emend`. The gate checks it against the remote with `.git` removed, with a control.
- **keywords:** five. Lengths are 4, 11, 5, 5 and 8, all under 20.
- **categories:** the two slugs.
- **docs.rs table:** `[package.metadata.docs.rs]` with `all-features = true`.
- **Nothing extra:** no `homepage`, `documentation`, `doc_cfg` or `license-file` is added.
- **Kept:** `publish = false` stays. `version = "0.1.0"` is outside the hunk and the run log reads it once. The `edition`, `license` and `rust-version` workspace lines are unchanged context.

One limit: the full manifest is not among my inputs. I cannot confirm that no `homepage` or `license-file` line existed before this phase, or that no `[package]` key sits after the hunk's trailing comment (it would now fall under the docs.rs table). The gate's "removes none", the 56-file package and the verify build passing make either unlikely.

**2. Do the logs contain every required line?**

Mostly yes.

- **Gate:** `CARGO_NET_OFFLINE=true` checked with controls, nine PASS lines, the gate-file listing (10 checked, 0 in the diff), the manifest-order check with controls, and "gate log checks all as expected: yes".
- **Test:** 514 off, 526 copy ("on"), 558 comments ("cm"), 573 pair and 918 all, on both targets, each equal to P1's figure and part 1 P3's figure.
- **Size:** "0 of 28" (line 7606), with controls at plus and minus 1. The record is installed with sha256 ce2340fe and is byte-equal. The change list NA to NH is printed, with NF covering ruling 5's build set. The budget is 436992.
- **Run:** all three packaging commands, each with exit and output.
  - The list comparison holds 56 names, with controls for one more, one fewer and a duplicate.
  - The .crate path, 522253 bytes, sha256 0f1b2ffb, and its tar listing equal to item 1's.
  - Ruling 1's archive finding (54 against 56).
  - The worktree diff of exactly one line.
  - Under ruling 2: the abort line counted once, other warning lines 0, with a control.
  - The network statement (two "Updating crates.io index" lines), ruling 4's sentence, `git worktree remove --force` after a path check, and `rm -rf "${PUB:?}"`.
  - The src diff: 40 lines changed, 0 that are code, with a planted-line control.
  - The listing: 544 rows; equal to the record (544 of 544, with a control); equal to part 1's listing on every column but line; 18 moved rows, all in `lib.rs`, by its growth of 8 (76 to 84 lines), with controls.
- **Negative:** NG1 prints cargo's text "error: `emend` cannot be published." (exit 101). NG2 prints "readme `../../NO-SUCH-README.md` does not appear to exist" (exit 101). Both have controls.
- **Scan:** `RESULT: PASS`, structural checks failed 0, placeholders all declared, removals all in the ruled form, and the three shape greps at 0.
- **specs-0001 rules:** I found no absolute machine path and no non-ASCII byte. Scripts are printed. Every log compares the phase commit with HEAD.

The defects and doubts in the logs are listed at the end, most serious first.

**3. Does every number, hash and quoted string in the report appear in a primary?**

Every count, hash, subject and quoted log string I checked matches a primary. I could not find these anywhere among my inputs:

- The superseded script hashes bd5657661c48... and fda251ee02ff..., and their paths `<phase>/scripts/superseded-run/...` and `superseded-size/...`. They appear in the report alone.
- The scan log's location `<phase>/scan/scan-pre.log`. The log only shows the script at `<phase>/scripts/m4p2-p2-scan.sh`.
- D3's statement that the README's first sentence reads "change, hand the file back" without "and". The README is not an input. The `lib.rs` doc comment in the run log's src diff does read that way, which supports it indirectly.
- D1's "the publish dry run on a git archive copy failed": in no log (see doubt 3 below). Its only source is ruling 1's own wording.
- The disclosures in D5 and D6: no primary, by their nature.

Where the report differs from the files:
- In run item (3), the report names only `template/_rels/.rels` as missing from the archive copy. The run log names two missing entries, `.cargo_vcs_info.json` and `template/_rels/.rels` (56 - 54 = 2). Ruling 1's phrase "the one missing path" has the same gap.
- The report says the gate log states "no prune". That appears only in a comment of the printed script (gate.log line 118). No output line of the gate says it.

**Defects and doubts, most serious first**

1. **run.log lines 3360-3361.** The status labels still read "this phase's logs or planning's two files" and "planning's uncommitted spec and ROADMAP.md". P1 verification ruling 3 said P2's scripts would "label the pattern by what it admits". This log's own line 70 says the pattern refuses `ROADMAP.md`, and one of the two status lines is the untracked size record. The same script labels the pattern correctly at line 56. The result itself stands.
2. **The P1 target was not pruned.** specs-0001 ("One target root per phase") says the previous phase's build targets are listed with `du -sb` and removed at the next phase's first gate that builds. P2's gate builds, yet there is no prune and no `du -sb` of `<home>/.cache/emend-build/m4p2-p1/target`. The only justification is a script comment ("the previous phase pruned everything due", gate.log line 118). specs/0031 names only P1's prune. Planning should rule whether P1's own target was due.
3. **The archive-copy verify-build failure is asserted, not measured.** In ruling 1's finding, the run log's FINDING line says the archive copy's "verify build fails". The log measures only the package list (54 names), never that failure (exit 101, "couldn't read"). Under specs-0001 ("Nothing is called measured unless the log prints the measurement"), the failure rests on unlogged dry runs, which D4 discloses.
4. **The "no other command of this phase reached the network" claim is true of logged runs only.** The run log's network line says no other command of this phase did, and the spec says "nothing else of this phase did". The unlogged dry runs also reached the network (D5). The log's sentence should be scoped to logged runs.
5. **negative.log lines 2259-2261.** The label says "this phase's logs or planning's spec: 2", but one of the two lines is the untracked size record. This is the same labelling issue as ruling 3.
6. **The description sentence contradicts the spec's own source note.** The spec calls the description "the README's first sentence", but its quoted text adds "and" (report D3). The manifest follows the quoted text. Planning should decide which wording holds.
7. **The negative tests run in a `git archive` copy.** specs-0001 asks for a `git worktree` or a clone. Ruling 1 shows the archive copy packages 54 files, not 56. NG1 and NG2 are unaffected, but it is a departure from the stated form, though it may be an inherited practice (change LD).
8. **The HEAD-comparison controls in gate.log (lines 87-88) and size.log (lines 45-46) refuse the null commit and a short sha, not a near-miss fabricated commit.** test, run and negative each refuse a fabricated commit with one hex digit changed. The null commit arguably counts as fabricated.
9. **Minor: the planted-manifest diff in negative.log (lines 2203-2204) prints local timestamps with a "-0700" timezone.** It is not a forbidden item, just noted.

I found no other defect.
