**Independent check, M2 part 4d P4** (checker directory `<phase>/check1`, inputs only)

## Input integrity

I ran `sha256sum -c` against `<phase>/check1/inputs.sha256`. All 17 inputs match: commit-messages.txt, gate.log, test.log, negative.log, size.log, run.log, scan-pre-check.log, phase.diff, spec-committed-0022.md, report-draft.md, the three record pairs and the size record. I read no other path.

## Question 1: does the diff do what the spec says for P4, and nothing else?

Verdict: YES, with one item I could not verify from the inputs and two doubts, all stated below.

**Doc commit (spec and ROADMAP)**
- phase.diff 418-571 adds two sections to specs/0022: "Planning rulings during P3" at 455-482 and "Planning verification and rulings on P3" at 491-570. It also carries the two corrections those rulings made, at 422-433 and 434-448.
- phase.diff 9-10 changes ROADMAP's P3 row to DONE, as ruling 6 (spec 1162-1164) asks. The P4 row stays PLANNED (11).
- gate.log 1841-1863 confirms both blobs match the hashes planning sent: specs/0022 at 88d91f40c9b03a080f0972af75e31b90648b0b8201ae4e6120b81a033a76b1b5 and ROADMAP at eaf4149f9de9f3ebe6fbbc2417a61360824fc813f8e0ae79bbdf09884dc8eff5. The doc commit carries exactly those two files.

**Ruling 1's four carry items (all in 5906712; gate.log 1953-1957 shows it touches only package.rs and tests/images.rs)**
- (1) Checked steps in image_header (phase.diff 224-262).
  - A closure `after` uses `checked_add` with IMAGE_NOT_PNG_OR_JPEG (226).
  - It replaces every step the ruling names: `at + 1` (235, 236, 239), `at + 5` and `at + 7` (244-245), `at += 2` (254), `at + 2` (257), and `2 + length` (262).
  - Doubt: the body of `u16_at` is not in the diff, so I cannot tell whether it does any unchecked arithmetic of its own. The ruling did not name it.
- (2) Images follow the drawing element's r:id (phase.diff 138-191 and 194-216).
  - A new shared `drawing_of` replaces `drawing_rel`. Both `images` (141-142) and `record_images` (216) use it. A sheet with no element returns `None`, which gives an empty list (171-173, 142-143).
  - Test at 275-338: two drawing relationships, the element names rId2. The read gives drawing2's entry and the write appends to drawing2 (307-328). A sheet with no element reads empty (329-337).
  - Side effect, consistent with the ruling's "through the shared reader": `images` can now refuse with "a worksheet has two drawing elements", and its doc comment says so (126-131).
  - The `RelTarget` cfg loses `images` (100-101) because `drawing_rel` is gone. This is a necessary consequence, not extra scope.
- (3) Strict sheet's created drawing (phase.diff 340-384). The drawing uses the XDR_NS and A_NS namespaces. The drawing's rels, the sheet's `xmlns:r` and the sheet's rels use the strict relationship namespace.
- (5) drawing then legacyDrawing created in one write (phase.diff 386-417), gated on `comments`. It expects `<drawing r:id="rId3"/><legacyDrawing r:id="rId2"/>` before tableParts.
- Item (4) was accepted with no carry, and the commit message numbers the items (1), (2), (3), (5) (commit-messages.txt 46-53). That matches the ruling.

**Catalogue and limits (f5819ce, phase.diff 22-92; gate.log 1975-1977 shows 71 lines changed, all `//!` lines)**
- Messages marked by feature: **NOT VERIFIABLE from the inputs.**
  - The new blocks name the feature that owns each block (23, 65), and each block names its read message (79-80).
  - Neither block lists the ten messages by feature. The commit message says "The messages were catalogued by feature in P1 and P3" (commit-messages.txt 35), but the earlier catalogue text is not among the inputs.
  - negative.log 1178-1206 reads the messages from the source constants, not from the catalogue.
- Shapes written and read:
  - Comments written: 36-45. Comments read: 62-63.
  - Images written: 69-78. Images read: 86-89.
- Never written:
  - Formatting runs, a comment's size or position beyond the fixed box, a threaded comment: 57-58.
  - A two-cell anchor, an SVG, an image removal: 83-84. The block also lists the absolute anchor, EMF, GIF, BMP, TIFF and `descr`, all taken from spec "Out of scope".
- What the read gives for a threaded comment: the legacy fallback (59-61).
- Ruling 6's items:
  - K counts v:shape root children: 39-42.
  - D's allocation reads the VML entries the write keeps: 42-44.
  - A commentList before authors, or a root other than comments, cannot be read: 52-54.
  - A shape of another ObjectType or none keeps the VML part: 46-51.
  - Ruling 3's image refusals: 79-82. All four are there: a pic with no blip, an empty wsDr on append, a cNvPr id outside u32, an external drawing or embed relationship.
  - The drawing found through the element's r:id: 77-78.
  - VML id uniqueness resting on distinct idmaps per part: 44-45.

**Probes and manifest (c1f6602)**
- Manifest (phase.diff 583-608): features `comments` and `images` plus two `[[bin]]` blocks. gate.log 2025-2036 shows these are its only non-comment changes, and there is no Cargo.lock change (1984-1986).
- Comments probe (614-1289) and images probe (1295-1974). I diffed the two sources. They differ only in the feature edit, the extra author and text arguments, the PNG constant and the last printed line.
- Images probe details:
  - Fixed cell AB10 (1335). `add_image` with `size: None` before the write (1640-1649). Count read back through `images` (1700) and printed last (1460).
  - I hashed the PNG constant at 1340-1346 myself: 82 bytes, sha256 d268586051b64827ed456d804e07a48de48145ac2c7f9b7b5b79c12964551b40. The run log's comparison constant, which is labelled as the tool's (run.log 1980-1989), gives the same 82 bytes and the same hash.
- "Format probe's work unchanged" cannot be diffed here because the format probe's source is not an input. size.log shows the behaviour instead:
  - Each probe prints the format probe's lines plus one (5862, 5946).
  - Its first output equals the fullstyles probe's byte for byte, and its third equals the sheets probe's (5864-5865, 5948-5949).

**Anything beyond the spec:** none found.

**Observations for planning (not diff faults)**
- The comments feature costs 57032 section bytes. That is above planning's own estimate of 40000 to 55000 (spec 69-71), and the draft report does not mention it.
- probe-emend-tables-write-xml moved 60 section bytes and 16 .text bytes with verdict "code" (size.log 4789, 4767). Its source causes are named (4417-4426), and the draft does not mention this either.

## Question 2: do the logs contain every line the spec requires?

Verdict: YES.

**Size log**
- Per-feature costs against the format probe rebuilt at the commit: size.log 4818. The size record repeats them in its figures table (size-m2p4d .md 111-112).
- Core growth against the format probe rebuilt at 3c5f0b7, both printed: 4803 (the rebuild), 4819-4820 (0, 0, 0), 4821 (not above 2000).
- Whole growth since b728fdf: 4822-4823 (556 sections) and 6190 (file 0, .text 288).
- Budget against 458752: 4824-4826 (435224; 23528 under the ceiling) and 4827 (0 against part 4c's figure).
- The nineteen earlier probes classified against part 4c's record: 4351-4415, 4525-4794 (summary 4790-4794), 4840-4861. Twentieth and twenty-first listed as new: 4236-4239.
- Cause control: 4522-4524, plus one per-probe check each (for example 4535). Manifest cause listed apart: 4457-4461, and per probe at 4464-4500.

**Negative log**
- Every message reachable through the binary: negative.log 1167-1597. The five reached are listed at 1599-1604.
- The five unreachable ones, each with its covering test: 1605-1610 and 1615-1625. Set checks: 1611-1614. Controls: 1626-1630.

**Both samples again, on both targets, with the size log's release binaries, matching P3 by sha256**
- The binaries are size.log's, by hash: run.log 1850-1866, against size.log 5977-5993.
- Image sample, host: run.log 2618-2722 and 2900-2904. Image sample, i686: 2725-2829 and 2933-2937. Byte-identical to P3: 2867-2938.
- Comment sample, host: 2941-3005. Comment sample, i686: 3006-3070. Byte-identical to P3: 3071.

**Record names**
- Image pair: run.log 3074 and 3223-3228. Comment pair: 3236 and 3261-3266. Open-check pair: 4237-4244. Size record: size.log 6195.
- All follow the spec's patterns (spec 817-823).

**Edit open check over the keep:** run.log 3444-4235. The previous record is named by its record commit at 3385-3387.

**Spec hash pin 88d91f40...**
- gate.log 23-25 and 1843. test.log 24-26. negative.log 63. size.log 4, 68 and 6206. run.log 4 and 1842-1843. scan-pre-check.log 8 and 51.

**End lines and first attempts**
- Each log's last line says all as expected: gate.log 2582, test.log 4136, negative.log 1663, size.log 6215 (with 6214 "checks not as expected: 0"), run.log 4269.
- RERUN_NOTE is unset in every log: gate.log 4, test.log 4, negative.log 3, size.log 5, run.log 8.

## Question 3: does every number and quoted string in report-draft.md appear in a primary?

Verdict: PARTLY. Every figure, hash and commit id checks out. The "Dry runs" claims, and three other statements, have no support in the inputs.

**Header and commits**
- L3: c1f6602 and the base 2b85b22093a3c3ee14dde3e9b0c4933b394f428b are at gate.log 1771-1775 and 1783.
- L6: 20b8c61, 88d91f40 and eaf4149f are at gate.log 1841-1863, and "unedited at the hashes" is supported by 1845 and 1850.
- L7-L9: the subjects are in commit-messages.txt 1, 20, 43, 61. "Ruling 1's four items" matches commit-messages.txt 45-53. The catalogue blocks are at phase.diff 23-92. The manifest features are at phase.diff 583-591.

**Logs**
- L11: "each pinning 88d91f40" and "all checks as expected" are supported (see Question 2). "First logged attempt" is supported by RERUN_NOTE being unset in all five logs.
- L12-16: the five log hashes match inputs.sha256 and scan-pre-check.log 1991, 2001, 2011, 2021, 2031.
- L17: diff hash ef918164... is at gate.log 1879 and matches the hash of phase.diff.
- L18: "RESULT: PASS" is at scan-pre-check.log 2247.
  - "Run before any check answer": NOT FOUND in any input.

**Dry runs (L21-L23)**
These attempts are not among the inputs. Only the fixes they describe can be seen.
- Negative, dry run 1:
  - Supported: the two families sharing the test name `the_call_refuses_each_rule_and_records_nothing`, and the fix of reading each constant's test from its own family's file (negative.log 260-264). The two controls named are at 1627-1628.
  - Not supported by any input: "two checks failed", "the comment one came first", "Dry run 2 passed", "lacked the dry gate log as its input", and "each failed attempt kept on disk".
- Size, dry run 1:
  - Supported: the script comment says "the size dry run found a pattern that matched no byte" (size.log 2820-2822), and the new check and control are at 5969-5970.
  - Not supported: "doubled escapes", "Dry run 2 passed", and the attempt being kept on disk.
- Scan:
  - Supported: the change BR text, including "size.log needs no exemption ... as part 4c P3's BL said", is at scan-pre-check.log 522-530.
  - Not supported: "passed at its first run", "run again", and "the first output is kept on disk". These are not in any input.
  - Doubt: the pre-check that is in the inputs shows RERUN_NOTE and RERUN_FILES unset (scan-pre-check.log 6-7), so it does not name an earlier pre-check as superseded.

**Figures (L26-L38)**
- L26:
  - 893 on both targets: test.log 2562 and 3503.
  - 890 and 893: test.log 3581.
  - 3 names added, 0 removed: test.log 3585-3588.
  - "Added exactly the carry's three": test.log 3629.
- L27: X1 to X4 are at test.log 4074, 4087, 4100, 4113.
- L28: negative.log has 10 messages (1206), 5 reached and 5 stated unreachable (1611), and each covering test holds its message (1615-1625).
- L30-L34: size.log 4818, 4820, 4823 with 6190, and 4825-4826. The size record repeats them (size-m2p4d .md 111-115).
  - The figures are 57032/61440/50096, 40960/40960/36384, 0/0/0, 556/0/288, and 435224 with 23528 under 458752.
- L35: "does not exceed 2000" is supported by size.log 4821. "No early message was sent" is NOT FOUND in any input; spec 76 only says planning is told in the report.
- L36, image run:
  - 27 rows, 27 ok, 0 refused: run.log 2650 and 2757.
  - 17 created=1 and 10 created=0: run.log 2714-2715 and 2821-2822.
  - changed 72, added 70: run.log 2654-2655 and 2761-2762.
  - 27/27/27/27 equal to P3: run.log 2900 and 2933.
- L37, comment run, 27/27/27: run.log 3002 and 3067.
- L38: the binaries matching size.log's hash lines are at run.log 1855-1863.

**Records (L41-L44)**
- All seven hashes match inputs.sha256.
- They also appear at run.log 3225, 3227, 3263, 3265, 4240 and 4243, size.log 6195, and scan-pre-check.log 2059-2119.

**Not done (L47)**
- "Nothing is pushed": NOT FOUND. No input covers pushes.

**Omissions planning may want added to the report**
- The comments cost exceeds planning's 40000 to 55000 estimate.
- The tables probe moved 60 section bytes with verdict code.
- No input confirms that the catalogue's existing text marks the ten messages by feature.