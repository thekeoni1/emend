Milestone 1 part 1 P3 (specs/0008-zip-layer.md) independent check 1
checked commit: 5a8b491ca402e9248cd4130890e7c353ec212a82
HEAD when the check ran: 5a8b491ca402e9248cd4130890e7c353ec212a82

Inputs, each as given to the checker, with the sha256 it had then. The
scan.log named here is the pass run for the checkers; the final pass names
it in its rerun header.
  specs/0008-zip-layer.md                  1c6ec7f05c3ef59e1a26e4b8047552e0c04579de5bcd9cf550e33f9ba24d2386
  phase diff 9b7ef68..5a8b491              8f0a7e8e9542f50de4e07ff5dfcf3c7e2f29edc29dcb2f12cc5d69aae9beff0e
  the draft report                         19342feba9b8782128a137b8cf9897861ef3fd1edb42c748434661bc83d01403
  records/logs/m1p1-p3/gate.log            07c400167ccdd2bc815ccee9199ffe87cd3c3b2b98f3f05de6e83209ad2ddb61
  records/logs/m1p1-p3/test.log            80183c5ad863db29ec994f1909dbd160e3af5a09fbb992a3c5f25935a87d5b5a
  records/logs/m1p1-p3/size.log            670d0c9b896348b55f76c76c741f0aa5b839964c9ab5725b960623c273389a32
  records/logs/m1p1-p3/negative.log        6a361c739d434b791ecda8c53f87fcf38b7dae662c273f129bcff41bcdfc8ecd
  records/logs/m1p1-p3/scan.log            6a299dbb72543e6bf98aed51e81cea4120db5c066d650e2f6042f1bb29cc4ee9
  records/size-m1p1-2026-09-17-5a8b491.md  bdaabc4c6443fa3b3048162a5e5d9651d098d99c533925455864435e5a532bbb

The answer below this line is verbatim. Non-ASCII characters replaced: none;
the checker's file was already plain ASCII, which this session verified
before writing it here.
---
Independent check 1, Milestone 1 part 1 P3

Inputs read (sha256 as I computed them):
- spec-0008-zip-layer.md          1c6ec7f05c3ef59e1a26e4b8047552e0c04579de5bcd9cf550e33f9ba24d2386
- phase-9b7ef68..5a8b491.diff     8f0a7e8e9542f50de4e07ff5dfcf3c7e2f29edc29dcb2f12cc5d69aae9beff0e
- report-draft.md                 19342feba9b8782128a137b8cf9897861ef3fd1edb42c748434661bc83d01403
- gate.log                        07c400167ccdd2bc815ccee9199ffe87cd3c3b2b98f3f05de6e83209ad2ddb61
- test.log                        80183c5ad863db29ec994f1909dbd160e3af5a09fbb992a3c5f25935a87d5b5a
- size.log                        670d0c9b896348b55f76c76c741f0aa5b839964c9ab5725b960623c273389a32
- negative.log                    6a361c739d434b791ecda8c53f87fcf38b7dae662c273f129bcff41bcdfc8ecd
- scan.log                        6a299dbb72543e6bf98aed51e81cea4120db5c066d650e2f6042f1bb29cc4ee9
- size-m1p1-2026-09-17-5a8b491.md bdaabc4c6443fa3b3048162a5e5d9651d098d99c533925455864435e5a532bbb

The spec file's hash equals the hash gate.log prints for
specs/0008-zip-layer.md at 5a8b491 (gate.log line 496), so I checked
against the committed spec. The five log hashes and the record hash
equal the hashes scan.log prints. No cargo, git or corpus access was used.

Overall: the phase does what P3 asks. I found no defect in the two
repairs. I found some requirements met only in part, some checks that
could pass for the wrong reason, stale comments, and a few report
sentences that no primary supports. Details follow.

======================================================================
1. Does the diff do what the spec says for P3, and nothing else?
======================================================================

Scope limit: the input is ONE combined diff over four commits. It cannot
show which hunks are in adb0cc8 (repairs) and which are in 5a8b491
(features). The spec requires "The two P2 repairs, in one code commit of
their own before P3's feature commit". Two things support that split but
do not prove it. gate.log lines 473-476 list the four commit subjects in
that order. negative.log line 595 says the repairs commit's tests.rs
hashes b2afc0b8..., and the pre-repair binary built with that tests.rs
held 59 tests ("58 filtered out" + 1, line 619), which equals P2's 56 + 3.
So the repairs commit carried tests (a)(b)(c) and none of the catalogue
tests. Whether archive.rs changed only in rewrite_eocd and
refuse_duplicate_names in adb0cc8 cannot be checked from these inputs.

1.1 Tests (a)(b)(c) on a 65535-entry fixture with no zip64 end records
- The fixture is many_entries(MAX_16) with MAX_16 = 0xFFFF. (a) asserts
  `!locator_precedes_end(&input)` and a count of 0xFFFF. MET.
- (a) adding_past_65535_entries_synthesises_the_zip64_end_records: one
  add. The zip64 end record is built field by field from literals
  (0x0606_4b50, 44u64, 45, 45, 0, 0, total, total, size, offset) and
  compared with output[record..locator]. The locator is built the same
  way (0x0706_4b50, 0, offset, 1). The plain end record's markers are
  asserted: 0xFFFF in both counts, size and offset plain. MET.
  Weakness: central_size and central_offset are not predicted by the
  test. They come from where the output puts things
  (`record as u64 - central_start`, `central_start - parsed.base`), and
  parsed.base comes from the independent parser reading the output.
  With no prefix, the base could have been asserted as the literal 0.
  So the offset in the expected record partly echoes the output. The
  end record's disk fields and comment length are not asserted.
- (b) removing_from_65535_entries_writes_a_plain_count: one removal.
  Asserts no locator, 65534 in both 16-bit counts, and
  parse_independently(...).entries.len() == 65534. MET.
- (c) a_65535_entry_archive_round_trips_byte_for_byte: `assert_identical`
  only. The spec says "Each output is read back by the independent
  parser"; (c) never calls parse_independently. PARTLY MET. Byte
  identity with a builder-made input makes a read-back nearly redundant,
  but the literal requirement is not met.
- All three pass on both targets (test.log lines 528-530 and 713-715).

1.2 The two repairs
- rewrite_eocd takes `input_had_zip64` (the call site passes
  `self.zip64_eocd_raw.is_some()`). A marker is kept only when
  `input_had_zip64 && value == U16_MAX_MARKER` (or U32). Otherwise the
  field is written clamped. This matches "keeps a marker only when the
  input carried zip64 end records ... otherwise writes every field
  clamped". MET.
- refuse_duplicate_names sorts indices with `sort_unstable_by` on
  (name, index), takes adjacent equal pairs, maps each pair to its
  second index, and returns the min. That is the first record in
  directory order whose name appeared earlier. I checked a run of three
  equal names: it gives the second-smallest index, as before. It is
  O(n log n). MET.
- The pre-repair failure is quoted from a run: negative.log lines
  615-618, "assertion `left == right` failed: entries on this disk /
  left: 65535 / right: 65534", matching ruling 1's text. MET.

1.3 The two probes
- probe-emend-copy opens, writes back with Edits::new() and prints
  "entries N". MET.
- probe-emend-edit takes five arguments: replace, remove, add. The
  replaced entry is replaced with its own bytes, read through
  Archive::read. The added entry's content is its own name. MET. The
  spec says "a replace of a named entry with its own bytes", which
  implies a read. Note that this puts the decompressing path (62540
  section bytes, size.log line 881) into the probe the budget is
  judged on. The report and the record say so.
- It uses `std::os::unix::ffi::OsStrExt`, so it builds on Unix only.
  That is fine for both targets.
- Defect (comments only): tools/size-probe/Cargo.toml says "emend's
  core, for probe-emend-copy only" and "Enables emend, for
  probe-emend-copy only" (diff lines 1209 and 1217). probe-emend-edit
  requires the feature too (diff line 1235). Both comments are stale
  in the committed file.

1.4 The record
- records/size-m1p1-2026-09-17-5a8b491.md matches the stem pattern. For
  each probe it gives file, sections and .text, and it gives the deltas
  over probe-zip and probe-zip-xml, both file and sections. MET.
- The budget is judged on probe-emend-edit, with the no-edit probe
  beside it and the difference stated (65140 / 50672 / 65536). MET.
- I diffed the committed record against the copy size.log prints (lines
  929-1014, 4-space indent removed): IDENTICAL.
- I recomputed all arithmetic in python and it holds: 118784, 119844,
  184320, 184984, 53248, 50948, 118784, 116088, 65140, 50672, 65536,
  2600, 1936, 62540, 48736, 137216, 139912, 256000 = 250*1024, 5016,
  8192, 184, 5056. Every file size mod 4096 = 2596.
- Wording nit: the record says "a change of a few bytes can cost a page"
  and gives as its example 5016 section bytes costing 8192 file bytes,
  which is two pages, not a few bytes.

1.5 size.log contents
- The baseline probes are re-measured: 473636 and 539172, read from
  records/size-baseline-2026-09-15-6a7533e.md (sha256 0e0dd78e...), and
  compared (lines 855-861). MET. Section figures 464320 and 533216 also
  match ruling 3.
- Candidates: quadratic, stable and unstable, each built for both probes
  (lines 888-910). The quadratic form is spliced from P2's code commit
  ac1bd12, resolved from git. Copy-probe figures 584164 and 584348 at
  the same file size (592420) match ruling 2. Unstable over quadratic in
  the copy probe is +5056 sections, +4304 .text, matching ruling 2. The
  unstable candidate is checked equal to the committed build. MET.
- No HashSet candidate. The spec says it is optional. OK.
- Every figure the record carries is printed in size.log: toolchain,
  zip/quick-xml versions, 256000, the table rows, the split and 2596.
  MET.

1.6 The catalogue in the crate docs
- archive.rs's module doc gains "# Errors" (every variant, each
  Unsupported message verbatim, 20 messages) and "# Limits". MET.
- the_catalogue_names_every_refusal scans archive.rs for
  `Error::Unsupported("...` and requires each message to appear in the
  joined `//!` text, with a floor of 15 messages. Limits of the test: it
  covers Unsupported messages only, it misses a message built from a
  const or variable, and it does not catch a doc entry whose message no
  longer exists.
- The P1 ruling 4 items that the spec sent to P3's catalogue are done:
  NoSuchEntry now carries the index asked for (zip_read_error(err,
  index)), and trailing bytes get "bytes after the
  end-of-central-directory record" instead of NotZip. The P1
  verification residue is done too: the scan-window refusal has its own
  message.
- Finding (wording heuristic): find_eocd sets `ends_early` whenever ANY
  EOCD signature candidate in the tail has
  `record_end < buffer.len()`. A non-zip file (or one with a bad
  comment length) that merely contains the 4 signature bytes in its
  last 65557 bytes, with a small comment-length value, will now be
  refused as "bytes after the end-of-central-directory record", not
  NotZip. The file is still refused, so this is safe, but the doc's
  claim "an end record exists but does not end the file" is stronger
  than what the code checks. No test covers a non-zip that contains a
  stray signature.
- Finding (window boundary): the new message is chosen when
  `locator_pos > ZIP64_EOCD_FIXED + MAX_ZIP64_SCAN`. So for any file
  whose locator sits more than about 65591 bytes in, a genuinely absent
  record is also reported as "not found within 65535 bytes ...". That
  is accurate wording. The doc's "when the whole space before it was
  searched" explains it. The spec text says "the 64 KiB zip64 scan
  window"; code and doc say 65535 bytes of extensible data. The value
  of MAX_ZIP64_SCAN is not in the diff, so I cannot confirm the exact
  boundary. The tests pin 0xFFFF as accepted and 0x10000 as refused.

1.7 A test for each branch the spec lists as untested
- plan's method refusal:
  a_replacement_for_an_entry_of_another_method_is_refused_when_planned
  (method 12, exact message, out empty). MET.
- compress_replacement's method refusal:
  compressing_with_another_method_is_refused_before_a_file_exists. MET.
- "something other than one entry", moved into
  sole_entry_fields(&Archive) as ruling 4 approved:
  a_temporary_archive_of_two_entries_is_refused, with a one-entry
  control. MET.
- put_value's two messages: a_value_past_32_bits_in_a_plain_field_is_refused
  (with a u32::MAX-1 control) and
  a_marker_with_no_zip64_extra_field_is_refused_on_rewrite. Both call
  rewrite_central_record, not put_value. That rewrite_central_record
  reaches put_value is not visible in the diff (only the hunk header
  "fn put_value(" shows it nearby). PROBABLY MET, not verifiable here.
- rewrite_local_header's "too large":
  a_replacement_too_large_for_a_plain_local_header_is_refused. It uses
  a synthetic Made with compressed_size 2^32. MET.
- A panicking Replacement leaves no layer temporary:
  a_panicking_replacement_leaves_no_temporary_and_a_refusing_archive.
  It compares replacement_temporaries() before and after, and also pins
  that read and write_to return "the archive's reader is checked out".
  MET.
- All pass on both targets. test.log checks 14 of the new names on the
  host by name (lines 897-910). It does not check the other two new
  tests by name: the_crate_path_reports_the_index_asked_for and
  bytes_after_the_end_record_are_refused_with_their_own_message. They
  do appear in the printed list.

1.8 The documented limits
- MAX_ZIP64_SCAN, the zip64 field the input does not carry, and the
  unsigned-descriptor ambiguity are all in "# Limits", each with tests.
  The descriptor limit has two tests: Truncated when nothing follows,
  and a signed descriptor plus 3 bytes of padding when 7 follow. MET.
- Extra limits beyond the spec's three: a count of exactly 65535, the
  checked-out reader after a panic, and mode 0600 on Unix only. The
  header says "Each of these is deliberate and each is pinned by a
  test". No test in this diff pins the Unix-only 0600 mode, and I cannot
  tell whether an older test does. Possible overclaim.

1.9 gate.log names the ten gate files in its checks section
- Lines 503-515 list one per line with "not in the phase diff", then
  "gate files checked: 10; of them in the phase diff: 0". MET.

1.10 The budget re-rule request
- The report says: "Request to the maintainer: please re-rule the size
  budget with these figures in hand". The record also says "The
  maintainer re-rules the budget with these figures in hand." MET.

1.11 What the diff holds that P3 does not ask for
- specs/0001-bootstrap.md (+6 lines, the scan-hash rule), ROADMAP.md
  (P2 DONE, P3 row) and the specs/0008 P2-verification text. These are
  planning doc commits, not implementer work. P2 verification ruling 11
  covers the 0001 rule.
- compress_replacement now checks the method BEFORE creating the
  temporary. The spec did not ask for this behaviour change. It is small
  and a new test depends on it, but it is beyond the letter of P3.
- A test-helper field, Spec.recorded_crc, and Spec::crc(), used by
  Build. These are needed for the descriptor-limit tests.
- Two tests and one fixture helper for the documented limits and
  reworded paths. They are in scope through the catalogue.
- size.log's third "reads nothing" scratch build (the read/edit split).
  The script itself says it is "beyond what the P3 rulings ask for".
  It is a measurement, not committed code.
- Nothing else in the diff looks unrequested.

======================================================================
2. Do the logs contain every line the spec requires? Are they
   internally consistent?
======================================================================

Required content, present:
- gate.log: eight PASS, exit 0, 0 stderr bytes, a cold cache
  ("Removed 6721 files"), run from / (lines 637-652). The ten gate files
  (503-515). The phase base resolved from git, with the two-doc-commit
  ambiguity asserted (464-469). Four commits (473-478). Nine files with
  hashes (482-500). Four binary hashes (655-658).
- test.log: the workspace on both targets, 141 passed, 0 failed, 2
  ignored each (633, 818). Layer 71 passed and 1 ignored on each (531,
  716). The same names on both targets (965). The 4 GiB case rerun on
  both targets, as the spec's "32-bit and streaming" requires after a
  change to the size/offset code: 20.40 s / 5032 kB and 27.49 s / 2532
  kB, ext4, 0 files left, harness hashes equal to gate.log's
  (a718bb29..., 3830dc06...). size-probe builds and clippy passes for
  xml, emend and xml,emend on both targets.
- size.log: see 1.5. All checks yes.
- negative.log: D1 before and after (P3 ruling 1's quoted assertion);
  X1; T1/T1c/T1n, T2/T2c; P1-P7 and P7c with the harness
  (recompressed/identical/dropped/added, no flags); temporaries; main
  tree untouched.
- scan.log: 9 diff files, 4 messages (Co-Authored-By last, f_session 0),
  4 sibling logs plus the size record, all hashes matching the files I
  was given, the bare-shape control over 14 files, RESULT PASS, "check
  answers scanned: 0" (the checkers' pass, as expected).
- Serialization by timestamps: gate 14:01:47-14:02:57, test
  14:03:11-14:04:20, size run 1 14:04:32, size rerun 14:07:30-14:08:34,
  negative run 1 14:08:56, negative rerun 14:10:57-14:11:29, scan
  14:13:11-14:13:12. No overlaps visible.

Rerun headers:
- size.log line 4 names the superseded log by sha256 (a3465772...) and
  its time, and says what changed (the page check replaced by a
  common-remainder check; the LTO attribution replaced by a measured
  split). OK.
- negative.log line 4 names the superseded log (4a4918c8...) and says
  what changed. OK. But see the inconsistency below.

Inconsistencies and checks that could pass for the wrong reason:
a) negative.log rerun header: "The real figures happened to satisfy the
   check, 30.45 s against 0.75 s; this run is what establishes it."
   This run measured 30.53 s against 0.73 s (lines 613, 625, 630).
   30.45 and 0.75 are the control constants in the script (lines
   164-170), and presumably the first run's figures, but the superseded
   log is not a primary here. The header quotes figures this run did
   not produce, right next to a claim that this run establishes them.
   Also unverifiable: "Measured before this run on a failing and a
   passing command."
b) negative.log copies "the gate's test binary" (sha256 832230ba...,
   line 590), found by glob in target/debug/deps. gate.log does not
   print that hash (it lists four binaries only), and test.log does not
   hash its test executable. So the "gate's" provenance of D1's
   "after" binary and of X1's binary is asserted, not shown. The script
   comment "gate.log and size.log print the same hashes" is true for
   the four copied binaries but not for this one.
c) Temporary and fixture checks are pattern-based with no positive
   control: test.log `find "$BIG" -name 'emend-4gib-*' | wc -l`;
   negative.log X1 `find x1tmp -name 'emend-4gib-*'`; negative.log
   `find "${TMPDIR:-/tmp}" -name 'emend-replace-*.tmp' -newer ...`.
   These patterns are not shown to match the names the layer and the
   test actually use (the TempFile name form is not in the diff), and
   find errors go to /dev/null before wc -l. A wrong pattern or a
   failing find prints 0 and passes. X1 could count every file in its
   dedicated x1tmp directory instead.
d) gate.log's gate-file check uses `grep -Fxq` and treats any non-zero
   exit, including exit 2 (a read error), as "not in the phase diff".
   The difffiles write is not status-checked either. The "files in the
   diff: 9" line gives a visible cross-check, but nothing asserts it.
   This is the error-reads-as-absence class that scripts/counting.sh
   exists to prevent.
e) scan.log does not assert the answer count or the record count.
   "check answers scanned: $nans" is printed only, and
   `records/size-m1p1-*.md` is skipped silently if absent. A final pass
   with 0 or 1 answers would still print RESULT PASS. The final pass
   should assert 2.
f) size.log: swap.py prints "the function reads as: <kind>" but does
   not assert it. Only the unstable candidate has a control (equal to
   the committed build). The attribution patch's exit status is lost in
   the pipe to sed. That one is indirectly guarded, because an unpatched
   build would give a 0 read part and fail the "reading is the larger
   part" check.
g) Stale text in the committed log scripts:
   - size.log script header (lines 23-24) still says the edit-over-copy
     difference is "what link-time optimisation removes from a consumer
     that never edits", the very sentence the rerun header calls
     unmeasured and false.
   - gate.log's section "the scripts this phase touches parse" checks
     check.sh, counting.sh, passthrough-run.sh and
     passthrough-record.py, none of which this phase touches.
   - scan.log's header says "the record pair"; P3 has a single .md
     record.
h) Absolute paths and hostname: I found no /home, /mnt, /Users, corpus
   name, hostname or user name in any log or in the record. Literal
   system paths do appear. /usr/bin/grep and /usr/bin/time appear.
   test.log lines 972-973 print "command: df --output=fstype /tmp" and
   "/tmp, which these runs do NOT use: tmpfs, available 11G", and
   negative.log line 369 has `${TMPDIR:-/tmp}` in script text. These are
   fixed system mounts, not machine-specific, and the P2 ruling 3 text
   itself names /tmp. I report them because the brief lists /tmp.
   test.log also prints free space (639G, 11G). All logs and the record
   are ASCII with no U+2014.
i) Header form: every log's FIRST line is "command: git rev-parse HEAD",
   and the commit and HEAD are on line 2. specs/0001 says "The first
   line of every log names the commit ... and the HEAD". This is a
   literal departure, presumably the accepted convention of earlier
   phases.
j) The quadratic candidate's splice also replaces the doc comment above
   the function (the span starts at the doc anchor). "differing only in
   that function" is true for code, and doc comments do not affect size.

======================================================================
3. Does every number and quoted string in the draft report appear in a
   primary?
======================================================================

Verified (present and equal):
- The four full commit hashes, their subjects, and base 9b7ef68e...
  (gate.log 468-476, scan.log).
- "9 files, 1068 insertions, 72 deletions": the diff has 9 file headers,
  1077 '+' lines including 9 '+++', and 81 '-' lines including 9 '---'.
  Derived by me, since no primary prints a stat line. Diff sha256
  8f0a7e8e... and 63855 bytes equal the file.
- The file list at 5a8b491 equals gate.log 491-499.
- The three repair test names, and the D1 quote, 101, 30.53 s, 0.73 s
  and c391f5b0... (negative.log 602-630).
- The catalogue description, the Limits list, the six quoted messages,
  and "no end-of-central-directory record: not a zip archive"
  (negative.log 681).
- The 12 new test names (test.log).
- The size table, deltas, budget 256000/137216/139912, split
  65140/50672/65536, 2600/1936/+0 and 62540/48736/65536, 2596, the
  candidate table, 5016, 8192 and 184 (size.log 833-910, record).
- The log hashes 07c40016..., 80183c5a..., 670d0c9b..., 6a361c73...,
  6a299dbb..., and the superseded hashes a3465772... and 4a4918c8...
  (rerun headers).
- 141/0/2, 71, 20.40 s, 5032 kB, 27.49 s, 2532 kB (test.log).

Differ or overstate:
- "The committed form costs 5016 section bytes (8192 file bytes, a page
  crossing)". 8192 is two pages, and in the edit probe both the stable
  and the unstable form cost 8192 (size.log 907-908). "a page crossing"
  is loose. (In the copy probe the cost is 4096, one page.)
- "the_catalogue_names_every_refusal ... fails if a refusal's message is
  missing". True for Unsupported messages only (see 1.6).
- "the ten gate files named in its checks section, none in the phase
  diff". True, but see 2(d): absence is inferred from grep -q.
- "71 zip-layer tests ran on the host, the same names on i686". test.log
  compares the whole workspace's ok-name lists, which includes the
  layer's, so this holds. Noted as derived.

Not in any primary given to me (unverifiable):
- "corrected to planning" (Corrections, first bullet). size.log's rerun
  header states the false claim and its correction in the log. The
  correction to planning is a message, not a primary. The committed spec
  still says "file sizes on i686 musl are page-aligned" (ruling 3) and
  "File sizes on this target move in 4096-byte pages" ("Size"). The
  latter is defensible for differences; the former is the false claim,
  still in the committed spec.
- "(planning: not to be chased now)" in "Not done / open". No primary
  records this ruling.
- "Nothing pushed". Not verifiable without git.
- "about 62 KB" is a rounding of 62540. Fine.
- The report does not repeat negative.log's header figures 30.45 / 0.75.
  Good, but see 2(a): the log itself carries them.

Omissions worth adding to the report:
- The record's sha256 (bdaabc4c..., in scan.log).
- Test (c) does not use the independent parser (1.1).
- The stale Cargo.toml comments (1.3) and the stale size.log script
  comment (2g).
- Ruling 3's scratch figures (+2500 sections, +2016 .text, at P2 code)
  differ from this phase's measured edit-paths part (+2600, +1936, at
  5a8b491). Both are primaries, measured at different commits. The
  report could say so, since the size.log rerun header cites "2500".
- Several catalogued messages have no test visible in the diff or in
  the test names: "a structure larger than this target's address
  space", "an entry name longer than a header can record", and the
  three zip-crate refusals. The spec did not require tests for them.
  The report could say they remain untested, if that is so.
