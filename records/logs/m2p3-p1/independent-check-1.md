# Independent check 1, M2 part 3 P1

Written by the implementing session: the checked phase commit is
b8595ce025fbe05cb3ac6437d3d51adbb1813f21, HEAD when the check ran. The
checker was a fresh general-purpose subagent given only the directory
<scratch>/p1/check1 (<inputs> in the answer), holding these files, by sha256:

    5de1941285f79aae63236a06b166109cedba78b17bd3fbf2ce2ce2436f56858e  diff-b8595ce.patch
    5c6e4e5858fe9f5aa28bfb9ce8c58dba79f621571349be22f1adef07e58441e1  diff-commands.txt
    97c20e1a98467c05f7407c9529dbb90a441bb24b3ff3172929dcde9f4985ca98  diff-d03ed7b.patch
    3cd0b636ca11172e00615898f766615fbab51054c026827bdb9a53e3328e8fcf  diff-fd6bbf8.patch
    7cf35238676cbd9cd5644e8591109f89bc328627035017a57055da260a2849f7  diff-phase.patch
    b7256dc377a9214c57ff9c6597f1f8b722dd26f8d414a4eb5bb24891bb3b9240  log-gate.log
    be40977466d839b80fd0c69de161073ce874a3715df2e85579717f623801e56a  log-negative.log
    cf9372652f3239d73c126b6fa6e4b049fe83ce1325d5951d5955d59e3562bfe0  log-run.log
    ee3f91b22f96402c6ec707b9d891482610275f16ff0eb162030325c56b2e0788  log-scan-pre-check.log
    e73bbb04cd8386f914034bb60facdda5bea8efb662e8458b81908ad8612036bb  log-test.log
    a5fb522aa3408fe365908828a0b558299ff5e84849ed4c195675bb472e13e1f1  record-m2p3-open-2026-09-26-b8595ce.md
    0cff42e8582b6cc61a7626cfa33eb2e542d8971af84bb8c69185e8acaaff39c9  record-m2p3-open-2026-09-26-b8595ce.txt
    94623053b9d2a5d1d2732699d390605da8a1a098fcb675517a82153bb3c93204  record-m2p3-p1-2026-09-26-b8595ce.md
    b5f3e455e76eed84e3853638d764d2af5949ea41dacce82cd90d1713e1e92e32  record-m2p3-p1-2026-09-26-b8595ce.txt
    c2fb01a1a81c359475f3fa3063c4744d009fdfdb3116de83f1003b7ad52c1d2a  report-draft.md
    79a209809e60f29a08d4473b213a4f338cc02c20db555f3933bfd8907e3c6a3c  specs-0001-bootstrap.md
    09c5c539f2563b4556e8038a7dd2259d7957b47fc9e8a1dfc9ed78a86298a52e  specs-0018-rows-and-columns.md
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-gate.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-negative.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-run.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-scan-pre-check.txt
    e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855  stderr-test.txt

The answer below the rule is the checker's final message, verbatim, taken
from its transcript (sha256 6dd513d0d34e66e1b57d0f340badc164c24dc63c5b39848b52c3fb5ff4d7c94e, 10567 bytes).

---

I was given CLAUDE.md, a memory index, git status and attribution text in my context; I did not use any of it as evidence. Everything below comes from the files in `<inputs>`.

**Question 1: yes.** The diff does what P1 and its rulings 1 to 9 ask, and only that. There is one departure that the report states but planning has not ruled on (F3), and a few small points (F4 to F9).

**Question 2: yes.** Every required line I could check is in the logs. There are two limits on that answer (F11, F12).

**Question 3: no.** Most numbers and strings trace to a primary. The exceptions are in F14 to F18.

### Question 1: the diff against the spec

**F1. Which diff I read for each "nothing else".**
- For the phase as a whole I compared the combined diff with the union of the three commit diffs. The added and removed line counts agree for all 10 files.
- The four diffs in `<inputs>` hash to the values log-gate.log prints at lines 185-192:
  - fd6bbf8: 56f0fa9e...
  - d03ed7b: 9df7cfa0...
  - b8595ce: 3bd8ff8d...
  - combined: 7cf35238...
- The per-commit diffs held no surprises:
  - diff-fd6bbf8.patch holds only ROADMAP.md, specs/0017 and specs/0018.
  - diff-d03ed7b.patch holds only cells.rs, lib.rs, package.rs, tests.rs (the `mod rowcol;` line) and rowcol.rs.
  - diff-b8595ce.patch holds only the tool's main.rs and null_rewrite.rs.
- The committed specs/0018 (diff-fd6bbf8.patch) is the working copy with lines 773-827 removed, which is the "Planning rulings during P1" section. The working copy hashes to 09c5c539...

**F2. Design points I confirmed in the code and tests** (diff-d03ed7b.patch unless stated):
- **Area:**
  - It is public, with the five derives and `parse`, `a1` and `cells` (lines 23-100); lib.rs re-exports it.
  - The private Area is removed (lines 623-660).
  - Normalisation happens at the call (ruling 6), tested by `an_unnormalised_area_is_normalised_at_the_call`.
- **Schema position:**
  - One 39-name table and the creation rule (lines 862-905).
  - Creation happens before the first later child in the main namespace, at the root's end, or inside an empty root.
  - Ruling 2 (an element created and then met again is refused) is at lines 1545-1556 and tested.
- **The four format calls:**
  - Range and size checks at the call (lines 505-535).
  - The `<cols>` split, placement by `min`, dropping a bare piece, and a `<cols>` left empty being dropped (lines 1065-1150).
  - Ruling 3's refusal of a malformed `<col>` (lines 1033-1045).
  - Rows retagged (lines 1630-1650) or created through `write_rows` (lines 766-805).
- **freeze_panes:**
  - A1 is refused, the three `activePane` forms are built (lines 962-988), the first view's pane and selections are replaced, other views are copied raw.
  - `sheetViews` is created in the template.
  - An empty or view-less `sheetViews` is refused for `None` as well as `Some` (ruling 9).
- **Merges:**
  - Merge and unmerge each record an entry, with `count` set.
  - The merge's clear goes through the existing clear path, so `s` is kept.
  - Ruling 8 (a covered cell with an attribute other than `r`, `s` and `t` is refused) is tested.
  - Ruling 5: overlap is checked against the file's merges less the unmerged ones, and against earlier pending merges; an unmerge matches the file only.
  - Ruling 4: a `ref` that does not parse is refused with the widened message.
- **Messages:** the nine constants match "Errors" and ruling 4 word for word (lines 574-586). They are catalogued in the module header (lines 233-272).
- **Tool:**
  - `--format-sample` makes the six calls with the spec's constants and M+1.
  - It asserts the three counts, removes the output when a check fails, and follows ruling 1 on counting rows and merges (diff-b8595ce.patch).
- **Tests:** every test P1's list names is present, 28 in the crate and 3 in the tool, plus the four added lines in the flag test.

**F3. A stated departure that still needs a ruling.**
- The spec says nothing of a worksheet is held in memory whole (specs-0018-rows-and-columns.md lines 434-439).
- The rewrite holds `<cols>` and `<mergeCells>` whole instead: `ColsBuffer` at diff-d03ed7b.patch line 1010 and `MergesBuffer` at line 1153. The module header says so (line 193), and the report lists it as decision 1.
- For `<mergeCells>` the buffer is forced, because `count` sits in its start tag.
- For `<cols>` it is a choice. The elements arrive in `min` order, and a refused write's temporary file is thrown away anyway.

**F4. A present attribute is overwritten where it stands.**
- `line_formatted` replaces an attribute where it already stands and appends only a new one (diff-d03ed7b.patch line 827).
- So a row that already has `customHeight` comes out with `customHeight="1" ht="30"` (test at line 2017).
- The design writes `ht` then `customHeight` (spec lines 200-203), and the harness asks for attribute order "as the design gives it" (spec lines 545-546).
- fmtcmp.py models the same rule, so the run log cannot show the difference.
- The corpus has no `customHeight` without `ht` (spec line 123), so no corpus workbook is affected.

**F5. Extra and consistent, but untested.**
- Five `retain` lines drop the new kinds of edit along with the cell and style edits in the same sheet-removal body (diff-d03ed7b.patch lines 581-585).
- The spec does not name this, and no P1 test exercises it.

**F6. Low-severity robustness point in `Area::cells`.**
- `Area::cells` multiplies two `u64` values (diff-d03ed7b.patch lines 69-73).
- For a hand-built public `Area` with a row-0 corner and a `u32::MAX` corner, both factors are 2^32. The product overflows, which panics in a debug build.
- The calls check the grid before calling `cells`, so it cannot be reached through `Package`.

**F7. A comment that no assertion backs.**
- The comment at diff-d03ed7b.patch line 2547 says "nothing refused was recorded".
- That test never checks the write.
- The claim is asserted elsewhere, in `a_merge_of_one_cell_or_off_the_grid_is_refused_at_the_call`.

**F8. The tool counts with the library's own tokenizer.**
- It counts rows and merges with `emend::xml::Stream` (diff-b8595ce.patch lines 212 and 223).
- Ruling 1 says "with quick-xml" (spec lines 779-783); the tool's doc comment says "quick-xml underneath".
- The run log's own comparison supplies the independence, and the "tool and comparison agree" check passes in all 27.

**F9. One control covers two cases.**
- N2 is the single control for both N3 and N4 (log-negative.log line 702).
- P1 says "each with a control" (spec line 632). The report states this plainly.

### Question 2: the logs

**F10. Lines present, all checked:**
- In each of the five logs:
  - The phase commit is given to the script and compared with HEAD, with a null-sha or short-sha control.
  - The three shape greps run over the log's own script, with fixture controls.
  - The exact script is printed.
  - The log's own stderr is counted at 0 bytes, with a one-byte control. The five stderr files in `<inputs>` are also 0 bytes.
- The scan's pre-check stage:
  - Checks that every printed placeholder is declared by a root line (log-scan-pre-check.log lines 1542-1560). My own grep found the same sets.
  - Counts control bytes and forbidden shapes at 0. My own count also found 0 control bytes, 0 non-ASCII bytes and 0 of each forbidden shape in all logs and records.
  - Prints hashes for the logs and records that match the files in `<inputs>`.
- log-negative.log:
  - The carried unbound-prefix case, N1 with control N0 (lines 264-276 and 300-302).
  - The two usage shapes.
  - The statement of what the binary cannot reach.
- log-gate.log: the gate-file listing with no self-test (lines 220-275), and the doc blobs checked against planning's hashes.
- log-run.log:
  - Both targets, with release binaries built at b8595ce.
  - The 12 checks of "Harness runs" P1, including `root_order` against the log's own 39-name table.
  - 12 of 12 plants turn their own flag (line 2799).
  - Host and i686 outputs are identical.
  - Only a `descriptor` flag is allowed, and only on the changed entry.
  - The keep is regenerated and compared with part 2 P3's record.
  - The open check runs, and the records are installed with their hashes.
- Rules that did not apply: no keep was moved, so the moved-keep listing path (run.log lines 254-266) did not run. P1 writes no size log, so the size-rerun and record-install rules do not apply.

**F11. Attempt numbers are not printed.**
- No log prints an attempt number.
- "Attempt 1" rests only on the absence of a rerun line: the rerun block prints only when `RERUN_NOTE` is set (log-gate.log lines 254-270).

**F12. One carried list was out of reach.**
- The carried list in specs/0017's "Phases" section is not in `<inputs>`.
- I checked specs/0001's "Rules for every phase", specs/0017's close-out ruling 4 (diff-fd6bbf8.patch lines 100-108) and the rules named in my brief. A rule that exists only in specs/0017's "Phases" is unchecked.

### Question 3: the report's numbers and quoted strings

**F13. Found in a primary:** everything not listed in F14 to F18. That includes:
- The sha256 09c5c539...
- The commit ids and their parents.
- 39, nine, ten, and the four added lines in the flag test.
- 665/0/2 on both targets, and 31 names added.
- N0 to N4.
- 27 of 27 and 843.
- Twelve flags, and 12 plants of 12.
- Rows created 4, merges added 27, mergeCells created 16, sheetViews and cols created 0.
- 266, 220 and 46 keep pairs, and 0 findings.
- The record names, and the "Planning rulings during P1 (2026-09-26)" heading.

**F14. "every one attempt 1"** (report-draft.md line 22) is not printed; see F11.

**F15. "28 crate, 3 tool"** (report-draft.md line 30).
- Only 31 is printed (log-test.log line 2211).
- 28 and 3 can be counted from the listing at log-test.log lines 2212-2242, but no log prints them.

**F16. "every first sheet has both"** (report-draft.md line 40) is not printed anywhere. It is an inference from `views_created` and `cols_created` being 0 in all 27 rows.

**F17. The quoted "<cols>, merged cells ... copied raw"** (report-draft.md lines 53-54) shortens the removed sentence at diff-d03ed7b.patch lines 176-178. The words exist in that sentence, but the elision drops five items.

**F18. Decision 3's forms and its placement sentence** (report-draft.md lines 56-58).
- `<row r="n" .../>` and `<col min max .../>` are schematic and appear in no primary.
- "set attributes go after the last attribute" leaves out that an attribute already present is replaced where it stands (diff-d03ed7b.patch line 827, test line 2017; see F4).
