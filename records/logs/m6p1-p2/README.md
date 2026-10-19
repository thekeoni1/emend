# M6 part 1 P2 logs

specs/0038-chartsheets.md, "P2", and planning's rulings D to I (the doc
commit's specs/0038) and J, held uncommitted in the working spec (sha256
6ca55be29ed16da27c640d737e43cc30d6a8e544baa48a5701c0ca0ffbdf1e65). The
phase commit is 2a706dc8cbaae422f8b1777e8fda88e3a2b1dbe2, the limits
commit. `<phase>` is the phase's directory,
`<home>/.cache/emend-build/m6p1-p2`.

| Log | Last line | sha256 |
|---|---|---|
| gate.log | gate log checks all as expected: yes | 084af196c7830b9eff2e161a4a86e53e50a8282482d5d21000cef82d2dcda5aa |
| test.log | test log checks all as expected: yes | d5fbde955343d9bcd6001dedc56ec6b6fba9f302bd0e5ae7a11a9be1b4175214 |
| negative.log | negative log checks all as expected: yes | ab4215ed6b97858e6058dae33e9aa0af4c00be5de8286ec41c68728317a8bcf7 |
| run.log | run log checks all as expected: yes | bd95bf0278bc17d0d0ee3b5218a0d05267c0be3165bd89d4cde5c7996284aa01 |
| scan.log | written last, by the scan | |

The logs ran serially at the phase commit and ended at
2026-10-19T08:32:10Z, 08:33:45Z, 08:33:50Z and 10:13:30Z (each log's "run
ended at" line), each with outer standard error 0 bytes. Each is the
first of its kind; no logged attempt was refused or stopped. The gate
pruned nothing. The run log wrote records/m6p1-p2-2026-10-19-2a706dc.{md,txt} and
records/m6p1-open-2026-10-19-2a706dc.{md,txt}.

## Before the logs (the session's statements; no log prints them)

1. **The first dry gate log failed, a generator fault.** Change I69, which
   replaces P1's block of the test-gating checks, cut the line of that
   block that set `PKG_TESTS`, so the gate stopped on "PKG_TESTS: unbound
   variable" (its inner standard error, 202 bytes) and printed
   UNEXPECTED for "part 1's test at the write is still gated to the
   feature off" and for its own standard error check. I69 now sets
   `PKG_TESTS` itself, and says so in its header; the gate was
   regenerated. The failed dry log, its standard error files and the gate
   script that wrote it (sha256
   a845d4fadd3dd0a119b4b9833116cb3409fa9f01d272fa68075f0fbd61a0d0af) are
   kept in `<phase>/superseded/dry-gate-1`; the log's sha256 is
   7f7e2af7178220e4423a46c5bb0536bdd5f42f84c5ee695ea6cbf9035d67b332.
2. **The first dry driver did not start, a driver fault.** The directory
   its output was redirected into was not made first; it ran nothing. The
   directory was made and the driver started again.
