# HPR Sim

HPR Sim flies a hobby or high-power rocket's design file from the rail to the ground and says how
far to trust the numbers.

**This page is for** a flier, a team or a developer deciding whether HPR Sim fits their work: what
it is, who it is for, how far its numbers have been from real flights, and how to install it. It is
the first page of the simulator's guide; [A tour of HPR Sim](simulator-tour.md) comes next and
shows it working.

> **How far to trust it.** Every number on this page is read by a test from a committed report.
> HPR Sim's apogees have been set against 7 public flights of university rockets, where they missed
> by 6.04% on average without signs and by −0.25% with them, outside the project's own 5% target
> ([report][real-report]). Against 53 logged flights of fliers' own designs they missed by 12.95%
> without signs and averaged +8.73% above the logged ones; OpenRocket, flying the same files,
> missed by 12.02% without signs and averaged +7.79% above ([report][fixture-report]). Those
> comparisons check apogee alone. HPR Sim gives ranges, not verdicts; the safety code and the range
> safety officer (the RSO) decide whether a rocket flies.

## What it is

**A flight simulator for rockets on commercial solid motors.** It reads an OpenRocket `.ork`
design file as it is and flies it in [six degrees of freedom](glossary.md#6-dof-six-degrees-of-freedom),
the rocket's position along three axes and its rotation about three, while the motor burns and the mass and balance point move. It
prints the apogee, the speeds, the stability margin, the events from liftoff to landing and the
known gaps in its models that bear on that flight, each with the side it pushes the numbers to.

It comes three ways, all from one code base, and none needs a network to fly a rocket:

- **The command line, `hpr`:** fly a file, scatter its inputs to see the spread (Monte Carlo
  dispersion), look up and convert motors, read an altimeter's log, and fetch a launch day's
  weather ([The command line](cli.md)).
- **The Python package:** the same flights from a script or a notebook ([Python](python.md)).
- **The Rust library:** the simulator inside a program of your own
  ([Getting started](getting-started.md)).

Every model cites a published source, tests pin each one, and the results against RocketPy,
OpenRocket and real flights are committed to the repository ([Accuracy](accuracy.md)).
It is early: the first release is out, and [Start here](start-here.md) lists what works today
and what doesn't yet.

## Who it is for

- **A flier with a design file** who wants a second opinion on a flight before it flies, with the
  known gaps stated rather than hidden.
- **A team** that needs a flight's spread, not one number: `hpr mc` flies the file many times with
  its inputs scattered.
- **A developer** who wants a simulator inside a program, a test or a notebook, offline, with the
  same numbers from the same inputs and seed on one machine.

It is not yet for a rocket on a hybrid, liquid or research motor, which it doesn't fly, nor for
drawing a design: it reads one made in OpenRocket. A design editor comes with the app.

## Install it

- **The command line:** download the archive for your system from
  [the releases page](https://github.com/nrdptel/fusionspace-eridanus/releases) and put `hpr` on
  your path, or, with Rust installed, run `cargo install fusionspace-hpr-cli --locked`.
- **The Python package:** `pip install fusionspace-hpr`, then `from fusionspace import hpr`.
- **The library:** `cargo add fusionspace-hpr` in your own Rust project.

[Install a release](getting-started.md#install-a-release) lists the systems each file serves, and
what to do on macOS when it can't check the program.

## Next

- **[A tour of HPR Sim](simulator-tour.md):** fly the guide rocket, read what it prints,
  scatter its inputs, and see how HPR Sim did on a public flight.
- **The task pages:** [Fly your .ork](fly-your-ork.md),
  [Pick a motor](pick-a-motor.md),
  [Check stability for a certification flight](stability-for-certification.md),
  [Monte Carlo dispersion](monte-carlo.md) and [Reading a flight log](reading-a-flight-log.md).
- **Before you rely on a number:** [Trusting a simulation](trusting-a-simulation.md).

**Sources.** The public flights are among those RocketPy's documentation flies against their teams'
logs; the logged flights are published by university rocketry teams, rocketry courses and
hobbyists. Both fly in ERA5 weather: generated using Copernicus Climate Change Service
information 2026, and contains modified Copernicus Climate Change Service information 2026; neither
the European Commission nor ECMWF is responsible for any use made of it. The
[logged flights' report][fixture-report] gives the full attribution and the ERA5 citations.

[real-report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/real-flights.md
[fixture-report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/fixture-flights.md
