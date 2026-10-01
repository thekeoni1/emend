#!/usr/bin/env python3
"""Generate one tier of the scale workbooks (specs/0023-scale.md, "The
generator").

usage: scale-gen.py <tier> <dir> [--force]

<tier> is one of t, a, b, c, d (the spec's tiers; t is the tiny tier the
generator's test uses). <dir> is the scale directory, outside the
repository: the workbook goes to <dir>/corpus/<id>.xlsx and its row to
<dir>/manifest.tsv, in the corpus manifest's form (specs/0005-corpus.md), so
the runner takes the directory as a corpus.

The content is fictional and deterministic from a fixed seed: numbers,
strings drawn from a vocabulary of made-up words held in this script (every
string unique on tier d), formulas of the form A1+B1 with cached values, date
serials with a date style, booleans. The package parts are the crate's own
template (crates/emend/template/, the parts Package::new embeds) with a
second sheet, a shared strings part, a date cell format and a core
properties part added. Rows are streamed into the zip entry in batches;
neither the sheet nor the shared strings table is held in memory.

Prints exactly one line on success:
generated <tier> <id> <bytes> <sha256> sheet_bytes=<n> seconds=<s>
Never prints a cell value or a string. A refusal prints one "refused: " line
on stderr and exits 1; any other error prints one "error: " line naming only
the exception class and exits 2.
"""
import datetime
import hashlib
import os
import random
import sys
import time
import zipfile

USAGE = "usage: scale-gen.py <tier> <dir> [--force]"
VERSION = "scale-gen 1"
SEED = 20261001
# rows, number of 24-column blocks, every string unique
TIERS = {
    "t": (100, 1, False),
    "a": (50000, 1, False),
    "b": (800000, 1, False),
    "c": (1048576, 5, False),
    "d": (800000, 1, True),
}
HEADER = ["id", "sha256", "bytes", "ext", "producer", "producer_version", "app",
          "source", "tags", "added", "original_name", "note"]
SOURCE = "generated"
BATCH = 1000
DATE_TIME = (1980, 1, 1, 0, 0, 0)
NS_MAIN = "http://schemas.openxmlformats.org/spreadsheetml/2006/main"
CT_SHEET = "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"
CT_SST = "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml"
CT_CORE = "application/vnd.openxmlformats-package.core-properties+xml"
REL = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/"
REL_CORE = "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties"
CORE = (
    '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>\n'
    '<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties"'
    ' xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:dcterms="http://purl.org/dc/terms/"'
    ' xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">'
    "<dc:creator>corpus</dc:creator><cp:lastModifiedBy>corpus</cp:lastModifiedBy>"
    '<dcterms:created xsi:type="dcterms:W3CDTF">2026-10-01T00:00:00Z</dcterms:created>'
    '<dcterms:modified xsi:type="dcterms:W3CDTF">2026-10-01T00:00:00Z</dcterms:modified>'
    "</cp:coreProperties>"
)
# One 24-column block: n number, s shared string, f formula over the group's
# first two numbers, d date serial, b boolean. 12 n, 6 s, 3 f, 2 d, 1 b.
BLOCK = "nnnnssf" * 3 + "ddb"


class Refused(Exception):
    pass


def vocabulary():
    """1000 made-up words, two or three consonant-vowel syllables each."""
    rng = random.Random(SEED)
    words, seen = [], set()
    while len(words) < 1000:
        w = "".join(rng.choice("bdfgklmnprstvz") + rng.choice("aeiou")
                    for _ in range(rng.choice((2, 3))))
        if w not in seen:
            seen.add(w)
            words.append(w)
    return words


WORDS = vocabulary()


def unique_string(i):
    """The i-th string of tier d: two words and the index, so every one differs."""
    return "%s %s %d" % (WORDS[i % 1000], WORDS[i // 1000 % 1000], i)


def column(i):
    s = ""
    i += 1
    while i:
        i, m = divmod(i - 1, 26)
        s = chr(65 + m) + s
    return s


def template(name):
    root = os.path.join(os.path.dirname(os.path.realpath(__file__)), "..", "crates", "emend", "template")
    with open(os.path.join(root, *name.split("/")), encoding="utf-8") as f:
        return f.read()


def insert(text, anchor, new, before=True):
    """text with new placed before (or instead of) anchor, which occurs once."""
    if text.count(anchor) != 1:
        raise Refused("the crate's template no longer has the anchor the generator expects")
    return text.replace(anchor, new + anchor if before else new)


def parts():
    """The small parts, from the crate's template with this workbook's additions."""
    ct = template("[Content_Types].xml")
    ct = insert(ct, "</Types>",
                '<Override PartName="/xl/worksheets/sheet2.xml" ContentType="%s"/>'
                '<Override PartName="/xl/sharedStrings.xml" ContentType="%s"/>'
                '<Override PartName="/docProps/core.xml" ContentType="%s"/>' % (CT_SHEET, CT_SST, CT_CORE))
    rels = insert(template("_rels/.rels"), "</Relationships>",
                  '<Relationship Id="rId2" Type="%s" Target="docProps/core.xml"/>' % REL_CORE)
    wb = insert(template("xl/workbook.xml"), "</sheets>", '<sheet name="Sheet2" sheetId="2" r:id="rId3"/>')
    wbrels = insert(template("xl/_rels/workbook.xml.rels"), "</Relationships>",
                    '<Relationship Id="rId3" Type="%sworksheet" Target="worksheets/sheet2.xml"/>'
                    '<Relationship Id="rId4" Type="%ssharedStrings" Target="sharedStrings.xml"/>' % (REL, REL))
    styles = template("xl/styles.xml")
    xfs = '<cellXfs count="1"><xf numFmtId="0" fontId="0" fillId="0" borderId="0" xfId="0"/></cellXfs>'
    styles = insert(styles, xfs,
                    '<cellXfs count="2"><xf numFmtId="0" fontId="0" fillId="0" borderId="0" xfId="0"/>'
                    '<xf numFmtId="14" fontId="0" fillId="0" borderId="0" xfId="0" applyNumberFormat="1"/>'
                    "</cellXfs>", before=False)
    sheet = template("xl/worksheets/sheet1.xml")
    if sheet.count("<sheetData/>") != 1:
        raise Refused("the crate's template no longer has the anchor the generator expects")
    head, tail = sheet.split("<sheetData/>")
    return ct, rels, wb, wbrels, styles, head + "<sheetData>", "</sheetData>" + tail


def row_format(blocks):
    """A str.format pattern for one row: {r} is the row number, {0}... the values."""
    out, k = ['<row r="{r}">'], 0
    kinds = BLOCK * blocks
    for i, kind in enumerate(kinds):
        ref = column(i) + "{r}"
        if kind == "n":
            out.append('<c r="%s"><v>{%d}</v></c>' % (ref, k))
        elif kind == "s":
            out.append('<c r="%s" t="s"><v>{%d}</v></c>' % (ref, k))
        elif kind == "f":
            a, b = column(i - 6), column(i - 5)
            out.append('<c r="%s"><f>%s{r}+%s{r}</f><v>{%d}</v></c>' % (ref, a, b, k))
        elif kind == "d":
            out.append('<c r="%s" s="1"><v>{%d}</v></c>' % (ref, k))
        else:
            out.append('<c r="%s" t="b"><v>{%d}</v></c>' % (ref, k))
        k += 1
    out.append("</row>")
    return "".join(out), kinds


def rows(rng, nrows, blocks, unique):
    """Yield the sheet's rows in batches of encoded bytes."""
    fmt, kinds = row_format(blocks)
    fmt_row = fmt.format
    bits = rng.getrandbits
    strings = 0
    batch = []
    for r in range(1, nrows + 1):
        vals = []
        nums = []
        for kind in kinds:
            if kind == "n":
                n = bits(32)
                nums.append(n)
                vals.append(repr(n / 100))
            elif kind == "s":
                if unique:
                    vals.append(strings)
                else:
                    vals.append(bits(16) % 1000)
                strings += 1
            elif kind == "f":
                vals.append(repr((nums[-4] + nums[-3]) / 100))
            elif kind == "d":
                vals.append(36526 + bits(16) % 10000)
            else:
                vals.append(bits(1))
            if kind == "f":
                nums.clear()
        batch.append(fmt_row(*vals, r=r))
        if len(batch) == BATCH:
            yield "".join(batch).encode()
            batch = []
    if batch:
        yield "".join(batch).encode()


def sheet_chunks(rng, nrows, blocks, unique, head, tail):
    yield head.encode()
    yield from rows(rng, nrows, blocks, unique)
    yield tail.encode()


def shared_strings(total, unique):
    count = total if unique else 1000
    yield ('<?xml version="1.0" encoding="UTF-8" standalone="yes"?>\n'
           '<sst xmlns="%s" count="%d" uniqueCount="%d">' % (NS_MAIN, total, count)).encode()
    if unique:
        for start in range(0, total, BATCH):
            yield "".join("<si><t>%s</t></si>" % unique_string(i)
                          for i in range(start, min(start + BATCH, total))).encode()
    else:
        yield "".join("<si><t>%s</t></si>" % w for w in WORDS).encode()
    yield b"</sst>"


def small_sheet(rng, head, tail):
    out = [head]
    for r in range(1, 11):
        out.append('<row r="%d">' % r)
        for c in range(10):
            out.append('<c r="%s%d"><v>%d</v></c>' % (column(c), r, rng.getrandbits(16)))
        out.append("</row>")
    out.append(tail)
    return "".join(out)


def info(name):
    zi = zipfile.ZipInfo(name, date_time=DATE_TIME)
    zi.compress_type = zipfile.ZIP_DEFLATED
    zi.external_attr = 0o600 << 16
    return zi


def write_workbook(path, tier):
    nrows, blocks, unique = TIERS[tier]
    ct, rels, wb, wbrels, styles, head, tail = parts()
    rng = random.Random(SEED * 256 + ord(tier))
    sheet_bytes = 0
    with zipfile.ZipFile(path, "w", compression=zipfile.ZIP_DEFLATED, compresslevel=6) as zf:
        for name, text in (("[Content_Types].xml", ct), ("_rels/.rels", rels),
                           ("docProps/core.xml", CORE), ("xl/workbook.xml", wb),
                           ("xl/_rels/workbook.xml.rels", wbrels)):
            zf.writestr(info(name), text.encode())
        # The large sheet: zip64 records forced on tier c, whose sheet passes
        # 4 GiB (specs/0023, P1 ruling 1), since zipfile must choose the local
        # header's form before the size is known.
        with zf.open(info("xl/worksheets/sheet1.xml"), "w", force_zip64=(tier == "c")) as w:
            for chunk in sheet_chunks(rng, nrows, blocks, unique, head, tail):
                w.write(chunk)
                sheet_bytes += len(chunk)
        zf.writestr(info("xl/worksheets/sheet2.xml"), small_sheet(rng, head, tail).encode())
        zf.writestr(info("xl/styles.xml"), styles.encode())
        total = nrows * blocks * BLOCK.count("s")
        with zf.open(info("xl/sharedStrings.xml"), "w") as w:
            for chunk in shared_strings(total, unique):
                w.write(chunk)
    return sheet_bytes


def sha256_of(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for block in iter(lambda: f.read(1 << 20), b""):
            h.update(block)
    return h.hexdigest()


def under(path, root):
    return os.path.commonpath([path, root]) == root


def read_manifest(path):
    if not os.path.exists(path):
        return []
    with open(path, encoding="utf-8") as f:
        lines = f.read().split("\n")
    if lines[0] != "\t".join(HEADER):
        raise Refused("the manifest header is not the one specs/0005-corpus.md defines")
    return [line for line in lines[1:] if line]


def main(argv):
    args = argv[1:]
    force = "--force" in args
    args = [a for a in args if a != "--force"]
    if len(args) != 2:
        raise Refused(USAGE)
    tier, target = args
    if tier not in TIERS:
        raise Refused("unknown tier (one of %s)" % ", ".join(TIERS))
    repo = os.path.realpath(os.path.join(os.path.dirname(os.path.realpath(__file__)), ".."))
    target = os.path.realpath(target)
    if under(target, repo):
        raise Refused("the target is under the repository")
    name = "tier-%s.xlsx" % tier
    tags = "scale,tier%s" % tier
    corpus = os.path.join(target, "corpus")
    manifest = os.path.join(target, "manifest.tsv")
    lines = read_manifest(manifest)
    old = [r for r in lines if r.split("\t")[10:11] == [name]]
    if old and not force:
        raise Refused("the tier already has a workbook in the manifest; --force replaces it")
    tmp = os.path.join(target, ".%s.part" % name)
    if os.path.exists(tmp) and not force:
        raise Refused("a partial file of the tier is there; --force replaces it")
    # The refusals that need no workbook come before anything is written: the
    # template is read here so a missing anchor refuses with no directory
    # created. A file already in the corpus is known only from the hash, so
    # that refusal removes the partial file it made.
    parts()
    os.makedirs(corpus, mode=0o700, exist_ok=True)
    started = time.monotonic()
    sheet_bytes = write_workbook(tmp, tier)
    seconds = time.monotonic() - started
    digest = sha256_of(tmp)
    size = os.path.getsize(tmp)
    ident = digest[:12]
    dest = os.path.join(corpus, ident + ".xlsx")
    if os.path.exists(dest) and not force and not old:
        os.remove(tmp)
        raise Refused("the workbook's file is already in the corpus; --force replaces it")
    for r in old:
        stale = os.path.join(corpus, r.split("\t")[0] + ".xlsx")
        if os.path.exists(stale) and stale != dest:
            os.remove(stale)
    os.chmod(tmp, 0o600)
    os.replace(tmp, dest)
    added = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%d")
    row = [ident, digest, str(size), "xlsx", "scale-gen", VERSION, "none", SOURCE, tags,
           added, name, ""]
    keep = [r for r in lines if r not in old]
    new = manifest + ".new"
    with open(new, "w", encoding="utf-8") as f:
        f.write("\t".join(HEADER) + "\n")
        for r in keep + ["\t".join(row)]:
            f.write(r + "\n")
    os.chmod(new, 0o600)
    os.replace(new, manifest)
    print("generated %s %s %d %s sheet_bytes=%d seconds=%.1f" % (tier, ident, size, digest, sheet_bytes, seconds))
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main(sys.argv))
    except Refused as e:
        print("refused: %s" % e, file=sys.stderr)
        sys.exit(1)
    except Exception as e:  # noqa: BLE001 - name the class only, never the content
        print("error: %s" % type(e).__name__, file=sys.stderr)
        sys.exit(2)
