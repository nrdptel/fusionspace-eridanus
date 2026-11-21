"""What RocketPy's tip-off phase moves on its own example rockets (M1.14b5, ADR-357 decision 4).

HPR Sim keeps a rocket guided until its last guide leaves the rail and then frees it with no
angular velocity (`docs/physics/flight.md`, The rail). A real rocket pivots about its last guide
once the forward one is off: tip-off. RocketPy gained that as an opt-in phase in its pull request
#920 (merged into `develop` 2026-09-14) and rewrote it in #1217 (merged 2026-11-17, in no
release). Since #1217 it has no switch: it runs in every flight that uses the standard 6-DOF
equations and starts with both buttons on the rail. After the upper button leaves the rail the rocket slides
on along the rail line, free to pitch and yaw about the lower button with roll held, until the
lower button leaves too. Without the phase, as in every release so far, RocketPy frees the
rocket where the upper button leaves.

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
  `validation/refs.lock.toml` at the merge commit of #1217, put ahead of the installed 1.13.0 on
  the import path. The script stops unless `rocketpy` loads from that checkout at that commit.
- **The equations** are that commit's, untouched. It carries pull request #1196's sign
  correction, which every whole flight of this suite flies (`corrections.py`, ADR-026; without
  it RocketPy takes its moments during the burn about the wrong point, and tip-off happens
  during the burn), and #1188. The script stops unless the commit's `u_dot_generalized` has
  #1196's three edits (`corrections.upstream_carries_1196`).
- **The phase off** is the commit's own branch for a rocket with no pair of buttons on the rail.
  The commit has no switch for the phase, so `PhaseOff` reports the lower button's rail length
  as the upper one's; the commit then frees the rocket where the upper button leaves, as the
  releases do.
- **The solver's step** is bounded (`flight.SOLVER`, `max_time_step`). #1217's description
  names two defects it leaves open. One is in the rail-button forces RocketPy reports, which
  this script does not read. The other is a rocket whose thrust starts below its weight never
  leaving the rail unless `max_time_step` is set: `flight.py` measured that on its own whole
  flights and bounds the step for it. `fly_one` fails a flight that does not end, does not
  climb, or whose phase did not run.
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

Where the lower button leaves the rail: with the phase on, RocketPy's own `tip_off_end_time`.
With it off the rocket is already free there, and the instant is found where the distance
travelled from the start reaches the lower button's rail length, `effective_2rl` as the commit
computes it, by bisection on the flight's own solution.

One rocket and wind fly with a changed recovery. NDRT 2020's rail is vertical, and in still air
the rocket falls tail-first through its apogee parachute's 1 s lag; RocketPy's integration of
that fall had not finished after `LAG_PROBE_WALL_S` seconds of wall time. Those three flights
open that parachute at apogee with no lag, and the output says so (`declared`). `--lag-probe`
is that measurement: it flies that rocket in still air with its lag, on the corrected
equations, and prints whether the flight finished and the time its integration had reached.

`cargo xtask tip-off` runs it, and the probe, and writes
`validation/reports/tip-off.json` and `.md`;
`cargo xtask tip-off --check`, in the gate's `refs` step, holds them. By hand, from the
repository root with the oracle environment, it prints one run:

    refs/venv/bin/python validation/oracles/rocketpy/tip_off.py [case ...]
    refs/venv/bin/python validation/oracles/rocketpy/tip_off.py --lag-probe

RocketPy is MIT-licensed (`THIRD-PARTY-NOTICES.md`).
"""

import faulthandler
import inspect
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
from rocketpy import Flight, NoseCone, Rocket  # noqa: E402

import corrections  # noqa: E402
import flight  # noqa: E402
import recovery  # noqa: E402

COMMAND = "refs/venv/bin/python validation/oracles/rocketpy/tip_off.py"
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
    if not corrections.upstream_carries_1196(inspect.getsource(Flight.u_dot_generalized)):
        fail("the pinned commit's u_dot_generalized does not carry pull request #1196's edits")
    return commit


class PhaseOff(Flight):
    """The pinned commit's flight without its tip-off phase.

    The commit has no switch: it runs the phase whenever `effective_2rl > effective_1rl > 0`,
    in a 6-DOF flight on the standard equations (`__handle_out_of_rail_event`). Here the lower button's rail length reads as the upper one's, so the commit takes its own
    branch for a rocket without a pair of buttons on the rail: free flight from where the upper
    button leaves, which is what every release does. `lower_button_rail_length_m` keeps the
    commit's own figure for the lower button."""

    @property
    def effective_2rl(self):
        return self.effective_1rl

    @property
    def lower_button_rail_length_m(self):
        return Flight.effective_2rl.func(self)


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
    off, where the distance travelled from the start reaches the lower button's rail length, by
    bisection on the flight's own solution."""
    if phase_on:
        return flown.tip_off_end_time
    elevation = flown.env.elevation
    target = flown.lower_button_rail_length_m

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
    # The body's other two axes there: the rotation matrix's first and second columns.
    first = (
        1 - 2 * (e2**2 + e3**2),
        2 * (e1 * e2 + e0 * e3),
        2 * (e1 * e3 - e0 * e2),
    )
    second = (
        2 * (e1 * e2 - e0 * e3),
        1 - 2 * (e1**2 + e3**2),
        2 * (e2 * e3 + e0 * e1),
    )
    along, up, right = rail_axes(rail)
    w1, w2 = flown.w1.get_value_opt(t), flown.w2.get_value_opt(t)
    # How fast the nose's direction moves: the body rate crossed with the long axis, which the
    # roll rate does not enter, `w2 first - w1 second`, in the launch frame, rad/s.
    turning = tuple(w2 * a - w1 * b for a, b in zip(first, second))
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
        # Its two parts, as the angle's: how fast the nose swings below the rail's line in the
        # rail's vertical plane, and to the right of the heading (M1.14b9 compares the rate as
        # a vector of these two).
        "pitch_rate_nose_down_deg_s": finite(
            math.degrees(-dot(turning, up)), "nose-down pitch rate"
        ),
        "pitch_rate_nose_right_deg_s": finite(
            math.degrees(dot(turning, right)), "nose-right pitch rate"
        ),
    }


def fly_one(inputs, site, rail, wind_kind, phase_on, guided):
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
    flown = (Flight if phase_on else PhaseOff)(
        rocket=rocket,
        environment=env,
        rail_length=rail["rail_length_m"] + (spacing if guided else 0.0),
        inclination=rail["inclination_deg"],
        heading=rail["heading_deg"],
        **flight.SOLVER,
    )
    faulthandler.cancel_dump_traceback_later()
    if flown.t_final >= flight.SOLVER["max_time"]:
        fail(f"{name}: the flight did not end before the time cap")
    if flown.apogee <= site["elevation"]:
        fail(f"{name}: apogee {flown.apogee} is not above the site")
    lower = flown.effective_2rl if phase_on else flown.lower_button_rail_length_m
    if not lower > flown.effective_1rl > 0:
        fail(f"{name}: the lower button does not leave after the upper one")
    if abs((lower - flown.effective_1rl) - spacing) > 1e-9:
        fail(f"{name}: RocketPy's two rail lengths differ by other than the button spacing")
    if phase_on and not flown.tip_off_duration > 0:
        fail(f"{name}: the tip-off phase did not run")
    if not phase_on and flown.tip_off_duration != 0:
        fail(f"{name}: the tip-off phase ran in a flight meant to fly without it")
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
    return result, flown_with(inputs, rocket)


# The Mach numbers RocketPy's normal force is recorded at: to the top of the declared drag
# table, every 0.02. A straight line between two follows RocketPy's own curve but at Mach 1.1,
# where its compressibility factor steps (Calisto's slope by 4.1%) and the line spreads the
# step over 0.02. The last guide leaves the rail at Mach 0.06 to 0.16, far below it.
NORMAL_FORCE_MACH_STEP = 0.02
NORMAL_FORCE_MACH_TOP = 3.0
# The angles of attack, rad, at which each surface's lift is checked to be its slope times the
# angle.
NORMAL_FORCE_CHECK_ANGLES = (0.05, 0.2, 0.45)


def normal_force_of(inputs, rocket):
    """RocketPy's normal force on the rocket, so another code can fly the same one.

    `Flight.u_dot_generalized` gives each aerodynamic surface a force across the rocket's axis,
    `q A_i cl_i(alpha, Mach)`, at the surface's center of pressure, with `alpha` the angle
    between the axis and the air it meets and `A_i` the surface's own reference area: the
    cross-section of the body it sits on (`AeroSurface.compute_forces_and_moments`). Every
    surface's `cl` is its slope times the angle, which this checks. So on the rocket's
    reference area `A` the whole rocket's normal-force coefficient is
    `alpha * sum(slope_i(Mach) A_i / A)`, at the station where those shares balance. Recorded
    against Mach number: that summed slope, per radian, and that station in meters aft of the
    nose tip. Both are checked against RocketPy's own totals (`Rocket.total_lift_coeff_der`,
    `Rocket.cp_position`).
    """
    cdm = rocket.center_of_dry_mass_position
    csys = rocket._csys
    tips = [
        position.z
        for surface, position, *_ in rocket.aerodynamic_surfaces
        if isinstance(surface, NoseCone)
    ]
    if len(tips) != 1:
        fail(f"{inputs['name']} has {len(tips)} nose cones")
    # The nose tip from the center of dry mass, toward the nose.
    tip = (tips[0] - cdm) * csys
    count = round(NORMAL_FORCE_MACH_TOP / NORMAL_FORCE_MACH_STEP)
    machs = [round(index * NORMAL_FORCE_MACH_STEP, 10) for index in range(count + 1)]
    slopes, stations = [], []
    for mach in machs:
        total, moment = 0.0, 0.0
        for surface, *_ in rocket.aerodynamic_surfaces:
            slope = surface.clalpha.get_value_opt(mach) if not hasattr(
                surface, "clalpha_multiple_fins"
            ) else surface.clalpha_multiple_fins.get_value_opt(mach)
            for alpha in NORMAL_FORCE_CHECK_ANGLES:
                lift = surface.cl.get_value_opt(alpha, mach)
                if abs(lift - slope * alpha) > 1e-12 * max(abs(slope * alpha), 1.0):
                    fail(
                        f"{inputs['name']}: {surface.name}'s lift at {alpha} rad, Mach {mach} "
                        f"is {lift}, not its slope {slope} times the angle"
                    )
            # On the rocket's reference area, at the surface's center of pressure, which
            # `surfaces_cp_to_cdm` gives from the center of dry mass, toward the nose.
            share = slope * surface.reference_area / rocket.area
            station = tip - rocket.surfaces_cp_to_cdm[surface].z
            total += share
            moment += share * station
        if not total > 0:
            fail(f"{inputs['name']}: no normal force at Mach {mach}")
        own_total = rocket.total_lift_coeff_der.get_value_opt(mach)
        own_station = (tips[0] - rocket.cp_position.get_value_opt(mach)) * csys
        if abs(total - own_total) > 1e-9 * abs(own_total) or abs(
            moment / total - own_station
        ) > 1e-9 * max(abs(own_station), 1.0):
            fail(
                f"{inputs['name']}, Mach {mach}: the surfaces sum to a slope of {total} at "
                f"{moment / total} m, where RocketPy's own totals are {own_total} at "
                f"{own_station} m"
            )
        slopes.append(finite(total, "normal-force slope"))
        stations.append(finite(moment / total, "center of pressure"))
    return {
        "source": "the sum over rocket.aerodynamic_surfaces of each surface's lift slope on "
                  "its own reference area, and the mean of rocket.surfaces_cp_to_cdm those "
                  "shares weight, as Flight.u_dot_generalized applies them; each surface's cl "
                  "is checked to be its slope times the angle of attack, and the totals "
                  "against Rocket.total_lift_coeff_der and Rocket.cp_position",
        "law": "C_N = slope_per_rad(Mach) * alpha, alpha in radians, on the reference radius, "
               "acting at cp_station_m(Mach) aft of the nose tip",
        "mach": machs,
        "slope_per_rad": slopes,
        "cp_station_m": stations,
    }


def flown_with(inputs, rocket):
    """What a flight carried besides its rail, site and wind, recorded so a code that flies the
    same flight reads it here and not from its own design (L75): the parachutes in the order
    they open (none: the rocket comes down ballistic), the dry mass, the area the drag acts on,
    and the motor."""
    by_name = {parachute.name: parachute for parachute in rocket.parachutes}
    chutes = inputs["geometry"]["parachutes"]
    return {
        "devices": [
            {
                "name": chute["name"],
                "cd_s_m2": finite(by_name[chute["name"]].cd_s, "cd_s"),
                "trigger": chute["trigger"],
                "lag_s": finite(by_name[chute["name"]].lag, "lag"),
                "sampling_rate_hz": finite(by_name[chute["name"]].sampling_rate, "sampling rate"),
            }
            for chute in (recovery.devices_of(inputs) if chutes else [])
        ],
        "dry_mass_kg": finite(rocket.dry_mass, "dry mass"),
        "reference_radius_m": finite(rocket.radius, "reference radius"),
        "normal_force": normal_force_of(inputs, rocket),
        "motor": {
            "total_impulse_ns": finite(rocket.motor.total_impulse, "total impulse"),
            "burn_out_time_s": finite(rocket.motor.burn_out_time, "burn-out time"),
            "propellant_initial_mass_kg": finite(
                rocket.motor.propellant_initial_mass, "propellant mass"
            ),
        },
    }


def lag_probe(document):
    """The reason for `NO_LAG`, measured: the rocket and wind that fly without the lag (one,
    today) are flown with it, the phase off, for `LAG_PROBE_WALL_S` seconds of wall time.
    Prints whether the flight finished and the latest time its integration had reached, and
    exits."""
    reached = [0.0]

    class Probe(PhaseOff):
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
            # To a tenth of a second, which is all the report keeps of it.
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
    keep = set(arguments)
    commit = pinned_commit()
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
            off, carried = fly_one(inputs, site, rail, wind_kind, False, False)
            on, carried_on = fly_one(inputs, site, rail, wind_kind, True, False)
            guided, carried_guided = fly_one(inputs, site, rail, wind_kind, False, True)
            if not carried == carried_on == carried_guided:
                fail(f"{name}, {wind_kind}: the three flights did not carry the same rocket")
            wind_u, wind_v = wind_of(wind_kind, rail["heading_deg"])
            winds.append({
                "wind": wind_kind,
                **({"declared": NO_LAG} if no_lag(rail, wind_kind) else {}),
                "wind_east_m_s": wind_u,
                "wind_north_m_s": wind_v,
                "flown_with": carried,
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
            "oracle": f"rocketpy develop at {commit} (pull request #1217's merge)",
            "generator": "validation/oracles/rocketpy/tip_off.py",
            "command": COMMAND,
            "evaluation": "What RocketPy's tip-off phase (udot_rail2) moves: each example "
                          "rocket flown with the phase off and on, in still air and in a 20 mph "
                          "crosswind; moved is on less off, and moved_from_guided is on less a "
                          "flight with the phase off on a rail longer by the button spacing, "
                          "which frees the rocket, unturned, where the lower button leaves",
            "equations": "as the pinned commit has them: it carries pull request #1196's "
                         "correction to the sign of the center-of-mass vector, which the "
                         "reference's script (tip_off.py) checks on the commit's source before "
                         "it flies, and #1188's to the nozzle's jet-damping lever",
            "phase_off": "the commit's own branch for a rocket without a pair of buttons on "
                         "the rail, reached by reading the lower button's rail length as equal to "
                         "the upper one's: free flight from where the upper button leaves",
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
