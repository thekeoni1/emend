# M6 part 6 P3 logs

specs/0043-measurement-and-close.md, "P3: the close", and planning's P3
brief. P3's doc commit 7f3b43c committed the spec at sha256
cc1b5d902817c7fd9b792aa56c8a5cdfb9b4132cc71e228ed2759415c550addc (rulings
H to K and "Planning verification of P2") and ROADMAP.md at sha256
d585c8e84f34fae994795a789444e3e2788a2460b429215df0f58cf8ae36c054. Its code
commit 7ac8b74d8f280fbdc9bea0cc2cdf631e9d11a2ee is the phase commit.
Planning's rulings L to Q, written during P3, are in the working
specs/0043, uncommitted; every log takes it at PLANNING_SPEC sha256
9c3809ca67b61ffda86e7d9a76496b282608997e9c509ffa72b98a54991774e9 (rulings
L to Q), and so does the scan. `<phase>` is the phase's directory,
`<home>/.cache/emend-build/m6p6-p3`.

The phase's two commits, on P2's record commit 3102354:

| Role | Commit | Subject |
|---|---|---|
| doc | 7f3b43c | M6 part 6 P3: commit planning docs (the P2 verification and rulings) |
| code (phase) | 7ac8b74 | M6 part 6 P3: a sheet add on a workbook root binding no relationships prefix |

| Log | Last line | sha256 |
|---|---|---|
| gate.log | gate log checks all as expected: yes | 656fe9033d6cc9969cc474bec4217f4b40a65e2a861c85c6e7f8bbc95b460897 |
| test.log | test log checks all as expected: yes | 3ef1e30a0b470212c36a19765a634636c9b36256395006fb948c3cede23f2167 |
| size.log | size log checks all as expected: yes | 9a574cdd4068200641f47213bc0ff3e356efc6b4b9a1adec608f6650da4d8d36 |
| negative.log | negative log checks all as expected: yes | db20292bf8f157dfb02d22db1a361c357397a9ba4b57f9989c7ed6f6f4ef632e |
| run.log | run log checks all as expected: yes | ae943f096adf126832bc1338791e8f59d8fe7386ce85118a90167be8b0b2fcf6 |

The logs ran in this order: gate, test, size, negative, run (ruling P).
The negative runs N8's accepting cases with the size log's release
emend-copy, host and i686, checked against the hash lines the size log
prints, so it runs after the size log; the run reads the same binaries.
The scan's log, scan.log, is written last over these five, this README,
the check answer and the 10 record files the logs installed.

## Rulings in the working spec

- L: P2's prune departure (P1's prune_dir for one directory) accepted.
- M: the fix's form. When the workbook root binds no prefix for the
  relationships namespace, the new element is `<sheet name=".."
  sheetId=".." xmlns:r="<the relationships namespace>" r:id=".."/>`, the
  transitional or strict namespace as the root's main namespace is; a root
  that binds a prefix gets the bytes it got before; a prefixed root's add
  is still refused (SHEET_ADDED_TO_PREFIXED). The in-test openpyxl form;
  the negative's accepting case.
- N: the 1968 words stand in specs/0038 ruling O; the size record cites
  ruling O and names specs/0043 ruling N as the correction.
- O: N8 turned into the accepting case, with N8P its control; G-R1, the
  CHANGELOG bullet inside the code commit; S-R1 (a), the feature cost
  movements printed unjudged; R-R3, the run log prints the exit status,
  the output's sha256, the new `<sheet>` and LibreOffice's result for each
  of the four openpyxl-form runs, with no record; R-R1, R-R2, R-R4 to
  R-R7 and T-R1 to T-R4 accepted.
- P (finding NG-F1): option (a), the sheet sample on N8 and B4 is the
  tool's own active-tab refusal (those fixtures have no bookViews; it
  predates the fix); the read-back is --survey; the order above.
- Q (finding F5, below): the logged script's pre-flight refusal stands as
  the pre-flight doing its work; the dry run's leftover moved, the logged
  script rerun from the start.

## What the logs show

- The gate (changes A87 to Z87): the three roles by full subject; the
  prune of P2's target, 1 of 1, 6250546308 bytes, gone after it, the
  34 kept names accounted for; the code commit's four files (CHANGELOG.md, package.rs, its
  tests.rs, tests/chartsheet.rs) and CHANGELOG.md's one bullet under
  "### Fixed" citing specs/0043; the crate's distinct messages 155 at the
  base and 154 at the phase commit by the one-line reader, 169 and 168 by
  the several-line reader, ruling M's one gone and none added; the crate's
  test sets off 538, ch 712, im 575, ci 754, cc 757, cr 819, all 1200 on
  host and i686.
- The test (changes ZD to ZJ): the workspace 1449 on host and i686; the
  sets off 538, read 645, rules 575, read,rules 685, all 1200 on both;
  the API listing against M6 part 5 P3's, 844 rows, none gone, none new;
  the record records/api-review-m6p6-2026-10-24-7ac8b74.md installed.
- The size (changes RL to RS): the format probe 984664 to 984744, the
  budget's growth 80 within part 5's headroom 7304, 7224 left under the
  ceiling 458752; the charts cost 166680, moved 16 over part 5's record;
  the other costs' movements printed unjudged (ruling O, S-R1 (a)); the
  record records/size-m6p6-2026-10-24-7ac8b74.md installed, citing
  specs/0038 ruling O with specs/0043 ruling N as the correction.
- The negative (changes NA60 to NG60): the 22 refused cases N1 to N7 and
  N9 to N23; N8's 14 runs, 6 accepted (the sheet sample over the kept
  openpyxl output 834a7f6d, the chartsheet sample over it and over N8,
  each on both labels), 4 the tool's own active-tab refusals (N8 and B4
  under the sheet sample) and 4 the control N8P refused with
  SHEET_ADDED_TO_PREFIXED's message.
- The run (changes OAA to OAG): the sheet, chartsheet and copy samples
  over the 27 workbooks, each rerun identical by sha256 to its reference
  run (sheet host 27; chartsheet and copy host and i686, 27 each); the
  four openpyxl-form runs exit 0, lo_in ok and lo ok, the sheet output
  7e77ce69... and the chartsheet output fb5b4f53..., the same bytes on
  both targets, each new `<sheet>` in ruling M's form; the open check,
  the keep 2240 against records/m6p6-open-2026-10-24-d88f735.txt's 2237,
  every record row found, the 3 new pairs exactly those this phase's
  tests write; the records of the sheet, chartsheet and copy pairs and
  the open pair installed.

## The session's statements (no log prints them)

- The generators were written by forks from `<phase>/FACTS.md` and the
  models under `<phase>/model/` (P2's gate, test, negative, run and scan;
  part 5 P3's test, negative, size and run and their generators; part 5
  P2's gate), each model checked byte-equal to its committed print.
  Generators and scripts, sha256:

  | Log | Generator | Script |
  |---|---|---|
  | gate | c78e4c9346d4af2eaa0f2443535003c1302cd3c12dd111d54ad7070684ed6f94 | b56a0189976eb2ac18b23f31b4ce06f96975144f53be294c1cc8effc541305a6 |
  | test | 1428e1cb32aac454e9cb3c3cfc1f82fca5b595d0068fa2fbefe6e9ab629f47b9 | 0d36b0770df384ef53601f90dc06164e7bfb1a18b820ea810afd1e5af5f09f9f |
  | size | 087ccff8db0022e7892f31cc204026af12221132affa5a23033e5394f8d174ca | 1cce8d315a0de376e593da552845c6b04698200b2294824a100a4ad6c102c850 |
  | negative | e5d4b9a43157390825c367ac1738f3aea37efa97f181bd0ff4cdfac642a29f81 | 72ed99171ab7799d912786babc3726733289b3a3078a0f467f5a56218b6b6c9a |
  | run | 7a7379467c15af1794e7d4f78203ab9deb141668f68093ca4f2882f3c472c289 | f7dbf6133b7359cd9fe306981f5d8fb1da74ea544773d2faa04ad92e675ca0fc |
  | scan | 3c2fd2c300ed092a23aa39512b526d3afa4214ba2d4602de1c511a4b443e723a | 38da22d71e9cd356200ea3f6f6d7d85b7f5336f9e55ccd758c8583a049ea82db |

  The test's formatter make_review.py is sha256
  3df644950e3f83c51119701d4167b698d23a5b159a916cf48701c177bd753b45.
- Dry run d1 (PLANNING_SPEC 72f241bc..., rulings L to P), in the order
  above, its builds, records, runs and keep under `<phase>/dry/d1`: each
  of the five "yes", outer standard error 0 bytes each.
- Finding F5 (ruling Q): the first logged attempt (logged.sh sha256
  d77621d2843beeb90291c04f54181c65a281550d71a2b5c2244353f001b662cc,
  PLANNING_SPEC 72f241bc...) exited 2 at its own pre-flight, printing
  "negative exists" and measuring nothing: no step ran and no log
  directory was made. The dry driver set no NEGATIVE_BUILD, so dry d1's
  negative used its default build root `<phase>/negative`, left holding an
  empty tmp/. Under ruling Q the directory was moved to
  `<phase>/dry/d1/negative-build`, the attempt's output and exit files to
  `<phase>/superseded/logged-refused-1` with a copy of its logged.sh, and
  the logged script rerun from the start. No step had begun, so there is
  no RERUN_NOTE. Ruling Q's addition moved the working spec to 9c3809ca...,
  so the rerun's logged.sh (sha256
  e4764992cea60a2e03bda4948ecfffb10f1280a6612b75d68338041b1ef3e29d) differs
  from the refused copy in two lines alone: the PLANNING_SPEC value and
  its header's "rulings A to P" made "A to Q". The rerun ended
  2026-10-24T19:37:04Z.
- The negative's readings, accepted as facts: NG-R1, the order above and
  why; NG-R2, the kept openpyxl output is read at run time from
  `<home>/.cache/emend-build/m6p1-p2/tmp/...`, read only, and nothing
  pruned it before the logged run (the gate prunes only P2's target);
  NG-R3, the new `<sheet>` names the logs print ("emend", "emend
  chartsheet") are the tool's own constants, fictional.
- The forks' departures: the gate's crate figures are read from part 5
  P3's committed gate.log, the generator checking P2's gives the same
  seven; the run judges its reruns by identity with the reference run
  and carries no per-sample line rule (R-R2, accepted under O); the sheet
  reference run holds the host label alone, so the i686 sheet rerun is
  held to the host reference rows and to the host rerun (R-R1, accepted);
  the scan keeps its template's stale comments. Earlier generator outputs
  were moved aside, none run: `<phase>/superseded/gate-gen1`, `gate-gen2`,
  `test-gen1`, `size-gen-try1`, `size-gen2`, `size-gen3`, `run-gen1` and
  `run-gen2`.

## After the independent check

The check (independent-check-1.md, a heading line then its answer
verbatim, sha256 d11e35a0032d350a6372bfaca0b6ac82ddd5dcbaf66f313e66b71f2a0aab9572
for the answer alone) read this README as it stood before this section
(sha256 6c83593bb9ee8f8e296bff3d3ac8b0f589f1fa044764f89285e32661dd174e27);
nothing above it changed after the check. Its prompt is the brief file
but for that file's final newline; the draft inline in it is the frozen
draft byte for byte (session's statement). Its verdicts: 1 yes with a
doubt; 2 yes; 3 yes with exceptions. Planning's ruling R, written after
the check, moved the working spec to sha256
6480fb11dea5ee48e49d5e84ef30eddd02ed1aee0f6cc62cb49a630905ba5393 (ruling R
added after Q, nothing else changed), the hash the final scan takes; the
logs stand at 9c3809ca.... Its points:

1. **The strict pairing untested** (its 1): no test exercises the strict
   branch of ruling M's declaration; both tests are transitional. Finding
   F7 under ruling R: P4 adds one crate test, a strict root binding no
   relationships prefix taking an add, in a code commit of its own.
2. **Ruling L committed** (3 a): the committed spec at 7f3b43c holds
   rulings H to L, not H to K as the first paragraph says; the rulings
   uncommitted in the working spec are M to Q (and R since the check),
   not L to Q.
3. **The size log's FINDING** (3 b), finding F6: size.log and
   records/size-m6p6-2026-10-24-7ac8b74.md print "FINDING:
   probe-emend-tables-write-xml: its growth over M6 part 5's record, -32
   section bytes, shrank 32, a FINDING (expected 0 to 80, accepted up to
   2080)". Dry run d1's size log printed the same line, and the session's
   report of d1 left it out. Under ruling R it stands as printed: the
   carried form records a shrink and does not fail, specs/0043 "Size"
   names no lower edge, no source of that probe changed; no rerun.
4. **logged.sh's two-line change** (3 c): accepted by ruling R, ruling Q
   having moved the spec; the logs print 9c3809ca....
5. **The sheet sample's i686 rerun**: "What the logs show" names the
   sheet sample's host rerun alone ("sheet host 27"); the run log also
   holds its i686 rerun, 27 of 27, held to the host reference rows
   (R-R1).

No code, log or record changed after the check; nothing was rerun.
