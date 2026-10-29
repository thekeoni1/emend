# M7 part 2 P4 logs

specs/0045-publish.md, P4 (the publication verified), with planning's
ruling AN. The phase commit is the P4 doc commit
27da22856b973ae29fd715370550f9ffa20c1309. It committed the spec at sha256
617333f9943bc16ec264f760153a70156531308821e38f52ac786764206641ee and
ROADMAP.md at sha256
c18006717449134c5bda86c337bc5fe266fb1828811b8891c8055d140b1137f7. Ruling
AN, written during P4, is in the working specs/0045, uncommitted. The
logged run took it at PLANNING_SPEC: sha256
e85e6c5cdb00f5c87f118f4b90672cb184f82b4e3972a7a40584c67e8bcbec08 (1580
lines). `<phase>` is the phase's directory,
`<home>/.cache/emend-build/m7p2-p4`.

The maintainer published emend 0.1.0 from the laptop (decision 4) and
said in the implementing session that the registry lists it. The logged
run was made once, after that.

## The logged run

| Log | Last line | sha256 |
|---|---|---|
| verify.log | verify log checks all as expected: yes | 2d082fd9d1b5bb06bcfa62bd793a552a478bb8607b5ad25c61de5ff76c57b4d2 |

Run 2026-10-29T22:32:48Z to 22:32:53Z; exit 0; the script's standard
error and the outer standard error 0 bytes each. FINDING lines printed:
0.

- The three inputs, each by curl with HTTP 200:
  - the crate file, 710099 bytes, sha256
    8e0aae29cb62627cf08ead813aa4280a77c4adc9aee4b820f87dd7e650fc74b2;
  - the index entry, 612 bytes;
  - the registry page, 3795 bytes.
- The index holds one line for 0.1.0, not yanked. Its cksum is the crate
  file's sha256, 8e0aae29cb62627cf08ead813aa4280a77c4adc9aee4b820f87dd7e650fc74b2.
- The crate file has 73 entries, all under emend-0.1.0/: P3's 72 plus
  .cargo_vcs_info.json.
- Against the tag's tree (git archive of cf727f7):
  - 70 packaged files are equal to the tag's, 0 differ, and 3 are written
    by cargo;
  - all 69 files of the tag's crates/emend are packaged.
- .cargo_vcs_info.json: git.sha1 cf727f700f3ddae476f2a83243f0cb296f1e239b,
  path_in_vcs crates/emend, dirty false.
- The normalised Cargo.toml names emend 0.1.0 with the tag manifest's
  keywords (excel ooxml openpyxl spreadsheet xlsx) and categories
  (encoding parser-implementations).
- The registry page has the same keywords and categories, and version
  0.1.0 once, not yanked, with the index's cksum as its checksum.
- The package's Cargo.lock differs from the tag's workspace Cargo.lock.
  This is printed as information, not checked (ruling AN, point 1).

## Scripts

| File | sha256 |
|---|---|
| generator gen/gen.py | bb96e9eca9207fcb9c689bd7bf84bd6e2b45a03ecc7adcbec918bd7d160c3140 |
| template gen/verify.tmpl | 09fa0ec8830252a9092d09e931f0f85c3c6140dd03e8bae27c54b6fe2796383c |
| script scripts/m7p2-p4-verify.sh | 8c5123b6bbd81d8c5c98f64d00ee4ff35865ba8d68805ff990ac3e82c9b24312 |
| model model/m7p2-p3-run.sh | 144ecba7a16699924c5993f98181ada15c1e38bace4e23b71ec61608ee14e209 |
| dry driver dry.sh | 1bfb56d610a3144c74b7386c2c79fdec79d10ebce98de7b3f04bc8fb864902ac |
| logged driver logged.sh | 67f5c5fb077cd34e09458ced0c3ec10703758bcdf9721cc9d3e7aab13e728636 |

The model is P3's run script, asserted equal to the script printed in
records/logs/m7p2-p3/run.log. The generator carries five blocks of it
verbatim: the helpers, the shape greps, the mktemp count, the API-key
check and the standard-error count. Changes PV1 to PV7 are the
template's own. verify.log prints the script that wrote it.

## The dry runs

Each dry run is under `<phase>/dry/dN` and uses no network. Its crate
file is built by `cargo package -p emend --offline --no-verify` in a
local clone of the tag, and its index entry and page are hand-written.
The neg run changes one byte of src/lib.rs in the package and repacks
it.

- d1: pos read yes; neg printed a FINDING and a STOP as intended. It
  found four defects in the script, all fixed before d2:
  - .cargo_vcs_info.json has no final newline, so the next heading ran
    onto it;
  - a self-compare control used the packaged lib.rs;
  - yanked printed as Python's False;
  - the Cargo.lock wording, and a missing blank line before the
    standard-error block.
  The d1 template and scripts are kept under `<phase>/superseded`.
- d2: pos exit 0, every check yes, 0 FINDING lines. Neg exit 1, with
  one FINDING ("the packaged src/lib.rs differs from the tag's
  crates/emend/src/lib.rs") and one STOP.
- The dry crate built to the same sha256 twice (027bad88). The
  published crate file was built on the laptop and has a different
  sha256 (8e0aae29). The verification compares the crate's files, not
  the archive's bytes.

## The points ruling AN accepted

1. Cargo.lock is a fourth file cargo writes, beside .cargo_vcs_info.json,
   Cargo.toml and Cargo.toml.orig. It is listed and read in full, but not
   compared. verify.log counts Cargo.toml.orig among the 70 equal files,
   compared with crates/emend/Cargo.toml.
2. The page check reads crate.keywords, crate.categories, and the 0.1.0
   version's yanked and checksum. The top-level keywords and categories
   objects are printed only.
3. The downloads use curl -sS --fail --proto =https --tlsv1.2
   --max-time 120, with the user agent "emend-p4-verify
   (https://github.com/thekeoni1/emend)".

Ruling AN also records the dry crate's equal sha256 on two builds
(027bad88).
