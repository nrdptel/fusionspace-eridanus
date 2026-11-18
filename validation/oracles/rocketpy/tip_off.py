"""What RocketPy's tip-off phase moves on its own example rockets (M1.14b5, ADR-357 decision 4).

HPR Sim keeps a rocket guided until its last guide leaves the rail and then frees it with no
angular velocity (`docs/physics/flight.md`, The rail). A real rocket pivots about its last guide
once the forward one is off: tip-off. RocketPy gained that as an opt-in phase in its pull request
#920 (`Flight(..., use_udot_rail2=True)`, merged into `develop` 2026-09-14, in no release): after
the upper button leaves the rail the rocket slides on along the rail line, free to pitch and yaw
about the lower button with roll held, until the lower button leaves too. With the phase off,
RocketPy frees the rocket where the upper button leaves.

This script flies every example rocket of `validation/fixtures/design/rocketpy-rocket-mass.json`
(the public designs under `validation/designs/rocketpy-*.json` are built from the same inputs)
with the phase off and on, in still air and in a 20 mph crosswind. It prints, per rocket and
wind, each flight's apogee, landing point and the state where the lower button leaves the rail,
and what the phase moved. It is a measurement of a cost, not a reference HPR Sim is scored
against.

A third flight in each wind stands for HPR Sim's own release: the phase off and the rail longer
by the distance between the buttons, so RocketPy's upper button leaves where the real rail's
lower one does and the rocket is guided, without turning, until then (`rail_release.py` measures
that release alone). The phase's move from that flight is what HPR Sim's release leaves out.

What is declared here, and why:

- **RocketPy's source** is the checkout `refs/rocketpy-tip-off`, pinned in
  `validation/refs.lock.toml` at the merge commit of #920, put ahead of the installed 1.13.0 on
  the import path. The script stops unless `rocketpy` loads from that checkout at that commit.
- **The equations** are that commit's with pull request #1196's sign correction, as every whole
  flight of this suite flies them (`corrections.py`, ADR-026): without it RocketPy takes its
  moments during the burn about the wrong point, and tip-off happens during the burn. The
  commit already carries #1188. `--as-pinned` flies the commit's equations untouched instead,
  to show the correction does not decide the result.
- **The drag** is the constant `flight.py` declares for same-drag flights, power on and off.
- **The motors** are the mass fixture's: each flies the bundled substitute thrust curve the
  fixture names, not the example's own file (`recovery.build_motor`).
- **The air** is RocketPy's standard atmosphere at the site (`recovery.environment_of`), and
  **the solver** runs at `flight.SOLVER`'s settings, which the output repeats.
- **The rail** is each example's own (`flight.py`'s table, and `RAILS` below for the examples it
  does not fly), and so is the site.
- **The wind** is declared: none, or 20 mph (8.9408 m/s) at every height, blowing square across
  the rail's heading, toward its right.
- **The parachutes** are each example's, with their noise zeroed. Three examples carry none in
  the mass fixture and come down ballistic; their landing points are recorded as such.

Where the lower button leaves the rail: with the phase on, RocketPy's own `between_rails_time`.
With it off the rocket is already free there, and the instant is found as RocketPy finds its own
rail events, where the distance travelled from the start reaches `effective_2rl`.

One rocket and wind fly with a changed recovery. NDRT 2020's rail is vertical, and in still air
the rocket falls tail-first through its apogee parachute's 1 s lag; RocketPy's integration of
that fall had not finished after `LAG_PROBE_WALL_S` seconds of wall time. Those three flights
open that parachute at apogee with no lag, and the output says so (`declared`). `--lag-probe`
is that measurement: it flies that rocket in still air with its lag, on the corrected
equations, and prints whether the flight finished and the time its integration had reached.

`cargo xtask tip-off` runs it both ways, and the probe, and writes
`validation/reports/tip-off.json` and `.md`;
`cargo xtask tip-off --check`, in the gate's `refs` step, holds them. By hand, from the
repository root with the oracle environment, it prints one run:

    refs/venv/bin/python validation/oracles/rocketpy/tip_off.py [--as-pinned] [case ...]
    refs/venv/bin/python validation/oracles/rocketpy/tip_off.py --lag-probe

RocketPy is MIT-licensed (`THIRD-PARTY-NOTICES.md`).
"""

import faulthandler
import json
import math
import os
import subprocess
import sys
import threading
import time
import tomllib
from pathlib import Path

CHECKOUT = Path("refs/rocketpy-tip-off")
LOCK = Path("validation/refs.lock.toml")
LOCK_NAME = "rocketpy-tip-off"

# This script's folder, for the oracle's own modules: `cargo xtask tip-off` runs it in Python's
# isolated mode, which leaves the folder off the path. Then the pinned checkout, ahead of the
# installed release, before anything imports `rocketpy`.
sys.path.insert(0, str(Path(__file__).resolve().parent))
sys.path.insert(0, str(CHECKOUT.resolve()))

import rocketpy  # noqa: E402
from rocketpy import Flight, Rocket  # noqa: E402

import corrections  # noqa: E402
import flight  # noqa: E402
import recovery  # noqa: E402

COMMAND = "refs/venv/bin/python validation/oracles/rocketpy/tip_off.py"
AS_PINNED_FLAG = "--as-pinned"
LAG_PROBE_FLAG = "--lag-probe"
# How long the probe lets the flight with its lag run, in seconds of wall time.
LAG_PROBE_WALL_S = 20

MPH_M_S = 0.44704
CROSSWIND_MPH = 20.0
CROSSWIND_M_S = CROSSWIND_MPH * MPH_M_S

# The rail and site of each example `flight.py` does not fly from the pad, cited in RocketPy's
# checkout at v1.13.0 (`refs/rocketpy`), where the mass fixture's inputs were read.
RAILS = {
    "calisto-getting-started-motor-at-minus-1.255": {
        "rail_length_m": 5.2,
        "inclination_deg": 85,
        "heading_deg": 0,
        "source": "docs/notebooks/getting_started.ipynb:791",
    },
    "cavour": {
        "rail_length_m": 12,
        "inclination_deg": 84,
        "heading_deg": 133,
        "source": "docs/examples/cavour_flight_sim.ipynb:315",
    },
    "genesis": {
        "rail_length_m": 12,
        "inclination_deg": 84,
        "heading_deg": 133,
        "source": "docs/examples/genesis_flight_sim.ipynb:327",
    },
    "lince": {
        "rail_length_m": 12,
        "inclination_deg": 84,
        "heading_deg": 133,
        "source": "docs/examples/lince_flight_sim.ipynb:435-437",
    },
    "astra": {
        "rail_length_m": 12,
        "inclination_deg": 84,
        "heading_deg": 133,
        "source": "docs/examples/astra_flight_sim.ipynb:317",
    },
    "andromeda": {
        "rail_length_m": 12,
        "inclination_deg": 84,
        "heading_deg": 133,
        "source": "docs/examples/andromeda_flight_sim.ipynb:389",
    },
    "erebus-11": {
        "rail_length_m": 12,
        "inclination_deg": 84,
        "heading_deg": 133,
        "source": "docs/examples/erebus_flight_sim.ipynb:321",
    },
}

SITES = {
    "calisto-getting-started-motor-at-minus-1.255": {
        "latitude": 32.990254,
        "longitude": -106.974998,
        "elevation": 1400,
        "site": "docs/notebooks/getting_started.ipynb:116",
    },
    "cavour": {
        "latitude": 39.388692,
        "longitude": -8.287814,
        "elevation": 150,
        "site": "docs/examples/cavour_flight_sim.ipynb:128-130",
    },
    "genesis": {
        "latitude": 39.38895,
        "longitude": -8.28837,
        "elevation": 160,
        "site": "docs/examples/genesis_flight_sim.ipynb:127-129",
    },
    "lince": {
        "latitude": 39.3897,
        "longitude": -8.288964,
        "elevation": 158,
        "site": "docs/examples/lince_flight_sim.ipynb:125-127",
    },
    "astra": {
        "latitude": 39.3897,
        "longitude": -8.2889639,
        "elevation": 160,
        "site": "docs/examples/astra_flight_sim.ipynb:126-128",
    },
    "andromeda": {
        "latitude": 39.388692,
        "longitude": -8.287814,
        "elevation": 160,
        "site": "docs/examples/andromeda_flight_sim.ipynb:126-128",
    },
    "erebus-11": {
        "latitude": 39.3897,
        "longitude": -8.2889639,
        "elevation": 160,
        "site": "docs/examples/erebus_flight_sim.ipynb:126-128",
    },
}

WINDS = ("still", "crosswind")

# A flight that has not finished in this long stops the script with a traceback.
WATCHDOG_S = 600

# The one declared change to an example's recovery. On a vertical rail in still air the rocket
# falls tail-first through the apogee parachute's lag, and RocketPy's integration of that fall
# had not finished after `LAG_PROBE_WALL_S` seconds of wall time (`lag_probe`). The flights of
# that rocket and wind (the phase off, on, and guided alike) open the apogee parachute with no
# lag.
NO_LAG = (
    "the parachute that opens at apogee opens with no lag: on a vertical rail in still air "
    "RocketPy's integration of the tail-first fall through the lag had not finished after "
    f"{LAG_PROBE_WALL_S} s of wall time"
)


def no_lag(rail, wind_kind):
    """Whether these flights open the apogee parachute without its lag (`NO_LAG`)."""
    return rail["inclination_deg"] == 90 and wind_kind == "still"


def fail(message):
    sys.exit(f"tip_off.py: {message}")


def finite(value, label):
    return flight.finite(value, label)


def pinned_commit():
    """The lock's commit for the checkout, after checking the checkout is at it and that
    `rocketpy` loaded from it."""
    lock = tomllib.loads(LOCK.read_text())
    pins = [entry for entry in lock["git"] if entry["name"] == LOCK_NAME]
    if len(pins) != 1 or pins[0]["dest"] != str(CHECKOUT):
        fail(f"{LOCK} has no single `{LOCK_NAME}` checkout at {CHECKOUT}")
    commit = pins[0]["commit"]
    if not CHECKOUT.is_dir():
        fail(f"{CHECKOUT} is missing: run `cargo xtask refs fetch {LOCK_NAME}`")
    head = subprocess.run(
        ["git", "-C", str(CHECKOUT), "rev-parse", "HEAD"],
        check=True, capture_output=True, text=True,
    ).stdout.strip()
    if head != commit:
        fail(f"{CHECKOUT} is at {head}, not the pinned {commit}")
    status = subprocess.run(
        ["git", "-C", str(CHECKOUT), "status", "--porcelain", "--", "rocketpy"],
        check=True, capture_output=True, text=True,
    ).stdout.strip()
    if status:
        fail(f"{CHECKOUT}/rocketpy differs from the pinned commit:\n{status}")
    loaded = Path(rocketpy.__file__).resolve()
    if CHECKOUT.resolve() not in loaded.parents:
        fail(f"rocketpy loaded from {loaded}, not from {CHECKOUT}")
    return commit


def site_and_rail(name, document):
    """The example's site and rail: `flight.py`'s where it flies the example, else the tables
    above."""
    known = {case["name"]: case for case in recovery.CASES + flight.WHOLE_FLIGHT_ONLY_CASES}
    if name in flight.RAILS:
        case = known[name]
        site = {key: case[key] for key in ("latitude", "longitude", "elevation", "site")}
        return site, dict(flight.RAILS[name])
    if name not in RAILS or name not in SITES:
        fail(f"{name} has no rail or site declared")
    return dict(SITES[name]), dict(RAILS[name])


def wind_of(kind, heading_deg):
    """The declared wind's east and north components, m/s: none, or 20 mph toward the right of
    the rail's heading."""
    if kind == "still":
        return 0.0, 0.0
    heading = math.radians(heading_deg)
    return CROSSWIND_M_S * math.cos(heading), -CROSSWIND_M_S * math.sin(heading)


def build_rocket(inputs):
    """The example rocket with the declared drag, as `flight.build_rocket` builds it, but an
    example with no parachute is flown ballistic instead of refused."""
    motor, _ = recovery.build_motor(inputs["motor"])
    rocket_inputs = inputs["rocket"]
    power_off_drag, power_on_drag = flight.drag_of(inputs["name"], False)
    rocket = Rocket(
        radius=rocket_inputs["radius"],
        mass=rocket_inputs["mass"],
        inertia=tuple(rocket_inputs["inertia"]),
        power_off_drag=power_off_drag,
        power_on_drag=power_on_drag,
        center_of_mass_without_motor=rocket_inputs["center_of_mass_without_motor"],
        coordinate_system_orientation=rocket_inputs["coordinate_system_orientation"],
    )
    rocket.add_motor(motor, position=rocket_inputs["motor_position"])
    recovery.add_geometry(rocket, inputs["geometry"])
    if len(rocket.rail_buttons) != 1:
        fail(f"{inputs['name']} has no pair of rail buttons")
    flight.zero_noise(rocket)
    return rocket


def rail_axes(rail):
    """The rail's unit vector (east, north, up), the unit vector square to it in its vertical
    plane, pointing up, and the horizontal unit vector to the right of its heading."""
    inclination = math.radians(rail["inclination_deg"])
    heading = math.radians(rail["heading_deg"])
    along = (
        math.cos(inclination) * math.sin(heading),
        math.cos(inclination) * math.cos(heading),
        math.sin(inclination),
    )
    up = (
        -math.sin(inclination) * math.sin(heading),
        -math.sin(inclination) * math.cos(heading),
        math.cos(inclination),
    )
    right = (math.cos(heading), -math.sin(heading), 0.0)
    return along, up, right


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def lower_button_time(flown, phase_on):
    """When the lower button leaves the rail: RocketPy's own event with the phase on; with it
    off, where the distance travelled from the start reaches `effective_2rl`, by bisection on the
    flight's own solution."""
    if phase_on:
        return flown.between_rails_time
    elevation = flown.env.elevation
    target = flown.effective_2rl

    def travelled(t):
        return math.sqrt(
            flown.x.get_value_opt(t) ** 2
            + flown.y.get_value_opt(t) ** 2
            + (flown.z.get_value_opt(t) - elevation) ** 2
        )

    low = flown.out_of_rail_time
    if travelled(low) > target * (1 + 1e-9):
        fail("the upper button left the rail after the lower one would have")
    high = low + 0.01
    while travelled(high) < target:
        high += 0.01
        if high > low + 5.0:
            fail("the lower button did not leave the rail within 5 s of the upper one")
    for _ in range(200):
        middle = 0.5 * (low + high)
        if travelled(middle) < target:
            low = middle
        else:
            high = middle
    return 0.5 * (low + high)


def state_at(flown, t, rail):
    """The rocket's attitude against the rail and its turning rate at `t`."""
    e0, e1, e2, e3 = (getattr(flown, name).get_value_opt(t) for name in ("e0", "e1", "e2", "e3"))
    # The body's long axis (toward the nose) in the launch frame: the third column of the
    # rotation matrix of the Euler parameters.
    axis = (
        2 * (e1 * e3 + e0 * e2),
        2 * (e2 * e3 - e0 * e1),
        1 - 2 * (e1**2 + e2**2),
    )
    along, up, right = rail_axes(rail)
    w1, w2 = flown.w1.get_value_opt(t), flown.w2.get_value_opt(t)
    return {
        "time_s": finite(t, "time"),
        "speed_m_s": finite(flown.speed.get_value_opt(t), "speed"),
        # The angle between the rocket's axis and the rail's.
        "off_rail_axis_deg": finite(
            math.degrees(math.acos(max(-1.0, min(1.0, dot(axis, along))))), "angle off the rail"
        ),
        # Its two parts: nose below the rail's line in the rail's vertical plane, and nose to
        # the right of the heading (downwind in the crosswind).
        "nose_down_deg": finite(math.degrees(math.asin(-dot(axis, up))), "nose-down angle"),
        "nose_right_deg": finite(math.degrees(math.asin(dot(axis, right))), "nose-right angle"),
        # The turning rate across the long axis: the pitch and yaw rates together.
        "pitch_rate_deg_s": finite(math.degrees(math.hypot(w1, w2)), "pitch rate"),
    }


def fly_one(inputs, site, rail, wind_kind, phase_on, guided, flight_class):
    """One flight: the tip-off phase on or off, or (`guided`) the phase off on a rail longer by
    the button spacing, which guides the rocket to where the real rail's lower button leaves."""
    if no_lag(rail, wind_kind):
        geometry = dict(inputs["geometry"])
        geometry["parachutes"] = [
            {**chute, "lag": 0.0} if chute["trigger"]["kind"] == "apogee" else chute
            for chute in geometry["parachutes"]
        ]
        if not any(chute["trigger"]["kind"] == "apogee" for chute in geometry["parachutes"]):
            fail(f"{inputs['name']}: no parachute opens at apogee to fly without its lag")
        inputs = {**inputs, "geometry": geometry}
    rocket = build_rocket(dict(inputs))
    wind_u, wind_v = wind_of(wind_kind, rail["heading_deg"])
    env = recovery.environment_of({**site, "wind_u": wind_u, "wind_v": wind_v})
    buttons = inputs["geometry"]["rail_buttons"]
    spacing = abs(buttons["upper_button_position"] - buttons["lower_button_position"])
    label = "guided to the lower button" if guided else f"phase {'on' if phase_on else 'off'}"
    name = f"{inputs['name']}, {wind_kind}, {label}"
    faulthandler.dump_traceback_later(WATCHDOG_S, exit=True)
    flown = flight_class(
        rocket=rocket,
        environment=env,
        rail_length=rail["rail_length_m"] + (spacing if guided else 0.0),
        inclination=rail["inclination_deg"],
        heading=rail["heading_deg"],
        use_udot_rail2=phase_on,
        **flight.SOLVER,
    )
    faulthandler.cancel_dump_traceback_later()
    if flown.t_final >= flight.SOLVER["max_time"]:
        fail(f"{name}: the flight did not end before the time cap")
    if flown.apogee <= site["elevation"]:
        fail(f"{name}: apogee {flown.apogee} is not above the site")
    if not flown.effective_2rl > flown.effective_1rl:
        fail(f"{name}: the lower button does not leave after the upper one")
    if abs((flown.effective_2rl - flown.effective_1rl) - spacing) > 1e-9:
        fail(f"{name}: RocketPy's two rail lengths differ by other than the button spacing")
    if phase_on and not flown.tip_off_duration > 0:
        fail(f"{name}: the tip-off phase did not run")
    # Guided, the rocket is freed where the real rail's lower button leaves: the longer rail's
    # own exit.
    last = flown.out_of_rail_time if guided else lower_button_time(flown, phase_on)
    result = {
        "apogee_agl_m": finite(flown.apogee - site["elevation"], "apogee"),
        "apogee_time_s": finite(flown.apogee_time, "time to apogee"),
        "apogee_east_m": finite(flown.apogee_x, "apogee east"),
        "apogee_north_m": finite(flown.apogee_y, "apogee north"),
    }
    result["landing_east_m"] = finite(flown.x_impact, "landing east")
    result["landing_north_m"] = finite(flown.y_impact, "landing north")
    result["flight_time_s"] = finite(flown.t_final, "flight time")
    result["upper_button_leaves_s"] = finite(flown.out_of_rail_time, "rail-exit time")
    result["lower_button_leaves"] = state_at(flown, last, rail)
    return result


def lag_probe(document):
    """The reason for `NO_LAG`, measured: the rocket and wind that fly without the lag (one,
    today) are flown with it, the phase off, for `LAG_PROBE_WALL_S` seconds of wall time.
    Prints whether the flight finished and the latest time its integration had reached, and
    exits."""
    reached = [0.0]

    class Probe(corrections.CorrectedFlight):
        def u_dot_generalized(self, t, u, post_processing=False):
            reached[0] = max(reached[0], float(t))
            return super().u_dot_generalized(t, u, post_processing)

    probes = []
    for name in [case["name"] for case in document["cases"]]:
        site, rail = site_and_rail(name, document)
        if not no_lag(rail, "still"):
            continue
        inputs = recovery.mass_case(document, name)
        inputs["name"] = name
        rocket = build_rocket(dict(inputs))
        env = recovery.environment_of({**site, "wind_u": 0.0, "wind_v": 0.0})
        reached[0] = 0.0
        probe = {"case": name, "wind": "still", "wall_s": LAG_PROBE_WALL_S}
        probes.append(probe)

        def stop():
            time.sleep(LAG_PROBE_WALL_S)
            probe["finished"] = False
            # To a tenth of a second: the integration creeps on, and the report must reproduce.
            probe["solver_time_s"] = round(reached[0], 1)
            json.dump({"lag_probe": probes}, sys.stdout, indent=2, sort_keys=True)
            sys.stdout.write("\n")
            sys.stdout.flush()
            os._exit(0)

        threading.Thread(target=stop, daemon=True).start()
        flown = Probe(
            rocket=rocket,
            environment=env,
            rail_length=rail["rail_length_m"],
            inclination=rail["inclination_deg"],
            heading=rail["heading_deg"],
            use_udot_rail2=False,
            **flight.SOLVER,
        )
        # Only one such rocket exists; a second would need the first's timer stopped.
        fail(f"{name}: the flight with its lag finished at {flown.t_final} s: fly it with the lag")
    if not probes:
        fail("no rocket flies without its parachute's lag: nothing to probe")


def between(a, b, east, north):
    return math.hypot(b[east] - a[east], b[north] - a[north])


def moved(base, on):
    """What the phase moved: the flight with the phase on less `base`."""
    a, b = base["lower_button_leaves"], on["lower_button_leaves"]
    result = {
        "apogee_m": on["apogee_agl_m"] - base["apogee_agl_m"],
        "apogee_percent": 100 * (on["apogee_agl_m"] - base["apogee_agl_m"]) / base["apogee_agl_m"],
        "apogee_point_m": between(base, on, "apogee_east_m", "apogee_north_m"),
    }
    landing = between(base, on, "landing_east_m", "landing_north_m")
    result["landing_point_m"] = landing
    result["landing_point_percent_of_apogee"] = 100 * landing / base["apogee_agl_m"]
    # How much farther from the pad the rocket lands: the sign says which way leaving the phase
    # out errs.
    result["landing_range_m"] = math.hypot(
        on["landing_east_m"], on["landing_north_m"]
    ) - math.hypot(base["landing_east_m"], base["landing_north_m"])
    result["off_rail_axis_deg"] = b["off_rail_axis_deg"] - a["off_rail_axis_deg"]
    result["nose_down_deg"] = b["nose_down_deg"] - a["nose_down_deg"]
    result["nose_right_deg"] = b["nose_right_deg"] - a["nose_right_deg"]
    result["pitch_rate_deg_s"] = b["pitch_rate_deg_s"] - a["pitch_rate_deg_s"]
    return result


def largest(cases, key):
    """The largest move of each kind over every rocket and wind, under `key`."""
    rows = [(case["name"], wind["wind"], wind[key]) for case in cases for wind in case["winds"]]
    by_apogee = max(rows, key=lambda row: abs(row[2]["apogee_percent"]))
    by_landing = max(rows, key=lambda row: row[2]["landing_point_percent_of_apogee"])
    by_rate = max(rows, key=lambda row: abs(row[2]["pitch_rate_deg_s"]))
    return {
        "apogee": {
            "case": by_apogee[0],
            "wind": by_apogee[1],
            "apogee_percent": by_apogee[2]["apogee_percent"],
            "apogee_m": by_apogee[2]["apogee_m"],
        },
        "landing_point": {
            "case": by_landing[0],
            "wind": by_landing[1],
            "landing_point_percent_of_apogee": by_landing[2]["landing_point_percent_of_apogee"],
            "landing_point_m": by_landing[2]["landing_point_m"],
        },
        "pitch_rate": {
            "case": by_rate[0],
            "wind": by_rate[1],
            "pitch_rate_deg_s": by_rate[2]["pitch_rate_deg_s"],
        },
        "flights_compared": len(rows),
    }


def main():
    arguments = sys.argv[1:]
    if arguments == [LAG_PROBE_FLAG]:
        pinned_commit()
        lag_probe(recovery.mass_fixture())
        return
    as_pinned = AS_PINNED_FLAG in arguments
    keep = {a for a in arguments if a != AS_PINNED_FLAG}
    commit = pinned_commit()
    flight_class = Flight if as_pinned else corrections.CorrectedFlight
    document = recovery.mass_fixture()
    names = [case["name"] for case in document["cases"]]
    if keep - set(names):
        fail(f"unknown case(s): {sorted(keep - set(names))}")
    cases = []
    for name in names:
        if keep and name not in keep:
            continue
        inputs = recovery.mass_case(document, name)
        inputs["name"] = name
        site, rail = site_and_rail(name, document)
        winds = []
        for wind_kind in WINDS:
            off = fly_one(inputs, site, rail, wind_kind, False, False, flight_class)
            on = fly_one(inputs, site, rail, wind_kind, True, False, flight_class)
            guided = fly_one(inputs, site, rail, wind_kind, False, True, flight_class)
            wind_u, wind_v = wind_of(wind_kind, rail["heading_deg"])
            winds.append({
                "wind": wind_kind,
                **({"declared": NO_LAG} if no_lag(rail, wind_kind) else {}),
                "wind_east_m_s": wind_u,
                "wind_north_m_s": wind_v,
                "phase_off": off,
                "phase_on": on,
                "guided_to_lower_button": guided,
                "moved": moved(off, on),
                "moved_from_guided": moved(guided, on),
            })
            print(f"tip_off.py: {name}, {wind_kind}", file=sys.stderr)
        buttons = inputs["geometry"]["rail_buttons"]
        cases.append({
            "name": name,
            "site": site,
            "rail": rail,
            "button_spacing_m": abs(
                buttons["upper_button_position"] - buttons["lower_button_position"]
            ),
            "parachutes": [chute["name"] for chute in inputs["geometry"]["parachutes"]],
            "winds": winds,
        })
    json.dump(
        {
            "oracle": f"rocketpy develop at {commit} (pull request #920's merge)",
            "generator": "validation/oracles/rocketpy/tip_off.py",
            "command": COMMAND + (f" {AS_PINNED_FLAG}" if as_pinned else ""),
            "evaluation": "What RocketPy's tip-off phase (use_udot_rail2) moves: each example "
                          "rocket flown with the phase off and on, in still air and in a 20 mph "
                          "crosswind; moved is on less off, and moved_from_guided is on less a "
                          "flight with the phase off on a rail longer by the button spacing, "
                          "which frees the rocket, unturned, where the lower button leaves",
            "equations": "as the pinned commit has them" if as_pinned else
                         "the pinned commit's with pull request #1196's sign correction "
                         "(corrections.py, ADR-026)",
            "corrections": [] if as_pinned else [corrections.CORRECTIONS[1]],
            "drag": {"declared_cd0": flight.DECLARED_CD0},
            "solver": flight.SOLVER,
            "crosswind": {
                "speed_mph": CROSSWIND_MPH,
                "speed_m_s": CROSSWIND_M_S,
                "direction": "square across the rail's heading, blowing toward its right, the "
                             "same at every height",
            },
            # The largest moves: the phase on less off, and on less the flight guided to the
            # lower button.
            "largest": largest(cases, "moved"),
            "largest_from_guided": largest(cases, "moved_from_guided"),
            "cases": cases,
        },
        sys.stdout,
        indent=2,
    )
    sys.stdout.write("\n")


if __name__ == "__main__":
    main()
