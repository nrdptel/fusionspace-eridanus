# HPR Sim

HPR Sim flies a hobby or high-power rocket's design file from the rail to the ground and says how
far to trust the numbers.

This is the first page of the simulator's guide, for a flier, a team or a developer deciding
whether HPR Sim fits their work. It says what HPR Sim is, who it's for, how far its numbers have
been from real flights, and how to install it. [A tour of HPR Sim](simulator-tour.md) comes next
and shows it working.

> **How far to trust it.** A test reads every number on this page from a committed report.
> HPR Sim's apogees have been set against 7 public flights of university rockets, where they missed
> by 6.06% on average without signs and by −0.22% with them, outside the project's own 5% target
> ([report][real-report]). Against 60 logged flights of fliers' own designs they missed by 12.07%
> without signs and averaged +7.76% above the logged ones. OpenRocket, flying the same files,
> missed by 11.26% without signs and averaged +6.72% above ([report][fixture-report]). Those
> comparisons check apogee alone. HPR Sim gives ranges, not verdicts. The safety code and the
> range safety officer (the RSO) decide whether a rocket flies.

## What it is

It's a flight simulator for rockets on commercial solid motors. It reads an OpenRocket `.ork`
design file as it is and flies it in [six degrees of freedom](glossary.md#6-dof-six-degrees-of-freedom).
Those are the rocket's position along three axes and its rotation about three. The mass and the balance
point move as the motor burns. It prints the apogee, the speeds, the stability margin and the
events from liftoff to landing. It also prints the known gaps in its models that bear on that
flight, each with the side it pushes the numbers to.

It comes three ways, all from one code base, and none needs a network to fly a rocket:

- The command line, `hpr`, flies a file and scatters its inputs to show the spread (Monte Carlo
  dispersion). It also looks up and converts motors, reads an altimeter's log and fetches a launch
  day's weather ([The command line](cli.md)).
- The Python package flies the same flights from a script or a notebook ([Python](python.md)).
- The Rust library puts the simulator inside a program of your own
  ([Getting started](getting-started.md)).

Every model cites a published source, and tests pin each one. The results against RocketPy,
OpenRocket and real flights are committed to the repository ([Accuracy](accuracy.md)). It's early:
the first release is out, and [Start here](start-here.md) lists what works today and what doesn't
yet.

## Who it is for

I built HPR Sim for three kinds of user:

- A flier with a design file who wants a second opinion on a flight before it flies, with the
  known gaps stated rather than hidden.
- A team that needs a flight's spread, not one number. `hpr mc` flies the file many times with its
  inputs scattered.
- A developer who wants a simulator inside a program, a test or a notebook. HPR Sim runs offline,
  and it gives the same numbers from the same inputs and seed on one machine.

It isn't for a rocket on a hybrid, liquid or research motor yet, because it doesn't fly those. It
doesn't draw a design either: it reads one made in OpenRocket. A design editor comes with the app.

## Install it

- For the command line, download the archive for your system from
  [the releases page](https://github.com/nrdptel/fusionspace-eridanus/releases) and put `hpr` on
  your path. With Rust installed, you can run `cargo install fusionspace-hpr-cli --locked`
  instead.
- For Python, run `pip install fusionspace-hpr`, then import it with `from fusionspace import hpr`.
- For the library, run `cargo add fusionspace-hpr` in your own Rust project.

[Install a release](getting-started.md#install-a-release) lists the systems each file serves, and
what to do on macOS when it can't check the program.

## Next

- [A tour of HPR Sim](simulator-tour.md) flies the guide rocket and reads what it prints. Then it
  scatters the inputs, flies the rocket again from Python, and shows how HPR Sim did on a public
  flight.
- [Compared with other simulators](simulator-compared.md) says what HPR Sim, OpenRocket, RocketPy
  and RASAero II each do, and what's been measured side by side. It also says where HPR Sim has
  less.
- The task pages are [Fly your .ork](fly-your-ork.md), [Pick a motor](pick-a-motor.md),
  [Check stability for a certification flight](stability-for-certification.md),
  [Monte Carlo dispersion](monte-carlo.md) and [Reading a flight log](reading-a-flight-log.md).
- Before you rely on a number, read [Trusting a simulation](trusting-a-simulation.md).

The trust note's flights come from two places. The public flights are among those RocketPy's
documentation flies against their teams' logs. University rocketry teams, rocketry courses and
hobbyists published the logged flights. Both sets fly in ERA5 weather: generated using Copernicus Climate Change Service information 2026, and
contains modified Copernicus Climate Change Service information 2026. Neither the European
Commission nor ECMWF is responsible for any use made of it. The
[logged flights' report][fixture-report] gives the full attribution and the ERA5 citations.

[real-report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/real-flights.md
[fixture-report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/fixture-flights.md
