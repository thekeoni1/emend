# M3 part 5 P3, independent check 1

The answer of a fresh subagent (not a fork) given only <phase>/check1/inputs (the committed spec specs/0027 with "Planning verification and rulings on P2", specs/0001, the phase diff, the doc commit's and each code commit's diffs, the commit messages, gate.log, test.log, negative.log, size.log, run.log, the scan's pre-check, the stderr files and the runner's summary, the phase's records, and the draft report), recorded verbatim below. <phase> stands for <home>/.cache/emend-build/m3p5-p3. Checked commit (the phase commit): 6b6125b94aa6446279ca4bb6f7312b4bbbfeb442; HEAD when the check ran: 6b6125b94aa6446279ca4bb6f7312b4bbbfeb442. The answer is ASCII as written; it was extracted from the subagent's final message and hashes to c9a2b1d9dca73d7a6f8d4e906dd4df82fc0d62b91722f9e89c293407743a9c18. Its reservations are reported to planning; no second check was run.

Inputs and their sha256:

- 66caa921bc28068dff929876c0699457e6692d734a55614d6e2155238454d4ba code-commit-1-carry.diff
- 986b2e5716d7aed9e950b789862b4ac0c5915776cad34e3395ed6613037feac4 code-commit-2-catalogue.diff
- 397763f97daef1be584a49d37e78e795f50ed03fc0ca0e3a9b8aa17285ad7f52 code-commit-3-probe.diff
- a1ba6f145a9c4404f489234d5f1a58e8a73de891bd86ef8ae301d2b774589b05 commit-messages.txt
- ac71b89667a02beffcf1191557cfccca2ba4c0475fb69b7bce0d8e6b9411537b doc-commit.diff
- b89f5188268fb9f360f2ed8222c07ef1979a5d9d8dd528c77d5794495fe0fa12 gate.log
- 398c0650cf177625a250832877eede6c77401906f5c765131735815350d127b9 logged.out
- a97ef1f4229ead862e682c700cb2f93ea74257a5b5314c55a2a430a1416de897 negative.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 negative.stderr
- a1207df48f28c8495b597bad9d5509d39945ba350f5f4fc246ab993c460dc292 phase.diff
- 356db44bb218dc75b3b382a7bf51cbfc1e2222cb9c660c1c03c97aa4afb5a24b records/m3p5-open-2026-10-05-6b6125b.md
- f4a3598708e4155eb135c062be91e958e8c4877bb1732743c284dc03e14bad8d records/m3p5-open-2026-10-05-6b6125b.txt
- 6cf900d67c0ef4920315bfd24c053b9eaa36baccd2923df7af3c777c524e9036 records/m3p5-p3-read-2026-10-05-6b6125b.md
- b273b9225a225d7eada788b6643ebf6b38f9468fb694bb1722ea384c8284a8ae records/m3p5-p3-read-2026-10-05-6b6125b.txt
- 19fbb1dd1c40bc18df6b70fca3c2004888f85150dcca037c63f430e140188608 records/m3p5-p3-roundtrip-2026-10-05-6b6125b.md
- f2783e9ff941cd79d2bd85fdd9e252646e06e4eeb7a9f882abf0dabb1e5fd674 records/m3p5-p3-roundtrip-2026-10-05-6b6125b.txt
- b33ec2843e033156fef26e2e2116f8800e4becebadcec3830b1c858fbb8c776d records/size-m3p5-2026-10-05-6b6125b.md
- 2358d37096d6879ccd93f6d9ab81d3ef5ca77f5ea9b871441386da26b2948680 report-draft.md
- ef08e33ce340466322c10551159a75a3fb4f2335473bd1b2787c5a6c961947d9 run.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 run.stderr
- faeb5ee73df1e1b009d786bf7a8da52dce1fd7db0b9c9770a14dd85d3022fb81 scan-pre-check.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 scan-pre-check.stderr
- c8dbc364252897b9e31170f504e31649936cab2db15fee699c997e19830413f3 size.log
- e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 size.stderr
- 79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c spec-0001.md
- 32e73e1fcd9ce90b00e61b7b6cd162c8e1798b6e6a16a282afffd26d5ca209df spec-0027.md
- b403a6aa4f511bd63bf747481a1671aa4ffb3073811afe8e8c49bbd6cb3994f7 test.log

---

# Independent check 1: M3 part 5 P3

A CLAUDE.md for the project was in my context from the harness. I did not use it as evidence. Every judgment below comes from the 27 input files alone.

## Input verification

I ran `sha256sum -c ../inputs.sha256` inside the inputs directory. All 27 files report OK and the exit code is 0. `spec-0027.md` hashes to 32e73e1fcd9ce90b00e61b7b6cd162c8e1798b6e6a16a282afffd26d5ca209df, which is the hash the phase pins.

I also removed the commit headers from each per-commit diff and hashed what was left. The four results match gate.log lines 2556, 2558, 2560 and 2562 (647f910d, 85f92321, 5b776d07, 153fabb9). `phase.diff` matches the combined-diff hash at gate.log line 2564 (a1207df4). Its added and removed lines are exactly the four per-commit diffs put together: 1096 lines on each side, the same set.

## 1. Does the diff do what the spec says and nothing else?

**Verdict: YES with reservations.**

**Doc commit** (I read doc-commit.diff). It changes `ROADMAP.md` (the P2 row only, line 23) and `specs/0027-read-back.md`. Both blobs match the hashes in the message (gate.log lines 2546 to 2550). Nothing else is in it.

**Carry, ruling 3** (I read code-commit-1-carry.diff).
- The change itself: `sheet_read.rs` lines 43 to 56. An absent attribute now takes the third element of `options()`, the schema default, instead of the struct's default. The doc comment is updated at lines 30 to 36.
- The two tests the ruling names are both added:
  - `a_protection_with_sheet_false_round_trips_through_protect_sheet`, gated to `metadata` (lines 86 to 113).
  - `a_url_hyperlink_with_a_range_ref_reads_its_top_left_cell` (lines 115 to 146). Its fixture URL is example.com.
- One existing test is renamed and its expectation changed to `sheet: false` (lines 64 to 78). The behaviour change forces this.
- Nothing else is in the commit. The gate confirms it: 2 paths, both inside the area (gate.log around 2640). test.log section 205 confirms exactly one test was removed by the rename.
- The plants show the tests bite: L1, L2 and L3 each fail at their own assertion (test.log lines 558 to 586 of the filtered section; the log's "plant L1" through "plant L3" lines).
- I could not check from the inputs that `options()` really returns the writer's comparison table as its third element. Plants L1 and L2 support it.

**Catalogue** (I read code-commit-2-catalogue.diff). Only `//!` lines change: 152 changed lines, 0 that are not `//!` (gate.log line 2680). Against what the spec says the catalogue must state:
- **The two messages by feature:** Errors paragraph, lines 40 to 56, with the existing bullets from line 58. Present.
- **What the typed view drops, element by element:** lines 149 to 161. Covers the five font children, the three alignment attributes, unnamed attributes, the insides of alignment and protection, later alignments, colour precedence, later views, panes and selection, the theme, and dxfs/cellStyles/cellStyleXfs. Present.
- **Gradient, empty fill and split pane as None:** lines 162 to 165. Present.
- **The None-fill asymmetry:** lines 170 to 173. Present.
- **The ranges refused:** lines 174 to 184. Present.
- **useFirstPageNumber and orientation:** lines 166 to 169. Present.
- **The hyperlink target rule:** lines 185 to 190. Covers r:id over location, unescaping, caching and range ref. Present.
- **Password never read:** line 194. Present.
- **The duplicate rule and the second sheetFormatPr:** lines 196 to 199. Present.
- **P1 ruling 1 item 2** (Package::styles refuses a part the typed read cannot take): lines 42 to 45. Present.
- **The table's hold with its figures:** lines 222 to 226 (216572 and 266288 kB, 46.2 and 56.8 bytes per item, near 46 million items). Present.
- **The iterator beside it with its tier d peaks:** lines 228 to 233 (7720 and 7536 kB). Present.

I found nothing missing or stated wrongly among the required items. Three minor reservations:
- **(a)** The Errors paragraph names only the writers' two-element messages as ones the reads meet. It does not say that the reads also meet the other shared messages spec "Errors" lists (spec lines 292 to 300): "no such part", specs/0010's reference message, part 2's unbound-prefix refusal, and Error::Xml. P3's text does not strictly require this.
- **(b)** The 56.8 bytes per item for the host appears in no input primary. It is consistent with 266288 kB over 4800000 items.
- **(c)** The iterator peaks quoted are the runner's figures. Planning's lower figures (4348 and 3728 kB, spec lines 663 to 666) are not mentioned. The quoted figures are accurate as stated.

**Probe** (I read code-commit-3-probe.diff).
- `tools/size-probe/Cargo.toml` gains the feature `read = ["emend", "xml", "emend/read"]` and a `[[bin]]` entry with `required-features = ["read"]` (lines 29 to 44).
- The new binary does the format probe's work and then `read_back`: `cell_style` of every cell of the named sheet with distinct fonts counted, `columns`, `rows`, `sheet_settings` merges and `shared_string_items` (lines 258 to 299). This is what "Size" defines.
- size.log line 7205 shows its second output is the format probe's, byte for byte, so the earlier work is unchanged.
- Only those two files are touched (gate.log line 2648).
- No styles file, lock file or crate message changes (gate.log lines 2700 to 2735).

**Commit messages.** commit-messages.txt is ASCII. Each message ends with the Co-Authored-By line only, and none has a session line.

## 2. Do the logs contain every line the spec requires?

**Verdict: YES with reservations.**

- **Gate:**
  - Crate tests run with features off, rt, read, pair, sp and md on both targets (gate.log line 3292).
  - The `pub` first lines are 179 at the base and 179 at the phase commit (line 3022).
  - The gate files are listed against the diff: 10 checked, 0 in the diff, no self-test needed (line 2617).
  - Clippy and doc are clean for every feature set.
  - The script is printed (line 79). The log ends all as expected (line 3430).
- **test.log:**
  - Workspace: 1061 tests on both targets (line 5511).
  - Crate runs: off, read, richtext, richtext+read and read+metadata on both targets, plus all-features (874).
  - The three added tests pass on both targets (lines 4884 to 4888).
  - The plants are present. The log ends all as expected (line 5525).
- **negative.log:**
  - Eight refusals are reached through the gate's binary, each with a control: NF, NI, NC, NV, NZ, NR, NH and NP (line 1881). Together they cover the part's two messages and the writers' three.
  - The one unreachable raise, NOT_WORKSHEET, is stated with its covering test (line 1607).
  - The phase adds no message (line 1583).
  - Reservation: the other shared messages are "printed and not accounted" as one group, with no covering test named for each (line 1610). That list also leaves out specs/0010's "a cell holds a reference this layer cannot expand".
- **size.log:**
  - 26 probes measured, 25 compared, and the new one named as new (line 5597).
  - The feature's cost is printed with its base: 1022116 - 970144 = 51972 (line 6348).
  - The core's growth is printed with both figures: 970144 - 970144 = 0 (line 6351).
  - The budget is judged against 458752: 21824 under the ceiling (line 6357).
  - A cause is given per probe (lines 5880 onward, for example line 5926). The cause control is at line 5954 and the manifest cause listing at line 5879.
  - The rowcol description is corrected in the record's probe table (size record line 53). The source of that text is the script's ROWCOL_DESC at size.log line 3929. No check line asserts that the old wording is gone.
  - The record is written and installed (lines 7894 to 8101).
- **Release binaries and samples:**
  - run.log lines 2345 to 2360 match all four binaries against size.log's hash lines (size.log lines 7871 to 7874).
  - Both samples are rerun on both labels, with outputs, lines and diff tables equal by sha256 to P1's run (e727b25) and P2's run (6ec0055) (run.log lines 3349, 3429, 3552 and 3632).
  - Both samples are recorded.
  - The run log also prints the devices of TMPDIR and the corpus (line 3265 onward) and the calc-chain count with its rule (lines 2367 and 3687).
- **Open record of the keep:** 1020 pairs, 3 new over P2's record (run.log line 3812). No verdict moved and there is no finding (lines 4896 and 5028 to 5045). The record is installed.
- **Spec pinned in every log:** gate, test, negative (line 70), size (line 4) and run (line 4) all pin 32e73e1f, and the scan confirms 5 of 5 (scan-pre-check.log line 2642).
- **Scripts printed:** every log prints its script.
- **spec-0001 per-phase rules:**
  - My own greps over the inputs find 0 control bytes and 0 non-ASCII bytes in any log or record.
  - The three forbidden shapes count 0 in all six logs.
  - Every `rm -rf` is the guarded `${var:?}` form or is inside the printed `counting.sh`.
  - Paths are written as placeholders. The few `/home/` and `/tmp/` strings are scan patterns or controls, not machine paths.
  - Each log checks the commit it was given against HEAD, with controls.
  - The pre-check scan ends RESULT: PASS (line 2867). Its final stage is still to run.

## 3. Does every number, hash and quoted string in report-draft.md appear in a primary?

**Verdict: YES with reservations.**

These all match a primary:
- **Commits:** 71d338e, afc6a91, 6b531f6, 6b6125b, 9d89487 and the base's subject (gate.log).
- **Doc hashes:** 32e73e1f and dcce14a3.
- **Test names:** commit-messages.txt and test.log.
- **Gate counts:** 513, 533, 547, 567, 540, 562 and 179.
- **Workspace count:** 1061.
- **Negative log:** "eight refusals".
- **Size figures:** 1022116, 970144, 51972, 55000, 2000, 436928, 21824 and 458752.
- **Run log:** 27 of 27, 1020 pairs and 3 new.
- **Log hashes:** b89f5188, b403a6aa, a97ef1f4, c8dbc364, ef08e33c and faeb5ee7 (inputs.sha256; the first five also in the scan log, lines 344 to 384).
- **Record names:** every one the report gives.

Reservations:
- "findings 0" is printed nowhere as a number. The primaries say "no finding" (run.log line 4896), and the open record shows "None" under both finding headings.
- "each at its first attempt" rests only on RERUN_NOTE being unset in each log.
- The disclosures (the doc commit's amend and the rewrite of the code commits) have no primary. The report says so itself.
- The report does not mention three things the logs show:
  - Three probes got a "code" verdict. The richtext probe grew 728 bytes and the richcomment probe 712 (size.log lines 6311 and 6312), which takes the richtext feature's cost to 13368, against 12640 in the spec.
  - The printnames probe also got "code" while its figures stayed the same.
  - The read feature's 51972 is above planning's estimate of 35000 to 50000 and the outline's 30000 to 45000, though below the 55000 that must be reported.

  None of this is required, but it is worth telling planning.

I found no disagreement between any number in the report and a primary.
