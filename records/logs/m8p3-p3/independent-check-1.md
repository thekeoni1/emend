# M8 part 3 P3: independent check 1

The answer of a fresh subagent given only these materials, under
<scratch>/check1, and the three questions of CLAUDE.md ("Independent
check before reporting"), with the P3 points of specs/0049 and
specs/0045 named in its brief:
- the working specs/0049 (12fe9f11c20ba40465477670a26dd5241c911fbec127ed46515676fd25c91a26), with rulings K and L, and specs/0045's P3 section and rulings AB to AH;
- planning's P3 brief as given by message (planning-brief.txt);
- git show of the four commits 43b8851, be6cdb1, 41203c5 and 7ea64ba;
- README.md and CHANGELOG.md at the phase commit, and the two records they cite for this phase's figures;
- gate.log, negative.log, run.log, the scan's pre-check output (505d41ed3abf145be337fba71e0e428eff83aac7874f4d1cc7ced6d376710b19) and <phase>/logged/driver.out;
- the logs README as it stood before the check (sha256 6fd70921c74537942cbd1c7f1f44244108f56a1aca25e5bd0e16d9dac18f3b87) and the draft report (4abe6f92095e7a9bf7c55967f23d23fc399ae1077b6caf31ca6425fe77830487).

The answer was extracted from the subagent's transcript by script into
<phase>/check1-answer.txt (sha256 9f18892836613c3eed7498dab9c4718433d3ff6f9ae07e951c1f8f7f73fa0250). It appears below verbatim. The
README's section "After the independent check" answers its one point.

root: <scratch> stands for <phase>, as in the gate and run logs
root: <phase> stands for <home>/.cache/emend-build/m8p3-p3, the phase's directory
root: <home> stands for the home directory of the account the runs ran under

---

The harness put a CLAUDE.md, a memory index, a git status and user details into my context. I did not use any of them as evidence.

1. Yes. The four commits and the phase's work do what the spec says for P3 with rulings J, K and L, and nothing else.
- **Version and lock:** commit-be6cdb1.diff lines 16-40 change only `crates/emend/Cargo.toml` (`version = "0.1.0"` to `"0.1.1"`) and the emend entry's version line in Cargo.lock. gate.log 10525, 10573-10575 and 10585 confirm that only the code commit touches Cargo.lock, on that one line.
- **CHANGELOG heading and entries:** commit-41203c5.diff 29-30 changes "## Unreleased" to "## 0.1.1 (unreleased)", which is ruling J's form. CHANGELOG.md 34-39 holds the Changed entry. After commit-7ea64ba.diff 25-34 it reads 456248, 2504 and 458752, cites `records/logs/m8p3-p3/gate.log`, and names part 2's 456264 with that record, as ruling K requires. CHANGELOG.md 41-49 is the Measured line with the Excel sentence. CHANGELOG.md 51-55 is ruling F's one line.
- **README:** README-repo.md 70 is the size sentence in ruling K's form (commit-7ea64ba.diff 47). README-repo.md 80 is the Excel sentence. README-repo.md 100-103 adds ruling F's sentence to the limits. run.log 2936-2938 shows nothing else was added to the limits.
- **Figures against their primaries:**
  - The Excel sentence (4848, 311, 1, 1905, 363, 1907, 361) equals the totals row in excel-check-m8-2026-11-12.md 18-20. "eight samples ... over 39" is at line 22 of that record, and the repair is ruling E's limit (spec-0049-working.md 384-387, 414-416).
  - 456264, 2488 and 458752 are in size-m8p2-2026-11-10-8065795.md 328, 384 and 391.
  - 989464 and 456248 are in gate.log 12158, 12160, 12165-12166 and 12171-12172.
  - The arithmetic holds: 458752 - 456248 = 2504; 989464 - 533216 = 456248; 989480 - 533216 = 456264.
- **0.1.0 entry untouched:** neither docs diff has a hunk at or below the "## 0.1.0 (2026-10-29)" line (commit-41203c5.diff 61; commit-7ea64ba.diff 21-34). run.log 2943-2944 shows matching sha256.
- **Doc comment:** left unchanged (run.log 2975, 3009). commit-41203c5.diff 14-16 says why.
- **Gate prune:** the prune set is 1 name, `target`, in m8p3-p2 (gate.log 191-192), removed by the `rm -r` form (233-234), with the 26 kept names checked (193, 236) and the form's control (185-190). gate.log 12171-12174 is ruling K's requirement with its controls.
- **Packaging dry run:** run.log 2830-2831 and 2846 show the archive's publish dry run without the dirty flag, per ruling L.
- **Doc commit:** commit-43b8851.diff holds only the spec (rulings H to J, matching spec-0049-working.md 431-506) and ROADMAP.md, with the Co-Authored-By line only.

2. Yes. Every required line is there:
- **Packaging dry run and counts:** run.log 2743-2843 and 2845-2876. This covers the archive outside a checkout (2828-2829), lists of 76 and 75 differing by `.cargo_vcs_info.json` (2835-2838), `template/_rels/.rels` once in each (2839), exit 0, "Packaged 75 files", the one aborting-upload warning (2868-2873), and v0.1.1 (2841, 2874-2876).
- **Token variables absent:** run.log 2735-2739, with controls. The credentials file's existence check is at 2741.
- **Heading form:** run.log 2922-2926, with the other-version control at 2925.
- **Texts' figures with planted and swapped controls:** for the Excel sentence, run.log 2882-2897 and negative.log 5759-5770. For the size sentence, run.log 2900-2919, with planted and swapped controls at 2918-2919.
- **Negative's packaging cases:** negative.log 5642-5683, cases (a), (b) and (c) at 5677-5681.
- **Citation rule:** run.log 2947-2972. Spec-only and version units are listed and not refused (2955-2956, 2962-2964), per ruling L.
- **Ruling K's gate requirement:** gate.log 12171-12174.
- **The run's check of the gate log:** run.log 2903-2907 and 2950-2953.
- **Others:** the feature list is at run.log 2977-2992. The FINDING lines are listed in driver.out 16-20 and run.log 3012-3013. The scan pre-check has "record files expected: 0" (scan-pre-check.log 112) and RESULT: PASS (7180).

No required line is missing.

3. Yes, with no mismatch. These figures and line numbers were checked and match their primaries:
- **README.md:** bytes, lines and sha256 of the three logs; start and end times (gate.log 385 and 12771, negative.log 105 and 5801, run.log 62 and 3027); gate.log 183-236, 185-190, 193, 232, 234, 236, 11724, 12158, 12165, 12167, 12172-12174 and 12138; negative.log 97-98, 5677, 5678, 5681, 5765, 5770 and 5778; run.log 12, 72, 2735, 2736, 2741, 2746, 2833, 2841, 2868, 2874, 2882, 2887, 2891, 2905, 2908, 2913, 2917, 2922, 2923, 2933, 2944, 2950, 2959, 2967, 2975, 3012 and 3013; the change ranges A107 to W107, NA68 to NJ68 and XU to XZ / YL to YN; the doc commit's blob sha256 values; driver.out's times, hashes and "its names: 26".
- **report-final-draft.md:** every hash, figure and line number in lines 6-17. "Packaged N files": 75 is at run.log 2868 and "(b) 75 files" at negative.log 5681.

Not verifiable from these files, though none is contradicted. README.md declares each one's source as outside this directory.
- The working directory name `tmp.4HXaSszNhA` (README.md 153, 173).
- d1's gate.log line 12080 (README.md 157).
- "three dry-layout lines" for d2's pre-check (README.md 159).
- The file name `<phase>/scan/pre-check-1.out` (report line 20).

Two small points:
- README.md 17 says `<scratch>` stands for `<phase>` "as in the gate and run logs". negative.log uses `<scratch>` differently: line 118 maps `<scratch>/tmp` to `<build>/tmp`, which is `<phase>/negative/tmp`. The scan reads README root lines as declarations for all three logs (scan-pre-check.log near its end), so this line misdescribes the negative's `<scratch>`.
- run.log 70 defines `<scratch>` as `<workroot>`, which is `<phase>` only in the logged run. That is consistent with the logged logs.

Sensitive content: no file holds a registry token value. run.log 2173 and 2175 hold only the fixture value "planted" and an empty value. No file holds a private workbook's content or name. The only .xlsx names are the committed fixture `tests/fixtures/openpyxl-ops.xlsx` and generic placeholders such as in.xlsx, out.xlsx and x.xlsx. No home-directory path appears outside a placeholder except scan-pre-check.log 5661, where the split control fixture builds '/home/' + 'someone/...' at run time. The commit diffs carry the author's GitHub noreply address in their Author lines (commit-*.diff line 2), which is git's own header and none of the three categories.
