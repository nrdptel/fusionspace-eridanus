# Changelog

Each release of FusionSpace HPR (called hpr-sim before release 0.1) has an entry here: what it holds, what works, how far to trust it, and the
gaps known when it was built. One version number covers every crate, the `hpr` command line and
the Python package. Until 1.0, a new minor version (0.2, 0.3) may change the library's API; a
patch release (0.1.1) carries only a fix for a critical issue
([the release plan](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0162-the-suite-and-its-releases.md)).

## 0.1.0 (not yet published)

**The first release: the simulator, as a Rust library, the `hpr` command line and the Python
package. Its files are built and checked, but not published yet.** Every number it prints is an
estimate from a model, never a go/no-go verdict: a motor's printed data and your range safety
officer decide.

### What it holds

- **The command line, `hpr`,** as an archive for Linux on x86-64, Macs with Apple silicon and
  Windows on x86-64, or from crates.io as `fusionspace-hpr-cli`. It flies a design (`hpr sim`), scatters it
  (`hpr mc`), looks up motors (`hpr motors`), converts design files (`hpr convert`), reads a flight
  log (`hpr analyze`) and fetches a launch day's weather (`hpr weather`)
  ([The command line](https://hpr.fusionspace.co/cli.html)). The Linux archive needs the
  GNU C library (glibc) 2.17 or later: RHEL and CentOS 7, Debian 8, Ubuntu 14.04 and anything
  newer, but not a distribution built on musl, such as Alpine.
- **The Python package, `fusionspace-hpr`,** which imports as `fusionspace.hpr`: one wheel per operating system for
  CPython 3.10 and later (on Linux a manylinux2014 wheel, for glibc 2.17 or later), and the
  source package ([Python](https://hpr.fusionspace.co/python.html)).
- **The library: 13 crates on crates.io,** `fusionspace-hpr`, the front door (`use hpr::…` in
  code), and the `fusionspace-hpr-*` crates under it
  ([the API reference](https://hpr.fusionspace.co/api.html)).
- **The license files.** Each archive and wheel carries FusionSpace HPR's two licenses, MIT and
  Apache-2.0, its notices of outside sources, and the license texts of every crate compiled in;
  each crate on crates.io carries the two licenses
  ([How a release is built](https://hpr.fusionspace.co/releasing.html)).

[Getting started](https://hpr.fusionspace.co/getting-started.html#install-a-release) says
how to install each.

### What works

- **The environment:** WGS 84 gravity and the Earth's rotation; the 1976 US Standard Atmosphere
  with offsets and humidity; soundings, ERA5 files and online forecasts; constant, layered and
  power-law wind with seeded turbulence.
- **Motors:** `.eng` and `.rse` thrust curves, 32 bundled motors, and any other fetched from
  ThrustCurve.org by name and cached for offline use.
- **Rockets:** nose cones, tubes, transitions, fins, pods and the parts inside them, their mass
  properties, and design checks that warn on everyday fits and refuse only what can't be built.
- **Aerodynamics** modeled from Mach 0 to 5 at small angles of attack: the normal force, the
  center of pressure and the drag. Whole flights are checked mostly below about Mach 1.15
  (*How far to trust it*, below).
- **Flight:** six degrees of freedom from the rail to the ground, with staging, separations,
  parachutes, streamers and tumbling.
- **Design files:** OpenRocket `.ork` files read and flown, and written back; the HPR design
  format (`.hpr`); OpenRocket's parts catalog built in. `hpr sim` flies 136 of the 170 motor configurations
  in the reference library and OpenRocket's examples as saved
  ([`.ork` design files](https://hpr.fusionspace.co/format/ork.html#how-many-configurations-hpr-sim-flies)).
- **Flight logs:** a PerfectFlite `.pf2` log read on its own, with liftoff, apogee, top speed and
  landing taken from it.
- **Monte Carlo** from Rust, `hpr mc` and Python, seeded so a run repeats exactly, with failed
  flights counted; **optimization** (CMA-ES, NSGA-II and EGO) from Rust.

[Start here](https://hpr.fusionspace.co/start-here.html#what-works-today) has each part with
its pages.

### How far to trust it

[Accuracy](https://hpr.fusionspace.co/accuracy.html) has every comparison, gaps included.
In brief:

- **Against seven public real flights, HPR Sim's apogees miss by 6.04% on average,** outside the 5%
  target ([real flights](https://hpr.fusionspace.co/accuracy.html#real-flights)).
- **Against 55 flights of a private collection, HPR Sim's apogees are 9.83% above the logs on
  average, and OpenRocket's 9.00%** ([private collection](https://hpr.fusionspace.co/accuracy.html#real-flights-of-the-private-collection)).
- **Given the same drag, whole flights match RocketPy's within 3%** in apogee, speeds, burnout and
  flight time on six of its example rockets
  ([against RocketPy](https://hpr.fusionspace.co/accuracy.html#whole-flights-against-rocketpy)).
- **The drag reads high at most speeds against NASA's Arcas Robin wind tunnel,** most of all with
  fins past Mach 1: 2 of 44 measurements are within the 10% target
  ([Aerodynamics](https://hpr.fusionspace.co/physics/aero.html#drag-against-the-arcas-robin-wind-tunnel)).
- **Whole flights are checked mostly below about Mach 1.15.** Faster flights still fly, and a
  flight beyond the validated range, at a high angle of attack or outside the envelope says so
  ([Start here](https://hpr.fusionspace.co/start-here.html#how-far-to-trust-it)).

### Known gaps

**Errors on the flattering side.** These open issues make a number look safer than it is: a
stability margin too high, or an apogee, a top speed or a drift too low. A flight whose shape or
speed meets an issue below prints a warning naming it, as its *warns* column says; one exception is
`hpr mc`'s drawn deployment lags (#354), which warn only for the nominal flight.
Near a limit, leave room on the side the error leans: a margin to spare above the minimum, an
apogee estimate below the waiver's ceiling with room above it, a recovery area wider than the
drift printed.

| issue | the error | the number it flatters | warns |
|---|---|---|---|
| [#87](https://github.com/nrdptel/fusionspace-eridanus/issues/87), [#120](https://github.com/nrdptel/fusionspace-eridanus/issues/120), [#121](https://github.com/nrdptel/fusionspace-eridanus/issues/121) | a step in radius, a long lip or a steep pointed tip takes the whole body off the supersonic method | the margin, past Mach 1.2 | yes |
| [#172](https://github.com/nrdptel/fusionspace-eridanus/issues/172) | margins above OpenRocket's on private designs, cause unknown | the margin, on every flight | yes, quoting the largest gap: four private designs read 0.0350 to 0.1108 calibers more stable than in OpenRocket ([How far to trust the margin](https://hpr.fusionspace.co/stability-for-certification.html#how-far-to-trust-the-margin)) |
| [#222](https://github.com/nrdptel/fusionspace-eridanus/issues/222) | supersonic pressure drag about twice OpenRocket's; which is right isn't known | the apogee and top speed, past Mach 1 | yes |
| [#67](https://github.com/nrdptel/fusionspace-eridanus/issues/67) | a cone-like nose's or shoulder's transonic pressure drag reads high: +87% at Mach 0.8 and +105% at 0.85 against a measured 3:1 cone, still +15% at Mach 1.5 | the apogee and top speed, past Mach 0.8 | yes, on a nose or shoulder whose shape takes the closed-form cone formula, past Mach 0.8 |
| [#68](https://github.com/nrdptel/fusionspace-eridanus/issues/68) | base drag reads high from Mach 0.8 to 1.2, low from Mach 1.5 | the apogee and top speed, from Mach 0.8 to 1.2 | yes |
| [#70](https://github.com/nrdptel/fusionspace-eridanus/issues/70) | a sharp fin section's drag reads high | the apogee and top speed, past Mach 1 | yes |
| [#72](https://github.com/nrdptel/fusionspace-eridanus/issues/72) | a boattail steeper than 10°: its drag reads high | the apogee and top speed, past Mach 1 | yes |
| [#325](https://github.com/nrdptel/fusionspace-eridanus/issues/325) | fin sets at one station counted apart for fin–fin interference: about 0.029 of the 0.071 to 0.076 calibers that OpenRocket's *Pods--airframes and winglets* example reads more stable than in OpenRocket | the margin, when the sets share more than four fins | yes |
| [#326](https://github.com/nrdptel/fusionspace-eridanus/issues/326) | a kinked freeform fin's center of pressure, the rest of that example's gap | the margin | yes, on every freeform fin set, at any speed |
| [#64](https://github.com/nrdptel/fusionspace-eridanus/issues/64) | a forward-swept fin's supersonic lift reads high | the margin, past Mach 1 | yes |
| [#367](https://github.com/nrdptel/fusionspace-eridanus/issues/367) | a packed mass drawn too wide for a nose cone's or a shoulder's room, where its center of gravity can't sit | the margin | yes, in the design checks' warning |
| [#18](https://github.com/nrdptel/fusionspace-eridanus/issues/18) | skin friction is taken as fully turbulent: on a smooth surface the drag reads high, by about 2% on RocketPy's Calisto at Mach 0.3 | the apogee and top speed | yes, on every flight on HPR Sim's drag |
| [#73](https://github.com/nrdptel/fusionspace-eridanus/issues/73) | the subsonic boattail rule: no drag for a long boattail where measurements give some, and on 16° ones from 40.8% too low to 41.7% too high | the apogee and top speed, below Mach 1, where it reads high | yes, for a boattail steeper than 9.5° (`atan(1/6)`), where the rule starts to give it drag |
| [#179](https://github.com/nrdptel/fusionspace-eridanus/issues/179) | a booster dropped at a separation tumbles side-on from the split, with no airframe drag, where a real one first coasts nose-first | the booster's peak and drift, which likely read short | yes, on every flight that drops a booster, at any speed |
| [#354](https://github.com/nrdptel/fusionspace-eridanus/issues/354) | a separated part has no drag at all until its first device opens | its drift, which likely reads short for a falling part | yes, when a separated part starts with nothing open; `hpr mc` warns only for its nominal flight, not for a drawn lag |
| [#219](https://github.com/nrdptel/fusionspace-eridanus/issues/219) | a center of gravity off the axis, as an off-axis lug, rail button or pod puts it: HPR Sim flies the moment the thrust makes about it, OpenRocket doesn't, and no reference sizes it | the apogee or the drift, low or high by which side the offset faces | yes, when the center of gravity sits more than 1e-9 m off the axis, which most designs with a lug or a rail button do |
| [#213](https://github.com/nrdptel/fusionspace-eridanus/issues/213) | a single pod's drag and normal force act on the axis, so their moments are left out | the apogee or the drift, low or high by which side the pod faces | yes, on a pod set of one pod off the axis that holds a part |
| [#106](https://github.com/nrdptel/fusionspace-eridanus/issues/106) | inside the supersonic join, a body part's pitch and yaw damping is sampled away from its own center of pressure; the margins don't use it | the apogee or the drift, either way, unsized | yes, past the join's start, Mach 1.2 or later |
| [#8](https://github.com/nrdptel/fusionspace-eridanus/issues/8) | a near-calm wind level, 1.5 m/s or less, still turns the interpolated direction | the drift, likely short where the other levels share one direction, and the apogee | yes, from the library, on a wind table whose near-calm level turns inside the flight's heights |

**Four of the rows above err either way.** [#8](https://github.com/nrdptel/fusionspace-eridanus/issues/8), [#106](https://github.com/nrdptel/fusionspace-eridanus/issues/106), [#213](https://github.com/nrdptel/fusionspace-eridanus/issues/213) and [#219](https://github.com/nrdptel/fusionspace-eridanus/issues/219) can read low on some flights and high on others, so each warns as if it flattered, as a `warning: flight:` line. They give a direction, not a size: where the apogee or the drift sits near a limit, leave room on both sides.

**Three print no wrong number.** HPR Sim never prints [#104](https://github.com/nrdptel/fusionspace-eridanus/issues/104)'s figure, a body part's own center of pressure (every one it prints is the whole rocket's), and its `.ork` reader refuses a file whose surface finish ([#360](https://github.com/nrdptel/fusionspace-eridanus/issues/360)) or pair of mass-override flags ([#361](https://github.com/nrdptel/fusionspace-eridanus/issues/361)) it can't read, rather than flying it.

**Near the speed of sound,** between Mach 0.8 and 1.2, the center of pressure sits up to 2.36
calibers off NASA's wind-tunnel data, behind it wherever it misses by more than half a caliber, so
the margin there reads high. A flight past the validated speed carries a flag
([Accuracy](https://hpr.fusionspace.co/accuracy.html#known-gaps)).

**A flight that turns unstable under power,** its static margin below zero while a motor burns,
is flagged: `hpr sim` says its apogee is not a prediction
([#335](https://github.com/nrdptel/fusionspace-eridanus/issues/335);
[Unstable under power](https://hpr.fusionspace.co/physics/metrics.html#unstable-under-power)).
Where HPR Sim can give no margin, a pitching moment that turns the rocket away under power raises
the same warning. A margin that turns negative only after burnout is printed but not flagged.

**Not in this release:**

- Motors other than commercial solids: no hybrids, liquids or research motors.
- From Python: no staged flights, and a design's parachutes fly only with `recovery=True`.
- `hpr mc` scatters one wind for every height
  ([#265](https://github.com/nrdptel/fusionspace-eridanus/issues/265)).
- The validation harness isn't published: its check runs from a copy of the repository, as
  `cargo xtask validate --check`.
- 22 of the private collection's 83 flights aren't flown yet, each for a cause with its own issue
  ([#181](https://github.com/nrdptel/fusionspace-eridanus/issues/181), and
  [#357](https://github.com/nrdptel/fusionspace-eridanus/issues/357) to
  [#364](https://github.com/nrdptel/fusionspace-eridanus/issues/364)).
