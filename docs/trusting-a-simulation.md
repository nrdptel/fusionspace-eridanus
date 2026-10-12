# Trusting a simulation

A simulated apogee is an estimate: this guide shows how far off it has been on real flights, and
which way it errs.

**This guide is for** a flier who reads a simulated apogee to pick a motor, plan a waiver or a
field's ceiling, or set up a flight. It explains what a simulator can and can't know about a
flight, how big its misses have been, and which miss to plan for. It is the first of the guides in
*Rocketry explained*, which explain the physics behind the numbers HPR Sim prints.

> **How far to trust it.** Every number in this guide's text is read by a test from a committed
> report, and CI reruns the worked example's command. The reports set HPR Sim's apogees against 53
> logged flights of fliers' own designs ([report][fixture-report]), and against 7 public flights
> ([report][real-report]). They measure
> apogee alone: no comparison here checks a top speed, a descent or a landing place against a real
> flight. The guide gives ranges, not verdicts; the safety code and the range safety officer decide
> whether a rocket flies.

## Two ways a simulation is wrong

A simulation can be wrong in what it is told and in how it works. The first is its inputs: the
masses and the balance point in the design file, the finish of the paint, the motor's real thrust,
and the day's wind and air. The second is its models: above all the drag, which a simulator
estimates from the rocket's shape instead of measuring it.

Scattering the inputs shows the first kind. HPR Sim's Monte Carlo command, `hpr mc`, flies the
rocket many times, each with its inputs drawn afresh, and prints the spread
([Monte Carlo dispersion](monte-carlo.md)). It can't show the second kind: a model that is wrong
the same way on every run moves every run together.

Only flights that really flew show the second kind. The comparisons below fly each rocket's design
in the day's weather and set its apogee against the altimeter's. So they measure both kinds at
once: the weather is the day's, but the masses and the finish are the design file's, as most
fliers' files are. The altimeter is a measurement too, with errors of its own
([Reading a flight log](reading-a-flight-log.md)).

## How far off it has been

A miss is the simulated apogee less the logged one, as a share of the logged one. A positive miss
means the simulator said higher than the rocket flew:

```text
miss = (simulated apogee − logged apogee) / logged apogee
```

For example, the flight of Cavour, Politecnico di Torino, EuRoC 2023 logged 2,789 m
(9,150 ft), and HPR Sim flew its design to 2,946 m (9,665 ft). The miss is
(2,946 − 2,789) / 2,789 = +5.63% ([report][real-report]).

Across 53 logged flights of fliers' own designs, each flown as drawn in its day's air, HPR Sim's
apogee averaged +8.73% above the logged one ([report][fixture-report]). Without their signs, the
misses averaged 12.95%, and 26 were within 10% either way. The figure counts them by size.

![A bar chart of HPR Sim's apogee misses on the logged flights: most bars stand right of zero, the shaded band holds the small misses either way, and the low side is marked as the side that flatters a ceiling.](images/trust-apogee-misses.svg)

The misses lean high: most simulated apogees are above the logged one. And they spread wide: the
lowest read between 15% and 20% low, the highest past +30% high. OpenRocket, flying the same files in each day's wind
and the pad's air, averaged +7.79% above and missed by 12.02% without signs. The two simulators
agreed with each other within 5% on 52 of the 53 flights. So they share most of their misses: the
cause lies in what both read from the design file and the motor's curve, or in models they have in
common, and a second simulator is no check on the first.

The 7 public flights, university teams' rockets flown in a reanalysis of the day's weather, tell a
kinder story ([report][real-report]). HPR Sim's apogee missed them by 6.04% on average without
signs, from −8.90% to +10.40% one by one, and with signs by −0.25%: they don't lean high. They are
fewer rockets, documented by their teams. The note HPR Sim prints with every apogee quotes the
53, the larger set and the less flattering one ([the trust note's decision record][adr-211]).

## Which miss flatters

A miss can flatter or warn, and which one depends on the plan. For a waiver or a field's ceiling, a
simulated apogee below the real one flatters: the rocket climbs past the height that was planned
for. A simulated apogee above the real one costs only height. For a contest's lowest altitude, or
a motor picked to reach a height, the sides swap, and the high side flatters.

The figure marks the low side, and a test ties the mark to the sign of a miss: the figure's code
marks the bins of negative misses, and
[`a_low_simulated_apogee_is_a_negative_miss`][direction-test] holds that a simulated apogee below
the logged one makes a negative miss. Of the 53 flights, 13 read low, the lowest of them between
15% and 20% low ([report][fixture-report]). A sample is not a bound: a rocket unlike these may miss
by more.

One of the public flights shows the low side in meters. HPR Sim's apogee for Juno III, Projeto
Jupiter, Spaceport America Cup 2023 was 2,923.4 m (9,591 ft); its altimeter logged 3,151.5 m
(10,340 ft), a miss of −7.24% ([report][real-report]). A ceiling planned on the
simulated apogee would have been passed by 228 m (748 ft). So plan a ceiling with room above the
simulated apogee; how much room is the flier's and the range safety officer's call, and this guide
has no number for it.

## The spread from the inputs

This is the guide's worked example: the small rocket the repository keeps for its how-to guides,
flown many times over with its mass, its drag and its motor's impulse each scattered. The command
gives each scatter as one standard deviation, as a share of the value: `--mass-sd 0.02` for the
mass.

<!-- cli: example `hpr mc validation/fixtures/ork/guides/level-1.ork --runs 200 --seed 2026 --mass-sd 0.02 --drag-sd 0.05 --impulse-sd 0.03` -->

```text
$ hpr mc validation/fixtures/ork/guides/level-1.ork --runs 200 --seed 2026 --mass-sd 0.02 --drag-sd 0.05 --impulse-sd 0.03
Level 1 guide rocket (level-1.ork)
configuration 1 of 2: [H170M-10]
help: --config flies the others, by number or name: 2 [I175WS-9]

200 simulated flights, seed 2026: 0 failed
                        nominal     mean  std dev       5%   median      95%
apogee (m AGL)           1065.1   1072.4     41.5   1006.2   1072.8   1137.3
apogee (ft AGL)            3494     3519      136     3301     3520     3731
landing distance (m)        0.7      0.7      0.0      0.7      0.7      0.8
landing distance (ft)         2        2        0        2        2        3

200 of 200 flights landed, centered 0.7 m (2 ft) west and 0.0 m (0 ft) north of the pad
landing ellipse                   semi-major            semi-minor  heading   flights inside
50%                             0.0 m (0 ft)          0.0 m (0 ft)   090° T            74.5%
95%                             0.1 m (0 ft)          0.0 m (0 ft)   090° T            98.0%
95%, the next flight            0.1 m (0 ft)          0.0 m (0 ft)   090° T            98.5%

motor: 1 × H170M (from the bundled catalog, as of 2026-09-17) in `Motor mount tube`, lit at launch
launched at 0.00000° N, 0.00000° E, 0 m (0 ft) above sea level, from a 1.5 m (4.9 ft) vertical rail, in calm air
scattered, one standard deviation each: --mass-sd 0.02, --drag-sd 0.05, --impulse-sd 0.03
note: the file's 1 recovery device flies as OpenRocket flies it: each opens fully at its event, with the file's drag coefficient or OpenRocket's own, and once one opens the rocket descends as a point under the open devices' drag alone
warning: drag, the nominal flight: issue #67: a cone-like nose's or shoulder's pressure drag reads high from Mach 0.8 (about twice a measured cone's at Mach 0.85, still +15% at 1.5), so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are; this flight reaches Mach 0.84 at 1.7 s (https://github.com/nrdptel/fusionspace-eridanus/issues/67)
warning: drag, the nominal flight: issue #68: base drag reads high from Mach 0.8 to 1.2 (0.225 against a measured 0.156 at Mach 0.9), so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are; this flight reaches Mach 0.84 at 1.7 s (https://github.com/nrdptel/fusionspace-eridanus/issues/68)
warning: drag, the nominal flight: issue #18: skin friction is taken as fully turbulent, but on a smooth surface the flow stays laminar near the nose, where friction is lower: HPR Sim's drag reads high by 3.6% on RocketPy's Calisto at Mach 0.3; if this surface is that smooth, the drag reads high, so the apogee, the top speed and the drift read low, and the flutter margin and the largest dynamic pressure look better than they are (https://github.com/nrdptel/fusionspace-eridanus/issues/18)
warning: stability, the nominal flight: issue #172: the static margin read up to 0.1108 calibers higher than OpenRocket's on four private designs, for a reason not yet found, so this flight's margin may read high by as much (https://github.com/nrdptel/fusionspace-eridanus/issues/172)

How far to trust it. Simulated from the design file, not measured; the spread above comes from the
inputs' scatter alone, not from the model's error. Against 53 logged flights of fliers' own designs,
each logged apogee the height climbed in the day's air, the simulated apogee averaged 8.7% above the
logged one; it was within 10% on 26 and read low on 13, by at most 20%. Plan a waiver or a field's
ceiling with room above this apogee; the altimeter's reading is the one to log, and the RSO decides.
More: https://hpr.fusionspace.co/accuracy.html
```

<!-- cli: end -->

Read it in three parts. The table's `5%` and `95%` columns bracket the middle of the apogees the
scattered inputs gave: most runs fell between them, a few on either side. The warnings name the model gaps that bear on this flight,
each with the side it pushes the numbers to: here, drag that reads high, so an apogee that reads
low. The note at the end is the miss on real flights from the sections above, which the spread
leaves out.

The two don't add as one range. The spread is this rocket's, from inputs the command was told to
scatter; the miss is a record of other rockets. Read the spread as how much the flight could move
if the file is right, and the miss as how far files like this one have been from the sky.

## What this guide leaves out

- **Anything but apogee.** No comparison here checks a top speed, a descent, a drift or a landing
  place against a real flight. Those are checked against other programs, which shows that HPR Sim
  computes what they compute, not that any of them matches the world ([Accuracy](accuracy.md)).
- **Rockets unlike these.** The flights are hobby and university rockets as their fliers drew them.
  A rocket much faster, much higher or of an odd shape sits outside them, and
  [Accuracy](accuracy.md) lists what each comparison covers.
- **Your own rocket's miss.** A weighed rocket, its mass and balance point put in the file, removes
  one input error the logged flights carry; only flying it and reading the altimeter measures the
  rest. [Reading a flight log](reading-a-flight-log.md) shows how to compare.

[fixture-report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/fixture-flights.md
[real-report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/real-flights.md
[adr-211]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0211-how-far-to-trust-a-result.md
[direction-test]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/xtask/src/fixture_flights.rs
