"""OpenRocket 24.12's pressure drag of a fineness-3 nose above Mach 1, per component (M2.3c10).

OpenRocket's issue #3391 (https://github.com/openrocket/openrocket/issues/3391, opened
2026-10-14) reports that OpenRocket 24.12 gives a tangent-ogive nose almost no pressure drag
above Mach 1, where the method in its own technical documentation gives the drag of a cone of the
same length and base diameter. A maintainer confirmed the report and merged a fix to the
development branch on 2026-11-02 (pull request #3431); by 2026-11-14 no release carried it. This
script measures what the 24.12 release gives: it runs OpenRocket 24.12 as an external oracle on
three small designs written here and records the rocket's and each part's drag columns at
Mach 1.3, 1.5 and 2.0, at zero angle of attack.

The designs are invented:

- `tangent ogive`: a tangent-ogive nose 150 mm long on a tube 50 mm across (fineness 3, the
  shape of #3391's report), a body tube 500 mm long, and three trapezoidal fins;
- `cone`: the same with a conical nose of the same length and base. It is the control: a cone is
  its own equivalent cone, so OpenRocket's answer for it shows what the accessor returns before
  the ogive's is read;
- `cone, wider body`: the conical nose on a tube 100 mm long, then a conical shoulder 50 mm long
  out to a tube 80 mm across, which carries the fins. The rocket's reference area is then 2.56
  times the nose's base area, so this control shows which of the two areas the coefficient is on.

The record gives each coefficient as OpenRocket returns it, with the rocket's reference area
beside it and the nose's base area, so the reader can put it on the nose's base area.

`hpr_validate::openrocket`'s tests read the same documents with `hpr_io::ork`, pin hpr's nose
pressure drag at each Mach number, and set OpenRocket's beside it and beside the equivalent
cone's.

Every part has a fixed id. Each (design, Mach) pair is asked of a newly loaded design and a new
calculator. OpenRocket is run, never read: its source is GPL, and nothing here comes from it. The
class and method names used are the public API that `javap` prints for the jar. OpenRocket 24.12
needs Java 17 exactly (see `automatic_radius.py`). Run from the repository root:

    refs/venv/bin/python validation/oracles/openrocket/nose_pressure_drag.py \\
        validation/fixtures/ork/openrocket-nose-pressure-drag.json
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

# Provenance written into the fixture (docs/VALIDATION.md). Update when regenerating.
GENERATED = "2026-11-14"

#: The Mach numbers each design is asked at.
MACHS = [1.3, 1.5, 2.0]

#: The nose's length and its base radius, m: fineness 150 / 50 = 3.
NOSE_LENGTH_M = 0.15
BASE_RADIUS_M = 0.025

MATERIAL = '<material type="bulk" density="1000.0">Probe</material>'

#: The parts' id numbers.
NOSE, TUBE, FINS, STAGE, ROCKET = 1, 2, 3, 99, 98

#: The wider body's radius, m, and the id numbers of its shoulder and its aft tube.
WIDE_RADIUS_M = 0.04
SHOULDER, AFT = 4, 5

#: Each design's nose, the `.ork` shape and its shape parameter where the shape takes one, and
#: whether the body widens behind it.
DESIGNS = {
    "tangent ogive": ("ogive", "<shapeparameter>1.0</shapeparameter>", False),
    "cone": ("conical", "", False),
    "cone, wider body": ("conical", "", True),
}


def uid(n):
    """A fixed id, so that hpr and OpenRocket name each part the same way."""
    return f"00000000-0000-4000-8000-{n:012d}"


def head(name, n):
    return f"<name>{name}</name><id>{uid(n)}</id>"


def fins():
    return (
        f"<trapezoidfinset>{head('Fins', FINS)}"
        '<position type="bottom">0.0</position><fincount>3</fincount>'
        "<rootchord>0.08</rootchord><tipchord>0.04</tipchord><sweeplength>0.04</sweeplength>"
        "<height>0.05</height><thickness>0.003</thickness><crosssection>square</crosssection>"
        f"<finish>normal</finish>{MATERIAL}</trapezoidfinset>"
    )


def body(wider):
    """The parts behind the nose: one tube with the fins, or a short tube, a conical shoulder
    and a wider tube with the fins."""
    if not wider:
        return (
            f"<bodytube>{head('Body', TUBE)}<finish>normal</finish>{MATERIAL}"
            f"<length>0.5</length><thickness>0.0005</thickness><radius>{BASE_RADIUS_M}</radius>"
            f"<subcomponents>{fins()}</subcomponents></bodytube>"
        )
    return (
        f"<bodytube>{head('Body', TUBE)}<finish>normal</finish>{MATERIAL}"
        f"<length>0.1</length><thickness>0.0005</thickness><radius>{BASE_RADIUS_M}</radius>"
        "</bodytube>"
        f"<transition>{head('Shoulder', SHOULDER)}<finish>normal</finish>{MATERIAL}"
        "<length>0.05</length><thickness>0.001</thickness><shape>conical</shape>"
        f"<foreradius>{BASE_RADIUS_M}</foreradius><aftradius>{WIDE_RADIUS_M}</aftradius>"
        "<foreshoulderradius>0.0</foreshoulderradius><foreshoulderlength>0.0</foreshoulderlength>"
        "<aftshoulderradius>0.0</aftshoulderradius><aftshoulderlength>0.0</aftshoulderlength>"
        "</transition>"
        f"<bodytube>{head('Aft body', AFT)}<finish>normal</finish>{MATERIAL}"
        f"<length>0.4</length><thickness>0.0005</thickness><radius>{WIDE_RADIUS_M}</radius>"
        f"<subcomponents>{fins()}</subcomponents></bodytube>"
    )


def document(design):
    """The probe called `design` as a whole `.ork` document."""
    shape, parameter, wider = DESIGNS[design]
    return (
        "<?xml version='1.0' encoding='utf-8'?>\n"
        '<openrocket version="1.10" creator="hpr-sim nose pressure drag probe">'
        f"<rocket><name>Probe</name><id>{uid(ROCKET)}</id>"
        "<referencetype>maximum</referencetype><subcomponents>"
        f"<stage>{head('Stage', STAGE)}<subcomponents>"
        f"<nosecone>{head('Nose', NOSE)}<finish>normal</finish>{MATERIAL}"
        f"<length>{NOSE_LENGTH_M}</length><thickness>0.001</thickness><shape>{shape}</shape>"
        f"{parameter}<aftradius>{BASE_RADIUS_M}</aftradius>"
        "<aftshoulderradius>0.0</aftshoulderradius><aftshoulderlength>0.0</aftshoulderlength>"
        "<aftshoulderthickness>0.0</aftshoulderthickness>"
        "<aftshouldercapped>false</aftshouldercapped></nosecone>"
        f"{body(wider)}"
        "</subcomponents></stage></subcomponents></rocket></openrocket>\n"
    )


def columns(force):
    return {
        "cd": float(force.getCD()),
        "friction": float(force.getFrictionCD()),
        "pressure": float(force.getPressureCD()),
        "base": float(force.getBaseCD()),
    }


def forces(path, mach):
    """The rocket's drag columns and each part's, by id, from a new load of `path` and a new
    calculator at `mach`."""
    from info.openrocket.core.aerodynamics import BarrowmanCalculator, FlightConditions
    from info.openrocket.core.logging import WarningSet

    rocket = automatic_radius.load(path).getRocket()
    configuration = rocket.getSelectedConfiguration()
    conditions = FlightConditions(configuration)
    conditions.setMach(mach)
    conditions.setAOA(0.0)
    warnings = WarningSet()
    total = BarrowmanCalculator().getAerodynamicForces(configuration, conditions, warnings)
    analysis = BarrowmanCalculator().getForceAnalysis(configuration, conditions, warnings)
    return {
        "mach": mach,
        "reference_area_m2": float(conditions.getRefArea()),
        "rocket": columns(total),
        "components": {
            str(component.getID()): {"name": str(component.getName()), **columns(force)}
            for component, force in analysis.items()
        },
        "warnings": sorted(str(warning) for warning in warnings),
    }


def main():
    if len(sys.argv) != 2:
        sys.exit("usage: nose_pressure_drag.py OUTPUT.json")
    output = Path(sys.argv[1])
    logging.disable(logging.CRITICAL)
    automatic_radius.start()
    import jpype
    from info.openrocket.core.util import BuildProperties
    from java.lang import System

    designs = {}
    with tempfile.TemporaryDirectory() as scratch:
        for name in DESIGNS:
            text = document(name)
            path = Path(scratch) / "probe.ork"
            path.write_text(text, encoding="utf-8")
            designs[name] = {
                "document": text,
                "machs": [forces(path, mach) for mach in MACHS],
            }

    fixture = {
        "source": "validation/oracles/openrocket/nose_pressure_drag.py",
        "command": "refs/venv/bin/python validation/oracles/openrocket/nose_pressure_drag.py "
        "validation/fixtures/ork/openrocket-nose-pressure-drag.json",
        "generated": GENERATED,
        "cites": "https://github.com/openrocket/openrocket/issues/3391",
        "inputs_sha256": {
            name: hashlib.sha256(Path(module.__file__).read_bytes()).hexdigest()
            for name, module in [
                ("nose_pressure_drag.py", sys.modules[__name__]),
                ("automatic_radius.py", automatic_radius),
            ]
        },
        "openrocket": str(BuildProperties.getVersion()),
        "jar_sha256": hashlib.sha256(automatic_radius.JAR.read_bytes()).hexdigest(),
        "java": str(System.getProperty("java.version")),
        "jpype": jpype.__version__,
        "nose_id": uid(NOSE),
        "nose_length_m": NOSE_LENGTH_M,
        "nose_base_area_m2": math.pi * BASE_RADIUS_M**2,
        "designs": designs,
    }
    text = json.dumps(fixture, indent=2, ensure_ascii=False, sort_keys=True) + "\n"
    output.write_text(text, encoding="utf-8")


if __name__ == "__main__":
    main()
