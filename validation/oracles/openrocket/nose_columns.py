"""OpenRocket 24.12's pressure drag of a design's nose cone against Mach number (M2.3c13).

M2.3c6 splits the gap between HPR Sim's and OpenRocket's apogees by cause (ADR-226, ADR-349).
Inside the drag part, M2.3c13 splits off the pressure drag of a tangent-ogive nose above Mach 1:
HPR Sim flies again with its own nose pressure drag replaced by OpenRocket's. This script records
what it is replaced with: for each flight's design and configuration, the pressure drag
coefficient OpenRocket's public interface reports for the nose cone, at zero angle of attack, at
each Mach number of `MACHS`. It is the column M2.3c10's controls identified
(`nose_pressure_drag.py`): the nose's own pressure drag, on the rocket's reference area.

Each flight either holds a column or lacks it, under the first reason of `REASONS` that applies;
the list is closed. A design with more than one nose cone (a nose on each pod, say) gets no
column: no probe has shown how the column counts a repeated nose.

A column is asked of one loaded design, with a new calculator and new conditions at each Mach
number. Its first and last values are then asked again of a newly loaded design, and must be the
same, or the flight lacks its column as not repeatable.

Two sets:

- `--public`: each motor configuration of the public designs given. The record is committed.
- `--collection`: every flight of `corpus-out/hpr-sim-fixtures/conditions.json`, its
  configuration chosen as `fixture_flights.py` chooses it. The record stays under the gitignored
  `corpus-out/`: the designs are private.

OpenRocket is run, never read: its source is GPL, and nothing here comes from it. The class and
method names used are the public API that `javap` prints for the jar. OpenRocket 24.12 needs
Java 17 exactly; see `automatic_radius.py`. Run from the repository root:

    refs/venv/bin/python validation/oracles/openrocket/nose_columns.py --public \\
        validation/fixtures/ork/openrocket-nose-columns.json \\
        validation/fixtures/ork/pod-flights/pods-none.ork \\
        validation/fixtures/ork/history-flights/mach-probe.ork
    refs/venv/bin/python validation/oracles/openrocket/nose_columns.py --collection \\
        corpus-out/hpr-sim-fixtures/openrocket-nose-columns.json

The record is written to the path given, not to standard output, which OpenRocket logs to.
"""

import hashlib
import json
import logging
import math
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / "rocketserializer"))

import automatic_radius  # noqa: E402 - the jar's path
import events  # noqa: E402 - the JVM start with the motor database loaded
import fixture_flights  # noqa: E402 - the collection's conditions
import geometry  # noqa: E402 - the same file reading and comment retry

#: The Mach numbers a column is asked at: 1.00 to 2.50, the top of the core band, every 0.01.
MACHS = [round(1.0 + 0.01 * i, 2) for i in range(151)]

#: Why a flight lacks its nose's column, in the order they are tried.
REASONS = {
    "not_opened": "OpenRocket could not open the design or find its configuration",
    "no_nose": "the configuration holds no nose cone",
    "several_noses": "the configuration holds more than one nose cone, and no probe shows how "
    "the column counts a repeated nose",
    "not_finite": "the column is not a number at a Mach number",
    "not_repeatable": "a newly loaded design gives a different column",
}


def relative(path):
    """A script's path from the repository's root, with forward slashes."""
    return Path(path).resolve().relative_to(Path(__file__).resolve().parents[3]).as_posix()


def sha256(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def analysis_at(configuration, mach):
    """Each part's forces at `mach` and zero angle of attack, from a new calculator, and the
    reference area they are on."""
    from info.openrocket.core.aerodynamics import BarrowmanCalculator, FlightConditions
    from info.openrocket.core.logging import WarningSet

    conditions = FlightConditions(configuration)
    conditions.setMach(mach)
    conditions.setAOA(0.0)
    analysis = BarrowmanCalculator().getForceAnalysis(configuration, conditions, WarningSet())
    return analysis, float(conditions.getRefArea())


def noses_of(analysis):
    """The nose cones among the parts the analysis lists, with their forces."""
    return [
        (component, force)
        for component, force in analysis.items()
        if str(component.getClass().getSimpleName()) == "NoseCone"
    ]


def column_of(configuration, machs):
    """The nose's pressure drag coefficient at each of `machs`, with the reference area and the
    nose's shape, or the reason there is none."""
    rows = []
    shape = None
    area = None
    for mach in machs:
        analysis, area = analysis_at(configuration, mach)
        noses = noses_of(analysis)
        if not noses:
            return "no_nose"
        if len(noses) > 1:
            return "several_noses"
        component, force = noses[0]
        value = float(force.getPressureCD())
        if not math.isfinite(value):
            return "not_finite"
        rows.append([mach, value])
        shape = {
            "shape": str(component.getShapeType().name()),
            "shape_parameter": float(component.getShapeParameter()),
        }
    return {"rows": rows, "reference_area_m2": area, "nose": shape}


def chosen(rocket, wanted):
    """The motor configuration called `wanted`, as `fixture_flights.main` chooses it, or `None`;
    with `wanted` `None`, nothing is chosen."""
    ids = [i for i in rocket.getIds() if rocket.getFlightConfiguration(i).hasMotors()]
    found = [i for i in ids if str(i.toString()).lower() == (wanted or "").lower()]
    if not found and len(ids) == 1 and wanted is not None:
        found = ids
    return found[0] if found else None


def entry_of(text, scratch, configuration_id):
    """A design's record: its nose's column for the configuration `configuration_id` names (a
    function of the rocket that gives the id, or `None`), or what it lacks."""
    try:
        document, _ = geometry.opened(text, scratch)
        rocket = document.getRocket()
        chosen_id = configuration_id(rocket)
        if chosen_id is None:
            return {"lacks": "not_opened"}
        column = column_of(rocket.getFlightConfiguration(chosen_id), MACHS)
    except Exception:  # noqa: BLE001 - a refusal is the measurement
        return {"lacks": "not_opened"}
    if isinstance(column, str):
        return {"lacks": column}
    # The same ends from a newly loaded design.
    document, _ = geometry.opened(text, scratch)
    rocket = document.getRocket()
    again = column_of(
        rocket.getFlightConfiguration(configuration_id(rocket)), [MACHS[0], MACHS[-1]]
    )
    ends = [column["rows"][0], column["rows"][-1]]
    if isinstance(again, str) or again["rows"] != ends:
        return {"lacks": "not_repeatable"}
    return column


def public(paths, scratch):
    """Every motor configuration of the public designs at `paths`."""
    designs = []
    for path in paths:
        text = geometry.document_text(path)
        document, _ = geometry.opened(text, scratch)
        rocket = document.getRocket()
        record = {"file": Path(path).as_posix(), "sha256": sha256(path), "flights": []}
        for configuration in rocket.getIds():
            if not rocket.getFlightConfiguration(configuration).hasMotors():
                continue
            name = str(configuration.toString())
            entry = {"configuration": name}
            entry.update(entry_of(text, scratch, lambda r, name=name: chosen(r, name)))
            record["flights"].append(entry)
        designs.append(record)
    return {"designs": designs}


def collection(scratch):
    """Every flight of the collection's conditions."""
    conditions = json.loads(fixture_flights.CONDITIONS.read_text(encoding="utf-8"))
    out = []
    for flight in conditions["flights"]:
        entry = {"flight_id": flight["flight_id"]}
        out.append(entry)
        try:
            text = geometry.document_text(flight["design"])
        except Exception:  # noqa: BLE001 - a refusal is the measurement
            entry["lacks"] = "not_opened"
            continue
        wanted = flight["configuration"]
        entry.update(entry_of(text, scratch, lambda r, wanted=wanted: chosen(r, wanted)))
    return {"conditions_sha256": sha256(fixture_flights.CONDITIONS), "flights": out}


def dumps(record):
    """`record` as JSON, each column on a line of its own."""
    columns = []

    def held(value):
        if isinstance(value, dict):
            return {k: held(v) for k, v in value.items()}
        if isinstance(value, list):
            if value and all(isinstance(r, list) for r in value):
                columns.append(value)
                return f"@rows{len(columns) - 1}@"
            return [held(v) for v in value]
        return value

    text = json.dumps(held(record), indent=1, sort_keys=True, allow_nan=False)
    for index, value in enumerate(columns):
        text = text.replace(f'"@rows{index}@"', json.dumps(value, allow_nan=False))
    return text + "\n"


def main():
    if len(sys.argv) < 3 or sys.argv[1] not in ("--public", "--collection"):
        sys.exit("usage: nose_columns.py (--public OUTPUT.json FILE... | --collection OUTPUT.json)")
    mode, output, paths = sys.argv[1], Path(sys.argv[2]), sys.argv[3:]
    if (mode == "--public") != bool(paths):
        sys.exit("--public takes design files; --collection takes none")
    logging.disable(logging.CRITICAL)
    events.start()
    import jpype
    from info.openrocket.core.util import BuildProperties
    from java.lang import System

    with tempfile.TemporaryDirectory() as scratch:
        body = public(paths, scratch) if mode == "--public" else collection(scratch)
    modules = (sys.modules[__name__], automatic_radius, events, fixture_flights, geometry)
    record = {
        "source": "validation/oracles/openrocket/nose_columns.py",
        "command": " ".join(["nose_columns.py", *sys.argv[1:]]),
        "inputs_sha256": {relative(m.__file__): sha256(m.__file__) for m in modules},
        "openrocket": str(BuildProperties.getVersion()),
        "jar_sha256": sha256(automatic_radius.JAR),
        "java": str(System.getProperty("java.version")),
        "jpype": jpype.__version__,
        "machs": MACHS,
        "reasons": REASONS,
        **body,
    }
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(dumps(record), encoding="utf-8")


if __name__ == "__main__":
    main()
