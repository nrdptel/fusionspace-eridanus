<picture><source media="(prefers-color-scheme: dark)" srcset=".github/brand/readme-banner-dark.png 1x, .github/brand/readme-banner-dark@2x.png 2x"><source media="(prefers-color-scheme: light)" srcset=".github/brand/readme-banner-light.png 1x, .github/brand/readme-banner-light@2x.png 2x"><img alt="FusionSpace HPR: FS-ACHERNAR · SW · TOOL 001" src=".github/brand/readme-banner-dark.png" width="100%"></picture>

# FusionSpace HPR
[![vs RocketPy, same inputs: the gated metrics that pass](docs/images/census-badge.svg)](docs/accuracy.md#the-census) [![real flights: the mean apogee error against its target](docs/images/real-flights-badge.svg)](docs/accuracy.md#real-flights)

**Pre-alpha, under active construction.** Nothing here is ready to rely on yet.

**Documentation: <https://hpr.fusionspace.co/>**, a searchable guide with the API
reference (rustdoc) beside it. The same pages also read here on GitHub: [Start here](docs/start-here.md) says what works, what
doesn't and how far to trust it; [Getting started](docs/getting-started.md) flies a first rocket;
[Accuracy](docs/accuracy.md) has every validation result. `cargo xtask site` builds the site on
your machine.

FusionSpace HPR · Sim (hpr-sim before release 0.1's rename) is an open-source flight simulator
for hobby and high-power rockets, written in Rust. It
is a library first: a 6-DOF simulator in the spirit of [RocketPy](https://github.com/RocketPy-Team/RocketPy),
built to be embedded in other tools through Rust, a CLI, Python, C and WebAssembly. It is also
built to be checked: every model is cited and tested, and the simulator is measured against
independent simulators and real flight data, with the results published.

Planned:

- 6-DOF simulation with variable mass, detailed aerodynamics (subsonic to supersonic), real
  atmospheres and winds, and recovery with drift.
- Designing rockets from a parts catalog; import and export of OpenRocket, RockSim, RASAero and
  RocketPy files; a new open design format.
- Online weather (Open-Meteo's forecasts, weather-balloon soundings and NOAA's GFS and RAP, in the
  library and `hpr weather` today),
  [a site's elevation](https://hpr.fusionspace.co/elevation.html) (Open-Meteo's, or from a
  GeoTIFF terrain file of your own, in the library today) and
  [live motor stock and prices](https://hpr.fusionspace.co/motor-stock.html) from
  [motor.fusionspace.co](https://motor.fusionspace.co), matched to ThrustCurve.org's thrust
  curves (in the library today; `hpr motors search` lists the stock and prices),
  all optional. Everything works offline on macOS, Windows and Linux.
- [Monte Carlo dispersion](https://hpr.fusionspace.co/monte-carlo.html) (seeded and
  reproducible) and [sensitivity analysis](https://hpr.fusionspace.co/sensitivity.html)
  (Morris screening and Sobol' indices), and [optimization](https://hpr.fusionspace.co/optimization.html)
  of design numbers and choices (a motor, a catalog part) by CMA-ES, under limits such as a
  stability margin, of trade-offs between goals (a Pareto front, by NSGA-II), and of slow models
  in few evaluations (EGO, a Bayesian method), in the library today; competition challenges next.
- Flight-log import, comparison with the simulation, and diagnosis of what went wrong.
- Later: a modern desktop/web/mobile UI with 3D flight replay.

Scope for now: commercial off-the-shelf solid rocket motors.

**Every figure this tool produces is an estimate from a model, not a measurement, and never a
go/no-go verdict.** The motor's printed data and your RSO are authoritative.

## Install and first flight

Release 0.1.0 is built and checked, but not published yet. Once it is:

- **the command line:** the `fusionspace-hpr` archive for your system from
  [the releases page](https://github.com/nrdptel/fusionspace-eridanus/releases), or
  `cargo install fusionspace-hpr-cli --locked`; either gives you the `hpr` command;
- **Python:** `pip install fusionspace-hpr`, then `from fusionspace import hpr`;
- **a Rust program:** `cargo add fusionspace-hpr`, then `use hpr::…`.

[Install a release](https://hpr.fusionspace.co/getting-started.html#install-a-release) has
the details, and [the changelog](https://github.com/nrdptel/fusionspace-eridanus/blob/main/CHANGELOG.md) says
what 0.1.0 does, how far to trust it, and its known gaps.

Until then, build it from a copy of this repository. You need [Git](https://git-scm.com) and
[rustup](https://rustup.rs), Rust's installer; the first build fetches the pinned Rust version and
takes a few minutes.
[Getting started](https://hpr.fusionspace.co/getting-started.html#what-you-need) lists what
each operating system needs.

```bash
git clone https://github.com/nrdptel/fusionspace-eridanus
cd fusionspace-eridanus
cargo install --path crates/hpr-cli --locked
```

That puts `hpr` on your path. Fly the how-to guides' rocket, an OpenRocket design saved with an
AeroTech H170M:

<!-- cli: example `hpr sim validation/fixtures/ork/guides/level-1.ork`; written by `cargo xtask cli`; do not edit -->

```text
$ hpr sim validation/fixtures/ork/guides/level-1.ork
Level 1 guide rocket (level-1.ork)
configuration 1 of 2: [H170M-10]
help: --config flies the others, by number or name: 2 [I175WS-9]

static margin         1.70 calibres off the rail; least 1.70 calibres, at 0.12 s, before apogee
apogee                1065.1 m above the site at 11.62 s
rail exit speed       26.3 m/s
delay                 apogee 9.48 s after burnout; the motor's set delay: 10 s; its charge fires 0.52 s after apogee
descent               4.4 m/s at landing under `Parachute`

motor: 1 × H170M (the design's) in `Motor mount tube`, lit at launch
recovery: `Parachute` at the ejection charge, 0.520 m² of drag area, opened at 12.14 s
launched at 0° N, 0° E, 0 m above sea level, from a 1.5 m vertical rail, in calm air
help: see the Accuracy page before trusting these numbers: https://hpr.fusionspace.co/accuracy.html
note: the file's 1 recovery device flies as OpenRocket flies it: each opens fully at its event, with the file's drag coefficient or OpenRocket's own, and once one opens the rocket descends as a point under the open devices' drag alone
warning: drag: issue #67: a cone-like nose's or shoulder's pressure drag reads high from Mach 0.8 (about twice a measured cone's at Mach 0.85, still +15% at 1.5), so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are; this flight reaches Mach 0.84 at 1.7 s (https://github.com/nrdptel/fusionspace-eridanus/issues/67)
warning: drag: issue #68: base drag reads high from Mach 0.8 to 1.2 (0.225 against a measured 0.156 at Mach 0.9), so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are; this flight reaches Mach 0.84 at 1.7 s (https://github.com/nrdptel/fusionspace-eridanus/issues/68)
warning: drag: issue #18: skin friction is taken as fully turbulent, but on a smooth surface the flow stays laminar near the nose, where friction is lower: hpr's reads high by 3.6% on RocketPy's Calisto at Mach 0.3; if this surface is that smooth, the drag reads high, so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are (https://github.com/nrdptel/fusionspace-eridanus/issues/18)
warning: stability: issue #172: the static margin read up to 0.1108 calibres higher than OpenRocket's on four private designs, for a reason not yet found, so this flight's margin may read high by as much (https://github.com/nrdptel/fusionspace-eridanus/issues/172)

event                   time     height       speed
liftoff               0.00 s      0.4 m     0.0 m/s
rail exit             0.12 s      1.9 m    26.3 m/s
burnout               2.14 s    406.4 m   240.4 m/s
apogee               11.62 s   1065.1 m     0.1 m/s
charge               12.14 s   1063.7 m     5.1 m/s
deployment           12.14 s   1063.7 m     5.1 m/s
ground hit          245.78 s      0.0 m     4.4 m/s
(heights are the center of gravity's above the site; speeds are over the ground)

top speed             285.6 m/s at 1.73 s
top Mach number       0.842
landing               0.7 m from the pad at 245.78 s, at 4.4 m/s
```

<!-- cli: end -->

Then fly your own: `hpr sim my-rocket.ork`. A motor hpr doesn't carry is fetched from
ThrustCurve.org once, then flies offline. Three guides go further:
[Fly your .ork](https://hpr.fusionspace.co/fly-your-ork.html),
[Pick a motor](https://hpr.fusionspace.co/pick-a-motor.html) and
[Check stability for a certification flight](https://hpr.fusionspace.co/stability-for-certification.html).

## Python

`crates/hpr-py` builds `hpr`, a Python package over the library's builder: build a rocket part by
part or read a design file, fly it, and get its recording as NumPy arrays. It isn't on PyPI yet;
[Python](https://hpr.fusionspace.co/python.html) says how to build and use it.

## The command line

`hpr` is the command-line tool; [The command line](https://hpr.fusionspace.co/cli.html)
shows each command's output. This table is generated from the commands the tool registers, so it
lists only what exists: a "not yet" command refuses, with exit status 3, until its milestone.

<!-- cli: commands, written by `cargo xtask cli` from the registered commands; do not edit -->

| command | what it does | reads | prints | status |
|---|---|---|---|---|
| `hpr sim` | Fly a .ork, an .hpr or .hprz design, or a rocket's .json from a rail and print its flight; export its recording | `.ork`, `.hpr` or `.hprz`, a rocket's `.json`, a motor from the bundled catalog, `.eng` or `.rse`, or fetched from ThrustCurve.org | text, JSON, a recording as `.csv`, `.json`, `.parquet`, `.geojson` or `.kml` | available ([how to use it](https://hpr.fusionspace.co/cli.html#hpr-sim)) |
| `hpr convert` | Convert a motor file between .eng and .rse, or a catalog motor to either; or a design between .ork, .hpr and .hprz | `.eng`, `.rse`, the bundled catalog, a design as `.ork`, `.hpr` or `.hprz` | `.eng` or `.rse`, `.ork`, `.hpr` or `.hprz`, text, JSON | available ([how to use it](https://hpr.fusionspace.co/cli.html#hpr-convert)) |
| `hpr motors` | Look up motors in the bundled catalog, read a .eng or .rse motor file, fetch a curve from ThrustCurve.org, or search vendors' stock and prices | `.eng`, `.rse`, the bundled catalog, motor.fusionspace.co's stock and prices, fetched or saved, ThrustCurve.org's curves, fetched and cached | text, JSON | available ([how to use it](https://hpr.fusionspace.co/cli.html#hpr-motors)) |
| `hpr weather` | Fetch a launch day's weather, or read a weather file, as a profile of air and wind | Open-Meteo, a University of Wyoming sounding, GFS or RAP, fetched or saved, a whole GFS file, an ERA5 `.nc` | text, JSON, a profile as `.json` | available ([how to use it](https://hpr.fusionspace.co/cli.html#hpr-weather)) |
| `hpr mc` | Fly a design many times, each flight's inputs scattered at random, and print how its apogee and landing spread; export every flight | what `hpr sim` reads: a design and its motor | text, JSON, every flight's draw and outcome as `.csv` | available ([how to use it](https://hpr.fusionspace.co/cli.html#hpr-mc)) |
| `hpr optimize` | Search a design's parameters for a goal | - | - | not yet: [M6.3](https://hpr.fusionspace.co/decisions-and-roadmap.html#m6-3) |
| `hpr compare` | Compare a flight log with its simulation | - | - | not yet: [M7.3](https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-3) |
| `hpr analyze` | Read a flight log and print its readings, with no design file | a PerfectFlite `.pf2` flight log | text, JSON | available ([how to use it](https://hpr.fusionspace.co/cli.html#hpr-analyze)) |
| `hpr diagnose` | Diagnose what went wrong in a flight from its log | - | - | not yet: [M7.4](https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-4) |
| `hpr completions` | Print a shell completion script for hpr | - | a bash, elvish, fish, powershell or zsh script, JSON | available ([how to use it](https://hpr.fusionspace.co/cli.html#hpr-completions)) |

<!-- cli: end -->

## Accuracy at a glance

What hpr has been compared with, and how it came out. A "code-to-code" line says how closely hpr
agrees with another simulator, not which of the two is right; only the real flights are
measurements, and on those hpr misses its target. Each number behind this table is also a
check: CI fails when one moves, better or worse, until the change is accepted with a written
reason. CI flies the RocketPy comparisons again on every change; the OpenRocket and real-flight
ones need files CI doesn't have, so it holds their committed numbers
([the census](docs/accuracy.md#the-census)).

<!-- census: written by `cargo xtask census --accept` from validation/reports/census.json; do not edit -->

| compared with | kind | held to | flights (speed) | result |
|---|---|---|---|---|
| [RocketPy 1.13.0](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/census.md): descents under a parachute | code-to-code, same inputs | gate: each metric's tolerance, at most 3% | 5 descents | 30 of 30 gated metrics pass |
| [RocketPy 1.13.0, patched](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/census.md): whole flights on the same drag | code-to-code, same inputs | gate: each metric's tolerance, at most 3% | 9 flights (8 subsonic, 1 transonic) | 142 of 142 gated metrics pass; apogee +0.04% to +1.21%; 11 not scored, each for a written reason |
| [RocketPy 1.13.0, patched](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/census.md): whole flights, each code on its own drag | code-to-code, each code's own drag | target: 3% on each metric, reported, not enforced | 6 flights (5 subsonic, 1 transonic) | 75 of 102 metrics within target; apogee -7.28% to +10.30% |
| [OpenRocket 24.12](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/census.md): calm flights of OpenRocket's examples | code-to-code, each code's own model | no target; an apogee more than 5% off needs a written cause | 54 flights (53 subsonic, 1 transonic); 3 more not flown | apogee -37.93% to +13.80%; apogee within 5% on 45 of 53, largest speed within 5% on 50 of 53, margin within 0.5 calibres on 52 of 53, launch mass within 1% on 54 of 54, mass at rod clearance within 1% on 50 of 54, center of mass at rod clearance within 0.5 calibres on 54 of 54; 3 values withheld by the report |
| [OpenRocket 24.12](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/census.md): calm flights of the private designs | code-to-code, each code's own model | no target; an apogee more than 5% off needs a written cause | 35 flights (28 subsonic, 6 transonic, 1 supersonic); 2 more not flown | apogee -4.84% to +13.60%; apogee within 5% on 34 of 35, largest speed within 5% on 34 of 35, margin within 0.5 calibres on 35 of 35, launch mass within 1% on 35 of 35, mass at rod clearance within 1% on 35 of 35, center of mass at rod clearance within 0.5 calibres on 35 of 35 |
| [the teams' altimeter logs](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/census.md): real flights | measured | target: mean absolute apogee error 5% | 7 flights (3 subsonic, 4 transonic) | mean absolute apogee error 6.04% (target 5%, missed); apogee -8.90% to +10.40%; apogee within 5% on 2 of 7, climb's RMS height error within 3% of apogee on 2 of 7 |

<!-- census: end -->

The census doesn't count 55 more real flights, from a private collection: hpr's apogees are
+9.83% above their logs on average and OpenRocket's +9.00%, both outside the 5% target
([private collection](docs/accuracy.md#real-flights-of-the-private-collection)).

"Patched" means RocketPy 1.13.0 run with two fixes from RocketPy's own pull requests, which
correct the point it takes the burn's turning moments about
([why](docs/accuracy.md#whole-flights-against-rocketpy)).

## Building

The toolchain is pinned in `rust-toolchain.toml`, so [rustup](https://rustup.rs) installs the right
version on first use.

```bash
cargo test --workspace --all-features   # unit tests
cargo xtask wasm-check                  # the pure core builds for wasm32-unknown-unknown
cargo deny check                        # dependency licenses, advisories and sources
cargo xtask site                        # build the documentation site and check its links
```

`cargo test` also runs the design format's generated TypeScript and Python readers, so it needs
Node.js 22.18 or later and Python 3.11 or later on the path
([TypeScript and Python](docs/format/hpr.md#typescript-and-python)).

`cargo xtask site` needs mdBook 0.5: `cargo install mdbook --version 0.5.4 --locked` (the version
CI uses) or `brew install mdbook`. The site is built from `docs/` into `target/site`: open
`target/site/index.html`, or read [`docs/start-here.md`](docs/start-here.md) on GitHub.

Validation work uses a local reference library (other simulators, papers, motor data), pinned in
`validation/refs.lock.toml` and downloaded into the gitignored `refs/`. Fetching it needs `git`,
`curl` and [uv](https://docs.astral.sh/uv/); the OpenRocket oracle also needs Java 17+.

```bash
cargo xtask refs fetch    # fetch everything to its pinned state (safe to rerun)
cargo xtask refs verify   # re-hash everything against the pins
cargo xtask refs doctor   # which tools and oracles work on this machine
```

The workspace crates live under `crates/`; `docs/ARCHITECTURE.md` describes what each one is for.
[The roadmap](docs/ROADMAP.md) lists open work and [its archive](docs/roadmap-done.md) what is
done.

## Design

hpr-sim is part of FusionSpace, the project owner's family of rocketry tools, and follows its
[product system](https://github.com/nrdptel/fusionspace-design/blob/main/product/README.md) (revision A, pinned at its October 7, 2026 commit `f45454f`): the rules for how its
tools look, word things and show how far to trust a result. It changes how hpr-sim presents
numbers, not how it computes them. [The decision record](docs/decisions/0164-the-fusionspace-product-system.md)
says what is adopted, and [M0.6, the product system milestone](docs/decisions-and-roadmap.md#m0-6),
brought the site, the command line, the plot and the exports in line with it.

- The system's [principles](https://github.com/nrdptel/fusionspace-design/blob/main/product/principles.md) apply to everything; the command line also follows
  [`cli.md`](https://github.com/nrdptel/fusionspace-design/blob/main/product/cli.md), the documentation site [`web.md`](https://github.com/nrdptel/fusionspace-design/blob/main/product/web.md), and numbers, plots and
  exports [`data.md`](https://github.com/nrdptel/fusionspace-design/blob/main/product/data.md).
- The site's theme and fonts and the command line's color styles are the system's files, copied in
  unchanged with their license lines ([third-party notices](THIRD-PARTY-NOTICES.md)); the plot
  uses its [`tokens/`](https://github.com/nrdptel/fusionspace-design/blob/main/product/tokens/) colors and no others.
- The designation `FS-ACHERNAR · SW · TOOL 001` names hpr-sim like a part: Achernar, its internal
  name, one of the stars of project Eridanus; then software, tool 1
  ([ADR-198, project Eridanus](docs/decisions/0198-the-2026-10-07-project-eridanus.md)).
  `hpr --version`, the site's title block and every file hpr writes show it beside the version.
  Files stamped with the earlier `FS · SW · TOOL 005` still read.
- Its [`review.md`](https://github.com/nrdptel/fusionspace-design/blob/main/product/review.md) checklist runs before each release.

## License

`FS-ACHERNAR · SW · TOOL 001` · part of [FusionSpace](https://fusionspace.co). Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option. Third-party sources and their licenses are listed in
[THIRD-PARTY-NOTICES.md](THIRD-PARTY-NOTICES.md).

The FusionSpace name, its logo (four nose cones) and the banners in `.github/brand/` are the project
owner's and are not covered by these licenses (see fusionspace-design's [`TRADEMARKS.md`](https://github.com/nrdptel/fusionspace-design/blob/main/TRADEMARKS.md)).

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
