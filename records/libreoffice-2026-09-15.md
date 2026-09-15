# LibreOffice image for the open check, 2026-09-15

The pins below are copied from the corpus directory's private record
`tools/libreoffice.md` (sha256
541945cc19f6b86c8e16a04d3dfdd68da2366798c2cf81ee28c0917b8e8edb29), written by
the CORPUS session that built the image (`specs/0005-corpus.md` P2, verified by
planning 2026-09-15). Nothing else from that directory is copied here: no file
name, no workbook content, no path.

The image is used by `scripts/passthrough-run.sh --libreoffice <image>` for the
T3 P3 open check (`specs/0006-passthrough.md` P3).

## Pins

| Item | Value |
|------|-------|
| Image tag | `localhost/emend-lo:25.2.3-2-deb13u6` |
| Image id | `3f9baefc6e233a6298c12f7cfa091fb28f47c61e7f6edffd961229a5a5ca62f1` |
| Base image | `docker.io/library/debian@sha256:abc9cb88a5587630d7f915f47b23b0668fe250fbfc6457aa4d52b534c1bbf73f` (trixie-slim, linux/amd64) |
| Package pin | `libreoffice-calc-nogui=4:25.2.3-2+deb13u6` (Debian stable, 2026-09-15) |
| Installed LibreOffice packages | libreoffice-base-core, -calc-nogui, -common, -core-nogui, -style-colibre, -uiconfig-calc, -uiconfig-common, all `4:25.2.3-2+deb13u6` |
| Other packages | `fonts-dejavu-core 2.37-8` (unpinned; recorded) |
| `soffice --version` | `LibreOffice 25.2.3.2 520(Build:2)` |
| Builder | rootless podman 5.7.0, no elevation |

Non-LibreOffice dependencies from apt (`--no-install-recommends`) are not
pinned one by one: the package version pins every `libreoffice-*` package
exactly, and the base digest pins the rest of the userland. If Debian replaces
`+deb13u6` in `stable`, the corpus session's record says to rebuild against
snapshot.debian.org at 2026-09-15 rather than change the pin.

## Build inputs (sha256, from the private record)

```
913314680ca9351e69f2742b824695191c37ca39cea5c47478c27f8d480f5d6b  tools/image/Containerfile
2c09931b3928938209647c6739705518c3502ce7d3c0e87c06825b863b670860  tools/image/lo-entry.sh
44c82743c7ab3b76b8abb270aaab5d5cd6cd3456bee1a2bfde4b2ad209cab5db  tools/image/registrymodifications.xcu
42d66808d87f7995443c1d3433bc1b8f0a21191bc8336860efe6dee4b59980b2  tools/image/basic/Standard/Corpus.xba
```

## How the open check runs it

```
podman run --rm --network=none --userns=keep-id \
  -v <output directory>:/in:ro -v <scratch>:/out \
  localhost/emend-lo:25.2.3-2-deb13u6 \
  --convert-to ods --outdir /out /in/<file>
```

The image's entrypoint is `lo-entry.sh`, which copies the profile warmed at
build time and runs `soffice -env:UserInstallation=file:///tmp/lo-profile
--headless --norestore --nolockcheck --nologo "$@"`. The arguments above are
therefore the tail of the command `specs/0006-passthrough.md` P3 names; the
word `soffice` is not passed, because soffice would treat it as a file to open.

The image has no Java, by design: `libreoffice-calc-nogui` is installed without
a JDK or JRE, so every conversion writes exactly this one line to stderr:

```
Warning: failed to launch javaldx - java may not function correctly
```

Planning ruled on 2026-09-15 that the open check's stderr condition stays
strict with one allowance, matched as a whole line: stderr may hold no line
containing `Error` or `Warning` except that exact line. Any other line holding
either word fails the check, including any other javaldx wording. The corpus
session's own open check allowed the same line.

## What this conversion does and does not detect (measured 2026-09-15)

Measured on generated two-sheet workbooks in a scratch directory, with the
command above. Both findings are planning's rulings of 2026-09-15 and shaped
the check:

- **The exit status is not evidence of a good load.** Every case below exited
  0, including the three that failed to load anything. The stderr condition
  and the ODS conditions carry the check on their own.
- **A sheet whose part is missing or unreferenced is invisible here.**
  LibreOffice substitutes an empty sheet, so the table count still matches:
  a workbook naming a third sheet with no part and no relationship converted
  with three `<table:table` elements, the third empty; a workbook whose
  worksheet part was removed from the zip while `xl/workbook.xml` and the
  relationship still named it converted with the full table count. What
  catches a dropped worksheet part is `xlsx-diff`'s `dropped` rows, not this
  check. That division of labour is what T3 P4 and Milestone 1 rely on.
- What does fail: a truncated worksheet entry, a removed
  `[Content_Types].xml`, and a cut central directory. Each wrote
  `Error: source file could not be loaded` to stderr and no ODS at all.

## Profile settings baked into the image

- `/org.openoffice.Office.Calc/Formula/Load` `ODFRecalcMode=0` and
  `OOXMLRecalcMode=0` (always recalculate on load).
- `/org.openoffice.Office.Common/Security/Scripting`
  `RemovePersonalInfoOnSaving=true`.
- A Basic module `Standard.Corpus` used by the corpus session to generate
  workbooks. The open check does not use it: it converts with
  `--convert-to ods`, whose filter is `calc8`.
