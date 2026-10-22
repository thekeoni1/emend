# Independent check 1: M6 part 4 P2

I read only the files under `<check>`. Beyond reading, I computed sha256 sums of the four logs, the scan pre-check, the six records and the working spec, and each file's added and deleted line counts from `<check>/diff.patch`. I also scanned the commit messages, the report and the diff for non-ASCII bytes and U+2014 with python3 -I. I wrote no file, ran no build, test or git command, and read nothing outside `<check>`.

## 1. Does the diff do what the spec says for this phase, and nothing else?

**Yes, with one disagreement (1a) and three doubts (1b to 1d).**

The diff covers every item the P2 section lists:

- **The doc commit.** It adds the spec's P1 rulings A to D and planning's P1 verification (113 lines). ROADMAP.md changes on one line: P1's row goes to DONE.
- **`style`.** `NewChart::style: Option<u8>`. It is checked against 1 to 48 with `CHART_VALUE_RANGE` right after `legend_text` and before the combined plot's rules (ruling F). It is written as `<c:style val="N"/>` after roundedCorners, with no mc:AlternateContent and no style or colour part (decision 2 A).
- **The edits.** `ChartSeriesEdit` (derives as "Types" says, re-exported in package.rs), `set_chart_title` and `set_chart_series`, and the new chart_edit.rs walk. It:
  - checks texts and references at the call and records nothing on an error;
  - applies the write's refusals in ruling E Q1's sequence;
  - refuses a chartsheet with NOT_WORKSHEET (Q4);
  - names cat/val, or xVal/yVal under scatterChart and bubbleChart, refusing a plot outside the sixteen (Q5);
  - keeps the reference form found and drops the cache (Q6);
  - keeps the rich title's bodyPr, lstStyle, pPr and the first run's rPr, replacing only the first run's t and dropping further runs and paragraphs (Q7);
  - handles autoTitleDeleted as Q8 rules;
  - takes prefixes as found and adds xmlns:a on an inserted rich where the root binds no DrawingML prefix (Q9).
- **The message.** One new message, "no series at that position", catalogued in the package header with the size, font and style limits; the catalogue test reads chart_edit.rs.
- **The tool.** `--chart-edit-sample` with three tool tests; the added chart takes `style: Some(26)` (ruling G).
- **The op.** The `chart-text` op and its sixteenth check in the ops test.
- **The carries.** P1's carried comment slips 1a and 1b (ruling C) are fixed.
- **The literals.** `style: None` on the tool's twelve NewChart literals and the probe's two. The probe gains no options, which is right because those are P3's.

I found nothing outside this scope. The per-file counts I computed from the diff equal the report's numstat table on all 13 files.

**Disagreements and doubts:**

- **1a. The kept chart's new title.** Ruling E (spec line ~566) says the edit sample's first kept chart's "new title the constant "emend edit"". The diff retitles it `emend kept`: `<check>/diff.patch` line 2736 `added.set_chart_title(&part, 0, Some("emend kept"))?`, asserted at line 2812. Commit 5548bcc, the README, negative.log ND53 and the run log's edcheck (`'Fixture title' 'emend kept'`) all use `emend kept`. In the diff, `emend edit` is the added chart's first-write title.
  - Neither the README nor the report names this as a departure from the ruling's text.
  - It may be deliberate: a distinct constant keeps the added chart's `emend edited` and the kept chart's title apart in the read-back. Planning should still rule on it.
- **1b. "Per part in record order" (ruling E Q1).** `record_chart_edits` (diff lines ~363-385) runs the twice, removal and addition refusals over every recorded edit of every part first. Only then does it run each part's remove_chart refusals, the index check and so on. With edits on two parts, part 2's "edited twice" refusal comes before part 1's remove_chart refusal. This matches the function's own doc comment, but it is one reading of "per part". I found no test that pins the order across two parts.
- **1c. Overlay on an inserted title.** A title inserted into a chart that has none carries `<c:overlay val="0"/>` (diff line 880). The spec says "title_bytes's form". The diff does not show `title_bytes`, so I cannot confirm that the writer's own form holds that overlay. Q7's "no overlay is added" covers the kept-title case, not this one.
- **1d. Prefix source (Q9).** Chart elements take the prefix of the element they sit in or replace (`chart.prefix`, `one.prefix`, `kept.prefix`, `tx.prefix`), not the root's binding as Q9 words it. The two are the same unless an inner element rebinds the chart namespace. The diff has a test for the prefix cases.

## 2. Do the logs contain every line the spec requires?

**Yes.**

- **Hashes and endings.** Every log takes the working spec at d59ed185... (my sha256 of `<check>/spec-0041-working.md` is the same). Each ends "... checks all as expected: yes"; scan-pre.log ends "RESULT: PASS" with "total hits: 0" and "structural checks failed: 0".
- **Tests (test.log).** ws-host and ws-i686 passed 1414, failed 0. Off, charts, charts,images, charts,comments, charts,read and all passed 515, 688, 730, 733, 791 and 1170 on both targets, failed 0. The ops test prints "ok the parts reader made its 16 checks" and "openpyxl-ops-test: PASS".
- **Corpus run, edit sample (run.log).** Host and i686: summary rows 27, ok 27, lo ok 27, lo_in ok 27; entry totals identical 794, recompressed 0, changed 76, dropped 0, added 70.
  - Every line has read_back=3; kept_edited is 1 exactly in the four workbooks whose W held a chart.
  - Rows are at most 1000, with two workbooks at 1000.
  - Outputs are byte-identical across the labels, 27 of 27.
  - Ruling G: every added chart part has style 26 after roundedCorners (27), and the kept parts keep the input's style (4).
- **P1's corpus run rerun.** "rows compared 27; identical, output and line, by sha256: 27" on each label.
- **Excel-form fixture through the edit sample.** "chartedit charts=2 series=1 rows=6 read_back=3 kept_edited=1" on both targets, byte-identical across them. edcheck's nine checks all yes: the run text alone replaced byte for byte, the four txPr, c14:style 102 and fallback 2, pPr and rPr kept, the rels and both style parts byte for byte. lo and lo_in ok.
- **openpyxl fixture.** Written by the pinned image, whose id matches the record. All three samples ran on both targets with byte-identical outputs: charts 1, styles 1, labels 0 (named as a reading fact, ruling G); text sample charts_after=4; edit sample kept_edited=1. The field comparison:
  - prints each txPr's parent and preceding sibling for both sides;
  - covers sz, b, latin and the style;
  - prints "style ... openpyxl 26, after none; emend 26 (the edit sample's added chart), after roundedCorners; the preceding siblings named, not judged; equal required";
  - names i, the empty run, the bare endParaRPr and the title's missing rPr, and does not compare them.
- **The keep.** 2215 pairs; 2173 of the record's unchanged; 42 new, exactly this phase's tests'; crate 2065 and tool 150 pairs with no output refused.
- **Gate and negative.** gate.log gives the roles, the file sets, the message sets 154 and 155, the probe figures and the prune. negative.log gives 15 cases refused and 11 outputs.

**Doubt 2a.** The spec's "every other entry identical to the first write's output" is checked by the tool itself, which exits 2 if it fails. The logs show it only indirectly, through the ok rows and the per-family entry rules; no line prints that comparison directly.

## 3. Does every number and quoted string in the report appear in a primary?

**Yes.** I found every number and quoted string in a primary except the ones the report marks as its own.

The report marks these as the session's own; I did not count them as failures:

- Nothing pushed.
- Each logged log is the first of its kind.
- The dry run at f229553, yes for all four.
- Outer standard error 0 bytes.
- The "session's statements" paragraph, which only the README holds (forks, generations, block O pinned dbc3805f..., the scan fork's `rm`, the development figures).

The numstat table is marked "no log prints it". I checked it against the diff and it matches.

**Points of wording (not failures):**

- **3a. The style commit's line counts.** "the style commit's one line each way, None to Some(26)" is gate.log's claim about the edit sample's NewChart literal alone. The style commit changes the tool's main.rs by 16 added and 11 removed lines (gate.log ~8238), plus the tool test. The sentence should say "its NewChart literal's".
- **3b. The openpyxl fixture's ids.** "(28875 bytes, id e6dcd6ce27d0, the image id the record's)" reads as if e6dcd6ce27d0 were the image id. It is the fixture's id; the image's id (894b239f...) is the one that matches records/openpyxl-image-2026-10-16.md.
- **3c. The probe figure.** The report prints the probe's 29036 over part 3's 1107960 without comment, as "judged nowhere here" allows. The figure is already above the spec's 25000 line, and over the 12000-to-20500 estimate, before P3 adds the probe's options and edit. Planning may want it said in so many words.

## Hashes and format

Every sha256 in the report's tables matches the file in `<check>`:

| File | sha256 |
|---|---|
| gate.log | 4ecd52bc... |
| test.log | bef63747... |
| negative.log | 647e3945... |
| run.log | be00488e... |
| scan-pre.log | 38103296... |
| The six records | 7b10b177..., 83c9ccde..., 095f5578..., c16fdea1..., 5637fa0c..., 920f5fdc... |

The commit messages, the report and the diff hold no non-ASCII byte and no U+2014. Each of the five commit messages ends with Co-Authored-By alone, and none has a Claude-Session line.
