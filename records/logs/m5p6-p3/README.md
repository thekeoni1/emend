# M5 part 6 P3 logs

specs/0037-measurement-and-close.md, "P3: the close", and planning's
rulings A to Y, held uncommitted in the working spec (sha256
d94a5f62a0b23e3eeb4025903a9087e70736cc26ef57c0175c0c260b82fe5244). The
phase commit is ef9da373bc462ed8a9ced3992c5e404b9c954e09. `<phase>` is the
phase's directory, `<home>/.cache/emend-build/m5p6-p3`.

| Log | Last line | sha256 |
|---|---|---|
| gate.log | gate log checks all as expected: yes | d9f79c5e7218502bc6db12f107edc4be44c942ce269597ff22f6a4ff7cec260e |
| test.log | test log checks all as expected: yes | deae0b5b73731de6479d8b128259234049ea485c72169d7f76bcffd7ddcae521 |
| negative.log | negative log checks all as expected: yes | cc2401e33d1c97dbaaaf3712c1b9b6b7a9c1aae4d66f43177951c7d875dcaafe |
| size.log | size log checks all as expected: yes | 17e29e556b68b66e512bef2224022e1fa01cc068e95023c38c984f79592ee6e5 |
| run.log | run log checks all as expected: yes | 11cf162e8a35e53f4ea691aef6a6c821c0d86f2a1f9d2d964de4ecf7cb286d20 |
| scan.log | written last, by the scan | |

The logs ran serially at the phase commit and ended at
2026-10-18T09:02:45Z, 09:07:01Z, 09:12:54Z, 09:18:09Z and 13:02:40Z.

## Two attempts refused at entry

Neither attempt measured anything or wrote a run directory. Each attempt's
files (empty log, empty outer stderr, the script's own stderr) are kept
under `<phase>/superseded`.

1. **FINDING, change PZ8: the first negative log refused at entry**,
   exit 2, "the working tree holds changes outside this phase's logs and
   planning's spec". The test log, which runs before it, writes the API
   record records/api-review-m5p6-2026-10-18-ef9da37.md into the tree, and
   the negative script's status pattern did not take it. The dry run sent
   that record to a stand-in directory, so it never met it. The fix is in
   the negative generator: the pattern also takes the untracked API record
   named by the phase commit's short sha, and its planted control holds
   that record (taken), one named by another commit and the size record
   (refused). The refused attempt is in `<phase>/superseded/negative-logged-1`
   (its stderr sha256
   35ef8de8b448bf1f3b774822dd58be8f77985f2941761b1f0e50f93fc7fc9920), the
   script as it was in `<phase>/superseded/negative-script-1`
   (1e03865f4f90dbb5c318ae150a3ffe6dc4614d16cbae21e5342ada32f5d913e6).
   negative.log is the regenerated script's, its header naming PZ8.
2. **The first run log refused at entry**, exit 2: the dry run's 16 staged
   records were still in `<scripts>/out/records` (each equal byte for byte
   to the dry run's records, the session's statement; moved aside to
   `<phase>/dry2/staged-records`).
   Its files are in `<phase>/superseded/run-refused-1`. run.log is the
   second attempt; its RERUN_NOTE and RERUN_FILES lines name the first and
   hash its files.

What no log carries (the independent check's Q2 items 1 and 2):

- The gate log's defined-helper lines (ruling S) read the negative script
  as it was before PZ8. No log prints that line for the regenerated script
  that wrote negative.log (sha256
  f568786756d34cc2d4b0d308f075eb54689ff23529aae13d1d006edb436fc8bb). PZ8
  changes no yn or ynnot call. The same check, run on the regenerated
  script by the session, printed "49 names called ... : yes" with its
  control "yes"; its output is kept as `<phase>/defined-negative-2.out`
  (the session's statement).
- negative.log's RERUN_NOTE is unset, and no log hashes the refused
  negative attempt's files. Their hashes above are the session's statement.
