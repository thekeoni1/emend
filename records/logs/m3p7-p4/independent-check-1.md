# Independent check 1, M3 part 7 P4

Written by the implementing session. Checked commit (the phase commit): ad7cfecced2d06f18c57cbf5da6908860dad19bd; HEAD when the check ran: ad7cfecced2d06f18c57cbf5da6908860dad19bd. The checker was a fresh general-purpose subagent given only the directory <phase>/check1/inputs (<phase> stands for <home>/.cache/emend-build/m3p7-p4), read-only, never the conversation. Its inputs, by sha256 (sha256sum over each file, paths relative to that directory):

    842bcb9dad8940a19820282d3a0adf610b4c8fe580681b8060a48ac8e8a66d70  ./CLAUDE.md
    3dd6a511a3cc5f84114c6e013d74afcb3d2de35da1bc2b498a037acbd0728bf9  ./README.md
    60a379312a5681cba0c1acfb128402c95fa1ec753158d1b29962bce2320e742c  ./close/carried/counting-m3p6close.sh
    5c1880bc84a0cc05e080272e4168e03521836eb23de3ec4058872e7dbd0c4812  ./close/carried/counting.sh
    9652a473a0d2eb99a1b4197770bd23ce61c9f9b73a0b572f34747f08feb33231  ./close/carried/m3p6close-ci.sh
    400f6015411985adfc54e062e7f00ad057d80fa42565c76c1370f61f3f66ee33  ./close/carried/m3p6close-push.sh
    3091e26b6c1f404c33eb0f7eb005a74a6d556c7d002b22d238d02c2e9fc7cb42  ./close/carried/m3p6close-scan.sh
    9652a473a0d2eb99a1b4197770bd23ce61c9f9b73a0b572f34747f08feb33231  ./close/carried/printed-m3p6close-ci.sh
    400f6015411985adfc54e062e7f00ad057d80fa42565c76c1370f61f3f66ee33  ./close/carried/printed-m3p6close-push.sh
    3091e26b6c1f404c33eb0f7eb005a74a6d556c7d002b22d238d02c2e9fc7cb42  ./close/carried/printed-m3p6close-scan.sh
    a2a262584cc0b3a645d7f7ad963a50301f910c34601fd79d3b59a19463d6c558  ./close/gen.py
    c3feb170ae49a2900b24f3620fb20bdd3e7a0b20fea4665b3e3647ab3408f66f  ./close/m3p7close-ci.sh
    1d0807d5c0ceb9cec52d0b2918eb3a52689f3a27dddf0f1baf4fc6ad1667e98a  ./close/m3p7close-push.sh
    08cb5afa54cfee2d3597128288f70795a0a1617122dea70c3ffeab186afb35a1  ./close/m3p7close-scan.sh
    9ad8a972320076eeab220ff225583219e45f822ee1cce39dc16e868625683f37  ./commit-2f05914.diff
    29c92edef05a6b7f1cfbb3d51c1b92736ae69f132c3ec5ca8cd9413390625910  ./commit-ad7cfec.diff
    c49564d96943a6bb318200c2ad31c91dd6eb132736a454b2b5a97a89fe864e58  ./commit-messages.txt
    f5df4edad199643dee2391e2f54aee87d4dec12f07b1095c61661a431309a51f  ./crates-emend-Cargo.toml
    c4b2d963869bef2e2960d8f1b1a13b9f8a3766e569b7267fd86cb673a6af7e24  ./gate.log
    9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa  ./logged.exit
    298d2c8e81b3725fcd26a8085c3ea88fcc575397ac38c41e577908255388e550  ./logged.out
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./logged.stderr
    ea9d7b7d36ce8259d79829a00827f63014c88432885fb6d0f9e3aee2f7acec61  ./negative.log
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./out-gate.outer.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./out-gate.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./out-negative.outer.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./out-run.outer.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./out-test.outer.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./out-test.stderr
    e7bb8a20b27c5a1660d472a681d3940a0e8c2b8d9262574219ba2441b8b5ae8c  ./phase.diff
    f87526a34e00242c582db02da8eaee92b6c6dd944253e0679fa82d0e67f25dbc  ./readme_check.py
    71a8c57878f151e2a460cf7dc37f30f70af8972644f98aa01367f13c2421fa70  ./records/api-review-m3p7-2026-10-09-09d2e0f.md
    89d3d9c4abbb2ca7af708e2fdb0a1685b9b4d9540fcc80bac618b58be94a2d32  ./records/excel-check-template.md
    f9f564cbc230c19a14a170302665ac0d3553338d98de3c8cf120fb69c423819c  ./records/m3p6-open-2026-10-08-9cfd92e.md
    1e06e59a73716087d30679230a528b2fc0e933c9ceb8788d072b4735f14c0acc  ./records/m3p6-p4-anchor-2026-10-08-9cfd92e.md
    6a73d57022c232fb2eb6d120f83ae8ca771716d8ba824eb8a575bbb68549daa6  ./records/m3p6-p4-commentbox-2026-10-08-9cfd92e.md
    86670a673839343219bfa83a68fd16de3d21b67e155505703aa0e77e3336311e  ./records/m3p6-p4-copy-2026-10-08-9cfd92e.md
    f93d9a97e7268b5fbf55fa262ad2ed457ab50cddde9ef7ccdb326129f4919759  ./records/m3p6-p4-removeimage-2026-10-08-9cfd92e.md
    0db5d43b5f3ed6164786adff13586a06b6f56e29789934516fe7dbfe098a9e49  ./records/scale-m3p1-2026-10-02-6578925.md
    30e1a5dcb24616469e4200f4d860e1f3b9fd5839c0ff7cc1bfc45b1bae713130  ./records/size-m3p7-2026-10-09-09d2e0f.md
    5a6834de5c46c10ae9d6bde992aa736659008c67985112788be11ff14d1ea013  ./records/toolchain-2026-09-15.md
    4ff3d8a12f8fd88f475cbfc6f6fdc1396090f61a9ec5b4eca605a48028590b65  ./report-draft.md
    e55ffaf4ee8d06d06361fc4889656df0ad1926eec65d1dfe050aa25256dfc2aa  ./run.log
    ad1fdce66c79910e8c1d5e1851c5917e9134d8f25d9bf781d7f1fb37a967d627  ./scan-pre-check.log
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./scan-pre-check.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./scripts-out-negative.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./scripts-out-run.stderr
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  ./scripts-out-scan-pre-check.stderr
    2ddd2b06105267673097508662375f7053e4b63016e1b340b6d4589385c8256a  ./spec-0029-at-phase.md
    3d6390e760e5568aec5590e1a8b7e469f28e46b7316b4a474dad78d1d1884617  ./spec-0029-working.md
    fb3f9c9a68e7e05c99df428a48d9972560b071875f32f40519b7d04196f47028  ./specs-0001-at-phase.md
    244afbf583841061ea57e1721d38bfcf4e0acf3cdbde624baf286effff365bd5  ./test.log

The answer below is the checker's final message, extracted from its transcript into <phase>/check1/check1-answer.raw, verbatim (sha256 af3583f0c9165fd828c76fdd7356db3c28f789e5159d33cfbe79d78f31b95279); it holds no non-ASCII byte and no absolute machine path, so nothing in it is replaced, and the text below the rule is byte for byte that file (sha256 af3583f0c9165fd828c76fdd7356db3c28f789e5159d33cfbe79d78f31b95279).

---

Context note: the harness put CLAUDE.md, memory and git status into my prompt. I did not use any of it as evidence. Everything below comes from <phase>/check1/inputs. I wrote nothing to disk. I ran a `diff ... > /dev/fd/3` once and it failed with "No such file or directory", so it created no file.

## Q1. Does the diff do what the spec says for P4 and nothing else?

**Verdict: yes, with reservations.**

**Diffs read**
- commit-ad7cfec.diff (the code commit):
  - It adds only README.md (diff line 1 to 82, "new file mode 100644").
  - The README's added lines are byte-identical to README.md, whose git blob hash I computed as 3d9eea6, the diff's index line 3.
  - Nothing else: confirmed from this diff, and the gate agrees (gate.log 3537 to 3542: "A README.md", "adds README.md and nothing else"; 3545 to 3547: 0 Rust sources, 0 Cargo.lock).
- commit-2f05914.diff (the doc commit) touches three files:
  - ROADMAP.md: 3 table rows, P1 to P3 PENDING changed to DONE (diff lines 5 to 17).
  - records/api-review: lines added only, under "## Corrections (2026-10-09)" (diff lines 22 to 33). The three items use the words of P3 verification ruling 4 (spec-0029-working.md 645 to 655).
  - specs/0029: 117 lines added at 532 (diff line 38).
- Post-image hashes I computed:
  - spec-0029-at-phase.md: blob b2bf9e4, sha256 2ddd2b06..., which matches the doc diff's index line.
  - api-review record: blob ced9f38, sha256 71a8c578..., also matching its index line.
  - I cannot check the ROADMAP.md post-image beyond its 3 diff lines; its hash 105a0827... comes only from gate.log 3416 and 3473.
- phase.diff is exactly commit-ad7cfec.diff followed by commit-2f05914.diff (checked by string concatenation, True). So for "nothing else" on the whole phase I read the two per-commit diffs. phase.diff adds nothing beyond them.

**README.md against "The README (decision 4)" A and P4 ruling 1: yes, with reservations.**
- **What emend is:** README 3 to 13 follows CLAUDE.md 9 to 15 and line 24. "NOT" is lowercased. All four exclusions are there.
- **Dependencies and features:** README 17 names the two dependencies. README 23 to 33 list 11 features, each with a specs/ path. That matches crates-emend-Cargo.toml [features], which has 11 entries, all `= []`, no defaults.
- **Build and test:** README 35 to 51 cover both targets.
- **Three measured sentences:** README 57, 59, 61, 63 and 65, each citing its record.
- **Out of scope:** README 67 to 71. **Licence:** README 73 to 76.
- **Ruling 1's sources are used:** the four "Files by outcome" host rows (README 57), the copy record's host total 917/784 (README 59), and the open record's totals 1093/1082 (README 61).

Reservations on the README:
- **(a) README 61, "the 1093 files the crate's own tests write":** the open record's Pairs table holds 992 `package.*` rows, 100 `emend-copy.*` rows and 1 `cells.*` row. So 100 fixtures come from the emend-copy tool's tests, not the crate's.
- **(b) README 61, "LibreOffice opened 1082":** the figure is the record's "lo ok". Of the 11 "lo fail" rows, 3 are reason "count" (records/m3p6-open-2026-10-08-9cfd92e.md 16 to 19 and rows 259, 262, 263), not refusals. Ruling 1 asks that "the sentence names what the figures are". This naming is loose, though the figures themselves are right.
- **(c) README 59:** the 917 entries include 47 "added" output-only entries and 5 "dropped" ones (the copy record's host total row). Ruling 1 words it the same way, so I accept it, but the denominator is not "entries in the inputs".
- **(d) README 65, the 4.8 MB/s upper bound:** the scale record's Throughput note says tier c's figure "includes the raw-copy samples; its rewriting samples run at 1.8 to 3.7 MB/s". The README leaves that caveat out.
- **(e) Claims the inputs cannot confirm:**
  - That `LICENSE-MIT` and `LICENSE-APACHE` exist at the phase commit (README 75 to 76). There is no tree listing in the inputs; only specs-0001 historic P2 rulings mention them.
  - That the build commands at README 49 to 56 match `scripts/check.sh`. check.sh is not in the inputs. gate.log 3958 to 3966 shows only its PASS lines.

**The close-out scripts' change against "The Excel check" and P4 rulings 3 and 4: yes, with reservations.**
- **Change 20 does what ruling 3 says** (close/m3p7close-push.sh 423 to 445; the header is 20 to 29):
  - exactly one `^records/excel-check-m2-[^/]*[.]md$` at HEAD (lines 425 to 427);
  - a "## Results" line and at least one row whose cell under the "result" header reads pass, repair or fail, with "(by hand)" allowed after the word (lines 430 to 440);
  - HEAD's message names the path exactly once (lines 443 to 444).
- **Pre-mode call and control** (lines 545 to 555): the check, then a clone with the record removed by `git rm` and an `--allow-empty` commit, refused through `excel_record_refused`.
- **Ruling 4:**
  - All three scripts are generated by gen.py from part 6's. gen.py and the generated scripts hash as test.log 2024 and 2229 to 2234 say.
  - The carried copies hash as T1 says (test.log 1989 to 2002).
  - The scan's TIP_PATTERN is the amended close-out subject (m3p7close-scan.sh 227; test.log 2516).
  - Push BASE_PATTERN and PREV_RECORD_PATTERN name part 6 (m3p7close-push.sh 288 to 289).
  - My own `diff` of each generated script against carried/m3p6close-*.sh is byte-identical to the diffs printed at test.log 2243 to 2375, 2379 to 2401 and 2405 to 2447. My counts match too: push 133/10/105, ci 23/4/12, scan 43/7/21.

Reservations on the scripts:
- **(f) The control only covers one condition.** The pre-mode control refuses because zero files match, so in pre mode it never exercises the message branch or the result-row branch. negative.log N5 to N7 do exercise those branches.
- **(g) The check reads whatever HEAD is.** The push script does not check that HEAD's subject is the close-out doc commit's subject. It relies on "HEAD is main" (line 503) and on the scan's TIP_PATTERN. "The Excel check" says "named in the close-out doc commit's message". That holds only if pre mode runs with HEAD at that commit.

## Q2. Do the logs contain every line the spec requires?

**Verdict: yes, with reservations.**

**The doc rule with its control (P3 verification ruling 2)**
- The doc directory is removed before every doc build, the full gate included: gate.log 3950 to 3955, then 4004 to 4007, 4013 to 4016, 4022 to 4025, 4031 to 4034 and 4040 to 4043.
- "Documenting emend" is asserted for each of the five crate builds: gate.log 4011 to 4012, 4020 to 4021, 4029 to 4030, 4038 to 4039 and 4047 to 4048.
- The controls on that assertion: gate.log 4049 to 4050. The control on the removal check: gate.log 3951.
- **Reservation:** for check.sh's own doc step there is no "Documenting" line. The log asserts instead that all.html was written again after the removal (gate.log 3971 to 3973, with a control). The report discloses this at report-draft.md 23.

**The index count (P3 verification ruling 3):** gate.log 4051 to 4061. "links: 125; section anchors: 5; item links: 120", with two planted-index controls.

**README hygiene**
- Gate: gate.log 3555 to 3562 (U+2014, control bytes, non-ASCII, UTF-8, each with a control) and 3605 to 3611.
- Run log: run.log 2131 to 2151 (U+2014, ASCII, control bytes, absolute path, corpus directory name, denylist, each with a control).
- **Reservation:** P4 says "the README counts as a documentation file for hygiene". Whether check.sh's own hygiene step covers README.md cannot be seen. The gate log's own scoring stands in for it.

**The prune:** gate.log 39 to 102. The 9 names are listed with `du -sb` (6597483554 bytes in all), each removed with `rm -r`, checked gone, the kept names checked, with controls.

**The gate-file listing:** gate.log 3491 to 3535. 10 files checked, 0 in the diff, no self-test log, with controls.

**The test log's diffs against part 6's:** test.log 2240 to 2448, in full. My recomputation shows them identical (see Q1).

**The negative log**
- Planted uncited sentence refused: N1 at negative.log 1867 to 1889 (exit 1, "UNCITED unit 31"). Control NC at 1845 to 1866. N2 (missing path) at 1890 to 1914.
- Ruling 5:
  - The printed range is "423 to 446" (negative.log 1923). I confirmed it in close/m3p7close-push.sh with the same two greps.
  - The extract's sha256 is 28cfad54... (1926). My `sed -n 423,446p | sha256sum` gives the same, and the log shows it equal to the script's lines with a control (1927 to 1928).
  - Refused at the current tip: N3 (2031 to 2038).
  - Accepted with a planted record in the template's form and a message naming it: N4 (2039 to 2047). The planted head equals records/excel-check-template.md lines 1 to 54, which I checked by cmp.

**The run log**
- README check: run.log 1963 to 1991, with the planted control at 1987 to 1989.
- Each figure read by a printed command: run.log 1992 to 2101, each with a value-changed control.
- Number tokens equal to the 26 figures: 2103 to 2108. Features equal to the manifest's: 2110 to 2129.

**specs/0001 rules that bind every log:** all present, with these reservations:
- Present in all logs: shape greps first, phase commit compared with HEAD with controls, script printed, own stderr counted, roots declared.
- Present in the scan: control bytes, home path, denylist, three shapes over printed scripts, bare-shape control, rm -rf form and placeholders (scan-pre-check.log 2586 to 2948, "RESULT: PASS").
- The log hashes the scan prints equal my sha256sum of each log.
- No log contains a non-ASCII byte or an ESC byte (my grep).
- **Reservation (minor):** the gate and test logs' "fabricated commit" controls use the null commit and a short sha. The negative and run logs also use a one-digit-changed sha.

## Q3. Does every number and quoted string in the draft report appear in a primary?

**Verdict: yes, with reservations.** Everything checks out except the following:
- **report-draft.md 21:** "20999b4c0f5e72279a6c61716f001bac696e63c1", "same tree", the first dry run refusing it, and "Nothing outside this session had referenced 20999b4". None of this appears in any primary in the directory.
- **report-draft.md 22:** the dry-run defects, and "<phase>/superseded/dry-1 and dry-2". No primary. There is indirect support only: test.log 2537's "(change HI)" control, test.log 2501's fixture parse-error count, and negative.log's `<source>` root line "(change ZE)".
- **report-draft.md 23:** "discards its output". This rests on the gate script's own comment (gate.log 1627, "the full gate prints nothing of its doc step") and on check.sh output that shows only PASS lines. check.sh itself is not in the inputs.
- **report-draft.md 17:** "final stage after this check". That is a future event, not checkable now.
- **Verified:**
  - Hashes: 3d6390e7..., 99dca446..., 2ddd2b06..., 105a0827..., and all five log hashes, including ad1fdce6... for the scan.
  - Gate figures: 6597483554, "nine", 4812 bytes, "ten checked", nine PASS lines, five doc builds, 125/5/120.
  - Diff counts: 133/10/105, 23/4/12, 43/7/21.
  - Run and negative figures: 36/18/18/0/0, 26 figures, eleven features, N1 to N7 outcomes.
  - Other: line 158, logged.out's four exit-0 lines, logged.exit 0, the four small files.

## Q4. Independent recomputation of the README's figures

**Verdict: yes, every figure matches.**

**Size record** (records/size-m3p7-2026-10-09-09d2e0f.md), "The figures" table, lines 176 to 189:

| Feature or figure | README figure | Recomputed from the probe table (lines 40 to 67) |
|---|---|---|
| dates | 4840 | 928448 - 923608 |
| rowcol | 58152 | 1028360 - 970208 |
| metadata | 50036 | 1020244 - 970208 |
| rules | 48468 | 1018676 - 970208 |
| tables | 46536 | 1016744 - 970208 |
| comments | 63356 | 1033564 - 970208 |
| images | 67832 | 1038040 - 970208 |
| richtext | 13384 | 983592 - 970208 |
| sheetprops | 28284 | 998492 - 970208 |
| read | 51988 | 1022196 - 970208 |
| copy | 13820 | 984028 - 970208 |
| budget | 436992 | 970208 - 533216 |

Record lines 215 and 220 give the ceiling 458752 and "21760 under the ceiling". 458752 - 436992 = 21760.

**Four sample records**, Files by outcome host rows (line 17 of each): files 27, ok 27, lo ok 27, lo_in ok 27.

**Copy record**, host total row: entries 917, identical 784.

**Open record** lines 8 and 14: pairs 1093, lo ok 1082.

**Scale record:** "Every streaming sample peaks between 7400 and 8004 kB ... tier c's 5025024826-byte sheet entry included." The table's minimum is 7400 (style i686, tier a) and its maximum 8004 (props i686, tier a). Throughput host 2.9/2.9/3.3/4.8 and i686 1.9/1.9/2.2/3.2.

**readme_check.py runs**
- With a listing built from the paths the README cites (`grep -o` of its records/ paths, via process substitution): "sentences 36 with-digit 18 cited 18 uncited 0 missing 0", exit 0.
- With a listing of the records/ files in the inputs: same result, exit 0.
- Control with an empty listing: "cited 0 uncited 0 missing 21", exit 1.

**Are these the records the phase commit holds?**
- **Known for 7 records:** run.log 1952 to 1958 prints sha256 values read by `git show <phase commit>:<path>` for the seven cited records with figures. Each equals my sha256sum of the copy in records/.
- **Known for the API review record:** gate.log 3420 and 3476 give 71a8c578... at the phase commit, which equals my hash.
- **Partly known for the Excel template:** lines 1 to 54 equal the text negative.log printed from the phase commit. Its remainder is not tied.
- **Not known for records/toolchain-2026-09-15.md:** no log hashes it. Its path is only shown to exist in the phase commit's tree, through the run log's README check. I cannot say this copy is the phase commit's.
