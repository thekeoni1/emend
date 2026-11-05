# emend corpus coverage, 2026-11-05

Written by `scripts/corpus-coverage.py --verify` over the private corpus
(M7 part 3 P1, `specs/0046-corpus-maintainer-items.md`, after the intake's
two rounds). The corpus itself is never committed; this record holds only
ids, extensions, byte counts, producers, sources, tags and counts, which is
the whole of the script stdout reproduced below.

- Commit: f8f4ee11e69783c305d91af0ea2e41ab81211abc
- That commit is where the run was made (M7 part 3 P1's phase commit). This
  record is committed later; the script it names is unchanged since the
  2026-09-16 record, so the commits differ by design rather than by accident.
- Date: 2026-11-05
- Script: `scripts/corpus-coverage.py`, sha256 ff02af14c45cd056b6296433a5583c8d9c6020d86547a1c0a466d948cca76c21
- Command: `scripts/corpus-coverage.py --verify`, exit 0, no stderr
- `--verify` recomputed every file size and sha256 against the manifest.
- The body below is the whole of that run stdout: 65 lines, sha256
  31fa6d7e855f77b1ef3ba394421a2767a85ab2867564335aff308a407bfa2661, as the
  intake's coverage log (records/logs/m7p3-p1/coverage.log) printed it.

---

```
files 39
d8245b84086f xlsx 16710 libreoffice libreoffice-export plain,strings,formulas,names,merge,autofilter,freeze
f56906d42e0a xlsx 15789 libreoffice libreoffice-export plain,strings,merge,validation,hyperlinks,comments
0fbf4e5e3be9 xlsx 17407 libreoffice libreoffice-export plain,strings,formulas,cf,charts
bdfe649d5b97 xlsx 30622 libreoffice libreoffice-export plain,strings,merge,images
ae7e17525a35 xlsx 10783 libreoffice libreoffice-export plain,strings,formulas,names,protection
63e820d98612 xlsx 11722 libreoffice libreoffice-export plain,strings,formulas,freeze,date1904
054df34b1472 xlsx 38257 libreoffice libreoffice-export plain,strings,formulas,names,merge,validation,hyperlinks,autofilter,freeze,cf,tables,charts,comments,images,protection
9132ce5303d8 xlsx 8283997 libreoffice libreoffice-export plain,strings,formulas,large
ed2fbd304d51 xltx 15792 libreoffice libreoffice-export plain,strings,merge,validation,hyperlinks,comments
80adab651c77 xlsx 17140 libreoffice libreoffice-export plain,strings,formulas,names,merge,autofilter,freeze
9ada43d71c80 xlsx 16190 libreoffice libreoffice-export plain,strings,merge,validation,hyperlinks,comments
885819a9642e xlsx 17853 libreoffice libreoffice-export plain,strings,formulas,cf,charts
f2ba081da509 xlsx 31035 libreoffice libreoffice-export plain,strings,merge,images
16c3be622455 xlsx 11116 libreoffice libreoffice-export plain,strings,formulas,names,protection
a72460f992a8 xlsx 12069 libreoffice libreoffice-export plain,strings,formulas,freeze,date1904
f8459281d739 xlsx 38699 libreoffice libreoffice-export plain,strings,formulas,names,merge,validation,hyperlinks,autofilter,freeze,cf,tables,charts,comments,images,protection
ab0e6a529465 xlsx 8284163 libreoffice libreoffice-export plain,strings,formulas,large
f9efcac9808f xlsx 68407 excel-win ms-template plain,strings,formulas,names,merge,validation,autofilter,cf,tables,charts,pivots,slicers,timelines,sparklines
8aaebea134fb xlsx 149135 excel-win ms-template plain,strings,formulas,names,merge,validation,hyperlinks,tables,charts,pivots,external,sparklines
6778a242d1f9 xlsx 41906 excel-win ms-template plain,strings,formulas,names,merge,validation,hyperlinks,autofilter,cf,tables,charts,slicers,images
75387404b8af xlsx 82449 excel-win ms-template plain,strings,formulas,names,merge,validation,autofilter,cf,tables,pivots,comments,images
40d747fb5d9e xlsx 23197 excel-win ms-template plain,strings,formulas,names,merge,validation,autofilter,freeze,cf,tables
3084c0591814 xlsx 46339 excel-win ms-template plain,strings,formulas,names,validation,cf,charts,sparklines,protection
f3a462783621 xlsx 97184 excel-win ms-template plain,strings,formulas,names,merge,validation,autofilter,cf,tables,charts,images
ed4c55f07c69 xlsx 21395 excel-win ms-template plain,strings,names,merge,validation,hyperlinks,freeze,tables
7935c5119065 xlsx 224149 excel-win ms-template plain,strings,formulas,names,merge,validation,cf,tables,charts,pivots,images,external
18d1e8ba72b5 xlsx 70123 excel-win ms-template plain,strings,formulas,names,merge,validation,hyperlinks,autofilter,tables,charts,sparklines
91d2bdffa46a xlsx 6783 google-sheets google-sheets-export plain,strings,formulas,freeze
90a5b01f9541 xlsx 12205 google-sheets google-sheets-export plain,strings,formulas,hyperlinks
453a166dcdfe xlsx 225468 google-sheets google-sheets-export plain,strings,formulas,freeze,images
15205770ae5c xlsx 14161 google-sheets google-sheets-export plain,strings,formulas,freeze,comments
6e0be3d0f2b1 xlsx 35370 google-sheets google-sheets-export plain,strings,formulas,hyperlinks,freeze
4bd861696651 xlsm 13504 excel-win maintainer plain,strings,macros
7055450ada76 xlsx 10297 excel-win maintainer plain,comments
495c7b79b183 xlsx 17800 excel-win maintainer plain,strings,names,pivots,slicers,timelines
0d7a34c16e9d xlsx 9245 excel-win maintainer plain,strings,sparklines
9de2ee20fc6a xlsx 13632 excel-win maintainer plain,strings,names,charts
9956034c4326 xlsx 10176 excel-win maintainer plain,strings,formulas,external
2adafe76df11 xlsx 8549 excel-win maintainer plain
tag plain 39 excel-win,google-sheets,libreoffice
tag strings 37 excel-win,google-sheets,libreoffice
tag formulas 27 excel-win,google-sheets,libreoffice
tag names 18 excel-win,libreoffice
tag merge 18 excel-win,libreoffice
tag validation 15 excel-win,libreoffice
tag hyperlinks 11 excel-win,google-sheets,libreoffice
tag autofilter 10 excel-win,libreoffice
tag freeze 12 excel-win,google-sheets,libreoffice
tag cf 11 excel-win,libreoffice
tag tables 11 excel-win,libreoffice
tag charts 12 excel-win,libreoffice
tag pivots 5 excel-win
tag slicers 3 excel-win
tag timelines 2 excel-win
tag macros 1 excel-win
tag comments 8 excel-win,google-sheets,libreoffice
tag images 9 excel-win,google-sheets,libreoffice
tag external 3 excel-win
tag sparklines 5 excel-win
tag protection 5 excel-win,libreoffice
tag date1904 2 libreoffice
tag large 2 libreoffice
uncovered 0
single-producer 8: pivots,slicers,timelines,macros,external,sparklines,date1904,large
```

Gaps: uncovered 0; single-producer 8: pivots,slicers,timelines,macros,external,sparklines,date1904,large.
