# M7 part 2 P2 logs

specs/0045-publish.md, "The second Excel check (P2)" and "P2: the second
Excel check", with planning's rulings. P2's doc commit 5ae8367, the phase
commit, committed the spec at sha256
b3121df72cb1aac9238151078270928eb467c7b06809de5e5f36c16f3ed412ea and
ROADMAP.md at sha256
466fc0daf4840dda5ea635dce584a1fd01356745b1a2877a87bbd4e0cfad68ea.
Planning's rulings Q to Z, written during P2, are in the working
specs/0045, uncommitted. The stage run took it at PLANNING_SPEC: sha256
2dd9a9b4a12c82dca306fdb530924141f585cc1db46ab08634f0c580072e62e8 (838
lines). The record run took it at PLANNING_SPEC: sha256
0024305601b95e92a16311942f1078ce3eb76642408ba3a1f053f1e813a4d0d2 (1075 lines; rulings
Y and Z). The phase commit is 5ae836744b4bb402ec9a73b84bacd1d44cec89f6; P2
has no code commit. `<phase>` is the phase's directory,
`<home>/.cache/emend-build/m7p2-p2`.

root: <dry> stands for <phase>/dry, the phase's dry runs' directory (specs/0043 ruling Q)
root: <superseded> stands for <phase>/superseded, the phase's superseded directory
root: <session> stands for the harness session's scratch directory, where check 1 wrote one file; not named here

P2 has two logged runs, both in this one record commit (planning's
answer 5 to d1): the stage run (2026-10-28), whose logs were frozen
under `<phase>/logged/` after its check 1, and the record run
(2026-10-29), after the maintainer's Excel results came back. The record
run's driver copied the stage run's three logs here, each at the frozen
hash below (ruling V (2)). The record is
`records/excel-check-m7p2-2026-10-28.md`, sha256
6ca1f6e8a300c1052549f9a3c0deacfc265449c1f8c556aed8c23bcc49b9888b, planning's, copied by
the record run.

The phase's commit, on P1's record commit 58c4f51:

| Role | Commit | Subject |
|---|---|---|
| doc (phase) | 5ae8367 | M7 part 2 P2: commit planning docs (P1's verification and rulings M to P) |

## The stage run

| Log | Last line | sha256 |
|---|---|---|
| gate.log | gate log checks all as expected: yes | 6ce0b43124d95e4bc9e7468914891ac19fbdcfeab38c625d2da422f910cc3d26 |
| negative.log | negative log checks all as expected: yes | d760f1c79c1987e0601b0cf27ae56c464f56a7038ba6f46f77e9b8a7ebb53d63 |
| run.log | run log checks all as expected: yes | 7c7ab5a23276e5dfc167771e5d7621ce4b324727d2f9fd57806a7a89051a4710 |

- The gate (P1's gate, changes A92 to S92), run at
  2026-10-28T02:44:45Z:
  - the root's mount propagation shared, one keepalive process;
  - the one role by full subject, the doc commit the phase commit,
    its parent 58c4f51; no code commit;
  - the prune: "nothing is removed: the prune set is empty (specs/0045
    ruling O; change C92)". P1's directory
    `<home>/.cache/emend-build/m7p2-p1` and its keep are listed and
    checked unchanged, at the prune and again at the end;
  - check.sh nine PASS lines; the test sets as P1's committed figures.
- The negative (part 1 P1's cases, changes NA65 to NH65), run at
  2026-10-28T02:51:44Z:
  - the list script's control LC and refusals L1, L2 and L3;
  - the run script's formula check, extracted by line range: control
    FC and refusals F1 (case (c), another sample's directory) and F2;
  - the stage assertion, extracted likewise: control SC and refusals S1
    (case (d), one byte changed in one staged file, its size kept) and
    S2;
  - the fixtures' run.host.tsv name P1's phase commit 43471ed, as the
    real directories do (change NH65).
- The run (part 1 P1's list, stage and zip block, changes TK to TN),
  run at 2026-10-28T02:51:46Z:
  - P1's six run directories, read from P1's committed run log, each
    checked as P1's phase commit's with its flag;
  - the order from the USAGE constant at the phase commit: `--run`
    `<runs>/2026-10-27-43471ed-formula`, then `--run-plain` edit,
    remove, protect, chartsheet and chart-surface;
  - P1's keep: files 4502, its listing hash
    41afdbbf4ecd142b1b1d0d2663eeecbe764946af7a0a4914513edba3c87cdf4b at
    entry and at the end; P1's open record at its pin, 1ed2350d;
  - the list script's own count line: "corpus rows 162 (with A 19, with
    F 15; 135 from --run-plain); fixture rows 2239 and input rows 2239
    of 2251 kept pairs (12 left out: input not accepted)";
  - the list `<stage>/excel-check-list.txt`, sha256
    dd796c22b4067112e7341c4a365bfa2d254590ad1253cb18e2f5e37fe26890ef,
    1241251 bytes, mode 600: rows 4640; corpus 162, fixture 2239, input
    2239; no row printed;
  - the stage `<stage>` = `<corpus>/work/excel-check-m7p2-2026-10-28/`:
    files 4642 (the list, the opener, and 4640 under files/);
    files/corpus 162, files/fixture 2239, files/input 2239, each its
    source's row count; the opener's sha256 0f2bed0b, the phase
    commit's; "staged files differing from their list rows: 0";
    directories 700 and files 600;
  - the zip `<stage>.zip`, stored: sha256
    df8248324a060831287804f74b5b9b5b103bba1126d6b6bd5b09e84d7b69a7ed,
    124132653 bytes, mode 600; entries 4642, each equal to the staged
    file, each stored; testzip ok;
  - FINDING lines 0.

## The record run

| Log | Last line | sha256 |
|---|---|---|
| record-gate.log | gate log checks all as expected: yes | c3d6e3249978ec02ccad6b90afa04e06c1b0bbe995fd356870da1552cbc0997a |
| record-negative.log | negative log checks all as expected: yes | 77febe49ed73bc43fdce1ac507c737544899184dccc51c97c580da185c6b04de |
| record-run.log | run log checks all as expected: yes | b6688a63df5607c6ab795312a1d323581e3469ac04934cccf6db2ca7aea51cc9 |

- The record gate (the stage gate with changes A93 and A94), run at
  2026-10-29T13:15:30Z (its phase line):
  - the root's mount propagation shared, one keepalive process; HEAD the
    phase commit; the working spec at PLANNING_SPEC, with its control;
  - the tree holds " M specs/0045-publish.md" alone at the record run's
    start (change A93), with a control that refuses an untracked record;
  - PRUNE_PREVIOUS list: "nothing is removed: the prune set is empty";
    P1's directory and its keep unchanged at the prune and at the end;
  - the two older stages and their zips, by name, size and modification
    time, there at the start and equal at the end (ruling T (b)).
- The record negative (the stage negative's frame and list cases, changes
  NI65 to NO65), run at 2026-10-29T13:22:25Z (its phase line):
  - the list script's control LC and refusals L1 to L3;
  - the assembly script `assemble-excel-record-p2.py` at its pin 35f29a5f
    (rulings X and Y); on fictional fixtures RC (exit 0, the six headings)
    and the stops R1 and R2;
  - on scratch copies of the maintainer's results file (ruling U (3)):
    the list, the results file (4640 data rows) and the by-hand file (3
    data rows) at their pins; AC, the unmodified copy, assembles to
    planning's record (sha256 equal, exit 0, nothing on stderr); A1, a
    planted row, and A2, a removed row, each exit 2 with nothing on
    stdout; each STOP: line printed with its row key cut (change NO65);
  - a root line declares `<stage>` (change NN65).
- The record run (part 1 P2's record run, changes TO to TS), run at
  2026-10-29T13:22:27Z (its phase line, which carries part 1's words
  "Milestone 7 part 1 P2 ... run.log"; the record logs' phase lines name
  themselves gate.log, negative.log and run.log, the templates' names):
  - the five inputs at their pins: the record (2461 lines, 637291
    bytes), the assembly script, the by-hand file, the list and the
    results file;
  - the record copied to `records/` byte for byte; the assembly run again
    over the list, the results file, the by-hand file and the machine
    section's three arguments: exit 0, empty standard error, output
    equal to the record;
  - 162 corpus and 2239 fixture table rows, 2401 of 2401 found in the
    list by bytes and sha256, and the input pairs 2239 of 2239 (ruling U
    (4));
  - the totals recomputed by awk, | 4640 | 159 | 3 | 0 | 1878 | 0 | 361 |
    1881 | 0 | 358 |, equal to the record's totals line;
  - decision 6 A (change TS): the corpus counts 159 pass, 3 repair, 0
    fail, the control refusing the results file's counts alone (159 0
    3); each by-hand cell "repair (by hand)" (3 of 3); each by-hand row
    names a corpus row the results file reads fail (3 of 3);
  - the stage's listing equal before and after; the list and the results
    file still at their pins; the older stages and zips equal at the
    start and the end;
  - FINDING lines printed: 0.

## The session's statements (no log prints them)

- Generators and scripts, sha256:

  | Log | Generator | Script |
  |---|---|---|
  | gate | 5c22354a83e7d41229e8df502994d7951676207122915a3a4c164bd52bd0228c | 451365b071e26d83392b023d86f7d7b3b9c08012690ad75f29911021e4de540f |
  | negative | eb9320f51eae2ea396ceebd515625e2823f10953978d56d360c123512f655587 | 085e88764a5d394487b36955d99dcd5bdef739ddf262967d48488f2a03995c30 |
  | run | 0c71a973c58f67e35a6cd11b589b883286ae59739fb40f65473f9c32b2dc7a52 | f9ebe04308fd328212ce85e2a239cecc88bfc0eec16a43ffed6b101416419096 |
  | scan | 612e7249f8ef6294f058f61521e4e8df3a676b1f4516876e57229cc44155b69d | 663febe109a7335dce94a323a05bea4655416332c1b2f1f8d2bcd2fc7aabb8aa |

  The drivers: `<phase>/dry.sh`, sha256
  992be5e22c66e160dd301ce310d45a6d21a4329939beb636ef23c941ed53eeb7;
  `<phase>/logged.sh`, sha256
  44b1c0871ec2cec5f3fdceba742c13e64b8592326661c44108566308e545bd00.

- The generators were written by four forks of the session, one per
  script. They ran no cargo and no script; each wrote its notes under
  `<phase>/fork-notes/`.

- The dry runs, each from the start under `<dry>/d<n>`, each kept:
  - d1 (no PLANNING_SPEC; the tree clean): gate and negative yes. The
    run exited 1: two calls `count_matches -xF -- <flag> /dev/stdin`
    took the `--` as the pattern, so the six flags were never found and
    no directory was read. dir_ok, defined inside that loop, was
    undefined for its control (the script's standard error: "dir_ok:
    command not found"). The script stopped before the list. Change TN
    drops the `--` and defines dir_ok before the loop. The earlier run
    script is kept under `<superseded>/run-gen1`.
  - d2 (the same, TN): gate, negative and run yes. Its scan pre-check
    read RESULT: FAIL on its two dry-layout lines alone (LOGDIR and
    RECDIR not the defaults; output lines holding the home directory,
    every one under `<dry>/d2`), total hits 0. After d2 came planning's
    answers 7 and 8: change LV (the scan takes EXPECT_RECORDS and
    counts the Excel record against it) and change NH65 (the negative's
    fixtures name P1's phase commit). The earlier scripts are kept
    under `<superseded>/negative-gen1` and `scan-gen1`.
  - d3 (PLANNING_SPEC d823f16e..., LV and NH65): gate, negative and run
    yes. Its scan pre-check read RESULT: FAIL on the same two
    dry-layout lines alone, with EXPECT_RECORDS 0 and its control as
    expected; total hits 0.
- The zip's sha256. d2's, d3's and the logged zip have the same bytes
  (124132653) and different sha256 (77d12042..., dd5c209d...,
  df824832...). Each zip entry carries its staged file's modification
  time, which is the copy time of its run. Planning compared d2's and
  d3's zips: every entry's CRC equal, every entry's time different. The
  list's sha256, dd796c22, is the same in all three and is the stage's
  identity.
- The stage run (PLANNING_SPEC 2dd9a9b4..., ruling S) started
  2026-10-28T02:44:44Z by the driver. Gate, negative and run each
  exited 0 and read "yes", with 0 bytes of outer standard error each.
  The tree after it: " M specs/0045-publish.md" alone.
- The record run's generators and scripts, sha256. The generators were
  drafted before the results came back; the drafts are kept under
  `<superseded>/record-gen-draft`. After planning's record brief (ruling
  Y) the session filled the brief's inputs, pinned the assembly script
  35f29a5f, wrote change TS (decision 6 A) and filled the scan's record
  date; the dry runs below brought A94, NN65, NO65 and the record's
  path under `records/` (ruling Z).

  | Log | Generator | Script |
  |---|---|---|
  | record-gate | 353b206bf5865a3c3336e49e450b9bfba88a689a4e9985566758ee2f1edaf076 | faceefd0544e60898572e0f64433d41b4c5425aa0650118b2aa40dd633fb5c26 |
  | record-negative | 8a5a2f39134a7f9ee1fa42f3a9a30de6d5ee3e3e1f6308671b916e11ba7bd80e | 5b2cb9dfaf5f23a0364b694e5dea1b6dd8c5c897ef48cb5f2422822b8dffc2c9 |
  | record-run | d3f554dea4d761860271620dd3b839b990b186dfcc23601eae1001229ffbdaec | 590c78b03dcfe4b30508c2ad4397f7d85ca3df3fdbca810ab109d39a7b1a321e |
  | scan (record) | e3bb9c6a3958ac4a5494afc38757234ffa5fc1dadb976788f3d84ae936fa96e2 | 7a58ada41797581b5ee25668856941cada42fbc02e39fcd78c8c4e33b3d8f219 |

  The scan's generator changed after the record gate hashed it
  (record-gate.log line 186, b8a430dee8a4e481ec7c58bf996cc209c023234bcd39b203431ba9da88e5df91, the
  draft, kept under `<superseded>/record-gen-draft`): the record date
  filled while the logged run went (7d03b6cf5011c4d5ccc5113a50193558835f9db77f56b4c3aaf6ddb97e0fe3ab, kept as
  `<superseded>/record-d1-scripts/scan-record.py.pre1`), then change LX
  (the hash in the table).

  The drivers: `<phase>/dry-record.sh`, sha256
  67379f74d6e4fc7e4ab2750508ad6a9ba1a6073347578d26f65f22f0f2ca670a;
  `<phase>/record-logged.sh`, sha256
  14fd7f3e717986839ae44b470ab63bf8fd80906ca43ba52ad0749ee55c192476.
- The record dry runs, each from the start under `<dry>/record-d<n>`,
  with the gate's and the negative's build roots and the record's
  destination under the dry directory, so the tree was not written;
  each kept. Superseded scripts and standard error files are under
  `<superseded>/record-d1-scripts`.
  - d1: the gate read no on one line, its own standard error (107
    bytes): A93's start check of the older stages called note_fn before
    the script defines it. Change A94.
  - d2: gate yes; the negative read no on change NF42's placeholder
    rule: NM65's command lines print `<stage>` and no root line declared
    it. Change NN65.
  - d3: gate and negative yes; the run stopped at entry (exit 2, "an
    input is not there"): it read the record at part 1's place,
    `<phase>/record-from-planning/<name>`, not under its `records/`.
    Change TP's path.
  - d4: all three yes, and the session found that NM65's two STOP: lines
    printed one corpus row's key after the message. Change NO65.
  - d5: all three yes; 0 FINDING lines; the record copy cmp-equal to
    planning's.
- The record scan's pre-check, under `<phase>/logged/record`: its first
  run read RESULT: FAIL on three structural lines, total hits 0. Two
  counted check 1's answer, which the session had installed here
  before the pre-check (a pre-check expects none); the answer was held
  under `<phase>/logged/record/held` until after check 2. The third was
  change LW's short-sha check, kept in P1's form (every record named
  by the phase's short sha), while the phase's one record carries its
  date: change LX gives it part 1 P2's LN form, none so named. The
  first run's log is kept under `<superseded>/record-scan-pre1`
  (sha256 7109eedae1c4494bb9d929eb396e359e6910cc12ac8ffb3b1da4333247f4cb17)
  with the scan it ran. The second run, with LX: RESULT: PASS, total
  hits 0 (sha256 83cd6d33e82de2c0bc8f95651b275f92a1ed42ac4fcdfb3bc68160474e1d3d6f).
- Finding R1 (ruling Z, R1 (a)): the stage run's negative build root
  `<phase>/negative` (one empty tmp/ of 2026-10-27) was there, and the
  record negative refuses a build root present at entry. Before the
  logged run the session moved it by mv to
  `<superseded>/stage-negative-build`; record-logged.sh unchanged.
- The record run (PLANNING_SPEC 00243056..., ruling Z) started
  2026-10-29T13:15:29Z by record-logged.sh. Record gate, negative and run
  each exited 0 and read "yes", with 0 bytes of outer standard error
  each (driver times: from 2026-10-29T13:15:29Z). The driver copied the six logs here, each copy's sha256 equal
  to its source's and the stage logs' to the frozen hashes above; its
  line "every copy equals its source (and the stage logs their frozen
  hashes): yes" is in `<phase>/record-logged.out`, not in a log. The
  tree after it: " M specs/0045-publish.md", "?? records/excel-check-m7p2-2026-10-28.md"
  and "?? records/logs/m7p2-p2/".

## After check 1 (the stage run)

Check 1 read the spec, the diff, the draft report, the three logs, this
README as it stood then (`<phase>/logged/README-before-check1.md`) and the
scan pre-check, with the tool's flag list and the stage's counts. Its
answer is `<phase>/logged/check1-answer.md` (sha256
6ef88673d23c50ade0bc38f914b62cda713030b4ad23a3a0e4e4bfc401bc4692); its
brief is `<phase>/check1-brief.txt` (sha256
b73ca914ba903683f94abbc01cf9330b28d23566cf04bbf33332081c87387f3a). It is
committed with the record run as `independent-check-1.md` (sha256
017fb1ccdcf831bedfed7865016d14d71c062a6e5aee49215123da5897c4d2ba), verbatim but for the
home directory's path written `<home>` once and the session's scratch
directory written `<session>` once, as its heading says.

Verdicts: 1 yes with one exception (the doc commit's subject names
rulings M to P, while its spec diff also adds rulings A to L); 2 yes with
exceptions; 3 yes with exceptions.
- The run log's directory list and order agree with the USAGE constant.
- The stage's per-source counts agree with the list's, by its own count.
- No log emits a FINDING line.
- The report took its wording points: run.log's two matches for the word
  outside its printed script; the LOGDIR echo line; d2's dry-layout FAIL;
  the script hashes, the dry runs and the outer standard error marked as
  the session's.
- Ruling P's "not read, moved or removed" for the Milestone 2 and part 1
  stages and zips is not proved by a log; run.log refuses their names.
  The check's listing found both present with their old times.

No stage log changed after check 1.

## After check 2 (the record run)

Check 2 read the spec, the diff 58c4f51..5ae8367, the three record logs,
the scan pre-check (the final scan runs after it), the record, this
README as it stood then and the draft report
(`<phase>/report-record-draft.frozen.md`, sha256
e94f3b9892b06c4e923c3094868548c1b333c29281720189dcfc1bfb49aec985). Its answer is
committed as `independent-check-2.md` (sha256
4759222a4b4de8755b43104e18f17d5bdc4e572fe9871d60c0ca6e5c2df36a9d), verbatim but for
the home path and its non-ASCII characters, as its heading says; the
answer as given is `<phase>/check2-answer.md` (sha256
a64694a8d448a18430e0201116a4fb220fb5e6aba4fc584d9d501ad79df328ed).

Verdicts: 1 yes; 2 yes with exceptions; 3 yes with exceptions. It found
no disagreement with the record's figures. Its points and what was done:
- E1, R1: the driver's copy line is in its output, not a log. Stated
  above as the session's; the report moves its quote there too. The
  scan reads the three stage logs at their frozen hashes.
- E2: the pre-check read this README before its last edits. The final
  scan reads it as committed.
- W1, W2: the record run's phase line carries part 1's words, and the
  scan's "record files expected: 1 (P2 commits no record, change LT)"
  carries the stage scan's words beside its count; both are carried
  template text, recorded above and here, not changed in the run logs.
- R2: check 1's answer was held aside for the pre-check; it is back in
  this directory, at the sha256 above, before the final scan.
- R3: the record gate's time is now its log's, 13:15:30Z; the driver's
  13:15:29Z is marked as the session's.
- R4: the scan generator's hash chain is stated above.
- The strictness note (pairs.awk tests membership) is carried part 1
  form; the equal per-source counts cover a repeated row.
