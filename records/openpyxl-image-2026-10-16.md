# openpyxl image for the comparison, 2026-10-16

Written by the Milestone 5 part 6 P1 test log (`records/logs/m5p6-p1/test.log`) at the phase commit 1e29c4eb146b4806658508e8b6a5da42097a9aa2, from the values the commands it prints read; `specs/0037-measurement-and-close.md`, "The openpyxl image" (decision 1 A). The image is amd64 and runs on the host only, with `--network=none` and `--userns=keep-id`, one workbook at a time, read-only.

## Pins

| Item | Value |
|------|-------|
| Image tag | `localhost/emend-openpyxl:3.1.5-dfsg-2-deb13` |
| Image id | `894b239fc0d1cd90df5fd3e86d89d662adc9562504da38d1a4ea32666985cf63` |
| Image bytes | 211283030 |
| Image created | 2026-10-16 15:06:42.109204171 +0000 UTC |
| Base image | `docker.io/library/debian@sha256:abc9cb88a5587630d7f915f47b23b0668fe250fbfc6457aa4d52b534c1bbf73f` (the LibreOffice image's, `records/libreoffice-2026-09-15.md`) |
| Containerfile | `tools/openpyxl-image/Containerfile`, sha256 `6c09db87fbd7eb5d393f5084079a40793c08fd0b6b4b142d11344281c33632a3` |
| Package pin | `python3-openpyxl=3.1.5+dfsg-2`; `python3-pil` unpinned, its version below |
| Python | 3.13.5 |
| `openpyxl.__version__` | 3.1.5 |
| `PIL.__version__` | 11.1.0 |
| Builder | podman version 5.7.0, rootless (`true`), no elevation |

## Installed python3 packages (`/py-packages.txt`)

| Package | Version |
|---------|---------|
| python3 | 3.13.5-1 |
| python3-et-xmlfile | 2.0.0-1 |
| python3-lxml | 5.4.0-1 |
| python3-minimal | 3.13.5-1 |
| python3-openpyxl | 3.1.5+dfsg-2 |
| python3-pil | 11.1.0-5+deb13u4 |
| python3.13 | 3.13.5-2+deb13u5 |
| python3.13-minimal | 3.13.5-2+deb13u5 |

## Build

`podman build -t localhost/emend-openpyxl:3.1.5-dfsg-2-deb13 tools/openpyxl-image`, rootless, the network used at build time only; the Containerfile is committed and its sha256 is above. If Debian's suite no longer offers the pinned version, the build is pinned to snapshot.debian.org at this record's date, a ruling and not a silent change (specs/0037).
