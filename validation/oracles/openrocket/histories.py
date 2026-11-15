"""OpenRocket 24.12's thrust, mass, center of mass and drag coefficient along its own flights.

M2.3c6 splits the gap between HPR Sim's and OpenRocket's apogees into thrust, mass and drag parts
by swapping each input for OpenRocket's (ADR-226, ADR-349). M2.3c11 records those inputs: for
each flight, OpenRocket's own rows from launch to the row of the largest altitude, each row its
time and, at that time,

- `thrust_n`: `TYPE_THRUST_FORCE`, the thrust OpenRocket flew, N;
- `mass_kg`: `TYPE_MASS`, the whole rocket's mass, kg;
- `cg_m`: `TYPE_CG_LOCATION`, its center of mass, m aft of the nose tip;
- `drag_coefficient`: `TYPE_DRAG_COEFF`, on the reference area of `reference_length_m` across;
- `mach`: `TYPE_MACH_NUMBER`, which the drag coefficient is taken against;
- `altitude_m`: `TYPE_ALTITUDE`, 0 at launch.

A value OpenRocket gives as not-a-number is written as `null`. Each flight either holds every
input or lacks one, under the first reason of `REASONS` that applies; the list is closed, and a
flight that fits none of them stops the run.

Two sets are flown:

- `--public`: each motor configuration of the public designs given, flown as `flights.py` flies
  it (the design's first stored simulation's conditions, or OpenRocket's defaults, in calm air,
  with the same seed) with nothing deployed. The record is committed: HPR Sim's own-history
  test samples a flight at its times.
- `--collection`: every flight of `corpus-out/hpr-sim-fixtures/conditions.json`, flown as
  `fixture_flights.py` flies it (its configuration choice, site, rail and day's weather, nothing
  deployed). Each flight's apogee must be the one `fixture_flights.py` recorded, within
  `SAME_FLIGHT`, or the flight lacks its inputs as another flight. The record stays under the
  gitignored `corpus-out/`: the flights are private.

OpenRocket is run, never read: its source is GPL, and nothing here comes from it. The class and
method names used are the public API that `javap` prints for the jar. OpenRocket 24.12 needs
Java 17 exactly; see `automatic_radius.py`. Run from the repository root:

    refs/venv/bin/python validation/oracles/openrocket/histories.py --public \\
        validation/fixtures/ork/openrocket-histories.json \\
        validation/fixtures/ork/pod-flights/pods-none.ork \\
        validation/fixtures/ork/history-flights/mach-probe.ork
    refs/venv/bin/python validation/oracles/openrocket/histories.py --collection \\
        corpus-out/hpr-sim-fixtures/openrocket-histories.json

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
import fixture_flights  # noqa: E402 - the collection's conditions, as its flights are flown
import flights  # noqa: E402 - the public designs' conditions, calm air and seed
import geometry  # noqa: E402 - the same file reading and comment retry

#: Why a flight lacks an input, in the order they are tried: the first that applies is recorded.
REASONS = {
    "not_flown": "OpenRocket did not fly it to an apogee: it refused or aborted the flight, or "
    "could not open the design or find its configuration",
    "another_flight": "the flight flown here is not the one fixture_flights.py recorded: its "
    "largest altitude differs by more than SAME_FLIGHT",
    "thrust_not_finite": "the thrust is not a number at a row",
    "mass_not_finite": "the mass is not a number at a row",
    "cg_not_finite": "the center of mass is not a number at a row",
    "drag_not_finite": "the drag coefficient is not a number at a row whose Mach number is "
    "above zero",
}

#: How far, relative, the largest altitude may move from `fixture_flights.py`'s and the flight
#: still be the one it recorded. OpenRocket 24.12 is not bit-identical from one run to the next,
#: so each flight records its move (`apogee_moved`), and the fixture report gives the largest.
#: The bound is a fiftieth of the own-history test's 5e-4.
SAME_FLIGHT = 1e-5

#: The columns of each row, in order.
COLUMNS = ["time_s", "thrust_n", "mass_kg", "cg_m", "drag_coefficient", "mach", "altitude_m"]


def relative(path):
    """A script's path from the repository's root, with forward slashes."""
    return Path(path).resolve().relative_to(Path(__file__).resolve().parents[3]).as_posix()


def sha256(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def number(value):
    """`value` as a float, or `None` for OpenRocket's not-a-number."""
    value = float(value)
    return value if math.isfinite(value) else None


def rows_of(simulation):
    """The first branch's rows from launch to the row of the largest altitude, and the
    reference length, or why there are none."""
    from info.openrocket.core.simulation import FlightDataType as F

    data = simulation.getSimulatedData()
    branch = data.getBranch(0)
    kinds = [
        F.TYPE_TIME,
        F.TYPE_THRUST_FORCE,
        F.TYPE_MASS,
        F.TYPE_CG_LOCATION,
        F.TYPE_DRAG_COEFF,
        F.TYPE_MACH_NUMBER,
        F.TYPE_ALTITUDE,
    ]
    columns = [[float(x) for x in branch.get(kind)] for kind in kinds]
    altitude = columns[-1]
    finite_altitudes = [a for a in altitude if math.isfinite(a)]
    aborted = any(events.name_of(e.getType()) == "SIM_ABORT" for e in branch.getEvents())
    apogee_events = any(events.name_of(e.getType()) == "APOGEE" for e in branch.getEvents())
    if not finite_altitudes or (aborted and not apogee_events):
        return None, None
    highest = altitude.index(max(finite_altitudes))
    rows = [[number(column[i]) for column in columns] for i in range(highest + 1)]
    reference = [float(x) for x in branch.get(F.TYPE_REFERENCE_LENGTH)]
    return rows, (number(reference[0]) if reference else None)


def lacking(rows):
    """The first reason after the flight's own that the rows lack an input, or `None`."""
    for name, index in [("thrust", 1), ("mass", 2), ("cg", 3)]:
        if any(row[index] is None for row in rows):
            return f"{name}_not_finite"
    if any(row[4] is None and row[5] is not None and row[5] > 0.0 for row in rows):
        return "drag_not_finite"
    if any(row[0] is None for row in rows):
        raise RuntimeError("a row's time is not a number, which no reason in REASONS covers")
    return None


def entry_of(simulation):
    """A flown simulation's record: its rows and what it lacks."""
    rows, reference = rows_of(simulation)
    if rows is None:
        return {"lacks": "not_flown"}
    entry = {
        "rows": rows,
        "reference_length_m": reference,
        "apogee_m": max(row[6] for row in rows if row[6] is not None),
        "max_mach": max((row[5] for row in rows if row[5] is not None), default=None),
    }
    reason = lacking(rows)
    if reason is not None:
        entry["lacks"] = reason
    return entry


def simulate(simulation):
    """Runs `simulation`; `False` if OpenRocket refused it."""
    try:
        simulation.simulate()
    except Exception:  # noqa: BLE001 - a refusal is the measurement
        return False
    return True


def public(paths, scratch):
    """Every motor configuration of the public designs at `paths`, as `flights.py` flies it,
    with nothing deployed."""
    from info.openrocket.core.document import Simulation

    designs = []
    for path in paths:
        text = geometry.document_text(path)
        document, _ = geometry.opened(text, scratch)
        rocket = document.getRocket()
        stored = list(document.getSimulations())
        base = stored[0] if stored else None
        record = {"file": Path(path).as_posix(), "sha256": sha256(path), "flights": []}
        for configuration in rocket.getIds():
            if not rocket.getFlightConfiguration(configuration).hasMotors():
                continue
            flights.never_deploy(document, configuration)
            simulation = Simulation(document, rocket)
            if base is not None:
                simulation.copySimulationOptionsFrom(base.getOptions())
            simulation.setFlightConfigurationId(configuration)
            options = simulation.getOptions()
            # As `flights.flight` sets them.
            options.setWindSpeedAverage(0.0)
            options.setWindTurbulenceIntensity(0.0)
            options.setLaunchIntoWind(False)
            options.setRandomSeed(flights.SEED)
            entry = {
                "configuration": str(configuration.toString()),
                "conditions": flights.conditions(options),
            }
            if simulate(simulation):
                entry.update(entry_of(simulation))
            else:
                entry["lacks"] = "not_flown"
            record["flights"].append(entry)
        designs.append(record)
    return {"designs": designs}


def collection(scratch):
    """Every flight of the collection's conditions, as `fixture_flights.py` flies it."""
    conditions = json.loads(fixture_flights.CONDITIONS.read_text(encoding="utf-8"))
    recorded = {
        f["flight_id"]: f
        for f in json.loads(fixture_flights.RECORD.read_text(encoding="utf-8"))["flights"]
    }
    out = []
    for flight in conditions["flights"]:
        entry = {"flight_id": flight["flight_id"]}
        out.append(entry)
        try:
            text = geometry.document_text(flight["design"])
            document, _ = geometry.opened(text, scratch)
        except Exception:  # noqa: BLE001 - a refusal is the measurement
            entry["lacks"] = "not_flown"
            continue
        # The configuration as `fixture_flights.main` chooses it.
        rocket = document.getRocket()
        wanted = (flight["configuration"] or "").lower()
        ids = [i for i in rocket.getIds() if rocket.getFlightConfiguration(i).hasMotors()]
        found = [i for i in ids if str(i.toString()).lower() == wanted]
        if not found and len(ids) == 1 and flight["configuration"] is not None:
            found = ids
        if not found:
            entry["lacks"] = "not_flown"
            continue
        stored = list(document.getSimulations())
        simulation, options = fixture_flights.fly(
            document, found[0], stored[0] if stored else None, None
        )
        fixture_flights.set_conditions(options, flight)
        fixture_flights.never_deploy(document, found[0])
        if not simulate(simulation):
            entry["lacks"] = "not_flown"
            continue
        entry.update(entry_of(simulation))
        if entry.get("lacks") == "not_flown":
            continue
        held = recorded.get(flight["flight_id"], {}).get("apogee_m")
        if held is None:
            entry["lacks"] = "another_flight"
            continue
        entry["apogee_moved"] = (entry["apogee_m"] - held) / held
        if abs(entry["apogee_moved"]) > SAME_FLIGHT:
            entry["lacks"] = "another_flight"
    return {
        "conditions_sha256": sha256(fixture_flights.CONDITIONS),
        "flights_record_sha256": sha256(fixture_flights.RECORD),
        "flights": out,
    }


def dumps(record):
    """`record` as JSON, each row on a line of its own."""
    rows = []

    def held(value):
        if isinstance(value, dict):
            return {k: held(v) for k, v in value.items()}
        if isinstance(value, list):
            if value and all(isinstance(r, list) for r in value):
                rows.append(value)
                return f"@rows{len(rows) - 1}@"
            return [held(v) for v in value]
        return value

    text = json.dumps(held(record), indent=1, sort_keys=True, allow_nan=False)
    for index, value in enumerate(rows):
        lines = ",\n  ".join(json.dumps(r, allow_nan=False) for r in value)
        text = text.replace(f'"@rows{index}@"', f"[\n  {lines}\n ]")
    return text + "\n"


def main():
    if len(sys.argv) < 3 or sys.argv[1] not in ("--public", "--collection"):
        sys.exit("usage: histories.py (--public OUTPUT.json FILE... | --collection OUTPUT.json)")
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
    modules = (sys.modules[__name__], automatic_radius, events, fixture_flights, flights, geometry)
    record = {
        "source": "validation/oracles/openrocket/histories.py",
        "command": " ".join(["histories.py", *sys.argv[1:]]),
        "inputs_sha256": {relative(m.__file__): sha256(m.__file__) for m in modules},
        "openrocket": str(BuildProperties.getVersion()),
        "jar_sha256": sha256(automatic_radius.JAR),
        "java": str(System.getProperty("java.version")),
        "jpype": jpype.__version__,
        "columns": COLUMNS,
        "reasons": REASONS,
        **body,
    }
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(dumps(record), encoding="utf-8")


if __name__ == "__main__":
    main()
