"""Checks an installed `fusionspace-hpr` wheel or source package as a user would get it.

The distribution carries both licenses, THIRD-PARTY-NOTICES.md and THIRD-PARTY-LICENSES.txt
(ADR-114), its version is the one asked for, and the Python page's example rocket flies to the
apogee that page prints, within a meter.

Usage: python smoke_python.py <version>
"""

import importlib.metadata
import sys

from fusionspace import hpr

LICENSES = {"LICENSE-MIT", "LICENSE-APACHE", "THIRD-PARTY-NOTICES.md", "THIRD-PARTY-LICENSES.txt"}

distribution = importlib.metadata.distribution("fusionspace-hpr")
if distribution.version != sys.argv[1]:
    sys.exit(f"smoke_python: version {distribution.version}, not {sys.argv[1]}")
carried = {path.name for path in distribution.files or [] if "licenses" in path.parts}
if not LICENSES <= carried:
    sys.exit(f"smoke_python: the distribution lacks {sorted(LICENSES - carried)}")
texts = next(p for p in distribution.files if p.name == "THIRD-PARTY-LICENSES.txt")
if "\n- pyo3 " not in texts.read_text(encoding="utf-8"):
    sys.exit("smoke_python: THIRD-PARTY-LICENSES.txt doesn't list pyo3")

# docs/python.md's first example.
environment = hpr.Environment(32.99, -106.97, 1400.0, wind_speed_m_s=5.0, wind_from_deg=270.0)
rocket = hpr.Rocket("My 54 mm rocket", 0.0563)
rocket.add_nose("ogive", 0.22, "abs", wall_m=0.0015)
rocket.add_tube(0.9, 0.00115, "kraft_phenolic")
rocket.add_motor_tube(0.2, 0.029, 0.001, "kraft_phenolic", overhang_m=0.005)
rocket.add_fins(
    3,
    root_chord_m=0.1,
    tip_chord_m=0.04,
    span_m=0.045,
    sweep_m=0.05,
    thickness_m=0.003175,
    material="birch_plywood",
    cross_section="rounded",
)
rocket.add_mass(0.2, position="top", offset_m=0.07, name="recovery bay")
rocket.set_motor(hpr.Motor.from_catalog("H54", delay_s=10.0))
rocket.add_parachute("parachute", diameter_m=0.9, trigger="motor_delay")
flight = hpr.Flight(rocket, environment, 1.8, inclination_deg=85.0, heading_deg=270.0)
print(f"smoke_python: fusionspace-hpr {distribution.version} flies to {flight.apogee_m:.1f} m")
if abs(flight.apogee_m - 1118.3) > 1.0:
    sys.exit(f"smoke_python: apogee {flight.apogee_m} m, not 1118.3 m within 1 m")
