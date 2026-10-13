# M5 part 2 P3: independent check 2

A fresh subagent, given only the directory <home>/.cache/emend-build/m5p2-p3/check2, at the phase commit f35de0d. It held:
- spec.md (specs/0033 at the phase commit, the doc commit's blob);
- diff.patch (c4d67b6..f35de0d) and commit-messages.txt;
- report-draft.md (sha256 d7aeea2d2ceaf4acdc9788aa4a620c7cbe15d27461234f839df977f896f62f6a, holding check 1's answer);
- the gate, test, negative, size and run logs in this directory, and the scan pre-check log;
- the ten record files at f35de0d.

Their sha256 values are in <phase>/check2-inputs.sha256. It answered the three questions (CLAUDE.md, "Independent check before reporting"). This is the second and last check of the phase, planning's limit of two. Its answer follows verbatim. It was ASCII as delivered, so nothing was replaced (sha256 d356caf2ae7a573d2474d582630fee5bc82ac97723e17b7a46e51c9e0054f439).

---

# Independent check 2: M5 part 2 P3 at f35de0d

I read every file in check2/ and modified nothing. I used one scratch directory (tmp/check2-scratch/) for two small Python helpers.

- **Hashes:** every log and record hash in report-draft.md matches `sha256sum` here (report-draft.md 56-60, 70-79). The scan pre-check is e4ef1033... (line 83). spec.md is 843740182d..., planning's pin. diff.patch is be7d7efe..., which is the combined-diff hash gate.log 4973-4974 prints.
- **Diff stat:** I computed it from diff.patch: 11 files, +1225 -26. That matches report-draft.md 22. The spec and ROADMAP hunks give the doc commit's +97 -2 (line 20).
- **Scripts:** I extracted the six scripts printed in the logs and hashed them. All six match the report's table (f): gate 1ad84f32, test 7a49df0d, negative 491af9d5, size eb49125a, run b61cb049, scan 1fcc2cf3.
- **Digests:** I recomputed the FNV-1a digests from the fictional constants. They reproduce:
  - the run-log fixture: f7f4031246258bc6 and 3db3b5e2dea5eeb9 (run.log 6040);
  - the tool test / B0 fixture: 1bc1bd5aa879c675 and 8fb0ce88c32de74f (negative.log 3694-3695);
  - the empty digest cbf29ce484222325.
- **Encoding:** diff.patch, commit-messages.txt and report-draft.md contain no non-ASCII byte. Every commit message ends with the Co-Authored-By line alone (commit-messages.txt 23, 37, 53, 88, 108, 137, 147).

## 1. Does the diff do what P3 says, and nothing else?

### Present, as P3 asks

- **Doc commit:** "Planning rulings on P2" and "Planning verification of P2" are added (diff.patch 1142-1235). ROADMAP marks P2 DONE and P3 IN PROGRESS (diff.patch 24-27).
- **Reader:** `Package::external_links` is read once and lent from a cached field (diff.patch 168-187, 285-292). It reads the first `<externalReferences>` in order and resolves each `r:id` through the workbook's `externalLink` relationships (296-315, 330-358). The types match spec.md 368-372 (diff.patch 220-258) and are re-exported under `read` (154-167).
- **Refusals:** every one in spec.md 382-390 is implemented:
  - no `r:id` (346-348);
  - an `r:id` naming no relationship, or one of the wrong type (309-312);
  - the root (379-381);
  - none of the three children, or more than one (411-417);
  - no `r:id` on a book or OLE link, one naming no relationship, or one that is not External (362-369);
  - a `sheetName` with no `val` (435);
  - a `ddeLink` missing either attribute (388-392);
  - no `progId` (400).
  
  A missing entry is "no such part" (318). `sheetDataSet`, `ddeItems` and `oleItems` are walked past (397, 405, 444-446).
- **Message:** one new constant, `EXTERNAL_LINK_UNREADABLE` (217-218). negative.log 3233-3247 confirms it is the only new constant.
- **Catalogue and limits (package.rs):**
  - The relationship-types paragraph names PRINTER_SETTINGS (P1 ruling 4) and SLICER/TIMELINE (ruling 2) (diff.patch 39-47).
  - The "no such part" entry gains the three readers' tests (56-61).
  - Both messages are listed under `read` (69-97).
  - The Limits entry covers every item P3 lists (106-145): messages by feature, the survey counting what the package says, the cache by relationship with `cacheId` not joined, `sheetDataSet` and records never read, targets as written.
  - The catalogue test now reads pivot.rs and extlink.rs (586-654).
- **Tool:** `--extlink-sample` has every key the spec names, FNV over the Book and Ole targets and over the sheet names, and writes the input back unedited (1248-1387). It has two tool tests (1401-1529).
- **Probe:** four calls and four lines, as "Size" asks (1530-1619).
- **CHANGELOG:** one Added bullet citing specs/0033 (1-12).
- **Tests:** every item on P3's test list is present (664-1133):
  - three references read in a shuffled order (852-895);
  - a book with no sheet names and no defined names (898-922);
  - `sheetDataSet` walked past (925-945);
  - strict (948-974);
  - every refusal on the list (977-1113);
  - an empty slice (846-849);
  - the tool's line (1486-1511).
- **The A1 fix works:**
  - `rIdW` now names a styles relationship whose target is a valid link part (diff.patch 984-1009).
  - The two root fixtures each hold a main-namespace `externalBook` (1019-1029).
  - test.log 3929-3936 and 3984-3991: plants P2 and P7 each fail exactly their test.
- **The A2 fix:** the strict test's workbook-to-link relationship is now strict (diff.patch 964 with 751-755, `{attr}` = ATTR_STRICT).

### Lacking

Nothing that P3 or the rulings binding it require.

### Beyond the spec (all disclosed and ruled by planning; none blocking)

- **Shared helper changed:** `pivot.rs`'s `Element` now captures `r:id`, and its helpers are `pub(super)` (diff.patch 482-584). This changes P2's reader: a malformed escape in a pivot `r:id` is now `Error::Xml` (report-draft.md 167).
- **Extra refusal:** a non-UTF-8 `Target` is refused with the link message (diff.patch 368). It is untested.
- **Untested error path:** an `externalLink` relationship with TargetMode External gives "an external relationship has no part" (doc at 279-282).
- **Two extra tests:** `the_list_is_read_once_and_lent` and `a_link_relationship_naming_no_entry_is_no_such_part` (1115-1133). They are harmless.
- **Observation, not a defect:**
  - The reader never checks the link part's own relationship TYPE (`externalLinkPath` / `oleObject`), only TargetMode (362-369).
  - The spec's refusal list does not ask for that check, so `EXTERNAL_LINK_PATH` goes unused by the reader.
  - So the strict test's link-part relationship type (957-961) exercises nothing.
  - Only the strict `EXTERNAL_LINK` matters there, and no plant covers it. The report gives the session's hand check (report-draft.md 50), which no log shows.

## 2. Do the logs contain every required line?

| Requirement (spec.md / ruling) | Where | Present |
|---|---|---|
| `input: TMPDIR=<scratch>/tmp`; mktemp directory under it, with a control | gate 10, 45-62; test 10-11, 74-82; negative 7, 71-80; size 15, 64-72 area; run 18-19, 114-122 | yes |
| P1 ruling 12: every mktemp call checked or no plain file | gate 52-62; negative 3204-3213; test 2699-2701; run 120-122 | yes |
| P2 ruling 7: one gate line for the counting library's directory under TMPDIR, with a control | gate 4801-4803 | yes |
| No committed script calls python3 without -B, with a control | gate 5181, 5195-5196 | yes |
| Doc commit at planning's hashes | gate (quoted at report 20); run 4115-4119 | yes |
| Crate tests: off, read, read,rules, all, both targets; pub fn lines | test 2485-2527, 2573-2598; gate 5364-5640, 5888+ | yes |
| Workspace count | test 2455, 2477, 2608-2611 | yes |
| clippy and doc clean | gate 5779-5884 | yes |
| Consumer crate check | gate 6018-6055 | yes |
| API listing rerun: new items listed, count stated | test 3689+; api-review record lines 14-26 | yes |
| Negative: every reachable message of the part through the binary | negative 3700-3705 (N1 extlink, N2 no such part, N3 pivot), controls B0/P0 3698-3699 | yes |
| The spec's case: an internal `externalBook` relationship, with a control | negative 3700 with B0 3698 | yes |
| Unreachable messages stated, with their covering tests | negative 3274-3277 | yes |
| Size: 28 probes classified | size 7554+, 10353 | yes |
| Size: feature cost | size 8377 | yes |
| Size: core growth, both figures printed | size 8367-8369 | yes |
| Size: budget against 458752 | size 8373-8376 | yes |
| Size: a cause per probe; the cause control; the manifest listing | size 8278-8312, 7883-7885, 7803-7805 | yes |
| Size: FINDING above 110000 | size 8399 | yes |
| Size record installed | size 10634+; size-m5p2 record 113+, 186-200 | yes |
| Run: `--extlink-sample` over the corpus, both targets | run 5679-5903 | yes |
| Run: `links` 0 against no `<externalReferences>` and no content type | run 5783, 5893, 5904-5921 | yes |
| Run: planted-flag controls | run 5923-5937 | yes |
| Run: generator text printed | run 5946-6037 | yes |
| Run: fixture on both targets, value by value, output byte-identical | run 6038-6062 | yes |
| Run: release binaries are size.log's | run 6064-6087 | yes |
| Run: three samples again, byte-identical by sha256 to P1/P2 (named by phase and record commit) and to this phase's run (ruling 11) | run 6089-7043 (6355, 6386, 6686, 6717, 7012, 7043) | yes |
| Run: kinds 24/24 with custom_xml 30 in 10 (P1 ruling 1) | run 7089-7118 | yes |
| Run: pivots 18/18 with data_field_flags 10 in 3 (P2 ruling 1) | run 7119-7136 | yes |
| Run: devices of TMPDIR and the corpus directory | run 5670-5677, 7580-7583 | yes |
| Run: calc-chain workbooks with their rule | run 4134-4142, 5785 | yes |
| Run: CHANGELOG citation check (specs/0031), with a control | run 7141-7395 | yes |
| Run: keep against the named record; edit open check; open record installed | run 7398-7470, 7579-8855, 8856-8865 | yes |
| Rerun: superseded attempts hashed in each step's own log | gate 154-156; test 58-60; negative 57-59; size 78-80; run 82-85; scan 69-70 | yes |
| scan.log (final) | not yet run; pre-check only (RESULT: PASS, scan-pre-check 3974-3978) | expected for a draft |

**Notes (non-blocking):**
- **B1 still holds.** The first extlinks corpus run and the "again" run use the same release binary (run 4128 = run 6074; size.log's hash). That comparison is a repeat-run check, not a second build. Planning ruled this accepted.
- **The negative log runs on host only.** It uses the gate's debug binary (negative 3193-3202). The spec does not ask for both targets there.

## 3. Does every number and quoted string in the report appear in a primary?

I machine-checked every double-quoted string and every hash, number and time in report-draft.md lines 1-246 against the logs, records, diff, commit messages, scan pre-check and spec, ignoring "..." elisions and whitespace. I also spot-checked by hand that each quote says what the report claims.

### Verified

- every commit hash and subject;
- every log and record hash, and every log's last line;
- the gate quotes (TMPDIR, mktemp 3/0, T52, 18/4/0 and the history control, pub fn 201/202 and the added line, the consumer line, clippy/doc, no prune and no self-test);
- the tests 515/618/656/990 and 1196, the 10 extlink and 2 tool tests;
- the eight plant lines;
- the API figures 660/567/7/100, the formatter line, 66215 bytes, the 93/7 split and the CHART text;
- negative B0 and N1-N3;
- the size figures 112600/110592/100912/70044/42556, 970208-970208, 436992/438272/343904, 21760, the 6/0/21/1 tally and "classified as code";
- the run figures (27s, 9 of 9, digests, fixture line, rows compared 27 x6, the P1/P2 phase and record commits, 50bcf1ea, 24/24, 18/18, calcChain 9, CHANGELOG 20/19 and "missing 1", keep 1102/2204, the new pair, the open-check totals);
- the superseded hashes (gate 4ecc4a35, test c3abc873, negative 2fe6b0ff, size 943fbd0a, run 12e274a5, sums d7090c08, scan f9e340de);
- the OA root line at size.log 88, as the report says;
- the six script hashes;
- the start and end times at report 81 (gate.log 153 and each log's "run ended" line).

### Not found in any primary, and not explicitly marked as the session's own

The report's blanket disclosure (report-draft.md 177, "Items with no log line are the session's statements") nominally covers all of these. Each is in a section that does not say so where it appears:

- **C1 (repeat of check 1's C1 class).** report-draft.md 46: the tests commit is "+44 -7".
  - No log prints per-commit line counts; diff.patch is cumulative and shows extlink.rs tests only as +464.
  - gate.log 5102-5104 confirms only that the commit touches that one file.
  - It is the same kind of item check 1 flagged and planning ruled fixed. It should be sourced or marked.
- **C2.** report-draft.md 165: "planning's message of 2026-10-13T04:12Z". No primary holds it.
- **C3.** report-draft.md 157: the quoted FACTS.md wording "the sheetDataSet read as sheetNames". FACTS.md is not a primary; test.log 1982 only says P8 was "adjusted".
- **C4.** report-draft.md 170: the hashes of report-draft-check1.md (8a1220c0...) and the 81230b5 draft (e862bb57...). No primary holds them.
- **Covered by the disclosure section:** everything in Disclosures (a)-(j) that no log prints. That includes:
  - logged.sh f6669dda;
  - the dry-run times and hashes;
  - 31e317b;
  - the superseded scan, size and run directory hashes;
  - the generator hashes;
  - 8972372078;
  - pid 304957 and the 22:25Z-23:53Z window;
  - the check-1 raw and scrubbed hashes;
  - "push 10/12/2026";
  - the quoted failing lines of the superseded scan attempt 1 and of test dry run 1.

### Sourced but inaccurate (non-blocking, should be corrected)

- **C5.** report-draft.md 143 says the keep's "pairs 1102; files 2204" were "taken over from the 81230b5 attempt's keep". The logs show the reverse:
  - The attempt-2 keep was moved aside before the run (run.log 86-90: "moved before this run: <keep> to <scripts>/out/superseded/keep-attempt-2").
  - The keep was then regenerated by `cargo test` (run.log 7398-7444). The figures are the regenerated keep's.
  - Also, it was moved to `<scripts>/out/superseded/`, not `<phase>/superseded/`.
- **C6.** report-draft.md 67 says each log prints "input: RERUN_NOTE=...". Only size.log 5 and run.log 7 use that form.
  - test.log 3 and negative.log 3 print "input: RERUN_NOTE: ..." (colon).
  - gate.log 4 prints "input (environment) RERUN_NOTE: set" with the text on a "rerun:" line at 154.
  - The text itself is in every log.
- **C7.** report-draft.md 155 says "The growth is the read probe's alone; this phase grew that probe by four calls." The size record names five commits as the read probe's cause: a301387, 7f7dc03, cdffef7, plus P1's ec9e58f and P2's 338e91c (size-m5p2 record 162 and 230-232; size.log 8304).
  - The 42556 is the survey, pivot and extlink code that the four new calls now link in.
  - The sentence is not false, but it should say the cost covers all three readers.

### Placeholders still unfilled (expected in a draft)

RECORD_COMMIT and RECORD_SUBJECT (3, 18), SCAN_HITS (3), SCAN_SHA and SCAN_LAST (61), CHECK1_SHA (62), CHECK2_SHA (63) and CHECK2_TEXT (408).

CHECK1_SHA could already be filled: line 243 gives the installed file's hash, 61d995db...

## Verdict

**blocking defects: 0.**

Non-blocking, for planning to rule on:
- C1: "+44 -7" is unsourced and unmarked.
- C2: the 04:12Z time is unsourced.
- C3: the FACTS.md quote is unsourced.
- C4: the two check-1 draft hashes are unsourced.
- C5: "taken over from" misdescribes the keep, which was regenerated.
- C6: the RERUN_NOTE form is misquoted for gate, test and negative.
- C7: Departure 1 credits the read probe's growth to this phase's four calls, while the size record names P1 and P2 commits as causes too.
- No log or plant shows the strict `EXTERNAL_LINK` coverage; only the session's hand check does.
- The extlinks "again" comparison is the same binary twice (B1, already ruled).
- The draft's placeholders must be filled from primaries.
