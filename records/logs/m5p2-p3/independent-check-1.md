# M5 part 2 P3: independent check 1

A fresh subagent, given only the directory <home>/.cache/emend-build/m5p2-p3/check1, at the earlier phase commit 81230b5. It held spec.md (specs/0033 at the doc commit), diff.patch (c4d67b6..81230b5), commit-messages.txt, report-draft.md (the draft at 81230b5, sha256 8a1220c082dab7797d599e7d9c0246cedffcd5878288836c0b587c5a716afde6), the gate, test, negative, size and run logs at 81230b5, the scan pre-check log and the ten record files at 81230b5. Their sha256 values are in <phase>/superseded/phase-81230b5/check1-inputs.sha256; every file it read is superseded (<phase>/superseded/phase-81230b5), so its line numbers and hashes refer to those files, not to this directory's logs. It answered the three questions (CLAUDE.md, "Independent check before reporting"). Its answer follows verbatim but for one scrub: as delivered (sha256 4c5914c23c2022aab713659e4c18daa91c2765ebd1b27ba8b1f6caa4f999376d) it held five U+2026, each replaced by "..." and nothing else changed (sha256 61d995db1ecbc5c1753bef0a99dd59eee2df57558b394d5eacafcedc76e350f5). Its blocking defect, A1, was fixed by the tests commit f35de0d, the phase commit (planning's ruling, 2026-10-13); every log here is from f35de0d.

---

# Independent check: M5 part 2 P3 (external links, catalogue, probe, size record, CHANGELOG)

I read every file in `check1/`, read-only. I made one scratch directory and wrote no files. Every log and record hash in report-draft.md matches `sha256sum` of the files here. `spec.md` hashes to 843740182d..., the pinned blob.

## 1. Does the diff do what P3 says, and nothing else?

### Present, as the spec asks

- **Doc commit:** specs/0033 gains "Planning rulings on P2" and "Planning verification of P2" (diff.patch 1105-1198). ROADMAP marks P2 DONE and P3 IN PROGRESS (diff.patch 26-28).
- **External link reader:** `Package::external_links`, the types and the message are in extlink.rs (diff.patch 188-454). It is gated `read` (154-163) and cached in a field (168-187).
- **Refusals:**
  - an `externalReference` with no `r:id`, or one naming no relationship or one of the wrong type (309-312, 346-348);
  - a root that is not `externalLink` (379-381);
  - none of the three children, or more than one (411-417);
  - an internal or missing book or OLE relationship (362-369);
  - a `sheetName` with no `val` (435);
  - a `ddeLink` missing either attribute (390-392);
  - an `oleLink` with no `progId` (400).
- **Walked past, never read:** `sheetDataSet`, `ddeItems` and `oleItems` (397, 405, 444-446).
- **`--extlink-sample`:** every key the spec names, FNV-1a over Book and Ole targets and over sheet names, and the input written back unedited (diff.patch 1296-1350). Two tool tests (1449-1492).
- **Catalogue and limits:** both read messages marked by feature (69-97). The "no such part" entry gains the readers' tests (56-61). The relationship types paragraph names PRINTER_SETTINGS (P1 ruling 4) and SLICER/TIMELINE (ruling 2) (39-47). The Limits entry covers all five items the spec lists: messages by feature, the survey counting what the package says, the cache reached by relationship with `cacheId` not joined, `sheetDataSet` and records never read, targets as written (106-145). The catalogue test reads pivot.rs and extlink.rs (590-651).
- **Probe:** four calls and four lines, as "Size" asks (diff.patch 1493-1583).
- **CHANGELOG:** one Added bullet citing specs/0033 (diff.patch 1-14).
- **Tests:** every item on P3's list is present (diff.patch 845-1096, 10 tests):
  - three references read in a shuffled order;
  - a book with no sheet names and no defined names;
  - a `sheetDataSet` walked past;
  - the strict namespace;
  - each refusal on the list;
  - an empty slice when there is no `<externalReferences>`;
  - "no such part".

### What the diff lacks or gets wrong

- **A1 (BLOCKING): two refusals the spec lists have no test that can fail.**
  - The refusals are the `externalReference` wrong-type check (diff.patch 311) and the root-name check (379-381).
  - The wrong-type test points at `rId1`, the worksheet relationship (diff.patch 978-979). With the type check removed, the worksheet part is read and refused for its root, with the same message.
  - Both root-test fixtures (987-1000) hold no main-namespace child. With the root check removed, each is refused as "none of the three children", again with the same message.
  - test.log confirms it: plant P2 (the type compared with EXTERNAL_LINK_PATH) does not fail `a_reference_with_no_id_or_naming_no_link_relationship_is_refused` (test.log 3920-3942). The report's Departure 2 says the same (report-draft.md 134-137).
  - The committed catalogue still names these tests as covering both refusals (diff.patch 87-96).
  - The spec's test list asks for "an `r:id` of the wrong type" (spec.md 630). The test exists but cannot fail when that check is gone. The fix:
    - point `r:id` at a relationship of another type whose target is a valid link part;
    - plant a main-namespace root of another name that holds a valid `externalBook`.
- **A2 (minor): the strict test only half-tests strict.** It uses strict element and attribute namespaces, but every relationship `Type` URI is transitional (diff.patch 716-719, 751-754, 958). The strict spelling of `EXTERNAL_LINK` is never exercised.

### What the diff does that the spec does not ask for

All of these are small.

- **Shared helper changed (pivot.rs, diff.patch 485-546).** `Element` now captures `r:id` and its helpers are `pub(super)`. As a side effect, the pivot reader now normalises `r:id` values, which it used to skip. A malformed escape in a pivot `r:id` would now raise `Error::Xml` where it was ignored before. This is a behaviour change to P2's reader that the spec did not ask for. The corpus and the tests are unaffected.
- **Extra refusal (diff.patch 368).** A non-UTF-8 `Target` is refused with the link message. The spec's list doesn't include it, and it is unreachable in practice.
- **Only the first `<externalReferences>` is read (diff.patch 338-353).** A second one would be silently ignored. The schema allows at most one.
- **A reachable error path that is documented but untested.** An `externalLink` relationship with `TargetMode="External"` gives "an external relationship has no part" (doc at 280-282). No test covers it.

## 2. Do the logs contain every line P3 and the binding rulings require?

| Requirement | Log | Present? |
|---|---|---|
| `input: TMPDIR=<scratch>/tmp`, the mktemp directory checked under it, with a control | gate 10, 49-62; test 10, 72-75; negative 7, 71-74, 3188-3197; size 14; run 18, 145-151 | yes |
| P1 ruling 12: every mktemp call checked | gate 54-62; negative 3189-3196; run 150-151 | yes |
| P2 ruling 7: gate prints one line for the library's mktemp directory under TMPDIR, with a control | gate 4771-4773 | yes |
| No committed script calls `python3` without `-B`, with controls | gate 5136-5161 ("files scanned: 18; Python calls: 4; calls without -B: 0") | yes |
| Doc commit at planning's hashes | gate 79-99, 4865-4906 | yes |
| Crate built and tested with features off, read, read,rules and all, on both targets; pub fn lines off and on | gate 5321-5530, 5845-5975; test 2467-2520 (515/618/656/990 on host and i686) | yes |
| Consumer crate check | gate 5975-6030 | yes |
| clippy and doc clean | gate 5736-5844 | yes |
| API listing rerun: new items listed, count stated | test 3673-3691, R2 3721-3853; record line 7 and lines 14-26 | yes (but see C2) |
| Negative: every reachable message of the part, through the binary | negative 3684-3689 (N1 extlink, N2 no such part, N3 pivot), B0 and P0 controls 3682-3683 | yes |
| Unreachable messages stated, with their tests | negative 3259 ("none of the part's messages") | yes |
| The spec's case: an `externalBook` naming an internal relationship, refused, with a control | negative 3684 with B0 | yes |
| Size: the 28 probes classified | size 7539+, 10337 | yes |
| Size: feature cost | size 8361 | yes |
| Size: core growth, both figures printed | size 8306, 8354-8356 | yes |
| Size: budget against 458752 | size 8358-8360 | yes |
| Size: a cause per probe, the cause control, the manifest cause listing | size 8262-8296, 7787-7789 | yes |
| Size: FINDING above 110000 | size 8383 | yes |
| Size record | size-m5p2...md 113, 177, 182 | yes |
| Run: `--extlink-sample` on corpus, both targets | run 5685-5900 | yes |
| Run: `links` 0 against the absence of `<externalReferences>` and of the type | run 5910-5927 | yes |
| Run: planted-flag controls | run 5929-5943 | yes |
| Run: generator text printed | run 5951-6038 | yes |
| Run: fixture on both targets, checked value by value, output byte-identical | run 6040-6068 | yes |
| Run: three samples again with the size log's release binaries, against P1's and P2's runs (named by phase and record commit) and this phase's own run (ruling 11) | run 6095-7052 | yes |
| Run: planning's figures, kinds 24/24 with custom_xml 30 in 10 (P1 ruling 1) | run 7109, 7120 | yes |
| Run: planning's figures, pivots 18/18 with data_field_flags 10 in 3 (P2 ruling 1) | run 7131, 7141-7142 | yes |
| Run: devices of TMPDIR and the corpus directory | run 5676-5684, 7577-7581 | yes |
| Run: calc-chain workbooks with their rule | run 4140-4148 and per run | yes |
| Run: keep against the named record | run 7449-7470 | yes |
| Run: edit open check over the keep | run 7577+, 8687 | yes |
| Run: open record installed | run 8854-8862 | yes |
| Run: CHANGELOG citation check (specs/0031), with a control | run 7340-7398 | yes |
| scan.log (the scan runs last) | pre-check only (RESULT: PASS); final scan not yet run | cannot check |

**Observation B1 (not a missing line).** The first `--extlink-sample` corpus run used release binaries the run log built at `<target>/main`. Their hash, 50bcf1ea..., is identical to the size log's binary (run.log 4135, size.log 10306). So "extlinks (release) against this log's first run" (run.log 6987-7049) runs the same binary twice. That is a determinism check, not a cross-build one. Ruling 11 allows this, but no debug-profile external link corpus run exists.

## 3. Does every number and quoted string in report-draft.md appear in a primary?

**Verified.** Everything below matches a primary:

- every log hash (report-draft.md lines 46-50) and every record hash (56-65);
- every commit hash and subject;
- the doc-commit hashes and +97 -2;
- the gate quotes;
- test counts 515/618/656/990 and 1196;
- the eight plant descriptions and their "yes" lines;
- the API figures 660 / 567 / 7 / 100;
- negative lines N1-N3;
- size figures 112600 / 110592 / 100912 / 70044 / 42556 / 970208 / 436992 / 438272 / 343904 / 21760 and the verdict tally;
- run lines (27s, 9 of 9, digests, fixture line, 24/24, 18/18, calcChain 9);
- CHANGELOG 20/19;
- keep 1102/2204, the new pair, open totals and findings 0;
- the scan pre-check hash and RESULT;
- the superseded size and run hashes (size.log, run.log, scan-pre-check.log);
- the OA root line;
- the RUN_SUFFIX and SUPERSEDED lines;
- "recursive removals not in the ruled form: 0".

I also hashed the script each log prints. That reproduces the report's (f) hashes for all six scripts. The helper-script hashes (extlinkcmp.py, kindscmp.py, pivotcmp.py, changelog_check.py, make_review.py, p1_checks.sh) appear in run.log and test.log.

**Not found in any primary, and not marked as the session's own:**

- **C1:** line 21 says extlink.rs is "+260" in the crate commit, and line 32 says pivot.rs has "43 lines changed". No log prints per-commit line counts. The diff gives 261 and 46 cumulative, since the catalogue commit also touches both files (gate.log 5051-5056). The figures are arithmetically plausible but unsourced.
- **C1b:** line 3 says "origin/main is c02942eb5b68b25dd02217c7b21863cbf3767225". No primary holds this full hash, or any statement about origin/main.

**Not in a primary, but covered by the report's own "Sources" disclosure (line 143) or by first-person narrative:**

- the logged.sh and logged2.sh hashes;
- the times 00:40:32Z and 02:12:36Z;
- the gen/size.py hashes and size.sh 67314d... (attempt 1);
- the size-dry-2, test-dry-1, negative-dry-1 and size-dry-1 hashes;
- superseded commit 31e317b;
- 6583894537 bytes, pid 304957, and the 22:25Z to 23:53Z window;
- the quoted attempt-1 scan line ending ": no", and "RESULT: FAIL". scan-pre-check.log's RERUN_NOTE only corroborates these.

That blanket disclosure is very broad. Planning may want each such item tagged where it appears.

**C2 (a statement that is sourced but misleading):** Departure 3 reports that CHART's added commit is 5fc1b3d, as fact.

- The spec's Facts list of existing constants does not include CHART (spec.md 146-151). Design adds it in this part (221).
- So the committed API record's "CHART ... added by 5fc1b3d / specs/0009" (api-review 28, 284) is a misattribution by the heuristic.
- test.log hard-codes that exception (test.log 1826, 1876-1880) instead of flagging it.
- The record also titles 100 rows "the new items" (api-review 26), but 7 of them are existing constants with their cfg widened, so there are 93 truly new items.
- The report should name both as a record defect for planning, not as a neutral fact.

**C3:** placeholders RECORD_COMMIT, RECORD_SUBJECT, SCAN_HITS, SCAN_SHA, SCAN_LAST and CHECK_TEXT remain (lines 3, 15, 51, 200). That is expected in a draft, but they must be filled from primaries.

## Verdict

**blocking defects: 1.** A1: the external link reader's wrong-type check (extlink.rs, diff.patch 311) and its root-name check (diff.patch 379-381) have no test that fails when either check is removed. The plant table in test.log and the report's own Departure 2 show this. The committed catalogue (diff.patch 87-96) still cites those tests as covering both refusals, and the spec's P3 test list requires the wrong-type refusal to be tested.

Non-blocking, for planning to rule on:
- A2: strict relationship types are untested.
- B1: the extlinks release rerun compares a binary with itself.
- C1 and C1b: unsourced numbers and hashes in the report.
- C2: CHART is misattributed in the API record, and its "new items (100)" count includes 7 widened rows.
- The pivot reader's `r:id` normalisation is a side-effect change to P2 code.
