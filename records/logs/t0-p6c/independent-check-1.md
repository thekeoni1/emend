# T0 P6c independent check 1

- Checked commit: d8ca1bda47b0ec799687aedf25bc80969779956f (P6c phase commit)
- HEAD when the check ran: 9a26ba0c858c06318eed2f1b1314b80a3b7a869a
  (P6c record commit with the five logs, local only)
- Checker: a fresh general-purpose subagent (not a fork), given only the
  inputs below
- Inputs, with sha256:
  - specs/0001-bootstrap.md
    3945cfaba0607a1e4610847fd8e377e7c3031a35759ad2cddc42994a34f978bc
  - the phase diff, `git diff 302b1b2..d8ca1bd`, saved as <scratch>/p6c-diff.txt
    5693cd2486f5bc58a1807ed07c65d6e59cd76f0acc2788068db08c478ea53722
  - draft report 1, <scratch>/p6c-report-draft.md
    663941746c88fc55841ef3f4ee929f2e2975d9ea57135657d84cbeb6dbf9f10f
  - records/logs/t0-p6c/gate.log
    b08ad192794ef36ae0e02c66c9c58e6340f4b261243c47b70dbf836434b17eb5
  - records/logs/t0-p6c/privacy.log
    5a518dc09c563d2260023875ddf2d269a8468651ba4f43d0f18e2885f7330d56
  - records/logs/t0-p6c/hook.log
    67ff628b68af0224435795bd4fc4af5f76015883c8486170878af3798c80e5f9
  - records/logs/t0-p6c/negative.log
    20c3254616926137404c18d66b1029330982d32304b17998a388337fe212e377
  - records/logs/t0-p6c/scan.log
    e499bb9108cd4c94866afba90d093467423ea95d32e7b4b50d526d5604311e2f

---
I can see a CLAUDE.md (and a user memory index) from the harness in my context. I did not use either as evidence. Everything below comes from the spec, the phase diff, the draft report and the five P6c logs. For a few mechanical checks I ran byte and line counts over the diff and log files.

## 1. Does the diff do what the P6c spec says and nothing else?

**Yes.** The planning doc commit (a371cff) changes only `ROADMAP.md` and the spec. The phase commit (d8ca1bd) changes only `.gitignore`, `scripts/check.sh`, `scripts/privacy-patterns.txt` and `scripts/privacy.sh` (scan.log lines 320-331). I found nothing outside the spec.

Spec bullets, one by one:
- **Office names with `#`, `~` or `.` suffixes:** done.
  - The office pattern ends in `($|[#~.])` in both `privacy.sh` (diff line 145) and step 7 (diff line 85).
  - `.gitignore` gets `*.ext`, `*.ext[#~]` and `*.ext.*` with both letter cases (diff lines 14-58).
  - Negative (a) proves all three names are ignored while untracked, and that each gives a count-only FAIL once committed.
- **Eight extra spreadsheet types:** all eight are in the privacy.sh pattern, the step 7 pattern and `.gitignore`. 7 + 8 = 15 extensions.
- **Zip, gzip and 7z signatures outside `tests/fixtures/`, whatever the name:** done. `signature()` reads 6 bytes, and the result is cached per blob but counted per tree entry against the fixtures index list (diff lines 200-260). The self-test puts the same zip blob inside and outside fixtures and expects a count of 3 (diff lines 298-322), so the per-entry counting is exercised.
- **Pattern, denylist and email checks over every raw header line:** done (diff lines 222-226). The header is everything before the first empty line. Continuation lines of `gpgsig` or `mergetag` start with a space, so they are not empty and stay in the header. The header check runs whether or not the identity check does. The self-test plants all three kinds of hit on header line 4.
- **Emails in path names:** done. `email_lines` now takes `-z`, and `scan_file` no longer skips the email check for NUL-separated input (diff lines 165-187). Findings print only a tree entry number (diff line 333, negative.log 651).
- **Patterns 10 and 11:** exactly as the spec gives them (diff lines 113-114). `EXPECTED_PATTERNS` is still 11, and the file still has 11 lines.

**Can a check silently miss, fail open, or print private data?**
- **Printing:** every new FAIL line prints only a count, a header line number or a tree entry number. None prints matched text or a path.
- **Fail-open risk (low):** `signature()` calls `error` on an `od` failure (diff line 202). But `signature` is only ever called inside a command substitution (`BLOB_SIG[$sha]=$(signature ...)`, diff line 246), so `error` runs in a subshell. It cannot stop the script, and nothing checks the substitution's exit status.
  - If `error` writes to stdout, the message lands in `BLOB_SIG`. That counts as an archive, so the result happens to fail closed, but with a misleading line.
  - If `error` writes to stderr, the scan carries on and can end in PASS with exit 0.
  - The diff does not show `error()`, so I cannot tell which case applies. `od` failing on a file git has just written is unlikely.
- **Behaviour I cannot see:** the diff does not show the grep call that applies the office pattern, the source of the fixtures index list, or how non-blob tree entries are skipped. The self-test's expected counts and negatives (b), (c) and h2/h4 show these work as intended.

**New spec gaps within the scope filter** (the diff complies with the spec's literal forms):
- **Suffixes other than `#`, `~` or `.`:** a copy from Windows into WSL can create a `<name>.xlsx:Zone.Identifier` file. `:` is not in the suffix set and `.gitignore` does not cover it, so the name passes. The file's content can hold the download or referrer URL. This is my own knowledge, not something shown in any primary.
- **Plain-text spreadsheets under another name:** a CSV, TSV or FODS file with its extension changed has no signature, so it passes. This is the same class as the report's finding 1.

## 2. Do the logs contain every required line, and follow the log rules?

**Required lines: yes.**
- **gate.log:** all eight PASS lines in order (lines 96-103), exit 0, stderr 0 bytes, control bytes 0.
- **privacy.log:** `PASS privacy` over 57 commits with HEAD equal to main (lines 117-133), summary line 129.
- **hook.log:** installer exit 0, the hook's sha256 equals the heredoc's sha256, `cmp` says yes, the hook is executable, `core.hooksPath` is unset, and the identity matches (lines 113-175).
- **negative.log:**
  - Cases (a) to (g) are all present with their expected results. (g) repeats P6b (a), (d) and (g), including the example.invalid PASS.
  - No line ends in UNEXPECTED.
  - Line 853 gives the overall result: yes.
  - Every captured output shows 0 fixture lines, 0 control bytes and 0 non-ASCII bytes.
  - Section headings outside captured output do name the spec's own fictional names (lines 214, 584, 680, 833). These are not private.
- **scan.log:**
  - The script is printed, starts with `#!/usr/bin/env bash`, calls `/usr/bin/grep` under `LC_ALL=C`, uses `-P` for non-ASCII and the three raw bytes for U+2014, and handles exit 2 as an error.
  - Every form has a positive control, including the new and P6b forms of patterns 10 and 11.
  - It scans the 6 files, 2 messages and 4 other P6c logs.
  - It scans the 3 P7 logs at HEAD and confirms each equals the working tree copy (lines 459-490).
  - Result: PASS.

**Log rules: met, with two minor points.**
- First lines: four logs name d8ca1bd and HEAD. scan.log names the diff range and HEAD.
- My byte counts: 0 control bytes and 0 non-ASCII bytes in all five logs. There is no session link or id, no Claude temporary-directory path, no Windows path, no token, and 0 matches for the hostname.
- The `/tmp/claude`, `/mnt/` and `claude.ai/code` text in scan.log (lines 40, 41, 133, 142) is pattern text and bracketed control fragments, not real paths.
- The only email address in the logs outside scan.log's allowlisted controls is the noreply identity address (hook.log line 174), which is allowed.
- Minor: gate.log line 3 and the "EMEND_PRIVATE_DENYLIST set: no" lines are fixed strings. They are true by construction (the script exits or unsets first) rather than evaluated where printed. They are not false results.
- Minor: the script copy printed in hook.log (line 64) still has the `yn LC_ALL=C` bug that privacy.log line 64-66 names as fixed. So the two logs print different versions of what is described as the same script. The bug does not affect hook mode. The failed first run of privacy.log is recorded only as a comment in the script.

## 3. Does every number and quoted string in the draft report appear in a primary?

**Checked and supported:**
- Diff stats (6 files, 179 insertions, 33 deletions): I counted these from the diff, and the report also marks them as session output.
- Hashes G, P, H and N: they match scan.log. S matches the file itself and is marked as session output.
- 135 files; 57 commits; line 129; hook hash 3c5d2d79; 13 fixture strings.
- Every N line number cited. For g-d3, line 732 is the FAIL line and the refs count is on line 740.
- Scan counts; 7 entries; the quoted FAIL line formats; the pattern text; the `.gitignore` description.

**Not in any primary:**
1. "29 captured outputs" (report line 74) is **wrong**. negative.log has 28 `captured output:` lines, and I also count 28 by hand. Not marked.
2. "(L-P)" (line 5) is never defined. "ahead by the local commits" and "Nothing is pushed" after the record commit are not in any log. privacy.log line 121 shows 5 commits ahead at d8ca1bd, before the record commit. Not marked.
3. Record commit 9a26ba0, "committed without editing", and the exact quoted "...(57): no" string: all marked as session output.
4. Findings 2 and 3 (xz, bzip2, zstd, rar and tar magic bytes; `50 4b 07 08`; the Numbers package layout with `Index.zip` and a preview image) are format knowledge, not in any primary. Unlike finding 1, they are not marked as understanding or as untested.

**Overstatements:**
- **Finding 3:** "the directory name reaches GitHub" contradicts the report's own sentence that `Index.zip` is caught. A commit holding that blob fails the archive check, which blocks the whole push. Also, `.gitignore`'s `*.numbers` pattern ignores a directory of that name unless it is force-added.
- **Line 3:** "is done" comes before the independent check and planning's verification, which the spec requires before a phase counts as done.

## Defects, by severity
1. **Low (fail-open risk):** the `error` call inside `signature()` runs in a command-substitution subshell, so an `od` failure cannot exit 2, and whether it fails closed depends on `error()`'s output stream, which the diff does not show.
2. **Low (spec gap, possible leak):** office names followed by other suffixes, such as the `:Zone.Identifier` files WSL copies create, pass the privacy.sh check, step 7 and `.gitignore`. Not in the spec's forms.
3. **Report accuracy:** "29 captured outputs" should be 28.
4. **Report accuracy:** finding 3 overstates what reaches GitHub; findings 2 and 3 are unmarked external knowledge; "(L-P)" and the ahead-count wording have no primary; "is done" is premature.
5. **Log consistency (no leak):** hook.log prints the pre-fix copy of the privacy/hook log script; gate.log and the other logs have a few fixed "yes"/"no" lines that are not evaluated where printed.
