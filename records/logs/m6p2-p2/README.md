# M6 part 2 P2 logs

specs/0039-chart-kinds-and-formatting.md, "P2", with planning's rulings
C, E (a) to (f) and G and its P2 brief. The doc commit c7b9421 committed the
spec (sha256
621573a3f9d1c5619c10db20928cb6eafc7a6d0047f02057c3baf055d6176a13) and
ROADMAP.md (sha256
2350890c677dd3b3eb2ec8e3ec1cad573a395ce5f7fe28ceefd5f40bdda2db25); the
tree is clean at the phase commit, so no log takes PLANNING_SPEC. The
phase commit is 32e4b7010d04a38749bb9c011bdd38f69f464eba, the op commit.
`<phase>` is the phase's directory, `<home>/.cache/emend-build/m6p2-p2`.

| Log | Last line | sha256 |
|---|---|---|
| gate.log | gate log checks all as expected: yes | bf2e2efbd79140fe928afa5dbf1224fbf5650f91d1e86d9b4f56058b0f93f46c |
| test.log | test log checks all as expected: yes | 702effec863c998232bd3ec2953d60f45711bce0a2bbbe6882b0c217ef8aa5c1 |
| negative.log | negative log checks all as expected: yes | 659a4c8fa31769c4c9f0e64c0cbf0335a7fdee994839f8be238f1a40cb01b8c6 |
| run.log | run log checks all as expected: yes | fd5de139a5d46d0f04667109214e1aed2b24231eb407f7eabbf5d125846547bd |
| scan.log | written last, by the scan | |

The logs ran serially at the phase commit, in the order gate, test,
negative, run, and ended at 2026-10-20T06:18:26Z, 06:20:13Z, 06:20:22Z
and 08:25:16Z (each log's "run ended at" line), each with outer standard
error 0 bytes (the driver's count of the file each log's standard error
went to; session's statement). Each is the first logged run of its kind;
no logged attempt was refused or stopped. The gate pruned nothing (its
change C72). The run log wrote
records/m6p2-p2-chartformat-2026-10-20-32e4b70.{md,txt},
records/m6p2-p2-chartsurface-2026-10-20-32e4b70.{md,txt} and
records/m6p2-open-2026-10-20-32e4b70.{md,txt}.

## The session's statements (no log prints them)

1. **The commits were rebuilt before any dry run.** A dev keep of the
   first chain's tests found three keep pairs of P1's record changed:
   the contour test's E (c) extension wrote its one-series surfaces
   between the four three-series ones, renumbering the outputs. The
   test was changed to write them in a second loop after the four, the
   core commit amended and the tool and op commits cherry-picked onto
   it. The first chain (0bf65b1, dc9db07, 8612b5e) is kept on the local
   branch p2-before-reorder. A second dev keep found 183 new pairs and
   none changed; the run log's keep block finds the same.
2. **Dry gate 1 failed; change A72 b.** The first dry run's gate, at
   32e4b70, ended "gate log checks all as expected: no" (gate.log sha256
   198521dcbcce49b127b5f4a8208558ac2768eb1faa072986d42ea9beabbc7216,
   script 7d557e9efdd14fa76d67eb8d02176302de2143aa7a30d530ec017652be8f7a0d,
   both kept in `<phase>/superseded/dry-gate-1`, with the generator as
   it ran). The code-commit control of commit_class named P1's record
   commit, which carries specs/0039 beside its logs and records and so
   is not code alone. The gate generator's change A72 b names part 1's
   close-out record commit, logs alone, as P1's gate had it.
3. **Dry run 2's run log failed; change NXW b.** Dry run 2, at 32e4b70,
   ended yes for the gate, test and negative logs, and "run log checks
   all as expected: no" for the run log, sha256
   ac86a9da297488c2d5e44e05609afe91a43cd6be33d092b64ea2e0a184fd6718 (kept
   in `<phase>/superseded/dry-run-2/d2`, with the run script that wrote
   it, sha256
   7471a8e7543fc12c51491603c27001d163cdc8301ee0a173a5edf3b23fa747bf, in
   `<phase>/superseded/dry-run-2`). The openpyxl fixture's field
   comparison printed nine of nine fields the same, but its check and
   control counted lines ending ": the same: yes" and ": the same: no"
   where the program prints "; the same: ", so both counted 0 and read
   no. The run generator's change NXW b makes the four patterns begin
   with the semicolon. A generator fault, not a code finding.
4. **Dry run 3**, at 32e4b70, with the gate script 731d3e58... and the
   run script d6dc0376..., ended yes for all four logs; no script was
   regenerated after it.
5. **Disclosures.** No separate carry commit: ruling E places its items
   in the core commit (the surface refusals, the three read-back
   extensions, add_chart's Errors paragraph rewrapped) and the tool
   commit (the surface sample's header rewrapped). A fourth base test,
   texts_are_escaped_and_references_written_as_given, is edited, forced
   by the type change: a NewSeries literal gains the three fields, nothing
   else. The core commit adds 9 lines to
   tools/size-probe/src/bin/probe-emend-charts-write-xml.rs, the same
   three fields on its three NewSeries literals, nothing else. The core
   commit also rewraps the axes_bytes doc comment in chart_write.rs (a
   P1 line of 87 columns, not past 100, so outside ruling E (e)); its
   message does not name it. The independent check found it.
6. **Ruling G.** `<p1phase>/gen/gate.py.gen1` was moved to
   `<p1phase>/superseded/gen-gate-py-gen1/` on 2026-10-20, before any P2
   generator was written (`<p1phase>` =
   `<home>/.cache/emend-build/m6p2-p1`); no log prints it.
