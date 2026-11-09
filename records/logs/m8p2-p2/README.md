# M8 part 2 P2 logs

specs/0048-the-two-refusals.md, "Threaded comments (`comments`)",
"Errors", "Size", "Harness runs" and "### P2: threaded comments", with
planning's rulings A to C. P2's doc commit 94fc5a4 committed the spec at
sha256 1328dd1a14ea7a8c25791631abb0932a9942efc4ae3ce06686a8abe1a50efa7f
and ROADMAP.md at sha256
e957658cbbf360e6c576a85eafeccdc56582dbf43450b71dd4baa741fb05e231.
Ruling C, written after the dry run, is in the working specs/0048,
uncommitted. The gate, the run and the scan took it at PLANNING_SPEC
25b65c82ff7823930e6b090c475defdc501d71e637f659088998b19ea98ec8ef
(gate.log and run.log print it). `<phase>` is the phase's directory,
`<home>/.cache/emend-build/m8p2-p2`.

root: <home> stands for the home directory of the account the runs ran under
root: <phase> stands for <home>/.cache/emend-build/m8p2-p2, the phase's directory
root: <corpus> stands for the private corpus directory the run reads
root: <runs> stands for <corpus>/runs, the corpus's run directories
root: <target> stands for <phase>/target, the phase's one cargo target root
root: <work> stands for run.log's working directory, as run.log's root line 82 defines it
root: <scratch> stands for <phase>, as in gate.log and run.log; independent check 1's materials are under <scratch>/check1

| Role | Commit | Subject |
|---|---|---|
| base | cbef379 | M8 part 2 P1 record: the logs, the README, the check |
| doc | 94fc5a4 | M8 part 2 P2: commit planning docs (the P1 verification and rulings) |
| test | 8586040 | M8 part 2 P2: the shift's tests cover the three gaps check 1 of P1 found |
| code (phase) | c609bf3 | M8 part 2 P2: a comment edit beside a threaded comment is written as on any sheet |

No file name, sheet or table name, note or cell content of any private
workbook is in these logs or in this README. Workbooks are named by id,
the first 12 hex digits of the sha256. The texts quoted below are the
logs' own lines.

## The logs

| Log | Bytes | Lines | sha256 | Last line |
|---|---|---|---|---|
| gate.log | 930651 | 12072 | d00275534278d90c8ee778a2372834bbb207a8170127c2cd16b9eab818b08646 | gate log checks all as expected: yes |
| run.log | 241897 | 3393 | 0646a0c7ace42a7e17e92716519b72393330323e6bbc63970592c5e05081d4a0 | run log checks all as expected: yes |
| scan.log | | | | the final scan, written after independent check 1 and before the record commit, which holds it |

The scripts are <phase>/scripts/m8p2-p2-gate.sh, -run.sh and -scan.sh.
Each log prints the script that wrote it. Each was written by the
generator under <phase>/gen, from P1's scripts as numbered changes. Dry
run d1, under <phase>/dry/d1, ran the same three scripts first, and the
logged driver checked them cmp-equal before starting.

The gate's own start, "run at 2026-11-09T15:44:27Z", is gate.log line
354. The run's, "run at 2026-11-09T15:57:14Z", is run.log line 69.

## The gate

- The prune (gate.log lines 181 to 238; specs/0044 ruling AD):
  - P1's build target was 6511593713 bytes (line 230).
  - It was removed by `rm -r "${PREV_DIR:?}/${n:?}"`, exit 0 (line 232).
  - Every other name of P1's directory was kept; there are 26 kept names
    (line 191).
- The crate's tests on both targets:
  - "the figures this run must give: off 539, ch 714, im 576, ci 756, cc
    762, cr 821, all 1236" (line 11055).
  - cc passed 762 on the host (line 11167) and on i686 (line 11356); all
    passed 1236 (lines 11221 and 11410).
  - Those are P1's figures, 759 and 1229, with this phase's tests:
    - the test-only commit's 3 shift tests (all);
    - the code commit's 5 comments tests, 4 of them in cc (the fifth is
      richtext's);
    - less the one comments test it removes.
- By name:
  - shift passed 47 (line 10598);
  - comments passed 60 with comments, richtext and rowcol (line 10734);
    its filter also selects the boxes and rich submodules (ruling C (3)).

## The run and the expected changes

The four samples, --comment-sample, --richcomment-sample,
--commentbox-sample and --copy-sample, ran over the 39 workbooks with
every output opened. Each was compared per id on outcome, lo and lo_in
with part 1 P2's records `records/m8p1-<name>-2026-11-09-8816db0`.

- Summaries (run.log lines 3118, 3184, 3250 and 3316):
  - comment: same 38, changed 1;
  - richcomment: same 38, changed 1;
  - commentbox: same 38, changed 1;
  - copy: same 39, changed 0;
  - new 0 and missing 0 in each.
- run.log line 3352: "the three pairs of the table are met: comment,
  richcomment and commentbox 15205770ae5c error/-/ok to ok/ok/ok, and
  copy 15205770ae5c kept error/-/ok (WL): yes". These are the changes
  "Harness runs" P2 names, and nothing else changed.
- The run's tables are this phase's evidence, not records; records are
  P4's. The four pairs stay in <work>/records, each hashed in run.log
  (lines 3112, 3178, 3244 and 3310):
  - m8p2-p2-comment-2026-11-09-c609bf3.md 9d07076d..., .txt 33a8ffd7...
  - m8p2-p2-richcomment-2026-11-09-c609bf3.md 6fa56b89..., .txt c5237f09...
  - m8p2-p2-commentbox-2026-11-09-c609bf3.md d1282663..., .txt 86124859...
  - m8p2-p2-copy-2026-11-09-c609bf3.md fdd744a6..., .txt 8953f436...
  - No file is under records/.

## FINDING lines

None. run.log line 3378: "FINDING lines printed above: 0". The other run
lines holding the word are the summaries' and sections' counts, each 0.
gate.log's two (lines 5721 and 5730) are inside the printed script's
carried history.

The comments probe's figure and its source (run.log lines 3368 to 3373):
- `size -A` of the size-profile i686 musl builds at the phase commit,
  under <target>/main.
- probe-emend-comments-write-xml: 1051304 section bytes, file 1055268,
  sha256 b230b37b79cadc8ac36496f4ba3bfbe838cf1ff718bf5345f79019a1490d9dab.
- probe-emend-format-write-xml: 984728 section bytes, file 993828,
  sha256 592a5fd3491c19ef8ccd7154d4f9f6ba97cad95817a15a7b7431b709a691f9af.
- The comments cost is 66576. Milestone 7 part 2's record,
  records/size-m7p2-2026-10-28-43471ed.md, gives 63440. The growth is
  3136.
- Expected 500 to 3000, accepted up to 5000: line 3373 reads "accepted",
  which prints no FINDING.
- Planning's ruling C (1) accepts the growth; P3's size record
  attributes it.

## Figures from files outside this directory

Each is named with its source; none is in a committed log:
- The logged driver, <phase>/logged.sh, wrote <phase>/logged/driver.out:
  - "gate started 2026-11-09T15:44:25Z" and "run started
    2026-11-09T15:57:14Z";
  - "outer stderr 0 bytes" for each step;
  - "m8p2-p1's target after the gate: gone".
- The run's working directory is <phase>/tmp/tmp.Zk89GDSXIA, the one new
  name in <phase>/tmp, from the driver's before and after listings
  (<phase>/logged/phase-tmp-before.txt and -after.txt).

## Leftovers (left in place for the maintainer's deletion list)

- <phase>/dry/d1 with its build roots.
- <phase>/dev-target, where the session built.
- <phase>/tmp/G, R and S, the forks' temporary files.
- <phase>/tmp/tmp.Zk89GDSXIA, the run's working directory.
- <phase>/target, this phase's target root. The next phase's gate prunes
  it.

## After independent check 1

The body above is kept as the check read it, as in P1. Each point it
raised, answered from the primaries:

- Question 1 (a), a box alone on the threaded cell:
  - `record_comments` groups each box with the addition on its cell before
    any part is read. A box on a cell this write adds no comment on is
    refused there with "no comment addition on that cell" (package.rs at
    c609bf3, the NO_COMMENT_ADDITION constant), before the threaded check.
  - So a box reaches the threaded check only beside an addition on its
    cell, and that addition refuses it with "a cell has a threaded
    comment". The spec's "a box ... on the threaded cell refused" is met
    through the addition.
  - The lone-box message is the existing one, tested by
    `boxes::a_box_on_a_cell_this_write_adds_no_comment_on_is_refused_at_the_write`.
- Question 1 (b), "nothing recorded":
  - `refused_with` calls the shared `refused` helper (tests.rs at c609bf3),
    which panics if the write produces an output. Each refusal in the
    test is therefore a write with no output.
  - The test's last lines show only that the refusal repeats on a second
    package. Its comment says more than those lines show.
- Question 1 (c), the copy:
  - `copy::a_copy_of_a_sheet_with_threaded_comments_or_a_comment_edit_is_refused`
    is unchanged in this range. Its fixture holds the threadedComment
    relationship (copy.rs at c609bf3, checked with grep by the session).
  - It ran in the full gate's sets on both targets (gate.log 11059 to
    11410).
- Question 1 (d) and (e):
  - Only Error::Xml from the threaded part's stream is mapped to the read
    message; a failing read of the archive (Error::Io) or another part
    error passes through as raised.
  - The external-target and XML-error refusals have no test.
  - Both are for planning's word.
- Question 2, "every output opened": the run passes --libreoffice for
  each sample, and the lo and lo_in columns are compared per id. As in
  P1, no line prints a per-sample count of LibreOffice outcomes.
- Line 55: the du reading is gate.log line 229, "target: 6511593713
  bytes"; line 230 repeats it as the total.
- Lines 102 to 103: run.log holds 41 lines with the word FINDING:
  - the summaries' and sections' counts, each 0;
  - the heading at 3377;
  - 33 inside the printed script (141 to 2699).
- The draft report's ruling C labels are the session's summary, not
  quotes. Ruling C (7) reads that planning edits specs/0048 alone before
  the record commit, and (7) and (8) are statements. "Nothing else
  changed" rests on the four summaries' same and changed counts (run.log
  3118, 3184, 3250 and 3316), not on line 3352 alone. The final report
  says so.
- The figures sourced outside this directory are named with their files
  in the section above.
