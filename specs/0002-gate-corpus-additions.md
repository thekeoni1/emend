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

## P6d: narrow the path patterns (added 2026-09-15; DONE 2026-09-16)

T3 P3's first independent check named the bare shape of privacy
pattern 11 in prose (the string formed by `.claude` and `/projects/`),
and the gate would have refused the verbatim answer. The T0 P2 ruling
says patterns match the shape of a real secret or path, never a bare
name; patterns 1 to 7 were narrowed then, the three path patterns (8,
10 and 11, counted from 1 as `scripts/privacy.sh` prints them) were
not. This phase narrows them. Planning's first draft of this section
counted from 0; the implementing session read the tool's numbering and
was right to.

Changes:
- In `scripts/privacy-patterns.txt`, the three path patterns require,
  after their trailing separator, a path character or any byte that is
  not printable ASCII and not whitespace:
  `([A-Za-z0-9._-]|[^[:print:][:space:]])`. A real path always has a
  following segment, an accented folder name included; the bare name
  never does, and neither does a shape written in prose inside
  backticks or quotes or followed by a comma (planning's first wording,
  "any byte that is not a separator and not whitespace", was measured
  by the implementing session to re-match exactly those, and was
  withdrawn). The count stays 11. Accepted residue: a segment starting
  with any byte outside the class is unmatched, and a bare shape
  followed by a full stop still matches.
- The per-pattern self-test in `scripts/privacy.sh` plants fakes with a
  following segment, and gains one planted bare shape per narrowed
  pattern that must NOT be reported.
- Every phase scan script from now on copies the new lines (they check
  their copy against the file at the phase commit).
- No change to the hook, the identity file or the denylist.

Logs, in `records/logs/t0-p6d/`:
- `gate.log`: the full gate.
- `privacy.log`: `scripts/privacy.sh` over every local commit,
  expected PASS.
- `negative.log`, in scratch clones: (a) a committed file holding a
  real path of each of the three forms still fails, each under its own
  pattern number; (b) a committed file holding each bare shape passes;
  (c) the P6b and P6c forms of the drive-letter pattern still fail
  with a segment; (d) the T3 P3 check-1 answer as extracted (sha256
  762fc434..., reconstructed from the committed file and its header if
  the scratch copy is gone) passes the narrowed scan; (e) the self-test
  fails with exit 2 when one narrowed pattern is widened back.
- `scan.log`: per the scan rules.

Acceptance: all eight PASS; `privacy.log` PASS; every negative case as
stated.

Planning rulings (2026-09-16): P6d is verified and DONE (4641ae3 to
8f45639; phase commits 721e863 and 14c669f).
- Doc commit 4641ae3 holds exactly planning's three doc files (140
  insertions, 23 deletions). The phase commits touch only
  `scripts/privacy-patterns.txt` (three lines) and `scripts/privacy.sh`
  (the bare-shape plant); the five record commits only
  `records/logs/t0-p6d/`. All eight messages end with the Co-Authored-By
  trailer only, under the noreply identity.
- Planning ran the full gate (eight PASS) and `scripts/privacy.sh` over
  the 168 local commits (PASS). Planning scanned every tracked file at
  HEAD with the pre-P6d forms of patterns 8, 10 and 11 and with the new
  forms: 0 hits each way, so no committed file writes a bare shape.
- In a scratch clone planning committed a real path of each form (the
  scan failed under patterns 8, 10 and 11), the three bare shapes with
  backticks, quotes and a comma (PASS), and the T3 P3 check-1 answer
  reconstructed from the committed file and its header (22219 bytes,
  sha256 762fc434..., PASS); with the pre-P6d pattern file restored the
  self-test refused to run, as case e says.
- The false home-path check in the first gate log (a single-file
  `grep -c` summed through `awk -F:` always gave 0) was found by check 2
  and repaired with controls that use the same form; the reruns after
  check 2 are log-only and are accepted as verified by planning. The
  wrong reason for pattern 11's `[\\/]+` in 721e863's message stays
  in history, corrected in the report; the widening it makes is real
  and tested (case g).
- The absolute-path drift in committed logs (86 occurrences in 55
  files, recounted by the implementing session and listed by planning):
  the repository path, the corpus directory name and deliberate test
  fixtures, plus the bare account name, which the T0 P1R ruling
  (`specs/0001-bootstrap.md`, question 2) accepted as generic. Nothing
  under the account home outside the repository is named. Accepted as
  recorded, no rewrite; from now on logs and check answers write
  `<repo>` and `<home>`.
- New scan rule (in `specs/0001-bootstrap.md`): every scan script also
  scans with the pre-P6d forms of patterns 8, 10 and 11 as a
  bare-shape control, expecting 0, so "no log writes a shape" stays
  provable now that the live patterns are blind to a bare shape.
