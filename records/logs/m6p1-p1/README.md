# M6 part 1 P1 logs

specs/0038-chartsheets.md, "P1", and planning's rulings A to C, held
uncommitted in the working spec (sha256
29536c9203415cbd01665a3dc0a7563ed2c38d3f45eef1289a8ca7ef1c880cb2). The
phase commit is f917618a1fdbe022dbe0ae25b59018d695afefff, the tool commit.
`<phase>` is the phase's directory, `<home>/.cache/emend-build/m6p1-p1`.

| Log | Last line | sha256 |
|---|---|---|
| gate.log | gate log checks all as expected: yes | 42f4e9a7fc2fec7b24fcbd4bdecd62fb9d175d44d169a5a91e2c8641aa2b11e9 |
| test.log | test log checks all as expected: yes | 3941559d54b712a61afcf41145e3dba35d6aa8bc4e3c0ddedca06e79cf89d2e3 |
| negative.log | negative log checks all as expected: yes | a18805e60bd77d55a00291535f2da6541a1214a367d2df84403ddcf122b8d992 |
| run.log | run log checks all as expected: yes | c89128302725ecb86f65cc9e3062b4cd2e4ae40e36c833dffca3ac3253ffdaf8 |
| scan.log | written last, by the scan | |

The logs ran serially at the phase commit and ended at
2026-10-19T02:07:59Z, 02:09:22Z, 02:09:27Z and 05:43:13Z, each with outer
standard error 0 bytes. The gate, test and negative logs are the first of
their kind and the run log the second (below). No logged attempt was
refused or stopped. The gate pruned the target roots of Milestone 5 part 6 P1, P4
and the close-out (P2's and P3's were not there). The run log wrote
records/m6p1-p1-2026-10-19-f917618.{md,txt} and
records/m6p1-open-2026-10-19-f917618.{md,txt}.

**FINDING, change NWZ: the first run log failed the scan's pre-check.**
It ended "run log checks all as expected: yes" at 03:50:53Z, but the scan
found it printing two placeholders before a slash, `<dir>` and `<id>`,
with no root line declaring them: in comments of the chartsheet rule and
the fixture's program, and in one command line, all new in this phase. The
run generator's own placeholder check read a list of ten tokens over the
code alone, so it passed them. Change NWZ names the directory in words,
prints the fictional fixture's id in the command line, and widens the
generator's check to the scan's rule over the whole script. The run log
was run a second time at the same commit; its RERUN_NOTE names the first
and the pre-check, and RERUN_FILES hashes them. The first run log, as
written and as installed, its records, the run script that wrote it and
the pre-check are kept in `<phase>/superseded/run-nwz-1`; the run moved
its run directory and keep into `<phase>/scripts/out/superseded`, and
its own run directory is 2026-10-19-f917618-b-chartsheet. The second
records differ from the first only in the run directory's name, the time
each was written and the runs' wall_s and peak_rss_kb columns; the
open-check .txt is byte-identical (the session's statement). The gate
log's defined-helper lines read the run script before NWZ, which adds no
yn, ynnot, is or isnot call.

## Before the logs (the session's statements; no log prints them)

1. **The first dry test log failed: the code commit was amended.** Clippy
   with `-D warnings` refused an unused constant in the chartsheet tests
   under the features charts and charts,read, and rustdoc with
   `-D warnings` refused an intra-doc link to `Package::charts` in the
   package catalogue with no features. The core commit was amended
   (the constant gated on images or comments, the link made plain text)
   and the tool commit replayed on it, the messages unchanged: core
   56b2937 (was f26bb06), tool f917618 (was 07bc4c8). The old pair is on
   branch p1-before-fix. The dry log is kept in
   `<phase>/superseded/dry-test-1` (sha256
   af1a3dbf7d315089f68d57bbb8a3bcf77308a71c0ea4b62e7216dcf92a3787af).
2. **A dry negative log failed, a driver fault.** The dry driver gave the
   negative script no gate log, so it read the logged path, absent then,
   and failed the binary's hash checks and its own standard error check.
   Rerun with the dry gate log as its second argument, it passed; the
   logged run reads the logged gate log. The failed dry log is kept in
   `<phase>/superseded/dry-negative-1` (sha256
   ca08ba01d472d7ad9d42b35601a38d0345be0cc36f998aeb7e16b79bf7a26914).
