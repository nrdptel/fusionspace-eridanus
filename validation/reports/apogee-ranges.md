# How often the apogee ranges hold the logs

Written by `cargo xtask apogee-ranges`; do not edit (M2.8a, decision record [ADR-386](../../docs/decisions/0386-m2-8-in-two-the-ranges-coverage-measured-first.md)). A Monte Carlo run flies a design many times with its inputs scattered, and gives a range of apogees. A 90% range says that nine real flights in ten end inside it. This page counts whether they do. Each flight of the private collection that is [compared with its log](fixture-flights.md) is flown 1,000 times more, its inputs drawn about the design file's values, and the logged apogee is placed against that run's 50% and 90% ranges. The collection's license allows only aggregates, so no flight is named or numbered here.

How far to trust it: the counts are of 60 flights, 23 of them held out, so each share comes with wide bounds, and the flights are the ones fliers chose to publish with a design file and a log. The spreads are stated ones, taken from published figures before any count was made, and nothing was fitted to these flights.

## The run

- **The flights:** each flight as the [collection's report](fixture-flights.md) flies it: the design as read, in the day's ERA5 air and wind, from its rail, nothing deployed. The run is the one `hpr mc` flies: every input an independent normal draw about its value, one random stream a sample and input. A flight's seed is the first eight bytes of the SHA-256 of its id followed by `ADR-386`.
- **A range** at 50% runs from the run's 25% point to its 75% point, and at 90% from its 5% point to its 95% point, of the height the center of mass climbed. The logged apogee is that height too, as the collection's report reads it from the log.
- **A sample that fails** can't help a range hold: the low edge counts each failed sample as above the run's apogees and the high edge as below them, the narrowest range they could have left. A run with no range left, or one that can't be set up, is a miss.
- **The bounds** on each share are the two-sided 95% Clopper–Pearson interval: the shares of all such flights that this many could have come from. A level outside the bounds is a range these flights show to be wrong; a level inside them is one they can't tell from right.

The standard deviation of each input drawn, the same for every flight:

| input | standard deviation | where it is stated |
|---|---:|---|
| mass without motors | 10.75% | docs/research/as-built-mass-growth.md: over 24 rockets with both a design-file mass and a scale mass, the scale's stands −1.4% to +13.1% from the file's at the quartiles, a standard deviation of 10.75% for a normal spread (the range over 1.349) |
| zero-lift drag coefficient | 16.3% | docs/research/flight-derived-cd.md, §2: 14 published fits of a coasting rocket's drag stand at 0.89 to 1.11 of the design code's at the quartiles, a standard deviation of 16.3% for a normal spread (the range over 1.349); the note behind the run's inputs, docs/research/dispersion-sigmas.md, under What this suggests, item 1, gives 10% to 16% for a design file's drag, and this is its wider end |
| total impulse | 3.56% | docs/research/hpr-lot-impulse.md, §2: 21 AeroTech H to L motors that Tripoli Motor Testing tested a second time differ by a standard deviation of 3.56 points between their two tests; a flown motor stands from 2.5% to 3.56% off its file's curve where each test fired one motor, and that range's wider end is taken; where each test's figure is the mean of three motors the top would be 5.03%, which isn't taken |
| burn time, at the same impulse | 9.2% | docs/research/hpr-burn-time-scatter.md, §3: over 14 AeroTech H to L motors tested again years apart, one test's burn time stands 9.2% from its motor's mean; within one lot and session the same note finds 4.1%, and the wider is taken; docs/research/dispersion-sigmas.md's table lists 10.7% measured in one hobby source and up to 19% assumed in others, which aren't taken |
| rail's angle above the horizon | 1° | docs/research/dispersion-sigmas.md, §5: a tilt of 1° in five of the seven hobby sources that disperse a launch angle, 1.5° and 2° in the other two; each assumes it and none measured it |
| rail's heading | 5° | docs/research/dispersion-sigmas.md, §5: 1° to 5° of heading in the hobby sources that disperse one by a standard deviation, each assumed, and that range's wider end is taken; some draw it evenly over a full turn instead, which isn't taken |

Left at the values flown, each of which would widen a range: the wind and the air, the center of mass, and the altimeter's own scatter. The [decision record](../../docs/decisions/0386-m2-8-in-two-the-ranges-coverage-measured-first.md) says what is known of each one's size. The mass is drawn stage by stage, so a staged rocket's whole mass spreads by less than the figure above. Each bound is rounded outward, the lower one down and the upper one up.

## The 50% range

| flights | count | held | share | 95% bounds | 50% inside the bounds | log below the range | log above the range | no range |
|---|---:|---:|---:|---|---|---:|---:|---:|
| every flight compared | 60 | 18 | 30.00% | 18.84% to 43.21% | no | 31 | 11 | 0 |
| without the early deployments | 41 | 13 | 31.71% | 18.08% to 48.09% | no | 17 | 11 | 0 |
| without the flights the catalog proposes below tier A | 49 | 14 | 28.57% | 16.58% to 43.27% | no | 25 | 10 | 0 |
| the calibration half | 37 | 13 | 35.14% | 20.20% to 52.54% | yes | 15 | 9 | 0 |
| the held-out half | 23 | 5 | 21.74% | 7.46% to 43.71% | no | 16 | 2 | 0 |

## The 90% range

| flights | count | held | share | 95% bounds | 90% inside the bounds | log below the range | log above the range | no range |
|---|---:|---:|---:|---|---|---:|---:|---:|
| every flight compared | 60 | 50 | 83.33% | 71.47% to 91.71% | yes | 9 | 1 | 0 |
| without the early deployments | 41 | 38 | 92.68% | 80.07% to 98.47% | yes | 2 | 1 | 0 |
| without the flights the catalog proposes below tier A | 49 | 42 | 85.71% | 72.75% to 94.06% | yes | 7 | 0 | 0 |
| the calibration half | 37 | 32 | 86.49% | 71.22% to 95.47% | yes | 4 | 1 | 0 |
| the held-out half | 23 | 18 | 78.26% | 56.29% to 92.54% | yes | 5 | 0 | 0 |

A log below its range is a flight the run flew higher than; a log above it, one the run kept lower than.

## Where the logs fall in their runs

Each flight is counted in the tenth of its run its logged apogee falls in: the first column counts the logs below the lowest tenth of their run's apogees, the last those above the highest. Runs whose ranges hold as often as they say fill the ten evenly. The offset is the median, over the flights, of the logged apogee less the run's median, as a share of that median.

| flights | median offset | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| every flight compared | -6.76% | 14 | 13 | 7 | 5 | 6 | 1 | 2 | 5 | 3 | 4 |
| without the early deployments | -4.90% | 3 | 11 | 5 | 5 | 3 | 1 | 1 | 5 | 3 | 4 |
| without the flights the catalog proposes below tier A | -6.60% | 12 | 9 | 7 | 2 | 5 | 1 | 2 | 5 | 3 | 3 |
| the calibration half | -3.91% | 5 | 8 | 3 | 4 | 6 | 1 | 1 | 4 | 2 | 3 |
| the held-out half | -10.99% | 9 | 5 | 4 | 1 | 0 | 0 | 1 | 1 | 1 | 1 |

## How wide the runs are

Each width and standard deviation is a share of its run's median apogee; the medians are over the set's flights.

| flights | median width of the 50% range | median width of the 90% range | median standard deviation | least | most |
|---|---:|---:|---:|---:|---:|
| every flight compared | 13.42% | 33.49% | 10.37% | 8.60% | 17.83% |
| without the early deployments | 13.37% | 33.41% | 10.33% | 8.60% | 15.17% |
| without the flights the catalog proposes below tier A | 13.37% | 33.41% | 10.33% | 8.60% | 17.83% |
| the calibration half | 13.47% | 33.41% | 10.33% | 8.72% | 16.07% |
| the held-out half | 13.37% | 33.63% | 10.80% | 8.60% | 17.83% |

## Samples that failed

| flights | samples | failed | flights with a failed sample | runs that couldn't be set up |
|---|---:|---:|---:|---:|
| every flight compared | 60,000 | 0 | 0 | 0 |
| without the early deployments | 41,000 | 0 | 0 | 0 |
| without the flights the catalog proposes below tier A | 49,000 | 0 | 0 | 0 |
| the calibration half | 37,000 | 0 | 0 | 0 |
| the held-out half | 23,000 | 0 | 0 | 0 |
