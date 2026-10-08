"""OpenRocket 24.12's drag with a part's drag-coefficient override (M4.5h, #165).

A `.ork` part may state its drag coefficient (`<overridecd>`) and whether that covers the parts
inside it (`<overridesubcomponentscd>`). OpenRocket's documentation says what the checkbox does,
not which of the part's drag terms the number replaces, on which area, or where a step's base
drag goes. This script measures it: it runs OpenRocket 24.12 as an external oracle on small
designs written here, each a variant of one airframe with one or two parts' drag stated, and
records the rocket's drag coefficient and its friction, pressure and base columns, and each part's
own, at several Mach numbers, at zero angle of attack.

The airframes:

- `flare`: an ogive nose and a body tube 25 mm in radius, and behind them a conical flare from a
  point to the body's radius, the shape of OpenRocket's *Base drag hack* example. On the tube: a
  trapezoidal fin set of three, a launch lug of two instances, two rail buttons, and inside, an
  inner tube, a centering ring and a mass component.
- `boattail`: the nose and tube, then a boattail to 18 mm.
- `step`: the nose and tube, then a second tube of 20 mm radius: a step down between them.
- `step up`: the nose and tube at 20 mm, then a second tube of 25 mm radius: a step up.

`hpr_validate::openrocket`'s tests read the same documents with `hpr_io::ork` and hold hpr's
change in drag, variant against its airframe with nothing stated, to OpenRocket's.

Every part has a fixed id. Each (variant, Mach) pair is asked of a newly loaded design and a new
calculator. OpenRocket is run, never read: its source is GPL, and nothing here comes from it. The
class and method names used are the public API that `javap` prints for the jar. OpenRocket 24.12
needs Java 17 exactly (see `automatic_radius.py`). Run from the repository root:

    refs/venv/bin/python validation/oracles/openrocket/drag_override.py \\
        validation/fixtures/ork/openrocket-drag-override.json
"""

import hashlib
import json
import logging
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import automatic_radius  # noqa: E402 - the JVM start and the loader

# Provenance written into the fixture (docs/VALIDATION.md). Update when regenerating.
GENERATED = "2026-10-05"

#: The Mach numbers each variant is asked at.
MACHS = [0.1, 0.3, 0.6, 0.9, 1.2, 2.0]

MATERIAL = '<material type="bulk" density="1000.0">Probe</material>'


def uid(n):
    """A fixed id, so that hpr and OpenRocket name each part the same way."""
    return f"00000000-0000-4000-8000-{n:012d}"


#: The parts a variant may state a drag coefficient on, by id number.
NOSE, TUBE, FINS, LUG, BUTTONS, INNER, RING, MASS, AFT, STAGE = 1, 2, 3, 4, 5, 6, 7, 8, 9, 99


def stated(states, n):
    """The override tags for part `n`: `states` maps an id number to `(cd, covers_children)`."""
    if n not in states:
        return ""
    cd, children = states[n]
    return (
        f"<overridecd>{cd}</overridecd>"
        f"<overridesubcomponentscd>{str(children).lower()}</overridesubcomponentscd>"
    )


def head(name, n, states):
    return f"<name>{name}</name><id>{uid(n)}</id>{stated(states, n)}"


def tube_contents(states):
    return (
        f"<trapezoidfinset>{head('Fins', FINS, states)}"
        '<position type="bottom">0.0</position><fincount>3</fincount>'
        "<rootchord>0.08</rootchord><tipchord>0.04</tipchord><sweeplength>0.04</sweeplength>"
        "<height>0.05</height><thickness>0.003</thickness><crosssection>square</crosssection>"
        f"<finish>normal</finish>{MATERIAL}</trapezoidfinset>"
        f"<launchlug>{head('Lug', LUG, states)}"
        '<position type="top">0.1</position><length>0.05</length><radius>0.005</radius>'
        "<thickness>0.001</thickness><instancecount>2</instancecount>"
        f"<instanceseparation>0.1</instanceseparation><finish>normal</finish>{MATERIAL}"
        "</launchlug>"
        f"<railbutton>{head('Buttons', BUTTONS, states)}"
        '<position type="top">0.05</position><outerdiameter>0.01</outerdiameter>'
        "<innerdiameter>0.006</innerdiameter><height>0.008</height><baseheight>0.002</baseheight>"
        "<flangeheight>0.002</flangeheight><instancecount>2</instancecount>"
        f"<instanceseparation>0.2</instanceseparation>{MATERIAL}</railbutton>"
        f"<innertube>{head('Inner', INNER, states)}"
        '<position type="bottom">0.0</position><length>0.1</length>'
        f"<outerradius>0.0125</outerradius><thickness>0.0005</thickness>{MATERIAL}</innertube>"
        f"<centeringring>{head('Ring', RING, states)}"
        '<position type="bottom">0.0</position><length>0.005</length>'
        f"<outerradius>auto</outerradius><innerradius>0.0125</innerradius>{MATERIAL}"
        "</centeringring>"
        f"<masscomponent>{head('Mass', MASS, states)}"
        '<position type="top">0.02</position><packedlength>0.05</packedlength>'
        "<packedradius>0.02</packedradius><mass>0.1</mass></masscomponent>"
    )


def aft_part(airframe, states):
    if airframe == "flare":
        return (
            f"<transition>{head('Aft', AFT, states)}<finish>normal</finish>{MATERIAL}"
            "<length>0.15</length><thickness>0.001</thickness><shape>conical</shape>"
            "<foreradius>0.0001</foreradius><aftradius>0.025</aftradius>"
            "<foreshoulderradius>0.0</foreshoulderradius><foreshoulderlength>0.0</foreshoulderlength>"
            "<aftshoulderradius>0.0</aftshoulderradius><aftshoulderlength>0.0</aftshoulderlength>"
            "</transition>"
        )
    if airframe == "boattail":
        return (
            f"<transition>{head('Aft', AFT, states)}<finish>normal</finish>{MATERIAL}"
            "<length>0.05</length><thickness>0.001</thickness><shape>conical</shape>"
            "<foreradius>0.025</foreradius><aftradius>0.018</aftradius>"
            "<foreshoulderradius>0.0</foreshoulderradius><foreshoulderlength>0.0</foreshoulderlength>"
            "<aftshoulderradius>0.0</aftshoulderradius><aftshoulderlength>0.0</aftshoulderlength>"
            "</transition>"
        )
    radius = "0.025" if airframe == "step up" else "0.02"
    return (
        f"<bodytube>{head('Aft', AFT, states)}<finish>normal</finish>{MATERIAL}"
        f"<length>0.15</length><thickness>0.0005</thickness><radius>{radius}</radius></bodytube>"
    )


def document(airframe, states):
    """The airframe called `airframe`, with drag stated as `states` says ([`stated`])."""
    contents = tube_contents(states) if airframe == "flare" else ""
    radius = "0.02" if airframe == "step up" else "0.025"
    return (
        "<?xml version='1.0' encoding='utf-8'?>\n"
        '<openrocket version="1.10" creator="hpr-sim drag override probe">'
        f"<rocket><name>Probe</name><id>{uid(98)}</id>"
        "<referencetype>maximum</referencetype><subcomponents>"
        f"<stage>{head('Stage', STAGE, states)}<subcomponents>"
        f"<nosecone>{head('Nose', NOSE, states)}<finish>normal</finish>{MATERIAL}"
        "<length>0.12</length><thickness>0.001</thickness><shape>ogive</shape>"
        f"<shapeparameter>1.0</shapeparameter><aftradius>{radius}</aftradius>"
        "<aftshoulderradius>0.0</aftshoulderradius><aftshoulderlength>0.0</aftshoulderlength>"
        "<aftshoulderthickness>0.0</aftshoulderthickness>"
        "<aftshouldercapped>false</aftshouldercapped></nosecone>"
        f"<bodytube>{head('Body', TUBE, states)}<finish>normal</finish>{MATERIAL}"
        f"<length>0.4</length><thickness>0.0005</thickness><radius>{radius}</radius>"
        f"<subcomponents>{contents}</subcomponents></bodytube>"
        f"{aft_part(airframe, states)}"
        "</subcomponents></stage></subcomponents></rocket></openrocket>\n"
    )


#: Each variant: its airframe, and the parts whose drag it states.
VARIANTS = {
    "flare": ("flare", {}),
    "flare, the flare at 0": ("flare", {AFT: (0.0, False)}),
    "flare, the flare at 0.5": ("flare", {AFT: (0.5, False)}),
    "flare, the flare at 0.5 with its children": ("flare", {AFT: (0.5, True)}),
    "flare, the tube at 0.5": ("flare", {TUBE: (0.5, False)}),
    "flare, the tube at 0.5 with its children": ("flare", {TUBE: (0.5, True)}),
    "flare, the tube and the flare at 0": ("flare", {TUBE: (0.0, False), AFT: (0.0, False)}),
    "flare, the nose at 0.3": ("flare", {NOSE: (0.3, False)}),
    "flare, the fins at 0.5": ("flare", {FINS: (0.5, False)}),
    "flare, the lugs at 0.5": ("flare", {LUG: (0.5, False)}),
    "flare, the buttons at 0.5": ("flare", {BUTTONS: (0.5, False)}),
    "flare, the inner tube at 0.5": ("flare", {INNER: (0.5, False)}),
    "flare, the ring at 0.5": ("flare", {RING: (0.5, False)}),
    "flare, the mass at 0.5": ("flare", {MASS: (0.5, False)}),
    "flare, the stage at 0.5": ("flare", {STAGE: (0.5, False)}),
    "flare, the stage at 0.5 with its children": ("flare", {STAGE: (0.5, True)}),
    "boattail": ("boattail", {}),
    "boattail, the boattail at 0.2": ("boattail", {AFT: (0.2, False)}),
    "boattail, the tube at 0.3": ("boattail", {TUBE: (0.3, False)}),
    "step": ("step", {}),
    "step, the fore tube at 0.3": ("step", {TUBE: (0.3, False)}),
    "step, the aft tube at 0.3": ("step", {AFT: (0.3, False)}),
    "step up": ("step up", {}),
    "step up, the fore tube at 0.3": ("step up", {TUBE: (0.3, False)}),
    "step up, the aft tube at 0.3": ("step up", {AFT: (0.3, False)}),
}


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
    calculator = BarrowmanCalculator()
    total = calculator.getAerodynamicForces(configuration, conditions, warnings)
    analysis = BarrowmanCalculator().getForceAnalysis(configuration, conditions, warnings)
    return {
        "mach": mach,
        "reference_area_m2": float(conditions.getRefArea()),
        "rocket": {**columns(total), "axial": float(total.getCDaxial())},
        "components": {
            str(component.getID()): {"name": str(component.getName()), **columns(force)}
            for component, force in analysis.items()
        },
        "warnings": sorted(str(warning) for warning in warnings),
    }


def main():
    if len(sys.argv) != 2:
        sys.exit("usage: drag_override.py OUTPUT.json")
    output = Path(sys.argv[1])
    logging.disable(logging.CRITICAL)
    automatic_radius.start()
    import jpype
    from info.openrocket.core.util import BuildProperties
    from java.lang import System

    variants = {}
    with tempfile.TemporaryDirectory() as scratch:
        for name, (airframe, states) in VARIANTS.items():
            text = document(airframe, states)
            path = Path(scratch) / "probe.ork"
            path.write_text(text, encoding="utf-8")
            variants[name] = {
                "airframe": airframe,
                "document": text,
                "machs": [forces(path, mach) for mach in MACHS],
            }

    fixture = {
        "source": "validation/oracles/openrocket/drag_override.py",
        "command": "refs/venv/bin/python validation/oracles/openrocket/drag_override.py "
        "validation/fixtures/ork/openrocket-drag-override.json",
        "generated": GENERATED,
        "inputs_sha256": {
            name: hashlib.sha256(Path(module.__file__).read_bytes()).hexdigest()
            for name, module in [
                ("drag_override.py", sys.modules[__name__]),
                ("automatic_radius.py", automatic_radius),
            ]
        },
        "openrocket": str(BuildProperties.getVersion()),
        "jar_sha256": hashlib.sha256(automatic_radius.JAR.read_bytes()).hexdigest(),
        "java": str(System.getProperty("java.version")),
        "jpype": jpype.__version__,
        "variants": variants,
    }
    text = json.dumps(fixture, indent=2, ensure_ascii=False, sort_keys=True) + "\n"
    output.write_text(text, encoding="utf-8")


if __name__ == "__main__":
    main()
