"""OpenRocket 24.12's pressure drag of the fineness-3 nose from Mach 1 to 1.3 (#1500).

`nose_pressure_drag.py` (M2.3c10) records OpenRocket 24.12's drag columns for an invented
fineness-3 tangent ogive and its conical control at Mach 1.3, 1.5 and 2. The flights the nose's
line is measured on (M2.3c13) reach Mach 1.06 on average and none passes Mach 1.3, so this script
asks the same two designs the same question every 0.01 from Mach 1.00 to 1.30: the band where
HPR Sim joins its value at Mach 1 to its value at Mach 1.3, and where OpenRocket's issue #3391
(https://github.com/openrocket/openrocket/issues/3391) reports a false hump near Mach 1.1.

The designs, the documents and the call are `nose_pressure_drag.py`'s, imported unchanged: only
the Mach numbers and the two designs asked differ (the wider-body control, which shows the
column's area, is not asked again). Each (design, Mach) pair is asked of a newly loaded design
and a new calculator, at zero angle of attack.

`hpr_validate::openrocket`'s tests read the documents with `hpr_io::ork`, pin hpr's nose pressure
drag at each Mach number shown on the aerodynamics page, and set OpenRocket's beside it.

OpenRocket is run, never read: its source is GPL, and nothing here comes from it. OpenRocket 24.12
needs Java 17 exactly (see `automatic_radius.py`). Run from the repository root:

    refs/venv/bin/python validation/oracles/openrocket/nose_pressure_drag_transonic.py \\
        validation/fixtures/ork/openrocket-nose-pressure-drag-transonic.json
"""

import hashlib
import json
import logging
import math
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import automatic_radius  # noqa: E402 - the JVM start and the loader
import nose_pressure_drag as probe  # noqa: E402 - the designs, documents and call

# Provenance written into the fixture (docs/VALIDATION.md). Update when regenerating.
GENERATED = "2026-11-16"

#: The Mach numbers each design is asked at: every 0.01 from 1.00 to 1.30.
MACHS = [round(1.0 + 0.01 * step, 2) for step in range(31)]

#: The designs asked: the tangent ogive and its conical control.
DESIGNS = ["tangent ogive", "cone"]


def main():
    if len(sys.argv) != 2:
        sys.exit("usage: nose_pressure_drag_transonic.py OUTPUT.json")
    output = Path(sys.argv[1])
    logging.disable(logging.CRITICAL)
    automatic_radius.start()
    import jpype
    from info.openrocket.core.util import BuildProperties
    from java.lang import System

    designs = {}
    with tempfile.TemporaryDirectory() as scratch:
        for name in DESIGNS:
            text = probe.document(name)
            path = Path(scratch) / "probe.ork"
            path.write_text(text, encoding="utf-8")
            designs[name] = {
                "document": text,
                "machs": [probe.forces(path, mach) for mach in MACHS],
            }

    fixture = {
        "source": "validation/oracles/openrocket/nose_pressure_drag_transonic.py",
        "command": "refs/venv/bin/python "
        "validation/oracles/openrocket/nose_pressure_drag_transonic.py "
        "validation/fixtures/ork/openrocket-nose-pressure-drag-transonic.json",
        "generated": GENERATED,
        "cites": "https://github.com/openrocket/openrocket/issues/3391",
        "inputs_sha256": {
            name: hashlib.sha256(Path(module.__file__).read_bytes()).hexdigest()
            for name, module in [
                ("nose_pressure_drag_transonic.py", sys.modules[__name__]),
                ("nose_pressure_drag.py", probe),
                ("automatic_radius.py", automatic_radius),
            ]
        },
        "openrocket": str(BuildProperties.getVersion()),
        "jar_sha256": hashlib.sha256(automatic_radius.JAR.read_bytes()).hexdigest(),
        "java": str(System.getProperty("java.version")),
        "jpype": jpype.__version__,
        "nose_id": probe.uid(probe.NOSE),
        "nose_length_m": probe.NOSE_LENGTH_M,
        "nose_base_area_m2": math.pi * probe.BASE_RADIUS_M**2,
        "designs": designs,
    }
    text = json.dumps(fixture, indent=2, ensure_ascii=False, sort_keys=True) + "\n"
    output.write_text(text, encoding="utf-8")


if __name__ == "__main__":
    main()
