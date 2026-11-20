"""Which of OpenRocket 24.12's three drag terms takes a launch lug, rail buttons and a stated
drag coefficient.

M1.14i2 (ADR-359) splits the drag below Mach 0.8 into friction, pressure, base and a rest, each
code's terms set against the other's. HPR Sim books launch lugs and rail buttons as parasitic
drag and a stated coefficient as a term of its own; OpenRocket's flight data gives only
friction, pressure and base (`TYPE_FRICTION_DRAG_COEFF`, `TYPE_PRESSURE_DRAG_COEFF`,
`TYPE_BASE_DRAG_COEFF`), which sum to its drag coefficient. This probe measures where
OpenRocket puts each of the three: it flies a public design as it is, then with one part added
or one coefficient stated, and records each term of each flight at Mach 0.3 and 0.6 on the
coast, linear between OpenRocket's rows. A term that moves holds what was added.

The variants, each on `mach-probe.ork`'s body tube unless named (its coast passes both Mach
numbers):

- `as_is`: the design as it is;
- `launch_lug`: a launch lug 1/4 in across the outside and 6 in long;
- `rail_buttons`: two rail buttons of OpenRocket's own default size;
- `stated_body`: the body tube's drag coefficient stated as 0.3, its own children's left theirs;
- `stated_fins`: the fin set's drag coefficient stated as 0.3.

Each is flown as `histories.py` flies a public design: the design's first stored simulation's
conditions, calm air, the same seed, nothing deployed.

OpenRocket is run, never read: its source is GPL, and nothing here comes from it. The class and
method names used are the public API that `javap` prints for the jar. Run from the repository
root:

    refs/venv/bin/python validation/oracles/openrocket/drag_terms.py \\
        validation/fixtures/ork/openrocket-drag-terms.json \\
        validation/fixtures/ork/history-flights/mach-probe.ork

The record is written to the path given, not to standard output, which OpenRocket logs to.
"""

import json
import logging
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / "rocketserializer"))

import automatic_radius  # noqa: E402 - the jar's path
import events  # noqa: E402 - the JVM start with the motor database loaded
import flights  # noqa: E402 - calm air and seed
import geometry  # noqa: E402 - the same file reading and comment retry
import histories  # noqa: E402 - the rows, as the histories record holds them

#: The Mach numbers each term is read at, on the coast.
MACHS = [0.3, 0.6]

#: The terms, as indices into `histories.COLUMNS`.
TERMS = {
    "drag_coefficient": histories.COLUMNS.index("drag_coefficient"),
    "friction": histories.COLUMNS.index("friction_coefficient"),
    "pressure": histories.COLUMNS.index("pressure_coefficient"),
    "base": histories.COLUMNS.index("base_coefficient"),
}

INCH = 0.0254


def coast(rows, mach, index):
    """Column `index` at `mach` on the coast: the rows after the last with thrust, linear
    between the two whose Mach numbers bracket it."""
    thrust, speed = histories.COLUMNS.index("thrust_n"), histories.COLUMNS.index("mach")
    last = max(i for i, row in enumerate(rows) if row[thrust] > 0.0)
    after = rows[last + 1 :]
    for before, row in zip(after, after[1:]):
        high, low = before[speed], row[speed]
        if high >= mach >= low and high > low:
            w = (high - mach) / (high - low)
            return before[index] + w * (row[index] - before[index])
    raise RuntimeError(f"the coast does not pass Mach {mach}")


def component(rocket, kind):
    """The first component of class `kind` in `rocket`."""
    for found in events.components(rocket, kind):
        return found
    raise RuntimeError(f"the design has no {kind}")


def vary(rocket, variant):
    """Changes `rocket` as `variant` names."""
    from info.openrocket.core.rocketcomponent import (
        BodyTube,
        FinSet,
        LaunchLug,
        RailButton,
    )

    body = component(rocket, BodyTube)
    if variant == "launch_lug":
        lug = LaunchLug()
        lug.setOuterRadius(INCH / 8)
        lug.setThickness(INCH / 32)
        lug.setLength(6 * INCH)
        body.addChild(lug)
    elif variant == "rail_buttons":
        buttons = RailButton()
        buttons.setInstanceCount(2)
        buttons.setInstanceSeparation(0.3)
        body.addChild(buttons)
    elif variant == "stated_body":
        body.setCDOverridden(True)
        body.setOverrideCD(0.3)
    elif variant == "stated_fins":
        fins = component(rocket, FinSet)
        fins.setCDOverridden(True)
        fins.setOverrideCD(0.3)
    elif variant != "as_is":
        raise RuntimeError(f"no variant {variant}")


def flown(path, variant, scratch):
    """The design at `path`, changed by `variant`, flown; its terms at `MACHS`."""
    from info.openrocket.core.document import Simulation

    text = geometry.document_text(path)
    document, _ = geometry.opened(text, scratch)
    rocket = document.getRocket()
    vary(rocket, variant)
    stored = list(document.getSimulations())
    configuration = next(
        i for i in rocket.getIds() if rocket.getFlightConfiguration(i).hasMotors()
    )
    flights.never_deploy(document, configuration)
    simulation = Simulation(document, rocket)
    if stored:
        simulation.copySimulationOptionsFrom(stored[0].getOptions())
    simulation.setFlightConfigurationId(configuration)
    options = simulation.getOptions()
    options.setWindSpeedAverage(0.0)
    options.setWindTurbulenceIntensity(0.0)
    options.setLaunchIntoWind(False)
    options.setRandomSeed(flights.SEED)
    simulation.simulate()
    rows, reference = histories.rows_of(simulation)
    return {
        "reference_length_m": reference,
        "at_mach": [
            {"mach": mach, **{name: coast(rows, mach, i) for name, i in TERMS.items()}}
            for mach in MACHS
        ],
    }


def main():
    if len(sys.argv) != 3:
        sys.exit("usage: drag_terms.py OUTPUT.json DESIGN.ork")
    output, path = Path(sys.argv[1]), sys.argv[2]
    logging.disable(logging.CRITICAL)
    events.start()
    from info.openrocket.core.util import BuildProperties

    variants = ["as_is", "launch_lug", "rail_buttons", "stated_body", "stated_fins"]
    with tempfile.TemporaryDirectory() as scratch:
        body = {variant: flown(path, variant, scratch) for variant in variants}
    modules = (sys.modules[__name__], automatic_radius, events, flights, geometry, histories)
    record = {
        "source": "validation/oracles/openrocket/drag_terms.py",
        "command": " ".join(["drag_terms.py", *sys.argv[1:]]),
        "inputs_sha256": {
            histories.relative(m.__file__): histories.sha256(m.__file__) for m in modules
        },
        "design": Path(path).as_posix(),
        "design_sha256": histories.sha256(path),
        "openrocket": str(BuildProperties.getVersion()),
        "jar_sha256": histories.sha256(automatic_radius.JAR),
        "variants": body,
    }
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(record, indent=1, sort_keys=True) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
