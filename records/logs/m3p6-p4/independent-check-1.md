# M3 part 6 P4, independent check 1

The answer of a fresh subagent (not a fork) given only <phase>/check1/inputs (the committed spec specs/0028, planning's working copy of it as it stood then, specs/0001, the phase diff, each of the phase's three commits' diffs, the commit messages, gate.log, test.log, negative.log, size.log, run.log, the scan's pre-check, the stderr files gate.stderr, test.stderr, negative.stderr, size.stderr, run.stderr and scan-pre-check.stderr, logged.out, the records the logs installed, and the draft report), recorded below. <phase> stands for <home>/.cache/emend-build/m3p6-p4. Checked commit (the phase commit): 9cfd92e922a5d802001118fc7a1b47f5de0f540b; HEAD when the check ran: 9cfd92e922a5d802001118fc7a1b47f5de0f540b. The answer is ASCII as written; it was extracted from the subagent's final message into <phase>/check1-answer.raw (not committed), sha256 f4404cd779b1d60ed680290e8fa0e80e7f31927c86d91d6f5bb817893e0ca0c6. One substitution was made, as specs/0028 "Planning rulings during P4" ruling 10 orders: the one machine path in its first line, the inputs directory, is written <phase>/check1/inputs, the substitution the logs make. Nothing else in the copy below is changed; the text as recorded hashes to 83cfd47849c701e62b5a0de560f48dc2754f0ac627c42223ddc21b302ec6278c. The report to planning answers each item.

Inputs and their sha256:

- c27784fcb9246c6b9d127a66b07579321126f57477c52de710599fadf9162213 commit-1-f54041f.diff
- 40d952d4d837807ed7ad38c021189b2fb953ba52ff0b2188ec7ec4e3e44d0052 commit-2-42b5de8.diff
- 466901b94b487ef1165a584467cf5fdaea6a695782c8867591e33cf1141b2e95 commit-3-9cfd92e.diff
- 87a3dedda71be25e79e38039b15d181af78130a31f63f01ed6e002b938e0ab02 commit-messages.txt
- 934c7a6d4d9224b697b56a8ab3acfe2f55e5fef64961e8d3bf970bdc132c3f21 gate.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 gate.stderr
- 8f96205468d225831f16122f4b0d053724a0c414ffba9c13f4bb14d07743eed5 logged.out
- 13560aded301d622d00b7256f6e75988ce9961b70bf037517f6289ef080b5c04 negative.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 negative.stderr
- 7d60cc3015217bbdcf56011667297830b0561c88c1cf218463743dd62eb2c87c phase.diff
- f9f564cbc230c19a14a170302665ac0d3553338d98de3c8cf120fb69c423819c records/m3p6-open-2026-10-08-9cfd92e.md
- 9144ce9d2e63f1490bb4471987a2c445bb422d4a680bdee0a09ac3b2e143ebe5 records/m3p6-open-2026-10-08-9cfd92e.txt
- 1e06e59a73716087d30679230a528b2fc0e933c9ceb8788d072b4735f14c0acc records/m3p6-p4-anchor-2026-10-08-9cfd92e.md
- 38832411d4114f4126de3347bc2e6a440e2f1abdb7d96f9d1373fa1383e4e6a1 records/m3p6-p4-anchor-2026-10-08-9cfd92e.txt
- 6a73d57022c232fb2eb6d120f83ae8ca771716d8ba824eb8a575bbb68549daa6 records/m3p6-p4-commentbox-2026-10-08-9cfd92e.md
- 96aaf261dc5f8b5c255e0cf785fac8f17a14969f09639bfd4cb1ff62222cd782 records/m3p6-p4-commentbox-2026-10-08-9cfd92e.txt
- 86670a673839343219bfa83a68fd16de3d21b67e155505703aa0e77e3336311e records/m3p6-p4-copy-2026-10-08-9cfd92e.md
- 8d49bc18cda5089ad344b75c2345cebf7ec8cde5b40bf722b21c1b874040c881 records/m3p6-p4-copy-2026-10-08-9cfd92e.txt
- f93d9a97e7268b5fbf55fa262ad2ed457ab50cddde9ef7ccdb326129f4919759 records/m3p6-p4-removeimage-2026-10-08-9cfd92e.md
- b4acd48978d801eca3a56260e08e7049c30b2afc11aef864396ede7bcdb4d980 records/m3p6-p4-removeimage-2026-10-08-9cfd92e.txt
- 8bc4df3126dad63724e9109340d0509b5bc628532d2c9f88ce1d22d0c0c55335 records/size-m3p6-2026-10-08-9cfd92e.md
- fdc46694ce7df4c4c907c70954787dfb581759a714e2cbc34d4107d517b6d840 report-draft.md
- a7044bf2b0e4d05005161143f14662b4a155063b4a03dc8025fd802ee64f3d78 run.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 run.stderr
- d48bf2930a1aefaf1a152c34cca7f86478dae2d897162e0cc66a1c4ab2f3e785 scan-pre-check.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 scan-pre-check.stderr
- f5e361091086b2a2e52830b6a25807d5136e3b113ef4e1476c67bfc9f05ab584 size.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 size.stderr
- 79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c spec-0001.md
- 81069c652aee1f5b5cd14b495670e1f3864e342550e474a668b793381bba80e7 spec-0028-committed.md
- 62762f0ab78cb1694344cf0b07b65df16471f2dcd6f1d0347055ed0652168211 spec-0028-working.md
- 854a6e2ab700002ace9b4d781c38f6fe256c11fa7d5312c916b1e1819507caca test.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 test.stderr

## The answer

I did not use the CLAUDE.md or memory material in my context as evidence. Everything below comes from files under <phase>/check1/inputs. I recomputed all hashes with sha256sum. Line numbers refer to the input files.

**Q1. Does the diff do what the spec (with the P4 rulings) says for P4, and nothing else?**

Verdict: yes, with reservations.

Evidence:
- **Doc commit.** commit-1-f54041f.diff touches only ROADMAP.md (the P3 row) and specs/0028. The spec change adds "Planning rulings during P3" and "Planning verification and rulings on P3".
  - spec-0028-committed.md hashes to 81069c652aee...80e7. That is the hash in f54041f's message and the pin in gate.log line 60.
  - spec-0028-working.md hashes to 62762f0ab78c...8211. Its only difference from the committed copy is the added "Planning rulings during P4" section, lines 1145-1289.
- **Code commit 1, 42b5de8 (ruling 4).**
  - xml.rs adds `const XML_NAMESPACE` with the W3C namespace. `Scope::lookup` now falls back to it for the prefix `xml` only when no binding is found, so a declaration of `xml` is still recorded as any other.
  - It adds the two tests the ruling names: `the_xml_prefix_resolves_without_a_declaration` (which also checks that an unbound `p:` is still refused) and `a_copy_keeps_an_attribute_in_the_xml_namespace`.
  - The package.rs header sentence on the unbound-prefix message gains the `xml` exception. No message constant is added; negative.log shows "constant names: at the phase commit 123; at the base 123; new 0; gone 0".
  - It is a separate commit placed before the catalogue commit (gate.log lines 65-80).
- **Code commit 2, 9cfd92e (catalogue and probes).** The package.rs header covers every catalogue item the rulings defer to P4:
  - The VML part the box read takes (P1 ruling 2), the rounding and range (P1 rulings 3 and 5), and the visibility rule (P1 ruling 4).
  - add_image_at's check order (P2 ruling 7).
  - The AlternateContent skip (P2 ruling 4) and the group limit (P2 ruling 5, P4 ruling 1). The header does not say grouped pictures are absent.
  - The hlinkClick limit (P2 verification ruling 1), the emptied drawing dropped (P2 ruling 3), and the emptied rels part kept (P2 ruling 2).
  - The copy section: the surface, with COMMENTS and HYPERLINK public and the MC constant private (P3 ruling 7, P4 ruling 2); "the relationship attributes of `pageSetup`" (P3 verification ruling 2); the root AlternateContent rule (P3 ruling 2); defined names not copied; the chain rule; the refusals.
  - The whole-part holds with corpus figures: 37 worksheet drawings, 38 by name, and 11 VML parts with the largest holding 10 shapes (decision 8, P2 ruling 10).
- **Probes (ruling 3).**
  - The images probe adds a two-cell `add_image_at` (`EditAs::OneCell`). It then runs `remove_image(sheet, 0)` in a second package written to a `Cursor`, reopens it, and prints `images_removed`.
  - The comments probe calls `set_comment_box` on AB10 (200 by 100, visible) and prints `box_width`, `box_height` and `box_visible`.
  - The new copy probe calls `copy_sheet` on the first sheet as `sheets()` gives it, under `COPY_NAME = "probe copy"`, and prints `sheets_copied`.
  - The new copycomments probe copies the comment's sheet and prints `copy_comments`.
  - The size-probe Cargo.toml gains `copy = ["emend", "xml", "emend/copy"]` and two `[[bin]]` entries with the required features, making 28 probes.
- **Nothing else.** phase.diff touches only those 11 files, and gate.log line 3043 checks each code commit's area.

Reservations:
1. Ruling 4 says "`lookup` and `bound` have no case for `xml`". The diff changes only `lookup`. The context line after it ("[`Scope::lookup`], refusing a prefix no ancestor binds") suggests `bound` wraps `lookup`, but the inputs do not show its body.
2. Two catalogue figures cannot be confirmed from the inputs:
   - "the largest holding 9 anchors (`records/logs/m3p6-p2/run.log`...)": that log is not among the inputs, and the spec gives no number.
   - "11 VML parts, the largest 10 shapes" matches the spec (spec-0028-working.md lines 369-370), not a measurement in the inputs.
3. Ruling 1 says `read_images` takes a `pic` held directly, in a `grpSp`, or in an AlternateContent's first Choice. The unchanged "What is read back" sentence shown in the diff context names only "a root anchor or the first anchor in a root `AlternateContent`'s first `Choice`". The following lines are not in the diff, so I cannot tell whether the header mentions the grpSp placement.

**Q2. Do the logs contain every line the spec requires for P4?**

Verdict: yes, with reservations.

Evidence:
- **Order (ruling 6).** The end times are gate 15:27:58Z, test 15:29:32Z, negative 15:29:37Z, size 15:34:09Z, run 17:09:24Z and scan pre-check 17:10:00Z. Each log ends "... checks all as expected: yes", and logged.out matches.
- **Gate.**
  - The prune (gate.log lines 23-56): 11 names, du -sb for each, "their bytes in all: 9624749323", each `rm -r exit 0`, all gone, the 9 kept names still there, with controls.
  - The spec is pinned to 81069c65, and the working copy 62762f0a is printed.
  - Commit roles are given, every commit message is ASCII and ends with the Co-Authored-By line, and the consumer crate exercises `copy_sheet` with the features off and on.
- **Test.**
  - The workspace runs on host and i686: "passed 1111" on each (test.log lines 3962 and 5122).
  - "this phase 1111, the previous phase 1109", with the two added names listed (line 5228 on).
  - The crate runs with the features off, copy, comments, and copy plus comments on both targets, then every feature.
  - The two new tests run alone (line 5477), and planted mutations of the xml prefix are each caught by their named test (line 5834).
- **Negative (ruling 5).**
  - The three families run through the gate's hashed host binary in a worktree. NU is refused with COMMENTS_UNREADABLE, NM with DRAWING_UNREADABLE and NC with NoWorksheet, each with a control (B0CB/NUC, B0RI/NMC, B0CP).
  - The nine messages are read by constant name and each is stated unreachable, with its covering test counted once at the phase commit.
  - The check that the phase adds and removes no message constant is present.
- **Size ("Size", "Size and packaging", ruling 7).** size.log lines 6172-7221 and the record cover:
  - 28 builds, and the 26 earlier probes classified against part 5's record with verdicts (identical 6, code 20). The two new probes are listed as new.
  - A cause per probe, the cause controls, and the manifest cause listed apart.
  - images growth 27016 (1011024 to 1038040), marked "above 25000 ... yes"; comments growth 6196.
  - copy cost 13756 (983964 - 970208) and the pair's cost 18864 (1052428 - 1033564).
  - The sum 46968 printed beside 30000 to 45000.
  - Core growth 64 (970208 - 970144), "above 2000 ... no; above 300 ... no".
  - Budget 436992, "21760 under the ceiling".
  - The record states each of these figures, including "above 25000 section bytes, which the report states".
- **Run ("Harness runs", P4).**
  - The release binaries are checked against the hashes size.log printed (run.log lines 2933-2950).
  - TMPDIR and corpus devices are printed (2096 and 2096, with /proc as the control), and the calc-chain count line reads "9 of the corpus's 27 workbooks".
  - For each of copy, anchor, removeimage and commentbox, on both host and i686: "ok workbooks 27; outputs equal to P3's by sha256 27; lines equal byte for byte 27; diff tables equal 27". Each is compared against `<corpus>/runs/2026-10-07-6a90674-<sample>`, named by record commit f73a91b.
  - The keep has 1093 pairs. "rows: the regenerated keep 1093, P3's record 1092; ... 1092", with one new pair. The open check gives "no verdict and no class of P3's pairs moved: yes", and the new pair reads `ok ok none none`.
  - Ten record files are installed, and every installed hash matches its file in records/.
- **Scan pre-check.** Every placeholder is declared, and the log ends "total hits: 0 / read or counting errors: 0 / structural checks failed: 0 / RESULT: PASS".

Reservations:
1. Ruling 8 requires the implementing session to verify that `findmnt -o PROPAGATION /` says shared before launching. No log or other input records that check. The only evidence is the 0-byte run.stderr.
2. The final scan.log does not exist yet. This is as designed (specs/0001 has the scan run last), but the "Phases" list of logs is not yet complete.
3. Ruling 4's finding (the copy refusing `xml:space` on the probe's fictional fixture) appears in no log. That is acceptable, since it came up in development rather than a logged run, but the report draft does not name it either (see Q3).

**Q3. Does every number, hash and quoted string in report-draft.md appear in a primary?**

Verdict: no. Most values match, but some cannot be found and one attribution disagrees with its primary.

These match their primaries:
- All 11 record sha256s and the 5 log hash prefixes.
- The working spec 62762f0a and the scan pre-check d48bf293.
- The five logged.out lines.
- 9624749323, the 11 prune names and the 9 kept names.
- 1111 and 1109.
- 436992, 21760, 458752, 64, 27016, 1011024, 1038040, 6196, 13756, 18864, 46968, and 30000 to 45000.
- "probe-emend-images-write-xml: verdict: code" (size.log, indented) and "6 commits" (size.log lines 6461-6511).
- The four "ok workbooks 27; ..." lines, `<corpus>/runs/2026-10-07-6a90674-*`, 1093 and 1092.
- `package.tests.copy.a_copy_keeps_an_attribute_in_the_xml_namespace.1`.
- PLANNING_SPEC, every stderr path, and the 149 bytes (ruling 8).
- The script line numbers derive from the printed scripts: size.log line 1240 is script line 1170, and run.log line 1039 is script line 959.
- The new line of the quoted run.sh diff is run.log line 1039.

**Not found in any primary:**
1. The start time "2026-10-08T15:24:07Z". The logs print only end times.
2. The logged.sh sha256 c42008026a2aa987a62f9cb9e992c5138e9551ad0db849fe41324c8e003aad5f.
3. The session name "emend-e8".
4. The script hashes "2ab287a7 -> 2f7ba42c" and "c8bcce96 -> c54017a2". No log prints its own script's hash, and FACTS.md is not among the inputs.
5. "gen/size.py line 483" and "gen/run.py line 107".
6. The old line of the quoted diff, "< #      <target>/main/<i686>/release, ...".
7. Disclosure 1's details: the "326 and 511 of 1093 rows" stops, `superseded/dry-run-attempt-1` and `-2` with NOTE.txt, "Attempt 3 ... exit 1", and "findmnt: / private" then "shared".
8. The command "wsl -u root mount --make-rshared /". Ruling 8 names `sudo mount --make-rshared /` instead. The report does explain the difference (the dev account has no sudo), but no primary records which command was run.
9. "The dry size was run again in full (exit 0, same figures)" and "the dry scan then failed exactly the dry-only LOGDIR check and the old run.log's <i686>".
10. Disclosure 3: the 149-byte overwrite of dry-run/run.stderr and its NOTE.txt.
11. The path "<phase>/scan/scan-pre-check.log". The log does not state where it was saved.

**Disagrees with or overstates its primary:**
12. The report says the core's growth of 64 bytes has "its cause code commit 42b5de8 (the xml prefix, ruling 4)". size.log line 7130 and the size record name six commits as the cause: "9cfd92e, 42b5de8, 4a8482c, b68e584, 7976d9e, 69b5e0e". They give 42b5de8 only as "this phase's" commit and do not isolate it as the source of the 64 bytes. Planning's ruling 7 makes the same attribution, but the logs do not show it.
13. Wording only: the report writes "total hits 0" (and the other two counts) without the colon in "total hits: 0", and "PRUNE_PREVIOUS=yes" where gate.log has "PRUNE_PREVIOUS: yes". The values are the same.

**Omission against a ruling:** P4 ruling 4 says "The report names the finding, the fixture (fictional, generated by the probe) and the fix's commit". The draft names 42b5de8 only in the size bullet. It never describes the finding (the copy refused a kept `xml:space` attribute) or the fixture (the pair probe's generated size fixture).