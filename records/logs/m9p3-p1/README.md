# Milestone 9 part 3 P1: the release of 0.1.2, prepared

Spec: `specs/0053-release-0-1-2.md`, "The equal-bytes proof (P1)", "The
release texts and the manifest (P1)" and "Acceptance" 1, 2 and 5, with
planning's rulings A (the size figure at the 0.1.2 manifest) and B (the
publish dry run's warnings).

| Role | Commit | Subject |
|---|---|---|
| base | ce847c5 | M9 part 2 close-out record: the push, CI, the post scan |
| doc 1 | bcc4d25 | M9 part 3 P1: commit planning docs (the 0053 spec, part 2's close-out verification) |
| code | 7622089 | M9 part 3 P1: the crate's version is 0.1.2 |
| doc 2 | 7bade30 | M9 part 3 P1: commit planning docs (0053 ruling A, the size figure at the 0.1.2 manifest) |
| docs (release commit) | ba39a74 | M9 part 3 P1: the CHANGELOG and the README carry 0.1.2's texts |

## Files

| File | What it holds | sha256 |
|---|---|---|
| gate.log | the commits, Acceptance 2, the gate, `scripts/msrv.sh`, the budget | 5cc1549f809d19f8fb9567380d7b2e89ffad980e38a52719181d4d44514e87cc |
| run.log | the texts, the equal-bytes proof, the packaging dry run, Acceptance 5 | 25b279f0f64ab15a974cc357a3e8bc7e1d1fb26a1d4f6414aac5d05c52e14817 |

Both were written by one logged run on
ba39a74634aa2dd86f3b88728793505f05181bb7: gate.log from 2026-11-24T02:08:48Z
to 02:17:51Z, run.log from 02:17:51Z to 02:21:40Z, each with "script exit
status: 0". Each prints the driver as run, with its sha256
87e7fd43bc9cff56365e82bb28358fef57eabbd7cfe5c8c6ec5971ee11a9a4f7. The run
mode starts only when the gate log of its own directory reads "gate log
checks all as expected: yes".

Log form (`specs/0050` ruling H): a line beginning `$ ` is followed by
that command's own output; every line the driver prints begins `# `. The
roots `<target-root>`, `<run>`, `<phase>`, `<repo>` and `<home>` are
declared in each log's first lines. The driver checks its tools with
`command -v` before it makes its directory and refuses an empty tool list
(`specs/0051` ruling L).

## gate.log

"gate log checks all as expected: yes", "FINDING lines: 0", "checks and
controls not as expected: 0".

- CARGO_REGISTRY_TOKEN and CARGO_REGISTRIES_CRATES_IO_TOKEN: absent.
- Acceptance 2: the code commit names `Cargo.lock` and
  `crates/emend/Cargo.toml` alone, the version line of each from 0.1.1 to
  0.1.2; the docs commit names `CHANGELOG.md` and `README.md` alone.
- The gate printed PASS for each of its nine steps, host and i686 among
  them.
- `scripts/msrv.sh`: eight commands, each exit 0, test counts 545, 1245,
  545, 1245; "PASS msrv 1.89".
- Ruling A 3: "the format probe's sections: 989480"; "the budget: 989480
  - 533216 = 456264, 2488 under the ceiling 458752".

## run.log

"run log checks all as expected: yes", "FINDING lines: 0", "checks and
controls not as expected: 0".

- Ruling A 4: the gate log of this directory prints 989480 and 456264.
- The texts: the CHANGELOG's first entry is "## 0.1.2 (unreleased)"; the
  0.1.1 entry and all below it are ce847c5's, byte for byte; the entry's
  size line names 456264 and 2488. README names 456264 twice, each beside
  `records/logs/m9p3-p1/gate.log`, and its size sentence keeps 0.1.1's
  456248 beside `records/logs/m8p3-p3/gate.log`.
- The equal-bytes proof, the source: `git diff --stat v0.1.1` over
  `crates/emend/src` and `crates/emend/template` names
  `crates/emend/src/lib.rs` alone, 52 changed lines; "the count of
  changed lines not beginning //! is 0". The control, one code line
  planted in a scratch clone, reads 1.
- The equal-bytes proof, the outputs: clones of v0.1.1 and of the release
  commit, each building with rustc 1.98.1 from its own
  `rust-toolchain.toml`, ran `cargo test -p emend --all-features --locked`
  on the host and on i686-unknown-linux-musl with their own
  `EMEND_KEEP_DIR`. Each of the four runs kept 4254 files; the sha256 of
  each sorted list is
  5193a25d4e9a6728e52ae7cf405f44908dbfd97826efec4b144265ca714d1b46. On
  each target: "the same names and the same sha256 for every file". The
  CHANGELOG's Measured line reads 4254 and 4254, this run's counts.
- The packaging dry run, in a `git archive` of the release commit with no
  repository above it: the archive's list holds 75 names and equals the
  v0.1.1 clone's own `cargo package --list` by name, with
  `.cargo_vcs_info.json` (written only from a repository) set aside from
  the clone's 76. No example and no `CONTRIBUTING.md` in it. The publish
  dry run exits 0: one "Packaging emend v0.1.2", "Packaged 75 files".
- Ruling B: the publish dry run prints exactly three warning lines and no
  error. Two are "warning: ignoring example `edit_cell` as
  `examples/edit_cell.rs` is not included in the published package" and
  the same for `new_workbook`; they follow from `specs/0052` decision 1 A
  (the examples stay outside the package), and the maintainer will see
  the same two when publishing. The third is "warning: aborting upload
  due to dry run".
- Acceptance 5: no U+2014 in the release commit's tree; "privacy: 4
  commit(s), 3152 blob(s) scanned, 0 finding(s)" over ce847c5..ba39a74.

Two dry runs came before the logged run (phase scratch, not committed):
d1, whose run log refused the publish dry run's two example warnings,
the cause of ruling B; and d2, on the driver after ruling B, with both
logs all as expected.

This record commit is local. No tag is made in this phase.
