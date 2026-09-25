# Independent check 1, M2 part 1 P5

- Checked commit (the phase commit): 975c9742ea5bd2b5c68d489852af72e01197b217
- HEAD when the check ran: 975c9742ea5bd2b5c68d489852af72e01197b217
- Checker: a fresh general-purpose subagent, given only the directory <scratch>/p5/check1 and the three questions of CLAUDE.md
- Answer: 10859 bytes, sha256 62f1842f286354202d71894a41b2c6a57b9bb5ccda19b4d37e7dad16a8f4064f, committed below verbatim; it is plain ASCII and names no machine path, so nothing was replaced
- Inputs, each with its sha256 (sha256sum over <scratch>/p5/check1):

    3799334b6dd8c10ae9a557b565870eb60ea745497499ee7b4f9a180b773536ee  diff-6a0086e.patch
    89bfbed748159da27a5152b93c02ffb9a4b3173e8cbdb3320fe7c8039840d624  diff-95fb486.patch
    881398a9a35ecdfe6e5b0c9efa2a90b0d97638f242a3481afe8c1617054bc37c  diff-975c974.patch
    93fe3156254d3be62d94524fcf4cbb72c047ea7328226c7ab01aaf27b60a258c  diff-commands.txt
    3b16f2e3185d95971643706c0241c2f52d782064a154276504ca4929830f5864  diff-doc-703e0ff.patch
    f1ceb796dbe1a4e10771e14d76a0fcbe98120d8066dfe70c3c6abef31dda91b2  diff-phase.patch
    c4cc3e66e6d7f048b3d1bd86c772bb2631560dd62ae0767c4cc495c5c61b2a18  log-gate.log
    2f0797b2447c0300d28a5cfd34c3cb2e996da4c739c64e4a632aada5075d6edc  log-run.log
    8bf1da6e8e249a3fc2e285a202024e828b01d784c590b58e4b259fc72869af19  log-scan-pre-check.log
    dfc6b9ac547c87b5e01b3afd8df3f4551c267e3630b1edc7740fb594ddf4113a  log-size.log
    3fccfa848212a434d6064ffbc595d58b43b1e8b10f1a672faa32440c70c641d3  log-test.log
    b35dc6b7d417310abe95e8c07cc9b90b021b675140b2e20a604c933abbcfcc2a  record-m2p1-open-2026-09-25-975c974.md
    f8876f9e6e46d254f3a587d6318f728afff283dcdba3f9d7d25409a925f81934  record-m2p1-open-2026-09-25-975c974.txt
    0bc909ba8be783364531941bd15188b358bb609439b241a3a42dd3f44a6aecd2  record-m2p1-p5-2026-09-25-975c974.md
    b25bd58d1cd1753683aa75c5c0d08db1f3c27f33c53463b401bdbb9a8d6f5517  record-m2p1-p5-2026-09-25-975c974.txt
    7cf001167c0b8b89efdc4ade17d23e907c80284eaaa126142a8f8ac5356d1dd9  record-size-m2p1-2026-09-25-3bd4257.md
    764e13de0f365c4a0adab97c356804bb190aba52ce8eb16419a52250fc59435f  record-size-m2p1-2026-09-25-975c974.md
    439086d13adb8305a47bf9d8aabc3ae69ba1fc3d2422e48f4c3ec8339f78fde9  report-draft.md
    84c149d39875db7cd6ba6eff3e185ff4e2be56e474ee9ba151983cf2243ea31f  specs-0001-bootstrap.md
    432b581007f2614b1ebe18e724d54365f338affa0e198516da19fe111b194188  specs-0016-new-workbook-and-sheets.md
    230760568eb9338d6d31ca5aa3db88eb23c362df21ead62b0da0565eb4beb671  superseded-run-attempt-1.log

---

No CLAUDE.md was given to me in the check directory. The harness did put the project CLAUDE.md into my context, and I did not use it as evidence. I read only the files in <scratch>/p5/check1 and changed none of them.

**Question 1. Does the diff do what the spec and rulings say, and nothing else?**

Diffs read:
- For archive.rs and archive/tests.rs: diff-6a0086e.patch. Its sections are identical to the matching sections of diff-phase.patch, index lines aside.
- For package.rs and package/tests.rs: diff-95fb486.patch, plus the package.rs hunk of diff-6a0086e.patch. The sorted +/- lines of those two together equal the package.rs +/- lines of diff-phase.patch.
- For cells.rs: diff-975c974.patch, identical to its section of diff-phase.patch.
- For ROADMAP and specs/0016: diff-doc-703e0ff.patch, identical to their sections of diff-phase.patch.

F1. I confirm from diff-6a0086e.patch that the mechanism matches ruling 1:
- a private `ReadSeek` trait with its blanket impl, and `Reader = Box<dyn ReadSeek>` (lines 99-109);
- `Core` holds `Option<Reader>` and `Option<ZipArchive<Reader>>` (lines 110-137);
- `Archive<R>` is `Core` plus `PhantomData<R>`, and every public method forwards (lines 146-207);
- `open` boxes the reader, and `write_to` hands `&mut out` on as `&mut dyn Write` (line 204).

No xml.rs or Stream change appears in any diff (ruling 6).

F2. I confirm the public-signature point from diff-6a0086e.patch lines 147-155 and 364-386:
- The only public change is `where R: 'static` on `Archive::open` and `Package::open`.
- Each constructor's doc comment names it.
- The archive header states it (lines 34-42), together with the loss of Send, Sync, UnwindSafe and RefUnwindSafe (ruling 3).
- `Package::write_to(mut self, mut out: impl Write)` (diff-95fb486.patch line 518) differs from the base only by a `mut` binding, which is not part of the signature.

log-gate.log lines 1392-1402 show the same thing: 85 `pub fn` first lines at the base and at the phase commit, and the bound's line once in each file. Limit of that check: it compares first lines only, after dropping `mut`. It would not see a `pub fn` that stayed as a `pub fn` inside `impl Parts` without a forward. None of the diff hunks shows one, and the tests still compile and pass (585).

F3. The package bodies moved with only the named edits. I rebuilt the check independently over diff-95fb486.patch: I took the package.rs lines removed and not added back, whitespace-trimmed and counted as a multiset. What remains is:
- the `use` line;
- the three `calc_prs` dereferences;
- nine `&mut self.archive` to `self.archive` edits: parse_content_types, parse_relationships, parse_workbook_sheets, rewrite_worksheet, rewrite_workbook, new_rels_part, and three bare `&mut self.archive,` lines;
- the nine free-function signature lines;
- the `write_to` signature line;
- two lines rustfmt joined.

This agrees with the report's unlogged claim at report-draft.md lines 40-45.

The only other new body code is the `into_*` wrappers and the one-line `Parts::part`, both allowed by ruling 7 (diff-95fb486.patch lines 541-545 and 1142-1176). `into_relationships` uses `find` by source bytes where the original used `position`. Since `relationships()` never pushes a duplicate source, both return the same element.

F4. Some type-driven body edits outside package.rs are not named in the report's "Body edits beyond moving" item (report-draft.md line 40). That item covers package.rs only:
- diff-6a0086e.patch line 170: `Ok(Archive {` became `Ok(Core {`.
- diff-6a0086e.patch line 242: `&mut out` became `out` in `Core::write_to`.
- diff-975c974.patch line 63: `package: PhantomData,` left `Walk::new`'s literal.
- diff-975c974.patch line 78: the body of `Iterator::next` now sits under `Walk::next` with the signature `Option<Result<Cell, Error>>`.

Each is mechanical. None changes what is read, written or refused. Ruling 5 asks the report to say that no body changed beyond the dereferences, so these should be named.

F5. Nothing reads, writes or refuses differently. Evidence:
- The same parser runs on the same bytes through `Core::open`. That includes `compress_replacement` (diff-6a0086e.patch line 319), and `sole_entry_fields` keeps its refusal.
- log-run.log lines 1742-1747: every output is byte-identical to P4's run directory, 27 of 27 on each label.
- The open-check .txt sha256 f8876f9e... equals P4's record (log-run.log lines 2055 and 2440).

F6. Auto-trait leakage the report does not name. `Archive::read` returns `impl Read + '_` (diff-6a0086e.patch line 139), whose concrete type is now `ZipFile<'_, Box<dyn ReadSeek>>`. That returned reader therefore also loses Send and Sync for every R. Ruling 3's header sentence mentions only `Archive`. This is not a signature change, but a consumer can observe it.

F7. The test edits are exactly the seven lines rulings 4 and 7 name (diff-6a0086e.patch lines 339-374, diff-95fb486.patch lines 1297-1336), and no assertion changed. log-gate.log lines 1404-1440 confirm this with an undo that has a control.

F8. There is no `unsafe`, no manifest change and no lock change (log-gate.log lines 1256-1258 and 1401-1402). Each code commit touches only its own area (log-gate.log lines 1238-1254).

F9. Two things I could not check from these inputs:
- Commit message bodies. Only subjects, line counts and trailers are shown (log-gate.log lines 1110-1127), so I cannot check any counts or claims inside them.
- The proposed private-signature list that ruling 1 accepted. It is not among my inputs, so I cannot compare the report's "What changed against the accepted plan" list with it.

**Question 2. Do the logs contain every line the P5 section and the specs-0001 rules require?**

F10. gate.log: present and ends "yes". It has nine PASS steps on a cold cache, and lists the gate files checked against the diff, 0 in the diff, so no self-test is due (log-gate.log lines 1187-1236 and 1442-1455).

F11. test.log: host 585 and i686 585, both passing, with the same names on both targets. No test was added or removed, and the total matches P4's committed log (log-test.log lines 1982-2073).

F12. size.log covers everything the section asks for:
- P4's record is read as the previous record, at sha256 7cf00116..., which matches the file I was given (log-size.log lines 2450-2453).
- All thirteen probes are rebuilt at 3bd4257 in a scratch clone, and each reproduces P4's sha256.
- The classification is read off the bytes, with an identical control and a code control: identical 3, code 10.
- The sheets probe is 909200 against 1012208, -103008, with a -99999 control.
- Every earlier probe's difference from its P4 figure is printed (log-size.log lines 2880-2893).
- Budget 375984 is under 393216.
- The size record body printed in the log equals record-size-m2p1-2026-09-25-975c974.md byte for byte, as I checked by extraction.

F13. run.log (attempt 2) covers the run requirements:
- `--remove-sample` runs on both targets, using the release binaries whose sha256 matches size.log's `hash:` lines (log-run.log lines 1177-1190).
- 27 of 27 outputs are byte-identical per label, with a control (log-run.log lines 1742-1747).
- The open check matches P4's: 201 pairs, no verdict moved; crate 158, tool 43, 0 outputs refused on an accepted input.
- The corpus record pair is written, checked table by table and installed. Its hashes match the record files I was given.

F14. The rerun header works. log-run.log lines 26-39 give attempt 1's reason and the change, and hash attempt 1 as 230760568..., which matches superseded-run-attempt-1.log. Attempt 1's script and record pairs are hashed, and its run directory's summaries are hashed before removal. Attempt 1 failed only its final status check and the rollup: superseded-run-attempt-1.log lines 1531 and 1533 are its only "no" results apart from the ANTHROPIC_API_KEY line.

F15. Commit ids are read from git in every log, and each log compares the given phase commit with HEAD, with controls. My own greps over all five logs and the superseded log, covering the chained, leading-bang and assignment-prefix shapes, found 0 of each. I found no absolute machine path, no non-ASCII byte and no control byte in any log, record or the report.

F16. Placeholders are used without declaring root lines:
- log-run.log uses `<sizebuild>` 8 times (first at line 1180) with no `root:` line defining it. Its root lines, at lines 43-49, define `<build>` as the run's own build directory, which is a different directory.
- log-size.log uses `<repo>` (for example the clone command at line 749 of its printed script, and the "outside <repo>" check) and `<home>` without root lines of their own. Its root lines, at lines 42-44, cover only `<scratch>/p5`, `<build>` and `<scratch>/w`.

The substitution itself is correct. The declaration is what is missing.

F17. The scan log given is the pre-check stage only (PASS, 0 answers). The final scan is still to come, which the report states.

**Question 3. Does every number and quoted string in the draft report appear in a primary?**

Everything in the Commits, Figures and run.log sections, and all ten sha256 values, appear in a log, a record or a diff. I checked each hash against the files. The report marks the following as unlogged, and I confirm none of them has a primary:

F18. The Send/Sync compile probe (<scratch>/p5probe), at report-draft.md line 121.

F19. The gate runs on the trees of code commits 1 and 2: "nine steps each", the doc-step failure on commit 1's tree, and the clippy and fmt failures on commit 2's tree (report-draft.md lines 122-125). One consequence: ruling 5 requires each code commit to build and pass the tests on both targets, and for commits 1 and 2 no log shows that. Only this unlogged claim supports it.

F20. The dry runs (report-draft.md lines 126-128), and the removed-lines check on package.rs (lines 43-45 and 129). I reproduced the removed-lines check independently in F3, and it agrees. It is still unlogged.

F21. Two claims have no primary and are not marked as unlogged:
- The path `<home>/.cache/emend-build/m2p1-p5-keep-host-attempt-1` (report-draft.md line 117). log-run.log line 28 says only that attempt 1's host keep "was moved aside on the disk, not removed", with no path.
- The reason given for `let file: File` (report-draft.md lines 29-31), that `File` would otherwise be an unused import. The diff shows the edit (diff-6a0086e.patch line 311), but no log shows the reason.

F22. One phrase is a wording point, not a missing primary. "The log's 'largest growth' line reads 0, probe-zip" (report-draft.md line 91) is in log-size.log and the size record. But no probe grew, so the size record's sentence "the largest growth is 0 section bytes, `probe-zip`" names a probe that did not change as the largest growth. The report explains this correctly.

I have no other disagreements.