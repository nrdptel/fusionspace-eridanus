# FusionSpace HPR for Python

The `fusionspace-hpr` package, imported as `fusionspace.hpr`:
[FusionSpace HPR · Sim](https://hpr.fusionspace.co/), a flight simulator for hobby and
high-power rockets, from Python.

**Status: RELEASED.** Version 0.1.0, released October 8, 2026.

## What works today

It wraps the Rust library's builder: an `Environment`, a `Motor`, a `Rocket` built part by part or
read from a design file, and a `Flight`. A flight comes with its metrics, and its recording as
NumPy arrays. A `MonteCarlo` run is the flight flown many times with its inputs scattered, its
flights as NumPy arrays.

Every figure it gives is an estimate from a model, not a measurement, and never a go/no-go
verdict. Against 53 logged flights of fliers' own designs, the simulated apogee
missed by 12.95% on average (mean absolute), outside the 5% target. On average it read high by
8.73%. It can err either way: from about Mach 0.8 the drag reads high, so a fast rocket's apogee
reads low ([Accuracy](https://hpr.fusionspace.co/accuracy.html)). The motor's printed data and your
range safety officer (RSO) are authoritative. The 53 flights' weather is ERA5, generated using
Copernicus Climate Change Service information 2026, and contains modified Copernicus Climate
Change Service information 2026. Neither the European Commission nor ECMWF is responsible for any
use made of it.

## What doesn't work yet

The package covers the Rust library's builder, not the whole library. These, among others, don't
work from Python yet:

- staging: a design that drops a stage under power is refused
- swapping a motor into a design read from a file
- Monte Carlo's landing ellipses and spread summaries
- the built-in winds that change with height, and weather files (write any of them as a wind
  function)
- parts from a parts catalog
- type stubs

The [Python page](https://hpr.fusionspace.co/python.html#what-is-not-here-yet) lists them all.

## Install

```bash
pip install fusionspace-hpr
```

Then fly a first rocket:

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

## Where the details are

The guide's [Python page](https://hpr.fusionspace.co/python.html) walks through it.

## License

`FS-ACHERNAR · SW · TOOL 001` · version 0.1.0 · part of [FusionSpace](https://fusionspace.co).
The project's own code is MIT OR Apache-2.0. The package also bundles OpenRocket's
parts files, which are Apache-2.0, so its declared license is `(MIT OR Apache-2.0) AND
Apache-2.0`; see the
[third-party notices](https://github.com/nrdptel/fusionspace-eridanus/blob/main/THIRD-PARTY-NOTICES.md).
