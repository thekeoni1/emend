# T2 P0 independent check 1

- Checked commit: d97b9ef2ec5e3deb45e5dc68ed63dd453f7a0f0b (T2 P0 phase commit)
- HEAD when the check ran: 384231f838f4ecd97c0b9b9300d070439fcd71c8
  (T2 P0 record commit with the four logs, local only)
- Checker: a fresh general-purpose subagent (not a fork), given only the
  inputs below
- Inputs, with sha256:
  - specs/0002-gate-corpus-additions.md
    9fac44a85e155ea248137974806bda68b00e4803864fa26feac4ef56b6af1c3a
  - specs/0001-bootstrap.md
    ebb2545a358b6c38b14dd00c177a3c47e6f36a332c79dfa14fcd4f380b231a37
  - the phase diff, `git diff 41e2919..d97b9ef`, saved as <scratch>/t2p0-diff.txt
    547f4d9cb4ff1dc71361700cda3597e387c8b17a898901b3bbae14ec48fbe599
  - draft report 1, <scratch>/t2p0-report-draft.md
    d18ea810086d672c256f4297b36973291ac1526853e3b8357c3994ff1866d00b
  - records/logs/t2-p0/gate.log
    43326ed211bf5ea961742156ba221278832754f979621deff2a1ca4899bf6aba
  - records/logs/t2-p0/privacy.log
    06b2a4e4c24e25750c759b057b375b4b15d59bfd945c31452c7ee20181d9c69f
  - records/logs/t2-p0/negative.log
    9cfccddffee31269791055699e08cdc129e08c737e058796f4b8d4cd6cb2dfe1
  - records/logs/t2-p0/scan.log
    f97bddb51b1aec0f9b7dce6a2a8ebc89cd43c4204b8927f0410b858ae4786ec4

---
**T2 P0 independent check 1**

My context holds a CLAUDE.md from the harness, and I did not use it as evidence. My evidence was the two specs, the phase diff, the draft report and the four logs under records/logs/t2-p0/. I did not run git, did not read any other repository file, and changed nothing.

## 1. Does the diff do what specs/0002 says, and nothing else?

Yes. Every numbered change is present and correct as written. I found no fail-open path and no printed path or matched text.

**Change 1, download metadata files**
- **privacy.sh (diff 186, 249-251).** Zone.Identifier paths are matched case-insensitively and anchored at the end of the name. Nothing excludes tests/fixtures/, and only the count is printed. N 500 shows the case under tests/fixtures/ failing.
  - A content finding inside such a file now prints a placeholder instead of the path (diff 280-281). Before this change, a Zone.Identifier file with a non-office name, or one under tests/fixtures/, would have printed its path.
- **check.sh step 7 (diff 109, 124-128, 136-142).** It matches the same form case-insensitively, counts only, and has a positive control (two of four names must match). N 837 shows it catching a staged file under tests/fixtures/.
- **Office suffix.** It is `($|[#~.:])` in both scripts (diff 106, 183). The check.sh control adds a colon form, and N f4 shows the self-test catching removal of the colon.
- **.gitignore.**
  - The extension form is widened to `[.:]*` for all 15 extensions.
  - The Zone.Identifier rule is a case-insensitive glob placed after the tests/fixtures/ exception, so it applies there too.
  - N 508-515 shows the file ignored both at the root and under tests/fixtures/.

**Change 2, content signatures (diff 222-243).** I checked each against the spec bytes and all match:
- OLE2 `d0cf11e0a1b11ae1`, 8 bytes read in full.
- xz `fd377a585a00`, bzip2 `425a68`, zstd `28b52ffd`, rar `526172211a07`, split zip `504b0708`.
- tar `7573746172` read at offset 257 for 5 bytes. It is only read when the file is 262 bytes or more, which is exactly the size needed to hold those bytes, so no real tar header is skipped.
- A blob that matches an earlier signature and also has tar bytes is still counted, just under the earlier name.
- The finding carries the "archive or compound-file signature" label and no path (diff 290).
- The existing tests/fixtures/ exclusion is unchanged; N 608 shows OLE2 under tests/fixtures/ passing.

**Change 3, self-test**
- I decoded the planted octal bytes (diff 321-327) and each matches its signature. The tar filler puts "ustar" at offset 257 and the file is over 262 bytes.
- The archive count goes from 3 to 10, the office count to 3, and the Zone.Identifier count to 2 (one at the root, one under tests/fixtures/).
- Diff 312-313 plant the older forms of patterns 10 and 11: a single backslash with lower-case Users, and forward slashes. In the shell they come out as written. Each is required under its own number, on lines 1 and 2 of its file (diff 337-340).
- The newer forms in the original fake file are untouched by the diff. N c1 and c2 are consistent with that file holding the newer forms.
- Because an exact count is required, removing any single signature would be caught.

**Change 4, exit statuses**
- **Email pipeline (diff 205-208).**
  - The full status array is saved immediately after the pipeline.
  - grep must be below 2 and the other three stages must be 0, or the script exits through `error`.
  - The pipeline always has four stages, so no index can be unset.
- **Signature helper.**
  - It returns 1 when `wc` or either `od` fails; with pipefail set, a failing `od` fails its pipeline.
  - The caller runs it in the main shell with `|| error`. The earlier version called `error` inside a command substitution, where it could not exit the script.
  - A read failure therefore never becomes a signature or a finding. N 679-684 shows exit 2 with no FAIL line.

**Nothing else in the diff**
- ROADMAP.md and specs/0001 changes belong to doc commit 9324e64. I cannot confirm from primaries that they are unedited, and the report marks that as session output.
- The remaining additions are the three reported departures, plus comment updates:
  - case-insensitive Zone.Identifier matching;
  - the placeholder for content findings in a Zone.Identifier file;
  - the step 7 count check, which I read as required by "matching step 7 forms wherever a path form applies".

**Weaknesses (not gate defects)**
- **d1 cannot tell which status fired.** Replacing `tr` with `false` may also kill grep with SIGPIPE, and grep's own check fires on that. From the code, stages 1 to 3 are checked. A test that breaks `sort` or `awk` while grep exits 0 would isolate the new check.
- **d2 only breaks the first `od`.** The offset-257 read and `wc` failures share the same return path but are not tested.

## 2. Do the logs contain every required line and follow the log rules?

Yes, apart from the minor notes at the end of this section.

- **gate.log**
  - Line 1 names the commit and HEAD (both d97b9ef).
  - The run starts from a cold cache, and lines 96-103 are the eight PASS lines in order.
  - stderr is 0 bytes, exit is 0, control-byte controls are present, status is empty after the run, and the final line reads all as expected: yes.
- **privacy.log**
  - Line 1 names the commit and HEAD, and HEAD equals main.
  - It scans 65 of the 65 commits on main: `PASS privacy`, 134 blobs, 0 findings, exit 0.
- **negative.log**
  - Line 1 names the commit and HEAD, and every clone is at d97b9ef.
  - Case (a): a1 FAIL, a2 FAIL, a3 ignored.
  - Case (b): b1 to b7 each FAIL with a single count-only line, and b8 PASS.
  - Case (c): c1 and c2 each exit 2 with the expected message.
  - Case (d): d1 and d2 each exit 2.
  - Case (e): the P6c negatives (a), (c) and (d) are all repeated and each shows its expected result.
  - All 30 captured outputs report 0 fixture strings, 0 control bytes and 0 non-ASCII bytes. I re-counted the 30.
  - Reading the output lines, I saw no fixture text, including names not on the fixture-string list, the random tails and the Zone.Identifier URL.
- **scan.log**
  - The script starts with the env bash line, calls /usr/bin/grep explicitly, and matches under LC_ALL=C.
  - Non-ASCII uses the `-P` form and the em dash is matched as its bytes.
  - Exit 2 is treated as an error, never as zero hits.
  - Every form has a positive control in the same command form: each pattern, the older forms of patterns 10 and 11, the hostname, each denylist entry, the email check, and a clean file.
  - The script is printed in full.
  - Its coverage:
    - the 5 diff files and 2 commit messages, each ending with Co-Authored-By;
    - the other 3 phase logs;
    - both files under records/logs/t0-close/, each equal to HEAD, with 2 files there at HEAD;
    - 0 hits and `RESULT: PASS`.
  - Line 1 names the diff range and HEAD.

**Log rules.** I independently counted all four logs and the report:
- No control bytes, non-ASCII bytes or em dashes.
- No hostname, no temporary session paths, no Windows user paths and no disallowed email addresses.

The only pattern-shaped strings are control fixtures inside the printed scan script (scan.log lines 52, 125, 141 and 142) and the fake self-test value at diff line 313. None is real data. Every check is printed once from a single evaluation.

**Minor notes**
- gate.log line 5 names the denylist location with the literal `$HOME` variable, not an expanded path.
- The empty-status lines print a double colon (gate.log 115, privacy.log 134, negative.log 846). This is cosmetic and was accepted before.
- negative.log lines 520-604 print the hex of each test blob's first bytes. Beyond the signature, these are only a digit, the constant word "fictional" and filler spaces. They come from the test itself rather than gate output, and they carry no random or private value.

## 3. Does every number and quoted string in the draft report appear in a primary?

Most do. I checked each of these against the diff, the logs or my own count, and they hold:
- the diff's 5 files, 130 insertions and 49 deletions;
- the log hashes, commit hashes and origin/main at e0a5175 with 4 local commits;
- 65 commits and 134 blobs, 10 fixture strings and 30 outputs;
- every N line number except the one below, and every quoted FAIL or ERROR string;
- the signature hex values, the counts 3, 2 and 10, and the 15 extensions;
- in S: mode 600, 7 entries, and the 5, 2, 3 and 2 coverage counts.

These are not supported by a primary:

| # | Report claim | Problem | Marked as session output? |
|---|---|---|---|
| a | "N 669 to 678" for od's rejection and usage text | Line 669 is the NOTICE line; od's text is lines 670 to 678 | No |
| b | Departure 9: "failed in cases d and f" and "each run exited 126" | Neither appears in any primary; N's script comment supports only that `mv` dropped the executable bit | No (only the re-test and the other dry-run cases are marked) |
| c | Departure 5: `od -j` fails when skipping past the end | Tool-behaviour claim with no primary; N shows empty offset-257 output with stderr discarded, which proves nothing either way | No |
| d | Finding 1: the pipeline shapes of `z_indices` and `hit_lines` | Neither function body is in the diff; the claim may be true but needs checking against privacy.sh | No |
| e | "matched with -i over the raw NUL-separated tree paths" | The `-i` and the `paths.z` input are in the diff; "raw NUL-separated tree paths" depends on `z_indices` internals that are not | No |
| f | 9324e64 "committed without editing" | Not verifiable from primaries | Yes |
| g | The `.gitignore` probe result | Not in a primary | Yes |

There is one mild overstatement: "S lists its message", for both commits. scan.log lists the commit hash and checks that the last non-empty line is the Co-Authored-By line, but it does not list the message text.

## Defects, ranked by severity

1. **Low (in scope: could let a push through).** Report finding 1 depends on function bodies that are in no primary and is not marked as session output. Planning should verify it against scripts/privacy.sh before acting on it.
2. **Low (in scope: possible private-name leak; existed before this phase, not introduced by it).** A blob with an archive or compound-file signature at a path that is not an office, corpus, Zone.Identifier or private path still prints its path in any content finding, for example a renamed old-format workbook. The archive line itself is count-only, as the spec requires. The exposure is limited to gate output, unless that output is logged.
3. **Low (report).** Departure 9's "126" and "cases d and f", and departure 5's reason, are not in primaries and not marked as session output.
4. **Low (report).** The line range "N 669 to 678" is off by one; it should be 670 to 678.
5. **Low (test strength, not a gate defect).** d1 cannot show which pipeline stage's status tripped the check, and d2 exercises only the first `od`.
6. **Very low (report wording).** "S lists its message" overstates what scan.log shows.
