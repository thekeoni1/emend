Check 1, M6 part 6 P2 (specs/0043 "P2: the runs", rulings H to K)

1. Does the diff do what the spec says and nothing else, and does change LC do what ruling K says and nothing else?

Verdict: yes with exceptions. The exceptions are about wording, not about what the change does.

The diff:
- `git diff 65d4c5a d88f735` matches diff.patch byte for byte. Both hash to sha256 ae336f09e6dc333b2da1962f120e452a96cc85faaba314deb0172f0ab4579a45.
- The range holds one commit, d88f7359f323da40da287d8f16ad9e1a8b8b9bd1, whose parent is 65d4c5a.
- The numstat is ROADMAP.md 1/1 and specs/0043-measurement-and-close.md 138/0. Nothing under crates/, tools/ or scripts/ changes.
- The ROADMAP change only sets the P1 row to DONE. The spec change adds rulings B to G and "Planning verification of P1". These are planning docs, so there is no code commit, as ruling H R1 (a) says.
- The commit message matches commit-messages.txt. Its two hashes match the committed blobs: `git show d88f735:<file> | sha256sum` gives a18af81d... for the spec and 0aa530c7... for ROADMAP.md. The message ends with the Co-Authored-By line alone.
- The working tree's spec hashes to ad36059f..., the brief's hash. Its only uncommitted change is the 88-line block of rulings H to K (`git diff --stat d88f735`: 88 insertions).

Change LC:
- It adds `declared_with` and `undeclared_with`. It reads `<logdir>/README.md`, or /dev/null when that file is absent. For each sibling log it uses the log's own root lines together with the README's. The script's own source (`$0`) still uses `undeclared`, its own root lines alone. Ruling K asks for exactly this.
- K requires one control: a token declared in neither the log nor the README, refused. LC has it (scan-pre.log line 8503, count=1 expected=1).
- Exception (a): LC also adds a second control, an acceptance case (line 8502, count=0 expected=0). K does not name this one. It is a control of the same change and only adds checking.
- The other edits are housekeeping: the docstring, the label guard (KZ, LA, LB, LC), the change-list entry, the label-count loop and the required-names list. I found nothing outside the change.
- Exception (b): the line printed for the script's own source now reads "declared by root lines (the log's and the logs README's, change LC)" (line 8514). In fact the code checks that source against its own root lines alone. The verdict is correct but the label is wrong for that one line.
- Doubt (c): ruling H R7 says the gate prunes "in the single-root form". The README records a fork departure: the gate uses P1's prune_dir for one directory instead. Rulings H to K do not rule on that departure. The draft cites "ruling H, R7" for the prune without mentioning it.

2. Do the logs contain every line the spec requires?

Verdict: yes.

The four logs' hashes match the draft and the copies under records/logs/m6p6-p2. Each ends with "<name> log checks all as expected: yes". The details:

- **The sixteen ops over the corpus:** run.log has a section for each of the sixteen: emend host, emend i686, openpyxl with --memory 4096 --timeout 600, and the record pairs. chart-analytic uses --chart-axis-sample and named-style uses --namedstyle-sample.
- **The scale tiers:** tiers a and b run all sixteen ops; a runs the open check and b does not. Tiers c and d run roundtrip only, and tier c uses the host xlsx-diff. The tier c openpyxl roundtrip ended as an outcome "cap 1" (line 14179). That is an outcome, not a rerun, as R9 says.
- **The open check:** the keep is compared against records/m6p5-open-2026-10-23-fe280ff.txt (lines 7932-7947). Rows are 2237 and 2237. "pairs in the keep and not in the record: 0; in the record and not in the keep: 0". "the new pairs expected: 0 (change NWF)".
- **The comparison record and its derivation:** lines 14530-14534 read corpus 48/48/0, openpyxl detail 16/16/0, scale 102/102/0, then "derived equal: yes". The check line ends ": yes" with a planted control. The record is installed with sha256 56673747... (line 14556), and that matches the file under records/.
- **203 record files:** line 14565 is "the whole run installed 203 record files, the set specs/0043 ruling H (R5) names (change RR): yes", with an 11-op control. `git status --untracked-files=all records`, leaving out records/logs, lists 203 files.
- **The rerun line:** run.log line 69, "rerun: finding F3 (specs/0043 ruling J): ...". RERUN_FILES names the three files under <phase>/superseded/run-refused-1.
- **The gate's prune of P1's target:** gate.log lines 246-252 show 6167760653 bytes, 1 of 1, `rm -r` exit 0, gone and kept checks as-expected.
- **The gate's other lines:** A86's docs-only range line is at line 8213. check.sh's nine PASS lines are at lines 8980-8991. The test sets on host and i686 are at lines 9190-9541. The size figures are at line 9600.
- **The test:** O and T1 lines are at 3259 and 3306.
- **The negative:** C0, R0 and N1 to N4 are at lines 6333-6444, and NC58/NC59 at line 6276.
- **Ruling K's control in the scan pre-check:** lines 8502-8503. Line 8505 says the logs README was read. Line 8512 shows run.log's <superseded> declared, undeclared 0. The log ends "total hits: 0", "structural checks failed: 0", "RESULT: PASS".
- **Hygiene:** I found no "/home/" in the logs or the README except inside the scripts' own pattern text, and no non-ASCII byte.

3. Does every number and quoted string in the draft report appear in a primary?

Verdict: yes with exceptions.

Checked and found:
- The PLANNING_SPEC hashes d761db90 (gate, test, negative), 4a971987 (run) and ad36059f (scan).
- The five log and README hashes, the full commit hash and subject, the base subject, and the numstat.
- 6167760653. The test-set figures 538, 711, 575, 753, 756, 818 and 1199. The size figures 1151328, 984664 and 451448.
- 109 and 34 ok lines. 2237 and 2237. The comparison record's hash. 203.
- The scan-pre hash c569c48d. On disk, <phase>/scan-pre/scan.log has that hash.
- The quotes "checks all as expected: yes", "total hits: 0", "structural checks failed: 0" and "RESULT: PASS".
- run.log's <superseded> appears only on lines 4 and 69, as the draft says.
- origin/main: `git rev-parse origin/main` gives ee78ffa. This is the local remote-tracking ref only; I ran no network command. f15a11b and d88f735 are ahead of it.

Exceptions:
- (a) The F4 quote "every placeholder records/logs/m6p6-p2/run.log prints is declared by a root line: no" and the first pre-check's "RESULT: FAIL" are not in check1/. The README carries them as the session's statement. I also read <phase>/superseded/scan-1/scan-pre/scan.log, which is outside the listed primaries. Its sha256 is 61ab485d..., as the README says. The quote is its line 8486, and it ends "structural checks failed: 1" and "RESULT: FAIL".
- (b) "the run exited 2 at its pre-flight": no primary carries the exit code. The README states it, and ruling J records the refusal without a code.
- (c) "Both ruled by planning before any fix": the timing is not provable from primaries. Rulings J and K record planning's own verification of the state before each fix, and that supports the statement.
- (d) "pairs new 0 and gone 0" and "every row derived again: yes" are not quoted strings. They paraphrase the run.log lines above, and their content matches.
- (e) The README's first paragraph says "Planning's rulings H to J ... are in the working specs/0043". The draft says H to K, which matches the working spec. The README names ruling K later in the same paragraph, so this is an inconsistency in the README, not an error in the draft.
- (f) The generator and script hashes in the README's table are the session's statements; no log prints them. The scan generator 5e2cf0dd... and 30ec892e... and the scan script c63cb0aa... and c99f8315... match the files on disk under <phase>. The scan-gen-LC.diff header names those two generator paths.
- (g) The draft's gate bullet on the prune cites ruling H R7 without the prune_dir departure (see 1(c)).

The draft states nothing as the session's that the README does not carry.
