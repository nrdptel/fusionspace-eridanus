# HPR Sim's Python package: open a design, fly it and get its landing spread (M0.11, ADR-241).
# Run from the repository's root. Each statement is one script line, imports included; a
# `# task:` line names the tasks the lines after it reach. `cargo xtask scoreboard` counts them,
# and crates/hpr-py/tests/test_first_answer.py runs this file, so the count is of lines that work.

# task: open
from fusionspace import hpr
rocket = hpr.Rocket.from_file("validation/fixtures/ork/guides/level-1.ork", recovery=True)

# task: fly
calm = hpr.Environment(0.0, 0.0, 0.0)
flight = hpr.Flight(rocket, calm, 1.5)
print(f"apogee {flight.apogee_m:.1f} m")

# task: spread
import numpy
windy = hpr.Environment(0.0, 0.0, 0.0, wind_speed_m_s=4.0, wind_from_deg=270.0)
run = hpr.MonteCarlo(rocket, windy, 1.5, wind_sd_fraction=0.25, wind_from_sd_deg=15.0)
print(f"landing spread {numpy.nanstd(run['landing_distance_m'], ddof=1):.1f} m, one standard deviation")
