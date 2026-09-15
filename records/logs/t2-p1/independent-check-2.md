# T2 P1 independent check 2

- Checked commits: e3dd52e777fe16fb58e5bf7cff6e562f2c332673 (phase commit 1)
  and 43188208156df77ff9a8fc8b982649ab5b4c818a (phase commit 2, the fix for
  check 1 gap 1)
- HEAD when the check ran: 40e32c9c56d1a65c21bec81c5cab71235bc1d75c (the
  record commit with the four logs rerun at phase commit 2, local only)
- This is the second and last check of the phase (specs/0001-bootstrap.md,
  "How this task runs": at most two checks per phase)
- Checker: a fresh general-purpose subagent (not a fork), given only the
  inputs below, asked to answer in plain ASCII, and told to run read-only
  commands only, never to run the two scripts or check.sh, and never to read
  the private corpus directory
- Inputs, with sha256:
  - specs/0005-corpus.md
    8224e7aaf15434203bcc81b82551f72b27cb2b565ec748b9ceb03aec4c885131
  - specs/0001-bootstrap.md
    5b7ed9aa84e89ec895f3cbd4065ef38c5d2d1cf2a1d7c2a0b00be4c7608e6f4b
  - phase diff 1, `git diff 24d4a16..e3dd52e`, saved as <scratch>/t2p1-diff.txt
    251d6eae2ff11c2ff5690a0219ff680a76e0d43559a0fc3cef5230ad5a7fbb59
  - phase diff 2, `git diff 17c6c4b..4318820`, saved as <scratch>/t2p1-diff2.txt
    6dee609fbba10bbd96a0772187c187b4dfba0c27131156ece1eb78783993fb45
  - records/logs/t2-p1/independent-check-1.md
    3be0d7d72cb21767cc9d6b51a754dfc9bdf306a155d1c705f9a6000f1b912a44
  - draft report 2, <scratch>/t2p1-report-draft2.md
    426b2cd71a3eb220fa6d095ea7c566bdb11577dc16f80b76ddce7231fc1ad00f
  - records/logs/t2-p1/gate.log
    7528b700fcd671a8a12b5ccc5827b2be50a4de1baa7aa22c666f8ee5191190be
  - records/logs/t2-p1/positive.log
    dca8ddf9a04377a61f96477e005e887d93eaffce3629b5e901d1ee56b8fb55b3
  - records/logs/t2-p1/negative.log
    c4c3803526fc1c77a0f1fd70b234bf24e09e47a9d1a7f71d568a60251969c300
  - records/logs/t2-p1/scan.log
    e74ef8ceb8726c971ad1d8f886ec0de65310372bbf59685a6fa68c8a86e4d172
- Answer: the checker's final message, extracted verbatim from its transcript
  with python, without printing the transcript, and concatenated to this
  header by the same script. The extracted answer is 17799 bytes, 97 lines,
  sha256
  dbf4083403182c51ac16d02394e97606910dbeb10da3ab054d45eb433e5217ee, and holds
  0 non-ASCII bytes, so no replacement was needed. It holds no `&lt;`,
  `&gt;`, `&amp;` or `&quot;`: the task notification delivers the answer
  HTML-escaped, the transcript text is not, and 4 lines carry a literal `<`
  and 5 a literal `>`.
- Privacy-scanned before the commit with the same command forms as scan.log
  (P6 patterns from scripts/privacy-patterns.txt, the hostname, the private
  denylist, addresses outside the allowlist, a Claude-Session line, U+2014,
  control bytes): every count 0 on the answer. The controls fired: 6 P6
  patterns, the hostname, one address, one Claude-Session line, one U+2014
  and all 7 denylist entries (session output).

---
Harness context notice: my context also contained CLAUDE.md, an auto-memory index, a git status snapshot, git attribution text and the user's email address. I say so as required and used none of it as evidence. Everything below cites the spec, the two diffs, the four committed logs, check 1's committed answer, or git objects I read read-only in the repository.

# 1. Does the diff do what the spec says and nothing else?

## Phase diff 2 (<scratch>/t2p1-diff2.txt)

It is one hunk in `scripts/corpus-add.py`: a three-line comment and a `try: field.encode("utf-8") / except UnicodeEncodeError: die("a text field is not valid UTF-8")`, inside the existing loop over `(pver, note, os.path.basename(src))`. Seven insertions, one file, nothing else (gate.log:232-237; scan.log:347-350). That is exactly what check 1's gap 1 asked for and nothing more.

The refusal does happen before anything is copied. In the blob at the phase commit the loop sits immediately after tag validation and before the sha256 pass, the manifest read, the duplicate checks, `read_app`, and the `os.umask` block that contains `os.makedirs`, `shutil.copyfile` and the append (the ordering is visible in the printed diff, gate.log:361-400: the new check lands in the `@@ -81,6 +111,13 @@` hunk, the copy in the `@@ -93,18 +130,23 @@` hunk and after). N13 confirms it empirically: exit 1, exactly the refusal line, manifest and file count unchanged, and `corpus/` still holding 2 files (negative.log:689-695).

The fix also closes the whole reachable class, not just the tested field. Every other column written to the row is either derived (`fid`, `sha`, `size`), validated against an ASCII fixed list (`ext`, `producer`, `source`, `tags`), a generated date, or `app`, which comes from `ET.fromstring` and so cannot carry surrogates: a part that is not well-formed or not decodable is refused earlier with `docProps/app.xml is not well-formed` (gate.log:300-303). So after this commit no reachable input raises `UnicodeEncodeError` at the append.

## Phase diff 1 (<scratch>/t2p1-diff.txt)

I checked check 1's findings against the diff and the blobs and agree with all of them, including the bullet-by-bullet mapping to the spec's P1 list, the completeness of the report's change list against the hunks (I redid that comparison on the full diff printed at gate.log:261-410 and every hunk is named by the report, and the report names nothing absent from the diff), and the print-path audit. I have no disagreement to record.

Two things to add that neither check 1 nor the report names:

- A committed inaccuracy. `scripts/corpus-coverage.py`'s own docstring says "Any failure prints one `FAIL <reason>` line whose reason names a manifest line number, never a value, and exits 1" (<scratch>/t2p1-diff.txt, the new file's docstring). Six reasons carry no line number: `manifest missing`, `manifest cannot be read`, `manifest is not UTF-8`, `manifest does not end with a newline`, `manifest header mismatch` (negative.log:657) and `unexpected error <Class>`, plus `HOME is not set`. The report corrected this claim in its own prose after check 1 gap 2, but the script still asserts it, and a later reader will read the script.
- Why the catch-all matters. `shutil.copyfile`'s own `OSError` message would name the destination, that is a corpus path. The `except Exception` handler that prints only `error: <ExceptionClass>` is what keeps that out of stderr. It is in the diff and the report lists it under change 2, but not as the thing that closes a corpus-path leak. Worth saying explicitly, because removing that handler later would reopen a privacy path, not just a tidiness one.

Beyond that I re-traced every print site in both scripts at the phase commit and found no path that can print an original name, a note, a `producer_version`, an `app` value, a corpus path or file content. The residual check 1 named stands: a `BaseException` that is not an `Exception` (KeyboardInterrupt) still prints a traceback naming the script's own path and source lines, with no column value and no corpus path.

Correctness residue the report does name: the copy still precedes the append, so an I/O failure at the append would still leave a corpus file with no row. No logged case, and none is easy to write.

# 2. Do the logs contain every line the spec requires?

Every item on the spec's test list is present in the reruns:

- generated xlsx added, expected id, one row, mode 600, right hash: positive.log:496-505 (`added 7eb89f523115 xlsx 643 libreoffice plain,strings` at 499, expected id from sha256sum at 501, the four checks at 502-505).
- the same file again, refused, manifest unchanged: negative.log:517-524.
- OLE2 signature, refused, nothing added: negative.log:526-533.
- unknown tag: negative.log:535-542. Unknown producer: 544-551. Upper-case `.XLSX` accepted and stored as `xlsx`: positive.log:507-518, including "no file with an upper-case extension in corpus/" at 518. A `.csv`: negative.log:553-560.
- a sentinel in the original name never appears in any output: sentinels are built for the file names, `producer_version` and the note (positive.log:5, 74-78), the counter controls fire (positive.log:466-474, negative.log:466-474), the scratch manifest holds them as a positive control (positive.log:623-624, negative.log:699-700), and over all 12 and all 40 captured outputs the counts are 0 sentinels, 0 scratch paths, 0 control bytes, 0 non-ASCII bytes (positive.log:622-627, negative.log:698-703).
- coverage listing every tag with the right counts: positive.log:547-582, against a 30-line expectation the script builds independently from the fixed tag list (positive.log:303-323).
- `--verify` failing after one file is modified in place: negative.log:610-651 (byte flipped at offset 10, same size confirmed at 612, plain coverage still exit 0 at 645, `FAIL line 3: sha256 mismatch` at 649 with the exit-1 check at 651).

Rules for every phase:

- Line 1 names the commit and the HEAD in all four (gate.log:1, positive.log:1, negative.log:1, scan.log:1).
- The script is printed into each log and each starts `#!/usr/bin/env bash`: gate.log:11-185, positive.log:8-463, negative.log:8-463, scan.log:6-242.
- Rerun header naming the record commit replaced: line 3 of every log names record commit f692dbf and what changed since check 1.
- `/usr/bin/grep` called explicitly under `LC_ALL=C`: gate.log:44, 62, 73, 102, 132, 136; case logs 93-94; scan.log:19, 49-57. Non-ASCII as `-P '[^\x00-\x7F]'` (scan.log:49) and U+2014 as its UTF-8 bytes via `printf` (scan.log:25, 51); no `\x{2014}` anywhere.
- Positive controls in the same command form: scan.log:253-310, covering every form, one control per denylist entry, and a "bare pattern names must not match" negative control at 284; gate.log:429 (home path) and 469-470 (control bytes); case logs 466-474.
- Exit codes: scan.log:73-78 treats rc >= 2 as an error rather than zero hits; totals 0 hits, 0 errors, 0 structural failures, RESULT: PASS at scan.log:549-553.
- No control bytes: gate.log:468-472, positive.log:627, negative.log:703, and scan.log's `f_ctrl` 0 on every diff file, message and log. I counted independently over the four committed logs: 0 control bytes other than tab/LF/CR and 0 non-ASCII lines in each.
- Log privacy: `f_host`, `f_deny`, `f_email`, `f_session` are 0 on all ten diff files, all six messages and the three other logs (scan.log:364-546). Independently, I found 0 hostname matches, 0 U+2014 and 0 `Claude-Session:` lines in all four logs including scan.log. The only home-path-shaped strings anywhere are inside printed control constructions (gate.log:132, 135, 136 and scan.log:142); there is no session-directory string in any log.
- Negative tests in a scratch copy: the scratch corpus, workbooks, broken copies and the wrapper repository are all under `mktemp` with a cleanup trap (negative.log:64-65, 224, 378-379), and the tree is clean afterwards (negative.log:704).
- Compound checks inside one function: now honoured. I grepped all four logs for a `$(yn ... && ...)` or `$(is ... && ...)` form and found none. The only remaining `eval` is `eval "ID_$c=$id"` (positive.log:188, negative.log:188), an assignment, not a check.

Check 1's gaps 5, 7 and 8:

- Gap 5 is closed. The three compounds became `all_stdlib` (gate.log:60-62, used at 131), `not_a_repo` (positive.log:100-101, used at 264) and `cov_files_two` (negative.log:108-111, used at 394). Nothing of that form is left.
- Gap 7 is closed. The doc commit's stat is logged (gate.log:196-203, 143 insertions and 30 deletions at 202) and the import commit's file mode is logged (gate.log:209, `100755 scripts/corpus-add.py`), along with per-commit stats and modes for every commit in the range.
- Gap 8 is substantially closed. Both case logs copy the scripts to a scratch directory and check each copy against its blob at the phase commit before running anything (positive.log:267-276 and 490-493, negative.log:490-493). The residue: the harness script itself still runs with the repository as its working directory and takes the copies from the working tree rather than from a clone or worktree. Since the copies are pinned to the blob sha and nothing is written in the tree, this satisfies what the rule protects, but it is not literally a scratch clone.

N13 really does exercise the fix. The argument carries one byte 0xff, proved by a control that counts it without printing it (negative.log:686-687). Python decodes argv with surrogateescape, so the note reaches the new check as a surrogate, passes the tab/CR/LF test, and is refused by the encode test: exit 1, stderr exactly `refused: a text field is not valid UTF-8`, nothing added, and no orphan file, the corpus count still 2 (negative.log:689-695). Before the fix that same argument would have passed validation and raised at the append, after the copy, which is precisely the path check 1 described. One caveat: N13 covers the note only. The file name and `producer_version` go through the same loop but have no case of their own.

One residue in the scan: a log cannot scan itself, so the committed scan.log is covered by no committed primary. The `scan.log` entry it scans as a diff file (scan.log:409, sha256 7e1f4dc6...) is the first-run log committed in f692dbf, not this rerun. The gate, positive and negative logs it scans in the working tree (scan.log:520, 529, 538) do carry the sha256 of the committed reruns, which I verified by hashing the committed files: 7528b700..., dca8ddf9..., c4c38035....

# 3. Does every number and quoted string in the draft report appear in a primary?

Verified against primaries and correct: the seven commit hashes and their order; 45dd9e3 touching exactly the three doc files with 143 insertions and 30 deletions (gate.log:196-203, scan.log:321-326) and their sha256 5312f3af..., 8224e7aa..., 7d4b6482... (scan.log:364, 436, 445); no later commit touching them (scan.log:313-351); the import record's three-way equality and 4571 bytes, mode 600 (gate.log:252-255), which I corroborated as far as a checker can by hashing the blob at e8087f6 (4571 bytes, 30c66ee9...); e8087f6 adding only that file (gate.log:256-258) with git mode 100755 (gate.log:209); e3dd52e's stat (gate.log:210-216); 40e32c9's 388 insertions and 86 deletions (git); 5ab35030... / 6304 and 613799ba... / 5955 with mode 100755 (gate.log:415, 416, 422, 423, 237), all four recomputed from git; the full change diff at gate.log:261-410; the log line counts 476, 630, 706, 553; the eight PASS lines, exit 0, 0 stderr bytes (gate.log:450-466); empty status (gate.log:474); "gate log checks all as expected: yes" (gate.log:476); ids 7eb89f523115, d54858d6b9e1, 2ac16f2dacc2, 742a0c4f98ff and sizes 643 and 315; `uncovered 18: ...` and `single-producer 4: strings,formulas,charts,large` (positive.log:579-580); every N1 to N13 citation, which I checked line by line and all land correctly; `FAIL line 3: sha256 mismatch` (negative.log:649); `FAIL manifest header mismatch` (negative.log:657); 12 and 40 captured outputs; the scan.log citations 311, 454, 456-517, 519-547, 549-553, 373-417; the four remaining `corpus-add.sh` / `corpus-coverage.sh` mentions in the spec (specs/0005-corpus.md:229, 253, 257, 265).

The corrections the report claims after check 1:

- The FAIL line-number claim (gap 2): now accurate. The report states the rule for row failures with five correct citations (negative.log:649, 663, 669, 675, 681) and names the header exception with the right line (negative.log:657). Closed.
- The "tool crate" quote (gap 3): removed. The report now quotes the gate line and adds separately that the argument list also covers `tools` and `.github`, which is true (gate.log:153). One slip remains: the quoted line is given as "no gate file, Cargo file or workflow changed in 24d4a16..4318820: yes", while gate.log:447 spells both commits in full 40-hex form. Presented as the line, it is an abbreviation, not a quote.
- The USAGE citation (gap 3): now points at gate.log:431-432, which is the effect of the fixed USAGE string, with the change itself left to the printed diff (the constant at gate.log:286, the substitution at gate.log:323-324). Accurate as worded.
- The trailing-space counts (gap 3): now exact. I counted 7 `command:` lines in positive.log with 6 ending in a space, and 21 in negative.log with 19 ending in a space; the two exceptions are the gen.py line (positive.log:477, negative.log:477) and the N13 line (negative.log:688), exactly as the report says. Closed.
- The GIT_DIR citation: positive.log:533-544 and 534-544 both land on the P4 block (heading 533, control 534, run 535-539, checks 541-544). Closed.
- The P5/P6 citations: 550-580 and 587-617 each begin on the `stdout (30 lines):` header line, so the 30 stdout lines are 551-580 and 588-617. Off by one at the start, harmless.

Still imprecise or unsupported:

1. "Its message names sha256 30c66ee9... (gate.log:259)". gate.log:259 says only "the import commit's message names the sha256: yes"; the value is at gate.log:252-254. The claim is true (I read the commit message: it names that sha256 and mode 755) but the citation does not carry the quoted string.
2. The gate.log:447 quote abbreviates both commit hashes, as above.
3. "`--verify` prints `FAIL line 3: sha256 mismatch` with exit 1 (649-651)". The literal `exit: 1` is at negative.log:647; 649 is the FAIL line and 651 the check that asserts exit 1. The range supports the claim only through the check line.
4. "e8087f6: scripts/corpus-add.py is the corpus directory's tools/corpus-add.py byte for byte" rests on gate.log:252-255 alone, as the report itself says. I can confirm the blob is 4571 bytes and hashes to the recorded value, so the record is self-consistent, but the private side is outside what a checker may read. Check 1 gap 6 stays open for planning.
5. "That these bytes are planning's own text is session output" and "Planning can confirm against its own copy": correctly labelled, supported by nothing I can read, and appropriately left to planning.

Items marked "(session output)", and whether any could now have been logged:

- The doc-commit stat and the import file mode are no longer session output; both are logged (gate.log:196-203, 209). Gap 7 closed.
- "40e32c9 ... 388 insertions, 86 deletions": in git, verified, and it could not have been logged, since the logs predate that commit.
- The scan.log self-check is still session output. It is a repeatable command with controls and could be a committed primary. I reproduced it: the committed scan.log holds 0 control bytes, 0 non-ASCII lines, 0 U+2014 lines, 0 hostname matches and 0 `Claude-Session:` lines. This remains the one "session output" item that most clearly belongs in a primary, and it is the same item check 1 flagged.
- The throwaway dry run that echoed a scratch path to the terminal: unloggable by nature, and recording it is right.

# Gaps, most important first

1. `scripts/corpus-coverage.py`'s docstring still asserts that every `FAIL` reason names a manifest line number; seven reasons do not, including `manifest header mismatch`, which the logs themselves show (negative.log:657). The report fixed its own prose but not the committed script, and names this nowhere.
2. The copy still precedes the manifest append. The reachable trigger check 1 found is closed and covered by N13, but an I/O failure at the append would still leave an orphan corpus file, with no logged case. The report names it as a residue; planning may want a ruling on whether a temporary-file-then-rename is worth it.
3. N13 exercises the note only; the file name and `producer_version` take the identical code path with no case. Same family as the report's own list of beyond-spec behaviours with no logged case (malformed app.xml, the manifest-newline refusals, coverage's size comparison and shape checks, `uncovered 0`, `single-producer 0`).
4. Citation slips that survive: gate.log:259 cited for the import sha256 value, which is at 252-254; the gate.log:447 quote abbreviating both hashes; the N11b exit-1 literal outside the cited range; the P5/P6 ranges starting on the `stdout (30 lines):` header.
5. The committed scan.log is covered by no committed primary (a log cannot scan itself, and the scan.log blob it scans at scan.log:409 is the first-run log). The self-check is still session output although it is repeatable and controllable.
6. Check 1 gap 6 is unchanged and correctly left for planning: the import equality rests on gate.log alone, and "these doc bytes are planning's unedited text" rests on nothing a checker can read.
7. Gap 8 residue: the case scripts run with the repository as their working directory and copy the scripts out of the working tree, pinned to the blob sha at the phase commit, rather than running from a clone or worktree.
