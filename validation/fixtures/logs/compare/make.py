"""Writes the invented log beside this script, for the guide's page on comparing a flight and the
test that runs its example (M7.3a).

Nothing here is a real flight. It takes HPR Sim's flight of the guide's Level 1 rocket in a 6 m/s
wind, reads its heights 4% low, as a logger on a warm day might, rounds them to a foot, and adds
10 s on the pad and 5 s on the ground. It is read by level-1.mapping.json. Compared with the same
rocket in calm air, it shows residuals with a known lean.

Run it from the repository's root with any Python 3, naming the binary: `python3
validation/fixtures/logs/compare/make.py target/debug/hpr`.
"""

import csv
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).parent
DESIGN = "validation/fixtures/ork/guides/level-1.ork"
FOOT_M = 0.3048
PAD_S = 10.0
GROUND_S = 5.0
INTERVAL_S = 0.1


def main() -> None:
    with tempfile.TemporaryDirectory() as folder:
        export = Path(folder) / "flown.csv"
        subprocess.run(
            [sys.argv[1], "sim", DESIGN, "--wind", "6", "--export", str(export), "--interval", "0.1"],
            check=True,
            capture_output=True,
        )
        with export.open() as file:
            rows = [
                (float(row["time [s]"]), float(row["height above ground [m]"]))
                for row in csv.DictReader(file)
            ]
    start = rows[0][1]
    last_s = rows[-1][0]
    lines = ["Time (s),Altitude (ft)"]

    def add(time_s: float, height_m: float) -> None:
        feet = round(0.96 * (height_m - start) / FOOT_M)
        lines.append(f"{time_s:.1f},{feet}")

    for step in range(round(PAD_S / INTERVAL_S)):
        add(step * INTERVAL_S, start)
    written = -1
    for time_s, height_m in rows:
        tenth = round(time_s / INTERVAL_S)
        if tenth != written:
            add(PAD_S + tenth * INTERVAL_S, height_m)
            written = tenth
    for step in range(1, round(GROUND_S / INTERVAL_S) + 1):
        add(PAD_S + round(last_s / INTERVAL_S) * INTERVAL_S + step * INTERVAL_S, rows[-1][1])
    (HERE / "level-1-flown.csv").write_text("\n".join(lines) + "\n")


if __name__ == "__main__":
    main()
