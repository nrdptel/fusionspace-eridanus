# FusionSpace HPR for Python

The `fusionspace-hpr` package, imported as `fusionspace.hpr`:
[FusionSpace HPR · Sim](https://hpr.fusionspace.co/), a flight simulator for hobby and
high-power rockets, from Python. It wraps the Rust library's builder: an `Environment`, a `Motor`,
a `Rocket` built part by part or read from a design file, and a `Flight` with its metrics and its
recording as NumPy arrays; and a `MonteCarlo` run, the flight flown many times with its inputs
scattered, its flights as NumPy arrays.

```python
from fusionspace import hpr

environment = hpr.Environment(32.99, -106.97, 1400.0)
rocket = hpr.Rocket("Small", 0.0563)
rocket.add_nose("ogive", 0.22, "abs", wall_m=0.0015)
rocket.add_tube(0.9, 0.00115, "kraft_phenolic")
rocket.add_fins(
    3,
    root_chord_m=0.1,
    tip_chord_m=0.04,
    span_m=0.045,
    sweep_m=0.05,
    thickness_m=0.003175,
    material="birch_plywood",
)
rocket.add_motor_tube(0.2, 0.029, 0.001, "kraft_phenolic")
rocket.add_mass(0.2, position="top", offset_m=0.07)
rocket.set_motor(hpr.Motor.from_catalog("H54", delay_s=10.0))
rocket.add_parachute("parachute", diameter_m=0.9, trigger="motor_delay")
flight = hpr.Flight(rocket, environment, 1.8)
print(flight.apogee_m, flight["height_above_ground_m"].max())
```

The guide's [Python page](https://hpr.fusionspace.co/python.html) walks through it, and
its [Accuracy page](https://hpr.fusionspace.co/accuracy.html) says how far to trust the
numbers. Licensed `(MIT OR Apache-2.0) AND Apache-2.0`: the project's MIT OR Apache-2.0,
and Apache-2.0 for OpenRocket's parts files it bundles (`THIRD-PARTY-NOTICES.md`).
