# M6 part 1 P3 logs

specs/0038-chartsheets.md, "P3", and planning's rulings K to M, committed
by the doc commit (specs/0038 sha256
f15be1a082884c256c102345554d1f12506e92d49132438afd288bf065bad65b). The
phase commit is 4cc4f2e048e8a2444bcd8b27445e5a95f6c96336, the CHANGELOG
commit. `<phase>` is the phase's directory,
`<home>/.cache/emend-build/m6p1-p3`.

| Log | Last line | sha256 |
|---|---|---|
| gate.log | gate log checks all as expected: yes | e4c14f71830b674dbd21fab3369afbe78879708ed53c243ac35f954d443f6ff1 |
| test.log | test log checks all as expected: yes | d3ba69331275f5370e1be0e25f9b585f0686e05de9a664c2f26a5af37cd0660c |
| negative.log | negative log checks all as expected: yes | 015cbd173575cf785f7c94b02a606932cdec8d5411162c2d1606676947b5d5da |
| size.log | size log checks all as expected: yes | bed4b4a6e2e72347107871a8dddd9c90325f6a096ea138a4c8aa9d70aa280f32 |
| run.log | run log checks all as expected: yes | 406223445748d2aef848cd22c44dcf77f3909ff023f8101f914d06300f470202 |
| scan.log | written last, by the scan | |

The logs ran serially at the phase commit, in the order gate, test,
negative, size, run, and ended at 2026-10-19T13:05:00Z, 13:07:17Z,
13:07:23Z, 13:12:51Z and 14:52:32Z (each log's "run ended at" line),
each with outer standard error 0 bytes. Each is the first of its kind;
no logged attempt was refused or stopped. The gate pruned nothing. The
test log wrote records/api-review-m6p1-2026-10-19-4cc4f2e.md, the size
log records/size-m6p1-2026-10-19-4cc4f2e.md, and the run log
records/m6p1-p3-chartsheet-2026-10-19-4cc4f2e.{md,txt} and
records/m6p1-open-2026-10-19-4cc4f2e.{md,txt}.

## The session's statements (no log prints them)

1. **One dry run of each log, all five passing.** The five generated
   scripts ran once each, serially, at the phase commit into
   `<phase>/dry/d1` (the gate's build, the test's API listing and every
   record pointed there), and each ended "checks all as expected: yes";
   no script was regenerated after it.
2. **The scan pre-check** passed (RESULT: PASS, structural checks
   failed: 0), sha256
   29ee3bf7530a8931a7d4ac386b6fbadab4dbbe992ab4ffbcc1cd21d1468c2677; after
   the independent check it was moved to
   `<phase>/superseded/pre-check-1`, which the final scan's RERUN_NOTE
   and RERUN_FILES name.
3. **The independent check** (independent-check-1.md) found two of the
   size log's judgments not as the spec asks: the images and comments
   probes' growth judged against the 1500 band where the spec puts it on
   the features' costs' movements (-116 and -64 in the record), and the
   record's copy paragraph citing specs/0038 for an expectation it does
   not state. The figures stand; the logs are committed as they ran and
   the report asks planning to rule on a corrected size and run log.
