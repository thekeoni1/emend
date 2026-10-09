# Independent check 1, M3 part 7 P3

Written by the implementing session (the session that continued the phase under specs/0029 "Planning rulings during P3 (2026-10-09)" ruling 1). Checked commit (the phase commit): 09d2e0f41454077456e1463f63415dcef4562360; HEAD when the check ran: 09d2e0f41454077456e1463f63415dcef4562360. The checker was a fresh general-purpose subagent given only the directory <phase>/check1/inputs (<phase> stands for <home>/.cache/emend-build/m3p7-p3), read-only, never the conversation. The inputs were assembled by the previous implementing session before ruling 1 was written, so their spec-0029.md is the blob of the doc commit db70ecd and holds no ruling of 2026-10-09; before the launch this session checked them against <phase>/check1/inputs.sha256 (sha256sum -c, every line OK). Its inputs, by sha256 (sha256sum over each file, paths relative to that directory):

    e9605a88d6bf1b48acd1e1e4b1597bcb8f24c1b86ec35c575af07344b496a569  ./commit-09d2e0f.diff
    936133ecf14413130a2c2c268c8e5b3c676c531afa498354519a9a8b156e67d5  ./commit-db70ecd.diff
    b397dcbe97e03655f378fe5a82ee91a0d5cb7d12eef0dbb4692d28aed9d2b847  ./commit-f065758.diff
    d735d3720f03baae5a3c99dff73d3f52352c108a3bb342782623bf957d58b426  ./files.tsv
    0eae242bd8297ce0eca070a9894269d19d121968a21311474c38fb5003daf0e6  ./gate.log
    7eb4382348b4fdaad8a23562c8777c86f5cc10f98a12ce2d1787a77949106fe8  ./listing.tsv
    d47f23c5b7f5394ccbb089281d3b31b019c3efe64b93ed55545f58d40ad86ca9  ./logged.out
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./logged.stderr
    6c0511e724c45ca4271ba9602356577bc9bc54992f931da7a26b8a937813354b  ./make_review.py
    e566b803ba68b7c3fb7b28178fb5c27ec48fabbc84babd5e537c41500df0337d  ./messages.txt
    f6665e8ca956beb8d9e6f097e8adbc245da25cfcc2ca98cf066e41dd39712c9a  ./negative.log
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./out-gate.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./out-negative.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./out-run.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./out-scan-pre-check.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./out-size.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./out-test.stderr
    9d24c4be8ec6f26b93d9e4d3a6e8d52023ca10ddcda6485dbe96dc0ac6c2ad8b  ./phase.diff
    d68acbaa21903a9658612438c826692b1d89ca25a9251811d9d74d082943e463  ./records/api-review-m3p7-2026-10-09-09d2e0f.md
    3ff73ef0410d417dec8f656e4611eecb76730e9b063f6e6c0fd694b0f5938842  ./records/m3p7-p3-anchor-2026-10-09-09d2e0f.md
    4a784f8ff6a29e9ec41a5313beeacfdd786b1b6bc09c10b03ae7084714bd8184  ./records/m3p7-p3-commentbox-2026-10-09-09d2e0f.md
    cd4d423c5d31554a8d75b93bff4c3ca8186216cb0402da575bcbacc8dcd72dde  ./records/m3p7-p3-copy-2026-10-09-09d2e0f.md
    e5495e97580ce98eaec1036f58234ab0ab5898ee23b946cf0737a13d0fa65b0f  ./records/m3p7-p3-removeimage-2026-10-09-09d2e0f.md
    30e1a5dcb24616469e4200f4d860e1f3b9fd5839c0ff7cc1bfc45b1bae713130  ./records/size-m3p7-2026-10-09-09d2e0f.md
    ae78240f8a6723287958f07429e0184be28d32e3fa655a550f94fd93121a664c  ./report-draft.md
    fe02ae8df060d7582c08c789ceedcb0dcdd00de32613385cd33d10776ef70db9  ./run.log
    1c3457d22318e1b2f57a0a7f5e43d65a5849114b35a892668935f971b2a86901  ./scan-pre-check.log
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./scan-scan-pre-check.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./self-test-self-test.stderr
    dd67ccfb3e0c7827c2f92c8cc9d674b75fbc53be8535661a1531dfc522844c05  ./self-test.log
    7c9b5ba1d4cf038b9f3bf834485d62bee821686c33bd8b2ac0f524bc7e487f47  ./size.log
    7005aa500002197c97e9504ed480e58b090d1a9d6e32618fcd144a8b8a7d31cb  ./spec-0029.md
    fb3f9c9a68e7e05c99df428a48d9972560b071875f32f40519b7d04196f47028  ./specs-0001-at-phase.md
    198be65692e8b0f7cc943545ba4d0f65a06f9a0f9dcefeb48f4da54dd3722d00  ./src/crates/emend/src/archive.rs
    9312b371884e4c1afa2057e814cbd44214deaeff024f1de7119b6f7e29538058  ./src/crates/emend/src/archive/tests.rs
    647aa30fa2f6be545df634feff74e54f65dcbbde892330d1a2e3dd712f4cae05  ./src/crates/emend/src/cells.rs
    528fb51aa1d3e88d0496f02c769c08503bfac9a67c6b851d07e8e2ff18a6341e  ./src/crates/emend/src/cells/tests.rs
    975f6d28e917939e5cdbebf3e89a7447234238e750e81c24ac6a149866909b21  ./src/crates/emend/src/dates.rs
    3b790a2e8b4c783bf867be7efff809da8a2786cc7303b8e44cb2587b6e27509f  ./src/crates/emend/src/dates/tests.rs
    0f28e3a58f26f71dd28da3558f00f7b9dfdcbbc0c76ae76ad735de52049a4bc1  ./src/crates/emend/src/lib.rs
    7ef78fbf26550db05be5684096e9c23a385c9755d22d70cdfe06e70be67869b7  ./src/crates/emend/src/package.rs
    0641e4c75a490e0997cea74f8606f96f645fd115eb33b1c548cfe554b324f11b  ./src/crates/emend/src/package/sheet_read.rs
    f3c1d9cc02b4fa60a6a4a27d4f6dbfb41828ee7ecfa47d2961ab1ce15700f5ea  ./src/crates/emend/src/package/tests.rs
    df6e8a9ad9f79e43224496a95b865777fa1cbfa310bd1614ef4b559be5941f9a  ./src/crates/emend/src/package/tests/comments.rs
    ed904a0c2e2cdc821eab7d2650fc05bfbdbc9fffa24131a9bbc0880dcb7cad71  ./src/crates/emend/src/package/tests/comments/boxes.rs
    bdffd991ef97a584ad63b42dd362cf69a40ec299085ce311085d76e0a7a16c22  ./src/crates/emend/src/package/tests/comments/rich.rs
    305a4acaf9e0aa59e9cf7712aab2983b8c1955217441b67685746b2c1bc68882  ./src/crates/emend/src/package/tests/conditional.rs
    9a9b1d8c98e65539c6e12f529411c34d55436f52e07746f208ada253fb76a878  ./src/crates/emend/src/package/tests/copy.rs
    10887b8abd781c3dc70f181ca75eda1b86bf635ad700d4205bcfd730eacb7804  ./src/crates/emend/src/package/tests/dxf.rs
    a3edacf71dad44136480ae1278635d1739efaab46dd756aa71b5b7a16a5a93e9  ./src/crates/emend/src/package/tests/grouping.rs
    1bbbee8cb4bd2658474fbcadbb72a5621f502f5862c217c51e32325b1ebf9e94  ./src/crates/emend/src/package/tests/images.rs
    8eb40731b6dbb5c7494247eb8bf9ec1b4c73363c516bf9eaf1008d4b8b14cb1d  ./src/crates/emend/src/package/tests/images/anchors.rs
    7a0274fd811ee3d979451761be60878c7aee3c4e85c50fa39b002de0b8a4fd0b  ./src/crates/emend/src/package/tests/images/removal.rs
    763f7145a16f72fd2a439078c7dd4b78d4f74759a374c833a68686fb2b23ca6a  ./src/crates/emend/src/package/tests/linestyle.rs
    c5115f9a1c6fc34b3a2f5b309fd21ad9c683175e09e88671dc26a76ce0c7bdbe  ./src/crates/emend/src/package/tests/links.rs
    19a400cb45256b85ea37a86152c9fd307fa80c2af0a3386aac7cf7162b271f67  ./src/crates/emend/src/package/tests/names.rs
    8ed1ca32e8cd27f50011d51f37c6ba6048996eb5acd055468c7ab2b06c1deb83  ./src/crates/emend/src/package/tests/names/print.rs
    aa1ac75e47a801179159d327b3297ce5c17b7e445b5c20a29e987767824bee5b  ./src/crates/emend/src/package/tests/properties.rs
    458dc480dae790ea2c1fc3eacb21046ef22cec743bce2a4d909ce7d92018a7f6  ./src/crates/emend/src/package/tests/protection.rs
    ae4571e1a4675cb1f2fa4da9f1bfc8b0fefdb22894827edb33c6df4cec4dcd8a  ./src/crates/emend/src/package/tests/readback.rs
    1b9f9a6faedb27ee37d8acdd2600f31731516032a3f12a80731aa77b2565164e  ./src/crates/emend/src/package/tests/richtext.rs
    e44e8d3429d7defe15134527b738db82a84c5a91e50db3fcbce6fa6f96cb09ec  ./src/crates/emend/src/package/tests/rowcol.rs
    f73879da1d88b72d3812a42a230c2bce7cc478919ce11eb33fe226401f24d394  ./src/crates/emend/src/package/tests/sheetprops.rs
    23d2225fc0827ae61b937b172e63144c091d5fc9235b4a6eabbcf12a6c096372  ./src/crates/emend/src/package/tests/sheetread.rs
    ae49b8bd3473eab9a4ac27c0f265b008ed9f4dad9fbf46a3e864c58a2346cc40  ./src/crates/emend/src/package/tests/shift.rs
    706f35185b72fe5b6a104d26c7ae883b78f4c02eef294fd1be80b0112481092a  ./src/crates/emend/src/package/tests/tables.rs
    67ef50650cb9a97c85595a34ff6c097080841449a72dd350f6a6d348950c69b2  ./src/crates/emend/src/package/tests/validation.rs
    3a8954692f088e89b7fdf5893e6085348f427ee0dc012049d41a4ff70c53d5c4  ./src/crates/emend/src/package/tests/visual.rs
    198d0dc62c5f57a80cedddb18a9fa2eeaff1e92e97394a77c1843102413ec603  ./src/crates/emend/src/styles.rs
    a40673a42397382d18ec5364cbf35927998a814c323e401d86c48ddbb8a32921  ./src/crates/emend/src/styles/elements.rs
    6ef87ab9f345c067523c9481a52fa77c8c5e0fa316f580cead096ac7147c0787  ./src/crates/emend/src/styles/elements/tests.rs
    e58eee0239c1d97105eee696ba272eddfbea1b57527b2ae1eb1b458b35e7017c  ./src/crates/emend/src/styles/read.rs
    a516799f63b71a8cc585d3f3904b723f79e28dc77ac73308be7075bdd23ab7e8  ./src/crates/emend/src/styles/tests.rs
    e57cbb4f498246b0510a417ade4040ec1d85b4194b952326d74869d2d6d848f5  ./src/crates/emend/src/testing.rs
    69443305df92acc123890e26ee30a6b48fe2f18d505ef40615c47c598ca00c0f  ./src/crates/emend/src/xml.rs
    7159bda84f7952744db2bc68227ce84d16bb92869dd680636aec071974381ad5  ./src/crates/emend/src/xml/tests.rs
    d7d4bc3824bcc9d3b8271fe5637565f73826f57cad5fc1a728ea7d7332384df2  ./test.log

The answer below is the checker's final message, extracted from its transcript into <phase>/check1/check1-answer.raw, verbatim (sha256 da540033861e7f1a9bc57c5e2a467f95498fbbacd0ff0171f8a5aafc1f57586c); it holds no non-ASCII byte and no absolute machine path, so nothing in it is replaced, and the text below the rule is byte for byte that file (sha256 da540033861e7f1a9bc57c5e2a467f95498fbbacd0ff0171f8a5aafc1f57586c).

---

Context note: the harness put CLAUDE.md, the memory index and a git status into my prompt. I used none of it as evidence. Everything below comes from <phase>/check1/inputs.

Scope note: the brief says spec-0029.md has a section "Planning rulings during P3 (2026-10-09)". It does not. The file has no such heading and no "2026-10-09" string. Its sha256 is 7005aa500002197c97e9504ed480e58b090d1a9d6e32618fcd144a8b8a7d31cb, which is the blob db70ecd committed (messages.txt line 34; gate.log lines 81-84). So I could not check the phase against any P3-time rulings.

**Q1. Does the diff do what the spec says for P3 and nothing else?**

Verdict: yes with reservations.

- **phase.diff is the three commits combined.** Split into per-file blocks, phase.diff equals the union of the three commit diffs, block for block. All four diffs hash to the values gate.log prints at lines 3407-3415, including combined-diff hash 9d24c4be..., checked with sha256sum.
- **commit-db70ecd.diff (doc commit).** I read it in full.
  - specs/0029 gains only the section "Planning verification and rulings on P2 (2026-10-08)" (lines 23-87 in that diff's frame).
  - specs/0028 gains only the three-line amendment under P3 ruling 11, T2 (lines 9-11), which P2 ruling 3 asks for (spec-0029.md lines 518-525).
  - No line is removed. Nothing else: confirmed from this diff.
- **commit-f065758.diff (rules commit).** One bullet is added inside "Rules for every phase" (diff lines 9-18; specs-0001-at-phase.md lines 300-309), dated 2026-10-08 and signed by specs/0029. Its content is decision 1 A's policy:
  - the four places agree;
  - the gate reads the first three from the index, with a control that plants a disagreement;
  - a move is a recorded decision;
  - the crate builds on no compiler older than the pin.

  This matches spec-0029.md lines 167-174. No line is removed. Nothing else: confirmed from this diff.
- **commit-09d2e0f.diff (phase commit).**
  - The workspace gains `rust-version = "1.98.1"` with a three-line comment (lines 9-12).
  - Each of the five members gains `rust-version.workspace = true` (lines 24, 86, 98, 110, 122).
  - scripts/check.sh gains pins() and its use inside the hygiene step before `echo "PASS hygiene"` (lines 36-74): a positive control on agreeing fixtures plus a moved CI pin that must be refused, then the three readings from the index.
  - That is decision 1 A (spec-0029.md lines 166-174) and P3's "decision 1's rust-version and the hygiene check with its control" (line 304).
  - No .rs file and no Cargo.lock change, which matches gate.log lines 3574-3575. Nothing else: confirmed from this diff.
- **Self-test.** check.sh is a gate file, so a self-test is due. self-test.log ran at 09d2e0f with RESULT: PASS (self-test.log lines 3 and 108). The gate asserts it at gate.log lines 3492-3515.
- **Reservation 1.** The P3 rulings named in the brief are missing from the spec (see the scope note), so "nothing else" is judged against the spec as committed.
- **Reservation 2.** The in-script positive control in check.sh (diff lines 57-67) has no line of its own in any log. Its success is only implied by the gate's "PASS hygiene" (gate.log line 4080) and by T3's and the negative log's clean PASS in clones.
- **Reservation 3.** I can read the CI-pin awk (diff lines 49-50) only against the fixture text. The CI workflow is not in the inputs. T3 reads the real CI pin as 1.98.1 by another form (test.log lines 3444-3446), and N3 shows a half-moved CI file reads "-" (negative.log lines 1652-1666).

**Q2. Do the logs contain every line the spec requires?**

Verdict: yes with reservations.

- **P3's logs are all present.** gate, test, negative, size, run, self-test and the scan pre-check exist. Each ends all as expected:
  - gate.log line 4303, test.log line 3470, negative.log line 1680, size.log line 9392 and run.log line 4939 each read "checks all as expected: yes";
  - self-test.log line 108 and scan-pre-check.log line 3208 read "RESULT: PASS";
  - "structural checks failed: 0" is at scan-pre-check.log line 3206.

  No log has an outcome line ending ": UNEXPECTED". Every stderr file is 0 bytes. The final scan.log does not exist yet; that is expected.
- **Decision 1 A's hygiene check and its control.**
  - gate.log: every member's rust-version read back as 1.98.1 by cargo metadata (lines 3584-3594), and "PASS hygiene" (line 4080).
  - test.log T3: a clone passes, and a clone with rust-version moved is refused with "1.98.1 1.98.1 1.98.0" (lines 3433-3457).
  - The negative log's planted disagreement is refused. NC passes (negative.log lines 1619-1627). N1 moves the toolchain file and is refused with "1.98.0 1.98.1 1.98.1" (lines 1628-1639). N2 moves both CI pins and is refused (lines 1640-1651). N3 moves one CI line and is refused, reading "-" (lines 1652-1666).
- **Item 9's per-probe attribution.**
  - Each probe's own sources are listed (size.log lines 6792-6819), read by a reader the log prints, with fixture controls (lines 6782-6790).
  - The commits touching each probe's sources are listed (lines 6831-6886).
  - The section at lines 7296-7329 gives two moved probes, each with cause 8d54741, and "of them with no cause among their sources: 0", with a control.
  - Part 6's open question is answered as unanswerable from this range (line 7329).
  - Reservation: attribution is at file granularity. package.rs is in every emend probe's sources, so all 25 emend probes name 8d54741, moved or not. The method cannot tell the two that moved from the rest. This is what item 9 literally asks for, and the record says so.
- **Size record.** The record is written and installed with sha256 30e1a5dc... and 24244 bytes (size.log line 9373). Its cause table is at record lines 128-158. Budget 436992 and 21760 under are at size.log lines 7390-7393 and record line 220. Core growth 0 is at size.log line 7381.
- **Pub listing, per-file count and its control.**
  - The listing program text is printed with its sha256 (test.log line 2347 onward).
  - The fixture control is at lines 2794-2799.
  - Per-file grep counts beside the rows are at lines 3374-3386, with the control "refuses one file differing" at line 3389.
  - The archive's 43 .rs files are all walked or under cfg(test) (lines 3390-3392).
- **cargo doc and its count.** Exit 0, no warning line, with a control (test.log lines 3410-3416). The count is at line 3418.
  - Reservation 1: the line labelled "documented items in the index: 125" counts five section anchors. Its own by-kind breakdown shows them (lines 3419-3423). The true figure of 120 appears only "for comparison, not asserted" (line 3431).
  - Reservation 2: the doc build printed "Finished ... in 0.05s" (line 3412). It was fresh in the target directory, so no rustdoc ran in this log. RUSTDOCFLAGS is part of the fingerprint, so a fresh unit implies an earlier clean `-D warnings` build. The log does not say this.
- **The four samples on both targets match part 6 P4.** Eight lines say "every workbook's outcome, output (sha256), line and diff table are part 6 P4's run's: yes" (run.log lines 4065, 4212, 4402, 4476, 4593, 4667, 4784, 4858). Each reports 27 of 27 equal. The run directories are named by part 6 P4's record commit, resolved by subject (lines 27-38 and 3891-3909).
- **Self-test.** At 09d2e0f, 50 cases, 0 unexpected, RESULT: PASS (self-test.log lines 3 and 106-108). It was launched detached (line 4) and asserted by the gate (gate.log lines 3492-3515).
- **specs/0001 rules binding every log.**
  - The phase commit is given and compared with HEAD, with null and short-sha controls (gate.log lines 111-120; test.log lines 66-69; negative.log lines 49-62; size.log lines 41-45; run.log lines 38-41).
  - Each log prints its script.
  - Messages are ASCII and end with Co-Authored-By only (gate.log lines 3337-3363).
  - The scan finds 0 non-ASCII, 0 control bytes and 0 U+2014 in all six sibling logs (scan-pre-check.log lines 2862-2915), and 0 forbidden shapes and ruled rm forms only (lines 3110-3172).
  - The environment section is first after the header and input lines (gate.log lines 12-26).
  - The prune lists 7 plus 2 names with du -sb and `rm -r` exit 0, and the kept names are present (gate.log lines 40-78).
  - Reservation: test.log has ANTHROPIC_API_KEY only as an input line (line 8), with no check-and-control line as the other logs have. specs/0001 "Rules for every phase" does not require one.

**Q3. Does every number and quoted string in the draft report appear in a primary?**

Verdict: yes with reservations. All hashes match my sha256sum of the input files and the logs that print them. The commit ids, the prune byte counts (gate.log lines 50, 67, 75), 545, 125, 120, 64, 128, 436992, 458752, 21760, the T3 readings, the timestamps (self-test.log lines 3 and 106; logged.out) and the verdict counts (record lines 610-614) all appear. The exceptions:

1. **Report line 16** says "the eight records ... each with the two Binaries lines in its header". Only the four .md records carry them (each record's lines 12-13). run.log line 1090 states the check is on "the .md". The .txt records are not in the inputs, so for them the claim has no primary.
2. **Report line 21**: "<phase>/review/make_review.py, sha256 6c0511e7..." and "as the record says".
   - The hash matches the input make_review.py, but no log or record prints it. It is the session's reading.
   - The record (line 9) says only "a formatter kept with the phase's files", with no path and no hash. "As the record says" holds only for the fact that a formatter exists.
3. **Report line 13**, "125 link lines in all.html". The log's words are "documented items in the index: 125" (test.log line 3418), counted from grep -o matches. The number matches; the wording does not.
4. **Report line 25** (the self-test files moved with mv -n) and **line 26** (logged.sh corrected, the first form kept in superseded) are session facts. No primary in the inputs records them. gate.log line 9 does show SELF_TEST_LOG under <phase>/self-test, which agrees with the outcome.
5. **Report line 21**, the review record's hash d68acbaa... It is in a primary: scan-pre-check.log line 3036.

**Q4. Against src/, does every reachable `pub` item appear in the review record exactly once?**

Verdict: yes.

How I checked:
1. I wrote my own extractor and ran it from stdin, writing no files. It blanks comments, strings, raw strings and char literals, tracks braces, and records every `pub` token not followed by `(`. It covered the 10 non-test files reached from lib.rs. The other 33 .rs files are testing.rs and the files under the cfg(test) `tests` modules (lib.rs line 44; `mod tests` in each module).
2. It found 545 `pub` declarations, none inside a fn body or block. The per-file counts are 18, 27, 45, 12, 309, 19, 14, 43, 51 and 7, which equal files.tsv and test.log lines 3376-3385.
3. A plain `grep -c '^[[:space:]]*pub '` per file gives the same counts.
4. The set of (file, line) pairs equals listing.tsv's exactly, with no extras either way and no duplicate keys.
5. Reachability:
   - lib.rs declares six `pub mod` (lines 37-45). styles/elements, styles/read and package/sheet_read are private.
   - Each of their 22 module-level items is named in a `pub use`: styles.rs lines 126-131 and package.rs line 1865.
   - Their methods and fields belong to those types or to Package.
   - No pub trait, static or macro exists.
6. Parsing the record's 545 table rows in order:
   - numbering is 1 to 545;
   - each row's item text, kind, feature, added-by and module equal the corresponding listing row;
   - no (module, item, kind, feature) key repeats.

Observations:
- Enum variants of the 31 public enums are public API but are not `pub` items, so they are not listed. The spec's wording does not ask for them.
- The listing program is a heuristic parser. My check is an independent second parser, not rustdoc JSON.

**Q5. Are the verdicts allowed, and do the closing counts and list agree with the table?**

Verdict: yes with reservations.

The verdicts:
- 539 rows read exactly "keep".
- Five read "rename before publication (`<name>`): <why>": table rows 63, 87, 90, 147 and 463, at record lines 89, 113, 116, 183 and 514.
- One reads "hide: <why>": table row 43, record line 64.
- None is "widen or narrow".

All are among the four allowed forms, each rename with its proposed name and the hide with its reason. The counts (record lines 610-614: 539, 5, 1, 0, 545) equal the table's. The ordered list (lines 618-623) holds the same six items with the same names and reasons.

Reservations:
1. **Order.** The list runs renames first, then hide, then by module and name (make_review.py line 113), not table order. The spec does not define "in order".
2. **List item 5** names `emend::package::sheet_read::Target`. sheet_read is a private module (package.rs line 1863), so that path cannot be named from outside. The public path is emend::package::Target.
3. **Kind's reason** says emend::Kind sits "beside SheetKind" at the crate root. SheetKind (package.rs line 2524) is not re-exported in lib.rs (lines 47-76).
4. **The hide reason** rests on "grep over the workspace in the session", which no primary shows. Within src/, Archive::prefix_len is called only from archive/tests.rs (lines 139, 157, 255, 291, 315). The tools crates are not in the inputs.
5. **"Written by hand" (spec-0029.md line 183).** The table is assembled by make_review.py, with only the six non-keep verdicts typed by hand (make_review.py lines 25-44). Every other row defaults to keep. The record discloses this at line 9.
6. **The "spec" column is heuristic, and at least two rows are wrong.**
   - Row 12 (the rules re-export) is attributed to a791e2c and specs/0019. The items it re-exports were added under specs/0020 (record lines 234, 253, 264).
   - Row 15 (the sheetprops re-export) is attributed to specs/0019. Its items were added under specs/0025 (record lines 405, 426, 440).
