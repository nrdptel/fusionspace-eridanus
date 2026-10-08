#!/usr/bin/env python3
"""Write xtask/src/figures/glyphs.rs: how wide and how tall each character can draw.

Usage:
  uv run --no-project --with fonttools==4.65.0 --with matplotlib==3.11.2 \\
      python3 scripts/glyph-widths.py

What it is for. The figure check (`xtask/src/figures.rs`) must know how much room a line of
text can take. An SVG shown through `<img>` can't load a web font, so it draws in whatever font
the reader's system picks from its `font-family` list. The check therefore takes, for every
character, the largest advance and the largest ink extent over a set of common system fonts: a
line no wider than that fits whichever of them draws it.

Where the numbers come from. Each font's own tables, read with fontTools: the advance from
`hmtx` (after `cmap` maps the character to its glyph), the ink box from the glyph's outline
(fontTools' BoundsPen, so TrueType and CFF outlines alike). A variable font is instanced at the
weight (400 or 700) and at the minimum, default and maximum of its optical size, which a browser
sets from the font size, keeping the largest; its other axes (width, grade) stay at their
defaults, as a figure that sets no `font-stretch` draws them. Values are in thousandths of an
em, rounded up. It reads macOS's font folders, so it runs on a Mac; CI only reads its output.

The fonts. macOS's own (/System/Library/Fonts and its Supplemental folder), the Microsoft fonts
that Office for Mac installs (Consolas, Lucida Console), Cascadia Mono where it is installed
(~/Library/Fonts), and DejaVu Sans, Sans Mono and Serif as matplotlib ships them, which stand
for a Linux desktop. Liberation Sans, Serif and Mono (and Arimo, Tinos and Cousine) are made
metric-compatible with Arial, Times New Roman and Courier New, so those cover them. The generic
families a browser maps to its defaults are among them: `serif` (and a text with no family) to
Times on macOS, Times New Roman on Windows, DejaVu Serif or Liberation Serif on Linux;
`sans-serif` to Helvetica, Arial, DejaVu Sans or Liberation Sans; `monospace` to Courier or
Menlo, Courier New or Consolas, DejaVu Sans Mono or Liberation Mono. `system-ui` is Segoe UI on
Windows, which isn't here, so the figure check refuses that family. A font that
isn't found fails the run, so a table is never quietly written from fewer fonts; the generated
file's header lists each font's file and version.

Two classes, each regular and bold. A character's numbers in a class are the largest over the
class's fonts. Where any of those fonts lacks the character, the system draws it in a fallback
font, so the numbers then take the largest over every font here, including the fallback fonts
(Lucida Grande, Arial Unicode and Apple Symbols). A character none of them has is left out, and
the check refuses it. A family with no bold face is drawn in a synthesized bold: its regular
glyphs widened by FreeType's emboldening strength, an em over 24 (FT_GlyphSlot_Embolden), added
here to the advance and to each side of the ink.
"""

from __future__ import annotations

import itertools
import math
import subprocess
import sys
from pathlib import Path

from fontTools.pens.boundsPen import BoundsPen
from fontTools.ttLib import TTFont
from fontTools.ttLib.ttCollection import TTCollection
from fontTools.varLib import instancer

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "xtask" / "src" / "figures" / "glyphs.rs"

SYSTEM = Path("/System/Library/Fonts")
SUPPLEMENTAL = SYSTEM / "Supplemental"
OFFICE = Path("/Applications/Microsoft Word.app/Contents/Resources/DFonts")
USER = Path.home() / "Library" / "Fonts"


def dejavu() -> Path:
    import matplotlib

    return Path(matplotlib.get_data_path()) / "fonts" / "ttf"


# Each family: its name as a `font-family` list names it, its regular face and its bold face
# (None: drawn in a synthesized bold). A face is a file, with the subfamily name to pick from a
# collection (.ttc).
PROPORTIONAL = [
    ("Verdana", (SUPPLEMENTAL / "Verdana.ttf", None), (SUPPLEMENTAL / "Verdana Bold.ttf", None)),
    ("Tahoma", (SUPPLEMENTAL / "Tahoma.ttf", None), (SUPPLEMENTAL / "Tahoma Bold.ttf", None)),
    ("Arial", (SUPPLEMENTAL / "Arial.ttf", None), (SUPPLEMENTAL / "Arial Bold.ttf", None)),
    ("Helvetica", (SYSTEM / "Helvetica.ttc", "Regular"), (SYSTEM / "Helvetica.ttc", "Bold")),
    (
        "Helvetica Neue",
        (SYSTEM / "HelveticaNeue.ttc", "Regular"),
        (SYSTEM / "HelveticaNeue.ttc", "Bold"),
    ),
    # macOS's default serif, which a `serif` family or none at all draws in.
    ("Times", (SYSTEM / "Times.ttc", "Regular"), (SYSTEM / "Times.ttc", "Bold")),
    (
        "Times New Roman",
        (SUPPLEMENTAL / "Times New Roman.ttf", None),
        (SUPPLEMENTAL / "Times New Roman Bold.ttf", None),
    ),
    ("Georgia", (SUPPLEMENTAL / "Georgia.ttf", None), (SUPPLEMENTAL / "Georgia Bold.ttf", None)),
    (
        "Trebuchet MS",
        (SUPPLEMENTAL / "Trebuchet MS.ttf", None),
        (SUPPLEMENTAL / "Trebuchet MS Bold.ttf", None),
    ),
    ("SF Pro", (SYSTEM / "SFNS.ttf", None), (SYSTEM / "SFNS.ttf", None)),
    ("DejaVu Sans", (dejavu() / "DejaVuSans.ttf", None), (dejavu() / "DejaVuSans-Bold.ttf", None)),
    (
        "DejaVu Serif",
        (dejavu() / "DejaVuSerif.ttf", None),
        (dejavu() / "DejaVuSerif-Bold.ttf", None),
    ),
]
MONOSPACE = [
    ("Menlo", (SYSTEM / "Menlo.ttc", "Regular"), (SYSTEM / "Menlo.ttc", "Bold")),
    ("Monaco", (SYSTEM / "Monaco.ttf", None), None),
    ("Courier", (SYSTEM / "Courier.ttc", "Regular"), (SYSTEM / "Courier.ttc", "Bold")),
    (
        "Courier New",
        (SUPPLEMENTAL / "Courier New.ttf", None),
        (SUPPLEMENTAL / "Courier New Bold.ttf", None),
    ),
    ("Andale Mono", (SUPPLEMENTAL / "Andale Mono.ttf", None), None),
    ("PT Mono", (SUPPLEMENTAL / "PTMono.ttc", "Regular"), (SUPPLEMENTAL / "PTMono.ttc", "Bold")),
    ("SF Mono", (SYSTEM / "SFNSMono.ttf", None), (SYSTEM / "SFNSMono.ttf", None)),
    (
        "Cascadia Mono",
        (USER / "CascadiaMono-Regular.ttf", None),
        (USER / "CascadiaMono-Bold.ttf", None),
    ),
    ("Consolas", (OFFICE / "Consola.ttf", None), (OFFICE / "Consolab.ttf", None)),
    ("Lucida Console", (OFFICE / "Lucida Console.ttf", None), None),
    (
        "DejaVu Sans Mono",
        (dejavu() / "DejaVuSansMono.ttf", None),
        (dejavu() / "DejaVuSansMono-Bold.ttf", None),
    ),
]
# Fonts a system falls back to for a character the named fonts lack.
FALLBACK = [
    (
        "Lucida Grande",
        (SYSTEM / "LucidaGrande.ttc", "Regular"),
        (SYSTEM / "LucidaGrande.ttc", "Bold"),
    ),
    ("Arial Unicode MS", (SUPPLEMENTAL / "Arial Unicode.ttf", None), None),
    ("Apple Symbols", (SYSTEM / "Apple Symbols.ttf", None), None),
]

# FreeType's synthesized bold: an em over 24, in thousandths.
SYNTHETIC_BOLD = math.ceil(1000 / 24)


def characters() -> list[str]:
    """The characters the table covers: printable ASCII, Latin-1, Greek, and the punctuation,
    signs and arrows figures use."""
    codes = list(range(0x20, 0x7F)) + list(range(0xA0, 0x100))
    # Greek capitals and small letters, less U+03A2, which Unicode leaves unassigned.
    codes += [c for c in range(0x391, 0x3AA) if c != 0x3A2] + list(range(0x3B1, 0x3CA))
    codes += [0x2013, 0x2014, 0x2018, 0x2019, 0x201C, 0x201D, 0x2022, 0x2026, 0x2032, 0x2033]
    codes += [0x2039, 0x203A, 0x2070, 0x2074, 0x2075, 0x2076, 0x2077, 0x2078, 0x2079]
    codes += [0x2190, 0x2191, 0x2192, 0x2193, 0x2194]
    codes += [0x2212, 0x2248, 0x2260, 0x2264, 0x2265, 0x221E, 0x221A, 0x2206, 0x22C5]
    # U+00AD, the soft hyphen, draws nothing or a hyphen at a line break; a figure's text
    # doesn't break, so it is left out and refused.
    return [chr(c) for c in sorted(set(codes)) if c != 0xAD]


CHARS = characters()


def name(font: TTFont, name_id: int) -> str:
    table_ = font["name"]
    record = table_.getName(name_id, 3, 1, 0x409) or table_.getName(name_id, 1, 0, 0)
    return str(record) if record else ""


def open_face(path: Path, subfamily: str | None) -> TTFont:
    if not path.is_file():
        sys.exit(f"{path}: not found; every font must be present (see the docstring)")
    if path.suffix.lower() == ".ttc":
        for font in TTCollection(str(path)).fonts:
            if name(font, 2) == subfamily or name(font, 17) == subfamily:
                return font
        sys.exit(f"{path}: no face {subfamily!r}")
    return TTFont(str(path))


def instances(font: TTFont, bold: bool) -> list[TTFont]:
    """The face itself, or a variable font's instances at the weight and at the optical size's
    minimum, default and maximum, its other axes at their defaults."""
    if "fvar" not in font:
        return [font]
    axes = font["fvar"].axes
    choices = []
    for axis in axes:
        if axis.axisTag == "wght":
            weight = 700 if bold else 400
            choices.append([max(axis.minValue, min(axis.maxValue, weight))])
        elif axis.axisTag == "opsz":
            # A browser picks the optical size from the font size by itself.
            choices.append(sorted({axis.minValue, axis.defaultValue, axis.maxValue}))
        else:
            # Width, grade and the rest stay at their defaults unless a style asks.
            choices.append([axis.defaultValue])
    out = []
    for values in itertools.product(*choices):
        location = {axis.axisTag: value for axis, value in zip(axes, values)}
        out.append(instancer.instantiateVariableFont(font, location, inplace=False))
    return out


def measure(font: TTFont, chars: list[str]) -> dict[str, list[float]]:
    """Each character the font has: advance, left and right ink overhang past the advance, ink
    top above and ink bottom below the baseline, in thousandths of an em (unrounded)."""
    cmap = font.getBestCmap()
    scale = 1000 / font["head"].unitsPerEm
    glyphs = font.getGlyphSet()
    out = {}
    for c in chars:
        glyph = cmap.get(ord(c))
        if glyph is None:
            continue
        advance = font["hmtx"][glyph][0]
        pen = BoundsPen(glyphs)
        glyphs[glyph].draw(pen)
        left = right = top = bottom = 0
        if pen.bounds is not None:
            x_min, y_min, x_max, y_max = pen.bounds
            left, right = max(0, -x_min), max(0, x_max - advance)
            top, bottom = max(0, y_max), max(0, -y_min)
        out[c] = [v * scale for v in (advance, left, right, top, bottom)]
    return out


def face_metrics(family, bold: bool, chars: list[str], sources: list[str]):
    """A family's numbers in the weight, its synthesized bold if it has no bold face."""
    _, regular, bold_face = family
    face, synthetic = (bold_face, False) if bold and bold_face else (regular, bold)
    path, subfamily = face
    font = open_face(path, subfamily)
    merged: dict[str, list[float]] = {}
    for instance in instances(font, bold):
        for c, values in measure(instance, chars).items():
            merged[c] = [max(a, b) for a, b in zip(merged.get(c, values), values)]
    if synthetic:
        merged = {c: [v + SYNTHETIC_BOLD for v in values] for c, values in merged.items()}
    label = f"{family[0]}{' Bold' if bold else ''}"
    how = " (synthesized bold)" if synthetic else ""
    where = path if subfamily is None else f"{path}, face {subfamily}"
    sources.append(f"{label}: {where}{how}; {name(font, 5)}")
    return merged


def table(families, others, bold: bool, measured):
    """A class's rows: each character's largest numbers over the class's fonts, or over every
    font where one of the class's lacks it."""
    own = [measured[(f[0], bold)] for f in families]
    every = own + [measured[(f[0], bold)] for f in others]
    rows = []
    for c in CHARS:
        pool = own if all(c in m for m in own) else every
        found = [m[c] for m in pool if c in m]
        if not found:
            continue
        rows.append((c, [math.ceil(max(v) - 1e-9) for v in zip(*found)]))
    return rows


def literal(c: str) -> str:
    if c in "'\\":
        return f"'\\{c}'"
    if " " <= c <= "~":
        return f"'{c}'"
    return f"'\\u{{{ord(c):04x}}}'"


def rust(rows, name_: str, doc: str) -> str:
    out = [f"/// {doc}", f"pub(super) const {name_}: &[(char, [u16; 5])] = &["]
    for c, values in rows:
        shown = "" if " " <= c <= "~" else f" // {c}"
        if c == " ":
            shown = " // no-break space"
        if c == "\u2014":
            shown = " // em dash"
        out.append(f"    ({literal(c)}, [{', '.join(str(v) for v in values)}]),{shown}")
    out.append("];")
    return "\n".join(out)


def families(name_: str, rows, doc: str) -> str:
    names = ", ".join(f'"{f[0]}"' for f in rows)
    return f"/// {doc}\npub(super) const {name_}: &[&str] = &[{names}];"


def main() -> None:
    sources: list[str] = []
    measured = {
        (family[0], bold): face_metrics(family, bold, CHARS, sources)
        for family in PROPORTIONAL + MONOSPACE + FALLBACK
        for bold in (False, True)
    }
    proportional = table(PROPORTIONAL, MONOSPACE + FALLBACK, False, measured)
    proportional_bold = table(PROPORTIONAL, MONOSPACE + FALLBACK, True, measured)
    monospace = table(MONOSPACE, PROPORTIONAL + FALLBACK, False, measured)
    monospace_bold = table(MONOSPACE, PROPORTIONAL + FALLBACK, True, measured)
    import matplotlib

    shipped = f"matplotlib {matplotlib.__version__}'s "
    home = str(Path.home())
    header = [
        "//! How wide and how tall each character can draw, the largest over a set of system",
        "//! fonts, in thousandths of an em, rounded up. Written by `scripts/glyph-widths.py`",
        "//! (read it for the rules); do not edit by hand.",
        "//!",
        "//! Each row: the character, then its advance, its ink's overhang left of its start and",
        "//! right of its advance, and its ink's height above and depth below the baseline.",
        "//!",
        "//! Measured from these faces (file; version string):",
        "//!",
    ]
    for source in sorted(set(sources)):
        source = source.replace(f"{dejavu()}/", shipped).replace(home, "~")
        header.append(f"//! - {source}")
    body = [
        "\n".join(header),
        "",
        families(
            "PROPORTIONAL_FAMILIES",
            PROPORTIONAL,
            "The proportional families measured.",
        ),
        "",
        families("MONOSPACE_FAMILIES", MONOSPACE, "The monospace families measured."),
        "",
        rust(proportional, "PROPORTIONAL", "Proportional fonts, regular weight."),
        "",
        rust(proportional_bold, "PROPORTIONAL_BOLD", "Proportional fonts, bold."),
        "",
        rust(monospace, "MONOSPACE", "Monospace fonts, regular weight."),
        "",
        rust(monospace_bold, "MONOSPACE_BOLD", "Monospace fonts, bold."),
        "",
    ]
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text("\n".join(body))
    # As `cargo fmt` lays it out, so a rerun on the same fonts changes nothing.
    subprocess.run(["rustfmt", "--edition", "2024", str(OUT)], check=True)
    missing = [c for c in CHARS if c not in dict(proportional)]
    print(f"wrote {OUT.relative_to(ROOT)}: {len(proportional)} characters", file=sys.stderr)
    if missing:
        print(f"no font has: {' '.join(missing)}", file=sys.stderr)


if __name__ == "__main__":
    main()
