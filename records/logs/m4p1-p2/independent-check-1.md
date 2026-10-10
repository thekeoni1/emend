# M4 part 1 P2 independent check 1

Checked commit (the phase commit): 8902f875ec95db492c7058b6a078cca258d38cb2
HEAD when the check ran: 8902f875ec95db492c7058b6a078cca258d38cb2
Checker: a fresh general-purpose subagent (not a fork), given only the files below in <check>/in, where <check> stands for <home>/.cache/emend-build/m4p1-p2/check1.
Inputs (sha256, name):
  71a8c57878f151e2a460cf7dc37f30f70af8972644f98aa01367f13c2421fa70  api-review-m3p7-2026-10-09-09d2e0f.md
  928f537faf8e0622c2b6b47b88937655ac1482ebae8f18f02cbc210a643967d3  diff-04b7070.diff
  bb11544d2659b879e6ea16311771c04ca6ff49df0421b88dc6f5da2858cd0ca9  diff-5e7bb62.diff
  a3461b390cccbd6da1819153e171212775922081c89c3c690d7f8b3a62ffeb51  diff-8902f87.diff
  65e83e82b7b5d9377aa448cb814f341664068d86976c1990cd87f9fae72e75c8  diff-c68f350.diff
  5da0b9b22b46b411542296ffe996297d311fb642b3287159f538a54b33f6fdef  diff-cce4285.diff
  2b3e08d86cfb0837ce198c963f3ed7938e2ac4fbaaccb4b84467befa4405f4f2  diff-combined.diff
  555079e874a4241df90158638b7d8d415bd78210ea70c67e2ce0d67205dac1c5  diff-f7bb23e.diff
  2a21008304dcef00e54d7a30fa7ebc2908457a18a65a51d48454d0843b04fe0d  diff-ff0a480.diff
  a47eed5e514ca14c111306e4aa721e1b1348a79a72e2edd89eebd56f8e26f576  gate.log
  848febe93916fb9f0fdde88228409da921c18de08092792592cd67df8f8d8d3a  msg-04b7070.txt
  e0577aea0aea13e862014a71cfff4f00bc865360bacbeecd30f42f3fcbaf0047  msg-5e7bb62.txt
  7de1a4302b4cee5061a7de26a0f04eb15b3b67304071840656b0d88bd766695f  msg-8902f87.txt
  36fc8edd98122453387ab1c8d71af5d101f6e7ca2da3e773c874c71f121b16e0  msg-c68f350.txt
  a0f8fd309e7a6cd4f023b1fd925d2bdb48e61b6af5c2f7211cda9162bc22f1da  msg-cce4285.txt
  c5040eeaed331c0173d97b9c50a27c6f8f4e9d747ce4f41c13d8f055bcdac3a9  msg-f7bb23e.txt
  6ab842284934a0256f818b8dc427da2b6c40cb78532a044f38cf8ae1a8ddb1dc  msg-ff0a480.txt
  65c26046ab08b8d089d731da66205350b56bd0ee1442ed5ebafacd5d3e06f3b8  negative.log
  ad52b0589df54ec0ed434a6ab74343fba274d42fa6aad7c11a292df5603bc893  report-draft.md
  aaeab2fd0e51bd980a5216af83c29d856b170702e7b6a735bd54364fee44d2d2  scan-pre-check.log
  88a72d93a4dfd064f11581354bf877c24f85ff91d9c7d09741fc6f147ec2de73  spec-0001.md
  31cd96896ae13dbc50e9ff0ad6d15dbad979c58e88478ea6d5147a6e079c691f  spec-0030-committed.md
  157f5b7780d4ccfdd536b78fc8288e223f5d375ee8f85a6e839f5fdef1d28725  spec-0030-working.md
  698aa73cef3c15b74ab8aeab7315f5cf1087d070092dab9b42aab94d2d0c01be  test.log
The answer below is the checker's final text, extracted from its transcript and written verbatim: 8125 bytes, sha256 0e46518e384615e6b843c5c44145a6f14a0b026d00425b34fd5530c78329a653; it holds no non-ASCII character and no machine path, so nothing was replaced.

---

Note first: the harness put CLAUDE.md, a memory index, git status and other context into my prompt. I have not used any of it as evidence. Everything below comes only from the files in <check>/in.

**Q1. Does the diff do what the spec says for P2, and nothing else?**

Yes, with three small points of reading noted after the list.

- **The six changes, in the review's order.** Each change has its own commit, as ruling 2 allows (spec-0030-working.md:291-297). The order matches the review record (api-review-m3p7-2026-10-09-09d2e0f.md:618-623) and the spec (spec-0030-working.md:87-94):
  - c68f350 renames `Item` to `SharedStringItem`. I read diff-c68f350.diff:1-75. It touches only cells.rs and one test in package/tests/sheetread.rs.
  - 5e7bb62 renames `Items` to `SharedStringItems`. I read diff-5e7bb62.diff. It touches cells.rs and `Package::shared_string_items` in package.rs.
  - f7bb23e renames `Kind` to `CellKind`. I read diff-f7bb23e.diff by its hunk list (lines 1-400). The crate root's re-export follows (diff-combined.diff:399-400), as do the tests, dates.rs, the module docs and tools/emend-copy.
  - ff0a480 renames `Default` to `ContentTypeDefault`. I read diff-ff0a480.diff. It touches package.rs only.
  - cce4285 renames `Target` to `HyperlinkTarget` at its definition in sheet_read.rs (diff-combined.diff:510-511). The public path follows with no `as` (diff-combined.diff:431-434), as ruling 2 requires.
  - 8902f87 handles `prefix_len`. I read diff-8902f87.diff. Both methods get `#[cfg(test)]`, the `Archive` one becomes `pub(crate)`, there is no allow attribute, and the rustdoc says the method is for the crate's tests (diff-combined.diff:22-41). This matches ruling 3 (spec-0030-working.md:298-311).
- **No alias, shim or old-name re-export.** None appears in diff-combined.diff. test.log:1022-1024 counts 0 aliases at the phase commit, with a control that finds 2 in a fixture. test.log:1004-1021 shows each old name declared 0 times at the phase commit and 1 time at the base, and no `cells::` or `package::` path names an old type (0 now, 9 at the base).
- **Each renamed item's rustdoc names the old name once.** The added lines are diff-combined.diff:80, 181, 194-195, 445 and 508. test.log:1025-1042 counts 1 for each, with controls.
  - Small point: `ContentTypeDefault`'s rustdoc still opens with "One `Default` element" (diff-combined.diff:440). That refers to the XML element, not the old type name, and the "formerly" count is still 1.
- **rustfmt only on touched files.** The commit messages name lib.rs and emend-copy's main.rs (msg-f7bb23e.txt:8-11) and sheet_read.rs (msg-cce4285.txt:10-12). All three appear in the diff. The reformatted lines are the re-export order (diff-combined.diff:400), lines that the longer name pushed over the width (772-775, 784-785) and a match arm (529-534). The gate passes fmt (gate.log:4658).
  - Small point: msg-cce4285.txt:9-10 says the re-export list in package.rs was "laid out in rustfmt's form by hand". The report does not mention this.
- **Nothing else.** Using diff-combined.diff (828b7dc..8902f87), I confirm that the only non-code changes are ROADMAP.md and specs/0030, and both come from the doc commit 04b7070 (diff-04b7070.diff:1,14). The committed spec hashes to 31cd9689..., which matches msg-04b7070.txt:4 and the gate's pin. I cannot check ROADMAP.md's hash, because its blob is not among my inputs.
  - The doc edits beyond the bare rename are each tied to a rename: the grammar fix "an" to "a" (diff-combined.diff:231-232), the reworded `Default` doc (diff-combined.diff:441-447, explained in msg-ff0a480.txt:7-8), and the rewrapped module docs (diff-combined.diff:412-423).
  - One doc line is now longer than its neighbours (diff-combined.diff:62). That is style only.

**Q2. Do the logs contain every line the spec and rulings require?**

Yes, with one gap of interpretation.

- **Tests on both targets.** test.log:851-895 runs `cargo test --workspace` on host and i686: 1112 passed and 0 failed each (lines 872 and 894). test.log:897-957 runs `cargo test -p emend` on both targets with five feature sets (none, copy, comments, copy+comments, all), passing 514, 526, 558, 573 and 918.
  - Gap: the gate also runs clippy over 14 or more feature sets of the size-probe crate (gate.log:4761 onward). test.log does not run tests with those sets. Its section header (test.log:897) only claims the sets the gate uses for emend's own clippy and doc steps. Planning should decide whether "every feature set the gate runs" includes the probe crate's sets.
- **clippy and doc at 0 lines.** test.log:963-1000 shows every run exit 0 with 0 diagnostic lines. The gate's own runs show the same (gate.log:4688-4750).
- **Negative log.** NG1 builds a test with the old name `crate::cells::Kind::Number` restored. It is refused with exit 101 and 2 error lines counted, including `error[E0433]: cannot find \`Kind\` in \`cells\`` (negative.log:1956-1977). It has a control on the unplanted tree (NG1c, negative.log:1951-1954) and a check that the same test builds with the new name (NG1n, negative.log:1979-1997). NG2, NG2c and NG2t do the same for `prefix_len` (negative.log:1999-2050), using the option ruling 3 offers.
  - Small point: the NG2 plant is a non-test public method inside archive.rs, built with `cargo build`. It is not literally "a test". I accept it as a reasonable reading of ruling 3's wording, but planning may want to rule on it.
- **Environment line.** It is the first section of the gate log (gate.log:11-27). The API key is unset (gate.log:6 and 169-170).
- **Ruling 1's prune.** gate.log:39-87 shows exactly the six names, each sized with `du -sb`, 5418344195 bytes in total. Each is removed with `rm -r` (no `-f`) and checked gone. The 16 kept names, including `dry-logs`, are listed and checked still present, with controls.
- **The gate.** Nine PASS lines (gate.log:4656-4667). The log ends "gate log checks all as expected: yes".
- **Scan last.** The run times are gate 02:28-02:32, test until 02:33:49, negative until 02:34:02, then the scan pre-check at 02:34:18-02:34:22, ending "RESULT: PASS" (scan-pre-check.log:3227-3232). The final scan comes after this check, so it is not among my inputs.

**Q3. Does every number and quoted string in report-draft.md appear in a primary?**

All but one quote. These are supported:

- The commit hashes and subjects, from the msg files and negative.log:26-33.
- The base 828b7dc and its subject (gate.log:112).
- The three log hashes, which I recomputed with sha256sum and which match report-draft.md:38-40.
- "six names (5418344195 bytes)" (gate.log:41 and 75).
- "nine PASS lines" (gate.log:4667).
- "passed 1112 each, failed 0" (test.log:872 and 894).
- "514, 526, 558, 573, 918" (test.log:902-957).
- The two quoted negative error lines (negative.log:1973 and 2023; the second is quoted shortened with "...").
- "structural checks failed: 0" and "RESULT: PASS" (scan-pre-check.log).
- "formerly `<old>`", as a template (test.log:1027-1041).
- The dry-run disclosure about tree_count's grep stderr, which test.log:555-556 supports.

Not found in any primary:

1. report-draft.md:32 quotes rustc's warning "method `prefix_len` is never used" and says it appeared "twice". Neither the quoted text nor the count appears in any input. Ruling 3 only paraphrases ("never used", spec-0030-working.md:301), and no log captures that build. This needs either a primary or rewording as a paraphrase.
2. report-draft.md:45-49 says the first test dry run ended "no" on one line only, and that the gate and negative dry runs passed first time. None of the dry-run logs are among my inputs. Only the test.log comment backs up the tree_count part.

**Disagreements, stated plainly**

- The rustc warning quote and its count "twice" in the report have no primary.
- It is unclear whether "every feature set the gate runs" includes the probe crate's clippy sets. test.log does not test with them.
- NG2 plants a method rather than a test.
- The report does not mention that the package.rs re-export list was laid out by hand.

None of these affects whether the code change is correct.
