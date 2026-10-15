Independent check 1

Inputs read: the two specs, diff.patch, commit-messages.txt, report-draft.md, the five logs and the six records, all under `<phase>/check1/`. I modified nothing. The sha256 of diff.patch is 07a4760a...afbe4c, which is the hash gate.log prints for "the phase's combined diff" (gate.log line 5796).

## 1. Does the diff do what the spec says for P2 (rulings F to J), and nothing else?

**Verdict: YES WITH NOTES**

- **The 3D kinds.** `NewPlot` gains `Bar3D`, `Line3D`, `Pie3D` and `Area3D` (diff.patch lines 146-168).
  - `view3D` is written after `autoTitleDeleted`: 15/20/1 for the bar, line and area, 30/0/0 for the pie (lines 539-554).
  - `bar3DChart` writes `gapWidth` 150 and `shape` `box` (lines 685-688).
  - `pie3DChart` writes `varyColors` 1 and no `firstSliceAng`.
  - `series_axis()` covers Line3D always and Bar3D/Area3D with Standard only, which is ruling F (lines 298-307). The third axId follows from it (lines 709-711).
  - `serAx` goes through `axis_bytes` with `Form::Series`: no gridlines, title, numFmt or tail, crossAx 20, crosses autoZero (lines 672-674 and 942-1015). That is ruling I. The test constant SERIES_AXIS matches "The chart part" byte for byte.
- **Combined charts.**
  - `NewCombined` is defined (lines 78-95) and re-exported (line 25). `NewChart::combined` is added.
  - The secondary pair is 30/40: a deleted catAx, or a valAx midCat for a scatter second, then valAx 40 at `r` crossing at `max` with `sourceLinked="0"` (lines 676-696).
  - Series indexes continue from the first plot's (line 727).
- **The rules.**
  - The first plot must be Bar, Line or Area, so every 3D first plot (ruling G) and every Scatter first plot (ruling J) is refused (lines 367-370).
  - The second plot is refused if it is the same variant, compared by `std::mem::discriminant`. That is ruling H's literal reading.
  - All combined rules run before the texts. The text order is title, axis titles, secondary title, secondary format, the first plot's series, then the second's. That is the spec's field order.
  - Refusals use CHART_OPTION_KIND or CHART_TEXT_INVALID.
- **The check-order test.** Its last case (lines 1607-1636) accepts a good chart, then refuses the first plot's title and the first plot's series reference, each with TEXT, while the second plot's texts stay valid. That is ruling I.
- **The other tests.** All 14 tests P2 lists are present, including `a_combined_3d_first_plot_is_refused` (line 1507), plus 3 tool tests.
- **The crate commit message** says "a 3D first plot (ruling G) or a scatter first plot (ruling J)".
- **`combined: None` edits.** They sit only in the helpers `column_chart`, `bare` and `titled`, the tool and the probe, all in the crate commit (gate.log 5890-5895 and "edit lines: 0").
- `--chart-3d-sample` matches "Harness runs".
- There is no Cargo change and no new message (gate.log: "Cargo.lock in the phase diff: 0; manifests in the phase diff: 0"; "distinct messages at the base: 148; at the phase commit: 148").

**Notes:**
- (a) `add_chart`'s doc comment cites "P2 rulings G and H" and leaves out J (line 287). The inline comment at lines 365-366 puts the scatter reason inside the parenthesis that ends "P2 ruling G". The spec only requires the commit message and the report to attribute to J; the code comment reads as G covering both.
- (b) The test module's doc comment says "P2 rulings F to H" (line 862).
- (c) Ruling H expected base test bodies to be edited. None were, because the edits went into helpers. This is consistent with the ruling, not a breach.

## 2. Do the logs contain every line the spec and brief require for P2?

**Verdict: YES**

- **No prunes.** gate.log 11 and 64-65: "this phase prunes nothing (change C60)".
- **The probe.** gate.log 6985-6998: "file 1067556 bytes; sections 1058244". Against part 4: 2876. Against P1's 1055684, read from P1's committed gate.log: 2560. Both are "told to planning, judged nowhere".
- **Test figures** come from P1's committed test.log (gate.log 6690-6702; test.log 3256-3384).
- **Six feature sets on both targets.** 515/599/551/637/702/1076, with clippy and doc over the six sets. Workspace: 1296 = 1279 + 17 on both targets (test.log 3402, 3404).
- **Negative log.**
  - N4 (lines 4459-4460) with control B2 (line 4452).
  - The four CHART_* messages and part 1's three messages are stated unreachable through the binary, each with its covering tests (lines 4134-4139).
- **Run log.**
  - The 3D sample runs on both targets with lo ok 27 and lo_in ok 27 (lines 4453-4631).
  - The more sample's rerun is compared to P1's run: 27 of 27 on each target (lines 4672-4820).
  - The keep is compared against records/m5p5-open-2026-10-14-ad48df3.txt.
  - The edit open check: findings 0.
  - Both record pairs are installed.
- **Endings.** Each of the four logs ends with "... checks all as expected: yes". Every UNEXPECTED string in the logs is script source text, not a result.
- **Scan pre-check.** Line 8: "input: PLANNING_SPEC=e91c254d...f418". Line 72: "...hashes to PLANNING_SPEC...: yes". It ends with "RESULT: PASS".

## 3. Does every number and quoted string in report-draft.md appear in a primary?

**Verdict: YES WITH NOTES**

- **Hashes.** I recomputed all 4 logs and 4 records and every hash matches the report's table. The spec hashes also match: c98cf950 for the committed spec, e91c254d for the working copy.
- **ASCII.** report-draft.md has no non-ASCII bytes and no U+2014.
- **Quotes.** I checked these verbatim and found them all:
  - gate "Cargo.lock...", "distinct messages...148", "edit lines: 0", and the probe figures
  - the N4 and B2 lines
  - the run lines: 27/27, "a drawing created: 17, appended to: 10", 798/0/72/0/178, the families
  - the NVS line, the keep lines 1278/1224/54/0, and the open totals 1266/12 and 1267/11
  - test names 1282/1299, plants P1 to P14, chart write 44 and 46
- **The `git diff --shortstat` and per-file numstat** (6 files, 1667/205) are not printed by any log. I recomputed them from diff.patch and they agree. The doc commit's 124/2 also agrees with diff.patch (ROADMAP 1/1, spec 123/1).

**Statements the report itself says have no primary:**
- the per-commit shortstats of the crate and tool commits (1265/203, 402/2)
- disclosure 1 (the first round)
- disclosure 3 (I60, the "_names" failure)
- disclosure 4 (NVR, "unbound variable")
- disclosure 7 (52 xlsx byte-identical)
- disclosure 10 (the dev run over 26)

**Statements I could not find in any primary, and which the report does not mark as having none:**
- Disclosure 2: "At 6bb8b26 it was 2880 and 2564; the serAx through axis_bytes saved 4".
- Disclosure 9: "53 in the first round".
- The Rulings I line: "The four 6bb8b26 logs and four untracked records deleted". The run log shows only the current status lines, 5 of them, all accounted for.
- The intro: the tool commit "with the same message and the same two files" as 6bb8b26.

All four describe the superseded round, so they cannot appear in these inputs. The report should label them "no primary" the way it labels the others.

**No disagreements** beyond the notes above.
