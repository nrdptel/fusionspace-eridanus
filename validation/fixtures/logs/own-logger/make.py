"""Writes the invented logs beside this script: one made-up flight, as a flyer's own logger would
record it, for the guide's page on reading your own logger and the tests that read it from the
library, the command line and Python (M7.1d2).

Nothing here is a real flight. The rocket boosts at 70 m/s^2 for 1.2 s, coasts with no drag to
apogee, falls at 20 m/s under a drogue to 150 m and at 6 m/s under its main to the ground. The
logger writes the same samples three ways:

- flight.csv: a table whose header states some units and not others: its altitude column, in
  feet, says nothing of its unit.
- flight.bin: the same samples as little-endian binary records, 16 bytes each.
- flight-capture.txt: those records as a serial console prints them, base64 between two markers.

and a GPS tracker beside it writes gps-track.csv, latitude, longitude and GPS altitude once a
second, with no barometer. Run it with any Python 3: `python3 make.py`. The mapping and layout
files beside them are written by hand, as a flyer would.
"""

import base64
import math
import struct
from pathlib import Path

HERE = Path(__file__).resolve().parent
G = 9.80665
BOOST_M_S2 = 70.0
BURN_S = 1.2
DROGUE_M_S = 20.0
MAIN_M_S = 6.0
MAIN_AT_M = 150.0
PAD_MSL_M = 1190.0
LIFTOFF_S = 1.0


def flight(t):
    """Height above the pad, m, and the axial accelerometer's reading, g, at `t` s from power on."""
    s = t - LIFTOFF_S
    if s <= 0:
        return 0.0, 1.0
    if s <= BURN_S:
        return 0.5 * BOOST_M_S2 * s * s, (BOOST_M_S2 + G) / G
    burnout_v = BOOST_M_S2 * BURN_S
    burnout_h = 0.5 * BOOST_M_S2 * BURN_S**2
    coast = s - BURN_S
    rise = burnout_v / G
    if coast <= rise:
        return burnout_h + burnout_v * coast - 0.5 * G * coast * coast, 0.0
    apogee = burnout_h + burnout_v**2 / (2 * G)
    # The fall to the drogue's speed is taken as instant: the logger's rate hides it.
    down = coast - rise
    h = apogee - DROGUE_M_S * down
    if h > MAIN_AT_M:
        return h, 1.0
    main_s = (apogee - MAIN_AT_M) / DROGUE_M_S
    h = MAIN_AT_M - MAIN_M_S * (down - main_s)
    return max(h, 0.0), 1.0


def pressure_pa(height_msl_m):
    """The 1976 standard atmosphere's pressure in its first layer, Pa."""
    return 101325.0 * (1 - 2.25577e-5 * height_msl_m) ** 5.25588


def main():
    rows = []
    t_ms = 0
    while t_ms <= 50_000:
        h, accel = flight(t_ms / 1000)
        rows.append((t_ms, h / 0.3048, pressure_pa(PAD_MSL_M + h), accel))
        t_ms += 200
    with open(HERE / "flight.csv", "w", newline="\n") as out:
        out.write("# An invented flight, written as a flyer's own logger would write it\n")
        out.write("Time (ms),Alt,Pressure (Pa),Accel X (g)\n")
        for t, ft, pa, g in rows:
            out.write(f"{t},{ft:.1f},{pa:.0f},{g:.2f}\n")
    records = b"".join(
        struct.pack("<Ifff", t, round(ft, 1), round(pa), round(g, 2)) for t, ft, pa, g in rows
    )
    (HERE / "flight.bin").write_bytes(records)
    lines = base64.b64encode(records).decode()
    with open(HERE / "flight-capture.txt", "w", newline="\n") as out:
        out.write("uart:~$ log dump\n")
        out.write("----++++//[[( flight start )]]\\\\++++----\n")
        for start in range(0, len(lines), 76):
            out.write(lines[start : start + 76] + "\n")
        out.write("----++++//[[( flight end )]]\\\\++++----\n")
        out.write("uart:~$\n")
    with open(HERE / "gps-track.csv", "w", newline="\n") as out:
        out.write("Time (s),Lat,Lon,GPS Alt (m),Sats\n")
        for second in range(0, 51):
            h, _ = flight(second)
            # The wind takes the rocket east at 3 m/s from apogee on, 10.8 s after power on: a
            # meter is 1 / (111,320 cos 40.88°), about 1.2e-5 degrees of longitude, there.
            drift_m = max(0.0, second - 10.8) * 3.0
            lat = 40.8800
            lon = -119.1200 + drift_m / (111_320 * math.cos(math.radians(lat)))
            out.write(f"{second},{lat:.6f},{lon:.6f},{PAD_MSL_M + h:.1f},{9 if second < 30 else 8}\n")


if __name__ == "__main__":
    main()
