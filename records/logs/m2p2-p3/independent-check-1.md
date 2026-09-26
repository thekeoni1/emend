# Independent check 1, M2 part 2 P3

- Checked commit (the phase commit): 1fc8327183963718e3bf66f2c1aa8ee5b63bb995
- HEAD when the check ran: 1fc8327183963718e3bf66f2c1aa8ee5b63bb995
- Checker: a fresh general-purpose subagent, given only the directory <scratch>/p3/check1 and the three questions of CLAUDE.md
- Answer: 16037 bytes, sha256 5d8f5ccced2df7abe979da95c0927c188ce2132a636ef31d07e8fbf802e66795, committed below verbatim; it is plain ASCII and names no machine path, so nothing was replaced
- Inputs, each with its sha256 (sha256sum over <scratch>/p3/check1):

    caa01ca57bf4a1fba976ea68c0ae09d16df7f07c0eda8718d8624e8598202049  diff-1fc8327.patch
    a8a34a02142d9b980642a828505e59575ba099162cebc68b0d09c12384bb67b3  diff-commands.txt
    87fd1604a9d6beef11ceb07679e10492794b522e6ad03a343575521ca77212d6  diff-doc-1f85cba.patch
    eca4692eebaaa7a46bec36a2ddea56e3e747702fa9b95f638e405995ba5b58d1  diff-f64063e.patch
    b52c346160a86fb7946e69c6afd6f9c86f1bfa09864f8772a5aeb8dc5e25823b  diff-ff76e34.patch
    e0c70cb3e5d4b25d8195aa6c7470f0ddc1376c746967669642162654222f38d7  diff-phase.patch
    6e58686c7d0b2d9fd36bf96234aa80e7056d9b722ae9842742dfa5cf28692009  log-gate.log
    e065b6bebd1895806752480e91244cfffe217822a8ef493bd6b87ccfc67266b2  log-negative.log
    7c2a3c7af837d0b5f9e9e8f6276afa87b31e4b593cffe5c1ac165cca186717c0  log-run.log
    6732c39255e1a87bd35d6ae32b15917dd69a03944a05cfb7d78df8ff6204ed97  log-scan-pre-check.log
    a36f1e56fead241d7304c89250c9b65650d0c8d8f63a3cef2f4c4ebe226c887f  log-size.log
    40b58768b13eed988b69ea2784d0da793dfd474402932023b1400045fe601e27  log-test.log
    c485dde334103f5666c360744e253e5fab92edf516282b3e2d39a336d8dc586d  record-m2p2-open-2026-09-26-1fc8327.md
    ae0580212410d70ece63ff8f4a1767606bdc8d033603494d6a57a59adc71a9b1  record-m2p2-open-2026-09-26-1fc8327.txt
    1e467e53fe273cc13b6682d3e9399bd9d12811d199bf7faf99c921ef5963d55b  record-m2p2-p3-border-2026-09-26-1fc8327.md
    3f9ecc9c2c363c4d7d8853328ee4213d34aa0429795f8e4562f2992a0b0db128  record-m2p2-p3-border-2026-09-26-1fc8327.txt
    d8b28788b8cda224653d76edbb8c4696961f19476b17634c106d0af90531cea1  record-m2p2-p3-font-2026-09-26-1fc8327.md
    68ef8d9cc78843f5ff26f968925a11099960bd445f777fef29e4189649eb4afc  record-m2p2-p3-font-2026-09-26-1fc8327.txt
    cb66ade27f3ec59e548bae730fea0ee00de1b8bce337840b62336295c6555b3b  record-size-m2p2-2026-09-26-1fc8327.md
    89f5578e598651a66018a533c7b0fbbc083e3d843933cfb68aaab4e93d0b416f  report-draft.md
    79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c  specs-0001-bootstrap.md
    bcab8a1f649ae5e063ed7de430f8d33419f24f9dba9305d3c43ca46e90e1cf65  specs-0017-fuller-styles.md
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-gate.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-negative.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-run.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-scan-pre-check.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-size.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-test.txt
    5637dbf2efde3d002b69d7dc8491250de5130eff38d36584f478f99e6733b688  superseded-gate-attempt-1.log
    e13a2681ff44e17dfcfffd75276b07c90af260a92edf78aa07ddeb9e62841089  superseded-gate-attempt-1.stderr
    9818a506fb4ee5d5a8ef08e83d82c76d014a9763df6e26b3d098116c5bdf09d0  superseded-m2p2-p3-gate-attempt-1.sh
    9e9cdd0814de6d93e08d641a4f9fbb63b73a17fe2d270a88191a5a9acbd62449  superseded-m2p2-p3-run-attempt-1.sh
    9362d874ff63c9ebc50a4dc750d9e37ac40d8d9cf05ea037f13ba6da7a3ccecd  superseded-m2p2-p3-scan-pre-check-attempt-1.sh
    529a341f073133f2e3fa7a90964407399f54c82dffdee12ff076538426c1b695  superseded-m2p2-p3-size-attempt-1.sh
    7b1b5423fc1166eafeb4047627035cca00e1749bb0e1f6df5dba56bbeb9ab710  superseded-records-attempt-1-m2p2-open-2026-09-25-1fc8327.md
    ae0580212410d70ece63ff8f4a1767606bdc8d033603494d6a57a59adc71a9b1  superseded-records-attempt-1-m2p2-open-2026-09-25-1fc8327.txt
    88bd757538a9e5bd795143e1a7341694346b942a09936712a0edeb5e63c8d0d4  superseded-records-attempt-1-m2p2-p3-border-2026-09-25-1fc8327.md
    a8a0fa35d2899622d02158b9154776fec38b914770cbe04330996898ecdfb983  superseded-records-attempt-1-m2p2-p3-border-2026-09-25-1fc8327.txt
    964e11aa6357f4525df265df4d88c0124c5963d91892f0ed40069d51a11fea22  superseded-records-attempt-1-m2p2-p3-font-2026-09-25-1fc8327.md
    11de55ad79a7d75aa30da26c280e8dab1168c953b5e546cff1b7b036756c70c3  superseded-records-attempt-1-m2p2-p3-font-2026-09-25-1fc8327.txt
    7a996f331aeb1ea1891e12f65420cc7b5710061dfa9f19bd7e2e116c727eb578  superseded-run-attempt-1.log
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  superseded-run-attempt-1.stderr
    4b907b6c377e2439d5213b679257befb91d8281524d8d7fd7f98052c81208d47  superseded-scan-pre-check-attempt-1.log
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  superseded-scan-pre-check-attempt-1.stderr
    c07befb6da056f168087e089b5b58269b3e8d4c846318a6532dfa698f60e0179  superseded-size-attempt-1.log
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  superseded-size-attempt-1.stderr
    9d29b44a277b82dba85e951bc75e087c0366ecda0f0bd21b7556b2b98384eb39  superseded-size-record-attempt-1.md

---

# Independent check 1, M2 part 2 P3

The harness gave me a CLAUDE.md and auto-memory context; I did not use either as evidence.

Inputs read: every file in <scratch>/p3/check1, and nothing else. I ran no cargo, git or build. I used grep, sed, awk, diff, sha256sum and python3 on these files only. The spec copy's sha256 is bcab8a1f649ae5e063ed7de430f8d33419f24f9dba9305d3c43ca46e90e1cf65, the hash the brief gives.

## Question 1: does the diff do what the spec says for P3, and nothing else?

Short answer: yes, with the remarks below. There is one claim about code I cannot see (F5).

I checked how the per-commit diffs relate to the phase diff. The phase diff, diff-phase.patch, is exactly the union of the four per-commit diffs:
- ROADMAP.md and specs/0017 equal diff-doc-1f85cba.patch.
- styles/tests.rs equals diff-ff76e34.patch.
- The tools/size-probe files equal diff-1fc8327.patch.
- The package.rs and styles.rs hunks are ff76e34's followed by f64063e's.

I compared these with diff. The doc commit's 80 added spec lines appear verbatim in the working-tree spec file here.

- F1 (remark). Doc commit 1f85cba (diff-doc-1f85cba.patch) touches only ROADMAP.md (the P2 and P3 rows) and specs/0017 ("Planning verification and rulings on P2" 1 to 8).
  - log-gate.log lines 1195-1223 check both blobs against the hashes in planning's message: 66c632ab... and ce1aebf7...
  - Nothing else is in that diff. Confirmed from diff-doc-1f85cba.patch and diff-phase.patch.

- F2 (remark). Code commit 1, ff76e34 (diff-ff76e34.patch), is the first code commit, as P2 verification ruling 5 and ruling 8 require. It carries three things:
  - The styles limits sentence on an unbound prefix in an xf child (styles.rs header).
  - The test styles::tests::an_xf_child_with_an_unbound_prefix_is_refused, with the same child under a bound prefix read as the control.
  - A two-line extension of the package.rs catalogue entry for "an element or attribute uses a prefix its document does not bind", naming the test.

  The catalogue line is slightly more than the ruling's "a styles limits sentence and a test". It is in scope because it names the pinning test beside the message, as the catalogue does elsewhere. There is nothing else in diff-ff76e34.patch.

- F3 (remark). Code commit 2, f64063e (diff-f64063e.patch), is doc comments only.
  - The package header gains the fuller-styles limits block, covering:
    - the caller's picture;
    - what is not read;
    - what is kept, with ruling 7's first main-namespace alignment/protection and the extLst child kept raw;
    - what cannot be written;
    - the missing-collection refusal;
    - that a consumer may not show a font name it does not have;
    - the pending Excel check.
  - The styles header gains:
    - the sharing of equal additions;
    - the "cannot be written" list (decision 2);
    - the widened list of refusals.
  - Every item in "Module `package`"'s "Consequences a caller should know" paragraph appears in the new text.

  Nothing else is in diff-f64063e.patch. Three small points:
  - The package block's refusal sentence names "the three messages catalogued above" plus "a style index is out of range". It does not name the widened two-containers message or the unbound-prefix refusal. The styles header names the unbound-prefix refusal, and ruling 5 asked only for a styles sentence, so this is a remark.
  - Two added doc lines run to 88 and 89 columns: diff-f64063e.patch lines 42-43, the two intra-doc links. The rest of the header wraps at 80. The gate's fmt step passed (log-gate.log line 1504).
  - The commit message says "The catalogue already lists the part's three messages and the widened one". The catalogue body is outside the diffs. The only supporting evidence is indirect: test.log line 819 shows archive::tests::the_crate_refusals_carry_their_catalogued_messages passing.

- F4 (remark). Code commit 3, 1fc8327 (diff-1fc8327.patch), adds the probe and its [[bin]] entry, and extends two feature comments in tools/size-probe/Cargo.toml.
  - The probe does "Size"'s work: add_style on base 0 of a Style with
    - a Font with a command-line name, a size, bold and an Rgb colour;
    - a solid Fill with two colours;
    - a Border of four thin sides in a Theme colour, with both diagonals;
    - an Alignment with every field;
    - a Protection.
  - It then runs set_cell_style on the number cell and, after the write, prints fonts, fills, borders, the cell's style and the xf's three ids.
  - One deviation in detail: the three ids are read at `styles.cell_xfs.get(full as usize)` (diff-1fc8327.patch, the `full_ids` binding). That is the index add_style returned, not the style read back from the cell through Package::cells, which is what "Size" words as "that xf's three ids". The cell's style is read and printed (`full_style`, the same variable as `number_style`), so no composer is dead and the figure is unaffected.
  - The tint of -0.25 and size 11.5 are the probe's own choices, which "Size" leaves open.
  - log-gate.log lines 1335-1350 confirm the manifest changes only by comments and one [[bin]] entry, with no Cargo.lock change. Nothing else is in diff-1fc8327.patch.

- F5 (claim about code I cannot see). Ruling 5 asks for a test "through `Package::styles` on a byte-built part". The test calls helpers `refused`, `part` and `read`, whose bodies are outside every diff. I cannot confirm from the files here that they go through Package::styles. The same applies to 1fc8327's module comment saying it is "`probe-emend-sheets-write-xml`'s work, unchanged".

- F6 (remark). log-gate.log lines 1476-1496 show that no public fn changed (89 against 89) and no base test changed.

## Question 2: do the logs contain every required line?

All six logs contain the following, and each ends with its verdict line at "yes" or PASS:
- The three shape greps with a fixture control, before the script is printed.
- The phase commit given, compared with `git rev-parse HEAD`, with a null-commit control.
- Root lines.
- The printed script.
- Its own stderr counted at 0 with a one-byte control.

The six stderr-*.txt files here are all 0 bytes.

I recomputed the placeholders printed before a slash in each log against its root lines, and every one is declared. This agrees with log-scan-pre-check.log lines 1578-1599.

My own run of the three forbidden-shape patterns over every file here finds none in any log, script or superseded script. The only hits are in specs-0001-bootstrap.md, where the rule text describes the shapes: 1 chained and 1 leading-bang.

I computed the sha256 of every file here. Each hash the scan cites for the five sibling logs and the seven record files matches the file here.

Rerun headers: every hash and byte count in the four rerun headers matches the superseded file here:
- log-gate.log lines 51-53 (3 files);
- log-run.log lines 30-38 (9 files);
- log-size.log lines 43-49 (7 files, including the pre-check scan attempt 1 and its script);
- log-scan-pre-check.log lines 44-45 (2 files).

The scan's attempt-2 script is byte-identical to superseded-m2p2-p3-scan-pre-check-attempt-1.sh (I extracted it from log-scan-pre-check.log lines 81-957). The size script's attempt 2 differs from attempt 1 by the header note G and the two root lines only, as its header says. Size attempt 2's figures, hashes and verdicts are identical to attempt 1's.

- F7 (gap in evidence). The negative log's list of reachable refusals leaves one out.
  - log-negative.log line 719 says the unbound-prefix refusal of an xf's child "is the styles reader's, reached by any operation that reads the part". It then defers to the crate test.
  - "P3" asks for "every refusal of this part reachable through the binary run once". The refusal is this part's: P2 introduced it, and ruling 5 kept and pinned it in P3.
  - By the log's own words, --font-sample or --border-sample on a fixture with an unbound-prefix xf child would reach it. No case R6 runs it.
  - The spec's parenthesis "(the two collection refusals; ...)" may have been meant as exhaustive. Planning should rule which reading holds.

- F8 (remark, a wording point). log-negative.log lines 361 and 719 say the tool's constants are "a valid name, size, tints and no rotation". The "Harness runs" constants carry no tint: F1 and L1 use Rgb and Indexed, and B1 uses `Theme { index: 1, tint: None }`. The tool source is outside the diffs, so I cannot check what it actually holds.

- F9 (defect in a log header). log-run.log line 28 (the rerun note) says attempt 1's keep was moved to <home>/.cache/emend-build/m2p2-p3-keep-host-attempt-1 "before this run; each is hashed below". It is not:
  - Lines 30-38 hash the attempt-1 log, stderr, script and six record files.
  - Lines 39-50 give each moved run directory's two summary sha256s and a file count.
  - No line hashes, counts or lists the moved keep.

  The same note says the installed record copies in <repo>/records were "removed after a byte comparison". The moved copies are kept and hashed, so the rule "hashed and kept" holds for the records, but the keep is named with no hash.

- F10 (gap in evidence). The run log and the committed size log are not the same attempt.
  - log-run.log lines 2797-2810 read "records/logs/m2p2-p3/size.log: sha256 c07befb6...". That is superseded-size-attempt-1.log (221511 bytes), not the committed log-size.log, whose sha256 is a36f1e56...
  - The run ran from 00:26:35Z to 01:01:19Z. The size log's attempt 2 ran from 01:02:49Z to 01:07:32Z, after it.
  - The binary check still holds in substance. log-size.log lines 3437-3441 and superseded-size-attempt-1.log lines 3420-3424 are byte-identical (I compared them with diff), and log-run.log shows each binary's sha256 equal to those lines.
  - However, no log prints that comparison between the two size attempts by command. The committed size record's sentence that "the run log compares the sha256 of what it runs with the one this log printed, reading it out of this log by command" is therefore true only of attempt 1's log.
  - The run log's header does not say it ran against a size log that was later superseded.

- F11 (remark). The logs ran against two copies of planning's spec:
  - 609a94f94f263effd326cfa021d37c54c731736708c4417e66c2e523d7b6a266 in gate (line 1538), run (lines 2791 and 5402) and size attempt 1;
  - bcab8a1f... in size attempt 2 (lines 62 and 3597) and the scan (line 1503).

  Each log checks its copy against "the hash planning gave", so each is consistent with itself. I cannot see the 609a94f... copy. It presumably lacked P3 ruling 2. Neither the logs nor the draft report say that planning's spec changed between logs.

- F12 (remark). Ruling 1's names for the run directories are `runs/<date>-<sha>-font` and `-border`. The runs used `2026-09-26-1fc8327-b-font` and `-b-border` (log-run.log lines 66 and 108; script header F, RUN_SUFFIX). The record stems are as ruling 1 gives them. The `-b` is disclosed in the script header but not in the rerun note or the draft report. The date had already changed from attempt 1's 2026-09-25, so the suffix was not needed to avoid a name clash.

- F13 (remark). The scan's pre-check attempt-2 header (log-scan-pre-check.log line 42) hashes attempt 1's log and stderr but not its script, saying the script is unchanged. The script's hash appears in log-size.log line 49, and I confirmed it byte for byte against the printed script. Ruling 9 of P1's verification is met through size.log rather than the scan's own header.

- F14 (remark). The size record is not installed by any log. log-size.log line 3595 counts "its size record: 0" in the tree at its end, and the scan at 01:07:46Z already hashes records/size-m2p2-2026-09-26-1fc8327.md (cb66ade2...). The install happened between the two, unlogged. I compared the record printed in log-size.log (the section "the size record, written to <scratch>") with record-size-m2p2-2026-09-26-1fc8327.md, and they are identical.

- F15 (remark). Every printed script in the six logs has its `set -uo pipefail` line two to four times over, carried from earlier phases (for example log-gate.log lines 127-129). This is harmless.

The size figures are all present in log-size.log:
- the fourteen probes' file, section and .text figures (lines 2454-2525);
- part 1's thirteen rebuilt at 975c974, reproducing part 1 P5's sha256, each classified: identical 3, code 10, with commits named;
- the budget figure 399176, with 59576 under 458752;
- the own cost 2036;
- the whole growth 23192 = 2036 + 21156;
- the dates cost 4808.

The run log has both samples on both targets:
- 27 of 27, with byte identity to P1's run directory (font) and P2's (border), 27 of 27 on each label;
- 22 font plants and 18 border plants turning their flags;
- F10's swap plant, turning first_xf_content through first_xf_tree alone;
- the two record pairs and the open-check pair of ruling 1;
- the keep of 220 pairs with 0 new, and the open check exiting 0.

P2 verification ruling 6 is carried into the scripts:
- F7: every CARRIED TO note names the log it was carried from (gate line 96, test 72, negative 76, run 94, size 94, scan 95).
- F9: change M at log-gate.log lines 857-863.
- F10: run change E.

## Question 3: are the report's numbers and quoted strings in the primaries?

Every commit id, count and figure in report-draft.md appears in a primary file here except the items below:
- 634/0/2 and 1 added: test.log lines 1431, 2113 and 2148.
- Nine PASS lines: gate.log lines 1502-1510.
- The size figures: size.log lines 2902-2944.
- 27 of 27, 22 and 18 plants, 220 pairs and 0 new: run.log.
- The rulings, which are cited in the spec.

The items that do not hold:

- F16 (defect in the report). The report says attempt 1's "run directories, records and keep were moved, never removed, and are hashed in attempt 2's header". This has three problems:
  - The keep is not hashed anywhere (see F9).
  - The run directories are represented by two summary sha256s and a file count each, not hashed as whole directories.
  - log-run.log line 28 says the installed record copies were removed, after a byte comparison with the moved ones.

- F17 (defect in the report). The report says negative.log covers "every refusal of the part the binary reaches, once". log-negative.log line 719 itself says the unbound-prefix refusal is reached by any operation that reads the part, and no case runs it (see F7).

- F18 (remark). The report says gate "Attempt 1 failed three checks". This matches log-gate.log line 49, but superseded-gate-attempt-1.log has four failing check lines:
  - line 1151, "the commit that modified the package tests resolved from git";
  - line 1152, the commit_class control on the same commit;
  - line 1317, the manifest check, whose verdict line holds the printed manifest lines;
  - line 1531, the stderr check.

  The header and report count the two lines from one cause as one check. The number is defensible, but it is not what the attempt-1 log shows.

- F19 (gap in evidence). The report says "its five hash lines identical to attempt 1's, so the run log's binary checks hold against it". No log prints this comparison. It is true: I checked it by diff between the two size logs. See F10.

- F20 (gap in evidence). The report's "scan.log: final stage after the check answer" describes a run that does not exist among the files here. Only the pre-check stage is present.

- F21 (remark). "P3" asks the report to state "the headroom left for part 3". The draft gives "59576 under the Milestone 2 ceiling of 458752" but does not name that figure as part 3's headroom. It also does not mention the two spec hashes (F11) or the `-b` run-directory suffix (F12).

- F22 (remark). "(every probe that adds a style now links the full add_style)" is P3 ruling 2's reasoning, found in specs-0017-fuller-styles.md lines 965-968. No log measures it.

The report's third decision says no claim is made that a planted change to the crate is caught by a test. I found no such claim in the four commit messages, the diffs, or the draft.
