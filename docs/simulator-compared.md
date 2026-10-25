# Compared with other simulators

What HPR Sim can do beside OpenRocket, RocketPy and RASAero II, what has been measured side by
side, and where HPR Sim has less.

This page is for a flier or a team choosing a simulator. It's also for one who uses another
simulator and wants to know what HPR Sim would add and what it would take away. The three are the
free simulators that hobby and high-power fliers use most. Each one's answers are read from its
own documentation. It's the last page of the simulator's guide, after [HPR Sim](the-simulator.md)
and [A tour of HPR Sim](simulator-tour.md).

> **How far to trust it.** The table says whether each tool can do a thing, not how well it does
> it. Its answers come from each tool's own documentation, as read on the date given below it. A
> documentation page can lag its release. And a *no* means the documentation shows no way to do
> it, not that no plugin or workaround exists.
>
> Only the numbers under *Measured side by side* compare results, each read by a test from a
> committed report. RASAero II has flown none of them yet, and every accuracy comparison checks
> apogee alone. Nothing here says which simulator is right for a flight. HPR Sim gives ranges, not
> verdicts. The safety code and the range safety officer (the RSO) decide whether a rocket flies.

## What each does

A *yes* links the page that shows it. A *partly* or a *no* links the page that shows what the
tool does instead, and says it in words below the table. For HPR Sim, a *yes* links the page of
this site that covers it, and a *no* links the roadmap entry that brings it.

A *yes* says nothing of accuracy, least of all past the speed of sound. HPR Sim's own drag there
is rough and can be off either way ([Drag through the speed of sound](drag-through-mach-1.md)).
OpenRocket's documentation aims only at low supersonic speeds. And no tool's accuracy there has
been set against a logged flight here.

The rows also leave out much that a tool may have, such as plots, parts catalogs and recovery
design. Check a tool's own pages before you switch.

<!-- features: table; written by `cargo xtask guides` from validation/features/simulator.toml; do not edit -->

| Capability | Each tool |
|---|---|
| Opens an OpenRocket `.ork` design file | HPR Sim [yes](fly-your-ork.md); OpenRocket [yes](https://openrocket.readthedocs.io/en/latest/setup/getting_started.html#file); RocketPy [no](https://docs.rocketpy.org/en/latest/index.html); RASAero II [no](https://rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf#page=8) |
| Writes a design to an `.ork` file | HPR Sim [yes](cli.md#converting-a-design); OpenRocket [yes](https://openrocket.readthedocs.io/en/latest/setup/getting_started.html#file); RocketPy [no](https://docs.rocketpy.org/en/latest/index.html); RASAero II [no](https://rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf#page=8) |
| Draws and edits a design in a window | HPR Sim [no](decisions-and-roadmap.md#m9-1); OpenRocket [yes](https://openrocket.readthedocs.io/en/latest/setup/getting_started.html#the-user-interface); RocketPy [no](https://docs.rocketpy.org/en/latest/index.html); RASAero II [yes](https://rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf#page=4) |
| Flies in six degrees of freedom | HPR Sim [yes](how-a-flight-is-simulated.md); OpenRocket [yes](https://openrocket.readthedocs.io/en/latest/introduction/features.html); RocketPy [yes](https://docs.rocketpy.org/en/latest/index.html); RASAero II [no](https://rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf#page=4) |
| Flies a rocket of more than one stage | HPR Sim [yes](cli.md#separation); OpenRocket [yes](https://openrocket.readthedocs.io/en/latest/user_guide/advanced_rocket_design.html#conventional-staging); RocketPy [partly](https://docs.rocketpy.org/en/latest/user/flight.html#initial-conditions); RASAero II [yes](https://rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf#page=49) |
| Works out drag and stability past the speed of sound | HPR Sim [yes](drag-through-mach-1.md); OpenRocket [yes](https://github.com/openrocket/openrocket/releases/download/OpenRocket_technical_documentation-v13.05/OpenRocket_technical_documentation-v13.05.pdf#page=57); RocketPy [partly](https://docs.rocketpy.org/en/latest/reference/classes/Rocket.html); RASAero II [yes](https://rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf#page=4) |
| Flies a design many times with its inputs scattered, for a landing spread | HPR Sim [yes](monte-carlo.md); OpenRocket [no](https://github.com/openrocket/openrocket/releases/tag/release-24.12); RocketPy [yes](https://docs.rocketpy.org/en/latest/notebooks/monte_carlo_analysis/monte_carlo_class_usage.html); RASAero II [no](https://rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf#page=2) |
| Fetches a launch day's forecast, reanalysis or balloon sounding into the flight | HPR Sim [yes](weather.md); OpenRocket [partly](https://github.com/openrocket/openrocket/releases/tag/release-24.12); RocketPy [yes](https://docs.rocketpy.org/en/latest/user/environment/1-atm-models/forecast.html#global-forecast-system-gfs); RASAero II [no](https://rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf#page=78) |
| Flies a design file from a command line | HPR Sim [yes](cli.md#flying-and-scattering); OpenRocket [no](https://openrocket.readthedocs.io/en/latest/dev_guide/command_line_arguments.html); RocketPy [no](https://docs.rocketpy.org/en/latest/user/installation.html); RASAero II [no](https://www.rasaero.com/dl_software_ii.htm) |
| Flies a design from a Python script | HPR Sim [yes](python.md); OpenRocket [no](https://openrocket.readthedocs.io/en/latest/dev_guide/using_openrocket_core.html); RocketPy [yes](https://docs.rocketpy.org/en/latest/user/first_simulation.html); RASAero II [no](https://rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf#page=2) |
| Estimates the speed at which the fins flutter | HPR Sim [partly](when-fins-flutter.md); OpenRocket [no](https://openrocket.readthedocs.io/en/latest/introduction/features.html#planned-future-features); RocketPy [yes](https://docs.rocketpy.org/en/latest/reference/classes/utils/utilities.html#rocketpy.utilities.fin_flutter_analysis); RASAero II [no](https://rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf#page=2) |
| Reads an altimeter's flight log | HPR Sim [yes](reading-a-flight-log.md); OpenRocket [no](https://openrocket.readthedocs.io/en/latest/introduction/features.html#planned-future-features); RocketPy [yes](https://docs.rocketpy.org/en/latest/user/flight_comparator.html); RASAero II [no](https://rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf#page=82) |
| Flies airbrakes driven by a control law | HPR Sim [no](decisions-and-roadmap.md#m6-4); OpenRocket [no](https://openrocket.readthedocs.io/en/latest/user_guide/simulation_extensions.html#example-simulation-extensions-provided-with-openrocket); RocketPy [yes](https://docs.rocketpy.org/en/latest/user/airbrakes.html); RASAero II [no](https://rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf#page=2) |
| Flies a hybrid or liquid motor with its tanks | HPR Sim [no](decisions-and-roadmap.md#m11-3); OpenRocket [no](https://openrocket.readthedocs.io/en/latest/user_guide/thrust_curves.html); RocketPy [yes](https://docs.rocketpy.org/en/latest/user/motors/tanks.html); RASAero II [no](https://rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf#page=93) |
| Warns when a flight leaves its models' assumptions | HPR Sim [yes](fly-your-ork.md#fly-the-file-as-it-is); OpenRocket [yes](https://github.com/openrocket/openrocket/releases/download/OpenRocket_technical_documentation-v13.05/OpenRocket_technical_documentation-v13.05.pdf#page=87); RocketPy [no](https://docs.rocketpy.org/en/latest/reference/classes/exceptions.html); RASAero II [no](https://rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf#page=114) |
| Names the known gaps in its models that bear on each flight, and which way each pushes the numbers where that is known | HPR Sim [yes](fly-your-ork.md#fly-the-file-as-it-is); OpenRocket [no](https://github.com/openrocket/openrocket/releases/download/OpenRocket_technical_documentation-v13.05/OpenRocket_technical_documentation-v13.05.pdf#page=87); RocketPy [no](https://docs.rocketpy.org/en/latest/reference/classes/exceptions.html); RASAero II [no](https://rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf#page=114) |

**What none of the others has.** Of the capabilities above, as far as each tool's documentation shows, HPR Sim alone *flies a design file from a command line* and *names the known gaps in its models that bear on each flight, and which way each pushes the numbers where that is known*: each of the others' cells in those rows says no.

**Where HPR Sim has less.** Each row where another tool says yes and HPR Sim doesn't, with the step of the roadmap that brings it:

- Draws and edits a design in a window: OpenRocket and RASAero II do. HPR Sim doesn't yet; [Desktop app shell](decisions-and-roadmap.md#m9-1) on the roadmap brings it.
- Estimates the speed at which the fins flutter: RocketPy does. HPR Sim does in part; [Submission packs](decisions-and-roadmap.md#m6-6) on the roadmap brings it.
- Flies airbrakes driven by a control law: RocketPy does. HPR Sim doesn't yet; [Airbrakes](decisions-and-roadmap.md#m6-4) on the roadmap brings it.
- Flies a hybrid or liquid motor with its tanks: RocketPy does. HPR Sim doesn't yet; [Certified hybrids](decisions-and-roadmap.md#m11-3) on the roadmap brings it.

**Each cell's note.** Every partly and no says what the page its cell links to shows instead; a yes may carry a limit its documentation states.

- Opens an OpenRocket `.ork` design file:
  - RocketPy (no): it has no `.ork` reader; a design is typed in as Python.
  - RASAero II (no): it opens its own files and imports RockSim's; its manual names no OpenRocket format.
- Writes a design to an `.ork` file:
  - RocketPy (no): nothing in its guide or reference writes a design file for another program.
  - RASAero II (no): it saves its own format only.
- Draws and edits a design in a window:
  - HPR Sim (no): no window yet; it flies a design drawn in OpenRocket.
  - RocketPy (no): it is a Python library, with no window.
- Flies in six degrees of freedom:
  - RASAero II (no): it flies two degrees of freedom in calm air and three with weathercocking when a wind is entered.
- Flies a rocket of more than one stage:
  - RocketPy (partly): it has no stage object or separation event; a flight can start where another ended, with a rocket of its own for each stage.
- Works out drag and stability past the speed of sound:
  - OpenRocket (yes): its technical documentation aims at reasonable estimates at transonic and low supersonic speeds.
  - RocketPy (partly): its drag comes from curves the user supplies, and its center of pressure varies with Mach, but its reference names no method past the speed of sound.
  - RASAero II (yes): its manual gives drag, normal force and center of pressure from Mach 0.01 to Mach 25.
- Flies a design many times with its inputs scattered, for a landing spread:
  - OpenRocket (no): its newest release's notes add none, its features page lists it as planned, and the landing-spread page in its guide is on the development branch only.
  - RASAero II (no): neither its contents nor its flight chapter has a spread or a Monte Carlo.
- Fetches a launch day's forecast, reanalysis or balloon sounding into the flight:
  - OpenRocket (partly): its newest release adds wind by height, typed in or read from a file, but fetches nothing.
  - RASAero II (no): the launch site takes a temperature, a pressure and one wind speed, on the standard atmosphere.
- Flies a design file from a command line:
  - OpenRocket (no): its command-line arguments set options for the window; none flies a design.
  - RocketPy (no): it is a Python library; the terminal only installs it.
  - RASAero II (no): it runs from its window; its manual describes no command.
- Flies a design from a Python script:
  - OpenRocket (no): its documentation covers a Java interface only; the separate orhelper package isn't in it.
  - RASAero II (no): its manual describes no scripting.
- Estimates the speed at which the fins flutter:
  - HPR Sim (partly): in the Rust library; the command line and Python don't print it yet, which the submission packs bring.
  - OpenRocket (no): its features page lists fin flutter as planned.
  - RocketPy (yes): an estimate from the same NACA criterion HPR Sim uses, which its documentation calls only an estimate.
  - RASAero II (no): its manual never mentions flutter.
- Reads an altimeter's flight log:
  - OpenRocket (no): its features page lists importing altimeter data as planned.
  - RocketPy (yes): its examples read a log saved as CSV and set it beside a flight.
  - RASAero II (no): it suggests writing the altimeter's readings into the design's comments, to compare by hand.
- Flies airbrakes driven by a control law:
  - HPR Sim (no): no airbrakes yet.
  - OpenRocket (no): its example extensions control roll, not drag; airbrakes would need a Java extension of one's own.
  - RASAero II (no): its manual describes no airbrakes or control.
- Flies a hybrid or liquid motor with its tanks:
  - HPR Sim (no): commercial solid motors only until its first full release.
  - OpenRocket (no): a motor is a thrust-curve file; its features page lists hybrids as planned.
  - RocketPy (yes): its tanks drain as the user prescribes; nothing predicts the flow.
  - RASAero II (no): a motor is a thrust-curve file or one from its own list.
- Warns when a flight leaves its models' assumptions:
  - RocketPy (no): its flight warning says the rocket is unstable at motor ignition, which is about the rocket, not its models.
  - RASAero II (no): its warnings, of a marginal and of an unstable rocket, are about the rocket, not its models.
- Names the known gaps in its models that bear on each flight, and which way each pushes the numbers where that is known:
  - OpenRocket (no): its warnings, as its technical documentation and its newest release's notes describe them, name a broken assumption and leave it to the user to judge whether the results still hold; none says which way they err.
  - RocketPy (no): its warnings say nothing of its models' errors.
  - RASAero II (no): its tips give general settings for a conservative altitude, not a flight's own gaps.

**The pages read.**

- HPR Sim: each cell links the page of this site that shows it, or the step of the roadmap that brings it.
- OpenRocket 24.12, released July 27, 2025, read on October 12, 2026. Its online guide is built from its development branch and describes features no release has yet, so what the release holds was read from its release notes. Its technical documentation is the edition its site still links; the page numbers in the titles are the printed ones, and the links open the file at them.
  - [OpenRocket 24.12's release notes](https://github.com/openrocket/openrocket/releases/tag/release-24.12)
  - [OpenRocket's guide: Features](https://openrocket.readthedocs.io/en/latest/introduction/features.html)
  - [OpenRocket's guide: Features, planned future features](https://openrocket.readthedocs.io/en/latest/introduction/features.html#planned-future-features)
  - [OpenRocket's guide: Getting Started](https://openrocket.readthedocs.io/en/latest/setup/getting_started.html#file)
  - [OpenRocket's guide: Getting Started, the user interface](https://openrocket.readthedocs.io/en/latest/setup/getting_started.html#the-user-interface)
  - [OpenRocket's guide: Advanced Rocket Design, staging](https://openrocket.readthedocs.io/en/latest/user_guide/advanced_rocket_design.html#conventional-staging)
  - [OpenRocket's guide: Command Line Arguments](https://openrocket.readthedocs.io/en/latest/dev_guide/command_line_arguments.html)
  - [OpenRocket's guide: Using OpenRocket Core in External Applications](https://openrocket.readthedocs.io/en/latest/dev_guide/using_openrocket_core.html)
  - [OpenRocket's guide: Simulation Extensions](https://openrocket.readthedocs.io/en/latest/user_guide/simulation_extensions.html#example-simulation-extensions-provided-with-openrocket)
  - [OpenRocket's guide: Thrust Curves](https://openrocket.readthedocs.io/en/latest/user_guide/thrust_curves.html)
  - [OpenRocket's technical documentation, supersonic drag (page 48)](https://github.com/openrocket/openrocket/releases/download/OpenRocket_technical_documentation-v13.05/OpenRocket_technical_documentation-v13.05.pdf#page=57)
  - [OpenRocket's technical documentation, warnings (page 78)](https://github.com/openrocket/openrocket/releases/download/OpenRocket_technical_documentation-v13.05/OpenRocket_technical_documentation-v13.05.pdf#page=87)
- RocketPy 1.13.0, released July 22, 2026, read on October 12, 2026.
  - [RocketPy's documentation: home](https://docs.rocketpy.org/en/latest/index.html)
  - [RocketPy's guide: Flight Class Usage, initial conditions](https://docs.rocketpy.org/en/latest/user/flight.html#initial-conditions)
  - [RocketPy's reference: the Rocket class](https://docs.rocketpy.org/en/latest/reference/classes/Rocket.html)
  - [RocketPy's guide: Monte Carlo class usage](https://docs.rocketpy.org/en/latest/notebooks/monte_carlo_analysis/monte_carlo_class_usage.html)
  - [RocketPy's guide: Forecasts](https://docs.rocketpy.org/en/latest/user/environment/1-atm-models/forecast.html#global-forecast-system-gfs)
  - [RocketPy's guide: Installation](https://docs.rocketpy.org/en/latest/user/installation.html)
  - [RocketPy's guide: First Simulation](https://docs.rocketpy.org/en/latest/user/first_simulation.html)
  - [RocketPy's reference: fin flutter analysis](https://docs.rocketpy.org/en/latest/reference/classes/utils/utilities.html#rocketpy.utilities.fin_flutter_analysis)
  - [RocketPy's guide: Flight Comparator](https://docs.rocketpy.org/en/latest/user/flight_comparator.html)
  - [RocketPy's guide: Air Brakes](https://docs.rocketpy.org/en/latest/user/airbrakes.html)
  - [RocketPy's guide: Tanks Usage](https://docs.rocketpy.org/en/latest/user/motors/tanks.html)
  - [RocketPy's reference: Exceptions and Warnings](https://docs.rocketpy.org/en/latest/reference/classes/exceptions.html)
- RASAero II 1.0.2.0, released May 22, 2019, read on October 12, 2026. Its manual is for the same release; the page numbers below are the manual's.
  - [RASAero II's download page](https://www.rasaero.com/dl_software_ii.htm)
  - [RASAero II's manual, introduction (page 4)](https://rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf#page=4)
  - [RASAero II's manual, contents (page 2)](https://rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf#page=2)
  - [RASAero II's manual, files (page 8)](https://rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf#page=8)
  - [RASAero II's manual, stages (page 49)](https://rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf#page=49)
  - [RASAero II's manual, launch site (page 78)](https://rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf#page=78)
  - [RASAero II's manual, altimeter data (page 82)](https://rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf#page=82)
  - [RASAero II's manual, motors (page 93)](https://rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf#page=93)
  - [RASAero II's manual, warnings and tips (page 114)](https://rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf#page=114)

<!-- features: end -->

## Measured side by side

The [scoreboard](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/scoreboard.md)
holds every comparison the project has measured, axis by axis, and a check that keeps HPR Sim's
numbers from getting worse. Of the table's 16 capabilities, HPR Sim has 12 in full, OpenRocket 7
and RocketPy 8. I chose the rows, so the count leans toward what HPR Sim was built to do: read the
rows, not the count.

On 53 logged flights of fliers' own designs, flown from the same files, HPR Sim's apogee missed by
12.95% on average without signs. OpenRocket's missed by 12.02% on average without signs. With
their signs, HPR Sim's averaged +8.73% above the logged apogees and OpenRocket's +7.79% above.
OpenRocket is ahead on both: this is HPR Sim's largest measured gap ([report][fixture-report]).

The flights are a private collection whose license allows only aggregates, so they can't be rerun
from this repository. HPR Sim flew 61 of its 83 flights whose design is an OpenRocket file. The
comparison holds the flights both tools flew, less those the report leaves out with its reasons.
So a design HPR Sim can't fly yet is in neither number. RocketPy hasn't flown them, since it can't
open their `.ork` files.

On the 7 public flights HPR Sim's apogee missed by 6.06% on average without signs, outside the
project's own 5% target ([report][real-report]). They're among the university rockets RocketPy's
documentation flies against their teams' logs. OpenRocket and RocketPy haven't flown them here yet
(see the planned milestones [an open benchmark](decisions-and-roadmap.md#m2-6) and
[the other tools' cells](decisions-and-roadmap.md#m2-9)).

The steps to a first answer are counted from a design's `.ork` file to its apogee and its landing
spread. HPR Sim takes 2 commands at a command line, or 10 lines of Python. OpenRocket flies it in
7 clicks in its window, but its newest release has no Monte Carlo for the spread. RocketPy's
Python can't open the file. Each step is listed on the scoreboard.

For a round trip through OpenRocket's file, HPR Sim reads an `.ork` file and writes it back.
OpenRocket then flies the original and the copy. Of 151 configurations, 0 moved by more than 0.002%,
and the largest change was 0.000050%; 2 of 75 designs didn't make the trip
([report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/openrocket-export-flights.md)).

## What it means for you

- If you design in OpenRocket, keep designing there. HPR Sim flies the same file as it is. Beside
  OpenRocket's numbers it sets a second flight, its spread from the command line or Python, and
  the known gaps that bear on that flight. Each gap comes with the side it pushes, where that's
  known.
- If you fly past the speed of sound, treat every tool's apogee there as rough. HPR Sim's
  supersonic drag is known to be off, either way. Neither HPR Sim's accuracy nor OpenRocket's has
  been measured against a logged flight there yet
  ([the planned milestone for published flights faster than sound](decisions-and-roadmap.md#m2-3c7)).
- If you need airbrakes, a hybrid or liquid motor, or a window to draw in, HPR Sim doesn't have
  them yet. The list above names the tools that do and the milestone that brings each.

## Where next

- To see it work, take [A tour of HPR Sim](simulator-tour.md).
- [Accuracy](accuracy.md) and [Trusting a simulation](trusting-a-simulation.md) say how close it
  has come to real flights.
- [Decisions and roadmap](decisions-and-roadmap.md) says what comes next and when.

**Sources.** The logged flights are a private collection of designs and their logs, shared as
aggregates only. They come from university rocketry teams, rocketry courses and hobbyists.

The public flights are among those RocketPy's documentation flies against their teams' logs. Both
fly in ERA5 weather: generated using Copernicus Climate Change Service information 2026, and
contains modified Copernicus Climate Change Service information 2026. Neither the European
Commission nor ECMWF is responsible for any use made of it. The [logged flights'
report][fixture-report] gives the full attribution and the ERA5 citations.

[real-report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/real-flights.md
[fixture-report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/fixture-flights.md
