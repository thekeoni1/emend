# Milestone 9 part 1 P2: the crate declares Rust 1.89, tested

Spec: `specs/0051-msrv.md`, "The change (P2)" and "Acceptance", with
planning's rulings F to K. Doc commit: 4e97262 (parent 62902dd, P1's
record). Code commits: ca5390f (the five files) and 1ab7caf (change 1,
ruling H: `scripts/msrv.sh` alone).

## Files

- `verify.log` sha256 743ac296710fde1b2fd7d3c6be0d26a03c190d3a934f28b89fb0ced597335e9a: the logged run, once,
  2026-11-14 (18:23:07Z to 18:31:46Z), on 1ab7caf, driver exit status 0.
  It prints the driver as run
  (sha256 4286f9b600c0deca96b2e05b3a7f31303cf06b3df65cbda533a6da775676f7c5) at its top.

Log form (`specs/0050` ruling H): a line beginning `$ ` is followed by
that command's own output; every line the driver prints begins `# `.
The roots `<run>`, `<phase>`, `<repo>` and `<home>` are declared in the
log's first lines and written by the driver in place of the paths.

## Result

Every check and control is as expected; no FINDING line (lines 4153 to
4158: "verify log checks all as expected: yes", exit status 0).

- The gate, both targets (line 350): nine PASS lines, host and i686
  among them, exit 0 (line 360).
- `scripts/msrv.sh` once with 1.89.0 (lines 363 to 4059): eight
  commands, each exit 0; tests run 545, 1245, 545, 1245 (host, host all
  features, i686, i686 all features), as in P1; last line
  `PASS msrv 1.89`. No command carries `--ignore-rust-version`.
- `scripts/msrv.sh` refuses a manifest copy naming 1.85, which is not
  installed (lines 4060 to 4089): exit 1, `FAIL msrv: toolchain 1.85 is
  not installed`, no cargo command run, and 1.85 still not installed
  after.
- The hygiene step (lines 4090 to 4104): a clone at 1ab7caf passes;
  with the README planted to 1.88 and staged it fails with
  `FAIL hygiene: the minimum Rust versions disagree (crate rust-version,
  msrv workflow, README): 1.89 1.89 1.88`.
- No U+2014 in the code commit's tree (lines 335 to 338), with a planted
  control.
- The budget (lines 4105 to 4152): the format probe's i686 size-profile
  total 989464 less `probe-zip-xml`'s 533216 from
  `records/size-m8p2-2026-11-10-8065795.md` is 456248, as required; the
  ceiling 458752 is 2504 above it.

## Dry runs, findings and rulings

Two dry runs preceded the logged run (not committed; under `<phase>/dry`).

- Before d1, planning's ruling H (change 1, a second code commit,
  `scripts/msrv.sh` alone): the cleanup removes its directory with
  `rm -r` and no `-f`, and the installed-target list is read into a
  variable before `grep -q`, since under pipefail `grep -q` leaving at
  its first match could fail the pipeline and report an installed target
  as missing.
- Ruling I: the driver lists the previous phase's eight target
  directories and removes none (lines 339 to 348); their removal waits
  on a method the maintainer names.
- Ruling J: the dry run waited until the root's mount propagation read
  shared. The maintainer set it, as they told the implementing session;
  the log shows only that it reads shared (lines 248 and 249).
- d1 (driver 43048db4) exited 0 with every check as expected and no
  FINDING line, as planning's ruling K states from d1's log (not
  committed). Reading its log found two display defects in the
  driver, neither changing a result: the U+2014 grep printed the byte
  itself on its `$` line, and the hygiene control's line read "the
  planted README 1.88 passes hygiene: as-expected" although the plant
  failed, as the line above it showed.
- Ruling K: both fixed (the grep's argument shown as
  `$'\xe2\x80\x94'`, the command run unchanged; the control states
  "fails hygiene (exit not 0)"); d2 with no other change; the logged run
  without a second word if d2 met its conditions.
- d2 (driver 4286f9b6, the driver of the logged run) exited 0, every
  check as expected, no FINDING line, no non-ASCII byte. So the logged
  run followed.

## Not done here

Nothing is pushed. The eight P1 target directories under
`<home>/.cache/emend-build/m9p1-p1` stay.
