# T3 P3 independent check 1

- Checked commit: 17ec824d227887d0b4f21e4f104ec54e97a6f0de (the phase commit)
- HEAD when the check ran: 9be61d6c5492f20b83bacc72b95ca632e3a822b9 (the record
  commit with gate.log, negative.log and scan.log, local only)
- Checker: a fresh general-purpose subagent (not a fork), given only the inputs
  below plus planning's three rulings of 2026-09-15 quoted in its prompt (the
  spec text is corrected after this report, so the rulings govern), asked to
  answer in plain ASCII, and told to run read-only commands only, never to run
  the scripts, cargo or podman, and never to read the private corpus directory
- Inputs, with sha256:
  - specs/0006-passthrough.md
    7d4b6482eb66a8ac0aff58f00d36429fa41acd0e7be01a503a00ab2cee15f985
  - specs/0001-bootstrap.md
    5b7ed9aa84e89ec895f3cbd4065ef38c5d2d1cf2a1d7c2a0b00be4c7608e6f4b
  - phase diff, `git diff 869796b..17ec824`, saved as <scratch>/t3p3-diff.txt
    1ca18687cab26a5cea858d785120ee792a5362782988aaf8eb025efdf67377cc
  - draft report 1, <scratch>/t3p3-report-draft1.md
    5fd449cab207c21125ff84dc65bd1633b43e6530cf1fdb7bef64fde875be9330
  - records/logs/t3-p3/gate.log
    24b75cdb990f1e608697942df9a19d993bf01b9d22de8d48332804465dc1e238
  - records/logs/t3-p3/negative.log
    062d6bdd42291258177f9793e28fca59a1218cbc0d5c0a8fa2abf174309b1f2d
  - records/logs/t3-p3/scan.log
    5047a29179d4e72edaa4d9db6e6a1e70108b34e7f1320f97811fdc82b0cac8a1
  - records/libreoffice-2026-09-15.md
    ef2177180704b1a7ecfc5bdd215410498d2c9b868055f52e35042131735dceeb
- Answer: the checker's final message, extracted verbatim from its transcript
  with python, without printing the transcript. The extracted answer is 22219
  bytes, 92 lines, sha256
  762fc4342b9f031f3a680d72c85fe3193989237b1c95b0de57437128c36523fd, and holds 0 non-ASCII bytes and no HTML escapes, so no
  character replacement was needed.
- ONE REPLACEMENT, and the reason (departure, planning's ruling of 2026-09-15).
  The extracted answer held one string that privacy pattern 10 matches, at line
  74, columns 344 to 360: the bare shape the checker was naming
  in prose while listing what the scan's control battery covers. It is not a
  path from this machine: it has no home prefix, no user id and no segment
  after it. Committing it verbatim would fail the privacy gate, so planning
  ruled that the reversible-replacement mechanism of specs/0001-bootstrap.md
  (written for non-ASCII characters, T1 P2 departure 12) is extended to this
  privacy-pattern hit. The original is the string formed by the four characters
  `.cla` and `ude`, followed by `/pro` and `jects/`, which is privacy pattern
  10's bare shape; it is written in pieces here so that this header does not
  form the shape either. It was replaced by the token `<pattern 10 shape>`.
  The change is reversible from this description, and the sha256 above is of
  the answer before the replacement; the answer below hashes to
  49f6c6d9da88661ff72507f9c64b6dc9c63fa17f3368f81b2746d40223198d78.
  Planning also scoped T0 P6d to narrow patterns 9 and 10 so that a real path
  still matches and the bare name does not; it runs after this phase and before
  T3 P4.
- The answer holds two lines with a scratch-directory path of the mktemp form.
  Planning ruled on 2026-09-15 that they stay verbatim: they match no pattern
  and are not Claude Code session directories, which is what the log privacy
  rule names.
- Privacy-scanned after the replacement with the same command forms as scan.log
  (the patterns from scripts/privacy-patterns.txt, the hostname, the private
  denylist, addresses outside the allowlist, a Claude-Session line, U+2014,
  control bytes): every count 0. The controls fired on a file built for them
  (session output).

---
I have everything needed. Writing the answer now.

---

**Note on my inputs.** The harness also put CLAUDE.md, a memory index, a git status snapshot, attribution text and the user's email into my context. I say so here and did not use any of it as evidence. Everything below is cited to the spec, `<scratch>/t3p3-diff.txt`, `<scratch>/t3p3-report-draft1.md`, the three logs under `records/logs/t3-p3/`, `records/libreoffice-2026-09-15.md`, or git objects I read read-only.

## 1. Does the diff do what the spec and the three rulings say, and nothing else?

**Scope.** `git diff 869796b..17ec824` touches five files (gate.log:185-190): ROADMAP.md and specs/0005-corpus.md in fd5dcf9 (planning's doc updates), and scripts/passthrough-run.sh, scripts/passthrough-record.py and the new records/libreoffice-2026-09-15.md in 17ec824 (gate.log:171-183). No crate, gate file, Cargo file or workflow changed (gate.log:228). That matches "Tests: none in Rust (no crate changes)".

**`--libreoffice` and its validation.** Added to USAGE, to the value-taking case arm and to the variable block (`<scratch>/t3p3-diff.txt` lines 219-251). Validation is `^[A-Za-z0-9][A-Za-z0-9._:/@-]*$` plus `command -v podman` and `command -v python3` (diff lines 259-263). The leading-character anchor means the value can never start with `-` and so cannot be read by podman as a flag; that is correct and worth keeping. The podman and python3 requirements are beyond the spec; the report names the python3 one ("The runner now needs `python3` on the host when `--libreoffice` is given"), not the podman one. **Neither refusal, nor the image regex, is exercised anywhere in negative.log** - the new argument validation has no negative case at all.

**lo_check's four conditions.**

1. *Exit 0 within the timeout*: `timeout --signal=KILL "$TIMEOUT" podman run ...` then `rc != 0` writes `the conversion exited %s` (diff lines 288-295). Same `--timeout` as the copy, as the spec requires.
2. *Stderr*: implemented exactly as ruling 1 requires, as a whole-line match: `grep -a -E 'Error|Warning'` piped into `grep -a -q -v -x -F -e "$LO_ALLOWED_STDERR"` (diff lines 297-300). `-x -F` means any other javaldx wording, any line with extra text around the allowed text, and any second Warning line all fail. The allowed line is a `readonly` constant with a comment citing the ruling and the record (diff lines 227-230). **One defect:** with `set -uo pipefail`, if the first grep errors (exit 2 or more) the pipeline status is non-zero, the `if` is false, and the condition is treated as *satisfied* - a grep error on stderr.txt silently reads as "no disallowed line". `specs/0001-bootstrap.md` line 105 states the project rule that an error is never reported as zero hits; this code violates its spirit. The report does not name it.
3. *ODS exists*: `ods=$outdir/${base%.*}.ods`, `[ ! -f "$ods" ]` (diff lines 302-306). Correct for both `<id>.xlsx` and `out.host.xlsx`.
4. *Table count*: a python3 one-liner counts `<sheet ` in the reference workbook's `xl/workbook.xml` and `<table:table ` in the ODS `content.xml`, compared with a zero-guard (diff lines 307-325). The reference is always the *input* (diff lines 350, 356), which is what the spec says ("as many ... as the input's `xl/workbook.xml`").

**Departure the report does not name:** the spec's fourth condition has two clauses - the counts must be equal "**and every one of those elements is present**". The implementation compares counts only; it never checks that the sheets are the same sheets (no name or identity comparison). Either the clause is redundant wording planning meant as emphasis, or it asks for something the diff does not do. The report restates the condition with that clause dropped and does not flag the omission. Planning should rule.

**Correctness risks in the counting, none named in the report.** (a) `<sheet ` with a trailing space fails to match a producer that writes a newline or other whitespace after the tag name; the result would be `sheets=0` and a `fail` with the reason "the ods holds N table elements and the workbook 0 sheet elements". The spec dictated this regex, so the diff is conformant, but over 27 real workbooks in P4 it is a plausible false-fail. (b) The spec says "Chart sheets count as sheets", and `<sheet ` does count a chartsheet in `xl/workbook.xml` - but nothing here measures whether LibreOffice emits a `<table:table ` for a chartsheet. If it does not, every corpus workbook with a chart sheet will read `lo fail` and `lo_in fail` in P4. Untested and unmentioned. (c) The same `--timeout` (default 300 s) covers a LibreOffice cold start plus conversion; for the two `large` corpus files this could time out and read as `fail`.

**`lo` and `lo_in` and the "-" cases.** `lo=- lo_in=-` are initialised before the `if [ -n "$LO_IMAGE" ]` block; `lo_in` is computed whenever an image was given, `lo` only when `outcome = ok` (diff lines 346-362). That is exactly the spec's "`-` (no image given or no `ok` output)", and it gives the intended separation of a file LibreOffice cannot open on either side from one the copy broke. Both columns are appended after `leftover` in SUMMARY_HEADER and in the row printf (diff lines 224, 364-366). The per-file stdout line is unchanged (diff line 368), so the T3 P1/P2 stdout contract holds, as the report says.

**A failure inside lo_check is NOT distinguishable from a genuine fail in the column.** `cannot prepare the conversion directory`, `cannot count sheets and tables` and `cannot read the ods or the workbook part` (diff lines 286-287, 316, 318-321) all return 1, and all surface as the same `fail`. The distinguishing text is in the reason file, which is never printed and which the record script does not read. In the P4 record a broken podman, a full disk or a python3 failure will look identical to a LibreOffice load failure. The report does not name this; it should, because P4's acceptance sentence treats any `lo fail` on an `lo_in ok` file as "a finding about real archives".

**Leak paths to stdout/stderr - I found none.** `lo_check` prints nothing: container stdout and stderr go to files under the run subdirectory, and the reason file holds only this project's own wording (diff lines 291, 294-323). The child is given `3>&-`, closing the saved die descriptor, so container output cannot reach the error channel; the runner's own stderr is already `/dev/null` by then (runner lines 74-75). The image name is regex-constrained and is never echoed. negative.log:414-415 confirms 0 lines with Error, Warning or javaldx and 0 lines with the scratch path across all eight captured runner outputs, with a positive control at negative.log:411.

**Other behaviour beyond the spec.** `rm -rf "$dir/lo.$LABEL" "$dir/lo-in.$LABEL"` per iteration (diff line 336) - sensible, unnamed in the report; note it does **not** remove the two `*-reason.*.txt` files, so a stale reason can sit beside a now-passing check. The record script gains `LO_VALUES` validation, two counting columns `lo fail` / `lo_in fail` in the outcome table, `lo` and `lo_in` columns and a widened heading in the "Files not ok" table, and `lo != fail`/`lo_in != fail` in the skip condition (diff lines 133-207). The spec asked only that the record script "reads it and lists `fail` files by id"; the counts and columns are additions the report does name. `EXPECTED` was not extended with the lo columns, which is why negative.log:433 shows a noct row listed with `differs` = `-`; that is coherent, since lo and lo_in are their own columns. Because `read_table` compares the whole header, any pre-P3 run directory is now refused; harmless, unmentioned.

**The LibreOffice record.** Contains everything ruling 1 and ruling 2 require: the pins and the private file's sha256; the allowance recorded next to the pins with the reason "The image has no Java, by design"; the entrypoint flags `soffice -env:UserInstallation=file:///tmp/lo-profile --headless --norestore --nolockcheck --nologo "$@"` with the explanation that `soffice` is not passed; and both ruling-3 findings, including "the exit status is not evidence of a good load". It names no corpus path (gate.log:214-215). Two claims in it have **no primary anywhere in the repo**: "a cut central directory" as a third failing case (no such case exists in negative.log), and "three `<table:table` elements, the third empty" for the ghost workbook (negative.log:385 shows only `lo=ok lo_in=ok`; the count of 3 is inferable from the code, "the third empty" is not). The faithfulness of the copied pins themselves cannot be checked from committed evidence at all - the gate proves only that the cited sha256 matches the private file as it is now, not that the table below it reproduces that file.

## 2. Do the logs contain every line the spec and the rulings require?

**The seven required cases.**

1. *Passes on a generated workbook* - yes, C1, negative.log:382 and 388.
2. *Fails on a copy with one worksheet entry truncated* - **only partly.** The spec's case is "a copy with one worksheet entry truncated (a deliberately corrupted output)". What the log shows is the truncated workbook placed in the scratch corpus as an **input**: negative.log:383 reads `truncated outcome=error lo=- lo_in=fail`, and the assertion at negative.log:389 is on `lo_in`, not `lo`. passthrough refused the file, so no copy existed to check. A copy failing the check is demonstrated once, by C3 (negative.log:384, `noct outcome=ok lo=fail lo_in=fail`). The draft report states the facts plainly ("Its outcome is `error`, so no copy was made and `lo` is `-`") but does not label it a departure from the spec's wording.
3. *Fails with `[Content_Types].xml` removed* - yes, C3, negative.log:384 and 391 (the assertion at 391 covers `lo_in`; `lo fail` is read off 384).
4. *Ghost sheet passes with the finding recorded* - yes, C4 at negative.log:385, 392, plus C5 (nopart) at 386, 393, and the finding is in the record, checked at gate.log:213.
5. *"-" when no image is given* - yes, C7, negative.log:399, over all six rows.
6. *A second different Warning line fails, the allowed line alone passes* - yes, C8a at negative.log:403-404 and C8b at 406-407, injected through a PATH wrapper around the real podman (negative.log:268-293). Two caveats: C8b's wrapper *repeats* the allowed line, so what is proven is "only allowed lines pass" rather than literally "only the allowed line"; and C8a's reason file is never shown, so the log does not positively establish that the fail came from the stderr condition rather than another one - the inference relies on the wrapper preserving podman's exit status.
7. *The container's stderr never reaches the log* - yes, C9, negative.log:410-415, with a genuine positive control (the container's own line counted 1 in the kept stderr file, 0 in the reason file, 0 across the runner's outputs).

**Rules for every phase.**

- HEAD on line 1: gate.log:1, negative.log:1, scan.log:1 all name the phase commit and the HEAD at run.
- Printed scripts: gate.log:10-165, negative.log:8-345, scan.log:5-241.
- Stopped-run note: negative.log:3 gives the start time (2026-09-15T23:55:51Z), the last completed check ("six workbooks and a manifest of six rows"), and the reason ("the system was running low on memory"), and adds that no log of that run was committed. This satisfies specs/0001-bootstrap.md lines 96-98 in full.
- `/usr/bin/grep` with `LC_ALL=C`: gate.log and scan.log use it everywhere. negative.log sets `G=/usr/bin/grep` and uses `LC_ALL=C` in every scan function - **except negative.log:324**, `"$(is $G -q -F -e '| lo fail | lo_in fail |' "$MD")"`, which calls grep without `LC_ALL=C`. Also negative.log:110 (`both_listed`) omits `-a`. Both are harmless in practice (ASCII fixed strings) but are deviations from the stated form.
- Positive controls: gate.log:202 (home-path control counts 1), gate.log:250-251 (ESC/SI file expects 3 bytes, tab/CR file expects 0); scan.log:253-309 runs the full control battery including every P6 pattern, the P6b forms, the "bare names must not match" negative, host, email and all seven denylist entries; negative.log:411-413 controls the container-line count in both directions. The one gap: negative.log:439's control-and-non-ASCII byte count over the captured outputs has **no** positive control of its own (the gate and scan logs each have one for the same form).
- Control bytes: 0 in all three logs. I verified this myself with `/usr/bin/grep -a -c -P` under `LC_ALL=C` and a control file that returned 1, and also found 0 non-ASCII bytes and 0 U+2014 bytes in all three logs and in records/libreoffice-2026-09-15.md. scan.log covers gate.log and negative.log at scan.log:406-424; scan.log cannot scan itself, and my check covers it.
- Log privacy: no hostname, no Windows path, no session id, no denylist hit, 0 total hits across files, messages and both other logs (scan.log:426-430). The corpus directory prints as `<corpus dir>` (gate.log:6, 205) and scratch paths as `<scratch>`.
- Negative tests in a scratch copy: the corpus, run directories and outputs are all under a `mktemp -d` (negative.log:4, 61-62) and `--corpus` always names the scratch one. But the script itself does `cd "$(git rev-parse --show-toplevel)"` and runs `cargo build` and the repo's scripts **in the main working tree** (negative.log:51, 144, 234). specs/0001-bootstrap.md line 118 says every negative test runs in a scratch copy, "never in the main working tree". The tree was clean afterwards (negative.log:440), and this is probably the pattern earlier T3 phases used, but it is worth planning's word rather than silence.
- Compound checks inside one named function: satisfied throughout - `image_present`, `record_states`, `record_cites_private`, `files_complete`, `messages_complete`, `scan_passed`, `lo_is`, `all_dash`, `run_all_dash`, `record_written`, `both_listed`, `refused_bad`, `reason_held`. No `$(yn [ ... ] && cmd ...)` form appears.
- Commit messages: both end with the Co-Authored-By line and nothing after it (scan.log:384, 394, 403), with no `Claude-Session` line (f_session count 0 on both).

**Are the gate's LibreOffice and image checks real evidence or circular?** Mixed, and the report presents them all as if equal.

- gate.log:205-207 (the record cites the private record's sha256 as it is now) is **real**: the gate hashes the private file at run time and greps the record for that value.
- gate.log:208-210 (the pinned image is in local storage with the recorded id) is **real**: `podman image inspect` returns the id, compared inside `image_present`.
- gate.log:211-213 (the record "states the image tag and id", "states the allowed stderr line and the entrypoint flags", "states the two measured findings") is **circular**: `record_states` greps the record for strings the same session wrote into it. It proves presence, not truth. The entrypoint flags in particular are asserted by the record and by negative.log:349 and are nowhere observed - no log in this repo shows the image's actual entrypoint. The allowed line is independently corroborated by C8b, and the ghost/nopart finding by C4 and C5; "every conversion exits 0" is corroborated by nothing in any log.

**One log defect:** negative.log:16 says "The scratch corpus holds five generated workbooks" and then lists and generates six (negative.log:17-24, 359-364). The count in the committed script comment is wrong.

## 3. Does every number and quoted string in the draft report appear in a primary?

Almost all of them do. I checked each citation; the ones that hold are: the 62/13 insertion counts (gate.log:176); the five file sha256 prefixes and byte counts, `2c7c8044`/15931, `b76bc058`/13943, `ef217718`/4999, `db90d10f`, `63bc9470` (gate.log:193-197, and the last two again at scan.log:335, 371); `541945cc...` and its "as it is now" check (gate.log:205-207); `3f9baefc...` (gate.log:208-210); the eight PASS lines, exit 0 and 0 stderr bytes (gate.log:231-247); the control-byte lines (gate.log:249-253); the empty status (gate.log:255); the summary line (gate.log:257); every negative.log citation for C1, C2, C3, C4, C5, C6, C7, C9 and C10, including the exact refusal string `error: summary.host.tsv has a lo value other than ok, fail or -` (negative.log:435); the stopped-run sentence with its start time and reason (negative.log:3); and every scan.log citation (250, 310, 380, 403, 405-424, 426-430). The log line counts 257, 442 and 430 are right, and "1129 insertions" for 9be61d6 is confirmed by `git show --stat 9be61d6`. I also confirmed the committed log blobs in 9be61d6 hash to exactly the working-tree files scan.log scanned (gate.log `24b75cdb...` at scan.log:406, negative.log `062d6bdd...` at scan.log:415).

Problems:

1. **The exit-0 finding is mis-cited.** The report writes "the truncated workbook fails while its conversion exits 0 (negative.log:383, 389)". Neither line shows a conversion exit status; no line in any log does. The support is indirect and sits at negative.log:410, where the reason reads "stderr holds a line with Error or Warning other than the allowed javaldx line" - a branch reachable only when `rc` was 0. The broader claim "Every conversion exits 0, including the ones that load nothing" has no primary in the repo at all; it rests on the record's own "measured 2026-09-15" sentence. Ruling 3 required the statement, and the statement is there, but the measurement behind it was never logged.
2. **Overstated scope at gate.log:198-202.** The report says "no file in the phase holds a literal home path"; gate.log:202 says "in the two scripts and the record" - three of the five changed files. ROADMAP.md and specs/0005-corpus.md were checked for home paths only indirectly, by scan.log's P6 battery (which covers `/mnt/<x>/Users/` and `<pattern 10 shape>` shapes, not a bare `/home/`).
3. **"committed unedited"** for fd5dcf9 has no primary. Nothing in the repo or the logs records what planning's text was before the commit; only the session knows.
4. **"LibreOffice recovers from the malformed XML"** (C6) is an interpretation. The primary at negative.log:387, 394 shows only `lo=ok lo_in=ok`.
5. **Unsupported record claims carried into the report by reference.** The report says the record "carries the pins ... and both findings"; the record additionally asserts a "cut central directory" failing case and "the third empty" for the ghost workbook, neither of which has any primary in this repo. If the report is committed as-is, planning is endorsing those two sentences on the session's word.
6. **Items marked "(session output)" that could have been logged.** The scan.log self-check (0 control bytes, 0 non-ASCII, 0 U+2014, with a control) could have been a section in gate.log, exactly as the gate already does for its own captured output - I reproduced it in seconds and it passes. The three dry-run harness fixes could not reasonably be logged as runs, but the current, fixed form of each is visible in the printed scripts, so the claim is checkable. The `/tmp/tmp.*` sentence could not have been logged and, more to the point, **the draft report contains a literal temporary-directory path pattern**; it would not trip any privacy pattern (scan.log:39 matches only `/tmp/claude-<digits>/`), but it is a machine path in a file that may be committed.
7. The report's "2 commit messages, each ending with the Co-Authored-By line (scan.log:403)" cites the count line; the per-message assertions are at scan.log:384 and 394. Trivial.

## Gaps, most important first

1. The spec's fourth condition includes "and every one of those elements is present"; the diff compares counts only, and the report neither implements nor flags the clause. Planning should rule on whether the clause meant anything beyond the count.
2. An internal failure of `lo_check` (directory, python3, ODS-read errors) is indistinguishable from a genuine LibreOffice failure in the `lo` column and in the record; only the unprinted reason file tells them apart. This matters directly for P4's acceptance rule.
3. "Every conversion exits 0" is required by ruling 3 and is stated in both the record and the report, but no log anywhere carries a conversion exit status. The same measurement session produced a "cut central directory" case and a "third empty table" observation that have no primary in the repo.
4. The stderr pipeline treats a grep error (exit 2 or more) as "no disallowed line", so an error reads as a pass - against the project's own "an error is never reported as zero hits" rule.
5. The spec's truncated case asked for a corrupted **copy**; the log exercises a corrupted **input** (`lo_in fail`, `lo` "-"). A copy failing is shown only by C3. The report states the facts but does not call it a departure.
6. Unmeasured false-fail risks for P4, none named: a chartsheet may produce a `<sheet ` with no matching `<table:table `; `<sheet ` with a trailing space may miss a producer that breaks the line after the tag name; and the shared 300 s timeout covers a LibreOffice cold start on the `large` files.
7. The new `--libreoffice` validation (image regex, podman and python3 requirements) has no negative case in negative.log.
8. gate.log:211-213 are circular checks - the record greps itself. The entrypoint flags in particular are asserted nowhere but in the record, and the faithfulness of the copied pins cannot be verified from committed evidence.
9. `timeout --signal=KILL` kills the podman client, which may leave a container running and skip `--rm` cleanup; and the whole corpus directory is mounted into the container for every `lo_in` conversion (the report names the mount, not the orphan risk).
10. Smaller items: negative.log:16 says "five" workbooks where six are generated; negative.log:324 calls grep without `LC_ALL=C`; the reason files are not cleared at the top of each iteration, so a stale reason can outlive a fail; negative.log:439's byte check has no positive control; the negative script builds and runs in the main working tree; C8a's failure reason is not shown; the report's gate.log:198-202 claim overstates which files were checked for a literal home path; "committed unedited" has no primary; and the draft report contains a literal `/tmp/tmp.*` path pattern.
