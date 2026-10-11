# M5 part 1 P1 independent check 1

Checked commit (the phase commit): c8b9391fc20fd9c6fed72fe2d4a4fc5ce9411f6a
HEAD when the check ran: c8b9391fc20fd9c6fed72fe2d4a4fc5ce9411f6a
Checker: a fresh general-purpose subagent (not a fork), given only the files below in <check>, where <check> stands for <home>/.cache/emend-build/m5p1-p1/check1. It read scan-pre.log, the scan's pre-check; the final scan.log runs after this answer.
Inputs (sha256, name):
  0a18b0013ffdeba32d1d9921ac6cc7878e02c2febfc0f2ddc6de16248a8fc107  diff-0b3e46f.diff
  54a688980860f3cfb866f0b775509f232cf28dc6fbee65039eacdbf32ea535b8  diff-c8b9391.diff
  0fa7e5a6282dfb4aa039370d6f5d228975f88e3da1744453b130294357f29605  diff-cd30099.diff
  e20437eea614fbb2bb20c74aaf2a4d1ad1980cf8d0411be010bf20eeb1c86c4b  diff-phase.diff
  525b6d00370ca057c9601d99082f6666914153cad644774a6bf90fa0e3b4b784  gate.log
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  gate.outer.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  gate.stderr
  ed5da088cf5598e9f8a12e6dabc21f7332572a35eb1cf8fba9cbab2362e3b4e1  m5p1-open-2026-10-11-c8b9391.md
  b2d1a74bd78f49821084f0ba3f47d89dc2f4dca4fc64df0c948af0c2bcaed763  m5p1-p1-2026-10-11-c8b9391.md
  85af306bcb2880f3002ffae7a8c55707e9d37d9ad85a352fbb0e0071031d65cf  negative.log
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  negative.outer.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  negative.stderr
  28fff397d57cb628316ad04c49bc6fa98aae9bf6eb38efd493bd1358ff3830cd  report-draft.md
  18ffd2968e0332cf405a3f61497559afcfe72a96dd72b79e5aad3eb515c5c1ce  run.log
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  run.outer.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  run.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  scan-pre-check.stderr
  43ece043f9efcf6cee793f21ca827bb2f590485877abc6c2c6d8a1ee3e466edd  scan-pre.log
  9f9c12c93a36bf040a9229b314b47733c8390273ca601c096b76e5ee3f8a25bd  spec-0032-at-cd30099.md
  62218f3c1a4774c7d2e108ae8d87f2aec03458b36a371741175ac1e2f8dbde67  spec-0032-working.md
  88a72d93a4dfd064f11581354bf877c24f85ff91d9c7d09741fc6f147ec2de73  specs-0001.md
  337b10369ced9fd0434d198d23353a0116f4f1dcabddbcfd2805e5d4bb698906  test.log
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  test.outer.stderr
  e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  test.stderr
Answer: 8481 bytes, sha256 e1cbc6eff8c61ab7ee0b796963f1c3ea89baf05e805d8f6cd5e0d572fdc24422; plain ASCII, no machine path, committed verbatim (a final newline added).

---

I computed sha256 over every file in `<check>` and read all of them except the empty stderr files, which I only hashed. Hashes: `diff-0b3e46f` 0a18b001, `diff-c8b9391` 54a68898, `diff-cd30099` 0fa7e5a6, `diff-phase` e20437ee, `gate.log` 525b6d00, `m5p1-open md` ed5da088, `m5p1-p1 md` b2d1a74b, `negative.log` 85af306b, `report-draft` 28fff397, `run.log` 18ffd296, `scan-pre.log` 43ece043, `spec-0032-at-cd30099` 9f9c12c9, `spec-0032-working` 62218f3c, `specs-0001` 88a72d93, `test.log` 337b1036. All nine stderr files hash e3b0c442, so they are empty.

The harness put CLAUDE.md into my context. I did not use it as evidence.

## 1. Does the diff do what the spec and rulings say for P1, and nothing else?

**Mostly yes.** I read `diff-0b3e46f` for the crate, `diff-c8b9391` for the tool and `diff-cd30099` for the doc commit. `diff-phase` is the union of their file and hunk lists.

What is present, against the spec and rulings:
- **`crates/emend/src/styles.rs`:**
  - `cell_style_xfs` and `cell_styles` are public fields under `read`.
  - The containers only the typed read enters are walked in the same single read.
  - A second main-namespace `cellStyleXfs`, `cellStyles` or `dxfs` is refused under `read` and walked past without it (ruling 3).
  - An `xfId` past `cellStyleXfs` is refused at the end of the read.
  - `<borders>` passes `inside=false`, so `vertical` and `horizontal` stay refused there (ruling 1).
- **`crates/emend/src/styles/read.rs`:**
  - `NamedStyle` has the six fields, derives `Eq`, and treats a missing `hidden` or `customBuiltin` as false.
  - One shared resolver serves `cell_style` and `named_style`.
  - `dxfs` sits in the private `Typed`, compared by bits (ruling 4).
  - `read_dxf` refuses a non-main child, an unknown child (`extLst` included), a second child of one kind, and a `numFmt` with no `formatCode` (ruling 5). A `patternFill` with no `patternType` reads as `Solid` (decision 2). A dxf border reads `vertical` and `horizontal` past, each at most once.
  - `default_font` falls back to font 0 and refuses a part with no `<fonts>` or a `fontId` past the list.
- **`crates/emend/src/package.rs`:** the three gated calls, plus catalogue text for the new refusals and the four new test pins.
- **`crates/emend/src/package/tests/namedstyles.rs`:** 16 tests, 2 of them gated on `rules`. They cover every test the P1 section lists, and the ruling 1, 3 and 5 tests.
- **`tools/emend-copy/src/main.rs`:** `--namedstyles-sample` prints counts only, with the two fields ruling 2 added, and asserts `resolved = named_xfs`, the kept prefix, and `dxf_equal = dxf_written`.
- **`tools/emend-copy/tests/null_rewrite.rs`:** 3 tests.

Reservations:
- **Edits to existing fixtures:**
  - The `dxf.rs` BASE hunk (`@@ -9,11 +9,14 @@`) and the `readback.rs` hunk (`@@ -413,12 +413,15 @@`) change existing test fixtures. Both are disclosed (disclosure 2) and follow from the new typed dxf and `cellStyle` reads.
  - The `readback.rs` change shows a behaviour change: under `read`, a styles part whose dxf the type cannot hold now fails the whole read, `cell_style` included. The spec asks for this ("refused at the read"). Planning should still note it.
- **`package.rs` header hunk (`@@ -678,7 +678,12 @@`):** catalogue text arrives in P1, ahead of P3's catalogue work. Ruling 3 asks for "the catalogue saying so", so I accept it. Two things are off:
  - The joined comment line runs well past the normal width.
  - The header does not yet list `vertical` and `horizontal` as unmodelled, which ruling 1 asks of "the catalogue". It is only in the `differential_styles` doc comment. This is probably P3 work, but ruling 1 is not fully met in P1.
- **"A value that does not parse":** `read_dxf` does not check a dxf `numFmt`'s `numFmtId`.
- **Small and cosmetic:**
  - `styles.rs` adds `let _ = named;` to silence a warning when `read` is off.
  - A doc comment in `null_rewrite.rs` (`@@ -5109`) has one over-long line.
- **Ruling 3's "without the feature, walked past as today":** no test covers this. Only code inspection supports it.

I found nothing extra beyond these. The gate confirms there are no manifest, lock or script changes, the message set is unchanged at 139, and no gate file changed.

## 2. Do the logs contain every line the spec requires for P1?

**Present:**
- **Gate log:**
  - The prune of the three M4 part 2 roots, with `du -sb`, `rm -r`, gone and kept checks.
  - `check.sh` with nine PASS lines.
  - clippy and doc with `-D warnings` on four feature sets.
  - The `pub fn` listing with its cfg lines, the `read` items 5 growing to 8.
  - Tests for off, `read`, `read,rules` and all, on both targets: 514, 562, 600, 934, with namedstyles counts 0, 14, 16, 16.
  - The consumer check.
  - The gate-file listing.
- **Test log:** W, F, E, N, K, R, C and D on both targets, and plants P1 to P8, each found once and failing exactly its table.
- **Negative log:** N1, the `xfId` past `cellStyleXfs`, plus N2 to N6. Each exits 2 with the exact message read from source and leaves no output.
- **Run log:**
  - Both targets over 27 workbooks; the sample line has every field, `dxf_implied_solid` and `dxf_inside_sides` included.
  - The independent comparison: `rest_same`, `dxfs_prefix`, `appended_match`, plus counts and planted-flag controls on a changed and a kept case.
  - Totals 14 of 14 the same as the corpus figures.
  - TMPDIR and corpus devices; calcChain workbooks 9, with the rule.
  - The named record commit for the keep comparison; the edit open check; both record pairs installed.
- **Every log:** the working spec hash `62218f3c`.

**Gaps and reservations:**
- **The scan.** `scan-pre.log` passes (total hits 0, structural checks failed 0), but it is a pre-check. The final `scan.log` does not exist yet. That fits specs-0001's order (the scan comes after the check answers), but the "scan last" requirement is still open.
- **The scan rerun.** `scan-pre.log` prints "RERUN_NOTE is not set", yet disclosure 5 says this is a rerun of a failed first attempt. The specs-0001 rerun rule (the header names the superseded log by sha256) is not met here, even for an uncommitted pre-check.
- **The negative log's control.** It has one shared control, B0 ("the control for every case"), not one control per case.
- **The totals line.** It is labelled "printed, not checked" against FACTS.md figures. They equal the spec's facts and ruling 2 figures, but the log does not compare them with the spec.

## 3. Does every number and quoted string in the report appear in a primary?

Found: all commit ids and subjects, the four diff hashes, the three blob hashes, the log line counts and hashes, the "checks all as expected: yes" lines, the prune byte counts (4021354627, 6102050457, 1779558608), the nine PASS lines, the host emend-copy hash 52d02817, the test counts (514, 562, 600, 934, 918, 0, 14, 16, 16), plants P1 to P8, N1 to N6, the run counts (27, 854, 16, 0, 0, 0, 6), the 14 totals, 831 and 831, the keep figures (1097, 4, 11, 1082 + 11 = 1093), the open totals (1085, 12, 1086, 11), the four record hashes, and the scan-pre figures (3452, 43ece043, RESULT: PASS, 0, 0).

**Not found in any primary:**
1. The `logged.sh` sha256 99a6a3f4...bd5.
2. The launch time 2026-10-11T20:46:13Z. The gate prints a run time of 20:46:15Z.
3. "exit 0" for the launcher. There is no `logged.exit` in `<check>`. The empty outer stderr files do support "every outer stderr 0 bytes".
4. The superseded commits 8eeebc4 and e1cfb6b (disclosure 3).
5. The scan attempt 1 hash 55e82a8f...706f and "failed 3 structural checks" (disclosure 5).
6. "`<phase>/dry-logs/attempt1`" and the dry-run fixes (disclosure 4). `p1_count` and `GATE_BUILD_ROOT` appear in the scripts, but the attempts do not.
7. The `gen/negative.py` U+2014 fix.
8. The disclosure 1 survey and the "script-writing fork" note (disclosure 6). These are narrative and have no primary.

**Inaccurate:**
- **Disclosure 7 is incomplete.** Besides the API-key and diff-alone lines, `run.log` line 92 ends "RUN_SKIP_KEEP: no" and `scan-pre.log` line 2934 reads "EMEND_PRIVATE_DENYLIST set: no". Both are expected input lines, but the disclosure says the ": no" lines are only the two kinds.
- **"Each with its control"** for N1 to N6 overstates it: the log has one shared control.
- **"Totals beside planning's ruling 2 figures"** should be beside FACTS.md's corpus figures, as the log labels them. The values match ruling 2 and the corrected facts, but the log does not cite ruling 2.
