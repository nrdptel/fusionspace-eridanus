"""Writes the invented samples beside this script: one log for each reason the readings withhold a
reading for that the survey's corpus of real logs has no case of, so that `cargo xtask logs` lists a
case for every reason and a test reads each sample to its reason with no private data (M7.2b).

Nothing here is a real flight. Each is a table a flyer's own logger might write, with a barometric
altitude above the ground, except the bad record, which no reader can make and so is the record
itself as JSON:

- bad-record.record.json: a record whose primary clock repeats a time.
- faster-than-free-fall.csv: a track that falls at 15 m/s for 3 s after apogee, then at 300 m/s to
  the ground, sooner than a fall from rest at apogee could.
- starts-off-the-pad.csv: an altitude above sea level in a column that says it is above the ground.
- step-too-coarse.csv: an altitude written in 20 m steps, coarser than the 10 m checked.

Run it with any Python 3: `python3 make.py`.
"""

import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
G = 9.80665
BOOST_M_S2 = 50.0
BURN_S = 1.5
DESCENT_M_S = 15.0
LIFTOFF_S = 3.0
PAD_MSL_M = 1190.0


def height(t):
    """Height above the pad, m, at `t` s from power on: a boost, a coast with no drag, a descent."""
    s = t - LIFTOFF_S
    if s <= 0:
        return 0.0
    if s <= BURN_S:
        return 0.5 * BOOST_M_S2 * s * s
    burnout_m = 0.5 * BOOST_M_S2 * BURN_S * BURN_S
    burnout_m_s = BOOST_M_S2 * BURN_S
    c = s - BURN_S
    coast_s = burnout_m_s / G
    if c <= coast_s:
        return burnout_m + burnout_m_s * c - 0.5 * G * c * c
    apogee_m = burnout_m + burnout_m_s * coast_s - 0.5 * G * coast_s * coast_s
    return max(apogee_m - DESCENT_M_S * (c - coast_s), 0.0)


def apogee_s():
    """The time of apogee, s from power on."""
    return LIFTOFF_S + BURN_S + BOOST_M_S2 * BURN_S / G


def plunge(t):
    """The height, m, with the descent at 300 m/s from 3 s after apogee."""
    slow_s = apogee_s() + 3.0
    if t <= slow_s:
        return height(t)
    return max(height(slow_s) - 300.0 * (t - slow_s), 0.0)


def table(name, rows):
    lines = ["Time (s),Baro Alt AGL (m)"] + [f"{t:.4f},{h:.2f}" for t, h in rows]
    (HERE / name).write_text("\n".join(lines) + "\n")


def times(dt, end_s):
    return [k * dt for k in range(int(round(end_s / dt)) + 1)]


table("faster-than-free-fall.csv", [(t, plunge(t)) for t in times(0.1, 30.0)])
table("starts-off-the-pad.csv", [(t, PAD_MSL_M + height(t)) for t in times(0.1, 60.0)])
table(
    "step-too-coarse.csv",
    [(t, 20.0 * round(height(t) / 20.0)) for t in times(0.1, 60.0)],
)

record = {
    "format": "generic_table",
    "logger": "invented",
    "serial_number": None,
    "firmware": None,
    "flight_number": None,
    "stated": {"apogee_m": None, "ground_elevation_msl_m": None, "samples": None},
    "time_s": [0.0, 0.05, 0.05, 0.1],
    "altitude_m": [0.0, 0.0, 0.0, 0.0],
    "vertical_speed_m_s": None,
    "temperature_k": None,
    "battery_v": None,
    "channels": [],
    "events": [],
    "notes": [],
}
(HERE / "bad-record.record.json").write_text(json.dumps(record, indent=2) + "\n")
