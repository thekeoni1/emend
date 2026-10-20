# M6 part 2 P3 logs

specs/0039-chart-kinds-and-formatting.md, "P3", with planning's rulings
I to M (the P2 verification) and its P3 brief; specs/0038 rulings N and
P. The doc commit 09e5f94 committed the spec (sha256
681e479969fc43702e39168d86f06ddd385336cbcde1149436e5576fe98fd9d1) and
ROADMAP.md (sha256
dc44a8680e6d7046b338f6ebf557afd6a0175c0c867bc62664e518b41c2ae9ff); the
tree is clean at the phase commit, so no log takes PLANNING_SPEC. The
phase commit is 50a3989bbace7bab0adceff67aa61650d647bb01, the changelog
commit. `<phase>` is the phase's directory,
`<home>/.cache/emend-build/m6p2-p3`.

| Log | Last line | sha256 |
|---|---|---|
| gate.log | gate log checks all as expected: yes | e500a082c9f160cf82ff2e56460852622a299d0a95fcccb9cd604ddbde57c517 |
| test.log | test log checks all as expected: yes | 00dbbb9562b995ba11e3d17df26e27fe86a44db59bec4e1ba545ed72b7969654 |
| negative.log | negative log checks all as expected: yes | 49889bcbd279f50774ad563baafda2fcee19b82d6e2b4cfe3e3973c3188a69ed |
| size.log | size log checks all as expected: yes | e81283b01d9f4ad2f9b9892241e81a1f2bd035018a997e0688335e3188bbf1bb |
| run.log | run log checks all as expected: yes | 86a24c2a775f60e1f774733690f20f02bad664c00ae259d0253105be3f36c754 |
| scan.log | written last, by the scan | |

The logs ran serially at the phase commit, in the order gate, test,
negative, size, run, and ended at 2026-10-20T12:04:05Z, 12:06:39Z,
12:06:47Z, 12:12:00Z and 14:16:50Z (each log's "run ended at" line), each
with outer standard error 0 bytes (the driver's count of the file each
log's standard error went to; session's statement). The run log read
the size log for its release binaries. Each is the first logged run of
its kind; no logged attempt was refused or stopped. One dry run (d1)
ran before them and ended yes for all five logs; no script was
regenerated after it.

The test log wrote records/api-review-m6p2-2026-10-20-50a3989.md; the
size log wrote records/size-m6p2-2026-10-20-50a3989.md; the run log wrote
records/m6p2-p3-chartformat-2026-10-20-50a3989.{md,txt},
records/m6p2-p3-chartsurface-2026-10-20-50a3989.{md,txt} and
records/m6p2-open-2026-10-20-50a3989.{md,txt}.

## The session's statements (no log prints them)

1. **The test commit was rebuilt before any dry run.** Its first form,
   85971db, failed clippy's type_complexity on the scatter, stock and
   bubble cases' expectation type. A local `type Expected` was added by a
   fixup commit, e3408a5, folded in by an autosquash rebase; the probe and
   changelog commits (50de67f, cd9816c before) were rebuilt onto it with
   no change of their own. Nothing of the first chain is kept on a
   branch.
2. **Ruling K, the session's reading.** The ruling asks the bubble case
   "to assert the dPt before xVal on both bubble series, named in the
   message". The test's first bubble series has categories, so its first
   reference after the points is `<c:xVal>`; the second has none, so
   emend writes no `<c:xVal>` there and its first reference after the
   points is `<c:yVal>`. The case asserts the points then `<c:xVal>` once
   and the points then `<c:yVal>` once, each named in its message.
   Planning is asked to confirm this reading.
3. **The charts cost is below planning's estimate.** The size log prints
   the part's growth of the charts probe over part 1's record, 1091520 to
   1097236, 5716, "below the estimate of 15500 to 25500; a FINDING: no".
   The spec tells planning the figure at any value and makes only a cost
   above 30000 a finding.
4. **The size log's change QF widens a check.** P1's and P2's core
   commits touch the format probe's sources (package.rs's charts-gated
   `pub use chart_write::{` list and comment lines), so the core-growth
   check now accepts commits that change comment lines and use-list lines
   alone, with a control; the measured core growth is 0.
5. **The API listing dates three new rows by its heuristic**:
   `NewMarker.color` and `NewPoint.color` at 784e3c0 and `NewMarker.size`
   at 4c36f05, where their types are 1637495's; the record prints them
   and does not say which cause applies. Enum variants (the surface, 3D
   surface and of-pie plots) are not rows of the listing, as in part 1's.
6. **The open record's .txt is P2's byte for byte** (sha256
   bfe4d8097807ba539a163450fdec68af9ec13458a6139ea1f75fcc5e9b5f4531): the
   phase adds no keep pair.
7. **The size record's charts probe row carries the second set's
   wording.** records/size-m6p2-2026-10-20-50a3989.md line 85 describes
   the second add as "index 0 to 12 over the thirteen `NewPlot`
   variants" before saying the kind index runs 0 to 15; the probe's
   header comment keeps the same second-set paragraph. Found by the
   independent check; not changed in this phase.
