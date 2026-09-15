# T2 P1 independent check 1

- Checked commit: e3dd52e777fe16fb58e5bf7cff6e562f2c332673 (T2 P1 phase
  commit)
- HEAD when the check ran: f692dbf5fbbaf2f2e7d3ae9135a195442ccb5f4e (the
  record commit with the four logs, local only)
- Checker: a fresh general-purpose subagent (not a fork), given only the
  inputs below, asked to answer in plain ASCII, and told to run read-only
  commands only, never to run the two scripts or check.sh, and never to read
  the private corpus directory
- Inputs, with sha256:
  - specs/0005-corpus.md
    8224e7aaf15434203bcc81b82551f72b27cb2b565ec748b9ceb03aec4c885131
  - specs/0001-bootstrap.md
    5b7ed9aa84e89ec895f3cbd4065ef38c5d2d1cf2a1d7c2a0b00be4c7608e6f4b
  - phase diff, `git diff 24d4a16..e3dd52e`, saved as <scratch>/t2p1-diff.txt
    251d6eae2ff11c2ff5690a0219ff680a76e0d43559a0fc3cef5230ad5a7fbb59
  - draft report 1, <scratch>/t2p1-report-draft1.md
    57c1e14cb95ebf2b08f1cd4cc881cd0a18239b31f92f5282b8066df79753b6db
  - records/logs/t2-p1/gate.log
    8d92731da3203da0060cac8a1b9ca9635b8a635747d038bb66ee872a14109710
  - records/logs/t2-p1/positive.log
    032921663c94af893d15cedff767e4ede765108dd8b8ec39d68efe3d311542d9
  - records/logs/t2-p1/negative.log
    b77f9d329cd0450a71259f0575c5fb347e775cd233dbb4e50e4516a64a34018e
  - records/logs/t2-p1/scan.log
    7e1f4dc64b2d1105e5a121d46ed7cfa3e824beed9591a89f8c428b68ddc3fca8
- Answer: the checker's final message, extracted verbatim from its transcript
  with python, without printing the transcript. The extracted answer is 21219
  bytes, 117 lines, sha256
  42e4637cca95a47d49d0a8a9f09abc04e75d6e9c93f0e8febf26b788c82721a0, and holds
  0 non-ASCII bytes, so no replacement was needed. The task notification
  delivers the answer HTML-escaped; the extracted text is not: it holds no
  `&lt;`, `&gt;` or `&quot;`, and 8 lines with a literal `<` and 9 with a
  literal `>`. Its one `&amp;` is the checker quoting the XML fixture's
  Application element.
- Privacy-scanned before the commit with the same command forms as scan.log
  (P6 patterns from scripts/privacy-patterns.txt, the hostname, the private
  denylist, addresses outside the allowlist, a Claude-Session line, U+2014,
  control bytes): every count 0 on the answer. The controls fired: 6 P6
  patterns, the hostname, one address, all 7 denylist entries, one
  Claude-Session line and one U+2014 (session output).

---
Checked. Here is the answer, ready to be saved verbatim.

---

Harness context notice: my context also contained CLAUDE.md, an auto-memory index, a git status snapshot, git attribution text and the user's email address. I say so here as required and I did not use any of it as evidence. Everything below cites the spec, the diff, the four logs, or git objects in /home/dev/emend that I read read-only.

# 1. Does the diff do what the spec says and nothing else?

## scripts/corpus-add.py against every P1 bullet

Line numbers are lines of the blob at e3dd52e (git show e3dd52e:scripts/corpus-add.py).

- "Resolves the corpus directory from EMEND_CORPUS_DIR or the default, requires it to exist with mode 700, and requires it to be outside any git work tree (git -C <dir> rev-parse fails)": lines 78-92. Present and exact; the work-tree test is `outside_git` (61-68) and returns "not inside" when rev-parse exits non-zero.
- "Requires the file's extension, lowercased, to be one of the four, and its first four bytes to be 50 4B 03 04. An OLE2 file ... is refused with the message `refused: not a zip-based workbook`. Nothing else about the file is printed": lines 94-103 with `die` at 36-38. Exercised at negative.log:473 (OLE2) and negative.log:500 (.csv).
- "Validates producer, source and every tag against the fixed lists": lines 104-110; the lists at 24-30 match the spec's four exts, five producers, four sources and 23 tags.
- "Computes sha256 and id, refuses a duplicate id, copies the file to corpus/<id>.<ext> with mode 600, reads app from docProps/app.xml (values only, XML-unescaped, whitespace collapsed; none when absent), and appends the manifest row, creating the manifest with its header when missing": lines 115-153. HEADER (31-32) is the spec's twelve columns in order. The app rule is implemented by ET plus `" ".join(el.text.split())` (53-58) and is exercised by the fixture whose Application element is "  Gen &amp;\n  Tool " and whose expected column is "Gen & Tool 1.1" (positive.log:443, 446).
- "Prints exactly one line: `added <id> <ext> <bytes> <producer> <tags>`. It never prints the original name, the note, the corpus path or any content of the file": line 156, checked as an exact match at positive.log:444.

## scripts/corpus-coverage.py against every P1 bullet

Line numbers from git show e3dd52e:scripts/corpus-coverage.py.

- Validation of header, column count, producer, source, tags, unique ids, files present: lines 66-98. `--verify` recomputes sha256: 102-115.
- "Any failure prints `FAIL <reason>` (reasons never include private columns) and exits 1": 145-147, plus the catch-all at 155-157. All reason strings are literals built from a line number.
- The five output groups in the spec's order: 118-134 emits `files <n>`, the per-workbook lines, one `tag <name> <count> <producers>` per tag in the fixed order, `uncovered ...`, `single-producer ...`. Confirmed end to end against positive.log:493-522.
- "Exits 0": 149.

## The import step, and whether every change to the imported file is named

e8087f6 adds exactly scripts/corpus-add.py and nothing else (gate.log:195-197; git show --name-status), with mode 100755 (git ls-tree e8087f6), and its message names sha256 30c66ee96732e329fda3de049cef5e9bfd8c21f58e07fc6385b12bf77602f250 (git show -s e8087f6; gate.log:198). gate.log:191-194 records the recorded value, the private file as it stands (4571 bytes, mode 600) and the blob at the import commit as equal. I cannot read the private file, so that equality rests on gate.log alone; what I could confirm independently is that the blob at e8087f6 is 4571 bytes and hashes to 30c66ee9..., so the recorded value is at least consistent with what was committed.

I compared the report's list of changes with the full diff (gate.log:201-335 and git diff e8087f6 e3dd52e) hunk by hunk. Every hunk is named by the report and the report names nothing that is not in the diff: docstring plus fixed USAGE (hunks 1-3), the read_app split into KeyError/BadZipFile/ParseError, `outside_git` with GIT_ variables stripped, the five new refusals (HOME, missing corpus dir, not a directory, unreadable file, git cannot be run), the manifest-newline refusal, `dest_dir` plus `os.makedirs(..., mode=0o700)`, and the `__main__` catch printing only the exception class with exit 2.

Behaviour kept from the helper that goes beyond the spec text: repeated tags refused (109), tab/CR/LF in producer_version, note or file name refused (111-113), refusals to stderr with exit 1 and usage errors exit 2. The report names all three. Three further beyond-spec behaviours it does not name: `os.umask(0o077)` around the write (140, 155), `os.chmod(manifest, 0o600)` (153), and validation of an existing manifest's header before appending (128-129). None is harmful, but they are unnamed.

## The doc commit

45dd9e3 touches ROADMAP.md, specs/0005-corpus.md and specs/0006-passthrough.md and nothing else (scan.log:317-322; git show --stat 45dd9e3: 143 insertions, 30 deletions). `git diff 45dd9e3 e3dd52e --` on those three paths is empty, so no later commit in the phase touched them (also scan.log:323-331). Whether they are byte-identical to what planning sent cannot be checked from my inputs; the report's claim that they equal the working-tree sha256s it sent planning is outside every primary I have.

## Departures from the spec text, and correctness

- corpus-coverage.py adds checks the P1 list does not name (sha256 shape, id equals its first 12 digits, bytes numeric, ext in the four, `--verify` size comparison, manifest must end with a newline, `single-producer 0`). The report names each of these and says which are unexercised. They follow from the "Manifest format" section, so I do not read them as scope creep.
- corpus-add.py adds refusals the P1 list does not name (see above). Named by the report except the three items in the previous section.
- Report overstatement: "The FAIL reason names a manifest line number and never a value". The header failure has no line number: negative.log:599 is `FAIL manifest header mismatch`, as is the code at line 67. The privacy half of the sentence holds.
- Print paths. I traced every print site in both scripts. corpus-add: `die` (fixed literals), USAGE, the added line (id, ext, size, producer, tags, all validated or derived), and `error: <ExceptionClass>`. corpus-coverage: USAGE, `FAIL <literal + line number>`, the report lines built only from id, ext, bytes, producer, source and tags after validation, and `FAIL unexpected error <ExceptionClass>`. No path can print original_name, note, producer_version, app, a corpus path or file content. Two residual points: (a) a BaseException that is not an Exception (KeyboardInterrupt) would still print a traceback, which names the script's own path and source lines but no column value and no corpus path; (b) see the next item.
- Correctness problem worth recording. The copy is made before the manifest row is appended (143 then 146-152). The report names the orphan-file flaw as inherited from the helper but describes the trigger only as "if the append fails". There is a concrete reachable trigger the report does not name and no logged case covers: only tab, CR and LF are refused in the file name, producer_version and note (111-113), so a non-UTF-8 byte in any of the three survives validation as a surrogate and raises UnicodeEncodeError at the append, after the copy. The result is `error: UnicodeEncodeError` with exit 2, an orphan corpus/<id>.<ext>, and, on a first add, a manifest holding only its header. corpus-coverage.py then does not list the orphan and a re-add of the same file is refused as a duplicate.
- Minor: `os.path.splitext` gives no extension for a name that is only an extension (".xlsx"), so such a file is refused as "extension not one of ...". Harmless.
- The spec's own P3, P6, P7 and "Out of scope" still say corpus-add.sh / corpus-coverage.sh (specs/0005-corpus.md:229, 253, 257, 265 at e3dd52e). The report flags this and correctly leaves planning's text alone.

# 2. Do the logs contain every line the spec requires?

Every item on the spec's test list is present:

- add a generated xlsx, expected id, one manifest row, mode 600, right hash: positive.log:438-447 (`added 7eb89f523115 xlsx 643 libreoffice plain,strings` at 441; expected id from sha256sum at 443; the four checks at 444-447).
- the same file again, duplicate refused, manifest unchanged: negative.log:459-466 (`refused: duplicate id`, and "nothing added (manifest sha256 and corpus file count unchanged)" at 466).
- OLE2-signature file, refused, nothing added: negative.log:468-475.
- unknown tag: negative.log:477-484. unknown producer: 486-493. upper-case .XLSX accepted and stored as xlsx: positive.log:449-460, with "no file with an upper-case extension in corpus/" at 460. .csv refused: negative.log:495-502.
- an original name holding a sentinel never appears in any output of either script: sentinels are generated for the file names, producer_version and note (positive.log:70-74), the counter controls prove the counters work (positive.log:413-421, negative.log:413-421), the scratch manifest holds them as a positive control (positive.log:565-566, negative.log:629-630), and the counts over all 12 and all 38 captured outputs are 0 for sentinels, 0 for the scratch path, 0 control bytes and 0 non-ASCII bytes (positive.log:564-569, negative.log:628-633).
- coverage on the scratch manifest lists every tag with the right counts: positive.log:488-524, compared against a 30-line expectation the script builds independently from the fixed tag list (positive.log:275-295).
- `--verify` fails after one file is modified in place: negative.log:552-593 (byte flipped at offset 10, same size confirmed at 554, plain coverage still exit 0 at 587, `FAIL line 3: sha256 mismatch` at 591 with exit 1 at 589).

Extra cases beyond the spec list, all logged: N6 unknown source, N7 zip signature over garbage, N8 mode 755, N9 missing directory, N10 inside a git work tree, N12 five broken manifests, P3 no app.xml, P4 GIT_DIR exported, P6 `--verify` on a good corpus.

All four required logs exist and are committed in f692dbf: gate.log (401 lines), positive.log (572), negative.log (636), scan.log (454).

Rules for every phase:

- First line names the commit and the HEAD: gate.log:1, positive.log:1, negative.log:1 all give phase commit e3dd52e and HEAD e3dd52e; scan.log:1 gives the diff range and HEAD.
- The script is printed into each log: gate.log:10-175, positive.log:7-410, negative.log:7-410, scan.log:5-241. All four start `#!/usr/bin/env bash`.
- /usr/bin/grep with LC_ALL=C: gate.log:43-44, 69, 92, 122, 126; scan.log:18 and 48-56; positive/negative.log:55, 89-92. Non-ASCII is matched as `-P '[^\x00-\x7F]'` and U+2014 as its UTF-8 bytes via `printf '\342\200\224'` (scan.log:24, 48-50); no `\x{2014}` anywhere.
- Positive controls in the same command form: scan.log:253-309 covers every form including one control per denylist entry and a "bare pattern names must not match" negative control (283); gate.log:354 is the home-path control and 394-395 the control-byte controls; positive/negative.log:413-421 are the counter controls.
- Exit codes: scan.log's `run` treats rc >= 2 as an error rather than zero hits (scan.log:72-77), and `check_ctl` asserts the expected exit for every control (78-85). Totals at scan.log:450-452 are 0 hits, 0 errors, 0 structural failures, RESULT: PASS at 454.
- No control bytes: gate.log:393-397, positive.log:569, negative.log:633, and scan.log's f_ctrl over every file, message and log (scan.log:342-447) all report 0. I also counted independently over the four committed logs: 0 control bytes other than tab/LF/CR, 0 non-ASCII lines, 0 U+2014 lines in each.
- Log privacy: f_host, f_deny and f_email are 0 on all five diff files, all three commit messages and the three other logs (scan.log:340-447). No hostname, no session link, no denylist entry. The only `/home/` and `/tmp/claude-` strings anywhere in the logs are inside printed control constructions (gate.log:122, 125, 126; scan.log:39, 130, 132, 141, 142), not real paths; scratch paths print as `<scratch>` and the corpus directory as `<corpus dir>` (gate.log:6, 191-192).
- Compound checks inside one function: mostly honoured, with named helpers `import_is_one_file` (gate.log:51-54), `three_equal` (56), `usage_exit_2` (58), `files_complete` (scan.log:196), `messages_complete` (213), `scan_passed` (233), and `refused`, `added`, `covfail`, `covok`, `copy_ok`, `row_is`, `modified`, `nothing_added` (positive/negative.log:130-174, 363). Three checks instead use `$(yn eval '... && ...')` or `$(is eval '... && ...')`: gate.log:121, positive.log:248 and 366 (same lines in negative.log). eval receives the whole compound as one argument, so both conditions really are evaluated and the failure the rule guards against does not occur, but it is not the letter of the rule.
- Negative tests in a scratch copy: the scratch corpus, the generated workbooks and the wrapper git repository are all under mktemp (negative.log:60, 75, 350-352) and the working tree is clean after the run (negative.log:634). The scripts under test are executed from the main working tree rather than from a scratch clone or worktree; nothing is written there, but it is a small departure from "every negative test runs in a scratch copy".
- Commit messages ASCII, ending in Co-Authored-By only: scan.log:389, 399, 409 (each "last non-empty line is the Co-Authored-By line: yes") and 418; f_session is 0 on all three.

One thing the spec asks for that no log shows directly: the spec says the phase report names "every change made to it", and gate.log:199-335 does print the full diff, so the requirement is met by the log; but nothing in any log checks that the report's prose list is complete. I did that comparison by hand (section 1) and it is complete.

# 3. Does every number and quoted string in the report appear in a primary?

Verified against primaries and correct: the three commit hashes and the record commit hash; 45dd9e3 touching exactly the three doc files (scan.log:317-322) and their sha256s 5312f3af, 8224e7aa, 7d4b6482 (scan.log:340, 367, 376, which I recomputed from the blobs); the import sha256 30c66ee9..., 4571 bytes, mode 600 and the three-way equality (gate.log:191-194); e8087f6 adding only that file (gate.log:195-197) and its message naming the sha (gate.log:198); f16e604a / 5954 (gate.log:340) and 613799ba / 5955 (gate.log:347), both recomputed from git; 5 files in the diff (scan.log:334-339) and 464 insertions / 30 deletions (gate.log:188); the log line counts 401, 572, 636, 454 and 2063 insertions (git show --stat f692dbf); the eight PASS lines, exit 0, 0 stderr bytes (gate.log:380-391); control bytes 0 (gate.log:393-397); empty status (gate.log:399); "gate log checks all as expected: yes" (gate.log:401); ids 7eb89f523115, d54858d6b9e1, 2ac16f2dacc2, 742a0c4f98ff and sizes 643 and 315 (positive.log:426-429, 441, 453, 466, 480); app `none` (positive.log:473); the GIT_DIR control and acceptance (positive.log:476-486); the 30-line coverage output, `uncovered 18: ...` and `single-producer 4: strings,formulas,charts,large` (positive.log:521-522); every N1 to N12 citation, which I checked line by line and all land on the right lines; `FAIL line 3: sha256 mismatch` (negative.log:591); 12 and 38 captured outputs (positive.log:564, negative.log:628); the scan.log citations 250, 310, 385, 389/399/409/418, 421-448, 450-454.

Items that are wrong, mis-cited or unsupported:

1. "No gate file, Cargo file, tool crate or workflow changed in the range (gate.log:372)". gate.log:372 says "no gate file, Cargo file or workflow changed"; "tool crate" is not in that line. The `tools` path is in the check's argument list at gate.log:143, so the claim is true but the quote is not what the cited line says.
2. Change 1 cites gate.log:356-357 for "A fixed USAGE string replaces the lookup of docstring line 3". Those two lines are the exit-2 usage output; the replacement itself is at gate.log:262-263 inside the printed diff. Mis-cited.
3. "The FAIL reason names a manifest line number and never a value (negative.log:591, 599-625)". negative.log:599 is `FAIL manifest header mismatch`, with no line number. The cited range contradicts half the claim.
4. "Every 'command:' line in the two case logs ends with a trailing space (cosmetic)". Not every one: 6 of 7 in positive.log and 19 of 20 in negative.log. The gen.py line (positive.log:424, negative.log:424) has no trailing space.
5. "accepted at positive.log:483-486" for the GIT_DIR case: the run block is 477-486; 483-486 are only the four notes. Harmless but imprecise.
6. "P5 and P6 ... stdout equal to a 30-line expected output (positive.log:493-524, 530-561)": the stdout blocks are 493-522 and 530-559; the cited ranges include the trailing note lines. Harmless.
7. Unsupported by anything I can read: "These equal the working-tree sha256s I sent planning before committing". Nothing in my inputs records what was sent.

Items marked "(session output)", and whether a log could have shown them:

- "143 insertions and 30 deletions" for 45dd9e3: this is in a git object (git show --stat 45dd9e3) but in no log; gate.log:177-181 prints `git log --format` without `--stat`, so a per-commit stat line could easily have been logged.
- "464 insertions, 30 deletions (session output; the stat is in gate.log:182-188)": the stat is in fact logged at gate.log:188, so the "session output" label is unnecessary.
- "2063 insertions" for the record commit f692dbf: in git, not in a log, and could not have been, since the logs predate that commit.
- "mode 755" for the import: not cited at all; it is checkable from git (100755 at e8087f6) and could have been a gate line, since gate.log:340 already prints the mode at e3dd52e.
- "the log's one line ending `: no` is header line 5": correct, and it is the only such line (gate.log:5, ANTHROPIC_API_KEY set in this shell: no).
- "Each log contains the word UNEXPECTED twice, both times in the printed script text (lines 94 and 192)": correct for positive.log and negative.log (gate.log has none; scan.log has one, at line 82, also in printed script text).
- "the helper used to add it with app `none`" for N7: this is a statement about the private file, which I cannot read; it is consistent with the pre-change code shown in the diff (`except (KeyError, zipfile.BadZipFile): return "none"`, gate.log:233-235), so it is supported by the diff rather than only by the session.
- "scan.log cannot scan itself. A separate check found 0 control bytes, 0 non-ASCII lines and 0 U+2014 lines in it": this could have been logged (it is a repeatable command with controls), and I confirmed the three counts myself. Leaving it as session output is the one "session output" item that most clearly belongs in a primary.
- "In a throwaway dry run ... my ad hoc echo lines printed the scratch path to this terminal": not checkable from any input, and by its nature unloggable. Recording it is right.

# Gaps, most important first

1. corpus-add.py copies the file before appending the manifest row, and a non-UTF-8 byte in the file name, producer_version or note passes validation (only tab, CR and LF are refused) and fails at the append, leaving an orphan corpus file and, on a first add, a header-only manifest. The report names the orphan-file flaw but not this reachable trigger, and no logged case covers it.
2. The report's claim that a coverage FAIL reason "names a manifest line number" is contradicted by its own citation: negative.log:599 has none.
3. Mis-quotes and mis-citations: "tool crate" attributed to gate.log:372; gate.log:356-357 cited for the USAGE replacement; "every command: line ends with a trailing space" (6 of 7 and 19 of 20).
4. Three beyond-spec behaviours of corpus-add.py kept from the helper are not named in the report: umask 077 around the write, chmod 600 on manifest.tsv, and header validation of an existing manifest.
5. Three compound checks use `$(yn eval '... && ...')` / `$(is eval '... && ...')` rather than a named function (gate.log:121, positive.log:248 and 366, same in negative.log). They evaluate correctly, but not in the form the phase rules require.
6. The import equality (recorded sha, private file, committed blob) rests on gate.log:191-194 alone, since the private file is outside what I may read; and "the doc files are planning's unedited text" rests on nothing I can read.
7. scan.log's self-check is left as session output although it is a repeatable, controllable command that could have been committed; the doc commit's 143/30 stat and the import mode 755 are likewise unlogged though loggable.
8. The negative cases run the scripts from the main working tree (writing only to a scratch corpus under mktemp, tree clean afterwards at negative.log:634) rather than from a scratch clone or worktree.
9. Several beyond-spec behaviours have no logged case at all (malformed docProps/app.xml, the manifest-newline refusal in both scripts, coverage's size comparison, its sha256/id/bytes/ext shape checks, `single-producer 0`, `uncovered 0`). The report names each as unexercised, so this is a known gap rather than a hidden one.
