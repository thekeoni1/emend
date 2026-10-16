Independent check 1

Inputs read, all under <phase>/check1/:

| File | sha256 |
|---|---|
| spec-committed.md | 931b7e5cfe63bcb407657b8f474fa9b719dd9d3870a974bb33237dc80d0a237b |
| spec-working.md | 7a2bffe1c5260f4d02864877411b4c688a3db1fbfc89c17fcb24af8570abcc2d |
| diff.patch | 515077bb9b7fe0a60927c6945eb1e17f61b10b187920cfa7826c0903ff3e6c92 |
| rewrite.patch | 11818a8fb971456ab7970899b3c0a6fa1778552fda88917d0078058acf526bf7 |
| commit-messages.txt | 1311be5574bd89fd8fdeafe8151ce8f4134afd08913aefa316a27ff3340812bc |
| logs/gate.log | 57a828bd75c2b98fa310a8a4d6d1b61d25416e745a2b998717c6be072fce3e55 |
| logs/test.log | b1d3668903331c8e84fd777a4c92eef9a86af06ea12d31a97e66d7b6837fb503 |
| logs/negative.log | 047a453de09ca87fbbe864615cd48ec3c1a669d20c968ab6d01da878743fab8d |
| logs/scan-pre-check.log | 29ce2f63dd022197631b8605ddabd1277cc9a0f7cb283fb2c17bef03fca38ebe |
| records/openpyxl-image-2026-10-16.md | a28d721d1baef7839ca457bd3b4326a6e14989202a43d5d25b462ed3c702727d |
| superseded/openpyxl-image-2026-10-16.md | d476e5322dd6eba7bbebc391f1fafc2e6ad8748e48059f1553e98d1c4bfba495 |
| superseded/scan-pre-check-failing.log | 7f908a60b4a458df3879f4639674214cacba33f0531295141534d373108b61bc |
| report-draft.md | 2a047a63d8742a325304a722ac5b078aa6227ef09ccd745fb231f7c285bda074 |

spec-working.md is spec-committed.md plus one inserted block, "Planning rulings during P1" A to G (working lines 372-450). `diff` shows nothing else changed.

## 1. Does the diff do what the spec says for P1, and nothing else? YES WITH NOTES

**Scope.** diff.patch (lines 1, 65, 347, 517, 680, 853, 926, 1334, 1361, 1365) touches exactly these 10 files:
- the doc commit's three files: ROADMAP.md, specs/0036 and specs/0037;
- tools/openpyxl-image/Containerfile;
- scripts/openpyxl-ops.py and scripts/openpyxl-ops-test.sh;
- tests/fixtures/openpyxl-ops.xlsx and tests/fixtures/README.md;
- scripts/compare-run.sh and scripts/passthrough-record.py.

No crate source, no Cargo.toml and no Cargo.lock appear (gate.log 6344). The specs/0037 hunk, reconstructed from diff.patch lines 926-1333, hashes to 931b7e5c..., the same file as spec-committed.md. gate.log 106, 124 and 4300-4304 pin specs/0036 at 8f85b6ca and ROADMAP.md at a86114da.

**Containerfile** (diff.patch 1365-1387; reconstructed sha256 6c09db87..., git blob 5cbbb5e matching the index line). It matches spec lines 79-86:
- FROM the base digest abc9cb88...;
- `python3-openpyxl=3.1.5+dfsg-2 python3-pil` with `--no-install-recommends`;
- the `dpkg-query` package list written to /py-packages.txt;
- the apt lists removed;
- `ENV HOME=/tmp` and `WORKDIR /work`;
- no ENTRYPOINT.

**openpyxl-ops.py** (diff.patch 517-679; the blob matches 5b37d51; sha256 3d56b34c..., the same value the test.log R1 Binaries line expects).
- The eleven ops match the table at spec 117-129. W is `wb.worksheets[0]` and M is `max_row`.
- Every constant is named in the docstring (553-570), and the docstring has no corpus content.
- An unknown op or a wrong argument count gives "unknown op" and exit 2 (645-647).
- An exception prints the class name only and exits 1 (655-658). MemoryError exits 137 (651-654).
- The last line is `openpyxl=%s\tpeak_rss_kb=%d` from `ru_maxrss` (639-641).

**compare-run.sh** (diff.patch 65-346).
- Arguments match spec 151-153.
- Each container is named `emend-openpyxl-$$-$id` (ruling A, line 251).
- The run uses `--network=none --userns=keep-id`, the input read-only at `/in/$id.$ext` and the output at `/out/$id.$ext` (ruling D, lines 255-258), the ops script read-only, `ulimit -v MiB*1024`, and `timeout --signal=KILL`.
- After exit 124 the runner calls `podman rm -f --time 0` and writes its exit to rm-exit (lines 266-271).
- Outcomes follow ruling B (lines 273-280). lo_in is "-" (ruling C, line 309).
- The header of run.openpyxl.tsv follows ruling E (line 112).
- At the end the runner counts the image's containers and fails if any remain (lines 341-346).
- The default run directory is `runs/<date>-<short>-openpyxl-<op>` (line 180).

**passthrough-record.py** (diff.patch 680-852) adds:
- the label openpyxl and its own outcome set (ruling B);
- `OPENPYXL_RUN_HEADER`, matching ruling E's list;
- the Binaries line naming the image id, the ops script's sha256 and xlsx-diff's;
- a refusal when the run rows name different ops.

The emend path is unchanged in effect: T4 R6 and R7 show the old and new scripts give the same record apart from the "Record written" line.

**Fixture.** tests/fixtures/README.md (1334-1360) names the tool, the command, 29191 bytes and sha256 12ec7ca0... (ruling F). The ops test regenerates tier t and asserts the sha256 and a `cmp` against the committed file (diff.patch 397-403).

**rewrite.patch is exactly the ruling G fix.**
- It touches scripts/openpyxl-ops-test.sh alone: 4 insertions and 2 deletions (lines 9-11 and 19-21).
- The two chained `yn [ ] && [ ]` checks become the named helpers `gen_ok` and `unknown_ok`. Nothing else changes.
- The `<dir>` root line ruling G asks for is in the test log driver, not in a committed file: test.log line 51.
- Ruling G says the runner commit is cherry-picked with the same message. That is consistent with this patch: the runner files are byte-identical between 71aaaee and 1e29c4e, and the superseded scan (lines 49 and 134) shows ops commit 05e4b25 under the same subject.

Notes:
- (a) Spec line 160 says "the log prints their byte counts" (the openpyxl stderr files). compare-run.sh prints no stderr byte count. I read this as the P2 run log's duty, not the runner's, but the point is open.
- (b) Ruling A says "at the end of every run the runner counts the image's containers". A `die` inside the loop (for example, lines 263 or 271) exits before the count at lines 341-346, so a run that dies early never counts. Every logged run reached the count.
- (c) The record title "# openpyxl comparison run" (diff.patch 807) is not named by ruling E. It is harmless and is tested by T5 P6.

## 2. Do the logs contain every line the spec and rulings require for P1? YES

**The image record** (records/ lines 1-36; test.log 2278-2420) holds everything spec 92-96 asks for:
- the base digest (line 13);
- the package pin (line 15);
- all eight /py-packages.txt lines (25-32);
- `openpyxl.__version__` 3.1.5 (line 17);
- image id 894b239f... and 211283030 bytes (10-11);
- the Containerfile's sha256 6c09db87 (14);
- podman version 5.7.0, rootless (19).

`diff` against the superseded record gives 4 lines: only line 3's phase commit changed (71aaaee to 1e29c4e). The image id is the same in both, as ruling G requires.

**The ops test is run by the test log and not the gate.**
- test.log 2422-2505: T2 runs scripts/openpyxl-ops-test.sh in the image. It ends "openpyxl-ops-test: PASS", and line 2504 reads "76 ok lines, no FAIL, the last line PASS". I recounted the script's checks: 4 + 44 + 2 + 10 + 1 + 12 + 1 + 2 = 76.
- gate.log 6389: "this script runs neither the ops test, scripts/openpyxl-ops-test.sh, nor the container tool (change E63): as-expected".

**The runner's and the record script's tests.**
- T3 (test.log 2507-3112) runs all eleven ops (C-roundtrip to C-copy) on xlsx and xltx inputs, then the controls, refusals R-op to R-missing, and C-ids.
- T4 (3114-3275) runs R1 to R8.
- T5 (3277-3381) runs plants P1 to P6, each caught.
- Container counts are 0 at entry (2273-2276) and at the end (3383-3385).
- The only line ending ": no" in test.log is line 55, "ANTHROPIC_API_KEY set in this shell: no", which is the expected answer.

**Negative cases N1 to N6** (negative.log), as the spec lists them:

| Case | What the log shows | Lines |
|---|---|---|
| N1 | the ops script exits 2 with "unknown op", and the runner exits 2 with no run directory | 2699-2712 |
| N2 | prints ValueError alone, exit 1, the marker in no file of the run directory | 2714-2732 |
| N3 | MemoryError, cap 137 | 2734-2748 |
| N4 | tier a, 50000 rows read from the source, under --memory 64 ends as cap 137 | 2750-2770 |
| N4c | the control: --memory 4096 gives ok | 2750-2770 |
| N5 | --timeout 1 gives timeout, 124, signal 9 | 2772-2795 |
| N6 | the runner prints no path, and the tables hold none | 2797-2810 |

N5 also shows the removal's exit 0 and the image's containers 0 before and after. Its control, N5c, counts its own container 1 and then 0 (ruling A). N6's control shows the planted path stays in stderr.openpyxl.txt. The log ends at line 2827: "negative log checks all as expected: yes".

**The scan's PASS.** scan-pre-check.log line 8 has PLANNING_SPEC=7a2bffe1..., line 4489 "structural checks failed: 0" and line 4491 "RESULT: PASS".

The gate ends at line 7594: "gate log checks all as expected: yes". The test log ends at 3402: "test log checks all as expected: yes".

## 3. Does every number and quoted string in report-draft.md appear in a primary? YES WITH NOTES

**Sha256s I recomputed:**
- 7a2bffe1 (spec-working.md);
- 931b7e5c (reconstructed from diff.patch);
- 515077bb (diff.patch);
- 6c09db87 (Containerfile, reconstructed);
- 57a828bd, b1d36689 and 047a453d (the three logs);
- 29ce2f63 (scan-pre-check.log);
- a28d721d (image record) and d476e532 (superseded record);
- 7f908a60 (failing scan).

Every one matches the report. I also recomputed the ops script's sha256 as 3d56b34c, matching test.log.

**Hashes I could only confirm from logs:**
- 8f85b6ca and a86114da: gate.log 4300-4301 and commit-messages.txt lines 50-51.
- The superseded files: gate.log 20832166 (gate.log 66; failing scan 4295), test.log f255e2ec (failing scan), negative.log d93ff08a (failing scan 4315).
- Commit 05e4b25: failing scan line 49.

**Figures I recomputed:**
- shortstat: 10 files, 1199 insertions, 28 deletions. Every numstat figure on report lines 28-31 matches diff.patch.
- rewrite.patch: 4 insertions and 2 deletions.
- The record diff: 4 lines.

**Figures confirmed in the logs:**
- Crate tests 515/617/551/657/720/1096 on both targets: gate.log 7139-7370.
- The charts probe at 1075632 with difference 0: gate.log 7426-7427.
- Log end times 16:30:19, 16:33:41 and 16:35:22: the last lines of each log.
- The gate's quoted lines: gate.log 6344, 6878, 6892 and 6962.
- T2's quote: test.log 2504.
- N2 and N5 quotes: negative.log 2730 and 2787.
- "N4 tier a (50000 rows)": negative.log 2634.
- The failing scan's quotes: failing scan 4433, 4475, 4489 and 4491.
- `gen_ok` and `unknown_ok`: rewrite.patch.

**ASCII and commit messages.** report-draft.md and commit-messages.txt contain no non-ASCII byte, so no U+2014. Each of the three commit messages ends with the Co-Authored-By line only (commit-messages.txt lines 21, 44 and 53), with no Claude-Session line.

**Statements with no primary that are not labelled "session's statement" or "no primary":**
- Line 19: the record commit's subject "M5 part 6 P1 record: logs, the image record, the check". No primary yet; it will exist only with the record commit.
- Lines 138-139: "Rulings A to F were asked for before the runner was written". The working spec supports that A was "ruled as the implementing session recommended" and E "as proposed". The timing has no primary.
- Line 41: "Built rootless before any log". This is consistent with the image creation time, 15:06:42 (record line 12), which comes before the first scan at 16:03:47. The "no elevation asked" part rests on the record's own word ("no elevation", line 19).
- Line 70: "outer stderr 0 bytes". Each log shows its own script's stderr as 0 bytes (gate.log end, test.log 3398, negative.log 2824). "Outer" here means the driver, and no primary in this directory shows the driver.

**Placeholders.** RECORD_COMMIT, SCAN_SHA and INDEPENDENT_CHECK are present, as expected.

## Disagreements with the implementer's report

1. Report line 121 says the two chained shapes were in "the committed ops test". The failing scan (superseded/scan-pre-check-failing.log 4425) counts them in "records/logs/m5p6-p1/test.log", where that script is printed. They are the same lines, but the primary attributes them to the log. This is a wording point only.
2. Report line 70, "outer stderr 0 bytes", and lines 138-139, the timing of the request for rulings A to F, are not labelled as the session's statement and have no primary in the check inputs.
3. On the record commit's subject (report line 19), the report is silent that it has no primary yet. It should be labelled or left as part of the RECORD_COMMIT placeholder.
4. Not a disagreement with a number, but omitted from the report:
   - compare-run.sh does not print stderr byte counts (spec line 160, "the log prints their byte counts"). Planning should say whether that belongs to P2's run log.
   - The end-of-run container count is skipped when the runner exits through `die` mid-loop (ruling A, "at the end of every run").
