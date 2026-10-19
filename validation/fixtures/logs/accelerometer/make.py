"""Writes the invented log beside this script: one made-up flight whose logger writes an
accelerometer along the rocket's axis, for the tests that read its top acceleration and burnout
from the command line and Python (M7.2c).

Nothing here is a real flight. A 2 kg rocket stands on the pad for 2 s, then a motor pushes
120 N, rising and falling by a tenth over its 1.3 s burn and tailing off to nothing over 0.15 s,
against a drag of 0.5 rho CdA v^2 with CdA 0.002 m^2 in air of 1.225 kg/m^3. It coasts to apogee,
falls at 20 m/s under a drogue to 150 m and at 6 m/s under its main to the ground. The logger
writes, 100 times a second, its time, its barometric altitude above the pad and the axial
accelerometer in g, gravity in: 1 g on the pad, the thrust less the drag over the mass in the
climb, and 1 g again under a canopy. Run it with any Python 3: `python3 make.py`.
"""

import math
from pathlib import Path

HERE = Path(__file__).resolve().parent
G = 9.80665
MASS_KG = 2.0
THRUST_N = 120.0
IGNITION_S = 2.0
BURN_S = 1.3
TAIL_S = 0.15
RISE_S = 0.05
CDA_M2 = 0.002
RHO = 1.225
DROGUE_M_S = 20.0
MAIN_M_S = 6.0
MAIN_AT_M = 150.0
STEP_S = 0.0005
LOG_DT = 0.01


def thrust(t):
    """The motor's thrust, N, at `t` s from power on."""
    s = t - IGNITION_S
    if s <= 0.0:
        return 0.0
    if s < RISE_S:
        return THRUST_N * s / RISE_S
    if s < BURN_S:
        return THRUST_N * (1.0 + 0.1 * math.sin(math.pi * s / BURN_S))
    if s < BURN_S + TAIL_S:
        return THRUST_N * (1.0 - (s - BURN_S) / TAIL_S)
    return 0.0


def climb():
    """The climb, integrated: each step's (time, height, specific force), to apogee."""
    t, h, v = 0.0, 0.0, 0.0
    rows = []
    flying = False
    while True:
        drag = 0.5 * RHO * CDA_M2 * v * abs(v)
        if not flying and thrust(t) > MASS_KG * G:
            flying = True
        if flying:
            force = (thrust(t) - drag) / MASS_KG
            a = force - G
        else:
            force, a = G, 0.0
        rows.append((t, h, force))
        if flying and v <= 0.0 and t > IGNITION_S + BURN_S:
            return rows
        v += a * STEP_S
        h += v * STEP_S
        t += STEP_S


def main():
    rows = climb()
    apogee_s, apogee_m, _ = rows[-1]
    main_s = apogee_s + (apogee_m - MAIN_AT_M) / DROGUE_M_S
    landed_s = main_s + MAIN_AT_M / MAIN_M_S
    lines = ["Time (s),Baro Alt AGL (m),Axial accel (g)"]
    for k in range(int(round((landed_s + 5.0) / LOG_DT)) + 1):
        t = k * LOG_DT
        if t <= apogee_s:
            i = min(int(round(t / STEP_S)), len(rows) - 1)
            _, h, force = rows[i]
        elif t <= main_s:
            h, force = apogee_m - DROGUE_M_S * (t - apogee_s), G
        else:
            h, force = max(MAIN_AT_M - MAIN_M_S * (t - main_s), 0.0), G
        lines.append(f"{t:.2f},{h:.2f},{force / G:.4f}")
    (HERE / "axial.csv").write_text("\n".join(lines) + "\n")


main()
