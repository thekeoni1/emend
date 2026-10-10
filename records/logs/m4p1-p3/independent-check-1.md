# M4 part 1 P3 independent check 1

Checked commit (the phase commit): 669fa91aa580e9d70fdc69d49839e815e80778cb
HEAD when the check ran: 669fa91aa580e9d70fdc69d49839e815e80778cb
Checker: a fresh general-purpose subagent (not a fork), given only the files below in <check>/in, where <check> stands for <home>/.cache/emend-build/m4p1-p3/check1.
Inputs (sha256, name):
  71a8c57878f151e2a460cf7dc37f30f70af8972644f98aa01367f13c2421fa70  api-review-m3p7-2026-10-09-09d2e0f.md
  a1dcbdc67a205c46e7d7591cebf2d09a19e572b8a33ff781aa068d0246425bf7  api-review-m4p1-2026-10-10-669fa91.md
  29565947f4cd1b098d6623609185510e3b0a0e6b1557d5bd36b77b9f5629b515  diff-combined.diff
  4753b01ef99e814b31f31b0950fb8cf526e442691a9006691b4431c219450f78  gate.log
  004a6a174308c12c051f659308c3378e98f0987cdda2c580fd232c2032a84289  make_review.py
  1640fe3f6eef3b392cbfd8bbc9f42c63c48ac010b6d0db23bd839b1f8ca6753e  msg-669fa91.txt
  4f040bfe8e43e44a3fcce96975389f2c995d68998431f138aafe4368dd221e9f  report-draft.md
  551c32b025601b656c34ffc42e4322df2130591e6ef9337e64bb3ade57b51b08  run.log
  38623477d5c0a3331c7320922de40d2c47c13014acc19dc16716f97e485e63be  sample-records/m4p1-p3-anchor-2026-10-10-669fa91.md
  c4f796fbd0f550fe0f0f7c513a3a9ea93111fe89d12fb31f709d6ced30295e52  sample-records/m4p1-p3-anchor-2026-10-10-669fa91.txt
  35202bfff4f0b28dd07d056733017f49363419da3444b21de82eed62e9d69c81  sample-records/m4p1-p3-border-2026-10-10-669fa91.md
  58678f536a4475391b1eb9fe2f0089b58567d683a3e9a3b6d63339713359723c  sample-records/m4p1-p3-border-2026-10-10-669fa91.txt
  9bf198b44e17962e7d8872ef45bc43442334b6b11d221183620e45cfabe7d9c8  sample-records/m4p1-p3-cf-2026-10-10-669fa91.md
  bd8e8183032ee924b04f6e22a233768498639fe7475c769abcd219160a416a26  sample-records/m4p1-p3-cf-2026-10-10-669fa91.txt
  20a75125617ac3e8343fa89957c479ba272af6a9d94a84b91742260fefb41732  sample-records/m4p1-p3-commentbox-2026-10-10-669fa91.md
  db8555006b3c70ff5e815564c1ed9323bf3bef290daec8f3e0db4c55971dc4b1  sample-records/m4p1-p3-commentbox-2026-10-10-669fa91.txt
  a8d10b838ca838dc5f05b0da9659cd4a1450c1635cbbd6ad29f1f719e50c21a0  sample-records/m4p1-p3-copy-2026-10-10-669fa91.md
  7674a63de2673c96f560a1b41336410ac82ff79b5dd59b41ab82361f953131bd  sample-records/m4p1-p3-copy-2026-10-10-669fa91.txt
  6671fdecbc5b704e750a37cdfb0261552365aa70ad58b5386e4dcd7579fd29d4  sample-records/m4p1-p3-date-2026-10-10-669fa91.md
  cb31af85e18d8d657973424496233d8ac93118ef41934e67919a17debfaa0996  sample-records/m4p1-p3-date-2026-10-10-669fa91.txt
  8d05a92c6b2912e0ffdd5721bfdffdc389b85551a03f8a8cd283e8b1b1bc3c46  sample-records/m4p1-p3-delete-columns-2026-10-10-669fa91.md
  4a6b44367acb7937ee3d06c50935f4f18038e7bc1fecb55677cef374e6b0c5ff  sample-records/m4p1-p3-delete-columns-2026-10-10-669fa91.txt
  705ae7ae80b17a77214038b28dcea87902df4b54126f1fd9abf582438e095337  sample-records/m4p1-p3-delete-rows-2026-10-10-669fa91.md
  23918f4efd5640ed62802bb4df0d72c304031338c070353d35bfcf5f88530aeb  sample-records/m4p1-p3-delete-rows-2026-10-10-669fa91.txt
  228592c395fb3c5e1939a4b55d8b4b01aa3904229b4efcb4705a9e994e1b5820  sample-records/m4p1-p3-edit-2026-10-10-669fa91.md
  7b6f23fa894cbff37685c29e182949ff645ea01014c3659f2f3cbf16742d8579  sample-records/m4p1-p3-edit-2026-10-10-669fa91.txt
  368289c3d02dc2949665938b3de9e197700ebad3b2b58913e37b2c16179277e1  sample-records/m4p1-p3-edit-formula-2026-10-10-669fa91.md
  5b5a0427bcaebf038e88ae566f8be484516c8ab97671245752d203bac846c82c  sample-records/m4p1-p3-edit-formula-2026-10-10-669fa91.txt
  94cb6fe9d8c200c47a322ddbff92c52e0b06439920b1ae53adc5565222bed128  sample-records/m4p1-p3-font-2026-10-10-669fa91.md
  02cd74c6506c2a40f4a53c381be79d8f60bbe3f2250bb45111ac4f4f53446379  sample-records/m4p1-p3-font-2026-10-10-669fa91.txt
  176dbdaa58616d76526139d3cde364b91e62228a5ed1e082773f14bc1342423f  sample-records/m4p1-p3-format-2026-10-10-669fa91.md
  fa8badf256ca71c84139f9ca2e9b417b202194c06a972cd78c196fb24af1f133  sample-records/m4p1-p3-format-2026-10-10-669fa91.txt
  2f06ec90e4aa9f7b47753ee63ffb30a23a4df3982b88d2bf1ca74a6ef2f9a6e4  sample-records/m4p1-p3-formula-2026-10-10-669fa91.md
  cd35df70ff3ba3dc16b4d7712b07988e24d9145278b7a6afa7e8762a0a86af1c  sample-records/m4p1-p3-formula-2026-10-10-669fa91.txt
  3fd89989b03efb05cf519e9c9695e281a2861464c7cf07ecddc03fa95144f039  sample-records/m4p1-p3-insert-columns-2026-10-10-669fa91.md
  602bb711f82b8a3cb4bee92bb178c1c6d588084a4a288255aa03cf9e087cd861  sample-records/m4p1-p3-insert-columns-2026-10-10-669fa91.txt
  2d499c429f2c38ae2b71cd88bc4a3420d775d2dd1231e0bca05aacec683f6802  sample-records/m4p1-p3-insert-rows-2026-10-10-669fa91.md
  ba9de8dd1eaf2bce03e4684388e22e9a6be2d91b411708269378a872b57327be  sample-records/m4p1-p3-insert-rows-2026-10-10-669fa91.txt
  cd45afc60919a7768d870268327e9f8d8cec3617f3554ada0077624185ee608f  sample-records/m4p1-p3-link-2026-10-10-669fa91.md
  a292fa5a4644e7c1836dbae00f18189803f8d5a45f53a6985b45f5c47f4704f8  sample-records/m4p1-p3-link-2026-10-10-669fa91.txt
  c004818854e65dbe49dcedc98bda54bd4dc1c06e19592630c14a98b24b34b12d  sample-records/m4p1-p3-names-2026-10-10-669fa91.md
  5c582fa1ddf70a446200c0b7ef8f06c82702f6db0de93a6f10965f3b4a899dea  sample-records/m4p1-p3-names-2026-10-10-669fa91.txt
  8bb6d293129b3736b871f53e9c46dcc6716f8163589a36385e88cf2a3e10ff5a  sample-records/m4p1-p3-printarea-2026-10-10-669fa91.md
  2d6270b78a539ef3ea1bdb20c81eea161f407dd39e3ffdf10d3ab993e39ce9ae  sample-records/m4p1-p3-printarea-2026-10-10-669fa91.txt
  e554d6e6df3012ea7532ac73a55a95f98608ec66b58f178f7038ff57b3aa6f58  sample-records/m4p1-p3-props-2026-10-10-669fa91.md
  e1728a7c3f449f92058f8662dae61e36028e28f183349227f08e75a8764d5ca5  sample-records/m4p1-p3-props-2026-10-10-669fa91.txt
  8132b05674db0ab63d205248373b70be08d7d3eeadc54f8ebda4f8edc03cd147  sample-records/m4p1-p3-protect-2026-10-10-669fa91.md
  b646a0d076677cb8f916ce032f941767ba2f3229df5541387ccc4cd5d224ceb0  sample-records/m4p1-p3-protect-2026-10-10-669fa91.txt
  a7abc2661adc112ac67c58288c54724394d3ba745925d429180392e614149429  sample-records/m4p1-p3-remove-2026-10-10-669fa91.md
  200f51a31109fe8bc4a5335fc60e6562000fa3336dccac5ff89b81ea7b5f5a68  sample-records/m4p1-p3-remove-2026-10-10-669fa91.txt
  77cf7584e6869c13c5085f8f9f5d6f5f2d1feb5743e7376ccc3a04aeac112ee7  sample-records/m4p1-p3-removeimage-2026-10-10-669fa91.md
  5be20ab3bb81ad526442631b036788ba1908aa336c94b84c31db383e8fd29b28  sample-records/m4p1-p3-removeimage-2026-10-10-669fa91.txt
  55f55e719ee98a8236fed4254bc913a1ceaa80625ee4ec687413bd0cd18c59b4  sample-records/m4p1-p3-scale-2026-10-10-669fa91.md
  406a118292eb75176fec4267c2e187af57e2afcc5317c94795f366cd3a39693d  sample-records/m4p1-p3-scale-2026-10-10-669fa91.txt
  1146f2534e50c9dbd8934c00a34976b341cb704e76b47061a2c31bb83bd98ec5  sample-records/m4p1-p3-sheet-2026-10-10-669fa91.md
  55bd6b4c1cc2e0bf00b267b429474f377def10c23d8fab56d2a91c99f14078cb  sample-records/m4p1-p3-sheet-2026-10-10-669fa91.txt
  5ebd9761094ce7674b3b565879ad4a71b89d83553812677c8dacd9580025f6d4  sample-records/m4p1-p3-style-2026-10-10-669fa91.md
  1dc2d1dedc5a7f4c2cc135f97aed44278915712323fc62231a343d7ef03bb50a  sample-records/m4p1-p3-style-2026-10-10-669fa91.txt
  6e44d6a06aa94aeda2b85e57970e9f31140aa14fe324fb9fba4d59682719c8a9  sample-records/m4p1-p3-table-2026-10-10-669fa91.md
  562b790083538eb84271d574e1a7aa471e7dcf3a997dd3f2aa44b9d7ab0ef858  sample-records/m4p1-p3-table-2026-10-10-669fa91.txt
  8c5e4a880f394f47fc47ccc9cf76683759430b8b512f9512370b9d3a99ae070d  sample-records/m4p1-p3-validate-2026-10-10-669fa91.md
  70dff1498f953b343cda04fc3c9e0f909abfd080b0db027b269628523235b63d  sample-records/m4p1-p3-validate-2026-10-10-669fa91.txt
  7e3cca3211a09dfb63d724cafcbad6ac99ffc12dce222f87790d60b746efa3d9  scan-pre-check.log
  30e1a5dcb24616469e4200f4d860e1f3b9fd5839c0ff7cc1bfc45b1bae713130  size-m3p7-2026-10-09-09d2e0f.md
  80e18c0a5f11dcc28233ad62398447597b76a73fccca740b370a314690fa2be8  size-m4p1-2026-10-10-669fa91.md
  da5d73a95d5b269af2ce1fe08347e6a80a6817cbc78158abd9ce737df8c30724  size.log
  88a72d93a4dfd064f11581354bf877c24f85ff91d9c7d09741fc6f147ec2de73  spec-0001.md
  99f64b5f14d232c3f241cdeed654e2e318365b58e8e0221d444001405b302b40  spec-0030-committed.md
  edbc016afa6f6c5f8f66ae9cf8af763bdba5559d4967f96cf9f588462277c20f  spec-0030-working.md
  75fa6ccd7cf1d77a5de4da4bddf65d65531655fbc32855969fe4f047d288c0e3  test.log
The answer below is the checker's final text, extracted from its transcript and written verbatim: 6868 bytes, sha256 851edd7c243b861c28754675f819226d465c3e68db3226e3e438cfe4eddc0cfa; it holds no non-ASCII character and no machine path, so nothing was replaced.

---

I was given CLAUDE.md, the memory index and git status as context. I did not use any of it as evidence. Everything below comes from the files in <check>/in.

**Q1. Does the phase do what the spec and the P3 rulings say, and nothing else?** Yes, with two small exceptions.

- **No code change: holds.**
  - diff-combined.diff touches only ROADMAP.md (lines 1-12) and specs/0030 (lines 14-95). ROADMAP marks P2 DONE and leaves P3 PLANNED.
  - gate.log 3561 and 3563 say the range is planning docs only, with no code commit.
  - run.log 37 says the phase commit is the doc commit.
  - msg-669fa91.txt 4 gives the spec hash 99f64b5f. That equals the sha256 of spec-0030-committed.md and test.log 66.
- **Size, every probe's growth 0: holds.**
  - size.log 7537-7564: each of the 28 probes reads "0 section bytes ... yes". 7565 reads "0 of 28".
  - size.log 7470-7477: copy, copy pair and core growth are all 0. Budget 436992, "21760 under the ceiling", 0 less part 7's figure.
  - size-m4p1 record lines 103-115 and 228-231 agree.
  - Observation, not a spec breach: size-m4p1 line 109 gives probe-emend-read-write-xml the verdict "code" even though all its figures are equal. So its code bytes changed with no size change. The record says "0 probes moved" (line 129), which is true by figures, but the code-byte change is not attributed. Planning may want to note it.
- **Listing: holds.**
  - test.log 1666 and 1690: 544 rows.
  - test.log 1697-1722: 12 of part 7's rows are gone and 11 rows are new. That is the ruling's eleven renamed rows plus `Archive.prefix_len` gone.
  - test.log 1724-1726: both sets equal the typed sets. 1729-1734: the five renamed rows carry their P2 commits as `added`.
  - The api-review-m4p1 tail gives keep 544, rename 0, hide 0, in all 544. Its body has 544 rows, all keep.
  - Its sha256, a1dcbdc..., equals the formatter trial's file (test.log 1773). The formatter hash 004a6a17... equals make_review.py.
- **Exception 1, the spec map (ruling 3 asks for specs/0030 on "the five renamed rows"):** six rows carry specs/0030 (api-review-m4p1 lines 58, 122, 146, 149, 216, 547). The sixth is the crate-root re-export row (line 58), whose `added` moved to f7bb23e. make_review.py 36-38 maps by commit subject, so this follows from that design.
- **Exception 2, wording only:** the record's header (line 8) says "types, functions, constants and aliases". test.log 1760 says "... constants, statics and aliases".
- **Samples: hold.**
  - run.log 1213, 1283, 1398, 1468, 1583, 1653, 1768 and 1838: the four part-6 samples on host and i686, each "ok workbooks 27; outputs equal ... 27".
  - Host and i686 outputs are the same: run.log 1289, 1474, 1659, 1844.
  - There are 31 such summary lines (8 + 23), all in the 27/27 form except printarea.
  - Printarea (run.log 4235-4239): 21 equal, "refused workbooks 6; their error lines equal 6". The error ids are the six that spec-0030-committed.md 62-64 names.
  - Every run uses --libreoffice (for example run.log 1149). Host-only samples use --allow-single (run.log 1959).
- **Records named by 669fa91: hold.**
  - The 54 files in sample-records hash to the run log's "installed records/... sha256" lines (54 of 54 match). run.log reports "record files installed by this run: 54".

**Q2. Do the logs contain every required line?** Yes, except the final scan, which runs after this check.

- **gate.log:**
  - Environment inputs at 4-9; the environment line at 11-27 (propagation shared, one keepalive).
  - The prune of P2's five names at 39-80: du -sb, 5274453938 bytes, rm -r, checked gone. It comes after the environment line.
  - ANTHROPIC_API_KEY unset at 6 and 141-142.
  - Nine PASS lines at 4108-4116.
  - Ends "gate log checks all as expected: yes".
- **test.log:**
  - T1 at 1016-1735, including the per-file pub-line counts at 1655-1668 that ruling 2 asks for.
  - T2 at 1738-1762.
  - The formatter hash at 1765; the trial at 1768-1776.
  - Ends "test log checks all as expected: yes" (1793).
- **size.log:**
  - Per-probe growth at 7537-7565.
  - The base rebuilt at 09d2e0f (6670, 6991), as ruling 4 asks.
  - Ends "size log checks all as expected: yes".
- **run.log:**
  - Each sample has a per-workbook sha256 comparison (for example 4213-4234) and a summary line.
  - Ends "run log checks all as expected: yes".
- **Scan:** only scan-pre-check.log is present. Its RESULT: PASS is at 3827 and "structural checks failed: 0" at 3825. The final scan, the one that must come last, is not yet among the inputs. The report states this.
- No line ending ": no" is an unexpected result. Each one is an informational value: test.log 48, size.log 3309-3310 and 7472/7477, gate.log 141 and 3590.

**Q3. Does every number and quoted string in report-draft.md appear in a primary?** Most do. Five items do not.

Confirmed:
- All five sha256s in report lines 17-26 match the files.
- 5274453938 (gate.log 73); nine PASS lines.
- 544, "12 gone and 11 new", 125/5/120/120 (test.log).
- The formatter's printed line (test.log 1770).
- "0 of 28", 436992, 21760 (size.log).
- The run-log quotes, including the printarea one.
- 27 pairs, 54 files.
- The base subject (run.log 39); the 669fa91 subject (msg-669fa91.txt 2).

Not supported, or not as written:
1. **"ruling 4 (a)"** (report line 13): the working spec's P3 ruling 4 (spec-0030-working.md, the added block) has no item (a). The content (shortsha 669fa91) is in ruling 4, but the label does not exist.
2. **The dry runs** (report lines 31-33: "all passed first time", the staged record pair moved to dry-logs): no dry log is among the inputs. run.log 150 shows only what a DRY run would run. These are session statements without a primary here and should be labelled as such.
3. **The scan quote** "RESULT: PASS (structural checks failed: 0)" (report line 21) joins two separate lines, scan-pre-check.log 3825 and 3827. Both exist; the combined string does not.
4. **"written by the parent with the formatter"** (report line 26): the act itself is not logged. It is supported only indirectly, because the record's hash equals the logged trial's (test.log 1773).
5. **The unlogged listing trial is not disclosed.** Planning's P3 ruling 1 says the scratch trial that found 544 was unlogged "and is disclosed here and in the report". report-draft.md does not mention it. Line 18's "scratch trial" is the logged formatter trial (test.log 1764-1776), not the one ruling 1 means. This is the main disagreement: the report should add that disclosure.

**Disagreements, in short:**
- The report does not disclose ruling 1's unlogged scratch listing trial.
- "ruling 4 (a)" cites a sub-item that does not exist.
- The dry-run statements have no primary among the inputs.
- Six rows, not five, map to specs/0030 in the listing record.
- The size record gives the read probe a "code" verdict with every figure unchanged; this is worth planning's note.
