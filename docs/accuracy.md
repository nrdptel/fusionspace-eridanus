# Accuracy

This page gathers every check HPR Sim has passed so far, and every known gap, in words and
numbers. Start with the bottom line: **when both codes fly the same drag
([same-drag](glossary.md#same-drag-and-predicted-mode)), HPR Sim's whole flights match RocketPy's
in height, speed and time, and in where they land without wind. In wind they agree for a rocket
that leaves the rail fast. For one that leaves it slowly they differ, in large part because HPR Sim
includes a sideways force on the body that RocketPy leaves out.** With each code's own drag
([predicted](glossary.md#same-drag-and-predicted-mode)), HPR Sim's heights differ from RocketPy's by
−7.280% to +10.302% ([report][report]), the larger gaps where its drag differs most from the
example's. **Against the logs of seven real flights, HPR Sim's apogees miss by 6.04% on average,
outside the 5% target, and by −8.90% to +10.40% one by one** ([real flights](#real-flights),
[report][real-report]); four of the five misses past 5% are consistent with HPR Sim's drag, and one
with the motor's impulse. Four of those seven altimeters' kinds are assumed, and none is
calibrated. **Against 55 more flights from a private collection, HPR Sim's apogees are +9.83% above
the logs on average and miss by 13.96% (mean absolute), also outside the target; OpenRocket's are
+9.00% above** ([private collection](#real-flights-of-the-private-collection),
[report][fixture-report]).

What has been checked so far:

- each model on its own, against exact answers, its published source and, in places,
  [RocketPy](glossary.md#rocketpy), an open-source flight simulator;
- the descent under a parachute, against RocketPy, for five rockets;
- whole flights from the pad to the ground, against RocketPy, for six rockets flown with one
  declared drag coefficient, one of them past Mach 1: heights, speeds and times agree, and so does
  the path, except for rockets that leave the rail slowly in a wind;
- the same flights with each code's own drag, reported against a target rather than gated, the one
  past Mach 1 included;
- seven real flights, flown in the weather of their day and compared with their altitude logs:
  the apogee and the climb to it ([real flights](#real-flights));
- 55 more real flights of a private collection, flown by HPR Sim and OpenRocket in the weather of
  their day and compared with their logged apogees as aggregates
  ([private collection](#real-flights-of-the-private-collection), [report][fixture-report]);
- the normal force and center of pressure from Mach 0.6 to 4.63, against NASA's wind-tunnel tests
  of a sounding rocket, and against [RASAero II](glossary.md#rasaero-ii), another code
  ([fixture][nf-fixture]);
- drag from Mach 0.6 to 4.63 against the same wind-tunnel tests, the forebody only
  ([Aerodynamics](physics/aero.md#drag-against-the-arcas-robin-wind-tunnel));
- a boattail's own drag and the base pressure behind it, from Mach 0.3 to 3.24, against measured
  boattails in six NACA and NASA reports ([drag fixture][drag-fixture]);
- the roll that [canted](glossary.md#cant) fins give, from Mach 1.5 to 4.63, against NASA's
  Arcas Robin wind-tunnel tests, and the roll damping from Mach 1.5 to 3 against the Basic
  Finner's, a standard finned test body ([roll fixture][roll-fixture]);
- drag from Mach 0.1 to 2.0 against the curves labelled RASAero II in RocketPy's example rockets
  ([Aerodynamics](physics/aero.md#drag-against-rasaero-ii-through-mach-2)), and from Mach 0.5 to
  3.2 against a worked example in MIL-HDBK-762, the U.S. Army's handbook for designing unguided
  rockets
  ([Aerodynamics](physics/aero.md#drag-against-mil-hdbk-762s-sample-calculation));
- the readings `hpr analyze` takes from a flight log, against an invented log whose every
  reading is known, each within the bound its rounding and filter allow
  ([Flight-log readings](physics/log-readings.md#checked-against)).

Every number here links to the page or file it comes from. [Checking a claim](checking-a-claim.md)
shows how to follow one back to its source and its test, and
[how the site keeps the two in step](checking-a-claim.md#rules-that-keep-the-trail-honest).

## The census

**In short: the census counts the numbers the validation reports hold HPR Sim to, once each, and
says what each group was compared with, how, how many flights it holds and how fast they flew. It is
also a check: CI fails when any of those numbers moves, better or worse, until the change is
accepted with a written reason.** A code-to-code line measures how closely HPR Sim agrees with
another simulator, not which of the two is right; only the real flights are measurements.

<!-- census: written by `cargo xtask census --accept` from validation/reports/census.json; do not edit -->

| compared with | kind | held to | flights (speed) | result |
|---|---|---|---|---|
| [RocketPy 1.13.0](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/census.md): descents under a parachute | code-to-code, same inputs | gate: each metric's tolerance, at most 3% | 5 descents | 30 of 30 gated metrics pass |
| [RocketPy 1.13.0, patched](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/census.md): whole flights on the same drag | code-to-code, same inputs | gate: each metric's tolerance, at most 3% | 9 flights (8 subsonic, 1 transonic) | 142 of 142 gated metrics pass; apogee +0.04% to +1.21%; 11 not scored, each for a written reason |
| [RocketPy 1.13.0, patched](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/census.md): whole flights, each code on its own drag | code-to-code, each code's own drag | target: 3% on each metric, reported, not enforced | 6 flights (5 subsonic, 1 transonic) | 75 of 102 metrics within target; apogee -7.28% to +10.30% |
| [OpenRocket 24.12](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/census.md): calm flights of OpenRocket's examples | code-to-code, each code's own model | no target; an apogee more than 5% off needs a written cause | 54 flights (53 subsonic, 1 transonic); 3 more not flown | apogee -37.93% to +13.80%; apogee within 5% on 45 of 53, largest speed within 5% on 50 of 53, margin within 0.2 calibres on 52 of 53, launch mass within 1% on 54 of 54, mass at rod clearance within 1% on 50 of 54, center of mass at rod clearance within 0.2 calibres on 54 of 54; 3 values withheld by the report |
| [OpenRocket 24.12](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/census.md): calm flights of the private designs | code-to-code, each code's own model | no target; an apogee more than 5% off needs a written cause | 35 flights (28 subsonic, 6 transonic, 1 supersonic); 2 more not flown | apogee -4.84% to +13.60%; apogee within 5% on 34 of 35, largest speed within 5% on 34 of 35, margin within 0.2 calibres on 35 of 35, launch mass within 1% on 35 of 35, mass at rod clearance within 1% on 35 of 35, center of mass at rod clearance within 0.2 calibres on 35 of 35 |
| [the teams' altimeter logs](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/census.md): real flights | measured | target: mean absolute apogee error 5% | 7 flights (3 subsonic, 4 transonic) | mean absolute apogee error 6.04% (target 5%, missed); apogee -8.90% to +10.40%; apogee within 5% on 2 of 7, climb's RMS height error within 3% of apogee on 2 of 7 |

<!-- census: end -->

**Reading the table.**

- *Compared with* names the reference and its version ([census][census]):
  [RocketPy](glossary.md#rocketpy), "patched" where RocketPy 1.13.0 runs with two fixes from
  RocketPy's own pull requests, which correct the point it takes the burn's turning moments about
  (see [whole flights against RocketPy](#whole-flights-against-rocketpy)),
  [OpenRocket](glossary.md#openrocket), or the teams' altimeter logs.
- *Kind* says whether both programs flew the same inputs, each flew its own model, or HPR Sim was
  compared with a measurement.
- *Held to* is what each report holds its numbers to ([gate and target](glossary.md#gate-and-target)).
  A gate fails the run when a number falls outside it. A target is reported: missing it never fails
  the run. The OpenRocket comparisons have neither, only a line past which an apogee needs a written
  cause.
- *Flights (speed)* classes each flight by its largest [Mach number](glossary.md#mach-number):
  subsonic below 0.8, transonic from 0.8 to 1.2, supersonic above 1.2 ([census][census]). For the
  harness's flights that is RocketPy's largest Mach number, for OpenRocket's flights OpenRocket's,
  and for the logged flights HPR Sim's own, since a log has none. A configuration an OpenRocket
  report lists and HPR Sim doesn't fly yet is counted as "not flown", with the report's reason.

**What it counts.** Each of these is one row of the census ([census][census]):

- every metric of every case in the harness's report, and each known gap;
- each real flight's apogee and the RMS of its climb;
- each OpenRocket flight's apogee, largest speed, stability margin, mass at launch and at rod
  clearance, and center of mass there, and each configuration not flown.

It leaves out the figures the reports give to explain a difference rather than to measure one, such
as a real flight's apogee on its team's own drag.

**The check.** The census accepted last is committed, with a page of its own
([census][census]). `cargo xtask validate --check`, which CI runs on macOS, Windows and Linux,
holds every row of the committed reports to it. To run it yourself, clone the repository and run
`cargo xtask validate --check`. It fails when:

- a row's difference moves by more than its slack, in either direction;
- a row's standing changes, for instance a miss that starts meeting its target;
- a row comes or goes, for instance a flight HPR Sim used to refuse and now flies;
- a group's reference changes, such as a new version of OpenRocket.

A row's slack is 0.1% of its scale, and for a harness row never less than the harness's
reproduction bound, 2e-6 in the metric's own unit or 1e-7 of the value, whichever is larger. The scale is the row's own
tolerance where it has one, and otherwise the bar of its kind: 3% of its reference for a harness
metric not scored, 5% for an OpenRocket apogee or largest speed, 1% for an OpenRocket mass, 0.5
calibres for a stability margin or center of mass, 5% for a logged apogee, and 3% of the apogee for a logged climb's RMS, the
bound the RocketPy comparisons hold a whole flight's height RMS to ([census][census]). For example, a 3% tolerance on a 1000 m (3,281 ft) apogee allows 30 m (98 ft), and the census lets
the difference move by 0.03 m (0.098 ft) before it fails.

So the census holds a number to where it was, not to its target. A flight on each code's own drag,
whose 3% is only a target, is held that way too ([census][census]): HPR Sim's own aerodynamics can't
drift unseen inside the target.

**Accepting a change.** `cargo xtask census --accept --reason "<why>"` writes the new census and
the table above. The reason and the list of what changed go into the census page, so a worse number
can still merge, but only in writing, in the change that brings it. An improvement has to be
accepted too; otherwise it could slip back later without anyone seeing.

> **How far to trust it.** The census adds no evidence of its own: it is exactly as good as the
> reports it counts. CI flies the harness's RocketPy comparisons again on every change. The
> OpenRocket and real-flight reports need files that CI doesn't have, so CI holds their committed
> numbers, and a change to the code that would move them is caught only when someone runs them again
> and commits the result.

## How to read the numbers

- **Powers of ten.** Very small and very large numbers are written the way programs print them.
  The number after the `e` says how many places the decimal point moves, to the left when it is
  negative. So 1e-12 is a millionth of a millionth, a 1 in the twelfth decimal place, and 1e6 is a
  million. Both appear in the two [Frames](physics/frames.md) rows of the results table below.
- **Absolute differences** carry a unit: they say how far apart two values are, in that unit.
  [Geodesy](physics/geodesy.md)'s round trips return heights within 2e-8 m, twenty billionths of a
  meter.
- **Relative differences** are marked *relative*: the difference as a fraction of the value it is
  compared with. [Gravity](physics/gravity.md) within 2e-14 relative means within two parts in a
  hundred million million. A percentage is a relative difference counted in hundredths.
- **Signs.** A signed difference is HPR Sim's value less the one it is compared with, over that one.
  A plus means HPR Sim's value is the larger in size, a minus the smaller.

## Four kinds of evidence

A model can be checked in four ways, from the weakest to the strongest evidence that it matches
reality. They are set out in the project's [validation plan][plan].

| kind | what is compared | what agreement shows |
|---|---|---|
| **Analytic** | The code against exact answers: formulas solved by hand (closed forms), conservation laws, and round trips (converting a value and converting it back) | The code computes what its equations say |
| **Published source** | The code against a source's printed tables and worked examples | The code implements the source correctly |
| **Another code** | HPR Sim against another simulator, such as RocketPy, flying the same inputs | The two codes agree on the physics; not that either matches reality |
| **Real flights** | HPR Sim against measured flights | The model matches reality, within the flight's own uncertainty, which here is not measured |

The first three check the code. Only the fourth checks the physics against the world. Seven
flights have been compared so far, each as a whole: the apogee and the climb to it, not each model
on its own ([real flights](#real-flights)). Besides them, two recovery models are checked against
published drop tests ([Recovery](physics/recovery.md)), and the normal force against NASA's
wind-tunnel tests of the Arcas Robin sounding rocket
([Aerodynamics](physics/aero.md#normal-force-through-mach-1)): measurements, but not flights.

## Where each model stands

A tick means the model has been checked that way; a "no" means it hasn't yet. *In the descents
only* means RocketPy's air density and wind were compared with HPR Sim's at just the 23 heights its
parachute descents sample, as part of that comparison, and nowhere else
([Recovery](physics/recovery.md#against-rocketpy)).

| model | analytic | published source | another code | real flights |
|---|---|---|---|---|
| [Frames](physics/frames.md) | ✓ | no | ✓ RocketPy | no |
| [Geodesy](physics/geodesy.md) | ✓ | ✓ | no | no |
| [Gravity](physics/gravity.md) | ✓ | ✓ | ✓ RocketPy | no |
| [The magnetic field](physics/magnetic.md) | no | ✓ | no | no |
| [Atmosphere](physics/atmosphere.md) | ✓ | ✓ | ✓ RocketPy, in the descents only | partial: its pressure altitude against two altimeters' readings of their own pressure, to 0.195 m (0.64 ft) and 1.321 m (4.33 ft, [report][real-report]) |
| [Wind](physics/wind.md) | ✓ | no | ✓ RocketPy, in the descents only | no |
| [Turbulence](physics/turbulence.md) | ✓ | no | no | no |
| [Design tree](physics/design.md) | ✓ | no | ✓ RocketPy, mass properties only | no |
| [Shapes](physics/shapes.md) | ✓ | no | no | no |
| [Mass properties](physics/mass.md) | ✓ | no | ✓ OpenRocket, the structure without motors, and the parts of its parts catalog | no |
| [Solid motors](physics/motor.md) | ✓ | no | ✓ RocketPy, ThrustCurve.org; OpenRocket for a curve file's own numbers | no |
| [Aerodynamics](physics/aero.md) | ✓ | ✓ Barrowman's examples; MIL-HDBK-762's drag example, fins left out: 6 of 12 within 10%, the body reading 6% to 10% low faster than sound | partial: drag and the normal force against RASAero II to Mach 2, the drag with the fins and finish guessed and 5% to 15% low faster than sound; and in whole flights, against a target | partial: in seven flights, four of the five apogees past 5% are consistent with its drag ([real flights](#real-flights), [report][real-report]); wind tunnel ✓: normal force, drag and boattails; the Arcas Robin's drag reads high at every speed |
| [Rigid-body flight](physics/flight.md) | ✓ | no | ✓ RocketPy, with the drag given; and on each code's own drag, against a target; OpenRocket on 53 configurations of its examples ([results](format/ork.md#the-simulators-flights-against-openrockets)) | partial: seven logged flights, as a whole: apogees 6.04% off on average, outside the 5% target ([real flights](#real-flights), [report][real-report]) |
| [Time integration](physics/integration.md) | ✓ | no | no | no |
| [Recovery](physics/recovery.md) | ✓ | ✓ | ✓ RocketPy | no (drop tests ✓) |
| [Staging](physics/staging.md) | ✓ ignition times, the mass step and momentum, against hand sums | no | ✓ OpenRocket: its two-stage, three-stage, cluster and air-start examples, every flight within 5% (three cluster apogees against OpenRocket's flight with no parachute, because its parachute opened before apogee) ([results](format/ork.md#staged-clustered-and-air-start-flights)) | no |
| [Moving mass](physics/moving-mass.md) | ✓ the mass properties and the static margin against hand calculations, and both momenta kept in free flight | no | no | no |
| [Released mass](physics/released-mass.md) | ✓ the mass properties after a release against hand calculations, and mass and both momenta kept across it in free flight | no | no | no |
| [Flight metrics](physics/metrics.md) | ✓ the boost's peak, the margins of the page's finless rocket with a boattail (including where a margin is withheld) and the landing placement against hand calculations; max q and top Mach against a 1 ms record; the least margins against 1 ms steps and a scan of every step; the flight margin and the optimum delay against HPR Sim's own models and a flight with no recovery | no | no | no |
| [Fin flutter](physics/flutter.md) | ✓ the scaling in thickness, stiffness and pressure, and the margin at max q against a 1 ms record | ✓ Martin's formula and both of his worked examples reproduced, with his verdicts against his figure 3 band; not compared with flutter data | no | no |
| [Flight-log readings](physics/log-readings.md) | ✓ each reading of an invented flight within the bound its rounding and filter allow | no | partial: one public altimeter log's apogee, read at 1,010 ft against the 1,009 ft the altimeter's software states from the same trace; checked only where the log is fetched, not in CI | no |
| [Interpolation](physics/interpolation.md) | ✓ | no | no | no |
| [Quadrature](physics/quadrature.md) | ✓ | no | no | no |

## Results by model

The headline results, one check to a row. Each model page opens with *In short*, and its
verification section lists every test and its [tolerance](glossary.md#tolerance), how far a result
may be from its reference and still pass.

| model | compared with | how close |
|---|---|---|
| [Frames](physics/frames.md) | RocketPy's starting attitude, worked out from the launch rail's angles, for 8 rail setups | within 1e-12 rad |
| [Frames](physics/frames.md) | an exactly solvable spin whose axis sweeps round a cone (coning), over 1e6 integration steps | attitude within 1e-9 rad |
| [Geodesy](physics/geodesy.md) | the ellipsoid values printed in Table 3.5 of the [WGS 84](glossary.md#wgs-84) standard | to their printed digits |
| [Geodesy](physics/geodesy.md#distance-and-bearing-geodesics) | Karney's published test set of 500,000 WGS 84 geodesics, solved both ways | all 500,000 within Karney's 15 nm (nanometers): distance within 11.18 nm, far point within 14.02 nm, heading there within 13.99 nm; the computed bearing and distance, flown forward from the first place, land within 11.26 nm. CI checks every 500th line and the 21 mirror lines; the whole set where downloaded, measured on macOS |
| [A launch site's elevation from a file](elevation.md#how-the-file-reader-is-checked) | GDAL 3.12.2's reading, through rasterio 1.5.2, of seven GeoTIFF files cut from a USGS tile and of the whole tile ([the tests](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-io/tests/geotiff_rasterio.rs)) | the same corner and pixel size to the last bit, the same sums over all 13 million pixels, and the same pixel and value at all 2,368 of 2,800 places on the seven files (9 of them nodata) and all 1,696 of 2,000 on the tile, the rest off the edge; the whole tile only where downloaded |
| [Geodesy](physics/geodesy.md) | round trips at random points from −10 km (−6.2 mi) to +1000 km (621 mi): latitude, longitude and height to Earth-centered x, y, z, and back | latitude within 1e-14 rad, height within 2e-8 m |
| [Gravity](physics/gravity.md) | the WGS 84 formulas, worked to 40 digits by a separate script, at 11 points from the equator to both poles and up to 200 km (124 mi) high | gravity's strength within 2e-14 relative |
| [Gravity](physics/gravity.md) | RocketPy's gravity formula, at 8 points | under 1e-12 relative |
| [The magnetic field](physics/magnetic.md) | NOAA's test values for WMM2025: the report's 12 test points and NCEI's 100 high-precision points | the 12 to their last printed digit; the 100 to their last digit in declination, inclination, the east component and four rates. The north component differs at 97 points by up to 7.18e-4 nT, and the horizontal and total intensities by up to as much with it; the down component, moved by the same residue, by up to 2.2e-6 nT, and the rates of the north, horizontal and total components by up to 1.5e-6 nT a year. A test places the north component's difference in NCEI's file; the rates' difference has no known cause |
| [Atmosphere](physics/atmosphere.md) | the tables of the 1976 [standard atmosphere](glossary.md#standard-atmosphere), at 32 heights from −2 to 86 km (−1.2 to 53 mi) | every value within 0.1% |
| [Atmosphere](physics/atmosphere.md) | CIPM-2007 (Picard et al., 2008), a published reference formula for the density of humid air that treats air as a real gas, from 15 to 27 °C | humid-air density within 0.047% |
| [Atmosphere](physics/atmosphere.md) | RocketPy's air density, at the 23 heights its descents sample ([Recovery](physics/recovery.md#against-rocketpy)) | within 3.7e-4 relative |
| [Wind](physics/wind.md) | RocketPy's wind, at the same heights ([Recovery](physics/recovery.md#against-rocketpy)) | each component within 1e-9 m/s |
| [Wind](physics/wind.md) | the drift of RocketPy's four descents with wind ([Recovery](physics/recovery.md#against-rocketpy)) | the distance drifted within 0.28% |
| [Turbulence](physics/turbulence.md) | the [Dryden](glossary.md#turbulence-dryden) gust spectra, published formulas for how gust strength spreads over wavelength, over 2²⁰ random samples (about a million) | within 4 standard errors (the scatter expected by chance) in every octave band (a range of wavelengths spanning a factor of two): ±1–3% in the wide bands. Unvalidated for rockets |
| [Design tree](physics/design.md) | a rocket worked by hand, loaded, burning and burnt out | mass within 1e-12 kg, center of mass within 1e-12 m, inertia within 1e-12 relative |
| [Design tree](physics/design.md) | RocketPy, for eight cases of its example rockets, at the times its equation solver computed the burning grains (up to 60 per case) | mass, center of mass and inertia within 8.0e-10 relative (the center as a fraction of the rocket's length) |
| [Design tree](physics/design.md) | the same, at 103 even times through the burn and after it, where RocketPy interpolates between its solver's times | mass within 1.3e-5 relative, inertia within 2.6e-5 relative |
| [Design tree](physics/design.md) | the propellant mass left in the grains, at both sets of times | within 2.4e-9 of the initial propellant mass at the solver's times, and 4.9e-5 between them |
| [Shapes](physics/shapes.md) | exact formulas (closed forms) for filled noses and transitions | within 1e-10 relative |
| [Shapes](physics/shapes.md) | separately computed high-precision integrals, for 22 noses and transitions | within 1e-12 relative |
| [Shapes](physics/shapes.md) | the same kind of integrals, for 20 hollow shells of a given wall thickness | within 1e-10 relative |
| [Mass properties](physics/mass.md) | a cone, a tube, four fins and an off-axis payload, added up by hand | within 1e-11 relative |
| [Mass properties](physics/mass.md) | fin cross-sections, against exact numerical integration | within 1e-13 relative |
| [Mass properties](physics/mass.md) | material densities, converted from the units their sources print | the sources' values, such as white ash at 678 kg/m³ |
| [Mass properties](physics/mass.md#checked-against-openrocket) | [OpenRocket](glossary.md#openrocket) 24.12's structure (every stage, no motor), on 71 compared designs in the current scratch-excluding survey | mass within 1% on 70 and center of mass within 1% of length on 70, the one file outside with a named cause: airfoil fins OpenRocket weighs by a factor, a cause named only when weighing them its way brings the design within both thresholds; pitch inertia within 1% on 58, the 13 outside with no named cause yet but the two copies of OpenRocket's tube fin example (below); roll inertia a median 1.619% apart, which OpenRocket's shortcut for fins accounts for, and on the cluster designs its stacking of their tubes on the axis ([clusters](physics/mass.md#clusters-and-fillets)): with the shortcut in HPR Sim's place the median is 0.001%, and 12 files (8 distinct designs) remain outside 1% with a named cause. A ring of tube fins departs in both inertias, kept on purpose: OpenRocket's roll inertia for it is more than any mass inside the ring could have, and its pitch inertia leaves out how far the tubes sit from the axis, which accounts for the tube fin example's pitch inertia being 1.98% below OpenRocket's ([tube fins](physics/mass.md#tube-fins)). HPR Sim's airfoil fins are 19.4% lighter than OpenRocket's, a departure kept on purpose ([fins](physics/mass.md#fins-rail-buttons-and-roll-inertia)). What a file leaves unsaid (a wall of no thickness, no material), which override wins, held to OpenRocket's on 32 probe designs, and each fin section and each kind of part alone on 50 more, packed parts among them ([packed parts](physics/mass.md#packed-parts)). Fin fillets, on 9 of those, agree to 1e-15 in the fillets' mass and center of mass (the test holds 1e-12), the whole probe's pitch inertia up to 0.64% apart ([fillets](physics/mass.md#fin-fillets)). An automatic radius inside a nose cone, on 14 more (13 of which HPR Sim flies), puts every part inside but one packed mass component ([#186](https://github.com/nrdptel/fusionspace-eridanus/issues/186)) at OpenRocket's mass to 1e-14 and station to 1e-15, as the test holds them; the nose cones' and the transition's own walls keep their earlier gaps, up to 3.9e-5 of mass and 2.5e-6 m ([the format guide](format/ork.md#inside-a-nose-cone-or-a-transition)). A tube fin set whose radius OpenRocket works out from the body, on 19 more, has OpenRocket's radius and wall within 1e-15 and every part's mass within 1e-14 but the one nose cone among those probes, within 5e-5 ([the format guide](format/ork.md#tube-fins-sized-from-the-body)). On those that ask what a file leaves unsaid: mass within 0.001%, center of mass within 0.001 mm (0.000039 in), bar an elliptical fin's 0.18%, and an attached tube that writes no thickness (−2.4% on the committed probe; measured in [ADR-061][adr-061], the `.ork` conventions decision). On the override probes: two rules kept as measured departures, and flags that disagree (4.7 mm (0.19 in)) ([probes](physics/mass.md#what-a-ork-leaves-unsaid-and-overrides)) |
| [Mass properties](physics/mass.md) | [OpenRocket](glossary.md#openrocket) 24.12's mass and center of mass of the parts in its [parts catalog](format/orc.md) that the builder makes (all 3,449 but four it refuses) ([test](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr/tests/catalog_openrocket.rs), [The builder](the-builder.md#the-masses-against-openrocket)) | tubes, rings, bulkheads, lugs, parachutes and streamers within 1e-14 of the mass; nose cones and transitions within 1e-3 of the mass, and their center of mass within 1e-3 of the part's length (largest 6.3e-4 and 9.7e-4); but four blunt hollow nose cones, up to 4.8e-3 heavier here and 1.7e-3 of their length apart in center, where the two codes define a wall differently; masses stated in ounces, 8.8 parts in 10 billion apart (OpenRocket's rounded ounce); and one streamer, whose stated mass OpenRocket ignores. A hollow part's shoulder weighs nothing in OpenRocket and has the part's wall here, and is taken out before comparing |
| [Solid motors](physics/motor.md) | [ThrustCurve.org](glossary.md#thrustcurveorg)'s own statistics code (total impulse, burn time, average and peak thrust), on all 32 bundled curves | within 1.8e-15 relative |
| [Solid motors](physics/motor.md#validation) | [OpenRocket](glossary.md#openrocket) 24.12's own reading of the same 32 bundled curve files (total impulse, peak thrust, the 5%-of-peak burn-time window and the curve's duration) | every one bit for bit equal. Its average thrust divides the window's own impulse by the window where HPR Sim divides the whole curve's, so HPR Sim's is +0.0107% to +0.3147% higher (median +0.0965%). Nothing else in the motor model is compared with OpenRocket |
| [Solid motors](physics/motor.md) | RocketPy's solid-motor model, on three bundled motors, at 203 times each | total mass and inertias within 7.9e-5 relative; the propellant's own mass and inertias within 1e-4 of their values at ignition |
| [Aerodynamics](physics/aero.md) | [Barrowman's](glossary.md#barrowmans-method) five worked examples, at Mach 0 (low speed): each rocket's [normal-force slope](glossary.md#normal-force-slope) and [center of pressure](glossary.md#center-of-pressure-cp) | every center of pressure within 1%. Every slope within 1% too, except the six-fin Recruiter's: +2.87% (+3.42% on its fins alone) |
| [Aerodynamics](physics/aero.md) | drag curves labelled [RASAero](glossary.md#rasaero-ii) in RocketPy's examples, at [Mach](glossary.md#mach-number) 0.3, with the fins and surface finish guessed because the curves don't record them | within 10% in four of seven cases; −18.3% for Cavour [power-on](glossary.md#power-on-and-power-off-drag) (motor burning), cause open |
| [Aerodynamics](physics/aero.md) | Valetudo's drag table, which is 1.44 times the drag in the [OpenRocket](glossary.md#openrocket) export for the same rocket | −47.0% power-off and −50.4% power-on. Against the OpenRocket export, HPR Sim is 23.5% under as designed here, and 1.9% under with the export's own surface finish and launch lugs |
| [Aerodynamics](physics/aero.md#drag-against-rasaero-ii-through-mach-2) | the same curves every 0.05 from Mach 0.1 to 2.0, as far as each reaches; Calisto's, the one real RASAero II export, to Mach 2 | Calisto within 10% at 15 of 15 subsonic Mach numbers, 3 of 7 transonic and 8 of 17 supersonic, where HPR Sim reads −14.9% to −5.1%, lowest at Mach 2 (−29.8% to −24.4% before the boattail's supersonic wave drag). Other plausible fins put 14 to 17 of the 17 within 10%, though none puts every row within it, so most of what is left is within the unrecorded inputs; part of it is HPR Sim's body, which reads low faster than sound against MIL-HDBK-762 too |
| [Aerodynamics](physics/aero.md#drag-against-mil-hdbk-762s-sample-calculation) | MIL-HDBK-762's worked drag example, a rocket whose every term the handbook calculates, from Mach 0.5 to 3.2: a calculation with every input known, not a measurement. Its fins are sharp-edged wedges, which HPR Sim can't represent, so their pressure drag is left out on both sides | 6 of 12 within 10%. From Mach 0.9 to 1.2, +12.3% to +31.9%, mostly the nose and the base; from Mach 1.6, −6.0% to −9.6%, friction and the base |
| [Rigid-body flight](physics/flight.md) | the exact motion of a tumbling, spinning rocket in a vacuum, over 22 s | the center of mass within 1.7e-6 m of the exact parabola |
| [Aerodynamics](physics/aero.md) | NASA's wind-tunnel tests of the half-scale Arcas Robin and a longer version, Mach 0.6 to 4.63: [normal-force slope](glossary.md#normal-force-slope) and center of pressure, 22 readings at 12 Mach numbers ([fixture][nf-fixture]) | from Mach 1.5, every slope within target: +8.8% to −3.3% (short) and +9.4% to −2.3% (long), since the committed designs fly the shock-expansion method to their base; the center of pressure within 0.53 [calibres](glossary.md#calibre-caliber), which misses the half-calibre target on the long model at Mach 1.8 and 2.3, where that model's body alone reads 15% to 19% high (fins off, the short model's reads as much as 38% high at Mach 1.5: the crossflow excess sized by [M1.8e6](decisions-and-roadmap.md#m1-8e6)); from Mach 0.8 to 1.2, 2 of 9 within 15% and half a calibre |
| [Aerodynamics](physics/aero.md) | RASAero II's normal-force slope and center of pressure for Calisto, Mach 0.1 to 2.0 ([fixture][nf-fixture]) | within 15% and half a calibre at 11 of 15 Mach numbers; HPR Sim's slope rises with Mach through subsonic flow where RASAero II's stays flat (+21.9% at Mach 0.9), and from Mach 1.5, where its von Kármán nose flies the shock-expansion method behind a [Newtonian cap](physics/aero.md#blunt-tips), reads +8.3% to +12.9% |
| [Aerodynamics](physics/aero.md#checking-the-shock-expansion-method) | the body faster than sound, by a method no flight uses yet: the tables of NACA TN 3527, its source, for 144 cone- and ogive-cylinders at Mach 3 to 6.28, both its authors' own values and their wind-tunnel measurements of the normal-force slope and center of pressure ([fixture][se-fixture]) | against the measurements, 117 of 120 slopes within ±0.2 per radian (−0.278 to +0.251) and 109 of 120 centers of pressure within 0.2 calibres (−0.540 to +0.328); against the authors' values, 102 of 144 slopes within 0.05 (−0.134 to +0.146) and 125 of 144 centers of pressure within 0.1 calibres (−0.670 to +0.257), so the targets are not met, where a separate implementation of the same equations, an uncommitted script, agrees with HPR Sim outside the 12 rows at the method's limit ([#81](https://github.com/nrdptel/fusionspace-eridanus/issues/81)) |
| [Aerodynamics](physics/aero.md#checking-the-shock-expansion-method) | the same method on the Arcas Robin's nose and cylinder, against its measured body alone, Mach 1.5 to 4.63, 11 readings; no target, since the measurement includes the boattail and crossflow ([fixture][se-fixture]) | the short model within 5% from Mach 1.8 to 2.96, +16.4% at 1.5, −15.0% and −18.7% at 3.96 and 4.63; the long model −13.7% to −26.4% |
| [Aerodynamics](physics/aero.md#blunt-tips) | a blunt or vertical nose tip's Newtonian cap ahead of the same method: NASA TN D-4865's sphere-cone (a nose radius of 0.175 diameters on an 11.5° cone), its measured normal-force slope and center of pressure at Mach 1.50 to 4.63, 6 readings; and the Arcas Robin's committed power-series nose on its cylinder and boattail, the lip left off, against its measured body alone, 11 readings; no target ([fixture][bt-fixture]) | the sphere-cone like for like (at its plotted angles, body lift included) −1.2% to +32.1%, high from Mach 2.96, where the report's own method reads −3.3% to +13.6% overall, and the center of pressure within 0.06 diameters; the Arcas Robin like for like −4.8% to +37.2%, below the fitted secant ogive's +3.4% to +41.0% at every Mach number and nearer the tunnel at nine of its eleven rows. No measurement checks a tip that isn't spherical, and a nose that is nearly a cone but for a vanishing tip carries an unmeasured bias ([issue #101](https://github.com/nrdptel/fusionspace-eridanus/issues/101)) |
| [Aerodynamics](physics/aero.md#a-lip-in-a-boattails-wake) | the lip at the Arcas Robin's base, a reflexed flare behind its boattail, which HPR Sim gives no normal force faster than sound: what the measured fins-off pitching moment says the lip's share is, 11 readings at Mach 1.5 to 4.63; no target ([fixture][lip-fixture]) | one share fitted to every row is +0.021 ± 0.019 per radian, so slender-body theory's 0.178 sits 8.4 standard errors above it; fitted to each model alone it is −0.016 ± 0.022 (short, whose six rows disagree among themselves, χ² per degree of freedom 6.4) and +0.108 ± 0.034 (long, whose five agree, 0.9). The number blames the lip for every miss in the center of pressure, so it rejects slender-body theory's 0.178 but cannot separate zero from Seiff's embedded Newtonian bound (0.044 down to 0.014) |
| [Aerodynamics](physics/aero.md#what-a-marched-flare-is-worth) | NASA TN D-4865's model 2, the one flared body in the sources whose normal force and pitching moment are printed: a blunt 2.75° cone with an 18.5° flare, Mach 1.50 to 4.63, six readings, three of them with its boundary layer separated; no target ([fixture][flare-fixture]) | like for like (at its plotted angles, body lift included) the normal-force slope −1.9% at Mach 1.90, +7.0% at 2.30, +13.4% at 2.96 and +51.5% and +50.4% at 3.95 and 4.63, with the center of pressure within 0.05 calibres through Mach 2.96 and 0.088 at 3.95. The report's shadowgraphs show that flare's boundary layer separated from Mach 2.96 up, though the cost only shows in the two fastest rows: the unflared model 1 reads +29.7% and +32.1% there against +12.5% at 2.96, so the flare itself adds 21.7 and 18.3 percentage points at 3.95 and 4.63 and between −1.9 and +0.9 below. Below about Mach 1.5289 there is no marched reading at all and a flared body falls back to slender-body theory, carried up over 0.3 Mach by the join: drawing the flare out to the steepest turn its shock holds lands past the steepest the march itself takes |
| [Aerodynamics](physics/aero.md#the-body-alone-against-the-15-target) | the Arcas Robin's body alone, fins off, against the 15% target the milestone set before the work began, 11 readings at Mach 1.5 to 4.63, the same bodies as the row above, with the lip in place, which moves them under a point; the boattail cap does not reach these, whose boattail is 15° ([fixture][body-gap-fixture]) | not met on six rows: the short model +37.7% at Mach 1.5, +25.9% at 1.8, +16.9% at 2.3 and 2.96, and the long +19.4% at 1.8 and +15.5% at 2.3; at Mach 3.96 and 4.63 both are within 5%. Where the miss sits can only be told so far: the measurement's own fit trades its slope at `α → 0` against its curvature at a correlation of −0.96. At `α → 0` HPR Sim is within 1.5 standard errors on every row outside, and on five of the six most of the gap is in the curvature body lift adds (1.3 to 2.8 times the measured, and ×12.20 at Mach 1.8 where the tunnel's curve barely bends); on the sixth, the short model at Mach 2.96, 77% of the gap is HPR Sim's own `α → 0` slope |
| [Aerodynamics](physics/aero.md) | NASA's wind-tunnel tests of the same two models, Mach 0.6 to 4.63: drag on the forebody (the models' bases sat on a sting), fins on and off, 44 readings ([Aerodynamics](physics/aero.md#drag-against-the-arcas-robin-wind-tunnel)) | 2 of 44 within 10%, and every reading high. With the fins off, +13.5% to +24.1% from Mach 1.5 and +12.0% to +54.1% below, most of it the models' 15° boattail, which HPR Sim over-predicts in a thick boundary layer. With the fins on, from Mach 1.5, +39.4% to +154.0%, where HPR Sim's fins' drag stays near 0.30 and the measured falls to 0.046. HPR Sim's base drag behind a plain cylinder is not measured by the tunnel and has been checked at no speed faster than Mach 0.3 |
| [Aerodynamics](physics/aero.md#roll-forcing-and-damping) | NASA's wind-tunnel tests of the two Arcas Robin models, Mach 1.5 to 4.63: [roll forcing](glossary.md#roll-damping-and-roll-forcing), the rolling moment per degree of cant, 11 readings ([roll fixture][roll-fixture]) | from Mach 2.3, all 8 within 5.3%; at Mach 1.5 and 1.8, +14.3% to +47.8% |
| [Aerodynamics](physics/aero.md#roll-forcing-and-damping) | The Basic Finner's roll damping measured in a wind tunnel, Mach 1.5 to 3.0, and Barrowman's computed value at Mach 0.07 ([roll fixture][roll-fixture]) | −5.9% to −16.2% against the wind tunnel, lower as the Mach number grows; −2.0% against Barrowman's computed value (a theory curve, not a measurement; it confirmed how HPR Sim reads his method) |
| [Aerodynamics](physics/aero.md#boattails-faster-than-sound) | conical boattails measured in six NACA and NASA reports, jet off, Mach 0.3 to 3.24: the boattail's own pressure drag and the base pressure behind it ([drag fixture][drag-fixture]) | attached boattails of 3° to 10° from Mach 1.2: −21.9% to +28.3%, within 0.0123 (58 readings); from Mach 1.0 to 1.1, −18.2% to −5.4% (4), and −46.2% to +60.0% at points near Mach 1 their report calls questionable (27); through the rise from Mach 0.85 to 0.95, −77.5% to +7.6% (28); under Niskanen's rule to Mach 0.8, −100% to −83.5% (58). 16° in a boundary layer a fifth of the diameter thick, +26.4% to +54.2% (9). Separated 30° and 45°, −2.8% to +6.6% (3, which set the separation angles). The base drag behind them within 0.0102 of the measured on the cylinder's area, though behind small bases that is up to about 40% of the base's own drag (8 of the 12 set the ratio below Mach 2.5) |
| [Aerodynamics](physics/aero.md#tube-fins-against-openrocket) | [OpenRocket](glossary.md#openrocket) 24.12's normal-force slope and center of pressure for tube fins, on 14 probe designs and its *Tube fin rocket*, at Mach 0.05 to 0.75 ([ADR-102][adr-102], the decision that measured it); a code-to-code comparison. Tube fins as a whole are unvalidated: no measurement of a tube fin's lift or center was found | not within the quarter calibre that [lesson L19](decisions-and-roadmap.md#l19) asks: HPR Sim's center of pressure is 0.42 to 3.0 calibres forward of OpenRocket's on every probe up to Mach 0.5, and 1.07 on the *Tube fin rocket*. From Mach 0.6, where OpenRocket moves the tubes' center to their leading edge, 5 of 28 are within it. OpenRocket's tubes lift 1.26 to 1.86 times HPR Sim's, and up to Mach 0.5 it puts their center at a quarter of their length, ahead of which HPR Sim's lies. All 70 probe gaps, and the *Tube fin rocket*'s, are pinned by a test |
| [Rigid-body flight](physics/flight.md) | RocketPy's whole flights from the pad to the ground, for six rockets, one past Mach 1, both codes flying one declared drag coefficient | heights, speeds, times and accelerations within 3% ([below](#whole-flights-against-rocketpy)), the largest +1.783% in the [report][report]; the path too, except the drifts of Juno III, Bella Lui and Prometheus 2022 in wind and NDRT 2020's apogee drift, reported, not scored, as measured differences between the models ([ADR-026][adr-026]) |
| [Rigid-body flight](format/ork.md#the-simulators-flights-against-openrockets) | [OpenRocket](glossary.md#openrocket) 24.12's calm flights of the 54 configurations of its examples that HPR Sim flies, each figure by OpenRocket's own definition: apogee, largest speed, and the [stability margin](glossary.md#stability-margin) at rod clearance ([report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/openrocket-flights.md)); a code-to-code comparison, with no target set. OpenRocket aborts one, so the figures are over the other 53, and that one is compared at two points up to the abort ([flights OpenRocket aborted](format/ork.md#flights-openrocket-aborted)) | margin within 0.016 calibres on 41 of 53, taken with the air along the axis as OpenRocket's is; the *Tube fin rocket*'s is 1.08 calibres below OpenRocket's ([tube fins](physics/aero.md#tube-fins)), the three-stage example's three are 0.039 to 0.058 below, cause not yet sized ([#185](https://github.com/nrdptel/fusionspace-eridanus/issues/185)), and the five of *Pods--airframes and winglets* are 0.071 to 0.076 above, the flattering side ([#325](https://github.com/nrdptel/fusionspace-eridanus/issues/325) and [#326](https://github.com/nrdptel/fusionspace-eridanus/issues/326), two open causes in the fins), and the three of *Pods--powered with recovery deployment* are 0.070 above, cause not yet traced. With no named cause, apogee −4.34% to +1.95% (36 flights), and largest speed −0.69% to +6.10% (51 flights). The two-stage, three-stage, cluster, air-start and parallel-booster examples are within 5% in apogee and largest speed on every flight, the bar the [M1.9c](decisions-and-roadmap.md#m1-9c) milestone set before measuring; three cluster apogees and two three-stage ones are compared with OpenRocket's flight with no parachute, since its parachute opened before apogee. One named cause moves the apogee more than 5%: OpenRocket's parachute opening while the rocket still climbs (HPR Sim's flight compared here flies none), up to +13.80%. OpenRocket flying those six again without it brings four within 5% ([M2.2e4](decisions-and-roadmap.md#m2-2e4), sizing the causes). The other two are *Base drag hack (short-wide)* on a D12-3 and an E12-4, whose part set to no drag now flies as OpenRocket flies it ([M4.5h](decisions-and-roadmap.md#m4-5h), a part's drag override): against OpenRocket's flight with nothing deployed they read +5.84% and +8.92%, and within 0.10% when HPR Sim flies OpenRocket's own drag, so their gap is the drag coefficient, by OpenRocket's breakdown the very blunt nose's. With recovery in both, the basis [M4.5o](decisions-and-roadmap.md#m4-5o) judged, the D12-3 is within 5% (+4.52%) and only the E12-4 is over (+7.80%). HPR Sim keeps a value for that nose read between Hoerner's measured round heads, so the gap stays ([ADR-173](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0173-a-blunt-ellipsoid-s-subsonic-pressure-drag-from-hoerner.md), [#177](https://github.com/nrdptel/fusionspace-eridanus/issues/177)). A seventh, the *Tube fin rocket*, reads +6.95%, and within 0.03% when HPR Sim flies OpenRocket's own drag, so the gap is HPR Sim's tube-fin drag, which probably reads low ([#228](https://github.com/nrdptel/fusionspace-eridanus/issues/228)). The flight OpenRocket aborts, *Pods--powered with recovery deployment*'s first configuration, is within 5% at both points (height −1.17% and −1.18%, speed −0.04% and +4.02%), weaker evidence than an apogee ([M4.5n](decisions-and-roadmap.md#m4-5n)) |
| [Rigid-body flight](format/ork.md#the-simulators-flights-of-the-private-designs) | [OpenRocket](glossary.md#openrocket) 24.12's calm flights of the 35 configurations of 11 private designs that HPR Sim flies, published only as differences under anonymised ids ([report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/openrocket-library-flights.md)); a code-to-code comparison, with no target set | covers 11 of the 12 private designs. Apogee −4.84% to +13.60%. One is more than 5% off: the one supersonic flight, `C06/1`, +13.60% in apogee and +7.98% in largest speed, whose cause is the drag coefficient: flown on OpenRocket's own drag, HPR Sim reads +1.11% and +1.04%; the [supersonic pressure drag](glossary.md#wave-drag) (mostly wave drag) and HPR Sim's unvalidated base drag under power are open ([#222: HPR Sim's supersonic pressure drag is about twice OpenRocket's](https://github.com/nrdptel/fusionspace-eridanus/issues/222)). The rest: largest speed −0.65% to +2.28%. Margin −0.0166 to +0.1108 calibres. HPR Sim calls four designs clearly more stable than OpenRocket does: `C09` by 0.0350 to 0.0478 and `C03` by 0.0564 to 0.0730 on every flight, cause not yet traced ([#172](https://github.com/nrdptel/fusionspace-eridanus/issues/172)); `C08`, a two-stage design, by 0.1108, a center-of-mass gap with a lead but no measured cause ([#186](https://github.com/nrdptel/fusionspace-eridanus/issues/186)); and `C06/1` by +0.0604, most of it a center of mass 0.0447 calibres forward of OpenRocket's, with a lead but no measured cause: its airfoil fins, which OpenRocket weighs by a factor of 0.85 (the [mass survey's fin-section cause](physics/mass.md#checked-against-openrocket), sized on the structure alone). A fifth, `C12`, launched from a tilted rod, reads 0.0125 to 0.0366, most of it OpenRocket's rocket clearing the rod at an angle of attack: at that angle HPR Sim reads 0.0033 to 0.0074 ([a tilted launch rod](format/ork.md#a-tilted-launch-rod)) |
| [Time integration](physics/integration.md) | a separate line-by-line transcription of `DOPRI5`, the published Fortran integrator by Hairer and Wanner that HPR Sim's [Dormand–Prince](glossary.md#dormandprince-and-rk4) stepper follows, on the problem Hairer's own example program for `DOPRI5` solves: the Arenstorf orbit, the closed, looping path of a small body pulled by two large ones that circle each other | the same step counts |
| [Time integration](physics/integration.md) | a vertical flight with drag that has an exact solution | apogee, deployment and landing times within 1.5e-8 s |
| [Recovery](physics/recovery.md) | RocketPy's descents under a parachute, for five rockets | every descent metric within 3% ([below](#the-descent-under-a-parachute-against-rocketpy)) |
| [Recovery](physics/recovery.md) | the same descents: the heights where the later parachutes fire, and the descent rate under the drogue | within 0.17% and 0.01% |
| [Recovery](physics/recovery.md) | published drop tests of five small models falling with nothing deployed ([tumbling](glossary.md#tumble-recovery)) | descent rate −10 to +19% off |
| [Recovery](physics/recovery.md) | Kidwell's [streamer](glossary.md#streamer) drop tests (2001) | descent rate +9% fast for his one flat streamer, and +58% fast for one folded into pleats, which HPR Sim doesn't model |
| [Interpolation](physics/interpolation.md) | the exact formula of a smooth curve (a spline) through three points, `y = 3x/2 − x³/2` | matched |
| [Interpolation](physics/interpolation.md) | property tests, which check a rule on many randomly generated tables | every table passes exactly through its own points |
| [Quadrature](physics/quadrature.md) | polynomials, whose integrals are known exactly | exact up to degree 22 |
| [Quadrature](physics/quadrature.md) | six test integrals with known answers, one of them infinite at an end | within 1e-11 relative |

## The descent under a parachute, against RocketPy

This is one of the two comparisons the validation harness runs; the other is
[whole flights](#whole-flights-against-rocketpy). The harness is the program behind
`cargo xtask validate`, which flies every [validation case](glossary.md#validation-case) and
writes the committed report.

Five of RocketPy's [example rockets](glossary.md#example-rockets) are flown down in both codes,
set up the same way:

- the same starting state near apogee, with the first parachute opening at once;
- the same [drag areas](glossary.md#drag-area), deployment triggers and wind;
- the random noise RocketPy can add to each parachute switched off, so its runs repeat exactly;
- RocketPy's gravity formula, and its way of interpolating the wind (by its east and north
  components), in place of HPR Sim's own defaults, to compare like with like.

The committed [validation report][report] gives the six numbers below for each descent. All of
them were scored and all are within tolerance; the largest difference is +2.865%.

Each metric must agree within 3% of RocketPy's value, with no absolute floor (a fixed allowance,
in meters or seconds, that would pass any smaller difference). Each case file argues why, for
example [NDRT's][ndrt-case]. The metrics:

- `descent_time_s`: the time from the shared start to landing.
- `impact_speed_m_s`: the vertical speed at landing; the wind adds to the speed over the ground.
- `mean_descent_rate_m_s`: the start's height over the descent time, so it repeats the descent
  time in another form.
- `drift_m`, `drift_east_m` and `drift_north_m`: how far the rocket lands from where the descent
  started, not from the pad, and that distance's east and north parts. A drift to the south or
  west is negative.

A difference is HPR Sim's value less RocketPy's, over RocketPy's. So a positive one means HPR Sim's
value is larger in size, in the same direction: NDRT drifts south, and HPR Sim carries it further
south.

Every result of the report, as HPR Sim's difference from RocketPy:

| case | `descent_time_s` | `impact_speed_m_s` | `mean_descent_rate_m_s` | `drift_m` | `drift_east_m` | `drift_north_m` |
|---|---|---|---|---|---|---|
| [`descent-calisto-tests-motor-at-minus-1.373`][report] | +0.077% | −0.029% | −0.077% | +0.075% | +0.074% | +0.080% |
| [`descent-valetudo`][report] | −0.019% | +0.004% | +0.019% | −0.889% | −0.889% | −1.766% |
| [`descent-ndrt-2020-nose-to-tail`][report] | +0.705% | +0.012% | −0.700% | +0.276% | +0.214% | +2.865% |
| [`descent-prometheus-2022-generic-motor`][report] | +0.083% | −0.030% | −0.083% | +0.083% | +0.086% | +0.082% |
| [`descent-juno-iii`][report] | −0.018% | −0.008% | +0.018% | −0.018% | −0.018% | −0.018% |

Each case is named after the RocketPy example it flies. Three names carry more:

- `descent-calisto-tests-motor-at-minus-1.373` is Calisto as RocketPy's own tests build it, with
  the motor at −1.373 m (−4.50 ft) in RocketPy's coordinates, where its getting-started notebook puts it at
  −1.255 m (−4.12 ft, [notes on RocketPy's example rockets][rocket-notes]).
- `descent-ndrt-2020-nose-to-tail` is the NDRT 2020 rocket, which RocketPy's example measures from
  the nose toward the tail ([notes on RocketPy's example rockets][rocket-notes]).
- `descent-prometheus-2022-generic-motor` is Prometheus 2022, whose motor RocketPy describes with
  its generic-motor model, which treats the propellant as a solid cylinder
  ([notes on RocketPy's example rockets][rocket-notes],
  [Recovery](physics/recovery.md#against-rocketpy)).

The Valetudo case flies RocketPy's own Valetudo example, not the flight on
[Getting started](getting-started.md). From the shared start, 800 m (2,625 ft) above the ground, it falls in
still air under the example's one drogue, with RocketPy's drag area of 0.4537 m², and lands at
17.627 m/s (57.83 ft/s). Getting started flies the same airframe from the pad, with its own drogue, a main
parachute and a 5 m/s (16 ft/s) wind ([case file][valetudo-case],
[Recovery](physics/recovery.md#against-rocketpy)).

In still air, Valetudo's drift, 0.19 m (0.62 ft), comes only from the Earth's rotation (the
[Coriolis acceleration](glossary.md#coriolis-acceleration)), and its north part is 19 µm. So a
small difference there is a large fraction ([Recovery](physics/recovery.md#against-rocketpy)).

The largest gap, NDRT's north drift, most likely comes from [added mass](glossary.md#added-mass):
RocketPy counts the air a canopy drags along, 15.9 kg for NDRT's main against the rocket's 20.8 kg,
and HPR Sim has no such term, so the two respond differently as the canopy opens
([Recovery](physics/recovery.md#against-rocketpy)). That explanation fits the size of the gap, but
no test has isolated it yet.

What this shows: the two codes agree on the physics of a descent. It says nothing about whether
either matches a real parachute on a real day.

## Whole flights against RocketPy

Six of RocketPy's example rockets are flown from the pad to the ground in both codes: Calisto,
Valetudo, NDRT 2020, Juno III, Bella Lui and Prometheus 2022. They are set up the same way
([ADR-021][adr-021], the whole-flight comparison):

- one declared [drag coefficient](glossary.md#drag-coefficient), a constant 0.5
  ([case file][juno-case]), on the same reference area;
- the example's launch rail, site, parachutes and motor, with the thrust curve flown as measured
  and no correction for the thinner air at the site, as RocketPy's examples fly it;
- RocketPy's gravity formula, standard atmosphere and frictionless rail, and a declared wind;
- the random noise RocketPy can add to each parachute switched off.

This is [same-drag](glossary.md#same-drag-and-predicted-mode) mode. It checks the equations of
motion, the motor and the air, not the drag. HPR Sim's own drag is compared
[below](#whole-flights-with-each-codes-own-drag).

**In short: how high, how fast and how long agree, and so does where the rocket goes, except for
rockets that leave the rail slowly in a wind.** The heights, speeds, times and accelerations of
six flights, one of them past Mach 1, and of three of them again in calm air, agree within the 3%
of each case's gate ([case file][juno-case]); the largest difference is +1.783%
([report][report]). Here *still air*
is an example flown with no wind (Valetudo's), and *calm air* a windy case flown again with its
wind switched off. The apogee and landing points agree too, within 2.2%, in every flight without
wind and for Calisto in wind ([ADR-026][adr-026]). Juno III and Bella Lui leave the rail slowly
in the wind, at a steep angle to the airflow. There HPR Sim's [body lift](glossary.md#body-lift),
which RocketPy leaves out, its later release from the rail and, for Juno III, its simpler fin
model put their drifts 10.195% to 38.158% from RocketPy's. Prometheus 2022's differ by −7.292% and
+4.505%, from body lift and the rail release. NDRT 2020's apogee drift differs by −4.333%, mostly from the rail
release. These seven drifts are reported, not scored. So the landing offset
that [M2.1](decisions-and-roadmap.md#m2-1) asks for is met except where the two codes' models
differ.

Each of fifteen numbers per flight must agree within 3% of RocketPy's, with no absolute floor, or
say in its case file why it is not scored. Each case file argues why, for example
[Juno III's][juno-case]. Two more numbers compare the whole trace; they are explained after the
tables.

The numbers are measured as RocketPy defines them, with one exception. Each code's solver advances
the flight in [time steps](glossary.md#adaptive-time-step) and keeps the state at each step's end.
RocketPy takes each maximum (top speed, top Mach, top acceleration) only at those step ends. HPR Sim
also searches between its own step ends for the true peak, so that its number does not depend on
where its solver happened to step. That search can only raise a maximum. Against HPR Sim's old
step-end readings it raised them by at most 6.3e-5 of themselves (NDRT 2020's top speed). How much
RocketPy's own step ends miss was not measured. Either way it is far inside the 3% gate
([ADR-023][adr-023], the decision that also sets how peaks are found in both modes).

The definitions:

- `apogee_agl_m` and `apogee_time_s`: the highest point, and when.
- `flight_time_s`: the time from ignition to landing.
- `max_speed_m_s` and `max_mach`: the top speed over the ground, and the top
  [Mach number](glossary.md#mach-number).
- `rail_exit_speed_m_s` and `rail_exit_time_s`: when the forward rail button reaches the top of
  the rail, and the speed then. HPR Sim's own rail-exit event waits for the last button, so the
  comparison finds RocketPy's instant instead.
- `burnout_altitude_agl_m` and `burnout_speed_m_s`: at the end of the thrust curve.
- `impact_speed_m_s`: the vertical speed at landing.
- `max_acceleration_power_on_m_s2`: the largest acceleration while the motor burns.
- `max_acceleration_m_s2` and `max_acceleration_time_s`: the largest over the whole flight, and
  when. For NDRT 2020 that is its main parachute opening, not a flight load
  ([case file][ndrt-flight-case]).
- `apogee_drift_m` and `landing_drift_m`: how far from the pad, along the ground, the apogee and
  the landing point are.

Speeds and accelerations are those of the rocket's [center of dry mass](glossary.md#center-of-dry-mass),
the point RocketPy's flight follows; HPR Sim's own output follows the center of mass of the loaded
rocket. Heights are measured from where that point starts, as RocketPy's are. A difference is
HPR Sim's value less RocketPy's, over RocketPy's.

The [validation report][report] scores all but eleven of the numbers of the nine flights (the
six, and Juno III, Calisto and Bella Lui again in calm air), and all of the scored ones are within
tolerance. The eleven are measured and reported but not scored, each for a reason written in its
case file (below). Every result of the report, as HPR Sim's difference from RocketPy:

| case | `apogee_agl_m` | `apogee_time_s` | `flight_time_s` | `max_speed_m_s` | `max_mach` |
|---|---|---|---|---|---|
| [`flight-calisto-tests-motor-at-minus-1.373`][report] | +0.052% | +0.103% | +0.123% | +0.015% | −0.120% |
| [`flight-valetudo`][report] | +0.112% | +0.238% | +0.233% | +0.035% | −0.026% |
| [`flight-ndrt-2020-nose-to-tail`][report] | +0.064% | +0.157% | +0.643% | +0.031% | +0.003% |
| [`flight-prometheus-2022-generic-motor`][report] | +1.208% | +0.652% | +0.830% | −0.006% | −0.235% |
| [`flight-juno-iii`][report] | +0.641% | +0.376% | +0.475% | +0.051% | −0.221% |
| [`flight-bella-lui`][report] | +0.342% | +0.201% | +0.229% | +0.016% | −0.071% |
| [`flight-juno-iii-calm`][report] | +0.085% | +0.073% | +0.066% | +0.015% | −0.123% |
| [`flight-calisto-tests-motor-at-minus-1.373-calm`][report] | +0.045% | +0.099% | +0.118% | +0.022% | −0.105% |
| [`flight-bella-lui-calm`][report] | +0.038% | +0.020% | −0.012% | +0.020% | −0.018% |

| case | `rail_exit_speed_m_s` | `rail_exit_time_s` | `burnout_altitude_agl_m` | `burnout_speed_m_s` | `impact_speed_m_s` |
|---|---|---|---|---|---|
| [`flight-calisto-tests-motor-at-minus-1.373`][report] | −0.010% | −0.072% | +0.019% | +0.019% | −0.020% |
| [`flight-valetudo`][report] | −0.002% | −0.100% | +0.048% | +0.042% | +0.009% |
| [`flight-ndrt-2020-nose-to-tail`][report] | −0.009% | −0.085% | −0.004% | +0.043% | +0.022% |
| [`flight-prometheus-2022-generic-motor`][report] | −0.014% | −0.033% | +0.600% | −0.007% | −0.013% |
| [`flight-juno-iii`][report] | −0.005% | −0.142% | +0.269% | +0.056% | −0.003% |
| [`flight-bella-lui`][report] | −0.013% | −0.029% | +0.141% | +0.018% | +0.021% |
| [`flight-juno-iii-calm`][report] | −0.002% | −0.137% | +0.042% | +0.020% | +0.001% |
| [`flight-calisto-tests-motor-at-minus-1.373-calm`][report] | −0.001% | −0.074% | +0.023% | +0.025% | −0.003% |
| [`flight-bella-lui-calm`][report] | −0.000% | −0.032% | +0.012% | +0.024% | +0.021% |

| case | `max_acceleration_power_on_m_s2` | `max_acceleration_m_s2` | `max_acceleration_time_s` | `apogee_drift_m` | `landing_drift_m` |
|---|---|---|---|---|---|
| [`flight-calisto-tests-motor-at-minus-1.373`][report] | +0.099% | +0.099% | −96.811% | −0.902% | +1.257% |
| [`flight-valetudo`][report] | +0.248% | +0.248% | +0.006% | −0.931% | −1.811% |
| [`flight-ndrt-2020-nose-to-tail`][report] | −0.020% | +83.059% | +0.197% | −4.333% | +1.627% |
| [`flight-prometheus-2022-generic-motor`][report] | +0.003% | +19.062% | +1.094% | −7.292% | +4.505% |
| [`flight-juno-iii`][report] | −0.197% | −0.197% | +0.001% | −38.158% | +36.678% |
| [`flight-bella-lui`][report] | +1.783% | +1.783% | +0.001% | −10.195% | −21.686% |
| [`flight-juno-iii-calm`][report] | −0.012% | −0.012% | +0.046% | −1.750% | −1.807% |
| [`flight-calisto-tests-motor-at-minus-1.373-calm`][report] | +0.108% | +0.108% | −96.811% | −0.246% | −0.415% |
| [`flight-bella-lui-calm`][report] | +1.778% | +1.778% | +0.001% | −1.155% | −1.791% |

The last two numbers compare the whole trace, not one point of it. The series height RMS
(`series_height_rms_m`) is the root mean square of HPR Sim's height less RocketPy's: square each
difference, average the squares, and take the square root. The series speed RMS
(`series_speed_rms_m_s`) is the same for speed. Both follow the center of mass without propellant,
at RocketPy's 120 series times, from ignition until HPR Sim lands. Both codes' clocks start at
ignition on the rail, so no time shift is fitted: a fitted shift would hide a real difference in the
burn or on the rail ([case file][juno-case]). The center of mass without propellant is the point
RocketPy's series records. Every time counts the same, so the long descent weighs most; a
difference during the burn shows in the burnout and top-speed numbers instead.

Exact agreement would give 0, so these two are given in meters and meters per second, not as a
percentage. Each is held to 3% of RocketPy's apogee (for height) or top speed (for speed). That is
[M2.1](decisions-and-roadmap.md#m2-1)'s 3% for one number, applied to the whole trace
([case file][juno-case]).

All nine flights pass, each well inside its bound. The largest height RMS is Prometheus 2022's,
35.350931 m (115.98074 ft) against its 110.3 m (362 ft) bound, about a third of it; its apogee is also the furthest off,
+1.208%. Body lift accounts for that too: RocketPy flown with HPR Sim's body lift and rail release
reaches 3723.8 m (12,217 ft), against HPR Sim's 3723.6 ([case file][prometheus-case]). Juno III's is 14.550622 m (47.73826 ft)
against 78.4 m (257 ft), about a fifth, and the other seven are at an eighth of theirs or less. The speed
RMS runs from 0.022685 to 1.553509 m/s (0.07443 to 5.09681 ft/s, [report][report]).

| case | `series_height_rms_m` | height bound, m | `series_speed_rms_m_s` | speed bound, m/s |
|---|---|---|---|---|
| [`flight-calisto-tests-motor-at-minus-1.373`][report] | +1.837754 | 78.3 | +0.058007 | 7.3 |
| [`flight-valetudo`][report] | +2.115503 | 23.3 | +0.171627 | 3.3 |
| [`flight-ndrt-2020-nose-to-tail`][report] | +2.425247 | 36.4 | +0.124657 | 5.4 |
| [`flight-prometheus-2022-generic-motor`][report] | +35.350931 | 110.3 | +1.553509 | 10 |
| [`flight-juno-iii`][report] | +14.550622 | 78.4 | +0.810392 | 6.7 |
| [`flight-bella-lui`][report] | +1.657682 | 15.9 | +0.273966 | 2.9 |
| [`flight-juno-iii-calm`][report] | +2.003892 | 78.6 | +0.065810 | 6.8 |
| [`flight-calisto-tests-motor-at-minus-1.373-calm`][report] | +1.821296 | 78.4 | +0.051504 | 7.3 |
| [`flight-bella-lui-calm`][report] | +0.089668 | 16.2 | +0.022685 | 2.9 |

What the two codes still do differently, and what it moves:

- **In wind: body lift, the rail release and Juno III's fins.** A rocket that leaves the rail
  slowly in a wind meets the airflow at a steep angle: Juno III leaves at 18 m/s (59 ft/s) in an 8.5 m/s (28 ft/s)
  wind, 26° off it ([ADR-026][adr-026]). Three things differ there.
  - HPR Sim's normal force includes [body lift](glossary.md#body-lift), which grows with the square
    of that angle; RocketPy's does not. Much of it acts ahead of the rocket's center of mass, the
    nose's above all, so it moves the center of pressure forward and weakens the moment that
    [turns the rocket into the wind](glossary.md#weathercocking): HPR Sim turns into it less.
  - HPR Sim keeps the rocket guided until its last
    [rail button](glossary.md#rail-exit-and-rail-exit-velocity) leaves the rail; RocketPy frees it
    at the first.
  - Juno III's example gives its fins an airfoil lift curve, which RocketPy uses and HPR Sim cannot
    model. RocketPy's fin slope is 7.6% steeper than HPR Sim's flat-plate one ([ADR-026][adr-026]).

  Juno III's apogee is 245.3 m (805 ft) from the pad in HPR Sim and 396.6 m (1,301 ft) in RocketPy (−38.158%). Adding
  HPR Sim's choices to RocketPy one at a time moves RocketPy's to 360.7 m (1,183 ft) with HPR Sim's rail
  release, 286.5 m (940 ft) with its body lift too, and 248.3 m (815 ft) with its fin slope as well
  ([ADR-026][adr-026], measured by
  [`wind_response.py`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/oracles/rocketpy/wind_response.py)).
  Every windy drift lands within 1.3% of HPR Sim's the same way. Bella Lui's drifts are −10.195%
  and −21.686%, Prometheus 2022's −7.292% and +4.505% (within 0.1% of HPR Sim's once RocketPy has
  its body lift and rail release; [case file][prometheus-case]), and NDRT 2020's apogee drift
  −4.333%, mostly its rail release. These seven are reported but not scored, as measured
  differences between the models. Every other drift is scored and passes: Calisto's in wind,
  Valetudo's in still air, NDRT 2020's landing, and all six in calm air ([report][report],
  [case file][juno-case]).
- **RocketPy's own equations, corrected.** RocketPy 1.13.0 takes the turning moments during the
  burn about the wrong point: as far in front of the rocket's
  [center of dry mass](glossary.md#center-of-dry-mass) as the real center of mass is behind it.
  That makes its rockets too stable while the motor burns, so they turn into the wind too far. The
  fix is proposed in [a pull request to RocketPy](https://github.com/RocketPy-Team/RocketPy/pull/1196), still open, built on
  [one that is merged](https://github.com/RocketPy-Team/RocketPy/pull/1188) but not yet released. RocketPy 1.13.0 as installed still has the
  error; the comparison applies both fixes ([ADR-026][adr-026]). Without them, Juno III's apogee drift was
  582.4 m (1,911 ft), and HPR Sim's drifts in wind were up to −60.8% short of RocketPy's at apogee and +151%
  beyond it at landing (the question of
  [issue #50][issue-50]).
- **On the rail,** HPR Sim keeps the terms for the center of mass moving inside the body as the
  propellant burns, and RocketPy's rail equation leaves them out. At a sharp ignition spike, with
  thrust and mass the same to five digits, HPR Sim's acceleration is 1.2 to 1.3 m/s² higher. That is
  Bella Lui's +1.783%, whose peak is 7 ms after ignition ([report][report],
  [case file][bella-case]).
- **Calisto's two peaks.** Calisto's acceleration peaks twice, 0.9% apart: on the rail at 0.05 s
  and at 1.568 s. The same rail terms make HPR Sim's first peak the higher, so
  `max_acceleration_time_s` moves from one peak to the other (−96.811%); it is reported but not
  scored. The peak's size is scored, but its +0.099% compares two instants; at 0.05 s HPR Sim is
  1.05% higher ([report][report], [case file][calisto-case]).
- **The main opening.** RocketPy adds the air a canopy drags along
  ([added mass](glossary.md#added-mass)), and HPR Sim has none. So NDRT's peak deceleration as its
  main opens is +83.059% in HPR Sim, and Prometheus 2022's +19.062%, reported but not scored. Their
  times are scored, since both codes put them where the main opens ([report][report],
  [case file][ndrt-flight-case]).

**Prometheus 2022 flies through Mach 1.** RocketPy's flight peaks at Mach 1.013 and HPR Sim's at
1.010371 (−0.235%). Until [M1.8a](decisions-and-roadmap.md#m1-8a) HPR Sim stopped any flight at Mach
1, and the case was a known gap; with the normal force carried past Mach 1
([Aerodynamics](physics/aero.md#fins-through-mach-1)) it flies on its drag table to the ground.
Its scored numbers agree within 1.208% ([report][report], [case file][prometheus-case]). This flight
is a light test of the transonic normal force: no committed check measures its angle of attack
there, but a local probe found it below 0.11 degrees from Mach 0.8 to 1.2
([case file][prometheus-case]).

What this shows: with the drag given, the two codes agree on how high, how fast and how long a
rocket flies, and on where it goes, except for rockets that leave the rail slowly in a wind,
where their models differ. [M2.1](decisions-and-roadmap.md#m2-1)'s landing offset is met
everywhere else ([ADR-026][adr-026]). Which code is nearer the truth for those is for real flights
to say; the [real flights](#real-flights) so far compare heights, not drift.
Nothing here says anything about HPR Sim's own drag, which the next section compares, or about a
real flight. The comparison with OpenRocket is on its own page
([HPR Sim's flights against OpenRocket's](format/ork.md#the-simulators-flights-against-openrockets)), and the
real flights are below.

## Whole flights with each code's own drag

The same six flights again, in [predicted](glossary.md#same-drag-and-predicted-mode) mode: HPR Sim
flies its own drag, from each design's shape, where same-drag mode gives it the declared one; both
modes use HPR Sim's own [normal force](physics/aero.md), so only the drag differs. RocketPy flies the drag each of its examples ships, as RocketPy 1.13.0 flies the example:
a drag curve for Calisto, Valetudo and Juno III, a function of Mach for Prometheus 2022
([case file][prometheus-predicted-case]), a constant for NDRT 2020 (0.44,
[case file][ndrt-predicted-case]) and Bella Lui (0.43, [case file][bella-predicted-case]).
Everything else is set up as in the same-drag flights above ([ADR-023][adr-023], the
predicted-mode comparison).

**In short: HPR Sim's heights are within 3% of RocketPy's for Calisto (−0.609%), Bella Lui
(+0.971%) and Juno III (+2.097%), well above for Valetudo (+10.113%) and NDRT 2020 (+10.302%),
where its drag is well below the example's, and below for Prometheus 2022 (−7.280%), where its
drag is above the example's through the coast** ([report][report]). These are
results, not a pass or fail. Neither code's drag is the truth: each example's drag came from
RASAero, OpenRocket or its team's own estimate. So each number is compared with the same 3% as the
same-drag flights ([M2.1](decisions-and-roadmap.md#m2-1), the validation milestone), but only as a
[target](glossary.md#gate-and-target). A miss is reported and explained in its case file
([Valetudo's][valetudo-predicted-case] and [NDRT 2020's][ndrt-predicted-case], for example), and
does not fail the test suite. The report is committed, so any number that moves shows up in
review.

Every predicted result of the report, as HPR Sim's difference from RocketPy:

| case | `apogee_agl_m` | `apogee_time_s` | `flight_time_s` | `max_speed_m_s` | `max_mach` |
|---|---|---|---|---|---|
| [`predicted-calisto-tests-motor-at-minus-1.373`][report] | −0.609% | −0.567% | −0.288% | +0.363% | +0.232% |
| [`predicted-valetudo`][report] | +10.113% | +6.179% | +8.916% | +2.292% | +2.237% |
| [`predicted-ndrt-2020-nose-to-tail`][report] | +10.302% | +6.534% | +6.870% | +1.378% | +1.351% |
| [`predicted-prometheus-2022-generic-motor`][report] | −7.280% | −4.920% | −4.978% | +1.280% | +1.069% |
| [`predicted-juno-iii`][report] | +2.097% | +1.046% | +1.686% | +1.006% | +0.737% |
| [`predicted-bella-lui`][report] | +0.971% | +0.536% | +0.748% | +0.241% | +0.154% |

| case | `rail_exit_speed_m_s` | `rail_exit_time_s` | `burnout_altitude_agl_m` | `burnout_speed_m_s` | `impact_speed_m_s` |
|---|---|---|---|---|---|
| [`predicted-calisto-tests-motor-at-minus-1.373`][report] | −0.009% | −0.071% | +0.240% | +0.526% | −0.020% |
| [`predicted-valetudo`][report] | +0.029% | −0.109% | +1.326% | +2.883% | +0.009% |
| [`predicted-ndrt-2020-nose-to-tail`][report] | +0.000% | −0.087% | +0.723% | +1.532% | +0.022% |
| [`predicted-prometheus-2022-generic-motor`][report] | +0.001% | −0.036% | +1.227% | +1.778% | −0.013% |
| [`predicted-juno-iii`][report] | −0.001% | −0.143% | +0.772% | +1.129% | −0.003% |
| [`predicted-bella-lui`][report] | −0.011% | −0.029% | +0.270% | +0.296% | +0.021% |

| case | `max_acceleration_power_on_m_s2` | `max_acceleration_m_s2` | `max_acceleration_time_s` | `apogee_drift_m` | `landing_drift_m` |
|---|---|---|---|---|---|
| [`predicted-calisto-tests-motor-at-minus-1.373`][report] | +0.123% | +0.123% | +0.002% | −2.233% | +0.901% |
| [`predicted-valetudo`][report] | +0.250% | +0.250% | −0.030% | +12.215% | +10.437% |
| [`predicted-ndrt-2020-nose-to-tail`][report] | +0.656% | +83.059% | +9.646% | +12.259% | +12.648% |
| [`predicted-prometheus-2022-generic-motor`][report] | +0.848% | +19.062% | −6.903% | −17.642% | −6.167% |
| [`predicted-juno-iii`][report] | +0.105% | +0.105% | +0.002% | −36.078% | +40.482% |
| [`predicted-bella-lui`][report] | +1.797% | +1.797% | −0.029% | −9.390% | −20.805% |

The whole-trace numbers, in meters and meters per second, defined as for the same-drag flights
above. [Valetudo's][valetudo-predicted-case], [NDRT 2020's][ndrt-predicted-case] and
[Prometheus 2022's][prometheus-predicted-case] height RMS are outside the target, and so is NDRT
2020's speed RMS, for the same reason as their apogees: HPR Sim's own drag differs from those
examples' drag. Each bound is 3% of that case's own RocketPy
apogee or top speed, so it differs from the same-drag bound: Juno III's 54.699661 m (179.46083 ft) is inside its
83.9 m (275 ft) here ([report][report]).

| case | `series_height_rms_m` | height bound, m | `series_speed_rms_m_s` | speed bound, m/s |
|---|---|---|---|---|
| [`predicted-calisto-tests-motor-at-minus-1.373`][report] | +12.420364 | 84.6 | +0.331462 | 7.4 |
| [`predicted-valetudo`][report] | +75.375007 | 20.9 | +2.790876 | 3.2 |
| [`predicted-ndrt-2020-nose-to-tail`][report] | +116.136026 | 38.1 | +6.775468 | 5.5 |
| [`predicted-prometheus-2022-generic-motor`][report] | +263.920111 | 128.8 | +7.202224 | 10.3 |
| [`predicted-juno-iii`][report] | +54.699661 | 83.9 | +0.927234 | 6.8 |
| [`predicted-bella-lui`][report] | +5.264438 | 16.2 | +0.279185 | 2.9 |

Why the misses, largest first:

- **Valetudo and NDRT 2020 fly high: the drag.** HPR Sim's drag coefficient at Mach 0.3 is −47.0%
  from Valetudo's table, a hand-edited table 1.44 times the drag of the OpenRocket export for the
  same rocket ([Aerodynamics](physics/aero.md#verification)). For NDRT 2020 it is 0.318 against
  the example's constant 0.44 ([case file][ndrt-predicted-case]). These drags are compared at Mach
  0.3 only. Flown on the same drag, the apogees agree with RocketPy's to +0.112% and +0.064%
  ([report][report]). Less drag also means a
  later apogee, a longer descent and further to drift, which moves their times and drifts too.
- **HPR Sim's drag here is for the design as transcribed.** Where RocketPy's examples say nothing,
  the designs' fin thickness and edges and their surface finish are placeholders, so these results
  compare HPR Sim's drag for those designs, not for the rockets as built. For Valetudo, the rocket's
  own OpenRocket finish and launch lugs take HPR Sim's drag coefficient from 0.5566 to 0.714
  ([Aerodynamics](physics/aero.md#drag-verification)). So a miss here is not a gap for HPR Sim
  to close toward the example's drag.
- **The drifts of Juno III and Bella Lui in wind:** HPR Sim's body lift and rail release, and Juno
  III's fin slope, as in same-drag mode ([ADR-026][adr-026]).
- **Prometheus 2022 flies low: the drag again, the other way.** It passes Mach 1 on HPR Sim's own
  drag since [M1.8b1](decisions-and-roadmap.md#m1-8b1), the drag through Mach 1, peaking at Mach
  1.060 against RocketPy's 1.048. Its coasting drag rises to about 0.49 at Mach 0.8, where the
  example's falls to 0.30, so HPR Sim peaks −7.280% low and sooner, and its drifts and times follow.
  Flown on the same drag the apogees agree to +1.208% ([report][report],
  [case file][prometheus-predicted-case]).
- **NDRT 2020's and Prometheus 2022's peak deceleration** at their main openings, +83.059% and
  +19.062%, are the added-mass difference explained above. Their times move +9.646% and −6.903%
  with the apogee ([report][report], [case file][ndrt-predicted-case]).

What this shows: with its own drag, HPR Sim's heights differ from RocketPy's by −7.280% to +10.302%
([report][report]), and the larger gaps are the two drags differing, not the flight. It does not
say which drag is right; the [real flights](#real-flights) below start to, one rocket at a time.

## Real flights

This section covers [M2.3b](decisions-and-roadmap.md#m2-3b), RocketPy's logged flights
([ADR-082][adr-082], the decision). **In short: HPR Sim flew seven rockets whose teams logged their
flights, in the weather of the day. Read the way each log's altimeter reads the air, HPR Sim's
[apogees](glossary.md#apogee) miss the logs' by 6.04% on average, outside the 5% target; with its
sign the mean is −0.25%, so these seven show no bias high or low. Five miss by more than 5%. Each
has an explanation checked against the report's numbers: four are consistent with HPR Sim's drag,
and one with the motor's impulse ([report][real-report]).** Four of the seven altimeters' kinds are
assumed, none is calibrated, and the designs' fin edges are guesses, so read the numbers as those
seven flights', not as a bound on every rocket.

**What is flown.** [RocketPy](glossary.md#rocketpy)'s documentation flies more than a dozen
rockets against their teams' logs. HPR Sim flies seven of them the way a user would: the design
built from the example's masses and shapes, HPR Sim's own aerodynamics, and the example's own thrust
file, read as RocketPy reads it, to the same total impulse. It flies from the example's launch rail
and site, in the weather of the launch hour from the example's ERA5 file, a
[reanalysis](glossary.md#reanalysis) of the atmosphere ([ERA5 weather files](format/era5.md)).
Genesis and Lince were added to the public designs for this.

**What is left out, and why.** The sample is the documented flights HPR Sim can fly today:

| rockets | why not yet |
|---|---|
| Astra, Andromeda | their weather file is in a newer format that needs converting first ([ADR-082][adr-082]) |
| Camões, Erebus 11, Halcyon, Hedy | they fly their teams' own motors, outside HPR Sim's scope of commercial motors ([ADR-082][adr-082]) |
| Valetudo, Defiance | their logs give an apogee, not a climb ([ADR-082][adr-082]) |
| Valkyrie | its inputs are only in a data file ([ADR-082][adr-082]) |

Juno III flies its team's own motor too, but its design was already public, and HPR Sim flies the
thrust file as it would any other: nothing of the motor is modeled ([ADR-082][adr-082]).

The [private designs](format/ork.md#the-simulators-flights-of-the-private-designs) add none: none of them
is the rocket of a flight in the [private collection of logs](VALIDATION.md#real-flight-data), so
none has a log to compare with ([ADR-083][adr-083]). A private flight collection added on
2026-10-04 pairs designs with their flights' logs; its 55 compared flights are
[below](#real-flights-of-the-private-collection).

**Reading the logs.** Every log here comes from a [barometric
altimeter](glossary.md#barometric-altimeter), or is assumed to. Such an altimeter turns pressure
into the [standard atmosphere](glossary.md#standard-atmosphere)'s altitude. On a day warmer than the
standard it reads less than the height climbed, 6.5% less on a day 20 K warmer ([worked
example](physics/atmosphere.md#pressure-altitude-what-a-barometric-altimeter-reads)), and more on a
cold one. Two logs record their pressure, and their heights are that reading, to 0.195 m (0.64 ft) and 1.321 m (4.33 ft)
([report][real-report]). So HPR Sim's height is read the same way, from the ERA5 pressure at its
center of mass. The table gives HPR Sim's apogee both ways. The reading moves it from −7.9% (Juno
III, in June at Spaceport America) to +1.7% (NDRT 2020, in February) ([report][real-report],
[ADR-082][adr-082]).

The kind of four altimeters is assumed rather than known; each row of the
[report][real-report] gives its evidence. Reading all seven as heights would give a mean of
4.47%, but three logs are known to be barometric. Reading only the four assumed ones as heights
gives 6.63%, so the target is missed either way. Read that way, Lince (+7.99%) would miss by more
than 5% with no checked explanation, and Genesis would not ([report][real-report],
[ADR-082][adr-082]).

Two logs also carry satellite (GNSS) heights, a geometric reference. Their barometric apogees are
0.943 and 0.935 of the satellite ones, and HPR Sim's conversion makes its own apogee 0.932 and 0.921
of its height ([report][real-report]). That is the same direction and nearly the same size, 1 to
2 points lower on both, on the side of both flights' misses. Juno III's log is cut up to 20 m (66 ft)
below its apogee, which would put its own ratio as high as 0.941 ([ADR-082][adr-082]).

Three logs are cut by hand just before a pressure transient at their apogee, where the reading
jumps. Juno III's rises 62 m (203 ft) in 0.3 s as it levels off, and the team's reported apogee is that
spike. At the cut its own velocity column still reads 17.5 m/s (57 ft/s) up, so its apogee may be 10 m (33 ft) to
20 m (66 ft) low ([report][real-report]).

**What is compared.**

- **The apogee:** the log's highest reading, against HPR Sim's highest point read the same way. A
  plus means HPR Sim flies higher.
- **The climb:** the root mean square (RMS) of HPR Sim's height less the log's over the ascent, the
  report's *trace RMS*. A log's clock starts when its altimeter says so, not at ignition, so both
  clocks are set to zero where each first reaches 30 m (98 ft). The RMS runs from there to the first of
  the two apogees ([report][real-report]). The descent is not compared: which parachute opened,
  and when, was the team's.

The logs, thrust files and weather files are other people's data, so they stay out of the
repository. `cargo xtask real-flights` reads them from a pinned copy of RocketPy (fetched with
`cargo xtask refs fetch rocketpy`) and commits only the numbers ([report][real-report]).

| flight | log apogee (m) | HPR Sim apogee (m) | apogee error | HPR Sim as a height (m) | climb RMS (m) | error on the team's drag |
|---|---:|---:|---:|---:|---:|---:|
| [Bella Lui, EPFL, 2020][real-report] | 459.0 | 462.9 | +0.86% | 463.6 | 3.2 | +0.24% |
| [NDRT 2020, Notre Dame][real-report] | 1320.4 | 1457.6 | **+10.40%** | 1432.8 | 89.9 | +0.17% |
| [Prometheus, Western Engineering, 2022][real-report] ¹ | 3895.8 | 3549.3 | **−8.90%** | 3809.9 | 190.9 | +0.45% |
| [Juno III, Projeto Jupiter, 2023][real-report] | 3151.5 | 2923.4 | **−7.24%** | 3174.5 | 228.8 | −9.05% |
| [Cavour, Politecnico di Torino, 2023][real-report] | 2789.0 | 2946.0 | **+5.63%** | 3052.1 | 125.3 | −2.29% |
| [Genesis, EuRoC 2023][real-report] | 2916.7 | 2746.0 | **−5.85%** | 2875.3 | 95.2 | −0.08% |
| [Lince, EuRoC 2023][real-report] | 3587.7 | 3709.2 | +3.39% | 3874.3 | 63.9 | −12.20% |

¹ Flown in the weather of the same day a year later, 2023, as RocketPy's example flies it: no
file of the flight's day is available ([report][real-report]).

The mean absolute apogee error is 6.04%, against the 5% target of the
[validation plan][plan] ([gate and target](glossary.md#gate-and-target)). The largest climb RMS is
Juno III's, 7.26% of its apogee ([report][real-report]). No target is set on the climb. The
[census](#the-census) counts each climb against a bar of 3% of its apogee, the bound the RocketPy
comparisons hold a height RMS to; Bella Lui and Lince are within it ([census][census]).

**The last column is a diagnostic, not a prediction.** It flies each flight again with HPR Sim's
drag replaced by the drag the example's RocketPy notebook specifies: the team's estimate, a table, a
curve from RASAero II or a CFD analysis, or a constant the notebook gives no source for. When the
error falls inside 5%, the miss is consistent with HPR Sim's drag. Neither drag is the truth, and a
team may have tuned its drag to this very flight. On the teams' drag the mean absolute error is
3.50% ([report][real-report]). Where a notebook reshapes its thrust file to a stated burn time and
total impulse, as Juno III's does, a second diagnostic flies the file as recorded.

**The five flights past 5%.** Each explanation is a claim the report checks against its own
numbers. A change to HPR Sim that makes one false fails `cargo xtask real-flights --check`, which
runs only where the pinned RocketPy copy is. CI checks each claim against the committed numbers
([ADR-082][adr-082]).

- **NDRT 2020, +10.40%: consistent with HPR Sim's drag.** On the team's constant drag coefficient,
  0.44, the apogee is +0.17% ([report][real-report]). HPR Sim's own drag is lower, as it was against
  RocketPy flying the same constant.
- **Prometheus, −8.90%: consistent with HPR Sim's drag.** On the team's drag table the apogee is
  +0.45% ([report][real-report]). The barometric reading uses the temperatures of 24 June 2023,
  a year after the flight. Against the satellite heights, HPR Sim's conversion reads 1 to 2 points
  below the altimeter both here and on Juno III, which flew in its own day's weather, so the
  wrong day's share of the miss can't be told apart.
- **Juno III, −7.24%: consistent with the motor's impulse.** The notebook reshapes the team's
  own motor curve to 8800 N s, 4.9% less than the file. On the file as recorded (9251.7 N s, its
  negative end read as zero) the apogee is +1.13%, while on the team's drag it is −9.05%
  ([report][real-report]).
- **Cavour, +5.63%: consistent with HPR Sim's drag.** On the curves the team labels RASAero II the
  apogee is −2.29% ([report][real-report]). HPR Sim's drag is below those curves, 8.3% at Mach 0.3
  with the motor off and 18.3% with it burning ([Aerodynamics](physics/aero.md#drag-verification)).
- **Genesis, −5.85%: consistent with HPR Sim's drag.** On the team's curves the apogee is −0.08%
  ([report][real-report]).

These are consistent explanations, not proofs: the teams' drags, and the impulse a notebook sets,
are estimates too. Lince, inside the target on HPR Sim's drag, is −12.20% on its team's; that is not
investigated ([report][real-report]).

> **How far to trust it.** Measurements, but from altimeters not calibrated here: a barometer's error, the filter
> of the four that are filtered, and the assumed kind of four of them all sit in the reference.
> Drift and landing are not compared, nor speeds. HPR Sim's designs of these rockets have placeholder
> fin edges and surface finish, which move its drag.

### Real flights of the private collection

This covers [M2.3c1](decisions-and-roadmap.md#m2-3c1), the logged apogees of a private collection
([ADR-184][adr-184], the decision). **In short: 55 real flights, each a design paired with the log
of the same flight, were flown by HPR Sim and by [OpenRocket](glossary.md#openrocket) in the weather
of their day. Both over-predict: HPR Sim's [apogees](glossary.md#apogee) are +9.83% above the logs
on average (mean absolute 13.96%), OpenRocket's +9.00% (13.08%). HPR Sim and OpenRocket agree with
each other within 5% on 53 of the 55 ([report][fixture-report]). Neither meets the 5% target.**
The [report][fixture-report]'s histogram gives the low side, the one a waiver depends on: its bins
below zero hold the flights whose simulated apogee read under the log. Each logged apogee is the
height climbed in the day's air, the altimeter's reading corrected for the weather as the report
sets out.
HPR Sim and OpenRocket agree with each other far better than with the logs, so the gap is not a
difference between the two codes; what causes it is not shown here. The flights' owners allow only
aggregate statistics, so no flight is named here, and the logs' readings were taken by hand from
free text.

**What is flown.** The collection pairs each design as flown with its own flight's log, the
motor, date, site and the [ERA5](glossary.md#reanalysis) weather of the day
([ADR-151](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0151-hpr-sim-fixtures-joins-the-reference-library-as.md)).
Of its 89 best-documented (tier A) flights, 83 have an OpenRocket `.ork` design
([report][fixture-report]). HPR Sim flies the whole ERA5 profile of the launch hour: temperature,
pressure and wind at every level. OpenRocket 24.12 can take only a temperature and pressure at
the pad, with the standard atmosphere above, so it gets those, and the same wind every 50 m (164 ft). Both
fly OpenRocket's curve for the motor the file names, from the same rail, with no parachute
opened, so each apogee is the climb's. A third row flies HPR Sim in OpenRocket's air, which shows
how much of the difference between the codes is the atmosphere.

**What each log says.** Every log here is a [barometric
altimeter](glossary.md#barometric-altimeter)'s. Such an altimeter turns pressure into the
[standard atmosphere](glossary.md#standard-atmosphere)'s altitude and subtracts the pad's, so on a
warm day it reads less than the height climbed: 6.5% less on a day 20 K warmer than the standard
([worked example](physics/atmosphere.md#pressure-altitude-what-a-barometric-altimeter-reads)).
The comparison turns each reading back into a height through the day's ERA5 air. Across these
flights the heights came out 1.0361 times the readings on average, from 0.9601 to 1.0777 times
([report][fixture-report]). Where a log's highest value was an ejection charge's pressure spike,
the reading before the charge was taken; 24 of the 83 readings differ from the collection's own
figure by more than 0.5%, each with its reason recorded ([ADR-184][adr-184]).

| [report][fixture-report] | flights |
|---|---:|
| [tier A, from a `.ork`][fixture-report] | 83 |
| [flown by HPR Sim][fixture-report] | 61 |
| [flown by OpenRocket 24.12][fixture-report] | 79 |
| [flown by both][fixture-report] | 61 |
| [left out: an airbrake or canard acted on the climb][fixture-report] | 6 |
| [compared with the log][fixture-report] | 55 |

| [over 55 flights][fixture-report] | mean | median | mean absolute | within 5% | within 10% |
|---|---:|---:|---:|---:|---:|
| [HPR Sim][fixture-report] | +9.83% | +9.47% | 13.96% | 10 | 27 |
| [OpenRocket 24.12][fixture-report] | +9.00% | +6.82% | 13.08% | 9 | 26 |
| [HPR Sim in OpenRocket's air][fixture-report] | +9.99% | +9.67% | 14.07% | 10 | 26 |
| [HPR Sim against OpenRocket][fixture-report] | +0.69% | +0.55% | 1.72% | 53 | 55 |

Each error is the simulated apogee less the logged one, over the logged one, in percent, so a
positive error is a prediction above the log. The mean absolute error averages the errors without
their signs. The last row is HPR Sim's apogee less OpenRocket's, over OpenRocket's, so a positive
value means HPR Sim flies higher. "Within 5%" and "within 10%" count the flights whose error is
strictly smaller than that, either way ([report][fixture-report]).

**Early deployments.** On 19 of the 55, a parachute charge fired or the rocket separated before
the highest reading, while it was still climbing ([report][fixture-report]). The readers' notes
put each at under 1.5 s before the peak and a few meters of climb, so the apogee barely moves
([ADR-184][adr-184]). Without them, over 36 flights, HPR Sim is +5.88% (mean absolute 11.94%) and
OpenRocket +5.29% (11.24%). The two sets differ in which rockets they hold more than in what the
early charges cost.

**What is not flown, and why.** Of the 83, HPR Sim flies 61, and each cause of the rest has an open
issue ([report][fixture-report]). Most are small reading problems in HPR Sim's `.ork` importer that
OpenRocket accepts: a fin tab a few tens of microns past its root (6 flights, [#357](https://github.com/nrdptel/fusionspace-eridanus/issues/357)), a
fin outline with a repeated point (3, [#358](https://github.com/nrdptel/fusionspace-eridanus/issues/358)), rail buttons spaced forward (1,
[#359](https://github.com/nrdptel/fusionspace-eridanus/issues/359)), unread surface
finishes (3, [#360](https://github.com/nrdptel/fusionspace-eridanus/issues/360)), two override flags read as one (1, [#361](https://github.com/nrdptel/fusionspace-eridanus/issues/361)), and a part in
an off-axis inner tube (2, [#181](https://github.com/nrdptel/fusionspace-eridanus/issues/181)). Three fly motors whose curve OpenRocket's database
lacks ([#362](https://github.com/nrdptel/fusionspace-eridanus/issues/362)). In three more the collection itself falls short: two designs don't hold
the flown motor ([#363](https://github.com/nrdptel/fusionspace-eridanus/issues/363)) and one flight's day has no weather file ([#364](https://github.com/nrdptel/fusionspace-eridanus/issues/364)).
OpenRocket flies 79, missing the motor in one ([#362](https://github.com/nrdptel/fusionspace-eridanus/issues/362)).

> **How far to trust it.** Logged measurements, read from free text and logs by hand, and checked for
> form, not cross-checked flight by flight by a second reader. Of the 83 flights, 26 have no known
> launch hour and fly at local noon ([report][fixture-report]); the altimeters are not calibrated; the designs are
> what their fliers drew, whose masses some fliers measured and some did not. Both codes
> over-predicting alike is common in hobby rocketry, where the real airframe carries rail buttons,
> paint and joints a design leaves out, but this comparison does not show which. The six tier-A
> flights in other formats wait for [M3.4 to M3.6](decisions-and-roadmap.md#m3-4), and the climb
> traces for [M2.3c2](decisions-and-roadmap.md#m2-3c2).

**Sources.** The flights are published by university rocketry teams, rocketry courses and
hobbyists on GitHub, team websites and The Rocketry Forum. The weather is ERA5, generated using
Copernicus Climate Change Service information 2026, and contains modified Copernicus Climate
Change Service information 2026; neither the European Commission nor ECMWF is responsible for any
use made of it. The [report][fixture-report] gives the full attribution and the ERA5 citations.

## Known gaps

These are the largest known differences and missing pieces. Each model page's *In short* lists the
rest.

- **HPR Sim's own drag in a whole flight.** Its heights are +10.113% and +10.302% above RocketPy's
  for Valetudo and NDRT 2020, where its drag is well below the examples', and −7.280% below for
  Prometheus 2022, where it is above ([report][report]). Against real flights, four of the five
  apogees more than 5% from their logs are consistent with HPR Sim's drag: NDRT 2020 +10.40% and
  Cavour +5.63%, Prometheus −8.90% and Genesis −5.85% ([real flights](#real-flights),
  [real-flight report][real-report]).
- **Drag faster than sound reads high** against NASA's wind tunnel, above all with fins: with the
  fins on, +39.4% at Mach 1.5 to +154.0% at 4.63. The fins take a blunt leading edge's formula,
  and nothing models a thin, sharp fin's own wave drag. Niskanen's cone, which ogives share,
  reads 45% to 105% above a measured cone through the rise near Mach 1
  ([Aerodynamics](physics/aero.md#drag-against-the-arcas-robin-wind-tunnel)).
- **A steep boattail's drag reads high** in a thick boundary layer: 16° boattails +26.4% to
  +54.2% from Mach 1.0 to 1.28, and the Arcas Robin's 15° boattail puts its forebody, fins off,
  +13.5% to +24.1% high from Mach 1.5 and +20.3% to +50.8% from Mach 1.0 to 1.2. No cited
  correction exists in the sources used (issue [#72](https://github.com/nrdptel/fusionspace-eridanus/issues/72);
  [Aerodynamics](physics/aero.md#boattails-faster-than-sound)).
- **Drag past Mach 1.6 reads low against RASAero II's Calisto**, to −14.9% at Mach 2, with the fins
  and finish the Mach 0.3 check declares; other plausible fins bring most rows within 10%. Part of
  it is HPR Sim's body, which reads 6% to 10% low faster than sound against MIL-HDBK-762's worked
  example as well
  ([Aerodynamics](physics/aero.md#drag-against-rasaero-ii-through-mach-2)).
- **Roll: the forcing high near Mach 1.5, the damping low, nothing measured below Mach 1.5.**
  The roll forcing from canted fins reads +47.8% at Mach 1.5
  and +14.3% to +17.8% at 1.8 against NASA's wind tunnel, where linear theory's load climbs toward
  Mach 1 faster than the fins' does; the roll damping reads 5.9% to 16.2% low against the Basic
  Finner's. Nothing measured checks either below Mach 1.5
  ([Aerodynamics](physics/aero.md#roll-forcing-and-damping)).
- **A flared rocket's supersonic normal force rests on one measured flare.** A conical flare flies
  the shock-expansion method since [M1.8e17](decisions-and-roadmap.md#m1-8e17), which moves the
  [center of pressure](glossary.md#center-of-pressure-cp) of a test rocket with a 10° flare
  forward by 0.089 to 0.238 [calibres](glossary.md#calibre-caliber) against the model it had
  before, growing with Mach number. The one measurement beside it is NASA TN D-4865's model 2, an
  18.5° flare on a 2.75° cone, and it is not that rocket
  ([Aerodynamics](physics/aero.md#what-a-marched-flare-is-worth)). No other flare angle has been
  checked, and the reading a flare steeper than its corner's limit gets has been checked against
  nothing at all. A flare shallow enough to *reduce* its own element (a few thousandths of a
  degree on that rocket) is read by the older generalized method instead, which no measurement
  checks either, and which leaves a step where the corner's pressure crosses its tangent cone's
  ([Aerodynamics](physics/aero.md#a-near-flat-flare)). On a body with a short shoulder that
  crossing is not near-flat at all (0.7° to 4.6°) and the step reaches +4.3% of the body's
  normal force and 0.19 calibres; its exact size for any conical flare is on that page.
- **Some shapes fall back to [slender-body theory](glossary.md#slender-body-theory) faster than
  sound, and read more stable.** Past Mach 1.2, a body with a step in its radius
  ([#87](https://github.com/nrdptel/fusionspace-eridanus/issues/87)), a flare behind a boattail too long for
  its wake ([#120](https://github.com/nrdptel/fusionspace-eridanus/issues/120)), a pointed tip steeper than 30°
  ([#121](https://github.com/nrdptel/fusionspace-eridanus/issues/121)) or a vertical tip steeper than its cap's
  handover keeps slender-body theory for the whole body. The sizes are measured at Mach 3 and 4°
  on two test bodies: −8.65%, −27.5%, −7.7% and −7.0% in normal force, with the center of pressure
  1.03, 0.29, 0.81 and 0.64 calibres aft, so the body reads more stable than it is. The flare's is
  measured on a finless body, so its share is of a body's normal force alone; the other three are
  on a straight rocket with four fins, so the two kinds of share don't compare
  ([Aerodynamics](physics/aero.md#the-body-faster-than-sound-in-a-flight)). No size inside the core
  band, below Mach 2.5, is pinned yet. They are first in
  [M1.14d](decisions-and-roadmap.md#m1-14d), supersonic accuracy, and in
  [M1.14h](decisions-and-roadmap.md#m1-14h) above Mach 2.5.
- **The normal force near and far past Mach 1.** Against NASA's wind tunnel, between Mach 0.8
  and 1.2 HPR Sim's slope runs up to +27.1% high and its center of pressure up to 2.36
  [calibres](glossary.md#calibre-caliber) off. From Mach 1.5 up its slope holds to within 9.4%
  ([fixture][nf-fixture], [Aerodynamics](physics/aero.md#normal-force-through-mach-1)) and its
  center of pressure to 0.53 calibres, which misses the half-calibre target on the long model at
  Mach 1.8 and 2.3, where the body alone reads 15% to 19% above the tunnel
  ([Aerodynamics: body lift](physics/aero.md#body-lift)).
- **Tube fins' center of pressure is unmeasured, and OpenRocket's differs.** HPR Sim's is 0.42 to
  3.0 calibres forward of OpenRocket 24.12's on 14 probe designs up to Mach 0.5, and 1.07 on its
  *Tube fin rocket*, whose margin HPR Sim gives as 0.79 calibres to OpenRocket's 1.87. Neither code
  has been checked against a measured tube-fin rocket; HPR Sim leaves out the body's interference,
  which slender-body theory predicts
  ([#234](https://github.com/nrdptel/fusionspace-eridanus/issues/234)), and its tube-fin drag
  probably reads low ([#228](https://github.com/nrdptel/fusionspace-eridanus/issues/228); [tube
  fins](physics/aero.md#tube-fins-against-openrocket)). Check a tube-fin design in both programs and
  treat the smaller margin as the more cautious estimate, not a bound.
- **Drag against the RASAero curves** at Mach 0.3 is within 10% in four of seven cases, with the
  fins and surface finish guessed, because the curves don't record them. Cavour power-on is
  −18.3%, cause open. Valetudo's −47.0% and −50.4% are against a table 1.44 times the drag in the
  OpenRocket export for the same rocket ([Aerodynamics](physics/aero.md#verification)).
- **Six fins.** The [normal-force slope](glossary.md#normal-force-slope) of Barrowman's six-fin
  Recruiter is +2.87% above his printed value, and +3.42% on the fins alone, mostly because HPR Sim
  uses a different six-fin rule ([Aerodynamics](physics/aero.md#verification)).
- **Tumbling** is −10 to +19% off its source's own drop tests, and is used far outside the fit
  behind it. That fit comes from small models falling at 5.0 to 6.6 m/s (16 to 22 ft/s); if Valetudo came down
  tumbling, with nothing deployed, HPR Sim would bring it down at 36 m/s (118 ft/s). The default streamer model
  reads +58% fast on a pleated streamer ([Recovery](physics/recovery.md)).
- **Opening loads,** the force on the rocket as a canopy opens, are no safe bound either way. With
  a [filling time](glossary.md#inflation-and-filling-time), HPR Sim leaves out the brief rise of
  drag above its steady value while the canopy fills. Opening at once, it ignores how a light rocket
  slows while the canopy fills. The deployment speed can itself read high: a
  [separated](glossary.md#separation) body falls with no drag until its device opens
  ([Recovery](physics/recovery.md#inflation)).
- **No added mass under a canopy,** the likely cause of the 2.86% drift difference above
  ([Recovery](physics/recovery.md#against-rocketpy)), and the cause of NDRT's +83.059% and
  Prometheus 2022's +19.062% peaks as their mains open in the whole flight ([report][report],
  [case file][ndrt-flight-case]).
- **Body lift in wind.** A slow rocket leaves the rail at a steep angle to a crosswind, and there
  HPR Sim's body lift, which RocketPy leaves out, is the largest reason its drift differs: Juno
  III's apogee drift is −38.158% against RocketPy's ([report][report]). How much body lift a rocket
  body makes is itself uncertain. HPR Sim takes it from Jorgensen's crossflow term
  ([Aerodynamics](physics/aero.md#bodies-of-revolution)), whose factor is about 0.9 for these
  rockets at low speed. Flown in RocketPy with HPR Sim's rail release and fins, that puts Juno III's
  apogee 248.3 m (815 ft) from the pad, against HPR Sim's own 245.3 m (805 ft, [case file][juno-case]). In the same
  runs, Galejs's constant `K`, which HPR Sim used before, gives 240.2 m (788 ft) at 1.0, 231.1 m (758 ft) at 1.1 and
  194.1 m (637 ft) at 1.5, across its source's range, and the drift would be 328.0 m (1,076 ft) with no body lift at
  all ([ADR-026][adr-026]). Only real flights can say which is right, and the
  [real flights](#real-flights) so far compare heights, not drift.
- **Airfoil fins.** HPR Sim's fins use the flat-plate lift slope. It cannot model an airfoil lift
  curve such as the one Juno III's example gives its fins, which makes RocketPy's fin slope 7.6%
  steeper ([ADR-026][adr-026]).
- **Turbulence** is an aircraft model, unvalidated for rockets, and no flight uses it yet
  ([Turbulence](physics/turbulence.md)).
- **Wall and fin mass** may follow different conventions from OpenRocket's, which its documentation
  doesn't state; six are measured so far ([Mass properties](physics/mass.md#checked-against-openrocket)). Measuring a nose cone's wall thickness straight out from the axis, rather than
  square to its surface, changes the wall's volume by 1.4% on a cone three
  [calibres](glossary.md#calibre-caliber) long ([Shapes](physics/shapes.md)).

[ndrt-case]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/cases/descent-ndrt-2020-nose-to-tail.toml
[plan]: VALIDATION.md#principles
[census]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/census.md
[real-report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/real-flights.md
[fixture-report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/fixture-flights.md
[adr-184]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0184-logged-apogees-of-the-private-collection.md
[adr-102]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0102-tube-fins-centre-of-pressure-measured-against.md
[adr-082]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0082-real-flights-read-from-refs-compared-over-the.md
[adr-083]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0083-m2-3c-blocked-no-private-design-is-the-rocket-of.md
[report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/latest.md
[rocket-notes]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/rocketpy-rocket-mass.md
[valetudo-case]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/cases/descent-valetudo.toml
[adr-021]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0021-whole-flights-against-rocketpy-what-is-compared.md
[adr-023]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0023-predicted-mode-each-codes-own-drag-reported.md
[bella-case]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/cases/flight-bella-lui.toml
[calisto-case]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/cases/flight-calisto-tests-motor-at-minus-1.373.toml
[issue-50]: https://github.com/nrdptel/fusionspace-eridanus/issues/50
[bella-predicted-case]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/cases/predicted-bella-lui.toml
[ndrt-predicted-case]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/cases/predicted-ndrt-2020-nose-to-tail.toml
[prometheus-predicted-case]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/cases/predicted-prometheus-2022-generic-motor.toml
[valetudo-predicted-case]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/cases/predicted-valetudo.toml
[juno-case]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/cases/flight-juno-iii.toml
[adr-026]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0026-the-path-in-wind-rocketpys-corrected-equations.md
[adr-061]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0061-what-a-ork-leaves-unsaid-read-as-openrocket.md
[ndrt-flight-case]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/cases/flight-ndrt-2020-nose-to-tail.toml
[prometheus-case]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/cases/flight-prometheus-2022-generic-motor.toml
[nf-fixture]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/aero/normal-force-vs-mach.json
[se-fixture]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/aero/shock-expansion.json
[bt-fixture]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/aero/blunt-tips.json
[lip-fixture]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/aero/arcas-robin-lip.json
[flare-fixture]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/aero/marched-flare.json
[body-gap-fixture]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/aero/arcas-robin-body-gap.json
[drag-fixture]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/aero/drag-vs-mach.json
[roll-fixture]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/aero/roll-vs-mach.json
