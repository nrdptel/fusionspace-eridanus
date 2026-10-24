# Synthetic flights

See the 2,200 simulated flights, with known truths and their loggers'
[flight logs](../glossary.md#flight-log), on which [readings](log-readings.md) and 90% intervals
will be tested.

## In short

- **What it models:** 2,200 flights of the ten public RocketPy designs that HPR Sim, FusionSpace
  HPR's simulator, validates against, in three sets on fixed [seeds](../glossary.md#seed). Each
  flight draws its motor, drag, wind, rail angle, site and air, and one of six simulated loggers,
  each like a family of real loggers that FusionSpace HPR reads. The logger writes what its
  sensors would: a pressure with an offset, a gain, a drift, fast and slow correlated noise and
  the port's error near the speed of sound; an accelerometer with its scale, bias, noise and
  range; a GNSS fix that lags in boost and drops out. Each flight's truth is HPR Sim's trajectory,
  every 0.01 s, with its [dynamic pressure](../glossary.md#dynamic-pressure) and
  [Mach number](../glossary.md#mach-number).
- **Sources:** the pressure sensors' and accelerometers' datasheets and the noise measured in
  public logs ([the sensor noise note][noise]), with some terms assumed where no source states
  them ([The noise, term by term](#the-noise-term-by-term)); a wind-tunnel test of the static
  pressure along an ogive-cylinder through Mach 1 ([the Mach error note][mach]); the ejection
  pulse of the public Pnut log that Debrief ships; the lag of hobby GNSS receivers in boost
  ([the GNSS note][gnss]); the NAR's [high-power safety code][hprcode] for the wind's limit.
- **How well it is validated:** not against real logs: the noise follows datasheets and a few
  public logs, and the flights are HPR Sim's own, so a check on them shows how well a method
  recovers a known trajectory, not how real flights behave. Each drawn noise term is pinned by a
  test that holds it to its stated value on a truth built for it, and fails when it is changed:
  the tests show the generator draws what it says, not that the terms are right. A flight drawn
  again is the same to the bit, on one platform. No test yet shows that the terms together look
  like a real log of the same family:
  [#725](https://github.com/nrdptel/fusionspace-eridanus/issues/725) tracks that comparison
  ([evidence row G32](../evidence.md#g32)).
- **What it leaves out:** the bay's vent lag, a temperature term, clock drift, vibration, the
  logger's own speed column, and flights at or above Mach 1.18, which none of the 2,200 reaches
  ([What it leaves out](#what-it-leaves-out)).

## Why simulated flights

A real log has no truth beside it. The altimeter's own apogee comes from the same pressure trace
that a reading takes it from, so the two agreeing shows they read the trace alike, not that
either is right ([Flight-log readings](log-readings.md#checked-against)). To learn how often a
90% interval holds the truth, the truth must be known, on many flights.

Each reading and fit is to state a 90% interval, and
[ADR-257, calibrated intervals](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md)
asks that each be tested on at least 1,000 flights whose truth is known: it must miss about 5%
of the time on each side. The sets were fixed before any interval was written, and by rule no
interval is tuned on the flights it is judged on
([ADR-280, the synthetic flights first](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0280-m7-2-in-seven-the-synthetic-flights-first.md)).

A simulated flight knows its truth, but a check on it is only as good as its noise. A model that
adds white noise alone flatters an interval: real barometer noise is correlated from sample to
sample, so its errors don't average away as fast ([the smoothing note][smoothing] §6). So most
terms here come from a measurement or a datasheet, and the few that no source states are said
to be assumed where they are described.

A check on these flights shows how well a method recovers HPR Sim's trajectory from noisy
sensors. It doesn't show how well HPR Sim predicts a real flight: that is
[M7.3, a flight against its simulation](../decisions-and-roadmap.md#m7-3)'s question, on real logs.

## The flights

The sets each flight belongs to, what it draws, and the loggers that record it.

### The sets

Three sets, each on its own seed, written down before any method was tuned:

| Set | Flights | Seed | Model flown |
|---|---:|---:|---|
| Tuning | 200 | 2026101700 | 100 of A's and 100 of B's, alternating |
| A | 1,000 | 2026101701 | HPR Sim's |
| B | 1,000 | 2026101702 | its drag, wind and thrust reshaped |

By rule, anything fitted or tuned uses the tuning set, and sets A and B only judge. Flight $k$
of a set draws each group of inputs (its design, motor, drag, wind, rail, site, model and
logger) from its own stream, keyed by the set's seed, $k$ and the group, so a set's first
flights stay the same if it grows, and an input added to one group moves no other group.

Set B flies a model that differs from HPR Sim's on purpose, so that a method can't pass by
knowing HPR Sim's model:

- **The drag** rises differently through Mach 1: the zero-lift drag is multiplied by
  $1 + a\exp(-x^2) + bM$, where $x = (M - 1)/0.2$, with $a$ between −0.2 and 0.2 and $b$
  between −0.05 and 0.05.
- **The wind** grows with height as a power law, its exponent 0.14 to 0.20, the drawn speed at
  18.3 m (60 ft), after NASA/TM-2008-215633 §2.2.5.2. That report ties the larger exponents to
  lighter winds; here the exponent is drawn on its own. Set A's wind is the same at every height.
- **The thrust curve** is tilted: its thrust times $1 + \tau(2t/t_b - 1)$, with $\tau$ between
  −0.3 and 0.3, then scaled back to the same total impulse.

### What a flight draws

| Input | Range | Why |
|---|---|---|
| design | one of the ten | the RocketPy designs, each with its example's parachutes; Cavour's has none, so it lands without one, as on a failed recovery |
| total impulse | 0.94 to 1.06 times | its half-width twice the 2.5 to 3.1% spread (1σ) of repeat certifications ([the dispersion note][sigmas] §2); drawn evenly, its own spread is 3.5%, wider |
| burn time | 0.90 to 1.10 times | its half-width near the 10.7% spread (1σ) of the one burn-time batch found, of small black-powder motors (§2); drawn evenly, its own spread is 5.8%, narrower |
| zero-lift drag | 0.8 to 1.2 times | its half-width twice OpenRocket's 10% default; drawn evenly, its spread is 11.5%, narrower than the 16% coast fits suggest (§4) |
| wind | up to 20 mph (8.94 m/s), from any direction | the safety code's limit |
| rail | 0° to 10° from vertical, toward any direction | inside the code's 20° |
| site | 0 to 1,500 m (0 to 4,921 ft) above sea level | sea-level fields to the high deserts |
| air | the 1976 standard, −15 to +15 K | a day off standard |

A range's half-width is its largest departure from 1. Drawn evenly over ±h, a value spreads by
h/√3 (1σ), the spread the rows compare with their sources'.

The loggers convert pressure to height by the standard atmosphere, so a day off standard is a
real error in their heights: on a cold day a barometer reads high. The readings must carry it.

The truth is the flight's center of mass, every 0.01 s, and its events: liftoff, rail exit,
burnout, apogee, each charge and deployment, and landing. Its heights are above the ground; a
reading's truth is the height above where the center of mass stood at liftoff. The specific
force along the rocket's axis, at the nose tip where HPR Sim records the acceleration, is what
an accelerometer would feel; under a canopy the rocket is taken to hang vertically.

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
does ([the height formulas note][heights]); the two differ by about 0.83 m (2.7 ft) in 2 km
(6,562 ft).

## The noise, term by term

What each sensor adds to the truth, where each term comes from, and a worked example.

### The pressure

The logged pressure is the true pressure plus seven terms, then rounded to the logger's step.
The sections are those of [the sensor noise note][noise] unless another note is named:

| Term | What it is | Drawn per flight | Source |
|---|---|---|---|
| offset | the sensor's absolute error, which the pad's zero takes out of the heights | ±100 Pa for an MS5607 and PerfectFlite's; none for a Bosch-class part | §2, §6 |
| gain | an error proportional to the change from the pad | ±0.25% for an MS5607, ±0.1% for PerfectFlite, ±0.08% for a Bosch-class part | §1, §2, §6 |
| drift | a slow change over the flight | 1 to 3 Pa over 30 to 300 s, either way | §3 |
| fast correlated noise | the noise measured on real logs' pads | 0.5 m (1.6 ft) at 50 Hz, correlation 0.5 | §3, §6 |
| slow correlated noise | the noise that averaging over a second keeps | 0.24 to 0.29 m (0.79 to 0.95 ft), time constant 0.65 to 1.26 s | §3 |
| white noise | the datasheet's | 2.3 Pa (RRC3), 2.4 to 13 Pa (AltOS), 0.2 to 6.6 Pa (Bosch class); none for PerfectFlite | §6 |
| port error | the static port's error near Mach 1 | the port 1 to 5 calibers behind the nose's shoulder, assumed | [the Mach error note][mach] §1–2 |

A charge adds an eighth, the ejection pulse, at each charge's time.

**What each profile's part gives.** An MS5607 zeroed on the pad holds ±1.0 mbar from 0 to 50 °C,
stated over 700 to 1,100 mbar (§2): read as a gain, that is ±0.25% of the change from the pad,
an inference, as the datasheet doesn't say how the error grows with the change. PerfectFlite
names no part, so its profile takes an MS5607's offset, an assumption, its maker's stated ±0.1%
of the height as a gain (§1), and no white noise beyond the noise measured on real logs.
Eggtimer and Featherweight name no part on most of their boards (§1), so the Eggtimer and 500 Hz
IMU profiles take a Bosch BMP3xx or DPS310-class part, 0.2 to 6.6 Pa by its setting and ±0.08%
relative (§2, §6): an assumption. The offset stays in the logged pressure, but a height zeroed on
the pad loses it; what is left after the zero is the gain and the drift.

**The fast correlated noise** is a first-order autoregressive process, a red noise: each sample
keeps part of the one before. On the pads of five Blue Raven logs at 50 Hz the spread is 0.43 to
0.62 m (1.4 to 2.0 ft), and the correlation from one sample to the next is 0.40 to 0.54 across
six windows of those logs, on the pad or at rest ([the sensor noise note][noise] §3); the
spreads are upper bounds that mix in wind and handling. The model takes
0.5 m (1.6 ft) and 0.5 for every barometer, and adds each part's white noise. It is drawn as an
Ornstein–Uhlenbeck process, a red noise in continuous time, so the correlation follows the time
between samples, whatever the logger's rate:

$$
\rho(\Delta t) = e^{-\Delta t/\tau}
$$

With 0.5 at 0.02 s, $\tau = 0.0289$ s. A 20 Hz logger's samples then correlate by 0.177, a
100 Hz logger's by 0.707 and a 500 Hz logger's by 0.933. The spread in pascals is the height's
times $\rho_\text{pad}\,g$: at a 500 m (1,640 ft) site in standard air, 0.5 m (1.6 ft) is
5.72 Pa.

**The slow correlated noise** is a second term of the same kind, slower: 0.24 to 0.29 m (0.79 to
0.95 ft), its time constant 0.65 to 1.26 s, each drawn per flight. Ten minutes of a BMP085 at
rest gave it (Sabatini and Genovese 2013, Tables 1–2; [the sensor noise note][noise] §3). The
fast term loses its correlation within a tenth of a second, so a reading that averages over a
second would see it shrink almost away; the slow one keeps most of its spread over that second.
One part measured on one bench is taken for every barometer: an assumption.

**The port error**, the static port's error where the pressure along the body changes near the
speed of sound, comes from a wind-tunnel test of the static pressure along an ogive-cylinder
from Mach 0.7 to 1.2 (Czarnecki and Monta, NASA TN D-3516, 1966, Fig. 5; [the Mach error
note][mach] §1–2). The port reads $C_p\,q$ off the true pressure. $C_p$ is read from the note's
trace of the report's figure, to about ±0.01, at the port's distance behind the shoulder, in
[calibers](../glossary.md#caliber), and the flight's Mach number, and held at its end values
outside the table. One caliber back at Mach 1, $C_p = -0.08$. In the worked example
below that reads 473 m (1,552 ft) too high, for a moment.

**The ejection pulse** has roughly the shape of the one on the public Pnut log that Debrief
ships, in seven samples 0.05 s apart from the charge: the height reads 8 m (26.2 ft) low at the
charge, 16 m (52.5 ft) high 0.1 s after it, and 3 m (9.8 ft) low from 0.25 s, then true again.
The shape is scaled by 0 to 2 per flight, a range assumed, as one log gives one pulse, and turned
into pressure at the density where the charge fires. No test yet compares it with real logs'
pulses ([evidence row G32](../evidence.md#g32)).

### The acceleration

The logged acceleration is the truth's axial specific force times $1 + s$, plus a per-flight bias
of up to 0.05 g and white noise, clipped at the part's range and rounded to its step. No source
states a bias left after the pad's zero, so the 0.05 g is assumed ([the sensor noise note][noise],
*What this suggests*; [evidence row G32](../evidence.md#g32)):

| Part | Noise density | Step | Range | Scale $s$ |
|---|---:|---:|---:|---:|
| high-g, ADXL375 class | 5 mg/√Hz | 49 mg | ±180 g | ±10% |
| low-g IMU axis | 0.16 to 0.4 mg/√Hz | 64/65,536 g | ±32 g | ±3% |

Each sample's noise is the density times $\sqrt{1.57 \cdot f_s/2}$, the noise bandwidth of a
single-pole filter at half the rate $f_s$ the logger samples at while climbing, an inference in
[the sensor noise note][noise] §4. The AltOS profile's high-g part at 100 Hz has 44.3 mg
(0.434 m/s²) per sample, after apogee too, when it logs at 10 Hz: the part's filter doesn't
change with what the logger writes. Past 32 g the low-g part clips: the high-range part that
takes over on a real IMU isn't modeled.

The tuning set's 500 Hz IMU flights, 38 of them, are also logged with the low-g part set to ±8 g,
in steps of 16/65,536 g: a full scale the BMX160 and the MPU-6000 offer on the same 16 bits, as
their datasheets state ([the sensor noise note][noise]), where the part holds its top value past
the range. The noise and the clock are the same as on its own log. Of these 38 flights, 28 have a
true top that reaches 8 g and 10 don't, as the
[synthetic report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/synthetic-flights.md#errors-on-the-tuning-set)
counts, so the readings' rule for a top clipped at its part's range is held on flights that reach
it ([Flight-log readings](log-readings.md#checked-against)); at ±32 g none does.

### The GNSS fix

A hobby GNSS receiver's navigation filter follows the rocket like a second-order loop. Under a
steady acceleration $a$ it settles behind by

$$
e = a/\omega_n^2
$$

where $\omega_n$ is the loop's natural frequency, drawn from 0.25 to 1.0 rad/s, with damping
$1/\sqrt{2}$. No receiver's loop has been measured against a truth. About 0.5 rad/s is near the
0.47 to 0.69 rad/s of the GNSS note's simulated filter, which fell 103 to 261 m (338 to 856 ft)
behind in boosts of 5 to 15 g, an inference ([the GNSS note][gnss] §4); one flyer's report of a
440 m (1,444 ft) lag, its acceleration not given, is larger than any of the simulated lags
(§3). Drawn evenly, two in three loops lie above 0.5 rad/s and lag less than 0.5 rad/s would:
the draws err toward a fix more accurate than a real receiver's. The damping is the usual choice
for such a loop, assumed
([evidence row G32](../evidence.md#g32)). At 0.5 rad/s, a steady 50 m/s² would leave the fix
200 m (656 ft) behind; a 2 s boost ends long before the loop settles, so the lag peaks after
burnout. Each axis has 3 m (9.8 ft) of white
noise, a fixed value, assumed. White noise flatters an average of fixes: a real receiver's error
wanders slowly, so averaging shrinks it less than here. No fix is written for 0 to 10 s from
liftoff and from burnout, each drawn per flight, as the GNSS note suggests, an assumption; the
fixes written carry no flag that they lag.

### The clock

The log starts 2 to 10 s before liftoff and ends 2 to 10 s after landing, each drawn. The AltOS
profile changes from 100 Hz to 10 Hz at its first sample after apogee.

### A worked example

An AltOS-like logger, launched from a sea-level pad, flies through Mach 1 at 1,000 m (3,281 ft)
above sea level, a geometric height, on a standard day, its port one caliber behind the nose's
shoulder.

1. **The air.** The standard atmosphere there has $p = 89{,}876$ Pa and density 1.112 kg/m³.
2. **The dynamic pressure** at Mach 1 is $q = 0.7\,p\,M^2$, which is 62,913 Pa.
3. **The port error** is $C_p\,q$, here $-0.08 \times 62{,}913$, which is −5,033 Pa: the port
   reads low.
4. **The height it writes.** 84,843 Pa is 1,473 m (4,833 ft) in the standard atmosphere, so the
   logger reads 473 m (1,552 ft) too high while the rocket is near Mach 1. Three calibers back
   the sign flips: $C_p = +0.03$ reads 1.9 kPa high, and the height low.
5. **The noise beside it.** The fast correlated noise is 0.5 m (1.6 ft) at the pad, 6.01 Pa at
   sea-level density, and the slow 2.9 to 3.5 Pa; the white noise 2.4 to 13 Pa. Near Mach 1 the
   port's error is about 840 times the fast correlated noise and 387 to 2,097 times the white,
   which is why a reading that leans on a barometer through Mach 1 must say so.

## Validation and limits

What the flights were checked against, and what they leave out.

### Checked against

**Each noise term, on a truth built for it.** The tests build a truth that stands still, or
climbs at a constant acceleration, with only the term under test switched on, and hold the log
to it: the offset as the residuals' mean, the drift as their slope, the white noise's spread to
within five standard errors, the fast and slow correlated noise's spreads and correlations, the
gain on a 2 km (6,562 ft) climb, the port's $C_p\,q$ at a measured point, the pulse at 0.15 s,
the accelerometer's scale, bias, spread (after apogee too) and clip, the GNSS lag and dropouts,
the log's span and rates, and its values lined up with the truth by the drawn pad span. Each
drawn noise term is checked again changed, and must then fail: the offset, drift, white noise
and gain at half and one and a half times their values, the others zeroed, halved, doubled or
moved (the port to 3 calibers, the GNSS loop to 1 rad/s). The fix rate and the spans are set to
other values, which the log must follow. Each
part's stated ranges are pinned to the values on this page, so a changed constant fails too.
The fixed 3 m (9.8 ft) GNSS noise is held to its value on each axis. These tests show the
generator draws what it says, not that the terms are right. The census, a file in the repository
([synthetic-flights.md](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/synthetic-flights.md)),
counts each set's flights by design and logger, and the ranges of eight quantities: apogee, top
Mach number and speed, flight time, wind, rail angle, site elevation and the air's offset.

**What the census shows.** Every one of the 2,200 draws flew. Apogees above the pad run from 411
to 4,810 m (1,348 to 15,781 ft), and top speeds from Mach 0.269 to under 1.18. Values are kept
to three significant figures, the digits every platform shares, each range's ends rounded
outward.
CI flies every flight again on macOS, Windows and Linux and holds the census to them.

### What it leaves out

- **Fast flights.** None of the 2,200 reaches Mach 1.18 (the census rounds its greatest up), so
  from there to the Mach 3.5 HPR Sim is built for ([the operating envelope](../VALIDATION.md#operating-envelope))
  there is no synthetic flight.
- **The bay's vent lag.** A bay's pressure lags the outside air through its vent holes. In an
  invented flight, three invented bays vented by the makers' rules moved the
  apogee by at most 2.1 cm (0.07 ft), but lagged the height by 0.4 to 135 m (1.3 to 443 ft)
  through Mach 0.9 to 1.07 ([the Mach error note][mach] §5, an inference; no hobby bay's lag has
  been measured).
- **A temperature term.** Not drawn. Below 4 km (2.5 mi) above the pad it is under 0.36 m (1.2 ft)
  at apogee on every site and day drawn, 0.3 m (1 ft) from sea level on a standard day, at a 4 Pa/K
  ceiling ([bay temperature][baytemp], an inference).
- **Clock drift.** The logger's clock is taken as exact.
- **The logger's own speed column.** Each logger filters its own speed its own way, so the
  synthetic logs carry none.
- **The rocket's attitude in the accelerometer.** Only the axial axis is logged, at the nose tip
  rather than where the logger sits, and under a canopy the rocket hangs vertically. Each
  parachute opens at full size after its lag, as the RocketPy examples define them, so a main's
  opening is a sharp spike.
- **Vibration.** The accelerometer's noise is the same through the boost as on the pad; a
  motor's vibration isn't drawn.
- **A GNSS error that wanders.** Each fix's noise is drawn afresh, so an average of fixes is
  quieter than a real receiver's, whose error drifts over seconds to minutes.
- **A sharp port jump.** The port's error is read linearly between the tunnel's Mach numbers,
  0.1 apart near Mach 1, so its jump is spread over that much speed; a real port's may be
  sharper.
- **A part's stated noise, where it is larger.** The noise from the density can understate a
  part's own stated figure: the MPU-9250's 8 mg at 94 Hz is 2.2 times what the density gives
  ([the sensor noise note][noise] §4). The low-g profile may be quieter than a real IMU.

## Tests and references

The tests that pin the flights, and the sources they follow.

### Tests

- `hpr_forensics::synthetic`: every draw inside its range and filling it, in every set; each
  set on its own seed; one flight drawn again the same to the bit, whether flown alone or after
  another that shares its drag table.
- `hpr_forensics::synthetic::noise`: the correlated noise's spread and correlation, at 10, 50
  and 100 Hz; the port table at its measured points; the pulse's shape; the GNSS loop's steady
  lag.
- `hpr_forensics::synthetic::logger`: each drawn pressure, acceleration and GNSS term held and
  failing when changed; the fast noise's correlation at 100 and 500 Hz and the slow noise's time
  constant; the GNSS noise's 3 m (9.8 ft) on each axis; the log's span and each part's rate;
  each profile's step and channels; the height formulas.
- `hpr_forensics::synthetic::held`: the 500 Hz IMU's part set to ±8 g, its range stated to the
  readings, and a true top past it read as clipped at the range, a floor that holds; a part with
  no lower setting writing no such log.
- `hpr_forensics::synthetic::truth`: set B's drag and tilted thrust, which keeps its impulse,
  and set B's three changes reaching its flights' inputs while set A's keep HPR Sim's.
- `cargo xtask synthetic --check`, in CI on every platform: the census reproduced.

### References

- K. R. Czarnecki, W. J. Monta, NASA TN D-3516 (1966), as [the Mach error note][mach] cites it.
- Sabatini and Genovese, *Sensors* 13(11):15692 (2013), as [the sensor noise note][noise] cites
  it.
- NASA/TM-2008-215633, *Terrestrial Environment (Climatic) Criteria Guidelines for Use in
  Aerospace Vehicle Development*, §2.2.5.2.
- The National Association of Rocketry, [High Power Rocket Safety Code][hprcode].
- The research notes this page draws on: [sensor noise][noise], [the Mach error][mach],
  [GNSS in boost][gnss], [height formulas][heights], [smoothing methods][smoothing],
  [dispersion sigmas][sigmas].

[noise]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/altimeter-sensor-noise.md
[mach]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/altimeter-mach-error.md
[baytemp]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/altimeter-bay-temperature.md
[gnss]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/gnss-in-boost.md
[heights]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/altimeter-height-formulas.md
[smoothing]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/altimeter-smoothing-methods.md
[sigmas]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/dispersion-sigmas.md
[hprcode]: https://www.nar.org/safety-information/high-power-rocket-safety-code/
