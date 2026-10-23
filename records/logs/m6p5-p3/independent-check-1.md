Check 1, M6 part 5 P3 (specs/0042 "P3", rulings AN to BF)

I read only: the files under <phase>/check1, and git show, log, diff and rev-parse in <repo>. I ran no cargo and no script, and wrote nothing. All 16 files under check1/logs and check1/records hash to the values the draft and the logs README give. spec-0042-working.md hashes to 7742fae1762057eb97e886cf2e986f2a3f15f51ddcea4c96f89518257ffe109b. diff.patch is byte-identical to `git diff d4d9eca fe280ff7` (sha256 9b4cade0... for both). The committed spec blob at 693497c is 7658019a...8566 and ROADMAP.md there is 69c26b1b...aec, as the doc commit's message says. The working spec differs from the committed one only by the added "Planning rulings during P3" section (165 lines added, 0 removed).

1. Does the diff do what the spec says (P3, as amended by AN to BF) and nothing else?

Verdict: yes.

- **Doc commit (693497c).** It adds 221 lines to specs/0042: the "Planning rulings during P2" and "Planning verification of P2" sections. It changes one ROADMAP.md row: P2 goes from PLANNED to "DONE 2026-10-23 (bf07f86; record d4d9eca local)".
- **Test commit (fd3c967).** One test in crates/emend/src/package/tests/named_style.rs: `self_closing_cell_style_xfs_and_cell_styles_are_opened_for_a_named_style`. It is ungated and loops over two forms, "" and ` count="0"`.
  - Through `add_named(..., 0)` it asserts that the index returned is 0.
  - It expects `<cellStyleXfs{opened}>` and `<cellStyles{opened}>`, where `opened` is `count.replace('0','1')`. So the form without `count` stays without one, and `count="0"` becomes `count="1"`. This is ruling AU's correction of AR.
  - It expects the new cellXfs entry with `xfId="0"` and compares the whole styles part (`assert_eq!` on `style_sheet(&expected)`).
  - No non-test code is touched (numstat: only this file under crates/, 34 added, 0 deleted).
- **Probe commit (f4b249b).** It touches all 16 sources holding `fn full_style`. `git grep -l` gives 16 at d4d9eca and at fe280ff7, which matches ruling AN.
  - With the USAGE line, the header doc comment and the per-probe println format string filtered out, the added lines hash the same in all 16 files. Each file deletes 4 lines.
  - Each adds `gradient_style()`: a Linear gradient at degree 90.0 with stops at 0.0 and 1.0 in `Rgb(0xFF33_6699)` and `Rgb(0xFFDD_EEFF)`. Those are the two Rgb constants `full_style` already uses. `Pattern::None` and no pattern colours; `..Style::default()` covers the rest.
  - It calls `add_named_style(style_name, Style { font: full_style(..).font, ..Style::default() })` and a third `add_style(None, Style { named: Some(idx), .. })` on the boolean cell.
  - It prints four lines, `named_xfs`, `gradient_fill`, `named_fill` and `named_xf`, right after `full_border` in every probe. `<style name>` is the new last argument of every USAGE. This matches AN, AO, AP and AV.
  - The fullstyles probe's 11 extra lines are its header comment describing the additions, as the commit message says.
- **Changelog commit (fe280ff7).** One bullet under "### Added" in the 0.1.0 (unreleased) section, citing `specs/0042-gradient-fills-and-named-styles.md`.

Nothing else is in the diff: 20 files, 1583 added and 65 deleted, by numstat.

Notes, none of them a disagreement:
- `named_xfs` is computed as `len() as u64`, which fits the u64 rule.
- AO said "every other field None". The named style's literal takes `..Style::default()`, which AV accepts.

2. Do the logs contain every line the spec requires?

Verdict: yes, with one note.

- **Phases.**
  - gate.log: `PRUNE_PREVIOUS: yes`; P2's target "names of the prune set there (M6 part 5 P2's directory): 1 of 1; their bytes in all: 7740025302"; `rm -r` with no -f; "every kept name is still there: as-expected" (ruling AT).
  - test.log runs host and i686 with the feature sets off, read, rules, read,rules and all.
  - negative.log, size.log and run.log are present, and scan-pre.log uses the whole-word denylist form "-i -w -F".
  - Every log ends "... checks all as expected: yes".
- **Size and packaging.**
  - size.log: "grew 12488, inside the estimate of 8000 to 15000; a FINDING: no".
  - "the headroom left under the ceiling: 7304 (specs/0042 ruling BD; changes RE and RK)". The size record says "The budget figure is 451448 section bytes, 7304 under the ceiling." That is ruling BD.
  - read 976 is "inside the accepted 3000 either way (specs/0042 ruling BC)". images -484, comments 52 and charts 488 are each "inside the accepted 1500 either way" (ruling BC).
- **Ruling BB.** The band line "expected 0 to 12488 ..., accepted up to 14488; the three that link no emend: expected 0, accepted up to 2000 (specs/0042 ruling BB ...)" is followed by a judged or printed line for every probe. The size record has a "The bands" section naming ruling BB.
- **Rulings AN and AW.**
  - The cause lines name f4b249b plus d0e7b32, 029400e and 6a7644e for the sixteen probes.
  - The size record states the addition has no precedent.
  - The moved lines are checked against part 4's size log for the fullstyles, format and read probes:
    - `xfs_after 4 -> 6`, `text_style 2 -> 4` and `fills 2 -> 3`, plus `read_named 1 -> 2` on the read probe;
    - "unmoved: fonts, then fonts 2, now fonts 2";
    - the four new lines at 2, 2, 0 and 1;
    - `merges 1`;
    - "... 3 of 3 (change RH): yes".
- **Rulings AO and AP.** The size log passes the constant style name "Probe style".
- **Rulings AQ and AX.** The size log prints wc -c before and after the container for all four fixtures (2496 to 2524, 2510 to 2538, 2512 to 2540, 2781 to 2809) and "4 of 4 (ruling AX ...): yes".
- **Ruling BA.** "right after cellXfs, last among its children (ruling BA ...): yes".
- **Rulings AS and AY.**
  - test.log: "M6 part 4 P3's rows: 834; this listing's rows: 844; M6 part 4 P3's not here: 3; this listing's not in M6 part 4 P3's: 13; equal by key: 831".
  - The three rows, each with the heuristic's commit beside the diff's: 0b3e46f against d0e7b32, f14885f against 6a7644e, and 35cd499 against 6a7644e. The other new rows are 8 at 6a7644e and 2 at d0e7b32.
  - The API record has the section "added-by: the rows the diff dates otherwise (ruling AY)".
- **Harness runs and P3's run log.**
  - Both samples are rerun with the size log's release binaries, copied into <work>/bin. Each gives "rows compared 27; identical, output and line, by sha256: 27" on each label, against P2's and P1's runs.
  - `gradients_before` is "6 in 2 workbooks" and `custom_before` is "36 in 23 workbooks".
  - Every fixture run line says "lo_in and lo ok".
  - The open record is installed with "lo_in ok 2225 ... lo ok 2226".
  - negative.log carries "M6 part 5 P3 adds no message and no binary path".

Note (not a failure): the gate's PLANNING_SPEC input line says "rulings AN to BA", while the spec it pins holds AN to BF. The README names this as a departure, and the hash check itself passes.

3. Does every number and quoted string in the draft appear in a primary?

Verdict: yes, with exceptions.

These match a primary, and I re-derived some with read-only git:
- origin/main f630f67, and the 15 local commits after it (git log origin/main..fe280ff7).
- The four full hashes and subjects.
- The numstat rows: 4/0, 1/1, 34/0, 221/0, and 1323/64 over 16 files.
- The directory counts: running the draft's own command prints exactly 2, 1, 1 and 16.
- Every log sha256 and last line, and every record sha256.
- The run-ended times 11:50:02Z, 11:52:22Z, 11:52:40Z, 11:58:15Z and 14:41:35Z.
- The scan pre-check's hash 6c35e5b6..., "total hits: 0", "records scanned: 8", "structural checks failed: 0" and "RESULT: PASS".
- Every gate, test, negative, size and run figure the draft quotes (I checked each against its log line).
- The API record's 76382 bytes.
- The fixture ids and byte counts: b9335e28dcc0 at 2402, and 3137dc555cde at 27230.
- The run directories `<corpus>/runs/2026-10-23-bf07f86-namedstyle` and `<corpus>/runs/2026-10-22-029400e-gradient`.
- The keep figures: "passed 1448; failed 0", "pairs 2237", 2 and 0, 0, lo_in 2225 and lo 2226.

Each statement the draft marks as the session's is carried by the README: nothing pushed, outer stderr 0 bytes, first of its kind, the dry run d1, gen1 and gen2, and the error list. The README is itself a session statement; no log prints these.

Exceptions:
- **(a) S1 to S4 described too narrowly.** The draft calls S1 to S4 (rulings BB to BE) "the size fork's readings of the bands, the read cost and the ceiling". That describes S1 to S3 only. S4 (ruling BE) is the rules probe's reader listing `cellStyles` and the forks' departures, which the draft does not mention.
- **(b) "before the dry run" not stated.** "The size generator regenerated (gen2) before the dry run" is not stated in that order by any primary. README item 4 says gen1 was superseded and that "No log ran a superseded script". Item 5 describes the dry run but does not say which generation it used.
- **(c) "the one derive change" holds only for existing types.** For `Fill`, "the one derive change" is accurate for the types that already existed. The log also shows three new derive lines, for Gradient, GradientKind and Stop, as "now not then".
- **(d) Unverifiable statements.** "No log prints it" for the numstat and the directory counts is true; I reproduced both from git. "Nothing pushed" rests on the local origin/main ref and the session's statement; no primary proves it beyond that ref.

No disagreement on substance.