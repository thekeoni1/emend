# emend corpus coverage, 2026-09-16

Written by `scripts/corpus-coverage.py --verify` over the private corpus
(T2 P7, `specs/0005-corpus.md`). The corpus itself is never committed; this
record holds only ids, extensions, byte counts, producers, sources, tags and
counts, which is the whole of the script stdout reproduced below.

- Commit: 8f4388242d866b93a6da90dcd876a622561d0a2f, the commit the run was
  made at. This record is committed later, and the script it names was
  introduced at 0c3c6a3 and is unchanged since, so the three commits differ
  by design rather than by accident.
- Date: 2026-09-16
- Script: `scripts/corpus-coverage.py`, sha256 ff02af14c45cd056b6296433a5583c8d9c6020d86547a1c0a466d948cca76c21
- Command: `scripts/corpus-coverage.py --verify`, exit 0, no stderr
- `--verify` recomputed every file size and sha256 against the manifest.
- The body below is the whole of that run stdout. An earlier version of this
  record attested the script at 58063fb (sha256 2b8be584...), which the
  docstring correction at 0c3c6a3 superseded; independent check 2 found that
  no current log showed the attested script producing this body.

---

```
files 27
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
tag plain 27 excel-win,libreoffice
tag strings 27 excel-win,libreoffice
tag formulas 21 excel-win,libreoffice
tag names 16 excel-win,libreoffice
tag merge 18 excel-win,libreoffice
tag validation 15 excel-win,libreoffice
tag hyperlinks 9 excel-win,libreoffice
tag autofilter 10 excel-win,libreoffice
tag freeze 8 excel-win,libreoffice
tag cf 11 excel-win,libreoffice
tag tables 11 excel-win,libreoffice
tag charts 11 excel-win,libreoffice
tag pivots 4 excel-win
tag slicers 2 excel-win
tag timelines 1 excel-win
tag macros 0 -
tag comments 6 excel-win,libreoffice
tag images 8 excel-win,libreoffice
tag external 2 excel-win
tag sparklines 4 excel-win
tag protection 5 excel-win,libreoffice
tag date1904 2 libreoffice
tag large 2 libreoffice
uncovered 1: macros
single-producer 7: pivots,slicers,timelines,external,sparklines,date1904,large
```
