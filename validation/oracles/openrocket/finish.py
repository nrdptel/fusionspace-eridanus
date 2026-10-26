"""What OpenRocket 24.12 reads for a part whose `.ork` states no finish, or one it has no name for.

HPR Sim's `.ork` reader reads an outside part with no `<finish>`, or with a word it has no
roughness for, as regular paint, 60 µm (`hpr_io::ork::attached::finish`, issue #216). This
script is the measurement behind that rule. It takes each example design in the jar, removes every
`<finish>` (or puts the unknown word `gilt` in place of each), opens the copy, and prints, for
every kind of part, what `getFinish()` and `getRoughnessSize()` report. A part with no
`getFinish` has no finish in OpenRocket at all. OpenRocket is run, never read: its source is GPL,
and the method names are the public API `javap` prints for the jar.

Run 2026-10-25, it printed `Regular paint (60 µm)` and `6e-05` for all 134 outside parts of the
17 examples both ways: 48 body tubes, 22 nose cones, 2 transitions, 36 trapezoidal, 7 freeform and
1 elliptical fin sets, 1 tube fin set, 10 launch lugs and 7 rail buttons. Inner tubes, couplers,
engine blocks, rings, bulkheads, mass components, parachutes and shock cords have no finish.

Run from the repository root with the oracle environment and Java 17 (see `automatic_radius.py`):

    refs/venv/bin/python validation/oracles/openrocket/finish.py

It prints to standard output and writes nothing.
"""

import collections
import gzip
import io
import logging
import re
import sys
import tempfile
import zipfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import automatic_radius  # noqa: E402 - found beside this script


def design_text(raw):
    """The design XML of an `.ork`, which is a zip, a gzip stream or plain XML."""
    if raw[:2] == b"PK":
        archive = zipfile.ZipFile(io.BytesIO(raw))
        entry = next(name for name in archive.namelist() if name.endswith(".ork"))
        raw = archive.read(entry)
    elif raw[:2] == b"\x1f\x8b":
        raw = gzip.decompress(raw)
    return raw.decode("utf-8")


def main():
    logging.disable(logging.CRITICAL)
    automatic_radius.start()
    seen = collections.defaultdict(collections.Counter)
    without = collections.Counter()
    with tempfile.TemporaryDirectory() as scratch, zipfile.ZipFile(automatic_radius.JAR) as jar:
        entries = [
            entry
            for entry in sorted(jar.namelist())
            if entry.startswith("datafiles/examples/") and entry.lower().endswith(".ork")
        ]
        for number, entry in enumerate(entries):
            xml = design_text(jar.read(entry))
            for variant, replacement in (("absent", ""), ("unknown word", "<finish>gilt</finish>")):
                text, count = re.subn(r"<finish>[^<]*</finish>", replacement, xml)
                if count == 0:
                    continue
                path = Path(scratch) / f"{number}-{len(variant)}.ork"
                path.write_text(text, encoding="utf-8")
                rocket = automatic_radius.load(path).getRocket()
                parts = rocket.iterator(True)
                while parts.hasNext():
                    part = parts.next()
                    kind = str(part.getClass().getSimpleName())
                    if hasattr(part, "getFinish"):
                        finish = part.getFinish()
                        size = float(finish.getRoughnessSize())
                        seen[variant][(kind, str(finish), size)] += 1
                    elif variant == "absent":
                        without[kind] += 1
    print(f"{len(entries)} example designs")
    for variant, counts in seen.items():
        print(f"finish {variant}:")
        for (kind, name, size), count in sorted(counts.items()):
            print(f"  {kind:18} {name:24} {size:g} x{count}")
    print("no finish in OpenRocket:", ", ".join(f"{k} {v}" for k, v in sorted(without.items())))


if __name__ == "__main__":
    main()
