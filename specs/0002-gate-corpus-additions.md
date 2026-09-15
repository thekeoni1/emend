# 0002: T2 P0 privacy gate, corpus-era additions

Status: DONE, verified by planning 2026-09-15 (9324e64 to 0870c36).
Context: `CLAUDE.md`, `specs/0001-bootstrap.md` ("How this task runs",
"Rules for every phase", and the planning rulings on P6c).

## Goal

Close the P6c findings that become realistic once corpus files exist on
this machine, before the maintainer starts collecting them. The
maintainer ruled on 2026-09-15 that this runs before T1.

After this phase, new privacy-gate findings go to a backlog in
`ROADMAP.md`, unless they concern a data flow this project actually has.

## How this phase runs

As in `specs/0001-bootstrap.md`, "How this task runs":

1. First commit: planning's pending doc updates, unedited.
2. The phase commit.
3. Logs in `records/logs/t2-p0/`, committed in record commits.
4. At most two independent checks.
5. Report, then STOP.

Nothing is pushed in this phase.

## Changes

To `scripts/privacy.sh`, and to the matching step 7 and `.gitignore`
forms wherever a path form applies:

1. Download metadata files:
   - Any tracked path ending in `:Zone.Identifier` fails, wherever it
     is, including under `tests/fixtures/`. The count is printed, never
     the path.
   - The office suffix form also accepts `:` after the extension, as
     in `($|[#~.:])`.
   - `.gitignore` ignores `*:Zone.Identifier`.
2. More content signatures. A blob outside `tests/fixtures/` that starts
   with any of these fails:
   - OLE2 compound file (old `.xls`, `.xlt` and `.xla`, and
     password-protected OOXML): `D0 CF 11 E0 A1 B1 1A E1`;
   - xz: `FD 37 7A 58 5A 00`;
   - bzip2: `42 5A 68`;
   - zstd: `28 B5 2F FD`;
   - rar: `52 61 72 21 1A 07`;
   - split or spanned zip: `50 4B 07 08`.

   Also fail on tar: the bytes `75 73 74 61 72` at offset 257. The
   finding keeps the P6c form, with the label "archive or compound-file
   signature", and never prints a path.
3. Self-test:
   - Plant fakes for both forms of patterns 10 and 11. The P6b form has
     a single separator and lower-case `Users`; the P6c form has doubled
     backslashes and `USERS`. Each must be detected under its own
     number.
   - Plant one blob per new signature, and one `:Zone.Identifier` path.
4. Exit statuses:
   - Every stage of the email pipeline has its exit status checked; any
     failure exits 2.
   - The signature helper exits 2 when reading a blob's first bytes
     fails. It never turns that failure into a finding line.

Renamed CSV, TSV or FODS files without a content signature are accepted.
There is no reliable signature for them.

## Logs

- `gate.log`: a full run, all eight steps.
- `privacy.log`: `scripts/privacy.sh` over every commit on main.
  Expected: PASS.
- `negative.log`: each case in a scratch clone. No output may contain
  any fixture text.
  - (a) `r.xlsx:Zone.Identifier` at the root: FAIL. The same under
    `tests/fixtures/`: FAIL. Untracked, it is ignored.
  - (b) One blob per new signature, each named `x.bin` outside
    `tests/fixtures/`: FAIL for each of OLE2, xz, bzip2, zstd, rar,
    split zip, and tar with `ustar` at offset 257. The OLE2 blob under
    `tests/fixtures/`: PASS.
  - (c) In a scratch copy, narrow pattern 10 back to a single `/` or
    `\` separator with `[Uu]sers`, or pattern 11 back to `/` only. The
    self-test fails, exit 2.
  - (d) In a scratch copy of `scripts/privacy.sh`, force one stage of
    the email pipeline to fail, and separately force the signature
    helper's read to fail. Each gives exit 2.
  - (e) Repeat P6c negatives (a), (c) and (d), and show they still fail.
- `scan.log`: per the scan rules. It also covers
  `records/logs/t0-close/`, if present.

## Acceptance

- All eight `PASS` in `gate.log`.
- `privacy.log` passes over the full history.
- Every negative case shows its expected result, and no fixture text
  appears in any output.

## Planning rulings (2026-09-15)

- T2 P0 is verified from primaries by planning's own checks:
  - Doc commit 9324e64 holds the two pending paths (18 insertions, 2
    deletions).
  - Phase commit d97b9ef changes `.gitignore`, `scripts/check.sh` and
    `scripts/privacy.sh`. `scripts/privacy-patterns.txt` still holds 11
    patterns.
  - The record commits 384231f, 9a726b9 and 0870c36 touch only
    `records/logs/t2-p0/`. All five messages end with the Co-Authored-By
    trailer only.
  - Planning ran `scripts/check.sh` from `/`: all eight steps PASS, zero
    control bytes. `scripts/privacy.sh` over the 68 local commits: PASS.
    GitHub is unchanged at e0a5175.
  - Planning scanned the T2 P0 logs, both check answers and the two
    close-out logs: zero hits against the denylist, the patterns,
    non-ASCII bytes, control bytes and the email allowlist.
  - Planning reproduced these cases in scratch clones:
    - `r.xlsx:Zone.Identifier` at the root gives count-only office and
      Zone.Identifier FAIL lines;
    - a Zone.Identifier file under `tests/fixtures/` gives FAIL;
    - an untracked Zone.Identifier file is ignored;
    - OLE2, xz, rar and tar (`ustar` at offset 257) blobs outside
      `tests/fixtures/` each give FAIL archives, and OLE2 under
      `tests/fixtures/` gives PASS;
    - the P6c zip case still gives FAIL;
    - narrowing pattern 10 back to its P6b form stops the self-test with
      exit 2;
    - an `od` that fails gives exit 2 with `ERROR privacy: cannot read
      the first bytes of a blob`, and no finding line.

    No FAIL line printed a file name.
- Departures 1 to 9 are accepted.
- Finding 1 (unchecked `sed`, `tr` and `cut` exit statuses in two helper
  pipelines) goes to the privacy gate backlog in `ROADMAP.md`. It
  concerns a tool failure, not a data flow this project has.
- Finding 2 (content findings in archive or compound-file blobs at
  ordinary paths print the path) goes to the same backlog. The push is
  already blocked by the count-only archives line, and the path reaches
  only the terminal or a local log.
- Both backlog items fit the gate self-test script task the maintainer
  approved for after T0.
