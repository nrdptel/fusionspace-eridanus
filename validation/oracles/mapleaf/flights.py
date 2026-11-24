"""Fly HPR Sim's public designs in MAPLEAF, as an outside program, on the inputs the two share.

MAPLEAF (MIT, https://github.com/henrystoldt/MAPLEAF) is a 6-DOF rocket flight simulator with its
own aerodynamics, written apart from HPR Sim and from RocketPy. This script reads the inputs
`cargo xtask mapleaf-flights --inputs` wrote from HPR Sim's own designs
(`validation/fixtures/flight/mapleaf-inputs.json`), writes each case as a MAPLEAF simulation
definition and motor file in a temporary folder, runs MAPLEAF on it at two fixed time steps, and
writes what MAPLEAF flew and found to `validation/fixtures/flight/mapleaf-flights.json`.

It passes MAPLEAF only what the inputs file gives: the outside shape, the mass without propellant
as one point mass with its inertia, the motor's thrust, propellant flow and propellant inertia at
each time, the guided travel and the rail's direction, and the site's elevation. Everything else
is MAPLEAF's own: its aerodynamics, its atmosphere table, its gravity and its integrator. After
each run it reads MAPLEAF's own log back and refuses a case whose mass, center of mass or inertia
at ignition or burnout is not the inputs'.

Each case flies twice. With its own drag, MAPLEAF works its aerodynamics out from the outside
shape. With the same drag, the shape is left out and one `TabulatedAeroForce` carries the inputs'
table of drag coefficient against Mach number, which HPR Sim flies too: what is left between the
two codes is then everything but the drag.

Build the MAPLEAF environment from this folder's lock file, then run the script with its Python:

    UV_PROJECT_ENVIRONMENT=$PWD/refs/venv-mapleaf uv sync --locked --project validation/oracles/mapleaf
    refs/venv-mapleaf/bin/python -I validation/oracles/mapleaf/flights.py

`--check` runs everything and compares with the committed file instead of writing it.
"""

import argparse
import bisect
import csv
import hashlib
import importlib.metadata
import json
import math
import os
import pathlib
import sys
import tempfile

ROOT = pathlib.Path(__file__).resolve().parents[3]
INPUTS = "validation/fixtures/flight/mapleaf-inputs.json"
OUTPUT = "validation/fixtures/flight/mapleaf-flights.json"
GENERATOR = "validation/oracles/mapleaf/flights.py"
COMMAND = "refs/venv-mapleaf/bin/python -I validation/oracles/mapleaf/flights.py"

# The fixed steps of MAPLEAF's classical Runge-Kutta integrator, s: the values the fixture gives
# are the finer step's, and the coarser step's sit beside them to show what the step is worth.
TIME_STEPS_S = (0.002, 0.001)

# How closely MAPLEAF's own log must give back the mass properties it was handed, as a fraction
# of the value (of the rocket's length, for a center of mass).
MASS_TOLERANCE = 1e-6

# The settings left at MAPLEAF's own defaults, named here so the fixture records them
# (MAPLEAF/IO/simDefinition.py at the pinned commit).
MAPLEAF_DEFAULTS = {
    "Rocket.Aero.fullyTurbulentBL": "true",
    "Rocket.Aero.addZeroLengthBoatTailsToAccountForBaseDrag": "true",
    "Environment.EarthModel": "Flat",
    "Environment.AtmosphericPropertiesModel": "USStandardAtmosphere",
    "Environment.TurbulenceModel": "None",
}


def vector(*values):
    """A MAPLEAF vector literal."""
    return "(" + " ".join(repr(float(v)) for v in values) + ")"


def motor_rows(case):
    """The motor's rows as dictionaries: the inputs give each as numbers in `columns`' order."""
    motor = case["motor"]
    return [dict(zip(motor["columns"], row)) for row in motor["rows"]]


def motor_file(case):
    """The case's motor in MAPLEAF's own text format.

    MAPLEAF reads, per row, the time, the thrust, an oxidizer flow, a fuel flow and each one's
    moments of inertia about its own center (Rocket/Propulsion.py). The propellant goes in as the
    fuel. It integrates the flow by the trapezoid rule backward from the last row for the mass
    left, and moves the fuel's center in a straight line with the burnt fraction between the two
    `BurnedFuel...CG` stations, so this hands it the first and last centers the inputs give.
    """
    rows = motor_rows(case)
    with_mass = [row for row in rows if row["propellant_mass_kg"] > 0.0]
    first, last = with_mass[0], with_mass[-1]
    lines = [
        "# Written by validation/oracles/mapleaf/flights.py; not an input anyone edits.",
        "BurnedOxidizerInitialCG 0",
        "BurnedOxidizerLastCG 0",
        f"BurnedFuelInitialCG {-first['propellant_cg_station_m']!r}",
        f"BurnedFuelLastCG {-last['propellant_cg_station_m']!r}",
        "",
    ]
    for row in rows:
        transverse = row["propellant_transverse_inertia_kg_m2"]
        axial = row["propellant_axial_inertia_kg_m2"]
        lines.append(
            f"{row['time_s']!r} {row['thrust_n']!r} 0 {row['mass_flow_kg_s']!r} (0 0 0) "
            f"({transverse!r} {transverse!r} {axial!r})"
        )
    return "\n".join(lines) + "\n"


def center_line_departure_m(case):
    """How far the inputs' propellant center strays from the straight line MAPLEAF moves it on.

    MAPLEAF's fuel center is `first·(1 − f) + last·f` with `f` the burnt fraction of its own
    trapezoid masses; the inputs give HPR Sim's center at every row. The largest gap between the
    two, m, over the rows that still hold propellant.
    """
    rows = [row for row in motor_rows(case) if row["propellant_mass_kg"] > 0.0]
    first, last = rows[0], rows[-1]
    initial = first["propellant_mass_kg"]
    worst = 0.0
    for row in rows:
        burnt = 1.0 - row["propellant_mass_kg"] / initial
        line = first["propellant_cg_station_m"] * (1.0 - burnt) + last["propellant_cg_station_m"] * burnt
        worst = max(worst, abs(row["propellant_cg_station_m"] - line))
    return worst


def definition(case, time_step_s, motor_path, inclination_deg=None, drag_path=None):
    """The case as a MAPLEAF simulation definition.

    MAPLEAF's z axis runs from the tail to the nose with its origin at the nose tip, so a station
    `s` m aft of the tip is `z = −s`. `inclination_deg` flies another rail angle than the case's,
    for the leaning-rail measurement only. `drag_path` leaves the outside shape out and flies
    that table of drag coefficient against Mach number instead, on the inputs' reference area.
    """
    rail = case["rail"]
    inclination = math.radians(rail["inclination_deg"] if inclination_deg is None else inclination_deg)
    heading = math.radians(rail["heading_deg"])
    # East, north, up: the heading is clockwise from north.
    direction = (
        math.sin(heading) * math.cos(inclination),
        math.cos(heading) * math.cos(inclination),
        math.sin(inclination),
    )
    dry = case["dry"]
    parts = []
    for index, part in enumerate(case["body"]):
        common = (
            f"            mass 0\n"
            f"            position {vector(0, 0, -part['fore_station_m'])}\n"
            f"            cg (0 0 0)\n"
            f"            MOI (0 0 0)\n"
            f"            surfaceRoughness {part['roughness_m']!r}\n"
        )
        last = index == len(case["body"]) - 1
        if part["kind"] == "nose":
            shape = {"tangent_ogive": "tangentOgive", "cone": "cone"}[part["shape"]]
            parts.append(
                f"        Part{index}{{\n            class Nosecone\n{common}"
                f"            baseDiameter {part['aft_diameter_m']!r}\n"
                f"            aspectRatio {part['length_m'] / part['aft_diameter_m']!r}\n"
                f"            shape {shape}\n        }}\n"
            )
        elif part["kind"] == "tube":
            parts.append(
                f"        Part{index}{{\n            class Bodytube\n{common}"
                f"            outerDiameter {part['fore_diameter_m']!r}\n"
                f"            length {part['length_m']!r}\n        }}\n"
            )
        elif part["kind"] == "transition":
            # MAPLEAF's boat tail is its transition at the very bottom, which adds the base drag.
            parts.append(
                f"        Part{index}{{\n            class {'BoatTail' if last else 'Transition'}\n{common}"
                f"            startDiameter {part['fore_diameter_m']!r}\n"
                f"            endDiameter {part['aft_diameter_m']!r}\n"
                f"            length {part['length_m']!r}\n        }}\n"
            )
        else:
            raise SystemExit(f"{case['name']}: no MAPLEAF part for {part['kind']}")
    fins = case["fins"]
    if fins["edges"] != "square":
        raise SystemExit(f"{case['name']}: only square fin edges are passed, not {fins['edges']}")
    sweep_deg = math.degrees(math.atan2(fins["sweep_m"], fins["span_m"]))
    fin_set = f"""        Fins{{
            class FinSet
            mass 0
            position {vector(0, 0, -fins['fore_station_m'])}
            cg (0 0 0)
            MOI (0 0 0)
            numFins {fins['count']}
            finCantAngle 0
            firstFinAngle 0
            sweepAngle {sweep_deg!r}
            rootChord {fins['root_chord_m']!r}
            tipChord {fins['tip_chord_m']!r}
            span {fins['span_m']!r}
            thickness {fins['thickness_m']!r}
            surfaceRoughness {fins['roughness_m']!r}
            numFinSpanSlicesForIntegration 10
            LeadingEdge{{
                shape Blunt
                thickness {fins['thickness_m']!r}
            }}
            TrailingEdge{{
                shape Blunt
                thickness {fins['thickness_m']!r}
            }}
        }}
"""
    if drag_path is not None:
        # The drag acts at the center of mass at ignition: it has no moment to speak of on a
        # flight straight up in calm air, and no other aerodynamic force is flown.
        diameter = case["drag"]["reference_diameter_m"]
        parts = []
        fin_set = f"""        Drag{{
            class TabulatedAeroForce
            position {vector(0, 0, -case['ignition']['cg_station_m'])}
            Aref {math.pi * diameter**2 / 4.0!r}
            Lref {diameter!r}
            filePath {drag_path}
        }}
"""
    return f"""# Written by validation/oracles/mapleaf/flights.py from {INPUTS}, case {case['name']}.
SimControl{{
    timeDiscretization RK4
    timeStep {time_step_s!r}
    EndCondition Apogee
    EndConditionValue 0
    plot None
    loggingLevel 2
    RocketPlot Off
}}
Environment{{
    LaunchSite.elevation {case['site']['elevation_m']!r}
    LaunchSite.railLength {rail['guided_travel_m']!r}
    MeanWindModel Constant
    ConstantMeanWind.velocity (0 0 0)
}}
Rocket{{
    name {case['name']}
    position {vector(0, 0, 0)}
    velocity (0 0 0)
    angularVelocity (0 0 0)
    initialDirection {vector(*direction)}
    Sustainer{{
        class Stage
        stageNumber 0
        position (0 0 0)
{''.join(parts)}        Dry{{
            class Mass
            mass {dry['mass_kg']!r}
            position {vector(0, 0, -dry['cg_station_m'])}
            cg (0 0 0)
            MOI {vector(dry['transverse_inertia_kg_m2'], dry['transverse_inertia_kg_m2'], dry['axial_inertia_kg_m2'])}
        }}
        Motor{{
            class Motor
            path {motor_path}
        }}
{fin_set}    }}
}}
"""


def read_log(path):
    """A MAPLEAF log as rows of floats, by column name; a cell that is no number is dropped."""
    rows = []
    with open(path, newline="") as file:
        for row in csv.DictReader(file):
            numbers = {}
            for key, value in row.items():
                try:
                    numbers[key] = float(value)
                except (TypeError, ValueError):
                    pass
            rows.append(numbers)
    return rows


def run_mapleaf(case, time_step_s, folder, inclination_deg=None, same_drag=False):
    """Runs MAPLEAF on one case at one step and returns its step log and its evaluation log."""
    # MAPLEAF imports matplotlib's plotting on the way in: no window, on any machine.
    os.environ.setdefault("MPLBACKEND", "Agg")
    from MAPLEAF.SimulationRunners import Simulation

    folder = pathlib.Path(folder)
    motor_path = folder / f"{case['name']}-motor.txt"
    motor_path.write_text(motor_file(case))
    definition_path = folder / f"{case['name']}.mapleaf"
    drag_path = None
    if same_drag:
        drag = case["drag"]
        drag_path = folder / f"{case['name']}-drag.csv"
        drag_path.write_text(
            "Mach,CD\n"
            + "".join(f"{mach!r},{cd!r}\n" for mach, cd in zip(drag["mach"], drag["drag_coefficient"]))
        )
    definition_path.write_text(definition(case, time_step_s, motor_path, inclination_deg, drag_path))

    here = os.getcwd()
    stdout = sys.stdout
    os.chdir(folder)
    try:
        _, log_paths = Simulation(simDefinitionFilePath=str(definition_path), silent=True).run()
    finally:
        os.chdir(here)
        sys.stdout = stdout
    steps = read_log(next(p for p in log_paths if p.endswith("timeStepLog.csv")))
    evaluations = read_log(next(p for p in log_paths if p.endswith("derivativeEvaluationLog.csv")))
    return steps, evaluations


def leaning_rail(case, folder):
    """What MAPLEAF does on the rail the case's example leans, which is why no case flies one.

    At the pinned commit MAPLEAF's rail projects the force it is handed onto the rail's direction
    in the launch frame and hands back a force along that direction (ENV/launchRail.py), and its
    rigid body takes the force handed back as one in body axes. On a rail that stands straight up
    the two frames agree. On one that leans, the body sits at the lean and the force, turned once
    more by the body's attitude, drives the rocket along a line at twice the lean, so it flies up
    the rail at an angle of attack equal to the lean. This flies it and reads the three angles
    halfway along the rail, from MAPLEAF's own logs.
    """
    inclination_deg = case["rail"]["example_inclination_deg"]
    steps, evaluations = run_mapleaf(case, TIME_STEPS_S[0], folder, inclination_deg)
    start = [steps[0][k] for k in ("PositionX(m)", "PositionY(m)", "PositionZ(m)")]
    half = 0.5 * case["rail"]["guided_travel_m"]
    row = next(
        row
        for row in steps
        if math.dist([row[k] for k in ("PositionX(m)", "PositionY(m)", "PositionZ(m)")], start) >= half
    )
    velocity = [row[k] for k in ("VelocityX(m/s)", "VelocityY(m/s)", "VelocityZ(m/s)")]
    speed = math.sqrt(sum(v * v for v in velocity))
    # The body's axis in the launch frame is the attitude quaternion's turn of (0, 0, 1), whose
    # upward part is 1 − 2 (q1² + q2²).
    axis_up = 1.0 - 2.0 * (row["OrientationQuaternion1"] ** 2 + row["OrientationQuaternion2"] ** 2)
    evaluation = next(e for e in evaluations if e["Time(s)"] >= row["Time(s)"])
    return {
        "case": case["name"],
        "rail_from_vertical_deg": 90.0 - inclination_deg,
        "travel_from_vertical_deg": math.degrees(math.acos(velocity[2] / speed)),
        "body_axis_from_vertical_deg": math.degrees(math.acos(axis_up)),
        "angle_of_attack_deg": evaluation["AOA(deg)"],
        "at_travel_m": half,
    }


def fly(case, time_step_s, folder, same_drag=False):
    """Runs MAPLEAF on one case at one step and returns what its logs hold."""
    steps, evaluations = run_mapleaf(case, time_step_s, folder, same_drag=same_drag)

    def position(row):
        return [row["PositionX(m)"], row["PositionY(m)"], row["PositionZ(m)"]]

    def speed(row):
        return math.sqrt(sum(row[k] ** 2 for k in ("VelocityX(m/s)", "VelocityY(m/s)", "VelocityZ(m/s)")))

    start = position(steps[0])
    top = max(steps, key=lambda row: row["PositionZ(m)"])

    # MAPLEAF's position is its center of mass's, which moves forward in the body as the
    # propellant burns, and its velocity is the body's: the log's speed is short of the
    # position's own rate by the rate the center moves in the body. Its rail lets go when the
    # center of mass has traveled the rail's length (ENV/launchRail.py), so the airframe has then
    # traveled that much less. HPR Sim's rail lets go on the airframe's travel. To read the same
    # thing in both, the rail-exit speed here is the body's speed where the body has traveled the
    # guided length, on a straight line between the two steps either side; MAPLEAF's own release
    # is recorded beside it.
    stations = sorted((row["Time(s)"], -row["CGZ(m)"]) for row in evaluations)
    station_times = [time for time, _ in stations]

    def cg_station(time_s):
        index = min(max(bisect.bisect_right(station_times, time_s), 1), len(stations) - 1)
        (t0, s0), (t1, s1) = stations[index - 1], stations[index]
        return s0 if t1 == t0 else s0 + (s1 - s0) * (time_s - t0) / (t1 - t0)

    def center_travel(row):
        return math.dist(position(row), start)

    def body_travel(row):
        return center_travel(row) - (stations[0][1] - cg_station(row["Time(s)"]))

    travel = case["rail"]["guided_travel_m"]

    def crossing(traveled):
        for before, after in zip(steps, steps[1:]):
            d0, d1 = traveled(before), traveled(after)
            if d0 < travel <= d1:
                w = (travel - d0) / (d1 - d0)
                time_s = before["Time(s)"] + w * (after["Time(s)"] - before["Time(s)"])
                return {
                    "time_s": time_s,
                    "speed_m_s": speed(before) + w * (speed(after) - speed(before)),
                    "body_travel_m": body_travel(before) + w * (body_travel(after) - body_travel(before)),
                }
        raise SystemExit(f"{case['name']}: MAPLEAF's rocket never traveled {travel} m")

    rail_exit = crossing(body_travel)
    own_release = crossing(center_travel)

    def mass_properties(row):
        return {
            "mass_kg": row["Mass(kg)"],
            "cg_station_m": -row["CGZ(m)"],
            "transverse_inertia_kg_m2": row["MOIX(kg*m^2)"],
            "axial_inertia_kg_m2": row["MOIZ(kg*m^2)"],
        }

    burn_out_s = motor_rows(case)[-1]["time_s"]
    ignition = evaluations[0]
    burnt = next(row for row in evaluations if row["Time(s)"] > burn_out_s)
    area = math.pi * case["reference_diameter_m"] ** 2 / 4.0

    def axial_coefficient(row, column="AeroFZ(N)"):
        # A force along the body axis, as a coefficient on the reference area: the log gives the
        # force in body axes, z from the tail to the nose.
        velocity = math.sqrt(sum(row[k] ** 2 for k in ("VelocityX(m/s)", "VelocityY(m/s)", "VelocityZ(m/s)")))
        return -row[column] / (0.5 * row["AirDensity(kg/m^3)"] * velocity**2 * area)

    # Each part's share, under the id the inputs give it. MAPLEAF adds a boat tail of no length
    # behind a rocket that ends in a tube, to carry the base drag: it is counted with the last
    # body part, as HPR Sim's base drag is.
    whole = axial_coefficient(burnt)
    if same_drag:
        # One table is the whole drag: no parts, and the force must be the table's.
        parts = []
        drag = case["drag"]
        index = min(max(bisect.bisect_right(drag["mach"], burnt["Mach"]), 1), len(drag["mach"]) - 1)
        (m0, m1), (c0, c1) = drag["mach"][index - 1 : index + 1], drag["drag_coefficient"][index - 1 : index + 1]
        table = c0 + (c1 - c0) * (burnt["Mach"] - m0) / (m1 - m0)
        if abs(whole - table) > 1e-6 * table:
            raise SystemExit(
                f"{case['name']}: MAPLEAF's drag coefficient at Mach {burnt['Mach']!r} is {whole!r}, the table's {table!r}"
            )
    else:
        parts = [
            {"id": part["id"], "axial_coefficient": axial_coefficient(burnt, f"Sustainer.Part{index}FZ(N)")}
            for index, part in enumerate(case["body"])
        ]
        if "Auto-AddedZeroLengthBoatTailFZ(N)" in burnt:
            parts[-1]["axial_coefficient"] += axial_coefficient(burnt, "Auto-AddedZeroLengthBoatTailFZ(N)")
        parts.append({"id": case["fins"]["id"], "axial_coefficient": axial_coefficient(burnt, "Sustainer.FinsFZ(N)")})
        if abs(sum(part["axial_coefficient"] for part in parts) - whole) > 1e-9 * abs(whole):
            raise SystemExit(f"{case['name']}: MAPLEAF's parts do not sum to its whole axial force")

    return {
        "time_step_s": time_step_s,
        "steps": len(steps) - 1,
        "apogee_rise_m": top["PositionZ(m)"] - start[2],
        "apogee_time_s": top["Time(s)"],
        "rail_exit_speed_m_s": rail_exit["speed_m_s"],
        "rail_exit_time_s": rail_exit["time_s"],
        "own_release": own_release,
        "max_speed_m_s": max(speed(row) for row in steps),
        "max_mach": max(row["Mach"] for row in evaluations),
        "ignition": mass_properties(ignition),
        "burnout": mass_properties(burnt),
        "gravity_at_ignition_m_s2": -ignition["GravityFZ(N)"] / ignition["Mass(kg)"],
        "air_density_at_ignition_kg_m3": ignition["AirDensity(kg/m^3)"],
        "after_burnout": {
            "time_s": burnt["Time(s)"],
            "mach": burnt["Mach"],
            "axial_coefficient": whole,
            "parts": parts,
        },
    }


def gravity_matched(case, own_gravity_m_s2, folder):
    """The same-drag flight again with MAPLEAF's gravity at the pad made HPR Sim's.

    A measurement, not MAPLEAF as it ships: its flat Earth's gravity is `GM / (z + 6371 km)²`
    (ENV/EarthModelling.py), the same at every latitude, where HPR Sim's is WGS 84 normal gravity
    at the site. Scaling `GM` by the ratio of the two at the pad leaves everything else as it
    was, so what this flight moves is what the gravity models' difference moves.
    """
    from MAPLEAF.ENV import FlatEarth

    scale = case["gravity_at_pad_m_s2"] / own_gravity_m_s2
    shipped = FlatEarth.GM
    FlatEarth.GM = shipped * scale
    try:
        flown = fly(case, TIME_STEPS_S[-1], folder, same_drag=True)
    finally:
        FlatEarth.GM = shipped
    return {
        "gravity_scale": scale,
        "gravity_at_ignition_m_s2": flown["gravity_at_ignition_m_s2"],
        "apogee_rise_m": flown["apogee_rise_m"],
        "rail_exit_speed_m_s": flown["rail_exit_speed_m_s"],
    }


def check_mass(case, flown):
    """Refuses a run whose logged mass properties are not the inputs'."""
    length = case["length_m"]
    for when in ("ignition", "burnout"):
        given, logged = case[when], flown[when]
        for key in ("mass_kg", "transverse_inertia_kg_m2", "axial_inertia_kg_m2"):
            if abs(logged[key] - given[key]) > MASS_TOLERANCE * abs(given[key]):
                raise SystemExit(
                    f"{case['name']}: MAPLEAF's {key} at {when} is {logged[key]!r}, the inputs' {given[key]!r}"
                )
        if abs(logged["cg_station_m"] - given["cg_station_m"]) > MASS_TOLERANCE * length:
            raise SystemExit(
                f"{case['name']}: MAPLEAF's center of mass at {when} is {logged['cg_station_m']!r} m "
                f"aft of the tip, the inputs' {given['cg_station_m']!r}"
            )


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("--inputs", default=str(ROOT / INPUTS))
    parser.add_argument("--out", default=str(ROOT / OUTPUT))
    parser.add_argument("--check", action="store_true", help="compare with the committed file; write nothing")
    args = parser.parse_args()

    inputs_bytes = pathlib.Path(args.inputs).read_bytes()
    inputs = json.loads(inputs_bytes)
    cases = []
    for case in inputs["cases"]:
        flights = {}
        for drag, same_drag in (("own_drag", False), ("same_drag", True)):
            runs = []
            for time_step_s in TIME_STEPS_S:
                with tempfile.TemporaryDirectory() as folder:
                    flown = fly(case, time_step_s, folder, same_drag=same_drag)
                check_mass(case, flown)
                runs.append(flown)
            coarse, fine = runs
            flights[drag] = {
                "metrics": {
                    "apogee_rise_m": fine["apogee_rise_m"],
                    "rail_exit_speed_m_s": fine["rail_exit_speed_m_s"],
                },
                "coarser_step": {
                    "apogee_rise_m": coarse["apogee_rise_m"],
                    "rail_exit_speed_m_s": coarse["rail_exit_speed_m_s"],
                },
                "flown": fine,
            }
            print(
                f"{case['name']}, {drag.replace('_', ' ')}: apogee {fine['apogee_rise_m']:.2f} m above the start "
                f"({coarse['apogee_rise_m'] - fine['apogee_rise_m']:+.3f} m at the coarser step), rail exit "
                f"{fine['rail_exit_speed_m_s']:.3f} m/s ({coarse['rail_exit_speed_m_s'] - fine['rail_exit_speed_m_s']:+.4f})"
            )
        with tempfile.TemporaryDirectory() as folder:
            flights["gravity_matched"] = matched = gravity_matched(
                case, flights["same_drag"]["flown"]["gravity_at_ignition_m_s2"], folder
            )
        print(
            f"{case['name']}, same drag, gravity scaled by {matched['gravity_scale']:.6f}: apogee "
            f"{matched['apogee_rise_m']:.2f} m, rail exit {matched['rail_exit_speed_m_s']:.3f} m/s"
        )
        cases.append(
            {
                "name": case["name"],
                "design": case["design"],
                "configuration": case["configuration"],
                "guided_travel_m": case["rail"]["guided_travel_m"],
                "inclination_deg": case["rail"]["inclination_deg"],
                "heading_deg": case["rail"]["heading_deg"],
                "elevation_m": case["site"]["elevation_m"],
                "propellant_center_line_departure_m": center_line_departure_m(case),
            }
            | flights
        )

    # The leaning-rail measurement, on the case whose example leans its rail the most.
    leaning = max(inputs["cases"], key=lambda case: 90.0 - case["rail"]["example_inclination_deg"])
    with tempfile.TemporaryDirectory() as folder:
        leaning = leaning_rail(leaning, folder)
    print(
        f"{leaning['case']} on a rail {leaning['rail_from_vertical_deg']:.1f} deg from the vertical: MAPLEAF travels "
        f"{leaning['travel_from_vertical_deg']:.2f} deg from it, its body {leaning['body_axis_from_vertical_deg']:.2f} deg, "
        f"angle of attack {leaning['angle_of_attack_deg']:.2f} deg"
    )

    document = {
        "oracle": f"MAPLEAF {importlib.metadata.version('MAPLEAF')} at commit {inputs['mapleaf_commit']}",
        "generator": GENERATOR,
        "command": COMMAND,
        "model": (
            "MAPLEAF's 6-DOF rigid body with its own aerodynamics (Barrowman's and Niskanen's "
            "methods as its Rocket package gives them), its US Standard Atmosphere table, its "
            "flat-Earth inverse-square gravity and its classical Runge-Kutta integrator at a "
            "fixed step"
        ),
        "overrides": (
            "every structural mass is zero but one point mass that carries the inputs' mass, "
            "center and inertia without propellant; the motor file is written from the inputs' "
            "rows; calm air; the rail stands straight up; no recovery, the run ends at apogee; "
            "in the same-drag flights the outside shape is left out and one TabulatedAeroForce "
            "carries the inputs' drag table"
        ),
        "inputs": {"file": INPUTS, "sha256": hashlib.sha256(inputs_bytes).hexdigest()},
        "python": {
            name: importlib.metadata.version(name)
            for name in ("numpy", "scipy", "pandas", "matplotlib", "Cython")
        }
        | {"python": ".".join(str(v) for v in sys.version_info[:3])},
        "time_steps_s": list(TIME_STEPS_S),
        "mapleaf_defaults_kept": MAPLEAF_DEFAULTS,
        "leaning_rail": leaning,
        "cases": cases,
    }
    text = json.dumps(document, indent=2) + "\n"
    out = pathlib.Path(args.out)
    if args.check:
        if out.read_text() != text:
            raise SystemExit(f"{OUTPUT} is not what this run writes: run {COMMAND} and commit it")
        print(f"{OUTPUT} reproduces")
    else:
        out.write_text(text)
        print(f"wrote {OUTPUT}")


if __name__ == "__main__":
    main()
