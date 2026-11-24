# Milestone 9 part 2 P1: two examples, CONTRIBUTING.md, two README sentences

Spec: `specs/0052-launch-prep.md`, "Acceptance" 1 to 6, with planning's
ruling A (the propagation check made a note; the broken-clone control
tied to the planted line). Doc commit: e37b276 (parent a031c83). Code
commit: 3adfeb1 (parent e37b276).

## Files

- `verify.log` sha256 ca87c4a20337cc28b2771f54cbc86dc6bb7049cc5e9cf2c44eadc975a24d9b3c:
  the logged run, once, 2026-11-24 (00:38:46Z to 00:49:47Z), on
  3adfeb1cce955d948d0c8d734cfb56b2be094c7c, driver exit status 0. It
  prints the driver as run, with its sha256
  c470bdecceba86968294b82eeb40708de386ae2462aba0ef231085448bb4aed3, at
  its top.

Log form (`specs/0050` ruling H): a line beginning `$ ` is followed by
that command's own output; every line the driver prints begins `# `.
The roots `<target-root>`, `<run>`, `<phase>`, `<repo>` and `<home>`
are declared in the log's first lines. The driver checks every tool it
calls with `command -v` before it creates its run directory, and each
control reads an exact value from a planted fixture (`specs/0051`
ruling L).

## Result

"verify log checks all as expected: yes", "FINDING lines: 0", "checks
and controls not as expected: 0", "script exit status: 0".

- Acceptance 1, the gate: nine PASS lines, host and i686 among them.
  The control: a clone of the repository at 3adfeb1 with one line of
  `crates/emend/examples/edit_cell.rs` broken; `scripts/check.sh --only
  host` there prints "FAIL host: 'cargo build --workspace --all-targets'
  exited non-zero" and the compiler output names
  `crates/emend/examples/edit_cell.rs:29:12`, the planted line. The
  repository stays clean.
- Acceptance 2: `scripts/msrv.sh` with 1.89.0: eight commands, each
  exit 0, tests run 545, 1245, 545, 1245, last line "PASS msrv 1.89".
- Acceptance 3: both examples built for the host and for
  i686-unknown-linux-musl ("ELF 32-bit LSB executable, Intel i386",
  "statically linked"), each run once on each.
  - `edit_cell` over `tests/fixtures/openpyxl-ops.xlsx`: on both
    targets, 9 entries in, 9 out, 8 byte-identical to the fixture; the
    one entry that differs is `xl/worksheets/sheet1.xml`, the first
    sheet as read from the fixture's workbook part. No FINDING.
  - The host and static outputs are byte-identical for each example
    (edit_cell e80b2f30bb49478bbd9e28ed532bb4a5b32f4df0f9c23c8df3343d11a81205d1,
    new_workbook 143dc907f7ab896961e1e3f9983660e8ae6260b1aba4a629edcf5bc497859542).
  - `new_workbook`'s output, read back: "A1 Item, B1 Count, A2 Bolts,
    B2 120, A3 Washers, B3 2.5".
- Acceptance 4: `git diff --stat` e37b276..3adfeb1 names
  `CONTRIBUTING.md`, `README.md`, `crates/emend/examples/edit_cell.rs`
  and `crates/emend/examples/new_workbook.rs` and no other file;
  README.md gains 6 lines and loses none. No office extension in either
  example.
- Acceptance 5: the format probe less probe-zip-xml, 989464 - 533216 =
  456248, 2504 under the ceiling 458752.
- Acceptance 6: no U+2014 in the code commit's tree; the privacy scan
  over a031c83..3adfeb1 prints "privacy: 2 commit(s), 3140 blob(s)
  scanned, 0 finding(s)".

The root's mount propagation read `private` during the run. Ruling A:
no step of this phase needs it, and the log notes it without a check.

Two dry runs came before the logged run (phase scratch, not committed):
d1 on the first driver, one check not as expected (propagation); d2 on
the driver after ruling A, all as expected.

This record commit is local and goes with the part's close-out push.
