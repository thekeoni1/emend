# M3 part 4 P3, independent check 1

The answer of a fresh subagent (not a fork) given only <phase>/check1 (the committed spec specs/0026 with "Planning verification and rulings on P2", specs/0001, the phase diff, the doc commit's and each code commit's diffs, the commit messages, gate.log, test.log, negative.log, size.log, run.log, the scan's pre-check, the stderr files and the runner's summary, the phase's records with part 3's size record, and the draft report), recorded verbatim below. <phase> stands for <home>/.cache/emend-build/m3p4-p3. Checked commit (the phase commit): ef392d440ccaa8f3976c986f0e7a4d9941315cdb; HEAD when the check ran: ef392d440ccaa8f3976c986f0e7a4d9941315cdb. The answer is ASCII as written; it was extracted from the subagent's final message and hashes to 9da5a90e0228063194d5ac2ed3117aa6bb57db373b652e48e6c720c83377464e. Its items on the report are corrected in the report; its observations on the code and the logs are reported to planning. No second check was run.

Inputs and their sha256:

- d98ebdcbb92597a19a0facf6235e2b84026ab39d76f520a01a3bcb4da412e87a code-commit-1-carry.diff
- eacc014a4a7741e1fc02ef4ff2f7c6a291928a85773ef3f66eaf7113d1ab9ab1 code-commit-2-catalogue.diff
- bcf695f5f97defeca2f77ebfe587b307597c99c005cfe7e3e3b6a6c14a664360 code-commit-3-probe.diff
- db34336e6a4668ce34baabc759ce3d004f8adb0b18e60f1d73f3032e3d2db461 commit-messages.txt
- 6de38bf09a1c3b81a1197bd50ca97799ac8b02cf718d52a950acdcf59c2d80c3 doc-commit.diff
- 9e1ddcdf0b63fe44020686518456ad4e750db96369b6420892c39479aaac0aa6 gate.log
- ee648b93fb0dac135fba9ffeaeeda62f98ea091071a03947a7cde12b82093b20 logged.out
- 16e4aea14caee206e9b0c2a1df7c968ba1f4482d0f2fae63de54e6254cc31707 negative.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 negative.stderr
- 5194f1535a6746ddf3c43258d468b88f0460888b4a4be752bd4686997f050942 phase.diff
- 6c2002b3fd1d7688e10202c2e1762d6c000546248df1cfc9d264a017114951ad records/m3p4-open-2026-10-05-ef392d4.md
- c794009dcb8051134e3c0f261e6cc9ebc682157400a8a7199fcd6c702182ccc0 records/m3p4-open-2026-10-05-ef392d4.txt
- e9aef61fdedefad530b2c608c9388891b5d4858672e5cfb6c7f0f56a059423b2 records/m3p4-p3-group-2026-10-05-ef392d4.md
- 99f3dc15c790ed2e2c04b11f6c60de94e411a52fa23f74847a5804b67c854e08 records/m3p4-p3-group-2026-10-05-ef392d4.txt
- 5cc7c68eafb5b9240a2a00be9f80c19a4fdaf2ce8d36299ece2a6568054376b1 records/m3p4-p3-linestyle-2026-10-05-ef392d4.md
- 3674f7321002cb00230cfb528ce899246f093624a0a3c1ff0238e85469566ced records/m3p4-p3-linestyle-2026-10-05-ef392d4.txt
- b847515ab9c7bafe7c8d2c4f11cf5194a89fb30d243c0e378019d0bb1f7f966f records/size-m3p3-2026-10-03-01b14c0.md
- 64492d84163345991557d4ab69077da4f5e18df99450bacdb70539fb53ae0e06 records/size-m3p4-2026-10-05-ef392d4.md
- 38eb1e6db2843bd7a22bc24f64613a0c06edc105da07180183c162fefe629e63 report-draft.md
- 8c63e9f61ddf833cc1237d1808e067092122fb2860c434d480897db29b86827a run.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 run.stderr
- 8e35ea48c2d652cc8c8990e5747fccd205689f3b0ee65f16bcbab496a4d5af86 scan-pre-check.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 scan-pre-check.stderr
- 0c2bd587cc183f8f3f51ad02bf66254c44798e341892655bc0f9ae4c7554aa63 size.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 size.stderr
- 79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c spec-0001.md
- 250ee8986af46cf040b0ae1ef28ceeee92bc990d0b7e298e1eb5cc507767b6e5 spec-0026-committed.md
- 7f51d643ac2c95abe7942347ac9b79adda67c273f4fec39b045d11566dd88f29 test.log

## The answer

I checked all 28 inputs against `inputs.sha256` and every hash matched. I read only the files in `inputs/`, changed nothing, and ran no build. The inputs contain no corpus content: the records hold only workbook ids, entry names, schema element and attribute names, counts and hashes, and every input is ASCII.

## Question 1: does the diff do what P3 says, and nothing else? Verdict: partly (yes on scope; three wording points to note)

I compared `phase.diff` file by file with the four commit diffs (doc, carry, catalogue, probe) put together. They are identical: 6 files, and gate.log's combined-diff hash `5194f153...` is the input's hash. I read each code commit's own diff for its "nothing else".

**Doc commit `dcdb50d`** (doc-commit.diff). It changes only `ROADMAP.md` (the P2 row) and `specs/0026`, adding "Planning rulings during P2" and "Planning verification and rulings on P2". gate.log confirms the blobs equal planning's hashes: "doc1's specs/0026... is the blob planning's message gave: as-expected" (250ee898...), and the same for ROADMAP f6a3afa7....

**Carry `34ac951`** (code-commit-1-carry.diff). It does what P2 verification ruling 2 asks:
- The `--linestyle-sample` doc comment now names its two unlisted assertions: "with row M + 1 present after the write" and "`col_split` from 1 to 3". Only `//!` lines change; gate.log says "of them not a //! line: 0".
- It adds the test for observation 6: `a_column_style_on_a_col_covering_the_column_alone_is_replaced_in_place`, where a `<col min="9" max="9" ... style="3">` becomes `style="2"` with every other byte kept.

**Catalogue `de5c09f`** (code-commit-2-catalogue.diff). It has every item "P3" lists:
- The message by feature: "an outline level is out of range" was already listed under rowcol (context line 617). The line-style index check is added to the errors section, per P2 verification ruling 1.
- The coalescing rule, including the whitespace merge.
- "The maximums ... never lowered (decision 3 A)".
- What is never written: "a `collapsed` the caller did not ask for", "a summary row chosen by the write", "an `outlineLevel` above 7", "a style on a cell".
- What part 5 will read: "Milestone 3 part 5's read (decision 6 A)".
- It also corrects the rows-and-columns block and the sheetprops "never written" list.
- Only `//!` lines change; gate.log says "lines added or removed 80; of them not a //! line 0".

Catalogue accuracy against the spec and rulings (I do not have P1's or P2's code diffs, so this is against their stated contents):
- These match: the attribute order and the example row `<row r="5" s="2" customFormat="1"/>` (P2 ruling 2); `hidden` false dropping a hidden attribute of any value (P1 verification ruling 1); a created `sheetFormatPr` with `defaultRowHeight="15"`, set attributes first, a non-number counting as 0 (P1 rulings 4 and 5); the whitespace-only merge where a comment ends the merge (P1 verification ruling 2); "the core's width and hidden calls alone write the split as before" (P1 ruling 2); one shift per worksheet per write; no Excel check yet.
- I cannot verify two phrasings, and they may be loose:
  - **"on a part where the write records a group, a collapsed column or a column style."** P1 ruling 2 limits the merge to the three column calls: `group_columns`, `set_column_collapsed`, `set_column_style`. "A group" could be read as including `group_rows`, which would let the merge run on a part where the only column call is a core width or hidden call. Whether the code does that is not in my inputs.
  - **"its content too"** (merging only when the element's content is also identical) is not in the spec or the rulings. I cannot confirm it against code.

**Probe `ef392d4`** (code-commit-3-probe.diff). It makes every call "Size" asks for:
- `group_rows(sheet, row, row+1, 1, true)`
- `set_row_collapsed(last_row + 1)`
- `group_columns(..., 1, false)`
- `set_outline_summary(Some(false), None)`
- `set_row_style` and `set_column_style` with the index `add_style` returns
- It reads back with its own `quick-xml` walk and prints `level_rows`, `collapsed_rows`, `styled_rows`, `level_cols`, `styled_cols`.
- The later shifts start from the group output, and size.log shows the shift moving the groups ("the fourth output: the grouped, collapsed and styled rows one row down... : yes").

Departures and points no document mentions:
- **An extra write.** "Size" says the calls go "on its fourth write before the shift" and "its fifth write, the shift, then moves them". The probe instead adds a separate write to a new file, plus three new command-line arguments (`<group output> <group row> <group column>`), so there are now three writes after the second output. It may be the only workable reading, since a group beside a shift on the same part is refused, but neither the commit message nor the report says it is an interpretation. The probe's doc comment also still calls the shift "a fourth time", which no longer counts the group write.
- **Commit message wording.** ef392d4 says "its fourth and fifth writes, the shifts, then start from the group output". Only the fourth starts from the group output; the fifth starts from the fourth's output, as the probe's own doc comment says correctly.

Nothing else is in the diff: gate.log shows each code commit inside its own area, "no styles file", no manifest or lock change, no gate file changed, and "pub first lines: at the base 174; at the phase commit 174".

## Question 2: do the logs have every line the spec requires for P3? Verdict: partly

Present, each with "as-expected" or "yes":

**Phases**
- gate and test on both targets with features off and on: gate.log reports "passed: host off 513, host sp 540, host on 584, host pair 612, i686 ..." the same; test.log reports 1020 on both targets.
- Commit messages are ASCII and end with the Co-Authored-By line only (all 4).
- Shape greps, the spec pin and the gate-file listing are all there.
- logged.out shows the five logs ending "all as expected: yes".

**P3: negative log**
- Reached through the binary: GN1 and LN1 (index message), GN2, GN3, GN4 (settings message), LN2 (two-elements message), each with a control.
- Stated unreachable, each with its covering test: LEVEL_OUT_OF_RANGE (`grouping:the_refusals_at_the_call_record_nothing`) and STYLE_OUT_OF_RANGE (`linestyle:an_index_past_cell_xfs_is_refused_at_the_write`), with "reached and stated unreachable together are exactly the messages accounted for: yes".

**P3: rerun of both samples**
- Both samples ran on both labels with size.log's release binaries ("emend-copy host is the binary size.log hashed, once: yes").
- They match the earlier runs: "outputs equal to the earlier run's by sha256 27; lines equal byte for byte 27; diff tables equal 27" for each sample and label, against the P1 and P2 run directories named by record commits d26c470 and 4a23c4b.
- The record files are installed with their hashes.
- The open check covers the keep: "no verdict and no class of M3 part 4 P2's pairs moved: yes".

**Size**
- 25 probes are classified against part 3's record.
- rowcol's cost is 58136 beside 35248, a growth of 22888, and the log says "above 25000 ...: no".
- The core's growth is printed both ways: 970144 - 969672 = 472, "above 2000 ...: no".
- The budget figure is 436928, "21824 under the ceiling".
- There is a cause per probe, with the cause control and the manifest cause listing.
- The grown probe's five lines are checked against the expected group output.

**Harness runs**
- lo and lo_in are ok on all 27; order_preserved yes; files_identical no.
- Entry totals: group "changed 27" with "identical 843"; linestyle "changed 54" with "identical 816".
- The flagged entries equal the inputs with bit 3 set.
- calcChain: "9 of the corpus's 27 ... identical and unflagged".
- Devices are printed for both TMPDIRs and the corpus directory, with a /proc control.

**P2 verification ruling 2**
- gate.log: "every line the carry changes in the tool's main.rs is a //! line" and "code commit 1 adds the exact-cover test".

**P2 verification ruling 3**
- The present-row plant runs for both samples on 75387404b8af: "every flag has a plant that turned it on the present-row case (18 of 18; ruling 3 ...)", and linestyle "present 17".
- The survey counts `hidden` as `1` or `true`, with a planted control: "the group survey counts hidden="1" and hidden="true" as hidden and hidden="false" not ...: yes".

Missing or weak:
1. **The negative log does not account for every reachable shared message.** "Errors" lists shared messages, including "a worksheet whose cols are out of order or overlap cannot be edited" and `Error::Xml` for bytes that are not UTF-8. Both samples edit columns, so the cols-order message looks reachable through the binary. negative.log neither reaches these nor states them unreachable: it lists them as "printed and not accounted", deferring to "earlier parts' negative logs" and two group-level tests. P3 says "every message of the part reachable there, the unreachable ones stated with the test that covers each". Planning should rule whether the shared messages count as "the part's".
2. **`scan.log`**: only the pre-check (RESULT: PASS) exists. The report says the final stage runs after the check, so this is expected; no final scan log is among the inputs.
3. **Earlier-part phase rules**: "Phases" binds the rules `specs/0025` "Phases" lists, but that spec is not among my inputs, so I could not check them.
4. **Minor: gate "pub fn lines listed off and on".** The gate lists them once with their cfg attribute lines plus a consumer `cargo check` off and on, not two separate listings. I take this as meeting the intent.

## Question 3: is every number and quoted string in report-draft.md in a primary? Verdict: yes, with two small wording flags

I found every number and quoted string in a primary:
- **Commits:** the four short shas and the base subject (gate.log).
- **Doc-commit hashes:** spec 250ee898 and ROADMAP f6a3afa7 (gate.log).
- **Carry test name:** in code-commit-1 and test.log.
- **Probe's five line names:** in the probe diff.
- **Log hashes:** 9e1ddcdf, 7f51d643, 16e4aea1, 0c2bd587 and 8c63e9f6 match the input file hashes.
- **Test counts:** 513, 540, 584, 612 (gate.log) and 1020 on both targets (test.log).
- **Four plants at four distinct assertion lines:** test.log, "plants: 4; distinct assertion lines they failed at: 4".
- **Negative cases:** GN1 at 1048574, LN1 at 1048576, the GN2 to GN4 shapes (script comment lines 90 and 91), and LN2's two `<cols>`.
- **Sizes:** 58136, 35248, 22888, 970144 - 969672 = 472, 436928, 21824 and 458752 (size.log and the size record).
- **Run:** 27 ok on each label, 27 of 27 equal, the present-row case 75387404b8af, and 839 pairs (run.log).
- **Scan:** "RESULT: PASS" and "PLANNING_SPEC is not set" (scan-pre-check.log), and the header's "as M3 part 3 P4's scan ran" (scan-pre-check.log line 916).
- **Record names and dates:** run.log and size.log. The date roll fits commit times of 16:38 to 16:41 -0700 on 10-04 and log starts at 2026-10-05T00:14Z.

Flags:
- "each at its first attempt" is supported only by `RERUN_NOTE: unset` in every log. No log states it outright.
- The report says "rowcol's growth 22888 is not above 25000" but does not mention that 22888 is above planning's own estimate of 10000 to 20000 ("Size and packaging"). That is not required, but planning may want to hear it.
- The report does not mention the probe's extra write and new arguments (Q1), or the shared messages the negative log leaves unaccounted (Q2 item 1).

## Summary
- **Q1, partly:** scope is clean and every listed item is present. Flags: the catalogue's "records a group" and "its content too" are unverifiable or loose, the probe's extra write and three new arguments are an undisclosed reading of "Size", and the probe commit message says "fourth and fifth" when only the fourth starts from the group output.
- **Q2, partly:** every required line is present except a full accounting of the shared "Errors" messages in negative.log (cols-order and `Error::Xml` are neither reached nor stated unreachable with a test). The final scan.log is pending, and I could not check `specs/0025`'s phase rules.
- **Q3, yes:** every number and quoted string is in a primary, with the minor completeness flags above.
