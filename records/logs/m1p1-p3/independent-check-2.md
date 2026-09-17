Milestone 1 part 1 P3 (specs/0008-zip-layer.md) independent check 2, after the repair round
checked commit: 6cec4161000c0c20bf55a797bf1fb972f0794960
HEAD when the check ran: 6cec4161000c0c20bf55a797bf1fb972f0794960

Inputs, each as given to the checker, with the sha256 it had then. The
size.log and negative.log named here were rerun after this check, on its
findings (named compound checks; two stronger size.log controls); each
rerun's header names the one given here. The scan.log named here is the pass
run for the checker; the final pass names it in its rerun header.
  specs/0008-zip-layer.md                      02ad0e09f9f7a152f12da73df7a4a67063c856882e997f37dce4844eb2d56470
  specs/0001-bootstrap.md                      6042f196b19c13f812b50052a9cb861f1cf3e0e0e010d756913c06f2d66f5499
  phase diff 9b7ef68..6cec416                  0cd60bb22134a7326734ae300e4fdfa408978ba87c04e0b78a5b550412d833f0
  the draft report                             b062ae5f00d52ee80c31ed651b62964df339018b9fb335c70f9a1be377036926
  records/logs/m1p1-p3/gate.log                1e26fc170b5586a3658f35071676971dbd911af21276a07b2dae797826a4acae
  records/logs/m1p1-p3/test.log                11415741992f49d728595cf1013afbe37af95871101c691038bb838de18cabdf
  records/logs/m1p1-p3/size.log                2e26642bfda1b8a8acb3a3d3bd728ca075e55a7d6fc33bbfae299fee85c0c0e0
  records/logs/m1p1-p3/negative.log            57961f663065f7d2e7ca8e8a7f8ebbbb9aff74099cf5d5ed3b14d29d249c933b
  records/logs/m1p1-p3/scan.log                2adb7f2c9b577d9d09dfdea4c773186b68c021d63caa6f8631b42f0701d6b5dc
  records/logs/m1p1-p3/independent-check-1.md  02d31ba3082ec1bec04fd6457a2fb82f4f56372495fe7c50f610efe1a44c832e
  records/size-m1p1-2026-09-17-6cec416.md      6012c5e25b957a58a2794ff0f7f9c6d7a20a67087ce22c7b52486b35bd14d785
  records/size-m1p1-2026-09-17-5a8b491.md      bdaabc4c6443fa3b3048162a5e5d9651d098d99c533925455864435e5a532bbb

The answer below this line is verbatim. Non-ASCII characters replaced: none;
the checker's file was already plain ASCII, which this session verified
before writing it here.
---
Independent check 2, Milestone 1 part 1 P3 (after the repair round)

Inputs read (sha256 as I computed them):
- spec-0008-zip-layer.md                 02ad0e09f9f7a152f12da73df7a4a67063c856882e997f37dce4844eb2d56470
- spec-0001-bootstrap.md                 6042f196b19c13f812b50052a9cb861f1cf3e0e0e010d756913c06f2d66f5499
- phase-9b7ef68..6cec416.diff            0cd60bb22134a7326734ae300e4fdfa408978ba87c04e0b78a5b550412d833f0 (371037 bytes)
- report-draft.md                        b062ae5f00d52ee80c31ed651b62964df339018b9fb335c70f9a1be377036926
- gate.log                               1e26fc170b5586a3658f35071676971dbd911af21276a07b2dae797826a4acae
- test.log                               11415741992f49d728595cf1013afbe37af95871101c691038bb838de18cabdf
- size.log                               2e26642bfda1b8a8acb3a3d3bd728ca075e55a7d6fc33bbfae299fee85c0c0e0
- negative.log                           57961f663065f7d2e7ca8e8a7f8ebbbb9aff74099cf5d5ed3b14d29d249c933b
- scan.log                               2adb7f2c9b577d9d09dfdea4c773186b68c021d63caa6f8631b42f0701d6b5dc
- independent-check-1.md                 02d31ba3082ec1bec04fd6457a2fb82f4f56372495fe7c50f610efe1a44c832e
- size-m1p1-2026-09-17-6cec416.md        6012c5e25b957a58a2794ff0f7f9c6d7a20a67087ce22c7b52486b35bd14d785
- size-m1p1-2026-09-17-5a8b491.md        bdaabc4c6443fa3b3048162a5e5d9651d098d99c533925455864435e5a532bbb

The two spec hashes equal the hashes scan.log prints for specs/0001 and
specs/0008 at 6cec416, so I checked against the committed specs. The four
sibling log hashes, check 1's hash and both record hashes equal what
scan.log prints. No cargo, git or corpus access was used. CLAUDE.md was in
my context; I did not use it as evidence.

Overall: the repair round does what items 1-7 and repair-round ruling 1
ask. All the arithmetic holds. Every log checks out as expected, and I
found no compound check in the four sibling scripts. Check 1's targeted
findings are fixed, most of them fully. I found some smaller problems:
one stale doc comment the round missed, a scan.log script line that
matches the forbidden-shape regex, a gap in what the bare-shape control
reads, an answer-count assertion that only runs if an environment
variable is set, two controls whose output is not printed or whose label
is unchecked, and four report sentences that no primary supports.

======================================================================
1. Does the diff do what the spec says, and nothing else?
======================================================================

Scope note: the diff covers the whole phase, 17 files, 6346 '+' lines
and 81 '-' lines excluding headers (my count). I cannot split it by
commit. For which files each commit touched, I relied on gate.log (at
740648e) and scan.log (at 6cec416). Every file hash is equal between
those two logs except specs/0001 (24bd2d5d... -> 6042f196...) and
specs/0008 (c9bce744... -> 02ad0e09...). So 6cec416 changed only the two
specs, and the code the size and negative logs measured at 6cec416 is
the code of 740648e.

Repair-round items (first-verification rulings 1-7):

1. Mode 0600 test: the_replacement_temporary_is_created_with_mode_0600,
   #[cfg(unix)]. Inside the Replacement it lists replacement_temporaries()
   not present before, reads each file's mode & 0o777, and asserts
   `vec![0o600]`. MET. The assertion needs exactly one file, so the test
   is not vacuous. Caveat (unverifiable here): replacement_temporaries()
   and REPLACEMENT_LOCK are not in the diff. Some tests that replace
   through the layer do not take the lock, e.g.
   an_unsigned_descriptor_with_the_signature_as_crc_gains_a_signature_over_padding.
   If replacement_temporaries() does not filter by thread (it cannot,
   within one process), a concurrent replace in another test could put a
   second 0600 file in the vector and make this test flaky. It passed on
   both targets this round (test.log 594, 801).
2. "an entry name longer than a header can record": new test
   an_added_name_longer_than_a_header_can_record_is_refused, 0x1_0000
   bytes refused. The control, 0xFFFF bytes, is written and read back by
   parse_independently. MET.
3. Stray signature: new test
   a_non_zip_with_a_stray_end_record_signature_is_refused_as_trailing_bytes
   (512 bytes of 7s, signature at 100, comment length 0). The control is
   the same bytes without the signature, which gives NotZip. The
   catalogue line now reads "in the file's last 65557 bytes an
   end-record signature was found whose record, by its comment length,
   ends before the file does, and no candidate ends exactly at the end
   ... a file that is not a zip but carries the four signature bytes
   there is refused with this message too". That matches find_eocd's
   `ends_early` logic. MET.
4. Limits preamble: PARTLY AS WORDED. The spec says the preamble "each is
   pinned by a test" is to be REPLACED by the truth about which messages
   no test reaches. The diff instead:
   - keeps the Limits preamble as "Each of these is deliberate, and each
     is pinned by a test (the mode of the temporary since P3's repair
     round)";
   - adds the unreached-message paragraph at the end of the "# Errors"
     list: "Every message in the list above is asserted word for word by
     a test, except 'a structure larger than this target's address
     space' ... 'a zip feature the zip crate refused' and 'a replacement
     the zip crate could not compress'. Those two are asserted at the
     functions that map the crate's errors."
   The content matches the spec's list of three. The retained Limits
   claim now looks true: each of the six limits has a test in the diff,
   except the "elsewhere it takes the process's default permissions"
   half of the 0600 limit, which a Unix test cannot pin. This is a
   departure in placement. I found no spec text approving it. The
   report describes it accurately.
   I checked the claim "every other message is asserted by a test that
   runs the code producing it" against the diff. Each catalogued
   Unsupported message except the three named has an assert_eq on the
   exact string in a test in the diff. "a zip64 marker with no zip64
   extra field" and "a value that would need..." are asserted by calling
   rewrite_central_record directly, not through open or write_to. That
   is running the producing code, but not from an archive. The report's
   phrase "Every other message is asserted by a test that runs the code
   producing it" is therefore true only in that narrower sense.
5. The Cargo.toml comments: fixed. They now read "for the three emend
   probes", "for probe-zip-xml and probe-emend-edit-xml", and "for
   probe-emend-copy, probe-emend-edit and probe-emend-edit-xml". MET.
   NOT FIXED, same class: tools/size-probe/src/bin/probe-emend-copy.rs's
   module doc still says "its size minus `probe-zip-xml` is the number
   the budget is measured against once `quick-xml` is linked". That is
   stale now: the budget is judged on probe-emend-edit-xml, and
   probe-emend-copy never links quick-xml. The report says only
   probe-emend-edit.rs's doc was updated. Also stale, in size.log's
   printed script (the candidates comment): "probe-emend-edit is the one
   the budget is judged on".
6. probe-emend-edit-xml: it is probe-emend-edit plus count_elements
   (Start|Empty events, streaming, over the replaced entry's bytes).
   Cargo.toml has required-features = ["emend", "xml"]. size.log builds
   it with `--features emend,xml`, and the dependency-tree check with
   both features shows quick-xml 1 and emend 1. MET. The count runs
   before the output file is created (negative.log P9 checks this).
7. Log scripts:
   - Every find-based count has a positive control: test.log count_named
     (planted 1, removed 0, missing dir error) and negative.log
     count_named (three planted names, removed 0, missing dir error).
     MET.
   - gate-file membership: count_matches with whole-line escaped
     patterns, and controls for planted (1), substring (0) and unreadable
     (error). A READ ERROR case is handled. MET.
   - test.log hashes every test executable it runs ("14 of 14" on each
     target). negative.log compares its copies with gate.log (emend-copy,
     xlsx-diff), size.log (three probes) and test.log (the test binary
     112ac906...). MET.
   - size.log header: the retracted sentence is gone. The only remaining
     linker attribution is the edit-path part ("which a no-edit consumer
     lets the linker drop"), which ruling 3 and the first-verification
     corrections keep. MET.
   - Final scan asserting two answers and the record count: the record
     count is asserted unconditionally ("size records scanned: 2 ...:
     yes"). The answer count is asserted ONLY when EXPECT_ANSWERS is set
     (`if [ -n "${EXPECT_ANSWERS:-}" ]; then structural ...`). A final
     pass run without that variable prints only "check answers scanned:
     N" and can still PASS, which is the defect check 1 2(e) named.
     PARTLY MET: planning should confirm that the final pass prints the
     line "check answers scanned: 2 (expected 2): yes". The assertion
     would be sturdier if it were unconditional, or if the log printed
     whether EXPECT_ANSWERS was set.

size.log handling (repair-round ruling 1):
- (a) all four earlier probes keep file, sections and .text (size.log
  "(a) ... : yes" x4; then/now lines equal). MET.
- (b) the rebuild at 5a8b491 in a scratch clone gives c737458b... and
  9d5c6615..., equal to the first log's sha256s. MET.
- (c) bytes.py: same length yes, same section layout yes. Copy: 134
  differing bytes, 20 in .note.gnu.build-id, 110 slots risen by exactly
  16, 0 unexplained. Edit: 141, 20, 117, 0. MET. Notes:
  - The comparison is 5a8b491 vs 6cec416, not 740648e as the ruling
    words it. That is equivalent, since the sources are identical (see
    the scope note).
  - The check proves a four-byte slot in .data.rel.ro rose by exactly 16.
    It cannot prove the slot is a panic-location line number. The label
    "a line number risen by 16" and the record's sentence "differ only in
    their build id and in panic-location line numbers" are inferences,
    though strong ones given equal layout, .text and totals.
  - The shift is computed as then line 132, now line 148, "the file from
    that line on is identical: yes", shift 16. It is derived from
    archive.rs only. probe-emend-edit.rs's doc also changed this round.
    If its line count changed, the probe's own panic locations (if any)
    would have moved by a different amount. The 0 unexplained result
    shows there were none that moved, so no issue arises. Noted because
    nothing in the log asserts it directly.
- (d) counts printed. MET.
- (e) probe-zip and probe-zip-xml sha256 equal (c63b22e6..., 52d1932f...).
  MET.
- Fifth probe: 670244 / 665948 / 510883. Judged 131072 file and 132732
  section bytes, as the ruling states. MET.
- New record: five rows and the judged figure. I diffed it against the
  copy size.log prints: IDENTICAL. The 5a8b491 record is unchanged
  (equal to the diff's added content and to hash bdaabc4c...). MET.
- Arithmetic, recomputed: 131072, 132732, 99728, 124928, 123268, 53248,
  50948, 118784, 116088, 196608, 201628, 118784, 119844, 184320, 184984,
  65140, 50672, 65536, 16644, 13280, 12288, 2600, 1936, 62540, 48736,
  5016, 184. All five sizes mod 4096 = 2596. All correct.

What the diff holds beyond the spec's items:
- Four existing tests tightened to exact messages
  (a_multi_disk_record_is_refused, reading_an_encrypted_entry_is_refused,
  replacing_an_encrypted_entry_is_refused,
  two_changes_for_one_entry_are_refused). There are also four new tests:
  a_zip64_locator_without_its_record_is_refused,
  a_locator_and_a_directory_disagreeing_on_the_base_are_refused,
  reading_an_entry_of_a_method_the_crate_cannot_decode_is_refused and
  the_crate_refusals_carry_their_catalogued_messages. The spec's item 4
  says "the rest pinned", which these serve, so I read them as within
  scope. The report says they were "approved by planning". No primary
  records that approval (see Q3).
- the_crate_refusals_carry_their_catalogued_messages also asserts that
  zip_write_error keeps Io as Io, a control. Fine.
- ROADMAP.md (planning's doc) says the round adds "four tests". The round
  added seven new layer tests (78 - 71, test.log line "71 before the
  repair round and 7 it adds"). This is a planning-doc inconsistency,
  not an implementer defect.
- The first round's changes (compress_replacement checking the method
  before the temporary exists, the recorded_crc fixture field, the
  read/edit split build) were ruled on at first verification ("Accepted
  without repair" and the corrections). No new unrequested code.

======================================================================
2. Do the logs contain every required line? Are they consistent?
======================================================================

Present and consistent:
- gate.log (740648e, 14:40:37Z-14:41:48Z): cold cache ("Removed 8390
  files"), eight PASS, exit 0, 0 stderr bytes. Three doc commits and
  seven commits asserted. The ten gate files are listed in the checks
  section with controls. "files under scripts/ in the phase diff: 0"
  with a planted control. First-record files are listed as records. The
  four binary hashes are printed. The rerun header names b28d839 and
  sha256 07c40016..., and says what changed; this matches check 1 2(d)
  and 2(g).
- test.log (740648e, 14:42:03Z-14:43:15Z): 148 passed, 0 failed,
  2 ignored on each target. Layer 78 passed and 1 ignored per target.
  Same names on both targets. 27 tests are required by name, including
  the two check 1 said were unnamed. 4 GiB case: host 20.18 s / 5056 kB,
  i686 28.34 s / 2544 kB, ext4, 0 left, harness hashes a718bb29... and
  3830dc06..., equal to gate.log. Rerun header names 80183c5a....
- size.log (6cec416, 14:52:39Z-14:54:02Z): see Q1. The baseline figures
  are read from the committed baseline record (0e0dd78e...). Candidates
  quadratic/stable/unstable for both probes, equal to the first record.
  Generated-archive runs for all five probes, including "probe-emend-copy
  exits 0 and its copy is byte-identical", now evaluated whole (this was
  the one compound check planning found unevidenced). edit-xml writes
  the same archive as edit. The rerun header names 670d0c9b... (b28d839)
  and the uncommitted a83d5455... with its time, commit and the two
  failed checks.
- negative.log (6cec416, 14:56:30Z-14:57:02Z): D1 before (exit 101,
  30.46 s, "left: 65535", "right: 65534", 58 filtered out) and after
  (exit 0, 0.76 s, 78 filtered out). Pre-repair binary c391f5b0...,
  equal to check 1's figure. X1, T1/T1c/T1n, T2/T2c, P1-P7/P7c, P8, P9,
  P9c. Temporaries 0. Main tree has 4 status lines, all logs or record.
  The rerun header names 6a361c73... (b28d839), 45128b14... and
  393a6b7f..., each with what was wrong. The timing (14:54:26 for the
  first run, then a second, then 14:56:30 for this ~32 s run) is
  plausible.
- scan.log (6cec416, 14:57:14Z-14:57:16Z): 17 files scanned at
  committed bytes, 8 messages ending in Co-Authored-By, four siblings,
  one answer, two records. Compound-shape controls work (hit 1,
  whole-evaluated/outside 0) and each sibling script has 0 hits, with
  289/308/645/462 script lines read. RESULT PASS.
- No overlapping runs visible: gate < test < size attempt (14:43:32) <
  size < negative runs < scan.
- No /home, /tmp/claude..., /mnt/, /Users, corpus name, hostname or
  foreign e-mail in any log, report or record. Remaining literal system
  paths: /usr/bin/grep and /usr/bin/time; test.log lines 1095-1096
  ("df --output=fstype /tmp", "/tmp ... tmpfs, available 11G");
  negative.log `LAYDIR=${TMPDIR:-/tmp}`. These are fixed system paths,
  not machine-specific. All files are ASCII.
- My own grep of the forbidden shape `[$][(](yn|is) .*(&&|[|][|])` over
  whole files: gate, test, size and negative give 0.

Problems:
a) scan.log's own printed script, line 266, matches the forbidden-shape
   regex: "    # $(yn ... && ...) or $(is ... || ...) reports only its
   first condition." It is a comment, so it is a false hit. But
   planning's whole-log grep, or a later phase's scan that reads this
   log as a sibling, will flag it. Ruling 2 says a false hit "still goes
   into a named function". For a comment, the fix is rewording. The
   final pass should reword it.
b) Bare-shape control coverage: the "pre-P6d" loop reads the WORKING
   TREE path for each diff file (`[ -f "$f" ] ... f_pre "$f"`), not the
   committed bytes. The four first-round logs and scan.log in the diff
   (07c40016..., 80183c5a..., 670d0c9b..., 6a361c73..., 943e2e06...)
   were therefore not run through the pre-P6d forms at their committed
   bytes. Their working-tree reruns were run twice instead (the list
   shows gate/test/size/negative twice, 24 files). The structural line
   "every file in the phase diff went through the bare-shape control" is
   true by path, not by content. The live forms DID run on committed
   bytes (git show). The first round's scan covered those logs when they
   were the working-tree copies, so the risk is low, but the claim is
   imprecise.
c) Compound checks in size.log and negative.log now use
   `"$( { [ A ] && [ B ]; } && echo yes || echo no)"`. This evaluates
   both conditions, as ruling 2 requires ("done"). But specs/0001's
   letter is "evaluated inside one function that returns yes or no", and
   these are not named functions. The scan regex cannot see this form.
   Low severity; noted as a letter-of-rule gap.
d) size.log control "compared with a shift of 17 leaves bytes
   unexplained" is asserted as "no line 'explained by neither: 0'". It
   would pass if bytes.py crashed, and its output (bytes-wrong-*.out) is
   not printed. So the log does not show this control passing for the
   right reason. The other control, "one byte changed in .text is
   reported unexplained", requires the exact line "neither: 1", which
   is sturdy. But the script never checks that offset 4200 lies in
   .text; the control is valid wherever that offset falls, but the
   ".text" label is unchecked.
e) size.log swap.py prints "the function reads as: <kind>" and does not
   assert it (check 1 2(f), not in the repair list). It is indirectly
   guarded by "the unstable candidate measures the same as the committed
   build" and by the candidate figures differing. The attribution
   patch's status is still lost in a pipe to sed. That is guarded by
   "reading an entry is the larger part".
f) find-based counts: the controls prove the counting form works. They
   do not prove the patterns ('emend-4gib-*', 'emend-replace-*.tmp')
   match the names the layer and the 4 GiB test actually create. The
   TempFile name form is not in the diff; spec item 1 confirms only the
   "emend-replace-" prefix. test.log also uses -maxdepth 1, so a fixture
   in a subdirectory would be missed. Residual from check 1 2(c).
g) scan.log has no "rerun:" line, although the committed scan.log
   (943e2e06..., b28d839) is being superseded. By the scan's own design
   (T6 P1), the final pass names what it replaces. It should name both
   943e2e06... (committed) and this pass, 2adb7f2c....
h) test.log's check label "exit is 0" has no target prefix, so the two
   lines (512, 719) read identically. Cosmetic.
i) The test.log header says "its size-probe section notes the fourth
   probe". The section comment calls edit-xml one of "the four"
   feature-gated probes, but it is the fifth size probe. The wording is
   ambiguous, not wrong.

Check 1's findings the round was meant to fix:
- 1.8 mode 0600 unpinned: FIXED (item 1).
- The untested "an entry name longer..." message: FIXED (item 2).
- 1.6 stray-signature wording, no test: FIXED (item 3).
- 1.8 Limits "each is pinned" overclaim: ADDRESSED. The claim is now
  true, but the spec's "replace the preamble" was done by adding a
  paragraph elsewhere (Q1 item 4).
- 1.3 stale Cargo.toml comments: FIXED. The same stale claim survives in
  probe-emend-copy.rs (Q1 item 5).
- Budget definition (probe-zip-xml subtracts quick-xml): FIXED (item 6,
  record text).
- 1.7 two tests not named in test.log: FIXED.
- 2(a) negative.log header quoting figures it did not produce: FIXED.
  This header quotes only this run's framing; 30.45/0.75 survive only as
  script control constants.
- 2(b) test binary provenance: FIXED (test.log hash 112ac906... =
  negative.log copy).
- 2(c) find counts without controls: FIXED for the form; residual (f).
- 2(d) gate-file grep read error as absence: FIXED.
- 2(e) scan does not assert answers or record count: record count FIXED;
  answers only with EXPECT_ANSWERS (Q1 item 7).
- 2(g) stale script text: the size.log header is FIXED; gate.log's
  four-script parse is replaced; scan.log "record pair" is gone. New
  stale text: the candidates comment in size.log and probe-emend-copy.rs.
- Record wording nit ("a few bytes can cost a page"): FIXED, now "a
  change of a few thousand section bytes can cost more than one page".

======================================================================
3. Does every number and quoted string in the draft report appear in a
   primary?
======================================================================

Verified equal:
- Base 9b7ef68e..., all eight full commit hashes and their order
  (scan.log commit list). The subjects are paraphrased, not quoted,
  except 740648e's, which matches.
- "17 files, 6346 insertions, 81 deletions", "371037 bytes", sha256
  0cd60bb2...: 17 files and the hash and size match the diff file I was
  given. 6346/81 match my count of '+'/'-' lines excluding headers. No
  log prints a stat line, so these are derived.
- Item 1 "exactly one, 0o600"; item 2 "65536 bytes", "65535 bytes";
  item 3 "NotZip" control; item 4's quoted messages and test names;
  item 5; item 6 (diff).
- "Layer tests: 78 passed, 1 ignored, on both targets" (test.log 594,
  801).
- The size table: all 15 figures (size.log, record). The budget: 131072,
  132732, 99728, 124928, 123268, 256000. The side figures 53248/50948
  and 118784/116088. The split 65140/50672/65536, 2600/1936/0,
  62540/48736/65536. 16644/13280/12288. 2596.
- c737458b..., 9d5c6615..., 134/20/110, 141/20/117, shift 16, lines 132
  and 148 (size.log).
- The log hashes and times: gate 1e26fc17... 14:40:37Z; test 11415741...
  14:42:03Z; size 2e26642b... 14:52:39Z; negative 57961f66... 14:56:30Z;
  scan 2adb7f2c.... The superseded hashes 07c40016..., 80183c5a...,
  670d0c9b..., a83d5455..., 6a361c73..., 45128b14..., 393a6b7f...
  (rerun headers).
- 148/0/2, 78, 20.18 s, 5056 kB, 28.34 s, 2544 kB, ext4 (test.log).
  30.46 s, 0.76 s, "65535 vs 65534" (negative.log).
- "eleven ... two ... 31 across six phases" (spec repair-round ruling 2).
- "The duplicate-name candidates are as the first record gave them"
  (579108/584348/584164, 644288/649488/649304 in both records).

Differ, overstate, or need a caveat:
- "Controls: a one-byte change in .text is reported unexplained": the
  log's label says .text, but nothing checks where offset 4200 is.
  "a shift of 17 leaves bytes unexplained": the check passed, but its
  output is not printed (Q2 d).
- "Every other message is asserted by a test that runs the code
  producing it" (quoted from the catalogue, which the report
  summarises): two of those messages are asserted by calling
  rewrite_central_record directly (Q1 item 4).
- "5. The two stale tools/size-probe/Cargo.toml comments;
  probe-emend-edit.rs's doc no longer says the budget is judged on it."
  True, but it omits that probe-emend-copy.rs still says the budget is
  measured against its delta over probe-zip-xml (Q1 item 5).
- "scan.log ... each sibling's printed script grepped for the compound
  shape, with controls, 0 hits": true for the siblings, but scan.log's
  own script has a line matching the shape (Q2 a).
- "The final pass will assert two answers": only if EXPECT_ANSWERS=2 is
  set (Q1 item 7).
- "the first record's files listed as records" (gate.log): true. Note
  that scan.log's pre-P6d loop reads those paths from the working tree
  (Q2 b).

Not in any primary given to me (unverifiable):
- "Beyond the listed items, approved by planning so that 'every other
  message is asserted' is true". No spec text records an approval. The
  spec's "the rest pinned" supports the tests, not the word "approved".
- "(planning's correction: that message IS reachable by an archive)".
  The spec's list of unreached messages omits "an entry the zip crate
  could not decompress", which is consistent. But no primary records a
  correction.
- "740648e is 20c3e2b amended (planning allowed it; the round had not
  been reported)". 20c3e2b appears in no primary. specs/0001 permits
  amending unreported commits, but the specific event is not recorded.
- "Planning's first doc edit dropped a heading; restored before commit."
  No primary.
- "Nothing pushed." Not verifiable without git.

Omissions the report could add:
- The Limits-preamble placement departure (Q1 item 4).
- The stale probe-emend-copy.rs doc and size.log candidates comment.
- The scan.log line-266 false hit and the working-tree bare-shape gap.
- That (c) proves "+16 in a .data.rel.ro slot", with "line number" as an
  inference.
- Possible flakiness of the mode-0600 test if a lock-free replacing test
  runs concurrently (unverifiable from the diff).
