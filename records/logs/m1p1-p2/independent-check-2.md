Milestone 1 part 1 P2 (specs/0008-zip-layer.md) independent check 2
checked commit: ac1bd12a8004740b66665f248f99d534dbd4d273
HEAD when the check ran: ac1bd12a8004740b66665f248f99d534dbd4d273

Inputs, each as given to the checker, with the sha256 it had then. This is the
second and final check specs/0001-bootstrap.md allows. It was given the
CORRECTED report and the RE-RUN negative.log, and check 1's committed answer,
so that it could judge whether check 1's findings were repaired or only
described.
  specs/0008-zip-layer.md                 95a04f3b8717882da3ad9f814d406bc224c808767a7743b382f3198601916514
  phase diff ff92a11..ac1bd12             f602de2f9a771169c953489712c5f0aaafa58d74e4f43e7e5a3bc32644a9ba1d
  the corrected report                    e5216bd5300ca32232a61dcdbdf917dc0013281870d50a4631458d9a14b22634
  records/logs/m1p1-p2/gate.log           e9e5aa3c6a375e5b29f07c4bf25494c8f8e5d1a5994e87768199386951b14c10
  records/logs/m1p1-p2/test.log           8a9283a9185b948e15affd211f11f0fd6fc2cfc440b2242fbdfec6ef1a6441b0
  records/logs/m1p1-p2/negative.log       72a792d09d1ac88d297a558b9bed4f91ac836b2648e39ebc35d7c3c7f66f71be
  records/logs/m1p1-p2/run.log            ec91ed884f412a681608be1c9b91fa3a1751048a204a5fcbfdb090cd3f9b610f
  records/logs/m1p1-p2/scan.log           29260dc2ed573f9319c55f454b2412fb55eed22839e22d831e17c96cea1e29fe
  records/logs/m1p1-p2/independent-check-1.md
                                          1eddf8b1465a5ec260538a67089727257d7ca2d16c89598eedf55138ab2f0abb

The answer below this line is verbatim. Non-ASCII characters replaced: none;
the checker's file was already plain ASCII, which this session verified before
writing it here.
extracted answer: 42197 bytes, sha256 af5e74f121e3b54539039cfe11c78a6806f15c573153f63e448b65987ae9e9d9
---
# Independent check 2: Milestone 1 part 1 P2 (specs/0008-zip-layer.md)

The harness put the project's CLAUDE.md into my context automatically. I say so
here as the rule requires, and I have not used it as evidence: every finding
below cites a spec, the phase diff, a log, the committed record pair, check 1's
committed answer, or git.

I did not read anything under the corpus directory, and I name no workbook id,
no file name of any corpus file, no address and no absolute machine path. Paths
below are repository-relative; the phase diff and the draft report are the two
files the implementing session supplied outside the repository.

Inputs, at the sha256s I computed myself:

    specs/0008-zip-layer.md                 95a04f3b8717882da3ad9f814d406bc224c808767a7743b382f3198601916514
    phase diff ff92a11..ac1bd12             f602de2f9a771169c953489712c5f0aaafa58d74e4f43e7e5a3bc32644a9ba1d
    records/logs/m1p1-p2/gate.log           e9e5aa3c6a375e5b29f07c4bf25494c8f8e5d1a5994e87768199386951b14c10
    records/logs/m1p1-p2/test.log           8a9283a9185b948e15affd211f11f0fd6fc2cfc440b2242fbdfec6ef1a6441b0
    records/logs/m1p1-p2/negative.log       72a792d09d1ac88d297a558b9bed4f91ac836b2648e39ebc35d7c3c7f66f71be
    records/logs/m1p1-p2/run.log            ec91ed884f412a681608be1c9b91fa3a1751048a204a5fcbfdb090cd3f9b610f
    records/logs/m1p1-p2/scan.log           29260dc2ed573f9319c55f454b2412fb55eed22839e22d831e17c96cea1e29fe
    records/logs/m1p1-p2/independent-check-1.md
                                            1eddf8b1465a5ec260538a67089727257d7ca2d16c89598eedf55138ab2f0abb

I also read specs/0001-bootstrap.md, specs/0004-xlsx-diff.md,
specs/0006-passthrough.md, the committed record pair
records/m1p1-p2-2026-09-17-ac1bd12.md and .txt, and git.

Line numbers written "diff N" are lines of the supplied phase diff file. Line
numbers written "<log> N" are lines of that committed log.

---

## 1. WERE CHECK 1'S FINDINGS ACTUALLY REPAIRED?

Nothing the report claims as repaired turns out to be undone. I checked all
eight claims against primaries and all eight hold. Both of the two that
mattered are genuinely repaired, not merely described. Details, and the two
new problems the repair round created, follow.

### The two that mattered

**Finding 2's bytes ARE now printed in a log. REPAIRED.** negative.log case X2
(negative.log 963 to 987) generates two archives with python's zipfile, one
with `force_zip64=True` on the target entry so its LOCAL header carries a zip64
extended information extra field and one without, replaces the entry in each
with the copied emend-copy binary, and prints:

    negative.log 968  withz64: emend-copy exit 0; xlsx-diff exit 1; status [recompressed]; flags [extra]; archive flagged 1
    negative.log 970  plain-local: ... status [recompressed]; flags []; archive flagged 0
    negative.log 976  input  local extra (20 bytes): 0100100060010000000000005300000000000000
    negative.log 977  output local extra (20 bytes): 0100100060010000000000005700000000000000
    negative.log 978  local fixed compressed-size field: input 4294967295, output 4294967295
    negative.log 983  local fixed compressed-size field: input 83, output 87

Those are exactly the figures the report's finding 2 quotes. The generator and
the reader are both printed in the script (negative.log 449 to 517), both
fixtures are generated, and the reader uses neither the layer nor the harness.
specs/0001's "nothing is called measured unless the log prints the measurement"
is now satisfied for this finding. I verified the hex character for character
against the report.

**negative.log no longer builds anything, in the main working tree or
anywhere. REPAIRED.** The string "cargo" occurs in the committed negative.log
at exactly four lines: the rerun header (line 4) and three comment lines inside
the printed script (368, 372, 380). No cargo command is executed. X1 now
globs `<repo>/target/debug/deps/emend-*`, asserts exactly one executable
candidate exists (negative.log 933, "executable candidates under
target/debug/deps: 1", as-expected), copies it into the scratch directory and
prints its sha256 (negative.log 935 to 936), and runs it there with
`EMEND_XLSX_DIFF` unset. That is precisely what specs/0001 "How this task
runs" asks for: the gate's built binaries copied in rather than rebuilt.
Reading the main tree's `target/` to find the gate's binary is not building in
it; negative.log 1048 to 1051 shows the main tree's only changes are this
phase's own log directory and record pair. I consider this repaired without
reservation.

### Every check 1 finding, one by one

Check 1's findings are spread over its sections 1a, 1b, 1c, 2 and 3 and
gathered in its summary. Taking them in the summary's order plus the ones the
summary omits:

1. Synthesised zip64 end records untested (check 1 section 1c). RECORDED
   HONESTLY, not repaired. Report gap 1 states it, names the three functions,
   says "THAT LAST PATH IS UNTESTED" in the delivery bullet too (report line
   46), declines to call check 1's hand-checked arithmetic a measurement, and
   offers a fixture. I confirmed independently: `needs_zip64`,
   `new_zip64_eocd` and `new_zip64_locator` occur only at crates/emend/src/
   archive.rs 836, 837, 838, 1737, 1744 and 1760, and nowhere in
   crates/emend/src/archive/tests.rs. Untested and unreachable from any test in
   the phase. Honest.
2. Finding 2's byte-level figures unsourced (check 1 section 3 item 1).
   REPAIRED; see above. Report repair 1.
3. "all 292 descriptor entries use 4-byte sizes" (check 1 section 3 item 2).
   REPAIRED. The corrected report's finding 2 now ends "The figure 292 in
   ruling 7 is the ruling's, not this phase's measurement" (report line 124),
   and the generalisation about 4-byte sizes is gone. I searched the corrected
   report for "292": it occurs once, in that attributing sentence.
4. "No corpus workbook does" over-reached (check 1 section 3 item 3). REPAIRED.
   The corrected sentence is "None of the 27 entries this run replaced carried
   one, which follows from there being no `extra` flag on any of them. I make
   no claim about the rest of the corpus" (report lines 121 to 123). It is
   correctly scoped and correctly labelled as following from the flag, not
   measured directly. run.log 675 and 763 give "any other flag: 0", which is
   the primary.
5. The 17/16/1 split presented as measured (check 1 section 3 item 4).
   REPAIRED, with one residual overclaim I name in section 4 below. The report
   now separates the three things explicitly: "What the corpus run MEASURED",
   "What the record MEASURES", and "The rest is a DEDUCTION, not a
   measurement" (report lines 65 to 78). That is the right shape.
6. "55 passed and 1 ignored on both targets" conflated crate and workspace
   (check 1 section 3 item 5). REPAIRED and correct: "the emend crate reports
   55 passed and 1 ignored on both targets (test.log 519 and 688); the
   WORKSPACE reports 125 passed, 0 failed and 2 ignored per target (test.log
   621 and 790)". I verified test.log 519 and 688 read "test result: ok. 55
   passed; 0 failed; 1 ignored", and test.log 621 and 790 read "passed 125;
   failed 0; ignored 2". Both ignored tests are named at test.log 851 to 855
   and counted at 857.
7. Decision 6's unrecorded history (check 1 section 3 item 6). RECORDED
   HONESTLY. Report decision 6 now says "NOT RECORDED IN ANY LOG: the first
   version of that test failed this way before the first test.log run, so no
   primary shows it." That is the correct treatment.
8. negative.log X1 built in the main tree (check 1 section 2 item 3).
   REPAIRED; see above. Report repair 7.
9. E9, the method refusal with no case (check 1 section 1c, untested error
   branches). PARTLY REPAIRED and correctly qualified. negative.log 1031 to
   1041 adds E9 and its control E9c: E9 exits 2 with the single line
   "error: unsupported zip feature: an entry the zip crate could not
   decompress", no output file left (negative.log 1035), and E9c copies the
   same archive byte for byte with no edit. The report is careful that this
   exercises the CRATE's refusal during extraction, not `plan`'s own
   "replacing an entry whose method is neither stored nor deflate" (diff 180
   to 184), which remains untested; report gap 6 records that. Correct and
   honest.
10. "a replacement error leaves no output file" converted to a temporary
    assertion (check 1 section 1a item 1 and section 2 item 4). RECORDED
    HONESTLY, report gap 5, and the report's reasoning is sound: emend-copy
    extracts the entry before it creates the output temporary. I verified the
    ordering in the diff: the extraction block is diff 2686 to 2704 and
    `let temporary = temporary_path(output, "")?` is diff 2706. So every
    replacement failure emend-copy can reach does land before the output file
    exists, and E2, E5, E7 and E9 each assert no output file was left
    (negative.log 1004, 1017, 1025, 1035). I agree with the report that
    closing the remaining case would need a fault hook the P1 ruling refused,
    and that planning should rule on whether the ordering satisfies the
    sentence.
11. Both offset conventions reach only two test families (check 1 section 1a
    item 2). RECORDED HONESTLY, report gap 2, with the cause named
    (`Build::absolute_offsets` defaults to false). Verified: the parameter is
    set only at crates/emend/src/archive/tests.rs 536, 572, 1462 and 1530, so
    only the P1 stub round trips and P2's `check_edit_behind_a_stub` and
    `check_remove_and_add_behind_a_stub` run in both conventions.
12. scan.log is the pre-answer pass (check 1 section 2 item 1 and section 3
    item 7). RECORDED, but incompletely; see my new finding in section 3.
13. Three doc commits against ruling 2's one (check 1 section 1b item 2).
    RECORDED HONESTLY in the report's Commits section, with the reason and
    without claiming the letter was met.
14. The added entry's zip64 extra field departing from "no extra field" (check
    1 section 1c). RECORDED, report gap 3.
15. `parse_independently` sharing the layer's constants and `usize_of` (check 1
    section 1a item 3). RECORDED, report gap 4, which also correctly adds
    `read_tail` to the same position.
16. The ten gate file names only inside the printed script (check 1 section 2
    first bullet). RECORDED, report gap 7. Verified: gate.log 115 to 124 lists
    the ten names inside the "## script" block; the checks section gives only
    the count at gate.log 494.

So: eight repaired, eight recorded honestly, none neither. The report's own
introductory count ("six places where it claimed more than its primary showed,
and one specs/0001 violation") is loose against its own list of eight repairs:
repair 8 (E9) answers check 1's untested-error-branch observation, not a report
overclaim. That is a wording slip in one sentence, not a false claim.

---

## 2. DOES THE DIFF DO WHAT THE SPEC SAYS, AND NOTHING ELSE?

I worked this independently of check 1 and reached the same conclusion on the
substance, with two small additions of my own at the end.

The diff touches exactly nine files, which I counted from the diff's own
`diff --git` lines: ROADMAP.md, crates/emend/src/archive.rs,
crates/emend/src/archive/tests.rs, crates/emend/src/lib.rs,
scripts/passthrough-record.py, scripts/passthrough-run.sh,
specs/0006-passthrough.md, specs/0008-zip-layer.md and
tools/emend-copy/src/main.rs. The two spec hunks reproduce the committed files
(specs/0008 sha256 95a04f3b..., specs/0006 sha256 721fe7d9..., both equal to
what gate.log records at its file listing), so they are planning's own text.

### "P2: replace, remove, add"

Delivered, with citations:

- `Edits` in full with the `io::copy` wrapper: `replacement_from_reader` at
  diff 114 to 116, re-exported at diff 2094, used by
  `a_replacement_may_come_from_a_reader` at diff 1238 to 1257, and used by the
  tool at diff 2701.
- `--replace <name>` in emend-copy: diff 2666 to 2668 and 2686 to 2704, the
  entry extracted through `archive.read(index)` into its own temporary,
  rewound, handed to the wrapper. Nothing is collected in memory, which the
  module header at diff 2615 to 2618 states as the reason.
- Descriptor form kept: `check_descriptor_kept` diff 1462 to 1554, four callers
  diff 1556 to 1574 covering signature x width. It asserts the local header's
  twelve bytes stay zeroed (diff 1486 to 1490), that the bytes outside the
  CRC-32 and the two sizes are identical (diff 1514 to 1519), that the width
  did not change (diff 1519), that the output's fields hold the new values and
  the input's held the old ones in the same places (diff 1530 to 1553). This
  is ruling 7(a) delivered in full and it is good work.
- Zip64 sizes on a replaced entry: `a_replaced_zip64_entry_keeps_its_markers`,
  diff 1577 to 1599, asserting the fixed fields keep the marker.
- Stored original diff 1213 to 1236; deflated original diff 1175 to 1210;
  remove diff 1260 to 1280; add diff 1283 to 1304; order preserved around a
  removed and an added entry diff 1308 to 1336.
- The ignored 4 GiB case: diff 1984 to 2084, with `write_large_archive`
  diff 1902 to 1947 and `read_tail` diff 1787 to 1898.
- Both offset conventions: `check_edit_behind_a_stub` diff 1638 to 1682 with
  four tests at 1684 to 1702, and `check_remove_and_add_behind_a_stub` diff
  1705 to 1751 with two tests. Both take `absolute` explicitly, both assert
  the fixture really was built in the convention asked for (diff 1648 to 1656)
  and that the output kept it (diff 1668, 1723), and both assert the edit
  really moved a later entry (diff 1677 to 1680), so the test cannot pass
  vacuously. That last assertion is the part that makes this worth having.
- The corpus run with the amended expectation: run.log 628 and 716 give the
  two commands with `--tool-arg --replace --tool-arg '[Content_Types].xml'`.

Not delivered as written, all three already declared by the report:

- "a replacement error leaves no output file" became
  `a_replacement_error_leaves_no_temporary_behind` (diff 1392 to 1419). Report
  gap 5. See section 1 item 10.
- "every edit test that moves an entry runs on a fixture built in BOTH offset
  conventions" is met for replace and for remove-plus-add only. Report gap 2.
- "an independent reader ... that shares no code with the layer":
  `parse_independently` (diff 1018 to 1139) shares no METHOD, and that is the
  part that matters here, since it locates the central directory by searching
  for a position at which exactly `total` records consume exactly
  `central_size` bytes (diff 1047 to 1063) rather than trusting the end
  records. But it lives in the same `mod tests`, uses the layer's constants and
  `usize_of`, so "shares no code" is not literally true. Report gap 4. I note
  the mitigation the report does not: the 4 GiB case reads its output back
  with the harness BINARY (diff 2041 to 2045), and the corpus run reads 27
  outputs back with the harness, so the layer's output is not checked only by
  code that shares its constants.

### Planning rulings before P2, items 1 to 7

1. tool-arg. MET. Runner: `--tool-arg` in the option list at diff 2176 and the
   assignment at 2184, the tab and newline refusal at diff 2192 to 2200, the
   space-joined value at diff 2201 to 2207, the child's command line at diff
   2217 with the `${TOOL_ARGS[@]+...}` guard that keeps an empty array from
   expanding to an empty argument, the run row at diff 2225 to 2230. Record
   script: `tool_args` last on RUN_HEADER at diff 2110, the disagreement
   refusal at diff 2118 to 2122, the `Tool arguments:` line at diff 2130 to
   2141. The ruling's wording "the next word is always the value" is honoured:
   `--tool-arg` is in the same case arm as the other value-taking options, so
   `--tool-arg --replace` passes `--replace`. The report names both script
   hunks as the ruling asks (report lines 51 to 54).
2. One doc commit of three files. NOT MET: three doc commits. I verified each
   with `git show --stat`: 608e6a4 (ROADMAP.md, specs/0006, specs/0008),
   9a1cd86 (specs/0008), e1c3b07 (ROADMAP.md, specs/0008). Every file is one of
   the three the ruling names, all three commits precede the code commit, and
   each message ends with the Co-Authored-By line and nothing else. The report
   declares the departure and its reason. The substance holds; the letter does
   not; planning rules.
3. The 4 GiB case. MET in every clause I can check. Shape: stored entry of
   4 GiB + 1 (diff 1759, 1905 to 1920) then a small deflated entry (diff 1939
   to 1945); the small one replaced with DIFFERENT bytes (diff 1765, 2010); the
   large one copied verbatim and asserted unchanged (diff 2024 to 2026). The
   output's zip64 end records (diff 2018), the second entry's offset over
   4 GiB (diff 2030 to 2033), the replaced entry's CRC-32 and sizes (diff 2034
   to 2038). The harness BINARY of the same target through
   `std::process::Command` (diff 2041 to 2045); the variable read FIRST, before
   anything is built (diff 1988, with `harness_binary` at diff 1966 to 1982
   panicking when unset or not executable, and printing no path). Expected
   verdict asserted: exit 1 (diff 2051), the large entry `identical`, the small
   one `changed` (diff 2071 to 2082), `flagged` 0 (diff 2083). test.log sets
   the variable to the gate's binary and prints its sha256 (test.log 894 and
   921, equal to gate.log 646 and 648); TMPDIR is named only as <scratch> with
   its type from `df --output=fstype` (test.log 883 to 887); peak RSS 4788 kB
   and 2324 kB, both under 64 MiB (test.log 909 to 911, 936 to 938); the test
   removed its files (test.log 913, 940); negative.log X1 covers the unset case
   (negative.log 931 to 962), citing test.log's passing run as its control.
   The one narrowing, `large_file` not set on the small entry, is explained at
   diff 1921 to 1938 and in report finding 2, is permitted by item 3's own "or
   byte by byte", and preserves item 3's actual requirement, which is about the
   second entry's OFFSET. I agree with the call; the alternative made the
   harness flag the replaced entry `extra`, which item 3 forbids, and X2 now
   measures that mechanism.
4. The layer's temporary. MET. `TempFile` under `env::temp_dir()` with the
   process id and an atomic counter (diff 659 to 674), `create_new` with mode
   0600 under `#[cfg(unix)]` (diff 705 to 715), removed in `Drop` (diff 670 to
   674), held by `Made` for that alone (diff 684 to 685). No crate was added:
   there is no Cargo.toml or Cargo.lock hunk in the diff. emend-copy's own
   temporary keeps P1's exact form, `.<name>.emend-copy-<pid>.tmp`, because
   `temporary_path(output, "")` inserts an empty `kind` (diff 2765 to 2771).
5. `identical` allowed, all 27 would be a finding. MET. run.log 220 to 221
   computes the condition and run.log 679 and 767 record "not every replaced
   entry came back identical, so the replacement path ran: as-expected". The
   diff exit and `files_identical` are checked per workbook against the
   recompressed count rather than assumed (run.log 238 to 258, 712, 800).
6. Record pair through `--stem`, logs under records/logs/m1p1-p2/, check
   answers in the specs/0001 item 3 layout. MET, and I can now confirm the part
   check 1 could not: records/logs/m1p1-p2/independent-check-1.md exists, and
   its layout is the prescribed one. Line 1 names the phase and the check, line
   2 the checked commit, line 3 the HEAD, then the input table with a sha256
   per input, then the note about verbatim reproduction, then a bare `---` at
   line 21, then the answer. The extracted body below that line is 28847 bytes
   with sha256 d5cfbe2a..., which is the figure the header itself gives. This
   closes P1's residue.
7. The descriptor amendment and its two pieces of evidence. MET. (a) is
   `check_descriptor_kept` above. (b) is run.log 803 to 817: a structure-only
   python reader printed in full at run.log 269 to 348, using neither the layer
   nor the harness, printing the central general purpose flags, the two
   compressed sizes and the sixteen bytes after the data as hex for one
   workbook, with "forms shown: 1" measured (run.log 814) rather than assumed
   and asserted equal to the number of pairs that differ in the size field
   alone (run.log 354 to 358). No cell content is printed, only a signature, a
   CRC-32 and two sizes. The count lives in the record's content-types family
   row, records/m1p1-p2-2026-09-17-ac1bd12.md lines 24 and 53 under the header
   at lines 22 and 51.

### In the diff and asked for by no spec

- The synthesised zip64 end records ARE asked for by specs/0008 "What the
  writer preserves" ("or the output needs them"), and are entirely untested.
  Section 1 item 1. This is the heaviest item in the phase and I agree with
  check 1's weighting.
- An added entry gets a zip64 extra field when large (diff 847 to 853, 878 to
  885) while "Touched entries" says an added entry has "no extra field".
  Declared as report gap 3. The code is right; the spec sentence needs a
  clause.
- NEW, not named by check 1 or the report: `added_central_record` writes
  VERSION MADE BY as `version_needed(large)`, that is 20 or 45 with a zero
  host-system high byte (diff 889), and writes internal attributes 0, external
  attributes 0 and disk start 0 (diff 901 to 903). specs/0008 "Touched
  entries" fixes version needed, flags, method, DOS time and date, extra field
  and comment for an added entry, and says nothing about version made by or the
  attributes. Nothing is wrong here and nothing is contradicted, but these are
  four header fields chosen by the implementer that no spec sentence fixes and
  no report line declares. They should be written into the spec or the report's
  decisions before P3, because "an added entry's record is exactly this" is the
  kind of thing a later phase will assume.
- NEW, minor: ruling 4 says the replacement temporary is removed "on every exit
  path, error and unwinding included". The `Drop` implementation does cover
  unwinding, but only the error path is tested
  (`a_replacement_error_leaves_no_temporary_behind`, diff 1392 to 1419); no
  test panics inside a `Replacement`. Also, the 0600 mode is inside
  `#[cfg(unix)]` (diff 710 to 714), so on any other platform the file is
  created with the process umask. Both are acceptable for this project's
  targets; neither is declared.
- Untested newly added error branches: `plan`'s method refusal (diff 180 to
  184), `compress_replacement`'s own method refusal (diff 720 to 728),
  `put_value`'s two messages (diff 560 to 565, 570 to 572), and
  `rewrite_local_header`'s "a replacement too large for a local header with no
  zip64 extra field" (diff 819 to 825), which the report's gap 6 does not list
  among its three. Four, not three. specs/0008's P2 test list names none of
  them and P3 owns the error catalogue, so this is a coverage note, not a
  violation; but the report's "Three error branches" should read four.
- Everything else in the diff is machinery the spec's behaviour requires:
  `has_zip64_sizes` (diff 97 to 100), `gap_of` (diff 230 to 266), `Gap`,
  `descriptor_len`, `descriptor_bytes` (diff 621 to 652), `put_value` (diff 551
  to 586), `TempFile` and `Made` (diff 659 to 694), `compress_replacement`
  (diff 703 to 767), `rewrite_local_header` (diff 775 to 831), the added-entry
  builders (diff 842 to 908), `recorded` (diff 442 to 446), and the
  `Action`/`Planned`/`Addition` types. The rename of `rewrite_central_offset`
  to `rewrite_central_record` and its two new messages are inside P3's scope.
- Report decisions 1 and 2 are real, are in the diff, and are declared: padding
  copied after a replaced entry (diff 341 to 346, test at diff 1602 to 1627)
  and a zip64 marker kept even when the new value fits (diff 457 to 464, test
  at diff 1577 to 1599). I agree with both and I want to put it more strongly
  than the report does: these are not preferences. specs/0004-xlsx-diff.md
  makes `descriptor` cover "the bytes between the end of an entry's compressed
  data and the next local header ... a data descriptor's bytes, and any
  padding", so dropping the padding WOULD fire the flag; and its `attrs`
  clause fires when "the raw bytes of the central record's ... compressed size
  or uncompressed size field differ while the decoded values are equal", so
  rewriting a marked field to a plain value WOULD fire that. Both decisions are
  forced by the harness's own definitions.
- Decision 4, the removal of `a_non_empty_edits_is_refused_in_p1`, is visible
  at diff 1306 to 1353: the test was rewritten in place into
  `two_changes_for_one_entry_are_refused`, which also asserts the output is
  still empty. Removing it was right; it asserted the refusal P2 exists to
  lift.

---

## 3. DO THE LOGS CONTAIN EVERY LINE THE SPEC REQUIRES?

Present and correct:

- The five logs specs/0008 "Phases" names all exist under
  records/logs/m1p1-p2/, plus independent-check-1.md. No self-test.log is
  required: gate.log 494 gives "gate files checked: 10; of them in the phase
  diff: 0" and gate.log 495 draws the conclusion. The ten names are at gate.log
  115 to 124, inside the printed script; report gap 7 concedes that the checks
  section gives only the count. I read specs/0001's "lists in gate.log the gate
  files it checked against the diff" as weakly satisfied, as check 1 did.
- Rerun headers. gate.log 4 names 913661fc... and says the first run passed but
  named a commit that no longer exists after the amend. test.log 4 names
  ab270a01..., says plainly that it FAILED, gives the cause and the fix.
  negative.log 4 names 7b1dbf27..., the log check 1 saw, and gives all three
  reasons: the main-tree cargo build, the unprinted finding-2 bytes, and the
  missing method-refusal case. Every one names the superseded log by sha256 and
  says what changed, as specs/0001 requires.
- The re-run negative.log's header specifically: yes, it names the superseded
  log by sha256 AND says what changed, in three numbered-in-prose parts, and it
  attributes each to independent check 1 rather than presenting the changes as
  the session's own idea. This is the most honest rerun header in the phase.
- SERIALIZED declared in all five (gate.log 5, test.log 5, negative.log 5,
  run.log 4, scan.log 7); the DEPARTURE (CARGO_BUILD_JOBS=4) at gate.log 7; the
  FOREGROUND note at test.log 6 to 10 and the BACKGROUND note at run.log 5 to
  15, each with its reason.
- Counting controls with a match, a miss and an unreadable file for every form
  are printed and run in negative.log (the function at negative.log 561 to 634,
  the results at 813 to 838, "counting controls: all as expected").
- Commit ids read from git, never typed: negative.log 1 to 2 prints the command;
  scan.log 3 to 5 resolves the base from the OLDEST doc commit's subject and
  asserts exactly three matches, so a fourth could not pass unnoticed.
- No absolute machine path: every script defines `hide()` with <scratch>,
  <repo> and <home> substitutions, and scan.log's per-file f_host counts are 0.
- Logs free of control and non-ASCII bytes by their own counts.

Requirements NOT met as committed:

1. **The committed scan.log does not cover the check answers.** scan.log 725
   reads "check answers scanned: 0". specs/0001 item 2 requires the scan to be
   the LAST log written, after the answers are saved, "so it covers them, and
   it lists each answer it scanned with its sha256". The script documents the
   two-pass arrangement (scan.log 22 to 28) and the report says a final pass
   supersedes this one. Unmet as committed; it cannot be met until an answer
   exists. Planning must check the FINAL pass, not this one.

2. **NEW, and neither check 1 nor the report names it: the committed scan.log
   scanned a negative.log that is no longer the committed one.** scan.log 706
   reads "scanned records/logs/m1p1-p2/negative.log (sha256
   7b1dbf2759ba32fc173ac725eeff9ee9c2f26d9b9f83f581382402b793c50a84)". That is
   the FIRST round's log. The committed negative.log is
   72a792d09d1ac88d297a558b9bed4f91ac836b2648e39ebc35d7c3c7f66f71be. So the
   file that is actually in the repository has been through NO committed
   privacy, non-ASCII, control-byte, U+2014, host-path or denylist scan, and
   the pre-P6d bare-shape control at scan.log 754 and the count of 15 files at
   scan.log 764 likewise covered the superseded bytes. This matters more than
   the missing check answers, because the re-run negative.log is the log that
   gained new printed material: two generated fixtures, a hex dump, a copied
   binary's sha256 and a longer header. The report's Logs section says only
   that scan.log is "the pre-answer pass"; it does not say that the pass
   predates the negative.log beside it. The final scan pass MUST hash
   72a792d0... and planning must verify that it does. I would put this in the
   report explicitly rather than leaving planning to notice it.

3. Following from that: scan.log is no longer even the last log written. Its
   run ended 2026-09-17T12:54:38Z (scan.log 771) and the committed negative.log
   ran at 2026-09-17T13:12:36 to 13:12:38Z (negative.log 3, 1051). The
   serialization claims are still true individually, and the order is
   recoverable from the timestamps, but the report's Logs section should state
   the real order: gate, test, negative (superseded), run, scan, negative
   (committed), then the answers, then the final scan.

4. A negative case for the layer's own "a replacement error leaves no output
   file" is still absent, as check 1 said. Report gap 5 records it.

Nothing else in specs/0001 "Rules for every phase" is missing that I can find:
compound checks are evaluated inside named functions returning yes or no, no
committed log was edited (the reruns are new files with headers), the toolchain
variables are printed (test.log 11), and every scan form has a positive
control.

---

## 4. DOES THE REPORT'S EVERY NUMBER AND QUOTED STRING APPEAR IN A PRIMARY?

I took the corrected report line by line. Almost everything is now sourced, and
several figures that check 1 could only accept on the report's word I have now
recomputed myself. The remaining problems are three, and they are smaller than
check 1's were.

### Sourced, with where

- "9 files, 2772 lines, sha256 f602de2f...": my own `wc -l`, my own sha256sum,
  and my own count of nine `diff --git` lines. scan.log 765 also gives "diff
  files: 9".
- The four commit ids and subjects: git log. Base ff92a11: scan.log 5 resolves
  it from git as the parent of the oldest doc commit.
- "NOT PUSHED": verified by me. origin/main is at d1868a7 and HEAD is 16
  commits ahead, so none of the four is pushed.
- "Each holds only the files you named, by path": `git show --stat` on all
  three doc commits, listed in section 2 above.
- "AMENDED once, after gate.log and the first test.log had run at 4db9f6a8...":
  gate.log 4 and test.log 4 both name 4db9f6a8. The commit itself no longer
  exists, so the two log headers are the only primary; that is what they are
  for, and the report does not claim more.
- The five log byte counts and sha256s in the Logs table: all five equal my own
  `wc -c` and `sha256sum`, including negative.log at 58438 bytes and
  72a792d0..., which is the re-run file.
- Check 1's file: 30381 bytes, sha256 1eddf8b1...; extracted answer 28847
  bytes, sha256 d5cfbe2a.... I computed all four myself by splitting on the
  first `---` line. Exact.
- "eight PASS, exit 0, empty stderr": gate.log 631 to 642, with the three
  checks at 640 to 642. "cold cache": gate.log 629, "Removed 5573 files".
- The superseded gate.log hash 913661fc... and test.log hash ab270a01...:
  gate.log 4, test.log 4.
- Crate 55/1 at test.log 519 and 688; workspace 125/0/2 at test.log 621 and
  790; "the two runs executed the same test names" at test.log 879; the two
  ignored tests named at test.log 851 to 855 and counted at 857. All exact.
  This is the repair of check 1's item 5 and it is now correct in every clause.
- The 4 GiB table (19.61 s / 4788 kB; 27.48 s / 2324 kB; exit 0; 1 passed):
  test.log 903 to 911 and 930 to 938. Note that the i686 test harness line says
  "finished in 27.49s" while GNU time says 0:27.48; the report uses the time
  figure, which is the right one to quote.
- "ext4 with 646G", "/tmp here is tmpfs with 11G", "Both runs removed their
  files": test.log 884, 887, 913, 940.
- "EMEND_XLSX_DIFF ... one of the four gate.log lists": test.log 894 and 921
  equal gate.log 646 and 648.
- negative.log claims: R1 at negative.log 869 to 877; R2 and its control at 879
  to 899; R3 at 901 to 906; R4's tool_args at 908 to 918, the control's empty
  at 916; R5, R5b, R5c at 919 to 930; X1 at 931 to 962; X2 at 963 to 987; E1 to
  E9c at 989 to 1041; the closing "negative log checks all as expected: yes" at
  1052. The E9 message is quoted character for character.
- scan.log claims (PASS, 0 hits, 0 errors, 0 structural failures, 15 files
  against 9): scan.log 764 to 772.
- Every corpus figure: 27 replaced rows, recompressed 27, identical 0, changed
  0 at run.log 673 to 674 and 761 to 762; "flags empty: 11; flags exactly
  descriptor: 16; any other flag: 0" at run.log 675 and 763; other entries 843
  with 0 not identical and 0 flagged at run.log 682 and 770; "0 workbooks
  disagree" at run.log 712 and 800. "entry rows in total 870" is 843 + 27 and
  is independently confirmed at run.log 830.
- The content-types family row and its fourteen numbers: run.log 838 and 839,
  and records/m1p1-p2-2026-09-17-ac1bd12.md lines 24 and 53, under the header
  at lines 22 and 51, which gives exactly the column order the report states.
- Record pair sizes and hashes: run.log 832 and 833, and my own check of both
  files. "Its header carries Tool arguments: --replace [Content_Types].xml":
  record .md line 10 and run.log 835. "neither half names a home, mount or
  corpus path": run.log 841 to 847.
- **The 381 row, which check 1 could only take on the report's word, I
  recomputed from the committed record .txt myself.** Over the entry table,
  filtering to the replaced entry's rows: 27 rows per label; exactly one row
  per label has compressed_in equal to compressed_out; its value is 381 on both
  labels; its status is `recompressed` and its flags column is empty; 16 rows
  per label carry `descriptor` and 11 carry no flag. Every clause of the
  report's sentence is true, and the report now cites the record rather than a
  log, which is the correction check 1 asked for.
- The X2 hex block: negative.log 976 to 978 and 981 to 983. Byte for byte the
  same bytes, and both "83" and "87" are the log's own numbers.
- The ruling 7(b) block: run.log 807 to 813. Flags 0x0808 with bit 3 set,
  compressed size input 496 and output 497, the two sixteen-byte strings, and
  "differs in the size field alone: True". All present.

### Still claiming more than its primary shows

1. **"The mechanism the deduction rests on IS measured" (report lines 80 to
   82).** The sentence that follows is "a descriptor records the compressed
   size, so when re-deflation reproduces that size the descriptor's bytes are
   unchanged and the flag cannot fire." No primary in this phase measures that.
   What is measured is the opposite case: run.log 807 to 813 shows a descriptor
   whose size DID change and whose entry IS flagged, and
   `check_descriptor_kept` shows that only the CRC-32 and the size fields are
   rewritten. The case where the size is reproduced and the flag therefore does
   not fire is an inference from those two plus specs/0004's definition of
   `descriptor`. It is a sound inference and I believe it, but calling it
   measured is the same move check 1 struck twice elsewhere in this report, and
   it should go the same way: say that it FOLLOWS from the descriptor-form test
   and specs/0004, not that it was measured.

2. **Both quoted evidence blocks are presented as printed lines but are not
   verbatim.** Report line 96 introduces the X2 block with "these are its
   printed lines", then regroups the hex into four-byte and eight-byte groups
   ("0100 1000 6001000000000000 5300000000000000" where negative.log 976 prints
   one unbroken string) and rewrites "local fixed compressed-size field: input
   83, output 87" as "input 83, output 87" on a reflowed line. The ruling 7(b)
   block at report lines 322 to 327 does the same: the hex is regrouped into
   four-byte groups and the log's two separate result lines (run.log 812 and
   813) are condensed into one sentence ending "differs in the size field
   alone." Nothing is falsified and the bytes are the same bytes, but a block
   set as a quotation should either be verbatim or be introduced as regrouped
   for reading. Check 1 made the same observation about the ruling 7(b) block
   and the report did not act on it; I am repeating it because the report now
   presents a second such block as a measurement, which raises the standard.

3. **"Three error branches added this phase have no test" (report gap 6) is
   four.** `plan`'s method refusal (diff 180 to 184), `put_value`'s two
   messages (diff 560 to 565, 570 to 572) and, unlisted,
   `rewrite_local_header`'s "a replacement too large for a local header with no
   zip64 extra field" (diff 819 to 825). Also unlisted and untested:
   `compress_replacement`'s own method refusal (diff 723 to 727) and "the zip
   crate wrote something other than one entry" (diff 743 to 747). A number in a
   report should be right or absent; say "several" or count them.

### Smaller notes, not errors

- The report's opening count of check 1's findings ("six places ... and one
  specs/0001 violation") does not match its own list of eight repairs. See
  section 1.
- specs/0008 ruling 7 records the pre-run measurement as 397 becoming 399,
  while run.log 808 records 496 becoming 497 for the workbook the 7(b) script
  printed. These are different workbooks and both are consistent with the
  record: the record .txt's first content-types row does read 397 to 399. The
  report quotes only the run.log figures and is right to. No action needed; I
  mention it so planning is not surprised by the two pairs of numbers.
- The report's "No replaced entry came back identical, so the replacement path
  ran; ruling 5's 'all 27 identical would be a finding' does not arise" is
  sourced to run.log 674, 679, 762 and 767 and is correct.

Everything else in the report that carries a number, a hash, a hex string, a
status or a quoted message, I traced to a log, to the diff, to the committed
record pair, to check 1's committed file, or to git. I found no figure that
contradicts its primary, and no figure I could not source at all.

---

## Summary of what I put to planning

Repairs: all eight of the report's claimed repairs are real. Finding 2's bytes
are measured in negative.log X2 at 976 to 983, and negative.log builds nothing
anywhere, in the main tree or otherwise. The eight items the report records
rather than fixes are recorded honestly, without softening. I found no claimed
repair that was not made.

New, in order of weight:

1. The committed scan.log hashed and scanned the SUPERSEDED negative.log
   (scan.log 706, 7b1dbf27...), not the committed one (72a792d0...). The
   negative.log now in the repository has been through no committed scan, and
   the bare-shape control's 15 files included the wrong bytes. The final scan
   pass must hash 72a792d0..., and the report should say so rather than
   describing the scan only as "the pre-answer pass". Section 3 item 2.
2. scan.log is no longer the last log written at all: it ended 12:54:38Z and
   the committed negative.log ran at 13:12:36Z. The report's Logs section
   should state the real order. Section 3 item 3.
3. An added entry's central record fixes version made by, internal and external
   attributes and disk start (diff 889, 901 to 903), four header fields no spec
   sentence fixes and no report line declares. Write them into the spec or the
   decisions before a later phase assumes them. Section 2.
4. "The mechanism the deduction rests on IS measured" (report line 80) is not
   measured; it follows from `check_descriptor_kept` and specs/0004. Rescope
   it, as its two neighbours were rescoped. Section 4 item 1.
5. The X2 and ruling 7(b) blocks are introduced as printed lines but are
   regrouped and condensed. Quote verbatim or say they were reformatted.
   Section 4 item 2.
6. "Three error branches ... have no test" is four, or five counting
   `compress_replacement`'s two. Section 4 item 3.
7. Carried from check 1 and still open, all declared by the report: the
   synthesised zip64 end records are untested and unreachable; the both-
   conventions parameter reaches only two test families; "a replacement error
   leaves no output file" is exercised only by the ordering; the added entry's
   zip64 extra field departs from "no extra field"; `parse_independently` and
   `read_tail` share the layer's constants; the ten gate file names are only in
   the printed script; three doc commits against ruling 2's one.

Where I disagree with a decision the report defends, plainly: nowhere. I agree
with decisions 1 to 6, and I would state decisions 1 and 2 more strongly than
the report does, because specs/0004's own definitions of `descriptor` and
`attrs` force them rather than merely favouring them. My disagreements are with
four sentences, not with any decision: the "IS measured" sentence, the two
quotation blocks, and the error-branch count.

I found nothing that contradicts the corpus result. The acceptance is measured
where specs/0008 says it lives, in the entry rows rather than the summary, with
the replaced entry and every other entry counted separately, on both targets,
with the file-level columns checked to follow from the entry rows in every
workbook rather than assumed, and with the one result that could have made the
run vacuous ("all 27 identical") tested for and absent. That is the right shape
for this phase, and the amended descriptor expectation was ruled before the run
rather than after it, which is the part that matters most.
