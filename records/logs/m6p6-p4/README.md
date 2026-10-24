# M6 part 6 P4 logs

specs/0043-measurement-and-close.md, "P4: the README" and "The README",
and planning's P4 brief. P4's doc commit 92c6b38 committed the spec at
sha256 de2ec8154ab10fcacd6d59c9217956293b302bf931680f1b2ba9df11969d2848
(rulings A to S and "Planning verification of P3") and ROADMAP.md at sha256
7ae3724496c6221adf90bc26d42a9e045cf5fe0c414cfcc19294f856478b3aa6. The phase
commit is the README commit 37062b7bd644e176ecc451ce3b63f73834ac9eb7.
Planning's rulings T and U, written during P4, are in the working
specs/0043, uncommitted; every log takes it at PLANNING_SPEC sha256
9c6c8dac0cc447bbfa439bcdaaf298b3f87e8933a61485daed54dc796f3ba1ec, and so
does the scan. `<phase>` is the phase's directory,
`<home>/.cache/emend-build/m6p6-p4`.

The phase's three commits, on P3's record commit 42cf079:

| Role | Commit | Subject |
|---|---|---|
| doc | 92c6b38 | M6 part 6 P4: commit planning docs (the P3 verification and rulings) |
| test (code) | bb0a29e | M6 part 6 P4: a strict workbook root binding no relationships prefix takes an add |
| README (phase) | 37062b7 | M6 part 6 P4: the README brought to Milestone 6 |

| Log | Last line | sha256 |
|---|---|---|
| gate.log | gate log checks all as expected: yes | 42f49b22a4358a65ec60427730ecc4925c6b042c6ed02e1e92277deb8473e080 |
| test.log | test log checks all as expected: yes | cf4fe32db443c60cfb10a429ccfe981f6c3a989cec8018adf02e38414d6d00f0 |
| negative.log | negative log checks all as expected: yes | da05769f447b9a9f170cd13991ce2adc8d0f88b6c12c71ea540e420fabad1d30 |
| run.log | run log checks all as expected: yes | 50b4d3d377f1b633a6399e2e11080c0775f009221abcb1069757b515dfb85a43 |

The logs ran in this order: gate, test, negative, run (ruling T). There
is no size log: P4 changes the README and adds one crate test, and the
README cites P3's size record. The negative runs N8's cases through the
gate's debug copies of emend-copy, host and i686, each checked against the
hash line the gate log prints, so it runs after the gate. No log installs
a record; the record commit holds this directory alone. The scan's log,
scan.log, is written last over these four, this README and the check
answer.

## Rulings in the working spec

- T (the generators' readings): the four roles, the README commit the
  phase commit; the order above; N8 through the gate's debug copies; the
  four settable build roots set by the dry driver, the three fixed roots
  (`<phase>/target`, `<phase>/out`, `<phase>/tmp`) shared by the dry and
  logged runs as in P3; nothing added under P3's directory before the
  logged gate; the test log's API blocks dropped; every test figure P3's
  plus one; the transitional-test guard; P3's binary cases carried whole;
  no record installed; the status patterns admitting the spec and not
  ROADMAP.md; the fidelity readers' first-cited-record attribution; stale
  comments carried.
- U (finding F8, below): a generator defect; the gate's I88 line writes
  the one new test's name as a quoted literal, and the generator refuses
  any Python concatenation token in its output, with a planted control.

## What the logs show

- The gate (changes A88 to Z88): the four roles by full subject; the
  prune of P3's target, 1 of 1, 13541824503 bytes, gone after it; the
  test commit's one file, tests.rs 33/0, and the README commit's one file,
  README.md 19/15; test names 1452 at the base and 1453 at the phase
  commit, the one new name a_strict_workbook_root_binding_no_relationships_prefix_takes_an_add,
  none lost; messages unchanged; CHANGELOG.md the base's blob; check.sh
  nine PASS lines; the crate's sets off 539, ch 713, im 576, ci 755, cc 758,
  cr 820, all 1201 on host and i686, P3's each plus one; P3's size
  record's figures printed, the charts probe 1151424, the format probe
  984744, the budget 451528, judged nowhere.
- The test (changes ZK to ZQ): the workspace 1450 on host and i686
  against P3's 1449; the sets off 539, read 646, rules 576, read,rules
  686, all 1201 on both; the strict test passing by name in every run, and
  P3's transitional accepting test too; no crate source naming the README.
- The negative (changes NA61 to NH61): the 22 refused cases N1 to N7 and
  N9 to N23; N8's 14 runs through the gate's debug copies, 6 accepted, 4
  the tool's own active-tab refusals and 4 the control N8P; the README
  check's control NC, the README as committed, passing, and its plants N1
  to N10, each refused as its kind.
- The run (changes RRP to RRZ): the README check over the phase commit's
  README, "sentences 45 with-digit 27 cited 27 uncited 0 missing 0 figure
  0 number 0 feature 0 dash 0 abspath 0 word 0 figures-checked 28", its
  planted uncited sentence refused; each figure read from the record it
  cites by a printed command, from nineteen records; the README's number
  tokens, 46, the figures checked; the features the manifest's.
- FINDING lines (ruling R, searched for the word): none in any log's
  output; run.log's two matches are comment text in its printed script.

## The session's statements (no log prints them)

- The generators were written by four forks from `<phase>/FACTS.md` and
  the models under `<phase>/model/` (P3's gate, test, negative and scan
  and their generators; Milestone 5 part 6 P4's run, its README check and
  its generators), each model checked byte-equal to its committed print.
  Generators and scripts, sha256:

  | Log | Generator | Script |
  |---|---|---|
  | gate | 2aa83d9e98edee8100d2ad5b1ab693d28882ce827b27fe9c45da0193f034e04b | 03199fbaa17e99de8c483cc97302bb330a2bccde7e0b2113596491ee14a0f596 |
  | test | 668821fb3a8d1e87bd1bd9bed6eaae62da87d25f6cf13ce3e47fe05c2aadcbb9 | 24945c9613e2f81ba818f168e911d5153d14f33f8a37dc6cf76c76e8f81e3524 |
  | negative | 5aef98e91e106dbca4180485213d2dec84d3c07be03f74ec8026e3bd6c52baa0 | 2608ed9360f815eeb839c3a6607702b434be3f9e78dc89038bcfc1dcd777dfaf |
  | run | 5b4a553120aa36f3dbde936a35ead8530f1f50c3e5746d9b6864b18c1fbd5877 | b7719ef76d512ffed47f6dadb6180a78ca84de5c7ee1a8a62fe1324d6e685ade |
  | scan | b6daa79d9c0fa08e9943b25a078d0de0b4c61c2d97f5926e52a96a20dab07a14 | 1c9278ec54c30df0e53fe6791ec4d6f7184894bc51058e74efa3df8632b602a9 |

  The run's README check program readme_check.py is sha256
  870e84ba562a90fc691b67ac673eb17b71e5a63c8310f1b4f3f60d5257a5611e and its
  defined-helper check defined_check.py
  82c55b4ab2afb71713292132927bf964c35c0b261c74e7151a04cb76ef4e5d2c.
- The gate's generator was changed twice, its script once. Under ruling T
  its accepted working-spec hashes gained da71e010...; the regenerated
  script was byte-identical, b1d84a87.... The first generator (8a8da454...)
  and script are kept in `<phase>/superseded/gate-gen1`.
- Finding F8 (ruling U): dry run d1 (PLANNING_SPEC da71e010...) stopped at
  the gate, "gate log checks all as expected: no", outer standard error 0
  bytes, test, negative and run not run. One check read UNEXPECTED: "the
  new test is the one FACTS.md names, the strict root's add (ruling R, F7;
  change I88)". The generator had emitted Python concatenation as literal
  shell text, so the expected list held 7 words in place of the one name;
  the measurement itself, one new test of that name, was right. Under
  ruling U the generator writes the name as a quoted literal and refuses
  any concatenation token in its output, the d1 line its planted control;
  its accepted hashes gained 9c6c8dac.... Against b1d84a87 the new script
  differs in that one line alone. The second generator (3dfce789...) and
  d1's script are kept in `<phase>/superseded/gate-gen2`, d1 under
  `<phase>/dry/d1`.
- Dry run d2 (PLANNING_SPEC 9c6c8dac..., dry.sh sha256
  f4b8d30c5d3c47237dd74bea6f0c987f1f057c44c9c062a85d7a42f181a56e0c), every
  settable build root under `<phase>/dry/d2` (ruling Q), the gate's prune
  listed only: each of the four "yes", outer standard error 0 bytes each.
- The logged runs (logged.sh sha256
  a70c8510adfd5b79c6b31fa6a6e84b44e3795666f05483607ea4131793a509b2): each
  of the four "yes", outer standard error 0 bytes each, ended
  2026-10-24T20:48:20Z; no attempt refused or stopped.
- The forks' departures: the gate's R87 CHANGELOG-bullet checks replaced
  by the blob equality R88; the negative's and run's status patterns
  admit specs/0043 and not ROADMAP.md, and with PLANNING_SPEC unset they
  require the working spec to equal the committed blob; the gate's
  generator left twelve scratch directories in `<phase>/tmp`; two
  gate-parts files in P3's form, spec.sh and docs.sh, were moved aside to
  `<phase>/superseded/gate-parts-copies`, the generator reading P3's parts
  from `<phase>/model/m6p6-p3/gen/gate-parts/` instead.

## After the independent check

The check (independent-check-1.md, a heading line then its answer
verbatim, sha256 07428a82bada8f055b7938309d0f9f9c57224a7b52428620098cbf8bdf62e773
for the answer alone) read this README as it stood before this section
(sha256 47002ad12907e454d83ab13086fa3058303a3341306185d48920d7056b297ea9);
nothing above it changed after the check. Its prompt is the brief file
but for that file's final newline; the draft inline in it is the frozen
draft byte for byte (session's statement). Its verdicts: 1 yes, with a
doubt; 2 yes; 3 yes with exceptions, no figure missing or differing. Its
points:

1. **The comparison paragraph's reach** (its 1, a doubt): "The README"
   asks it to state per op or for the set what each tool left
   byte-identical, dropped and opened; the paragraph gives identical and
   dropped for the roundtrip and opened for the set, Milestone 5's form
   carried. Raised to planning with this record.
2. **Dry run d2** (3 a): the report's "Dry run d2 then passed all four
   steps" is the session's statement, carried by this README's list of
   them; no d2 log is a primary.
3. **"Nothing pushed"** (3 b): the session's statement; origin/main reads
   ee78ffa.
4. **"The gate's two regenerations"** (3 c): the generator changed twice
   and its script once; the first regeneration (ruling T) was
   byte-identical, the second (ruling U) changed one line.
5. **"7 words"** (3 d): the expected list held seven lines, one of them a
   single space; "words" is loose.
6. **Nineteen and twenty** (its note under 2): the run reads figures from
   nineteen records; the README cites twenty paths, the twentieth,
   records/toolchain-2026-09-15.md, in a sentence with no checked figure.

No code, log or record changed after the check; nothing was rerun.
