# Milestone 9 part 1 P1: the minimum Rust version, by trial

Spec: `specs/0051-msrv.md`, "The trial (P1)", with planning's rulings A
to E at P1's stop report. Doc commit: e07544e (parent e1d8fc7).

## Files

- `trial.log` sha256 5830696a5530e27dfd0f2d6b84ea957595e16ada49210ad26447469e79ec6e46: the logged run, once, 2026-11-14,
  driver mode `run`, driver exit status 0. It prints the driver as run
  (sha256 65e5645c06c3f074a339bbb147432f5c20dee33c2b181c0f96d31a182295dfcd) at its top.

Log form (`specs/0050` ruling H): a line beginning `$ ` is followed by
that command's own output; every line the driver prints begins `# `.
The roots `<repo>`, `<phase>` and `<home>` are declared in the log's
first lines and written by the driver in place of the paths.

## Result

The floor is 1.89.

| Trial | rustc | Eight exit statuses | Tests run (host, host all, i686, i686 all) | Result |
| --- | --- | --- | --- | --- |
| pin 1.98.1 (reference, not a trial) | 1.98.1 | all 0 | 545, 1245, 545, 1245 | pass |
| 1.88 | 1.88.0 | all 0 | 545, 1245, 544, 1244 | fail |
| 1.93 | 1.93.1 | all 0 | 545, 1245, 545, 1245 | pass |
| 1.90 | 1.90.0 | all 0 | 545, 1245, 545, 1245 | pass |
| 1.89 | 1.89.0 | all 0 | 545, 1245, 545, 1245 | pass |

The summaries are the log's lines 3940 to 3949 (pin), 7632 to 7641
(1.88), 11342 to 11351 (1.93), 15048 to 15057 (1.90) and 18754 to 18763
(1.89); the floor line is 18764 and the next lower minor's line 18765.

Why 1.88 fails: every command exits 0, but its two i686 test commands
(lines 5822 and 6377) run no doctest. Each other test command in the
log prints `Doc-tests emend` and runs the crate's one doctest
(`crates/emend/src/lib.rs - (line 42)`); `grep -n -F 'Doc-tests emend'`
finds it 18 times: four in each run except 1.88's, which has two, both
on the host (lines 4533 and 5795). The counts therefore differ from the
pin's by one (lines 7639 and 7640), and the spec's rule makes 1.88 a
failed trial. No command printed an error line. The trial was not rerun.

Why 1.87 is not tried is moot, since 1.88 failed; the log still prints
zip 8.6.0's `rust-version = "1.88"` (lines 18766 and 18767).

## Dry runs, findings and rulings

Two dry runs preceded the logged run (not committed; under `<phase>/dry`).

- d1 (driver 3b5575d6) exited 0 with the pin standing in for a trial.
  - F1: the stand-in trial reused the reference run's target directory,
    which the spec forbids. Fixed: one directory per run and version.
  - F2: d1's install line updated rustup itself from 1.29.1 to 1.29.3,
    which no plan named.
  - F3: the implementing session ran `rustup set auto-self-update`
    meaning to read the setting; it wrote `auto_self_update = "enable"`
    to rustup's settings file. That is rustup's default.
- Planning's rulings at the stop report: A, every command carries
  `--ignore-rust-version` (the manifest inherits 1.98.1 until P2); B,
  every install line carries `--no-self-update`, and rustup stays at
  1.29.3 and the setting stays, nothing reverted; C, F1's fix accepted;
  D, a dry control of the search over a planted table (fail below 1.91,
  pass from it) through the same loop as the logged run; E, d2 first,
  then the logged run without a second word if d2 met its conditions.
- d2 (driver 65e5645c, the driver of the logged run) exited 0: seven
  controls "as expected", among them the search control printing floor
  1.91 with 1.90 the failed minor below; no FINDING and no ": no" line;
  the stand-in trial's first build compiled afresh. So the logged run
  followed.

## Toolchains installed by this phase

1.88, 1.93, 1.90 and 1.89, each with `--profile minimal --target
i686-unknown-linux-musl --no-self-update`, user-level under `<home>`.
