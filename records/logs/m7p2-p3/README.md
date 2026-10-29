# M7 part 2 P3 logs

specs/0045-publish.md, "The release texts and the manifest (P3)" and "P3:
the release texts and the manifest", with planning's rulings AB to AH.
P3's doc commit 612986a committed the spec at sha256
8a515384b571822e0b32e7d6a6794b45532689c6be5adc745b48f232d38d9f2c and
ROADMAP.md at sha256
884c6e33d682db71e69dafd5aa99e1841f11d91816658162d1e7ce2ef29d676b.
Rulings AE to AH, written during P3, are in the working specs/0045,
uncommitted. The logged run took it at PLANNING_SPEC: sha256
64044f95716b69f54e39e5bb75ddc438e749ff2e12fa7b1df9b59ce359002bce (1294 lines). The phase commit is
f7cabbad1d3ed9332e8b656d648cdcd0671d7ee2. `<phase>` is the phase's
directory, `<home>/.cache/emend-build/m7p2-p3`.

root: <dry> stands for <phase>/dry, the phase's dry runs' directory (specs/0043 ruling Q)
root: <superseded> stands for <phase>/superseded, the phase's superseded directory

The phase's commits, on P2's record commit 16c083f:

| Role | Commit | Subject |
|---|---|---|
| doc | 612986a | M7 part 2 P3: commit planning docs (P2's verification and rulings Q to AD) |
| code | 790c488 | M7 part 2 P3: the manifest names the packaged files and may be published |
| docs | b016818 | M7 part 2 P3: the CHANGELOG and the README carry the fixes and both Excel checks |
| docs 2 (phase) | f7cabba | M7 part 2 P3: the Excel sentences say asked to open; the doc comment names charts |

## The logged run

| Log | Last line | sha256 |
|---|---|---|
| gate.log | gate log checks all as expected: yes | 9e76fec0cd29bf2c6af5987a1b176e5137c791e444b420885dfab874245e20eb |
| negative.log | negative log checks all as expected: yes | 8588b18399b749accebf25780b0d3345d6fdb45976e80c1081dce3c00a04d774 |
| run.log | run log checks all as expected: yes | 906e641baa62fdc1dfe26179b172a660106e7c83026af1ba1fea2da17ee08d20 |

- The gate (P2's gate with changes A95 to T95), run at
  2026-10-29T14:28:09Z (its phase line):
  - the root's mount propagation shared, one keepalive process;
  - the five roles by full subject, in order, the second docs commit the
    phase commit;
  - every message ASCII and ending with the Co-Authored-By line the
    session's harness gives, part 1's "(1M context)" form refused (T95);
  - the prune: "nothing is removed: the prune set is empty"; P1's
    directory and its keep unchanged at the prune and at the end;
  - crates/ changed in crates/emend/Cargo.toml by the code commit alone
    and in crates/emend/src/lib.rs by the second docs commit alone;
  - check.sh: PASS host, i686, fmt, clippy, doc, native, licences,
    hygiene and privacy.
- The negative (P2's stage negative's frame, changes NP65 to NU65), run
  over a `git archive` of the phase commit, extracted with no repository
  above it (ruling AF):
  - (a) the doc commit's manifest, no include list: the list without
    template/_rels/.rels, `cargo package` exit 101 with one error line
    naming `_rels/.rels`, specs/0031 P2 ruling 1's finding;
  - (b) information: the code commit's manifest less its
    template/_rels/.rels line still lists the file;
  - (c) the control, the code commit's manifest: the file listed, exit 0;
  - the figure check refusing a planted figure, the order check refusing
    two figures swapped (NU65), and the feature-list check refusing a
    doc comment without `charts`.
- The run (P2's stage run's frame, changes TT to TY):
  - CARGO_REGISTRY_TOKEN and CARGO_REGISTRIES_CRATES_IO_TOKEN absent;
    `<home>/.cargo/credentials.toml` not there, by `test -e` alone (TY);
  - the package list: 73 entries in the checkout (--allow-dirty passed,
    planning's spec being modified), 72 in the archive (no git there, no
    --allow-dirty), the checkout's less .cargo_vcs_info.json,
    template/_rels/.rels once in each;
  - `cargo publish -p emend --dry-run` in the archive: exit 0, the one
    "aborting upload" warning, no other warning or error, "Packaged 72
    files";
  - the four Excel sentences (README and CHANGELOG, one per record), each
    holding its record's seven totals figures in the totals line's
    order, a swap refused (TY);
  - the CHANGELOG heading "## 0.1.0 (unreleased)" of ruling AB's form,
    another version refused;
  - the two limits read in both texts; the five Fixed items in the order
    of "The fixes (P1)", each citing part 1's record;
  - the doc comment's feature list naming the manifest's 12 features;
  - FINDING lines printed: 0.

## The session's statements (no log prints them)

- Generators and scripts, sha256:

  | Log | Generator | Script |
  |---|---|---|
  | gate | 2af2fe940f243f775a36a3d26eb79def34e8e5295dbc6a5cb372f1370ebcc25a | 5beb8bed5a5709da85bfdb9cffa84ae1fa3404f52d3a791df0dfd8a4a09f5932 |
  | negative | 1678d11ea0e62499e7b49304046c6c3424fb218558589e1686c9a0055ccb0d64 | 8a5d38e068a1fc7b4d5041ce30eb9bb9ced6e1d8377848892759a69428afc10e |
  | run | 3fdc87640f455455040df81b4ee1a1553b6c4c0aafa04a423411aa8f44162a1a | 144ecba7a16699924c5993f98181ada15c1e38bace4e23b71ec61608ee14e209 |
  | scan | afc64fc5495385fe96ce331a6bce1025e9c8ff37c872810286c9b5803c02a940 | 98b7a22366b359b784f57248b1ba0ca2b7022efa91bac8fb1696e8614ee78694 |

  The figure and feature checks the negative and the run share are
  `<phase>/fork-notes/shared-checks.sh` (sha256
  05acde9921b8e5835797aa15ded51f8bbce1c81bfdd09dd16a28ef56bbe8d3f0), carried byte
  for byte by both. The drivers: `<phase>/dry.sh`, sha256
  b16d9f6752863a787f8c97822d66d6b245a407d154993e678993f789c32e7bf8;
  `<phase>/logged.sh`, sha256
  85d2f66fa880a1d77a1b24ddd3aa8be9f207e97e63438d89ee8db7eceb6e7347.
- The generators were written by four forks of the session, one per
  script, from P2's generators and scripts, P1's gate and M4 part 2 P2's
  run (copied under `<phase>/model`). They ran no script and no cargo.
  The gate and run forks kept their own first scripts under
  `<superseded>/gate-gen1` and `<superseded>/run-gen1`.
- Before the generators, the session tried the manifest by hand in a
  `git archive` copy (kept under `<superseded>/try-archive`): the list,
  the publish dry run and the two negative forms ruling AF names.
- The dry runs, each from the start under `<dry>/d<n>`, each kept:
  - d1: the gate read no on four lines, the message check of the four
    commits: the carried form pinned part 1's "(1M context)" trailer.
    Change T95. The earlier gate is kept under `<superseded>/gate-d1`.
  - d2: gate, negative and run yes; the scan pre-check FAIL on its two
    dry-layout lines alone, hits 0. The run printed "so --allow-dirty is
    passed--allow-dirty for its list"; its echo was fixed within TU.
    The session removed the earlier run script with `rm -f` after
    copying it to `<superseded>/run-d2` (ruling AH: moved, not removed,
    from here on).
  - d3: the same as d2, the echo fixed.
  - d4 (ruling AH: the order check, TY and NU65; the credentials line):
    gate, negative and run yes; the pre-check FAIL on its two dry-layout
    lines alone, hits 0. The earlier scripts are kept under
    `<superseded>/d3-gen`.
- The logged run (PLANNING_SPEC 64044f95..., ruling AH) started
  2026-10-29T14:28:08Z by logged.sh. Gate, negative and run each exited
  0 and read "yes", with 0 bytes of outer standard error each. The
  session copied the three logs from `<phase>/logged` here, each
  cmp-equal. The tree after the run: " M specs/0045-publish.md" alone.

## After check 1

Check 1 read the spec, the range 16c083f..f7cabba and its four commits,
the three logs, this README as it stood then, the scan pre-check, the
two Excel records and the draft report
(`<phase>/report-draft.frozen.md`). Its answer is committed as
`independent-check-1.md` (sha256
700798b8cb05eed4067839026a2e41ea9bc23f49fd071a052bab4efe766f7bf2), verbatim but for
the home path, as its heading says; the answer as given is
`<phase>/check1-answer.md` (sha256
34acdcf43da61d351feda5c2417fb879cc2898603df7074a39a8272744a8c881).

Verdicts: 1 yes; 2 yes with exceptions; 3 yes with exceptions. It found
no disagreement on the diff, the figures or any log's verdict. Its
points and what was done:
- E1: no log line says whether check.sh's hygiene step reads
  `publish`; the check found it does not, and the gate prints PASS
  hygiene. Recorded, no rerun.
- E2: the figure and order controls plant into the README's sentences
  alone; the CHANGELOG's go through the same functions. Recorded.
- E3, D3: the steps' exit codes are the driver's (`<phase>/logged.out`),
  the session's statement above; the report marks them so.
- D1: run.log holds the word FINDING on 16 lines, 18 times (two lines
  hold it twice); the report says lines.
- D2: d1 stopped at its gate, so it ran no scan pre-check; the report
  says d2 to d4.
- D4: the README carries the Excel sentences, not the fixes, which are
  the CHANGELOG's "Fixed"; the report says so. The docs commit's subject
  stands.
- D5: `<superseded>/gate-gen1` and `run-gen1` are named above.
- The include list's "README.md" entry matches no file under the crate;
  cargo packages the README through the `readme` key at the root
  whether or not the entry is there (the session's trial before the
  code commit). Recorded.
