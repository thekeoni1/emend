# Independent check 1, M3 part 7 P5

Written by the implementing session. Checked commit (the phase commit): f39bc7bcca895a1020eda34f1f44d9d1e8da2538; HEAD when the check ran: f39bc7bcca895a1020eda34f1f44d9d1e8da2538. The checker was a fresh general-purpose subagent given only the directory <phase>/check1/inputs (<phase> stands for <home>/.cache/emend-build/m3p7-p5), read-only, never the conversation; the spec's check for this phase is the run log's directory list against the specs' "Harness runs" sections. Its inputs, by sha256 (sha256sum over each file, paths relative to that directory):

    842bcb9dad8940a19820282d3a0adf610b4c8fe580681b8060a48ac8e8a66d70  ./CLAUDE.md
    a5076384b85c5a5a4f4facee6937576a3e037c1908be26091ae207e45e92134e  ./commit-515c3e2.diff
    856057ff493c13808c30a6109f9439df6e6e74257f69a4d0efd3d7b870253465  ./commit-f39bc7b.diff
    f5c6dc161eb24f25f91a61d4b9938d002491f10f0cf87c1e510c3663cac9b5f9  ./excel-check-list.py
    aa666d92f3ac27d26ee3d69429b03b2c844f656cc869c0a8b25979ecdf71c346  ./gate.log
    9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa  ./logged.exit
    2cf4c37d7cd68bd013f10b96dcde7ee5f786523a966c67666c959c95291d7095  ./logged.out
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./logged.stderr
    0ce81b6be22970d7322a9aac3338509f956c42d549c747d2fc22ac9a791f926a  ./negative.log
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./out-gate.outer.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./out-gate.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./out-negative.outer.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./out-run.outer.stderr
    29c87c273d8dc6e0accf5237dab75bfcd6e4ef1d1ea8d527935cb6d4bd7102bf  ./phase.diff
    b7b5e04709212a4b42a4eabb96e73fd9163e45fc9ec476588825217343e907b7  ./records/m3p7-open-2026-10-09-f39bc7b.md
    cec68bcaade5728922d0934dee8eb61c73366f1bb04ee9696ea904d4367d6be9  ./records/m3p7-p5-border-2026-10-09-f39bc7b.md
    c919ec8bb74f8a422f70ab057f07011b5925ecf4c0dbb65eb283a6cbe8b57172  ./records/m3p7-p5-cf-2026-10-09-f39bc7b.md
    9946764ced041a6474f2c7844ceabeeaba681c88342a2019ca9fdcbdab0c7f3e  ./records/m3p7-p5-date-2026-10-09-f39bc7b.md
    fa76d643fd62c9569bc222283d7d1f306e0ae0df1a6e56da5ecf0212da4bd979  ./records/m3p7-p5-delete-columns-2026-10-09-f39bc7b.md
    a378f68b601f483b7161b2ec9cd1f620f1f7afd4e25c9911920e35c537143cac  ./records/m3p7-p5-delete-rows-2026-10-09-f39bc7b.md
    6e6c9586b13c32cfb7f0f5ab8c1fe70ded912d799dcbc6f0b619abbfc857d7f2  ./records/m3p7-p5-edit-2026-10-09-f39bc7b.md
    9eec5e0c164ba83714ebb7872b1b8b1c100da874a97e7c0e8d89cdb60a043329  ./records/m3p7-p5-edit-formula-2026-10-09-f39bc7b.md
    d264ee3a437a538fe3dad3cfe0b12f8f98f4c4c837d6dc462bda2afd21463a43  ./records/m3p7-p5-font-2026-10-09-f39bc7b.md
    bce5089ecccc30b5e7bfb4edc49b13422a7ef9a217272c3d5d5c62b926d6c35c  ./records/m3p7-p5-format-2026-10-09-f39bc7b.md
    56faff50faf75f5d20e7715def006add464956f113abcd0f14fb8374733b65de  ./records/m3p7-p5-formula-2026-10-09-f39bc7b.md
    baa4f6623135f57fdb00f9ccca0e38006bb9de9117698caa6ba3d172bbb2229a  ./records/m3p7-p5-insert-columns-2026-10-09-f39bc7b.md
    d2aab558405d67c32d0b129be872d8c543f2c14b8e5e9a6c0b2bcbcb98bd9a18  ./records/m3p7-p5-insert-rows-2026-10-09-f39bc7b.md
    50ec5df9a673fd065c12183b6878042395a6734925adf9894ec9a04647f6848d  ./records/m3p7-p5-link-2026-10-09-f39bc7b.md
    3624f99d8e1c7fbe3d65eb1c3b5544d568730e59bc0d8f3938986cf319ea042b  ./records/m3p7-p5-names-2026-10-09-f39bc7b.md
    e1b26a5b725023edea81f35636aec14b9f4c79e559c21bd2de080fd7f61242b4  ./records/m3p7-p5-printarea-2026-10-09-f39bc7b.md
    e7b41a5ae4a2c4915f908fe4fce169b663fb874701c3d48c533b2b2baf534ff5  ./records/m3p7-p5-props-2026-10-09-f39bc7b.md
    b7bab7912f1d346235b372718f8dfee373db77a16dcff8d4937c4511e6e3f20c  ./records/m3p7-p5-protect-2026-10-09-f39bc7b.md
    81097a22e80b60cfb4986855b4320869b701d3877101382e6ca8dfce713be9f9  ./records/m3p7-p5-remove-2026-10-09-f39bc7b.md
    4463b0c52f45c19ce60dbb44028b40c68b72e610781fb73d7f578eec9bfd8fa8  ./records/m3p7-p5-scale-2026-10-09-f39bc7b.md
    16752fac798944e02ee524763aac5c6f4537f1fd0fdb157f0adafd107137f7f5  ./records/m3p7-p5-sheet-2026-10-09-f39bc7b.md
    bf32f584f0547c4f47247afd4c06dc14cbdae012453fe508e20b8ce6fe89f260  ./records/m3p7-p5-style-2026-10-09-f39bc7b.md
    3764e6370bb461ada643652cfb9738b23e63e3e0db0131483f18829ab7076b23  ./records/m3p7-p5-table-2026-10-09-f39bc7b.md
    b44ff6c42cb2e2f65d3b6025cdd5864135bab6d8a688c9b68e399351dce3731d  ./records/m3p7-p5-validate-2026-10-09-f39bc7b.md
    fd33c2c9c8f7f3e4e25ad4a0ffac9af75d47a55f5ee184f87b58041ecf89524b  ./report-draft.md
    d560dd056d4054ae71a2aaa9146a6c199292acd99f80b80cf1c0ab59b2cc6c94  ./run.log
    bd57368224c73d5157ca6bfb4fcbda3cfadcc52729f317d6a2fd17a6a8e8b2f7  ./runs-listing.tsv
    47231870ef35fe489a63575f57e2a30ed3f88e88fb3586d89d3977c343c1a5a1  ./scan-pre-check.log
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./scan-pre-check.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./scripts-out-negative.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./scripts-out-run.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./scripts-out-scan-pre-check.stderr
    6ba07ae5a568e71cf8e25ade114c2a4fcdf0ed43b98f0e32130b1a6137eef484  ./spec-0029-working.md
    fb3f9c9a68e7e05c99df428a48d9972560b071875f32f40519b7d04196f47028  ./specs/0001-bootstrap.md
    94e1c38f7aafda03bb05b955461191e5914ae73080e48fbfa66ef382960d4611  ./specs/0011-editing-cells.md
    a7cf94e158b5e3559dd5bcddf409733000f9def6a0e040b711bbb5ed6fd3acea  ./specs/0012-styles.md
    0a363cd12c08d06f4b4223b422e2cd454337f93d79ff668cfa42fc5df6216500  ./specs/0013-formulas.md
    9be1047e0e9ec6b7a7d65bca51d298028b9927eac9770f2b4863335609099d31  ./specs/0014-xml-comparison.md
    04a6fac816ce1faad7deb4ec56914a1abf4744777e1aee9633d558bfffb38021  ./specs/0015-open-checks.md
    1e546b57b8f60c69dfc35c9b7c588622f7b127de77d322c61b25707a793e36c6  ./specs/0016-new-workbook-and-sheets.md
    f880aeaacedb4903885e56ab0a98d1523f864098bf4b05d96507ce737c599958  ./specs/0017-fuller-styles.md
    541003c1280e93fad39ec97767d539f1263b2799b9917156fd3c55f92bb9b4e1  ./specs/0018-rows-and-columns.md
    977ef2ba9714c7b3018b873262a32eab87966b796bc09af6fe71bbc3feeb8e1e  ./specs/0019-names-links-properties-protection.md
    5fc132f271b9d9655434a0cae5085c115dd0d5d0f02dd7600c9c5aa58b4ce851  ./specs/0020-validation-and-conditional-formatting.md
    7f723b2ab1db868f5152c31e204f28c885b6445ccacc214f4d155e86ede95aa5  ./specs/0021-tables.md
    bfec1e8fe16060694c8f5bf9b81eda93c9d72f777d6f294f2d2efad63c466c95  ./specs/0022-comments-and-images.md
    edd145655bae295448b7d984f6d1dc5534688f4b3ce08774c7bf5fc31a3734c5  ./specs/0023-scale.md
    ae1ab201b5c7071c36235f2c5d8c253c6badfab95a9f163575ef7daa0e78863a  ./specs/0024-rich-text.md
    ef2fe2221752fd654240a4ebe3f65f07e2b7d1fa37d247493f2dbf7873f713be  ./specs/0025-sheet-and-print-settings.md
    aecc629a60110b0593ea1672671574e69c924a78d0fefa8b485f8f8df0c93ae3  ./specs/0026-grouping-and-line-styles.md
    93db44a72adb54ddc16b22da6d5fa214d490af4383e25044fde13d43774d27e9  ./specs/0027-read-back.md
    9b95ddc92bafa98b5cf84b3eb0cf846efe98a643f6e7d69767476eeac5ab7b17  ./specs/0028-images-and-comments-second-half.md

The answer below is the checker's final message, extracted from its transcript into <phase>/check1/check1-answer.raw, verbatim (sha256 13dce6b491a63089b2b968e785a1657a4e5879f61655bec3715ba15f64b22912); it holds no non-ASCII byte and no absolute machine path, so nothing in it is replaced, and the text below the rule is byte for byte that file (sha256 13dce6b491a63089b2b968e785a1657a4e5879f61655bec3715ba15f64b22912).

---

Harness context (CLAUDE.md, auto-memory, git status) appeared in my prompt. I did not use it as evidence. Everything below comes from <phase>/check1/inputs only.

**Bottom line:** the run log's directory list matches the rerun set the spec defines, the code diff changes the two README lines and nothing else, and the logs hold every line the spec requires. I have three reservations: `--drop-calc-chain`, the closing-commit wording P4 ruling 2 asks for in the records, and a few report sentences with no primary in the directory.

## Q1. Does the run log's directory list cover every corpus sample in the specs' "Harness runs" sections, once each, in spec order, and nothing else?

**Verdict: yes with reservations.**

### Coverage and order
- The sample table (run.log 2153-2196) has 37 rows: "samples 37; rerun 23; found 14" (run.log 2192).
- R4 lists the formula sample as `--run` (run.log 3509) and 35 directories as `--run-plain` (run.log 3510-3544). The check "the --run-plain directories are in spec order: yes" is at run.log 3545.
- The script builds the argument array from the same file it prints from (the printed script at run.log 1777-1787), so the printed list is the list that was passed.
- 37 Harness-runs samples minus printarea gives 36 directories. Each sample appears once. Nothing else is on the list.

### Each sample: where the spec names it, and the directory the log used
Line numbers in the second column are in the named spec file. The directory column is from run.log 3509-3544.

| Sample | Spec and line | Directory (R1 rerun or R2 found) |
| --- | --- | --- |
| edit | specs/0011 305 | 2026-10-09-f39bc7b-edit (rerun) |
| edit-formula | specs/0011 324 | 2026-10-09-f39bc7b-edit-formula (rerun) |
| style | specs/0012 437 | 2026-10-09-f39bc7b-style (rerun) |
| date | specs/0012 459 | 2026-10-09-f39bc7b-date (rerun) |
| formula | specs/0013 237 (rerun again in specs/0014 195; counted once) | 2026-10-09-f39bc7b-formula (rerun, the list's `--run`) |
| sheet | specs/0016 370 | 2026-10-09-f39bc7b-sheet (rerun) |
| remove | specs/0016 382 | 2026-10-09-f39bc7b-remove (rerun) |
| font | specs/0017 471 | 2026-10-09-f39bc7b-font (rerun) |
| border | specs/0017 493 | 2026-10-09-f39bc7b-border (rerun) |
| format | specs/0018 503 | 2026-10-09-f39bc7b-format (rerun) |
| insert-rows | specs/0018 515 | 2026-10-09-f39bc7b-insert-rows (rerun) |
| delete-rows | specs/0018 523 | 2026-10-09-f39bc7b-delete-rows (rerun) |
| insert-columns | specs/0018 530 | 2026-10-09-f39bc7b-insert-columns (rerun) |
| delete-columns | specs/0018 533 | 2026-10-09-f39bc7b-delete-columns (rerun) |
| names | specs/0019 536 | 2026-10-09-f39bc7b-names (rerun) |
| link | specs/0019 548 | 2026-10-09-f39bc7b-link (rerun) |
| props | specs/0019 559 | 2026-10-09-f39bc7b-props (rerun) |
| protect | specs/0019 567 | 2026-10-09-f39bc7b-protect (rerun) |
| validate | specs/0020 557 | 2026-10-09-f39bc7b-validate (rerun) |
| cf | specs/0020 576 | 2026-10-09-f39bc7b-cf (rerun) |
| scale | specs/0020 593 | 2026-10-09-f39bc7b-scale (rerun) |
| table | specs/0021 438 | 2026-10-09-f39bc7b-table (rerun) |
| comment | specs/0022 560 | 2026-10-02-6578925-comment (found) |
| image | specs/0022 582 | 2026-10-02-6578925 (found) |
| richtext | specs/0024 322 | 2026-10-03-0c3e4fa-richtext (found) |
| richcomment | specs/0024 335 | 2026-10-03-0c3e4fa (found) |
| sheetview | specs/0025 381 | 2026-10-03-01b14c0-c-sheetview (found) |
| pagesetup | specs/0025 392 | 2026-10-03-01b14c0-c-pagesetup (found) |
| printarea | specs/0025 405 | rerun into 2026-10-09-f39bc7b-printarea, left out of the list |
| group | specs/0026 266 | 2026-10-05-ef392d4-group (found) |
| linestyle | specs/0026 287 | 2026-10-05-ef392d4-linestyle (found) |
| read | specs/0027 338 | 2026-10-05-6b6125b-read (found) |
| roundtrip | specs/0027 356 | 2026-10-05-6b6125b-roundtrip (found) |
| commentbox | specs/0028 425 | 2026-10-09-09d2e0f-commentbox (found) |
| anchor | specs/0028 437 | 2026-10-09-09d2e0f-anchor (found) |
| remove-image | specs/0028 448 | 2026-10-09-09d2e0f-removeimage (found) |
| copy | specs/0028 470 | 2026-10-09-09d2e0f-copy (found) |

### Samples left out because they run only on generated workbooks or not on the corpus
- **new:** rightly out. specs/0016 365-366 says "The runner does not run this operation over the corpus", and P5 ruling 1 (spec-0029-working.md 833-834) keeps it out. The log asserts it at run.log 2235.
- **Scale tiers:** rightly out. specs/0023 has no "Harness runs" section. Its "The runs" (specs/0023 111-128) and "The tiers" (92-109) are generated workbooks under `--corpus` the scale directory.
- **`--cells`, `--shared-strings`, `--items`** in specs/0024 and specs/0027: rightly out. They are reads on the generated tiers.
- **`--styles`** (specs/0012 425-436) and **`--calc`** (specs/0013 232-236): rightly out. Both leave the output byte-identical to the input.

### Reservation 1: `--drop-calc-chain`
specs/0011 345-348 names a `--drop-calc-chain` corpus run in "Harness runs" whose outputs do differ: nine chains dropped and parts changed. It is not a `-sample` flag. P5 ruling 1 (spec-0029-working.md 819-836) names only edit and edit-formula for specs/0011. Earlier Excel lists (specs/0015 123, specs/0016 466-472) did not carry it either. So leaving it out follows the ruling, but neither the run log nor the ruling says it is out. Planning should state it.

### Reservation 2: order inside specs/0028
The check at run.log 3545 tests spec-number order only (its printed form is at run.log 1782). Inside specs/0028 the list reads anchor, commentbox, copy, remove-image. The Harness runs section's order is commentbox (425), anchor (437), remove-image (448), copy (470). The ruling says "spec order from specs/0011 to specs/0028", so I read this as conforming, but the order within a spec is not specified.

### Is each found directory the newest for its flag?
Yes, all 14, by runs-listing.tsv's started column:

| Flag | Listing line | Other directories for the flag (all older) |
| --- | --- | --- |
| comment | 9, 2026-10-02T07:23:17Z | four from 2026-10-01 |
| image | 8, 2026-10-02T07:15:50Z | two from 2026-10-01 |
| richtext | 19 | 2026-10-02-7d2f40d |
| richcomment | 18 | dc33002, 03:37:30Z |
| sheetview | 14, 20:34:16Z | 19:23:47Z and 12:21:53Z |
| pagesetup | 12, 20:40:11Z | 19:29:43Z and 14:20:27Z |
| group | 30 | 2026-10-04 |
| linestyle | 31 | 2026-10-04 |
| read | 26, 19:53:22Z | 17:10:14Z |
| roundtrip | 27, 19:48:16Z | 14:58:13Z |
| anchor, commentbox, copy, remove-image | 44-47, 2026-10-09T14:xx | older 2026-10-06 to 2026-10-08 directories |

- The log's own survey (run.log 2239-2285) agrees with the listing row for row: 45 directories, 0 differences in flag or started time.
- R2 asserts each found directory equals the expected one (run.log 3417-3460).
- **Note:** the listing holds one directory the survey cannot see, 2026-10-03-01b14c0-b-sheetview (runs-listing.tsv 11). It has no run.host.tsv, tool_args "-" and a 3-row summary. The survey counts only directories with a run.host.tsv (run.log 2239), and the log prints no count of directories without one. It does not change the newest choice.
- **Note:** for edit-formula, the listing's started time is 16:44:47Z and the log's is 16:44:46Z (run.log 2358). The listing reads the runner's clock and the log reads the script's. This has no consequence.

### printarea and P4 ruling 2 (spec-0029-working.md 717-724)
Matches.
- It was rerun although 3 directories existed, each with "error 6 ok 21" (run.log 3362-3367).
- The rerun has 6 rows not ok: run.log 3397 and 3402, and the record's host row (records/m3p7-p5-printarea-2026-10-09-f39bc7b.md, "Files by outcome").
- The finding line is printed at run.log 3404, and printarea is absent from R4 (run.log 3509-3544).
- Naming the count in the Excel record's header is still to come.

## Q2. Does the diff do what the spec says for P5, and nothing else?

**Verdict: yes with one reservation.**

### Code: commit-f39bc7b.diff
- README.md alone, two one-line changes, at lines 61 and 65 (diff lines 5-16).
- Line 61 reads exactly "Of the 1093 output files the crate's and the copy tool's tests keep, LibreOffice opened 1082; the record's own sections explain the eleven it did not", with the same citation.
- Line 65 gains exactly ", the largest tier's figures including the raw-copy samples, whose rewriting samples ran at 1.8 to 3.7 MB/s" after "on the static target" and before the citation.
- Both match P4 verification ruling 2 (spec-0029-working.md 783-791).
- This is the nothing-else read for code. The gate confirms it at gate.log 3541 (name-status M README.md only), 3547 (hunks at 61 and 65) and 3556-3557 (no .rs, no Cargo.lock).

### Doc: commit-515c3e2.diff
- ROADMAP.md: the P4 row becomes DONE and a P5 row is added.
- specs/0029: lines added only.
- Both are planning's files, at the hashes the brief gave (gate.log 3425-3467).

### Combined: phase.diff
phase.diff is exactly the code diff (phase.diff 1-18) followed by the doc diff (phase.diff 19-225). I compared both byte for byte.

### Working spec
spec-0029-working.md differs from the committed blob only by the uncommitted "Planning rulings during P5" section (spec-0029-working.md 817-835). The doc hunk's added text matches the working copy up to that heading. Its sha256 is 6ba07ae5a568e71cf8e25ade114c2a4fcdf0ed43b98f0e32130b1a6137eef484.

### No code commit beyond the one the ruling names
The spec says "no code commit unless the ruling names one" (spec-0029-working.md 338). Ruling 2 names exactly this one. The list script is unchanged: sha256 f5c6dc161eb24f25f91a61d4b9938d002491f10f0cf87c1e510c3663cac9b5f9 at negative.log 1861 and run.log 3508, equal to excel-check-list.py here.

### Reservation 3: the records do not say the reruns are closing-commit measurements
P4 ruling 2 (spec-0029-working.md 715-716) says the reruns "are the harness's measurements at the closing commit, not repeats of the old ones, and their records say so". The 23 rerun records name "Commit: f39bc7b..." (for example records/m3p7-p5-formula-2026-10-09-f39bc7b.md 6). No record carries a sentence saying they are measurements at the closing commit and not repeats: grep finds 0 records with such wording. RERUN_NOTE is unset (run.log 4). Planning should say whether the commit line is enough.

## Q3. Do the logs contain every line the spec requires?

**Verdict: yes.**

### Gate
| Required line | Where |
| --- | --- |
| Environment line as the first section after the header and input lines | gate.log 11-25 |
| Shape greps | gate.log 27-37 |
| P4's prune: 9 names, du -sb each, total 4210677269 | gate.log 39-84 |
| rm -r with no -f, each name gone, kept names present, controls | gate.log 85-98 |
| Spec pin dad50b9c... | gate.log 100-108 |
| README hygiene: 4954 bytes, U+2014 0, control 0, non-ASCII 0, UTF-8 yes, with controls | gate.log 3564-3570 |
| Ten gate files checked, 0 in the diff, no self-test | gate.log 3524-3536 |
| Nine PASS lines | gate.log 3958-3969 |
| Doc rule: all.html written again after the removal | gate.log 3972-3973 |
| "Documenting emend" seen in each crate doc build | gate.log 4012-4049 |
| links 125, section anchors 5, item links 120 | gate.log 4053 |
| Final line | gate.log 4234 |

### Negative log
| Required line | Where |
| --- | --- |
| Messages read from the source, not typed | negative.log 2259-2265 |
| L1, missing summary: exit 2, no list | negative.log 2428-2435 |
| L2, `--run-plain` under another root: exit 2, no list | negative.log 2437-2444 |
| L3, a row not ok: exit 2, no list | negative.log 2446-2453 |
| LC control: exit 0, mode 600, 4/2/2 rows | negative.log 2413-2426 |
| Final line | negative.log 2471 |

### Run log
| Required line | Where |
| --- | --- |
| Per-flag line: spec, flag, rerun or found, expected directory | run.log 2155-2191 |
| Per-flag spec and directory in R4 | run.log 3510-3544 |
| Each rerun's directory | R1 headers, from run.log 2305 to 3361 |
| Stage path | run.log 3553 and 3562-3573 |
| Rows by source and total: corpus 972, fixture 1081, input 1081, total 3134 | run.log 3554 |
| List sha256 aabf420a..., "no row printed" | run.log 3553 |
| Zip sha256, bytes and mode | run.log 3573 |
| README check: program hash f87526a3... equal to P4's | run.log 3578-3581 |
| README check summary, and the control refused | run.log 3583-3586 |
| The two changed lines | run.log 3587-3592 |
| Final line | run.log 3712 |

### specs/0001 rules in every log
- Each log opens with the shape greps (gate.log 27, negative.log 10, run.log 11, scan-pre-check.log 13).
- RERUN_NOTE is unset in all four (gate.log 4, negative.log 3, run.log 4, scan-pre-check.log 5). ANTHROPIC_API_KEY is unset.
- No non-ASCII or control byte in any log, and no absolute path outside printed patterns.
- Every stderr file is 0 bytes. logged.exit is 0.
- The scan pre-check ends with "structural checks failed: 0" and "RESULT: PASS" (scan-pre-check.log 3616-3618).

**Note:** run.log prints 621 per-workbook runner rows (27 per rerun), for example run.log 2311-2337. These are runner output, not list rows, so "never a row" is not breached. I have not quoted them.

## Q4. Does every number and quoted string in report-draft.md appear in a primary?

**Verdict: yes with reservations.**

### Verified
- Every log hash, record hash, binary hash, list hash, zip hash and spec hash.
- Every count: 4210677269, 4954, ten, nine, 125/5/120, 1112, 1093, 1081, 1082, 972/1081/1081/3134, 3136/3134, 604025974, 36/18/18/0/0, 6 and 21, 24 pairs.
- Every directory name, and the times 16:42 and 18:53Z.
- The counts line, quoted exactly.
- "the gate builds no release binary": gate.log has 0 lines containing --release.
- specs/0016 lines 365 to 366.

### Without a primary in the directory, or not exactly as the primary shows
1. **"the doc rule as P4's (links 125, section anchors 5, item links 120)":** the three numbers are at gate.log 4053. Nothing in the directory shows P4's figures, so "as P4's" can only mean the same rule.
2. **R0's "the dry run's R0 had built them at the same commit into the same target, from the same sources":** no primary. The dry logs are not in the directory.
3. **The dry-chain paragraph:** no primary for any of it. That covers <phase>/superseded/dry-1, <phase>/dry-*, <home>/.cache/emend-build/m3p1-p3/keep-host and records/m3p1-open-2026-10-02-6578925.txt.
4. **"Each of the 22 other than printarea: ... 27 outputs and 27 inputs opened by LibreOffice":** this is not in run.log. Its primary is the 22 records' "Files by outcome" host rows (lo ok 27, lo_in ok 27), which I confirmed.
5. **"comment ... and image ... (specs/0023's reruns of 0022's samples)":** specs/0023 has no "Harness runs" section. The reruns are named in its P3 (specs/0023 237-241) at commit 6578925 (specs/0023 552). This is supported but not from a "Harness runs" section.
