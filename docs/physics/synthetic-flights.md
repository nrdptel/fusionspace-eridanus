# Synthetic flights

See how FusionSpace HPR makes the simulated flights, each with its logger's
[flight log](../glossary.md#flight-log) and a truth that is known, on which its readings and fits
are tested, and what their noise holds and leaves out. The noise follows the makers' datasheets
and the noise measured in a few public logs; it has not been checked against the statistics of
real logs, and the flights are HPR Sim's own.

## In short

- **What it models:** 2,400 flights of the ten public RocketPy designs HPR Sim validates against,
  in three sets on fixed seeds. Each flight draws its motor, drag, wind, rail angle, site and air,
  and one of six simulated loggers, each like a family of real loggers that FusionSpace HPR reads.
  The logger writes what its sensors would: a pressure with an offset, a drift, correlated noise
  and the port's error near the speed of sound; an accelerometer with its scale, bias, noise and
  range; a GNSS fix that lags in boost and drops out. Each flight's truth is HPR Sim's trajectory,
  every 0.01 s, with its [dynamic pressure](../glossary.md#dynamic-pressure) and
  [Mach number](../glossary.md#mach-number).
- **Sources:** the pressure sensors' and accelerometers' datasheets and the noise measured in
  public logs ([the sensor noise note][noise]); a wind-tunnel test of the static pressure along an
  ogive-cylinder through Mach 1 ([the Mach error note][mach]); the ejection pulse of Debrief's
  public Pnut log; the lag of hobby GNSS receivers in boost ([the GNSS note][gnss]); the NAR's
  [high-power safety code][hprcode] for the wind's limit.
- **How well it is validated:** each noise term is pinned by a test that holds it to its stated
  value on a truth built for that term alone, and fails when the term is changed. A flight drawn
  again is the same to the bit. No test yet shows that the terms together look like a real log
  of the same family: [#725](https://github.com/nrdptel/fusionspace-eridanus/issues/725) tracks
  that comparison.
- **What it leaves out:** the bay's vent lag, a temperature term, clock drift, the logger's own
  speed column, and flights past Mach 1.17, the fastest of the ten designs
  ([What it leaves out](#what-it-leaves-out)).

## Why simulated flights

A real log has no truth beside it. The altimeter's own apogee comes from the same pressure trace
that a reading takes it from, so the two agreeing shows they read the trace alike, not that
either is right ([Flight-log readings](log-readings.md#checked-against)). To learn how often a
90% interval holds the truth, the truth must be known, on many flights.

Each reading and fit states a 90% interval, and
[ADR-257, calibrated intervals](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md)
asks that each be tested on at least 1,000 flights whose truth is known: it must miss about 5%
of the time on each side. The sets were fixed before any interval was written, so no interval
can be tuned on the flights it is judged on
([ADR-280, the synthetic flights first](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0280-m7-2-in-seven-the-synthetic-flights-first.md)).

A simulated flight knows its truth, but a check on it is only as good as its noise. A model that
adds white noise alone flatters an interval: real barometer noise is correlated from sample to
sample, so its errors don't average away as fast ([the smoothing note][smoothing] §6). So each
term here is drawn from a measurement or a datasheet, and the census and the tests say which.

A check on these flights shows how well a method recovers HPR Sim's trajectory from noisy
sensors. It doesn't show how well HPR Sim predicts a real flight: that is
[M7.3, a flight against its simulation](../decisions-and-roadmap.md#m7-3)'s question, on real logs.

## The flights

The sets each flight belongs to, what it draws, and the loggers that record it.

### The sets

Three sets, each on its own seed, written down before any method was tuned:

| Set | Flights | Seed | Model flown |
|---|---:|---:|---|
| Tuning | 200 | 2026101700 | alternating A's and B's |
| A | 1,000 | 2026101701 | HPR Sim's |
| B | 1,000 | 2026101702 | reshaped in three ways |

Anything fitted or tuned uses the tuning set; sets A and B only judge. Flight $k$ of a set draws
each input from its own stream, keyed by the set's seed, $k$ and the input, so a set's first
flights stay the same if it grows, and adding an input moves no other.

Set B flies a model that differs from HPR Sim's on purpose, so that a method can't pass by
knowing HPR Sim's model:

- **The drag** rises differently through Mach 1: the zero-lift drag is multiplied by
  $1 + a\,e^{-((M-1)/0.2)^2} + bM$, with $a$ between −0.2 and 0.2 and $b$ between −0.05 and
  0.05.
- **The wind** grows with height as a power law, its exponent 0.14 to 0.20, the drawn speed at
  18.3 m (60 ft), after NASA/TM-2008-215633 §2.2.5.2. Set A's wind is the same at every height.
- **The thrust curve** is tilted: its thrust times $1 + \tau(2t/t_b - 1)$, with $\tau$ between
  −0.3 and 0.3, then scaled back to the same total impulse.

### What a flight draws

| Input | Range | Why |
|---|---|---|
| design | one of the ten | the RocketPy designs, each with its example's parachutes |
| total impulse | 0.94 to 1.06 times | twice a motor's 3% spread ([the dispersion note][sigmas] §2) |
| burn time | 0.90 to 1.10 times | about its 10.7% spread (the same) |
| zero-lift drag | 0.8 to 1.2 times | twice the low end of the 10 to 16% spread (the same) |
| wind | up to 20 mph (8.9 m/s), from any direction | the safety code's limit |
| rail | 0° to 10° from vertical, toward any direction | inside the code's 20° |
| site | 0 to 1,500 m (0 to 4,921 ft) above sea level | sea-level fields to the high deserts |
| air | the 1976 standard, −15 to +15 K | a day off standard |

The loggers convert pressure to height by the standard atmosphere, so a day off standard is a
real error in their heights: on a cold day a barometer reads high. The readings must carry it.

The truth is the flight's center of mass, every 0.01 s, and its events: liftoff, rail exit,
burnout, apogee, each charge and deployment, and landing. Heights are above the pad, where the
center of mass stood at liftoff. The specific force along the rocket's axis is what an
accelerometer would feel; under a canopy the rocket is taken to hang vertically.

### The loggers

A flight draws one of six profiles, each like a family of real loggers, at that maker's logged
rate and step. Its log is tagged with that family's format, its logger named as synthetic.

| Profile | Rate | Pressure written as | Accelerometer | GNSS |
|---|---|---|---|---|
| PerfectFlite | 20 Hz | height, whole feet | none | none |
| RRC3 | 20 Hz | pressure, 10 Pa steps | none | none |
| Eggtimer | 20 Hz | height, whole feet | none | none |
| AltOS | 100 Hz climbing, 10 Hz after | pressure, 1 Pa steps | high-g | 1, 5 or 10 Hz |
| 500 Hz IMU | 500 Hz | pressure, 2 × 10⁻⁵ atm steps | low-g, ±32 g | none |
| GNSS tracker | 1, 5 or 10 Hz | none | none | yes |

The height a profile writes is its pressure's height in the 1976 standard atmosphere, less the
mean of its samples on the pad. The RRC3 uses the NOAA formula instead, as its maker's software
does ([the height formulas note][heights]); the two differ by about 1 m (3.3 ft) in 2 km (6,562 ft).

## The noise, term by term

What each sensor adds to the truth, where each term comes from, and a worked example.

### The pressure

The logged pressure is the true pressure plus six terms, then rounded to the logger's step:

| Term | What it is | Drawn per flight |
|---|---|---|
| offset | the sensor's absolute error | ±100 Pa for an MS5607 and the measured model; none for a Bosch part |
| gain | an error proportional to the change from the pad | ±0.08% for a Bosch part |
| drift | a slow change over the flight | 1 to 3 Pa over 30 to 300 s, either way |
| correlated noise | the noise measured on real logs | 0.5 m (1.6 ft) at 50 Hz, correlation 0.5 |
| white noise | the datasheet's | 2.3 Pa (RRC3), 2.4 to 13 Pa (AltOS), 0.2 to 6.6 Pa (Bosch) |
| port error | the static port's error near Mach 1 | the port 1 to 5 calibers behind the nose |

A charge adds a seventh, the ejection pulse, at each charge's time.

**The correlated noise** is a first-order autoregressive process, a red noise: each sample keeps
part of the one before. On real logs at 50 Hz its spread is 0.5 m (1.6 ft) and the correlation
from one sample to the next is 0.5 ([the sensor noise note][noise] §3). It is drawn as an
Ornstein–Uhlenbeck process, so the correlation follows the time between samples, whatever the
logger's rate:

$$
\rho(\Delta t) = e^{-\Delta t/\tau}
$$

With 0.5 at 0.02 s, $\tau = 0.0289$ s. A 20 Hz logger's samples then correlate by 0.177, a
100 Hz logger's by 0.707 and a 500 Hz logger's by 0.933. The spread in pascals is the height's
times $\rho_\text{pad}\,g$: at a 500 m (1,640 ft) site in standard air, 0.5 m (1.6 ft) is
5.72 Pa.

**The port error**, the static port's error where the pressure along the body changes near the
speed of sound, comes from a wind-tunnel test of the static pressure along an ogive-cylinder
from Mach 0.7 to 1.2 (Czarnecki and Monta, NASA TN D-3516, 1966, Fig. 5; [the Mach error
note][mach] §1–2). The port reads $C_p\,q$ off the true pressure, $C_p$ read from the test's
table at the port's distance behind the shoulder, in [calibers](../glossary.md#caliber), and the
flight's Mach number, and held at its
end values outside the table. One caliber back at Mach 1, $C_p = -0.08$. In the worked example
below that reads 473 m (1,552 ft) too high, for a moment.

**The ejection pulse** has the shape of the one on Debrief's public Pnut log, in seven samples
0.05 s apart from the charge: the height reads 8 m (26.2 ft) low at the charge, 16 m (52.5 ft)
high 0.1 s after it, and 3 m (9.8 ft) low from 0.25 s, then true again. The shape is scaled by
0 to 2 per flight, and turned into pressure at the pad's density.

### The acceleration

The logged acceleration is the truth's axial specific force times $1 + s$, plus a per-flight bias
of up to 0.05 g and white noise, clipped at the part's range and rounded to its step:

| Part | Noise density | Step | Range | Scale $s$ |
|---|---:|---:|---:|---:|
| high-g, ADXL375 class | 5 mg/√Hz | 49 mg | ±180 g | ±10% |
| low-g IMU axis | 0.16 to 0.4 mg/√Hz | 64/65,536 g | ±32 g | ±3% |

Each sample's noise is the density times $\sqrt{1.57 \cdot f_s/2}$, the noise bandwidth of a
single-pole filter at half the sample rate $f_s$, an inference in [the sensor noise note][noise]
§4. The AltOS profile's high-g part at 100 Hz has 44.3 mg (0.434 m/s²) per sample. Past 32 g the low-g part clips: the high-range part that
takes over on a real IMU isn't modeled.

### The GNSS fix

A hobby GNSS receiver's navigation filter follows the rocket like a second-order loop. Under a
steady acceleration $a$ it settles behind by

$$
e = a/\omega_n^2
$$

where $\omega_n$ is the loop's natural frequency, drawn from 0.25 to 1.0 rad/s (about 0.5
measured in one flight, [the GNSS note][gnss] §3), with damping $1/\sqrt{2}$. At 0.5 rad/s, a
steady 50 m/s² would leave the fix 200 m (656 ft) behind; a 2 s boost ends long before the
loop settles, so the lag peaks after burnout. Each axis has 3 m (9.8 ft) of white noise. No fix is
written for 0 to 10 s after ignition and after burnout, drawn per flight; the fixes written carry
no flag that they lag.

### The clock

The log starts 2 to 10 s before liftoff and ends 2 to 10 s after landing, each drawn. The AltOS
profile changes from 100 Hz to 10 Hz at its first sample after apogee.

### A worked example

An AltOS-like logger flies through Mach 1 at 1,000 m (3,281 ft) above sea level on a standard day,
its port one caliber behind the nose's shoulder.

1. **The air.** The standard atmosphere there has $p = 89{,}875$ Pa and density 1.112 kg/m³.
2. **The dynamic pressure** at Mach 1 is $q = 0.7\,p\,M^2 = 62{,}912$ Pa.
3. **The port error** is $C_p\,q = -0.08 \times 62{,}912 = -5{,}033$ Pa: the port reads low.
4. **The height it writes.** 84,842 Pa is 1,473 m (4,833 ft) in the standard atmosphere, so the
   logger reads 473 m (1,552 ft) too high while the rocket is near Mach 1. Three calibers back
   the sign flips: $C_p = +0.03$ reads 1.9 kPa high, and the height low.
5. **The noise beside it.** The correlated noise there is 0.5 m (1.6 ft), 5.45 Pa at the pad's
   density on that site; the white noise 2.4 to 13 Pa. Near Mach 1 the port's error is about
   900 times the noise, which is why a reading that leans on a barometer through Mach 1
   must say so.

## Validation and limits

What the flights were checked against, and what they leave out.

### Checked against

**Each noise term, on a truth built for it.** The tests build a truth that stands still, or climbs
at a constant rate, with one term switched on, and hold the log to that term: the offset as the
residuals' mean, the drift as their slope, the spread to within five standard errors, the
correlation at the logger's rate, the gain on a 2 km (6,562 ft) climb, the port's $C_p\,q$ at a
measured point, the pulse at 0.15 s, the accelerometer's scale, bias, spread and clip, and the GNSS
lag and dropouts. Each is checked again with the term changed, and must then fail. The
[census](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/synthetic-flights.md)
counts each set's flights by design and logger, and the range of each input and outcome.

**What the census shows.** Every one of the 2,400 draws flew. Apogees above the pad run from 412
to 4,810 m (1,352 to 15,781 ft), and top speeds from Mach 0.27 to 1.17. Values are kept to three
significant figures, the digits every platform shares, and CI flies every flight again on macOS,
Windows and Linux and holds the census to them.

### What it leaves out

- **Fast flights.** The ten designs top out at Mach 1.17, so the core band past that, to Mach
  2.5, has no synthetic flight, nor does the extended band. A reading checked here is checked to
  Mach 1.17.
- **The bay's vent lag.** A bay's pressure lags the outside air through its vent holes; for any
  maker's sizing rule it is under 3 cm (0.1 ft) at apogee ([the Mach error note][mach] §5).
- **A temperature term.** No source gives how far a bay's temperature changes in flight, so the
  sensor's temperature error isn't drawn.
- **Clock drift.** The logger's clock is taken as exact.
- **The logger's own speed column.** Each logger filters its own speed its own way, so the
  synthetic logs carry none.
- **The rocket's attitude in the accelerometer.** Only the axial axis is logged, and under a
  canopy the rocket hangs vertically. Each parachute opens at full size after its lag, as the
  RocketPy examples define them, so a main's opening is a sharp spike.
- **A part's stated noise, where it is larger.** The noise from the density can understate a
  part's own stated figure: the MPU-9250's 8 mg at 94 Hz is 2.2 times what the density gives
  ([the sensor noise note][noise] §4). The low-g profile may be quieter than a real IMU.

## Tests and references

The tests that pin the flights, and the sources they follow.

### Tests

- `hpr_forensics::synthetic`: every draw inside its range and filling it, in every set; each
  set on its own seed; a flight drawn again the same to the bit, whether flown alone or after
  others that share its drag table.
- `hpr_forensics::synthetic::noise`: the correlated noise's spread and correlation, at any
  spacing; the port table at its measured points; the pulse's shape; the GNSS loop's steady
  lag.
- `hpr_forensics::synthetic::logger`: each pressure, acceleration and GNSS term held and
  failing when changed; each profile's rate, step and channels; the height formulas.
- `hpr_forensics::synthetic::truth`: set B's drag and tilted thrust, which keeps its impulse.
- `cargo xtask synthetic --check`, in CI on every platform: the census reproduced.

### References

- K. R. Czarnecki, W. J. Monta, NASA TN D-3516 (1966), as [the Mach error note][mach] cites it.
- NASA/TM-2008-215633, *Terrestrial Environment (Climatic) Criteria Guidelines for Use in
  Aerospace Vehicle Development*, §2.2.5.2.
- The National Association of Rocketry, [High Power Rocket Safety Code][hprcode].
- The research notes this page draws on: [sensor noise][noise], [the Mach error][mach],
  [GNSS in boost][gnss], [height formulas][heights], [smoothing methods][smoothing],
  [dispersion sigmas][sigmas].

[noise]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/altimeter-sensor-noise.md
[mach]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/altimeter-mach-error.md
[gnss]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/gnss-in-boost.md
[heights]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/altimeter-height-formulas.md
[smoothing]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/altimeter-smoothing-methods.md
[sigmas]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/dispersion-sigmas.md
[hprcode]: https://www.nar.org/safety-information/high-power-rocket-safety-code/
