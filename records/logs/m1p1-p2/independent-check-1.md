Milestone 1 part 1 P2 (specs/0008-zip-layer.md) independent check 1
checked commit: ac1bd12a8004740b66665f248f99d534dbd4d273
HEAD when the check ran: ac1bd12a8004740b66665f248f99d534dbd4d273

Inputs, each as given to the checker, with the sha256 it had then. The
negative.log named here is the FIRST round's: this check is what caused it to
be re-run, and the rerun's own header names it in turn.
  specs/0008-zip-layer.md                 95a04f3b8717882da3ad9f814d406bc224c808767a7743b382f3198601916514
  phase diff ff92a11..ac1bd12             f602de2f9a771169c953489712c5f0aaafa58d74e4f43e7e5a3bc32644a9ba1d
  the draft report                        a210dc3016eaddf657f05ee465ef6df97de31c4560e1a31909e58b978921b428
  records/logs/m1p1-p2/gate.log           e9e5aa3c6a375e5b29f07c4bf25494c8f8e5d1a5994e87768199386951b14c10
  records/logs/m1p1-p2/test.log           8a9283a9185b948e15affd211f11f0fd6fc2cfc440b2242fbdfec6ef1a6441b0
  records/logs/m1p1-p2/negative.log       7b1dbf2759ba32fc173ac725eeff9ee9c2f26d9b9f83f581382402b793c50a84
  records/logs/m1p1-p2/run.log            ec91ed884f412a681608be1c9b91fa3a1751048a204a5fcbfdb090cd3f9b610f
  records/logs/m1p1-p2/scan.log           29260dc2ed573f9319c55f454b2412fb55eed22839e22d831e17c96cea1e29fe

The answer below this line is verbatim. Non-ASCII characters replaced: none;
the checker's file was already plain ASCII, which this session verified
before writing it here.
extracted answer: 28847 bytes, sha256 d5cfbe2a2defec20213ecd7c20e5eec76221badc2ed50db5c6d87a53d01615d0
---
# Independent check 1: Milestone 1 part 1 P2 (specs/0008-zip-layer.md)

The harness put the project's CLAUDE.md into my context automatically. I say
so here as the rule requires, and I have not used it as evidence: every
finding below cites a spec, the diff, a log, the committed record pair, or
git.

Inputs I read, at the sha256s I was given and re-checked myself:
specs/0008-zip-layer.md (95a04f3b...), the phase diff ff92a11..ac1bd12
(f602de2f..., 2772 lines, 9 files), the draft report, and the five logs
gate.log (e9e5aa3c...), test.log (8a9283a9...), negative.log (7b1dbf27...),
run.log (ec91ed88...), scan.log (29260dc2...). As further primaries I read
specs/0001-bootstrap.md, specs/0004-xlsx-diff.md, specs/0006-passthrough.md,
the committed record pair records/m1p1-p2-2026-09-17-ac1bd12.md and .txt, and
git. I read nothing under the corpus directory and name no workbook id below.

Line numbers written "diff N" are lines of the phase diff file as supplied.

---

## 1. DOES THE DIFF DO WHAT THE SPEC SAYS, AND NOTHING ELSE?

### 1a. The "P2: replace, remove, add" list

Met, with citations:

- "Edits in full, with the io::copy wrapper that turns a reader into a
  Replacement". `replacement_from_reader` at diff 108-116, exported at
  diff 2094 (crates/emend/src/lib.rs), exercised by
  `a_replacement_may_come_from_a_reader` at diff 1238-1257. `Edits::replace`,
  `remove` and `add` existed from P1; `plan` (diff 167-222) now consumes them
  and `write_all_parts` (diff 270-438) applies them.
- "--replace <name> in emend-copy replaces the named entry with its own
  decompressed bytes": diff 2663-2704. The entry is extracted through
  `archive.read(index)` into a second temporary, rewound, and handed to
  `replacement_from_reader`, so nothing is collected in memory.
- Descriptor form kept: `check_descriptor_kept` at diff 1462-1554 with four
  callers (diff 1556-1574) covering both signature forms and both size
  widths. It asserts the local header stays zeroed (diff 1486-1490), the
  bytes outside the CRC-32 and the two sizes are byte-identical (diff
  1514-1519), and the input's and the output's field values (diff 1530-1553).
  This is ruling 7(a) delivered in full.
- Zip64 sizes on a replaced entry: `a_replaced_zip64_entry_keeps_its_markers`,
  diff 1577-1599.
- A stored original: diff 1213-1236. A deflated original: diff 1175-1210.
  Remove: diff 1260-1280. Add: diff 1283-1304. Order preserved around a
  removed and an added entry: diff 1308-1336.
- The ignored 4 GiB case: diff 1984-2084, with `read_tail` (diff 1787-1898)
  and `write_large_archive` (diff 1902-1947).
- Corpus run replacing the content-types entry with the amended expectation:
  run.log 628-713 (host) and 716-801 (i686).

Not met, or met only in part:

1. "a replacement error leaves no output file" is NOT the test that was
   delivered. The delivered test is
   `a_replacement_error_leaves_no_temporary_behind` (diff 1392-1419), which
   asserts that the layer's temporary directory holds no new
   `emend-replace-<pid>-` file. Its own comment (diff 1393-1397) says the
   layer writes to a `Write` and not to a path, so emend-copy owns the output
   file and negative.log covers that. negative.log's "no output file was
   left" checks are E2 (a name the archive does not hold, negative.log 795),
   E5 (a non-zip input, 808) and E7 (a missing output directory, 816). None
   of those is a replacement error occurring after emend-copy has created its
   output temporary, which is the case the spec sentence describes. The
   clause is therefore covered by analogy only, and the report does not name
   this as a gap. Ruling 4 asks for the temporary assertion "also", that is,
   in addition to the output-file assertion; only the addition was delivered.

2. "every edit test that moves an entry runs on a fixture built in BOTH
   offset conventions from an explicit parameter" is met for two families and
   not for the rest. `check_edit_behind_a_stub` (diff 1638-1682, four tests at
   1684-1702) and `check_remove_and_add_behind_a_stub` (diff 1705-1751, two
   tests at 1743-1750) take `absolute` as an explicit parameter, assert the
   fixture really was built in the convention asked for (diff 1648-1656), and
   assert the output kept it (diff 1668). But `Build::absolute_offsets`
   defaults to false, so these edit tests, all of which move an entry, run in
   the relative convention only:
   `a_deflated_entry_is_replaced_and_the_others_are_untouched`,
   `a_stored_entry_is_replaced_stored`, `a_replacement_may_come_from_a_reader`,
   `a_removed_entry_leaves_the_others_intact`, `an_added_entry_is_appended`,
   `order_is_preserved_around_a_removed_and_an_added_entry`, the four
   `check_descriptor_kept` tests, `a_replaced_zip64_entry_keeps_its_markers`
   and `padding_after_a_replaced_entry_is_kept`. On a literal reading the spec
   sentence is not satisfied, and the report does not declare the narrowing.
   I judge the substance adequate (replace and remove-plus-add are each run in
   both conventions, plain and zip64), but planning should either accept the
   narrowing explicitly or ask for the parameter to reach the other tests.

3. "read back by an independent reader ... that shares no code with the
   layer". `parse_independently` (diff 1018-1139) genuinely reimplements the
   parse and deliberately locates the central directory by searching for a
   position at which exactly `total` records consume exactly `central_size`
   bytes (diff 1045-1063), which is a different method from the layer's. But
   it lives inside `mod tests` in archive.rs and uses the layer's own
   constants (EOCD_SIG, ZIP64_EOCD_SIG, CENTRAL_SIG, CENTRAL_FIXED,
   U32_MAX_MARKER, ZIP64_EXTRA_ID, LOCAL_SIG) and the layer's helper
   `usize_of` (diff 1447). "Shares no code" is therefore not strictly true.
   The logic is independent; the constants are not. A small point, recorded
   because the spec chose that wording after P1's defect.

4. "The check answers use the layout specs/0001-bootstrap.md prescribes"
   cannot be checked by me. records/logs/m1p1-p2/ holds only the five logs;
   scan.log 725 reports "check answers scanned: 0". Planning must verify the
   header, the `---` line and the verbatim body when they exist.

### 1b. Planning rulings before P2, items 1 to 7

1. tool-arg: MET. Runner hunks at diff 2145-2231 (`--tool-arg` in the option
   list 2176 and 2184, the tab and newline refusal 2192-2200, the joined
   value 2201-2207, the child's command line 2217, the run row 2225-2230).
   Record script hunks at diff 2095-2144 (`tool_args` on RUN_HEADER 2110, the
   disagreement refusal 2118-2122, the `Tool arguments:` line 2130-2141). The
   corpus run used `--tool-arg --replace --tool-arg '[Content_Types].xml'`
   (run.log 628, 716). The report names both script hunks, as the ruling asks.

2. "The doc commit is one commit of three files": NOT MET. There are three
   doc commits: 608e6a4 (ROADMAP.md, specs/0006, specs/0008), 9a1cd86
   (specs/0008 alone) and e1c3b07 (ROADMAP.md, specs/0008), each verified by
   `git show --stat`. The report declares the departure and gives its reason
   (two rulings arrived after the first doc commit). Each commit holds only
   doc files, all three precede the code commit, and each message ends with
   the Co-Authored-By line and nothing else (scan.log 642-682). The rule's
   substance holds; the letter does not. Planning to rule.

3. The 4 GiB case: MET in every clause I can check.
   - Shape: a stored entry of 4 GiB + 1 (diff 1759, 1905-1920) then a small
     deflated entry (diff 1939-1945); the small one replaced with DIFFERENT
     bytes (diff 1765, 2010); the large one copied verbatim.
   - Asserts the output's zip64 end records (diff 2018), the second entry's
     offset over 4 GiB (diff 2030-2033), the replaced entry's CRC-32 and
     sizes (diff 2034-2038), and the large entry unchanged (diff 2024-2026).
   - The harness BINARY of the same target through `std::process::Command`
     (diff 2041-2045), the path read FIRST before anything is built (diff
     1988), with a panic when unset or not executable (diff 1966-1982).
   - Expected from the harness: exit 1 (diff 2051), the large entry
     `identical`, the small one `changed` (diff 2071-2082), `flagged` 0
     (diff 2083).
   - test.log sets the variable to the gate's binary and prints its sha256:
     test.log 894 and 921, equal to gate.log 646 and 648.
   - negative.log X1 shows the unset case failing before a fixture is written
     (negative.log 749-778), citing test.log's passing run as its control.
   - Both targets under /usr/bin/time -v with peak RSS under 64 MiB
     (test.log 905-911, 932-938); TMPDIR named only as <scratch> with its
     type measured by df (test.log 883-887); the test removed its files
     (test.log 913, 940).
   One deliberate narrowing: item 3 says the fixture is built "as the
   harness's was (the crate's writer with large_file(true))". `large_file` is
   set on the large entry only and deliberately not on the small one, with a
   long comment giving the reason (diff 1921-1938) and the report's finding 2
   explaining it. The words "or byte by byte" in item 3 leave the fixture's
   construction open, and item 3's own requirement (the second entry's OFFSET
   needs the zip64 form at real size) is still asserted, so I do not read this
   as a violation. It is the right call: setting `large_file` on the small
   entry made the harness flag the replaced entry `extra`, which item 3
   forbids.

4. The layer's temporary: MET. `TempFile` under `env::temp_dir()` with the
   process id and an atomic counter (diff 654-674), opened `create_new` with
   mode 0600 under `#[cfg(unix)]` (diff 705-715), removed in `Drop` (diff
   670-674) and held by `Made` for that alone (diff 684-685). No crate was
   added (no Cargo.toml or Cargo.lock in the diff). emend-copy's own
   temporary keeps P1's exact form: `temporary_path(output, "")` yields
   `.<name>.emend-copy-<pid>.tmp` (diff 2765-2771).

5. `identical` allowed, all 27 identical would be a finding: MET. run.log
   220-221 asserts `t_ident < t_rows` and 679/767 record the result. The
   expected diff exit 1 and `files_identical` no for a recompressed entry are
   checked per workbook rather than assumed (run.log 248-258, 712, 800).

6. Record pair through --stem, logs under records/logs/m1p1-p2/: MET (run.log
   829-833). Check-answer layout: see 1a item 4.

7. The descriptor amendment and its two pieces of evidence: MET. (a) is the
   four `check_descriptor_kept` tests above. (b) is run.log 803-817: a
   structure-only python reader that uses neither the layer nor the harness
   (its source is printed at run.log 269-348), printing the general purpose
   flags, the two compressed sizes and the sixteen bytes after the data as
   hex for one workbook, with "forms shown: 1" measured rather than assumed.
   The count lives in the record's content-types family row (run.log 837-839;
   I confirmed the column header at records/m1p1-p2-2026-09-17-ac1bd12.md
   line 22).

### 1c. In the diff but asked for by no spec, or asked for and unproven

- The synthesised zip64 end records (diff 423-432, 924-955) ARE asked for:
  specs/0008 "What the writer preserves" says the end carries a zip64 record
  and locator "when the input had them or the output needs them (a count,
  size or offset at a threshold)". But this code is entirely untested.
  `grep -rn 'needs_zip64|new_zip64_eocd' crates/emend/src` matches only
  archive.rs lines 836, 837, 1737 and 1744: no test, no negative case, and no
  fixture in the phase can reach it (it needs more than 65535 entries or a
  central directory past 4 GiB). I checked its arithmetic by hand and it is
  coherent: ZIP64_EOCD_FIXED is 56 (u64) so the size field is eight bytes and
  the record is 56 bytes; ZIP64_LOCATOR_LEN is 20 and the locator writes
  4+4+8+4; `rewrite_eocd` leaves a field alone when it already holds a marker
  and otherwise writes `clamp_u16`/`clamp_u32`, which clamp to the markers, so
  the plain end record ends up with markers pointing at the synthesised
  record. That is reasoning, not a measurement. The report's "What the phase
  delivers" bullet announces the feature without saying it is unexercised.
  This is my main "in the diff, unproven" finding.
- Added entries get a zip64 extra field when the addition is large (diff
  847-853, 878-885), while specs/0008 "Touched entries" says an added entry
  has "no extra field". The code is right and the spec sentence is written for
  the ordinary case, but it is an undeclared departure; the report's
  decisions list does not mention it.
- Untested error branches, all newly added: the refusal of a replacement whose
  method is neither stored nor deflate (diff 180-184 and 720-728), and
  `put_value`'s "a value that would need a zip64 field the input does not
  carry" (diff 559-565) and "a zip64 marker with no zip64 extra field" (diff
  570-572). The tests cover the encrypted, two-changes, absent-index and
  duplicate-add refusals but not these. specs/0008's P2 test list does not
  name them, so this is a coverage observation rather than a violation. P3
  owns the error catalogue and should pick them up.
- Nothing else in the diff is unexplained. The supporting additions
  (`has_zip64_sizes` diff 97-100, `Gap`/`descriptor_len`/`descriptor_bytes`
  diff 621-652, `put_value` diff 551-586, `Made` diff 677-694,
  `rewrite_local_header` diff 775-831, `added_local_header` /
  `added_central_record` diff 842-908, `version_needed` and `sized`, and the
  `Action`/`Planned`/`Addition` types) are all machinery the spec's behaviour
  requires. The rename of `rewrite_central_offset` to `rewrite_central_record`
  and its two new message strings are inside P3's error-catalogue scope.
- The doc changes in the diff (specs/0006 2232-2349, specs/0008 2351-2595,
  ROADMAP.md 1-39) are planning's own text; I compared the specs/0008 hunks
  against the committed file at 95a04f3b... and they agree.
- Decisions 1 and 2 of the report (padding copied after a replaced entry,
  diff 341-346 and the test at 1602-1627; a zip64 marker kept even when the
  new value fits, diff 457-464 and the test at 1590-1596) are real, are in the
  diff, and are declared. I agree with both: dropping the padding would set
  the harness's `descriptor` flag, and rewriting a marked field to a plain
  value would set `attrs` under specs/0004's P6 raw-bytes rule.

## 2. DO THE LOGS CONTAIN EVERY LINE THE SPEC REQUIRES?

Present and correct:

- The five logs exist under records/logs/m1p1-p2/ (specs/0008 "Phases"), and
  no self-test.log is required because no gate file changed. gate.log 493-495
  says so and gives "gate files checked: 10; of them in the phase diff: 0".
  The ten names are printed, but only inside the embedded script (gate.log
  114-125), not in the checks section. I read specs/0001 ("lists in gate.log
  the gate files it checked against the diff") as satisfied, weakly: a reader
  has to go to the script text for the list.
- Rerun headers naming the superseded log by sha256: gate.log 4 names
  913661fc... and test.log 4 names ab270a01..., both with the state they ran
  at (4db9f6a8...) and what changed. test.log's says plainly that the first
  run FAILED and why. negative.log, run.log and scan.log carry no rerun line
  and are first runs, which is consistent: the amendment landed before they
  ran.
- SERIALIZED: gate.log 5, test.log 5, negative.log 4, run.log 4, scan.log 7.
  The clock also supports it: gate 12:43:48 to 12:44:50, test 12:46:27 to
  12:47:22, negative 12:48:01 to 12:48:03, run 12:48:41 to 12:53:24, scan
  12:54:37 to 12:54:38. No overlap.
- Positive controls for every counting form: `counting_controls` is printed
  and run in all five logs (gate.log 439-463, test.log 431-455, negative.log
  636-660, run.log 585-609, scan.log 471-495), each form exercised on a
  match, a miss and an unreadable file. gate.log adds controls for its five
  scoring forms (508-517) and scan.log for all of its forms including the
  pre-P6d bare shapes (scan.log 510-538). This is thorough.
- Commit ids read from git: gate.log 466-471 and scan.log 3-5 resolve the
  base from the doc commit's subject and assert that exactly three commits
  match, so a fourth could not pass unnoticed. Nothing is typed.
- scan.log is the last log written (12:54:37Z, after run.log ended
  12:53:24Z), hashes the four siblings at their committed bytes (scan.log
  688-723, all four hashes equal the ones I computed), scans the record pair
  (726-743), and runs 15 files through the pre-P6d bare-shape control against
  9 files in the diff (749-765). RESULT PASS with 0 hits, 0 errors, 0
  structural failures (768-772).
- No absolute machine path in any log: <corpus>, <scratch>, <repo>, <home>
  substitutions are in each script, and scan.log's f_host control fires on a
  file naming the host (525) while every scanned file counts 0.
- Logs are free of control bytes and non-ASCII bytes by their own counts
  (test.log 624, 793, 868, 875, 912, 939; scan.log per-file f_ctrl and
  f_nonascii counts, all 0).

Requirements I could NOT find:

1. specs/0001 item 2 requires the scan to be the last log written AFTER the
   check answers are saved, "so it covers them, and it lists each answer it
   scanned with its sha256". The committed scan.log does not: line 725 says
   "check answers scanned: 0". The script documents the two-pass arrangement
   (scan.log 22-28) and the report says a final pass will supersede this one.
   As committed today the requirement is unmet, and it cannot be met until an
   answer exists. I record it plainly so that planning checks the final pass
   rather than this one.

2. No log measures the figures the report's finding 2 quotes as a
   measurement. See section 3 item 1.

3. specs/0001 "How this task runs" says every negative test runs in a scratch
   copy "with the gate's built binaries copied in rather than rebuilt".
   negative.log's X1 runs `cargo test -p emend --no-run` in the MAIN working
   tree: the script cd's to the repository top level (negative.log 40) and the
   cargo command at negative.log 338 is not wrapped in the `( cd "$CLONE" ...)`
   form used for every other command. Only then is the test binary copied to
   <scratch> and run there (343-348). The build writes only under target/, and
   the log's closing check shows the main tree unchanged (830-831), so nothing
   leaked; but a rebuild in the main tree during a negative log is the thing
   the rule exists to prevent, and building it inside the clone would have
   cost nothing. A finding, modest.

4. A negative case for the layer's own "a replacement error leaves no output
   file" (see section 1a item 1) is absent from negative.log.

Nothing else in specs/0001 "Rules for every phase" is missing that I can see:
the toolchain variables are printed (test.log 11), the DEPARTURE
(CARGO_BUILD_JOBS=4) and the FOREGROUND and BACKGROUND notes are declared
(gate.log 7, test.log 6-10, run.log 5-15), compound checks are evaluated
inside named functions returning yes or no (for example `status_accounted`,
`big_ok`, `e1_ok`, `pre_clean`), and no committed log was edited.

## 3. DOES THE REPORT'S EVERY NUMBER AND QUOTED STRING APPEAR IN A PRIMARY?

Sourced, checked one by one (I list the source, not the value, where the
value is long):

- Base ff92a11..., the four commit ids and their subjects, "9 files, 2772
  lines", diff sha256 f602de2f...: git log, gate.log 470-479, scan.log 5 and
  541-545, and my own wc -l and sha256sum of the supplied diff.
- "NOT PUSHED": verified by me from git. origin/main is at d1868a7 and HEAD
  is 16 commits ahead, so none of the four commits is pushed.
- "Each holds only the files you named, by path": verified by
  `git show --stat` on all three doc commits.
- 4db9f6a8..., 913661fc..., ab270a01...: gate.log 2 and 4, test.log 4.
- The five log byte counts and sha256s: identical to my own wc -c and
  sha256sum. scan.log's "first pass is 39613 bytes, sha256 29260dc2..." is the
  file I was given.
- "eight PASS, exit 0, empty stderr": gate.log 631-642. "cold cache": gate.log
  628-629 (cargo clean removed 5573 files).
- "every changed file scored at the base, in the added lines and at HEAD,
  since all nine already existed": gate.log 518-625; scan.log 549-557 shows
  all nine as M.
- The 4 GiB table (19.61 s / 4788 kB and 27.48 s / 2324 kB, exit 0, 1
  passed): test.log 903-911 and 930-938. "ext4 with 646G", "/tmp ... tmpfs
  with 11G", "Both runs removed their files": test.log 884, 887, 913, 940.
  "EMEND_XLSX_DIFF ... one of the four gate.log lists": test.log 894 and 921
  equal gate.log 646 and 648.
- negative.log claims (R1 to R5, X1, E1 to E8 all as expected; R2's tool_args
  and its empty control; E1 identical and unflagged): negative.log 684-833,
  724, 727, 788, 833.
- scan.log claims (PASS, 0 hits, 0 errors, 0 structural failures, 15 files
  against 9): scan.log 764-772.
- Every corpus figure in "The corpus run": run.log 660-713 (host) and 748-801
  (i686). 27 replaced rows, recompressed 27, identical 0, changed 0, flags
  empty 11, exactly descriptor 16, any other 0, other entries 843 with 0 not
  identical and 0 flagged; outcome ok, diff exit 1, files_identical no,
  order_preserved yes, comment_equal yes, leftover no, lo and lo_in ok; "0
  workbooks disagree" is run.log 712 and 800. "entry rows in total 870" is the
  report's own 843 + 27 and is confirmed by run.log 830 (870 entry rows per
  label in the record).
- The content-types family row and its fourteen numbers, and the report's
  column names for them: run.log 838-839, and the header line 22 of
  records/m1p1-p2-2026-09-17-ac1bd12.md, which I read and which gives exactly
  the order the report states.
- The ruling 7(b) block (0x0808, bit 3 set, 496 to 497, the two hex strings,
  "differs in the size field alone"): run.log 806-813. The report re-spaces the
  hex into four-byte groups; the bytes are the same bytes.
- Record pair "15231 bytes sha256 fd41ebba..." and "198706 bytes sha256
  4bfb5564...": run.log 832-833 and my own check of both files.
- "Its header carries Tool arguments: --replace [Content_Types].xml, and
  neither half names a home, mount or corpus path": run.log 835 and 842-845.
- Finding 1's "381 before and 381 after": sourced, but not to a log. I found
  it in the committed record's .txt: exactly one content-types row per label
  has compressed_in equal to compressed_out, and the value is 381, status
  recompressed, flags empty. The report should cite the record, since no log
  prints that number.

Figures and sentences I could NOT source, or that claim more than their
primary shows:

1. Finding 2's measurement is not in any primary. The report prints
   "withz64 input local extra: 0100 1000 6001000000000000 5300000000000000",
   the output's "5700000000000000", and "plain ... local csize field 83 -> 87".
   Neither gate.log nor test.log nor negative.log nor run.log prints those
   bytes or those two numbers. gate.log 4 and test.log 4 describe the same
   experiment in words and do give the outcomes the report quotes beside them
   ("status recompressed, flags [extra], archive flagged 1" and the control's
   "empty flags, flagged 0"), so the conclusion is supported; the byte values
   are not. Under specs/0001 "nothing is called measured unless the log prints
   the measurement", this is exactly the case the rule names, and the report
   introduces the block with "Measured before changing anything". Either put
   the bytes in a log or stop quoting them as a measurement.

2. "all 292 descriptor entries use 4-byte sizes" (finding 2) is not measured
   anywhere. 292 is specs/0008 ruling 7's figure, carried from T3 P4. The
   ruling 7(b) script inspects ONE entry name across the corpus and reports
   "forms shown: 1" (run.log 814) for the workbooks where THAT entry uses a
   descriptor; it says nothing about the other descriptor entries. The
   report's own ruling 7(b) paragraph is careful about this; the finding 2
   sentence is not. Drop the generalisation or measure it.

3. "No corpus workbook does [carry a local zip64 extra field on an entry emend
   replaces]" is an inference, not a measurement. It follows soundly from the
   absence of any `extra` flag on the 27 replaced entries (run.log 675, 763)
   plus specs/0004's rule that `extra` compares the local extra field raw: a
   local zip64 field recording a changed compressed size would have fired it.
   But it holds only for the entries this run replaced, not for the corpus at
   large, and the sentence reads as the latter.

4. Finding 1's "of the 17 workbooks whose replaced entry uses a data
   descriptor, 16 come back flagged descriptor and ONE does not" and "the
   remaining 10 workbooks have no descriptor" are deductions, not
   measurements. What the run measured is 16 flagged and 11 unflagged
   (run.log 675, 763). The 17 comes from specs/0008 ruling 7, which asserts it
   from earlier work; nothing in this phase's logs or in the record identifies
   which workbooks use a descriptor on that entry. The deduction is valid if
   17 is right (17 - 16 = 1, 27 - 17 = 10), and the record's single 381/381
   row is consistent with it, but the paragraph ends "Measured with a reader
   that uses neither the layer nor the harness", which is true only of the one
   workbook the ruling 7(b) script printed. The 16/11 split came from the
   harness. The sentence overclaims; say which part was measured how.

5. "test.log: 55 passed and 1 ignored on both targets, same test names" is
   true of the emend crate (test.log 519, 688) and false of the workspace,
   which reports 125 passed and 2 ignored per target (test.log 621, 790, 857).
   The next clause, "the ignored one is the layer's 4 GiB case", is right
   about the layer but there are two ignored tests in the run. Say "the
   layer's 55" and "two ignored, one of them the layer's".

6. Decision 6's "The first version of that test failed for exactly this
   reason" (the shared temporary directory and pid) is in no primary.
   test.log's rerun header attributes the superseded run's failure to the
   4 GiB fixture's `extra` flag alone. The mutex race is unrecorded history; a
   reader cannot check it. It is a decision note rather than a measurement
   claim, so I do not call it an error, but it should be marked as
   unrecorded.

7. "the FINAL pass supersedes it after the check answers are saved, and names
   it by that hash" describes a log that does not exist yet. Nothing wrong
   with saying so, but nothing in the phase as committed covers the check
   answers, and the report should not be read as if the scan requirement were
   already met. See section 2 item 1.

Everything else in the report that carries a number, a hash, a hex string, a
status or a quoted message, I traced to a log, to the diff, to the committed
record pair, or to git, as listed above. I found no figure that contradicts
its primary.

## Summary of what I would put to planning

Substantive, in order of weight:

- The synthesised zip64 end records are spec-required, newly added, and
  entirely untested and unreachable from any test in the phase (section 1c).
- The report's finding 2 quotes byte-level figures that no log prints, and
  generalises to all 292 descriptor entries without measuring them (section 3
  items 1 and 2).
- "a replacement error leaves no output file" was converted into a
  temporary-file assertion, and no negative case covers the output-file half
  (sections 1a item 1 and 2 item 4).
- "every edit test that moves an entry runs in BOTH offset conventions" is met
  for replace and for remove-plus-add only; the other moving edit tests run in
  the relative convention alone, undeclared (section 1a item 2).
- The committed scan.log is the pre-answer pass and covers no check answer
  (section 2 item 1).
- negative.log's X1 builds the test binary in the main working tree rather
  than in the scratch clone (section 2 item 3).
- Ruling 2 asked for one doc commit; there are three. Declared by the report
  (section 1b item 2).
- Smaller: the added entry's zip64 extra field departs from "no extra field";
  `parse_independently` shares the layer's constants and `usize_of`; the
  method-refusal and two `put_value` error branches are untested; the report's
  test-count sentence conflates the crate with the workspace; the 17/16/1
  split is a deduction presented as a measurement.

I found nothing that contradicts the corpus result itself. The entry-level
acceptance is measured where the spec says it lives, in the entry rows and not
the summary, with the replaced entry and every other entry counted separately,
on both targets, and the file-level columns checked to follow from the entry
rows rather than assumed. That is the right shape for this phase.
