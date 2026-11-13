# M8 part 3 P4 logs

specs/0049-excel-check-and-release.md, "The publication and its
verification (P4)", with ruling Q, which carries specs/0045 rulings AN
and AO with this part's names. The phase commit is the P4 doc commit
031732b330847a200588e709e3895612051f5908. It committed the spec at sha256
54e5309dbdf01fb82a81b27de914a718916504adc9ac1cd47191e564e48942cf and
ROADMAP.md at sha256
63324e0b524fda5a394c23d84ea8975e191ae6ee3ec6b2a365dde872d7e28249. The logged run read the
same spec (verify.log line 5, PLANNING_SPEC). `<phase>` is the phase's
directory, `<home>/.cache/emend-build/m8p3-p4`.

The maintainer published emend 0.1.1 from a clone of the tag v0.1.1 on
the laptop (ruling Q). Planning gave its word on the generator. The
maintainer then said in the implementing session that the registry
lists 0.1.1, and the logged run was made once, after that.

## The logged run

| Log | Last line | sha256 |
|---|---|---|
| verify.log | verify log checks all as expected: yes | 631b714566d8e17f5a07b297609d2b600e9264107fb99926076b153f35700054 |

Run 2026-11-13T19:21:48Z to 2026-11-13T19:21:54Z; exit 0; the script's standard
error and the outer standard error 0 bytes each. FINDING lines printed:
0 (line 1378). The only line ending ": no" is line 60, the API-key line.

- The registry token variables are absent (lines 855-856).
- The tag v0.1.1 is a lightweight tag on the close-out doc commit
  635aaba (lines 37-38).
- The three inputs, each by curl with HTTP 200 (lines 861-869):
  - the crate file, 725807 bytes, sha256
    170be41f42a142818edc71662d16262a1bed9ae45d7f069054ac135f6a0d13c3;
  - the index entry, 1224 bytes;
  - the registry page, 5615 bytes.
- The index holds one line for 0.1.1, not yanked. Its cksum is the crate
  file's sha256 (line 881).
- The index still holds one line for 0.1.0, not yanked, cksum
  8e0aae29cb62627cf08ead813aa4280a77c4adc9aee4b820f87dd7e650fc74b2
  (lines 883-884; change QV4).
- The crate file has 76 entries, all under emend-0.1.1/: P3's 75 plus
  .cargo_vcs_info.json (lines 889 and 893).
- Against the tag's tree (git archive of 635aaba), line 1054:
  - 73 packaged files are equal to the tag's, 0 differ, and 3 are written
    by cargo;
  - all 72 files of the tag's crates/emend are packaged.
- .cargo_vcs_info.json: git.sha1 635aabad55bba14f9412a335c92da7b52d2679dc,
  path_in_vcs crates/emend, dirty false (line 1342).
- The normalised Cargo.toml names emend 0.1.1 with the tag manifest's
  keywords and categories (line 1355).
- The package's Cargo.lock differs from the tag's workspace Cargo.lock.
  It is the fourth file cargo writes: listed, read in full, not compared
  (line 1357, information, not a check).
- The registry page reads back the keywords (excel ooxml openpyxl
  spreadsheet xlsx) and categories (encoding parser-implementations),
  version 0.1.1 once, not yanked, with the index's cksum as its checksum,
  and version 0.1.0 once, not yanked (lines 1360-1375).

## Scripts

| File | sha256 |
|---|---|
| generator gen/gen.py | d40ef9193f2102e7c889e2ce4f90e38aa799a585204095e83ddb95b0be84e1b9 |
| script scripts/m8p3-p4-verify.sh | b5ee24651c4a1a972da76669c9c7b43d3d1f9871db8e34142a5aac13a237b971 |
| model model/m7p2-p4-verify.sh | 8c5123b6bbd81d8c5c98f64d00ee4ff35865ba8d68805ff990ac3e82c9b24312 |
| dry driver dry.sh | 997ba5798e16db1e8b897e7debc2a728f77a4192d014c0876a6f15c059f9c28f |
| logged driver logged.sh | ad7ae5a8a68d1d5ac65676cd790f87ef4f876fa41133282059c7886c7924ca51 |

The model is M7 part 2 P4's verify script, asserted equal to the script
printed in records/logs/m7p2-p4/verify.log. The generator replaces its
header with this phase's (keeping its changes PV1 to PV7 verbatim) and
makes the changes QV1 to QV5 in the body, each an exact replacement with
an asserted count. verify.log prints the script that wrote it.

## The dry run pair

One pair, d1, under `<phase>/dry/d1`, with no network. Its crate file
was built by `cargo package -p emend --offline --no-verify` in a local
clone of the tag. Its index entry and page were hand-written, with
0.1.0's cksum read from records/logs/m7p2-p4/verify.log.

- pos: exit 0, "verify log checks all as expected: yes", 0 FINDING lines.
- neg: one byte of the packaged src/lib.rs changed and the package
  repacked. Exit 1, one FINDING ("the packaged src/lib.rs differs from
  the tag's crates/emend/src/lib.rs") and one STOP.

The dry pos run's crate file had sha256
170be41f42a142818edc71662d16262a1bed9ae45d7f069054ac135f6a0d13c3, the same as the
published crate file's.
