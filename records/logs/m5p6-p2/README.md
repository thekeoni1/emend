# M5 part 6 P2 logs

specs/0037-measurement-and-close.md, "P2: the runs", and planning's rulings
H to R, held uncommitted in the working spec (sha256
ce89ba9c957a650667ed1c856934be86d5327f6741db24e74e7d346c9ebff8d0). The
phase commit is 5e52758e379966672baa2e350e4bf997346aee01. `<phase>` is the
phase's directory, `<home>/.cache/emend-build/m5p6-p2`.

| Log | Last line | sha256 |
|---|---|---|
| gate.log | gate log checks all as expected: yes | da7b322fc50f33c73727a73f34f966fd0c6af6769b0a901ef0e6a498081c1a5f |
| test.log | test log checks all as expected: yes | 1c3bdc6b393d44889e26e7bb4b90b620b0fba5a1ff81626998f5c28b6e7a71f0 |
| negative.log | negative log checks all as expected: yes | b74a1500eb10f0a37a86732a2ea920b30cacefd2aebd32ba5fea0bda041fb237 |
| run.log | run log checks all as expected: no | 66e39a92f14a56e2ae27115678358c06995e7875ee906e1fa96e604b0222e5b4 |
| check.log | check log checks all as expected: yes | 41f34b4053312ead5c405c816053e7c9d4738dc1b70d23f6ca2c25261e792133 |
| scan.log | written last, by the scan | |

## The run log FAILED, and stays as it was written

run.log ended at 2026-10-16T23:11:51Z with exit 1 and "run log checks all
as expected: no". Every run, the keep and every record were written, and
its outer standard error is empty. Three of its checks failed. Each is a
defect in the run script's checks, not in the measurement (rulings Q and R):

1. `keep_pairs_ok` is called by the keep's check and never defined in the
   script. M5 part 5's run scripts defined it, and the P2 generator dropped
   the definition.
2. The script's own standard error is 105 bytes, the one "keep_pairs_ok:
   command not found" line.
3. The record check's "openpyxl: the xml families are the changed entries'
   families" read no for the openpyxl image op alone. openpyxl re-encodes
   the images, so the media family's changed entries have no XML
   comparison row.

Ruling R: nothing was run again. check.log reran the checks with the fix
named, over the run log's run directories, keep and records:

- it names run.log by sha256 and reads its failing lines (exactly those
  three);
- it matches all 143 installed records and the keep's listing against the
  sha256 lines run.log printed;
- it runs the restored `keep_pairs_ok`, and the corrected record check over
  all 70 record pairs, with ruling Q's planted controls;
- it prints the tree.

## The scripts (under `<phase>`)

| Script | sha256 | Wrote |
|---|---|---|
| superseded/run-script-3/m5p6-p2-run.sh | 04ce36719652ec524a81420779a974cfa07899e0acb79ece4508f0a0f985546b | run.log (its standard error kept beside it as run.stderr, ffae65ef1c50ba1427855f9c212be01371752c37c0730cdfa252cc59a2060b83) |
| scripts/m5p6-p2-run.sh | 7c32179e787acdd106f86b5fbc0577c0a4ada987083144c6fe98f860ef3113e6 | nothing; the regenerated run script (changes RK and RL, ruling Q), which the close-out's generator starts from |
| scripts/m5p6-p2-check.sh | 84064a18fc97127c2c47a03426c8298b0e70ae72e3a6f02f138f9d4b055194c3 | check.log |
| scripts/m5p6-p2-scan.sh | 568cd01f66077db767deda5be0ebe87eb8debf04aa37411502f6a3cfa75a807b | scan.log |

The regenerated run script and the check script both come from the
generators under `<phase>/gen`. The check script carries the regenerated
run script's record check, keep check and controls.

A first check log (sha256
b6dd7cee7e3a1d944a1ad4e14d6431b73e8270f81e360714614b6f29cebbb770), by a
script that did not yet print the spec pin the scan reads, is kept under
`<phase>/superseded/check-1`. It ended "check log checks all as expected:
yes".
