# Flight-log readings

See which readings HPR Sim takes from a flight log alone, the bound each states, and when it
withholds one instead of guessing.

## In short

- **What it models:** the readings taken from a [flight log](../glossary.md#flight-log) on its own,
  with no design file and no simulation: liftoff, apogee and its time, the top speed in the climb,
  landing, the flight time, the mean rate of descent, and each canopy's descent rate with the main's
  opening. They come from any logger's height track: its own barometric altitude, a pressure turned
  into a height by the 1976 standard atmosphere, or an altitude kept in a column of its own. An
  accelerometer along the rocket's axis adds the top acceleration and burnout; without one, burnout
  is a [floor](../glossary.md#floor) at the top speed's time. Each reading states a bound on its
  error and where it came from, or is **withheld** with one of 26 reasons. It never guesses. A GNSS
  (GPS) height gives no reading.
- **Sources:** the [running median](../glossary.md#running-median) is the Hampel filter at threshold
  zero, as Pearson and colleagues define it, and an accelerometer's trace is cleaned by the same
  filter at threshold five. The landing's thresholds and the least climb are those of Debrief, the
  project owner's earlier flight-log analyzer, and the flat top of a clipped accelerometer and the
  main's opening found back from landing are Debrief's, each with HPR Sim's changes. The bounds'
  terms come from the noise model of [the synthetic flights](synthetic-flights.md), the hypsometric
  equation, the safety codes' 3:1 thrust to weight and 20° rail, the National Weather Service's rate
  for a rapid change of pressure, the drag that sets a canopy's rate and slows its fall after it
  opens, and the 1976 standard atmosphere's density scale height ([References](#references)). The
  rest are HPR Sim's choices, recorded in [ADR-281, readings on every logger][adr-281], and
  [ADR-283, the rest of the readings][adr-283].
- **How well it is validated:** on the synthetic flights' [tuning
  set](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/synthetic-flights.md#readings-within-their-bounds),
  171 flights logged by a barometer, each logged again at 36 rates and steps and cut 6 ways as
  loggers that record from launch cut them, and the 71 of them with an accelerometer read again with
  it at 500, 100 and 20 samples a second, every reading given stayed within its bound. The largest
  share of its bound any error took was 0.717, on an apogee, and 0.715 on a top acceleration
  ([Checked against](#checked-against)). The bounds were tuned on those same flights, so this shows
  they are wide enough there, not on flights held apart. On an invented PerfectFlite log, every
  reading lands within its bound, and each but burnout's floor within the tighter one its test works
  out ([evidence row G10](../evidence.md#g10)). A real log has no truth beside it: the 1,058 records
  of the project's sample logs show which readings each logger gives (an apogee from 593 of them),
  not how right they are.
- **What it leaves out:** the Mach number and dynamic pressure; 90% intervals on the readings;
  loggers noisier than the tuning set's [six simulated logger
  profiles](synthetic-flights.md#the-loggers) and rockets slower off the pad than 2 g, either of
  which can be read outside its bound; air more than 15 K from standard, and any correction for the
  day's air, which sits in the bound instead; a canopy's filling time, which isn't in the opening's
  bound ([#779](https://github.com/nrdptel/fusionspace-eridanus/issues/779)); a fall under no
  canopy, read as one canopy's ([#780](https://github.com/nrdptel/fusionspace-eridanus/issues/780));
  a canopy far above the tropopause; accelerometer noise that runs from sample to sample, which
  would narrow burnout's bound ([#787](https://github.com/nrdptel/fusionspace-eridanus/issues/787));
  a top clipped at its part's range, held only by unit tests; and a board's three axes, which give
  no top acceleration ([#781](https://github.com/nrdptel/fusionspace-eridanus/issues/781); [What it
  leaves out](#what-it-leaves-out)).

## The method

Why the readings follow these rules, the track they come from, the filter applied first, and how
each reading is taken.

### Why these rules

Code: `hpr_flightdata::readings`
([API reference](../api/hpr_flightdata/readings/index.html)); on the command line
[`hpr analyze`](../cli.md#hpr-analyze); from Python, `FlightLog.readings()`
([Reading your own logger](../your-own-logger.md#from-python)). The readings came with
[M4.2d](../decisions-and-roadmap.md#m4-2d), the milestone that added `hpr analyze`, on
PerfectFlite's logs alone ([ADR-108][adr-108], the decision on how a log is read).
[M7.2b, readings on every logger](../decisions-and-roadmap.md#m7-2b), takes them from every
logger's height track and gives each a bound held on the synthetic flights
([ADR-281][adr-281]). [M7.2c, the rest of the readings](../decisions-and-roadmap.md#m7-2c), adds
the top acceleration and burnout from an accelerometer along the rocket's axis, the flight time
as a reading of its own, and the descent under each canopy ([ADR-283][adr-283]).

A barometric altimeter's height track is a good record of a flight's heights, with flaws a
reading must survive:

- **Steps.** It moves in steps of its resolution, a foot for a PerfectFlite, so the top of the
  climb sits flat for several samples.
- **Pulses.** When the ejection charge fires, the pressure inside the electronics bay jumps, and
  for a tenth of a second the height swings tens of feet away from the truth. Take the highest
  sample as the apogee and you read that swing.
- **Noise and drift.** The sensor's noise is correlated from one sample to the next, so it doesn't
  average away quickly, and its zero drifts.
- **The day's air.** A logger turns pressure into height by the standard atmosphere, so on a day
  warmer or colder than standard every height is off by a share of itself.

So every height and time comes from the track after a running median, which removes the pulse;
each reading states a bound that covers the rest; and each has a rule that says when the log can't
support it.

### The height track

Every reading of a height or a time comes from one track, the first of these the log holds. The
code each reading names as its `source` says which:

| `source` | the track | its zero |
|---|---|---|
| `barometer` | the logger's own barometric altitude, the record's primary track | the logger's zero, which it sets on the pad |
| `pressure` | a pressure channel, turned into a height by the 1976 standard atmosphere, as every logger documented does it ([the height formulas note][formulas]) | the pad, found from the samples |
| `altitude_channel` | a barometric altitude kept in a channel, above the ground or above sea level | the ground for the first; the pad, found from the samples, for the second |
| `gnss` | a GNSS receiver's altitude | none: no reading is taken |
| `accelerometer` | not a height: the accelerometer along the rocket's axis, for the top acceleration and burnout alone ([The accelerometer](#the-accelerometer)) | a fall in vacuum reads 0 and the pad +1 g, gravity put back where the logger took it out |

A GNSS height gives no reading. A receiver's navigation filter lags a climb and overshoots its top
by more than a barometer's bounds allow (`receiver_lag`), and its fix drops out or lags at liftoff
(`receiver_dropout`). An altitude whose sensor the log doesn't state, such as a table's `Altitude`
column read by its header, gives none either (`unstated_sensor`): a barometer's bounds can't be
stated for it. A column mapping can mark such a column as the barometer's own track
([Reading your own logger](../your-own-logger.md)).

A track's **step** is the logger's resolution where its reader knows it, a foot for a PerfectFlite;
otherwise it is the smallest gap between two of the track's values. A pressure's step becomes a
step in height that grows as the air thins, $\Delta h = \Delta p/(\rho g)$, with $\rho$ the
standard atmosphere's density at that height: 10 Pa is 0.83 m (2.7 ft) at sea level.

### The running median

The running median replaces each sample with the median of the samples within $K$ places of it,
the window cut short at either end of the log. The code sets $K$ from the log's own sample interval
$\Delta t$ so that the window spans 0.3 s: at a PerfectFlite's 20 samples a second, $K = 3$, seven
samples.

Pearson and colleagues define the Hampel filter by the window's median $m_k$ and its scale

$$
S_k = 1.4826 \times \operatorname{median} \lvert x_{k-j} - m_k \rvert
$$

A sample more than $t \cdot S_k$ from $m_k$ is replaced by $m_k$.
At $t = 0$ every sample is replaced, and the filter is the running median (§2, eqs. 1 and 2).

Two properties make it the right tool here:

- **It removes any pulse up to $K$ samples wide.** Such a pulse holds at most $K$ of the window's
  $2K + 1$ samples, so the median is never one of them. On the public log below, the ejection
  pulse's high side is two samples wide.
- **It reads a clean peak a little low, and never high.** At 20 samples a second, the apogee of a
  noise-free trace reads at most **0.077 m (0.25 ft)** low. The steps behind that number:
  1. At the highest sample, $K + 1$ of the window's $2K + 1$ samples lie within $\lceil K/2 \rceil$ places of
     it ($\lceil K/2 \rceil$ is $K/2$ rounded up). So the median is no lower than the altitude that far away.
  2. The true peak lies within half a sample of the highest sample, so that altitude is at most
     $(\lceil K/2 \rceil + \tfrac12)\, \Delta t$ from the true peak in time.
  3. At apogee the vertical speed is zero, so drag has no vertical part and only gravity curves
     the path. In that time the height falls at most $g\, ((\lceil K/2 \rceil + \tfrac12)\, \Delta t)^2/2$.
  4. With $K = 3$ and $\Delta t = 0.05\,\mathrm{s}$, that is $9.80665 \times 0.125^2/2$, or $0.077\,\mathrm{m}$.

  A property test checks this for every $K$ from 1 to 6 and every place the peak can fall between
  samples.

Debrief uses the Hampel filter at $t = 4$ over the same window. FusionSpace HPR doesn't, because of
what the public Pnut log shows. Around its ejection pulse the trace dips below itself just before
the pulse and stays lower after it. Those samples sit in the pulse's own window and widen its scale
$S_k$, and the Hampel filter keeps the pulse: its highest value is the pulse's 1,028 ft above the
altimeter's zero, against the 1,009 ft apogee the altimeter states. After the running median, the
apogee reads 1,010 ft above the zero, which is 1,009 ft above the pad. The invented flight below
repeats that shape, so the tests show the difference where anyone can run them.

### The pad and the climb

The readings measure heights from the pad, so the pad must be found first, and only from samples
taken before the rocket moved. Three rules do it, each with a number set from the noise:

1. **The allowance.** The filtered track may sit up to 2 m (6.6 ft) from the pad and still be on
   it, or as far as the pad's own filtered samples stray if that is more, plus one step. The 2 m
   (6.6 ft) is four times the synthetic flights' fast correlated barometer noise, 0.5 m (1.6 ft), HPR
   Sim's choice. Half a foot, the rule before
   [M7.2b](../decisions-and-roadmap.md#m7-2b), read liftoff up to 6 s early on a noisy
   500 Hz barometer, the noise lifting the pad's last samples above it; 1 m (3.3 ft) read one
   tuning flight 6.8 s early, against a 4.5 s bound; 2 m (6.6 ft) never did.
2. **The climb** begins at the first filtered sample twice the allowance and a step above where
   the log starts: 4.61 m (15.1 ft) on a PerfectFlite, whose step is a foot. The pad's noise lifts
   the filtered track by at most the allowance, from below at the start or from above at the climb,
   so twice that is out of its reach.
3. **The pad** is the median of the samples up to the last one within half that rise of the start,
   before the climb: 2.30 m (7.6 ft) on a PerfectFlite. The median means that no one sample's jitter
   sets the pad, and the half rise means the climb lends it few samples.

**The pad's error.** A log that starts just before liftoff holds little of the pad, and a rocket
may already be moving when its first sample is taken. A rocket that leaves the pad at $A = 2g$ or
more, moving at $v$ when the log starts, climbs $v t + \tfrac12 A t^2$ in the time $t$ the track
stays within half the rise of its start. That climb can't pass the rise $r$, the noise allowed at
either end, so its samples bound $v$, and with it how far up the rocket may already be:

$$
\begin{aligned}
v &\le (r - \tfrac12 A t^2)/t, \\
e_{\mathrm{pad}} &= v^2/(2A)
\end{aligned}
$$

That much enters the liftoff, apogee and landing bounds. The acceleration is the least the safety
codes allow: the NAR's high-power code asks that a rocket weigh no more than a third of its motor's
certified average thrust, and Tripoli's allows less than 3:1 only with an active stability system.
Thrust three times the weight leaves twice gravity to climb with. The tuning set's slowest rocket
averages 30 m/s² over its first 5 m (16 ft).

On the public Pnut log, for example, the filtered track stays within 2.30 m (7.6 ft) of its start
for 0.40 s. A rocket at $A = 19.6\,\mathrm{m/s^2}$ that climbed no more than the 4.61 m (15.1 ft) rise in
that time was moving at no more than $(4.61 - 19.6 \times 0.40^2/2)/0.40$, or 7.60 m/s
(24.9 ft/s), when the log started, so it can't have been more than $7.60^2/(2 \times 19.6)$, or
1.47 m (4.8 ft), up. That is within the allowance, so the pad stands, and the 1.47 m (4.8 ft) is part of
the apogee's 27.4 m (90 ft) bound.

**A short pad.** Where the pad's error could pass the 2 m (6.6 ft) allowance, the pad is short
(`short_pad`): the log starts after liftoff, as a logger that records from launch writes its zero
and then the climb, or holds too little of the pad to tell liftoff by. What happens next depends on
the track's zero:

- **A zero set on the pad**, the logger's own altitude as its format's reader reads it: the
  heights are read from that zero. Liftoff is withheld unless a sample before apogee sits
  within the allowance of the zero, and so is the top speed if it peaks within 0.5 s of the log's
  start, where a faster climb may have come before the first sample.
- **A zero only a word vouches for**, a mapped table's altitude or a channel whose header calls
  it above the ground ([#646](https://github.com/nrdptel/fusionspace-eridanus/issues/646), a
  wrong unit or a wrong zero in a mapping): such a zero may sit anywhere within 19.9 m (65 ft) of
  the pad ([below](#before-any-reading)), and with no pad of samples, nothing places it better
  than the first sample does. That sample reads `s` above the zero after a climb of up to the
  pad's error `C`, so the zero stands between `s` below the pad and `C − s` above it. The heights
  are read from it, as from a zero set on the pad, and the larger distance,
  `max(|s|, min(19.9 m, C − s))`, enters the liftoff, apogee and landing bounds as the pad's
  error. A log too sparse to bound the climb before it takes the whole 19.9 m (65 ft).
- **Any other track**, a pressure or an altitude above sea level: no zero places the pad, so
  every reading is withheld, the top acceleration too, as it is searched for up to apogee.

### Each reading

Each reading names its `method`, and its `provenance`: `measured` when it is read straight off the
track, `derived` when it is worked out from it, and `clipped` for a top acceleration at its
part's range or held flat at its top, which is a floor ([The accelerometer](#the-accelerometer)).
There are nine methods:

| reading | rule | `method` | `provenance` |
|---|---|---|---|
| liftoff | the last sample before apogee whose filtered height is within the pad's allowance | `last_on_the_pad` | `measured` |
| apogee | the filtered track's highest value; its time the middle of the run of samples that hold it | `highest_filtered` | `measured` |
| time to apogee | apogee's time less liftoff's | | |
| top speed | the steepest least-squares slope of the filtered track over 0.5 s, from liftoff to apogee | `steepest_slope` | `derived` |
| landing | the first sample after apogee back within the pad's allowance that stays below it, 3 m (9.8 ft) and a step for a second, and no sooner than a fall from rest in vacuum could lose that height | `back_on_the_pad` | `measured` |
| flight time | landing's time less liftoff's | `liftoff_to_landing` | `measured` |
| mean descent rate | the filtered height lost from apogee to landing, over the time taken, part of landing's reading | | |
| top acceleration | the axial accelerometer's highest reading over the burns, searched from liftoff's earliest time, gravity in | `highest_reading` | `measured`, or `clipped` |
| burnout | from the accelerometer: where its reading is least after it falls through zero, the end of the thrust's fall | `least_after_thrust` | `measured` |
| | without it: no sooner than the top speed's time, a floor | `speed_peak` | `derived` |
| drogue descent | the filtered height lost from apogee to the main's opening, over the time taken | `leg_chord` | `derived` |
| main descent | the same from the main's opening, found back from landing, to landing; from apogee where the descent shows one canopy | `leg_chord` | `derived` |

Liftoff is the last sample before the track shows the rocket climbing out of the pad's reach, so
it comes after the true liftoff by the time that climb took.

The **top speed** is HPR Sim's own, worked out from the heights rather than read from a speed
column a logger writes: every logger's track can give one, and its bound can be stated. A 0.5 s
fit averages the barometer's correlated noise down to a few meters a second, and is short enough to
keep most of the peak. It needs at least 5 samples in the fit, so a log sampled slower than every
0.125 s has none (`sampled_too_slowly`). It is withheld at 227.7 m/s (747 ft/s) and above
(`transonic`): Mach 0.8 in the coldest air the bounds allow below 20 km (12 mi), the standard
atmosphere's 216.65 K less 15 K. From Mach 0.9 the
pressure behind an ogive's shoulder jumps on its way to Mach 1.0 ([the Mach error note][mach] §1–2),
so a speed derived from the heights there can be far off. Debrief's two refusals stay: a
speed above 4,000 m/s (13,123 ft/s) (`implausible_speed`), and a climb whose speed swings below
zero by more than 20% of its top (`noisy_speed`).

**Landing** is the first sample within the pad's allowance, at least Debrief's 2 m (6.6 ft), that
stays under the allowance and 3 m (9.8 ft) more, and a step, for Debrief's second. So it comes
before touchdown by the time the last few meters took: about 0.4 s under a main at 6 m/s
(20 ft/s) on a PerfectFlite's log. A barometer drifts during a flight, so the ground
can read a little above or below the pad. The mean descent rate covers the drogue and the main
together; each canopy's own rate is read from the shape of the descent
([The canopies](#the-canopies)).

The **flight time** is a reading of its own, `flight_time`: landing's time less liftoff's, its
bound the sum of theirs, so it needs both. Landing's reading no longer carries it.

The **top acceleration** comes only from an accelerometer, never from the heights. Differencing
twice turns a track's steps into spikes: at 20 samples a second, a one-foot step differenced twice
is $0.3048/0.05^2 = 122\,\mathrm{m/s^2}$, over 12 g. So a log with no accelerometer gives none
(`no_accelerometer`), and its burnout is the floor at the top speed's time.

### The accelerometer

The top acceleration and burnout come from an accelerometer along the rocket's axis, and only from
one the log names as that axis ([ADR-283][adr-283] §1–4). A table in the code lists each logger
family's axial column, such as AltOS's `acceleration` or the Raven's `Axial Accel (Gs)`, with
whether the family says its readings hold gravity, and the part's range where the family states one.
A column whose name holds the word `axial`, as a table's or a mapping's may, is read with neither
known. A board's three axes don't say which lies along the rocket, and their magnitude never falls
below zero at burnout, so they give neither reading (`axis_unstated`).

**What an accelerometer reads.** It doesn't feel gravity's pull; it feels every other force on the
rocket, over its mass: the [specific force](../glossary.md#specific-force). Along the axis that is
the thrust less the drag, over the mass. Standing on the pad, the part reads +1 g, the pad's push;
coasting, it reads the drag's pull back, below zero; falling in vacuum, it would read 0. Some
loggers write what the part reads; others take gravity's 1 g out on the pad, so the pad reads 0. HPR
Sim puts the 1 g back where it was taken out, so every reading is the specific force, +1 g on the
pad. So is the top acceleration: it holds the 1 g, where `hpr sim`'s `max_acceleration_m_s2` is the
rocket's own acceleration, from the equations of motion. Less 1 g, the top acceleration is the
rocket's own acceleration up on a vertical climb. A part mounted nose down writes the thrust
negative. A pad reads 1 g or 0 either way up, and a rocket leaves it at 3 g or more, so the first
reading past 2 g, either way, is the thrust's: where it is negative, the readings are turned over,
and a canopy's shock later, however large, doesn't turn them. A sample that isn't a number is left
out, and so is one that repeats the time before it, the first kept, as the height track keeps
AltOS's rows that share a time. A part sampled faster than every 0.002 s, 500 times a second, the
fastest the bounds were checked at, gives neither reading (`sampled_too_fast`).

**A cleaned trace.** A charge's shock or one bad sample could turn the readings over, end the thrust
early or pass for burnout. So those are found on a cleaned trace: a Hampel filter over 0.1 s puts
back to the window's median each sample more than five robust standard deviations from it (1.4826
times the window's median absolute deviation), as Pearson and colleagues define it. It takes out a
spike narrower than half its window and keeps a step or a steady run as it was. The 0.1 s and the
five are HPR Sim's choice. The top acceleration is read off the samples themselves, not the cleaned
trace.

**Gravity, by three witnesses.** Whether the 1 g is in the readings is settled by three witnesses,
each within 0.25 g of the level it reads:

- **the family's word,** where the logger's maker says;
- **the pad,** read off the accelerometer itself: the steady level before the thrust lifts it by 1
  g, if that comes no later than liftoff's latest time by the heights. About 1 g holds gravity;
  about 0 doesn't;
- **the coast,** the last 1 s before apogee's earliest time, where a rocket falls freely but for a
  small drag. About 0 holds gravity; about −1 g doesn't.

A witness that reads neither level, such as the pad of a log that starts in the thrust, says
nothing. Gravity is settled when at least one witness says and none disagrees. Otherwise the top
acceleration is withheld (`gravity_unstated`), and burnout comes from the top speed. The 0.25 g
allows a 20° rail's 0.06 g, a part's zero offset and the small drag near apogee, HPR Sim's choice.
The pad is read off the accelerometer rather than as the samples before liftoff's earliest time by
the heights: most Raven logs start about 0.5 s before liftoff, against a liftoff bound near 0.9 s,
and that rule, tried when this one was chosen, withheld 69 of the 80 Raven CSV records. The coast
settles the Raven's logs that start in the thrust, whose family doesn't say ([ADR-283][adr-283]).

**The top acceleration** is the highest specific force while the motor burns. A burn runs from a
cleaned reading past 2 g to where the cleaned trace, averaged over 0.05 s, next falls through zero,
or to apogee where it doesn't. A span counts as a burn only if the cleaned trace holds past 2 g for
0.5 s in all, as a canopy's or a charge's shock or a knock doesn't; where none does, the first span
is the burn, a short motor's. Measured once, when the rule was chosen, on the sample logs: of the
spans after the first, every one that started within 3 s of apogee held past 2 g for at most 0.47 s,
and every longer one started more than 3 s before it, though nothing beside those logs says which of
the longer ones are burns. The 0.5 s is HPR Sim's choice. The burns are searched from liftoff's
earliest time by the heights, or from the log's start where liftoff is withheld: a rocket at rest
reads 1 g, so anything more before it moves is a knock or a bump on the pad. With no burn, the top
is searched from there to apogee. A charge or a canopy's shock in a coast lies outside the burns, so
it isn't taken for the top; a second burn, an air start's or a sustainer's, is searched too. Its
bound adds the part's own errors, the noise around the top and what a peak between samples can hide:

$$
\begin{aligned}
b_a &= s_A\, a + \beta + p \\
&\quad + g(1 - \cos 20^\circ) \\
&\quad + 5\sigma + \Delta + j
\end{aligned}
$$

Here $s_A a$ is a tenth of the reading $a$, for the part's scale; $\beta$ is 0.05 g of bias; $p$ is
how far the pad strays from the level it witnessed; $g(1 - \cos 20^\circ)$, 0.06 g, is the share of
gravity a part that zeroes on a tilted rail takes out, counted only where the logger took gravity
out; $\sigma$ is the noise, measured within 0.5 s either side of the top; $\Delta$ is a step; and
$j$ is the jump to the sample either side. On the tests' trace, a thrust that rises to a top of 6 g,
58.8 m/s², the scale alone is 5.9 m/s² of the bound and the bias 0.49 m/s².

**Clipped.** A part reads no higher than its range. A top is `clipped` where it reaches the part's
stated range, within two steps and 0.25 g, or holds flat at its top for 4 samples or 0.05 s,
whichever is more samples, as Debrief tells a part that clipped. Flat means within Debrief's 0.3% of
the top or 0.25 m/s², narrowed to half the part's noise where that is less, and never below half a
step: a part at its range writes one value, while a thrust's plateau read by a quiet part still
scatters by its noise. Tried when the rule was chosen, Debrief's tolerance alone called 82 of the
tuning set's 284 tops clipped, though none reaches its part's range ([ADR-283][adr-283]). A
clipped top is a [**floor**](../glossary.md#floor): the truth is no lower than the reading less its
bound, and may be higher by any amount. The JSON marks it `is_floor`, its provenance `clipped`, and gives the part's
range as `range_m_s2` where the family states one; `hpr analyze` says the top may have been higher.

**Burnout from the accelerometer.** In the climb the reading is $(T - D)/m$, the thrust $T$ less the
drag $D$ over the mass $m$: above zero while the thrust beats the drag, below zero once it doesn't,
and after burnout $-D/m$, which eases as the coast slows. So the reading falls through zero as the
thrust tails off, and is least at burnout, where the thrust's fall ends and the drag's easing
begins. Debrief takes the fall through zero as burnout, which comes earlier, by as long as the
thrust takes to tail off below the drag. The cleaned trace is first averaged over about 0.05 s, an
odd number $2h + 1$ of whole samples: 27 at 500 samples a second, 7 at 100 and 3 at 20, whose first
and last lie $2h$ intervals apart, 0.052, 0.06 and 0.10 s. That cuts its noise by the square root of
the $n$ samples averaged. With $N$ five times the averaged noise and a step,

$$
N = 5\sigma/\sqrt{n} + \Delta,
$$

an averaged sample lies within $N$ of its truth but about once in 1.7 million samples, so the
samples next to burnout read within $2N$ of the least. The thrust falls steeply and the coast's drag
eases slowly, so burnout lies near the first of them. The reading is that first sample, less half
the window, $h$ intervals, since the average reads the least late by that much. Its bound is the
span of those samples, an interval, and the window's $2h$ intervals, at least two intervals. If the
averaged trace never climbs $4N$ back above its least before apogee, or the least lies within $3N$
of zero, too little drag to see, the accelerometer gives no burnout. Nor does it after more than one
burn: the tuning set's ten designs each fly one motor in one stage, so it holds no bound for the end
of a second.

On the same trace, sampled 100 times a second with a noise $\sigma$ of 0.087 m/s², the thrust falls
through zero to −0.6 g at burnout, 4.1 s, and the drag eases to nothing by apogee, 8 s. Averaged
over seven samples, $N = 5 \times 0.087/\sqrt{7}$, 0.16 m/s². The drag eases by 0.6 g over 3.9 s,
1.5 m/s² a second, so the samples within $2N$ of the least span about 0.22 s, and with an interval
and the window's 0.06 s the bound is about 0.3 s. The reading falls within a sample, 0.01 s, of 4.1
s.

**Burnout from the top speed.** A rocket gains speed only while the thrust beats the drag and its
weight, so the top speed comes at or before burnout, the last burn's end. Where the accelerometer gives no burnout, or
the log has none, burnout is the top speed's time, a floor: it came no sooner than that less its
bound, and may have come later by any amount, as a motor whose thrust tails off below the drag and
the weight burns on after the rocket stops gaining speed. The bound is the time a coast slowing at
$g$ takes to lose twice the speed's 5 m/s (16 ft/s) noise, by which the fit can place the peak late,
then half the 0.5 s fit and an interval. On the invented PerfectFlite flight, which has no
accelerometer, $2 \times 5/9.81 = 1.02$ s, half the fit's 0.25 s and the 0.05 s interval add to 1.32
s, and `hpr analyze` reads burnout no sooner than 0.88 s, the top speed's time, 2.20 s, less that
bound. The true burnout is 2.10 s, 0.10 s before the top speed's time. The JSON marks such a burnout
`is_floor`, and a top speed withheld, as `transonic` on a fast flight, withholds it too.

### The canopies

Each canopy's descent rate comes from the filtered height track, its legs split where the main
opens. The opening is found back from landing as Debrief finds it, with two changes of HPR Sim's
([ADR-283][adr-283] §6).

**The main's opening.** Near the ground the rocket hangs under its main. The descent's rate is
the least-squares slope of the filtered track over 1 s, widened where needed to hold four of the
track's steps; a fit that would need more than 10 s is refused (`step_too_coarse`). The main's
rate is the median of that rate over the last 2 s before landing, or the fit's span where that
is longer, its last 0.25 s left out.
Walking back from landing, the run of samples whose rate stays within 1.6 times the main's, or
3 m/s (9.8 ft/s) more if that is more, is the main's descent, and its first sample is the opening.
These marks are Debrief's, set on its own logs.

HPR Sim walks on the rate's running median over 2 s, or twice the fit where that is longer, so a
spike of the fit's noise on a long leg doesn't end the walk, while a step, the opening, stays where
it is. On the raw rate, tried when the rule was chosen, the walk missed the drogue on 2 of the 87
tuning flights that came down under two canopies: a noise spike on a 125 s main leg stopped it 125 s
after the opening.

**At the ground's density.** A canopy falls at the speed where its drag balances the weight,
$v = \sqrt{2mg/(\rho C_D S)}$ (the [descent rate](../glossary.md#descent-rate)), so its rate goes as
$\rho^{-1/2}$: up high, in thinner air, one canopy falls faster than it does near the ground, and
from a high apogee its early descent would read as a drogue's. So HPR Sim scales each rate, read
at the height $h$ above the pad, to the ground's density:

$$
v_0 = v \exp(-h/2H)
$$

$H$ is the troposphere's least density scale height, 7,000 m (22,966 ft). In the 1976 standard
atmosphere, whose temperature falls by $L$, 6.5 K/km, the density falls as $\exp(-h/H)$ near any
height, with $H = RT/(g - RL)$ for air's gas constant $R$ and the temperature $T$. At the coldest
the bounds allow, the tropopause's 216.65 K less 15 K, $H$ is 7,289 m (23,914 ft), taken down to
7,000 m (22,966 ft). In the troposphere a canopy's rate grows with height by no more than
$\exp(h/2H)$ at the least $H$, so the scaling can only slow a rate read high up, never quicken it,
and one canopy doesn't read as two. Compared at each rate's own density, when the rule was chosen, a
tuning flight that opened one canopy at apogee was named a main.

**A drogue, or one canopy.** A drogue came before the main if the descent from apogee to the
opening fell faster than 1.4 times the main's rate, by its median, as Debrief asks; and, HPR Sim
adds, if the 2 s just before the run fell faster than the run's mark, since a drogue whose rate
crosses the mark leaves the run's start inside the drogue's leg. Otherwise the descent shows one
canopy (`one_canopy`): the drogue's descent is withheld, and the main's runs from apogee to
landing. A rocket that came down under none reads the same way.

**Each leg's rate** is the filtered height it loses over the time it takes: the drogue's from
apogee to the main's opening, the main's from there to landing. A leg that loses no more than 3 m
(9.8 ft) or a tenth of its top's height, Debrief's least, is withheld (`short_leg`). Its bound is
the error two heights of the same track can hold between them, over the leg's time
([What each bound holds](#what-each-bound-holds)).

**The opening's bound.** A fit centered half a window before the opening sees only the drogue's
descent, faster than the mark, so the run starts no sooner than half a window and a sample
before the opening. After the opening the main slows the fall as quadratic drag slows any
falling body (Taylor, *Classical Mechanics*, §2.4),

$$
\frac{\mathrm{d}v}{\mathrm{d}t} = -g\left(\frac{v^2}{v_m^2} - 1\right),
$$

and from any speed it reaches the mark $v_1$ within

$$
\tau = \frac{v_m}{2g} \ln\frac{v_1 + v_m}{v_1 - v_m},
$$

where $v_m$ is the main's rate at the opening's height, the rate near the ground grown by
$\exp(h/2H)$. So the run starts no later than $\tau$, half a window and a sample after the
opening, and the bound is $\tau$, half the 1 s fit and two sample intervals: one for the sample
the run starts on, and one more as HPR Sim's margin. A canopy's
[filling time](../glossary.md#inflation-and-filling-time) isn't in it: the synthetic flights open
each canopy at once ([#779](https://github.com/nrdptel/fusionspace-eridanus/issues/779), the
opening's bound and a canopy's filling time).

On the invented PerfectFlite flight, the main's rate near the ground is 6.0 m/s (20 ft/s), so the
run's mark is 9.6 m/s (31.5 ft/s): 1.6 times the rate, more than 3 m/s (9.8 ft/s) above it. The
run back from landing starts at 21.40 s, 148.4 m (487 ft) up, where the main's rate is 6.06 m/s
(19.9 ft/s), the rate near the ground grown by $\exp(148.4/14{,}000)$. From there the main slows
any fall to the mark within

$$
\tau = \frac{6.06}{19.6} \ln\frac{15.66}{3.54} = 0.46\,\mathrm{s},
$$

so the opening's bound is $0.46 + 0.50 + 0.10 = 1.06\,\mathrm{s}$, as `hpr analyze` prints it.
The main truly opened at 21.14 s, 150 m (492 ft) up: the reading is 0.26 s late, as the 1 s fit
still sees some of the drogue's faster fall just after the opening. The drogue's leg loses
241.7 m (793 ft) in 11.125 s, 21.7 m/s (71 ft/s). That is below the drogue's true 25 m/s
(82 ft/s), as the leg starts at apogee, where the rocket falls from rest; over the same span the
true rate is 21.74 m/s (71.3 ft/s).

## Bounds and refusals

What each bound adds up, a worked example, the checks a log passes before any reading, and the
reasons a reading is withheld.

### What each bound holds

Each reading states the bound its error stays within. A bound is a sum of terms, each the size of
one error the synthetic flights draw, or measured on the log itself. Most of an apogee's bound is
the day's air.

- **Apogee height.** The day's air and the barometer's gain as shares of the height $h$, then the
  noise, the pad's error, the drift since the log began, a step at the pad and one at the peak,
  and the median's peak bound:

  $$
  \begin{aligned}
  b_h &= (s_T + s_G)\, h + n + e_{\mathrm{pad}} \\
  &\quad + d\, t_a + \Delta_0 + \Delta_a + b_{\mathrm{med}}
  \end{aligned}
  $$

- **Apogee time.** The time the top of a coast takes to fall by the noise and a step,
  $\sqrt{2 (n + \Delta_a)/g}$, and the longer interval either side of the peak's flat run.
- **Liftoff.** The time a rocket at $A = 2g$ takes to climb the allowance and the pad's error,
  once above the pad and once below, $\sqrt{4 (a + e_{\mathrm{pad}})/A}$, and the longer interval
  either side of the sample, twice.
- **Top speed.** The day's air and the gain as shares of the speed $v$; the port's error (the
  port is the hole that lets the outside air in to the barometer) while the rocket accelerates at
  $a_1$, $0.02\, v a_1/g$; the fit cutting the peak, $(a_1 + a_2)\, w/4$, where $a_1$ and $a_2$ are
  the climb's change of speed before and after it, measured on the fits; and the noise in a speed.
- **Landing.** The time the descent takes to fall the allowance, the noise, the pad's error, the
  logger's height above the ground, a step and the drift, at its rate near the ground taken slow by
  a step and twice the noise; and the longer interval either side of the sample, twice.
- **Flight time.** Liftoff's bound and landing's, added.
- **Mean descent rate.** The apogee's height bound and the height landing is read early by, then
  the rate times the apogee's and landing's time bounds, all over the descent time.
- **Top acceleration.** The part's scale as a tenth of the reading, its bias, how far the pad
  strays, a tilted rail's share for a part that took gravity out, five times the noise, a step and
  the jump to a neighboring sample ([The accelerometer](#the-accelerometer)). A clipped top's bound
  is one-sided: the truth is no lower than the reading less it.
- **Burnout.** From the accelerometer, the span of the averaged samples within $2N$ of the least,
  an interval and the averaging window, or two intervals if that is more. From the top speed, a
  floor: $2 \times 5\,\mathrm{m/s}/g$, half the fit and an interval.
- **Each canopy's rate.** The day's air and the barometer's gain as shares of the height the leg
  loses, the noise at either end, the drift over the leg, a step at either end and, for a leg from
  apogee, the median's peak bound, all over the leg's time.
- **The main's opening.** The time the main takes to slow the fall to the run's mark, half the
  1 s fit and two intervals ([The canopies](#the-canopies)).

Each term, and where its size comes from:

| term | size | source |
|---|---|---|
| $s_T$, the day's air | 6.92% | a height read by the standard atmosphere is off by $\Delta T/T$ of itself over the layer it spans (the hypsometric equation); $\Delta T$ is the synthetic flights' 15 K range of air ([ADR-280][adr-280] §2), and $T$ no colder than the standard atmosphere's 216.65 K below 20 km (12 mi). Near the ground it is about 0.35% a kelvin |
| $s_G$, the barometer's gain | 0.25% | an MS5607 barometer chip's ±1.0 mbar over the 400 mbar span its datasheet states it on, the widest of the synthetic flights' parts; reading that error as a gain is an inference ([ADR-280][adr-280] §3) |
| $n$, the noise | 3 m (9.8 ft) | the fast and slow correlated noise, the part's white noise, the pad's median and the port's error at apogee's sideways speed, added and tuned on the tuning set |
| $d$, the drift | 0.015 m/s (0.049 ft/s) | the synthetic flights' fastest drift, 3 Pa in 30 s ([ADR-280][adr-280] §3), at the density of air 5 km (3.1 mi) up, 0.74 kg/m³ |
| $\Delta$, a step | the track's step | the rounding of the logger's numbers; a pressure's step at the height it is read at |
| $b_{\mathrm{med}}$, the median's peak bound | 0.077 m (0.25 ft) at 20 samples a second | gravity's bend over the median's reach ([The running median](#the-running-median)) |
| $e_{\mathrm{pad}}$, the pad's error | up to 2 m (6.6 ft) | how far up the rocket may be when the log starts ([The pad and the climb](#the-pad-and-the-climb)) |
| $a$, the allowance | at least 2 m (6.6 ft), and a step | four times the fast correlated noise, HPR Sim's choice |
| the logger above the ground | 3 m (9.8 ft) | a tall rocket's electronics bay on a rail, HPR Sim's choice: it lands lower than it started by as much |
| the port, while accelerating | a pressure coefficient of 0.02 | the ogive-cylinder's measured −0.02 one caliber behind its shoulder below Mach 0.9 ([the Mach error note][mach] §1–2), the largest the synthetic flights draw |
| $w$, the fit's width | 0.5 s | HPR Sim's choice; a fit cuts a peak by about $(a_1 + a_2)\, w/8$, and twice that is allowed |
| the noise in a speed | 5 m/s (16 ft/s) | tuned on the tuning set |
| $s_A$, the accelerometer's scale | 10% | the ADXL375 accelerometer chip's ±10% sensitivity (its datasheet, p. 3, in the [sensor noise note][noise]), the widest of the synthetic flights' parts ([the acceleration](synthetic-flights.md#the-acceleration)) |
| $\beta$, the accelerometer's bias | 0.05 g | the bias the synthetic flights leave after the pad's zero, the top of the 0.01 to 0.05 g the [sensor noise note][noise] calls a guess, as no source states one |
| a tilted rail | 0.06 g, $g(1 - \cos 20^\circ)$ | the 20° from vertical the NAR's and Tripoli's high-power safety codes allow a rail (the NAR's model rocket code allows 30°, 0.13 g): a part that zeroes on the pad takes out $g \cos\theta$ |
| the accelerometer's noise | 5σ | measured on the log, within 0.5 s of the top, or from the top to a second past the fall through zero for burnout; a normal noise passes it, either way, about once in 1.7 million samples |
| gravity's witnesses | 0.25 g | a 20° rail's 0.06 g, a part's zero offset and the small drag near apogee, HPR Sim's choice |
| the descent's fit | 1 s, widened to hold four steps, at most 10 s | a canopy's rate changes slowly, so a fit twice the climb's averages the noise down; four steps keep a step from reading as a spike, HPR Sim's choice |
| $H$, the density scale height | 7,000 m (22,966 ft) | the 1976 standard atmosphere's 7,289 m (23,914 ft) at its coldest less 15 K, taken down ([The canopies](#the-canopies)) |

The two tuned terms, the 3 m (9.8 ft) and the 5 m/s (16 ft/s), were set on the same flights the bounds are held on
([Checked against](#checked-against)).

### A worked example

The invented PerfectFlite log that the command line's own example reads
([`hpr analyze`](../cli.md#hpr-analyze) prints it) reads an apogee of 390.144 m (1,280 ft), 10.275 s into the
log, on a track in one-foot steps sampled 20 times a second. Its filtered track stays within
2.30 m (7.6 ft) of its start for 0.80 s, longer than the 0.69 s a rocket at rest would need to
climb the 4.61 m (15.1 ft) rise at 2 g, so the pad's error is zero. The bound adds up so:

| term | worked out | meters |
|---|---:|---:|
| the day's air | 0.06924 × 390.144 m | 27.01 |
| the barometer's gain | 0.0025 × 390.144 m | 0.98 |
| the noise | | 3.00 |
| the pad's error | | 0.00 |
| the drift | 0.015 m/s × 10.275 s | 0.15 |
| a step at the pad, and one at the peak | 2 × 0.3048 m | 0.61 |
| the median's peak bound | | 0.08 |
| the bound | | 31.83 |

`hpr analyze` prints it rounded up, as it prints every bound: "within 31.9 m (105 ft)". The flight's true apogee is 390.31 m
(1,280.5 ft), so the reading is 0.17 m (0.56 ft) low: about a two-hundredth of its bound, as it
should be on a log written from the true heights with no noise. About 85% of the bound is the day's
air, which a real log can't rule out.

### Before any reading

A log passes four checks before any reading is taken from it. Each keeps the readings inside the
range the bounds were checked over, or catches a track that isn't what it says:

- **It is sampled no slower than every 2 s and no faster than every 2 ms**, the slowest and
  fastest rates on the tuning set: half a sample a second (`sampled_too_slowly`) and 500 a second
  (`sampled_too_fast`). A clock a part in a thousand faster than 500 a second still reads, as a
  logger's rounding of its own clock may make it. One sample log, a Raven's CSV at 800 samples a
  second, is withheld as too fast.
- **Its step is no coarser than 10 m (33 ft), or 100 Pa for a pressure** (`step_too_coarse`), the
  coarsest checked on the tuning set. The coarsest the readers meet are a MARSA's 8.8 m (29 ft)
  heights and an ARTS2's 92 Pa raw counts.
- **Its clock increases** (`clock_out_of_order`). A reader keeps a channel's clock as the file
  writes it, which can repeat or run back, as a GNSS receiver's fixes or a team's merged table do.
- **Its climb slows as a coast does.** A coast loses $g t^2/2$ in the last $t$ before its top, 78.5 m
  (257 ft) in 4 s, gravity slowing it and drag only adding. A track must lose more than twice its
  noise and step in the 4 s before its top, 6.6 m (22 ft) on a PerfectFlite, or it is withheld
  (`slower_than_free_coast`). The threshold isn't a coast's 78.5 m (257 ft), since a flight whose motor burns
  into those 4 s loses less.

**A disturbed top.** A top that isn't the coast's is withheld as `disturbed_top`, by either of
two checks. Which point is the apogee can't be told, so the apogee, the top speed and landing are
withheld; liftoff is still given.

- **A fall before the top.** A coast doesn't fall and rise again. A track that falls more than
  twice its noise and step below a peak in the 4 s before its top, then climbs to that top, holds
  a top that a disturbance after apogee made, such as a deployment's pressure bump that lasts
  longer than the 0.3 s median takes out. A top such a bump holds after apogee can put the
  apogee's time past its bound.
- **A spike at the top.** A coast slows into its top at gravity and drag, and near the top drag
  is smaller than gravity, so it climbs no more than $g \Delta^2$ in its last $\Delta$, as a body
  slowing at twice gravity would. And nothing falls from its top faster than gravity pulls it. A
  track that, within a second either side of its top, or two samples on a slower track, both
  climbs into the top faster than that and falls from it faster than a body at rest there can,
  each by more than twice its noise and step, holds a spike, such as a charge's. The fall allows
  for the filtered top trailing the true one: the speed a fall from rest gathers within the
  median's window and twice the noise and step, then gravity's fall from there.

Drag's upward share stays under gravity over a last second for any ballistic coefficient (the
mass over the drag coefficient times the frontal area) over 150 kg/m², even at 120 m/s (394 ft/s)
across; a rocket 29 mm (1.1 in) across, of 0.3 kg, has about 800. That covers a 1 s span. On a slower track the span is two
samples, up to 4 s at half a sample a second. There the argument holds only over about 380 kg/m²
on a vertical coast, and about 700 kg/m² at 120 m/s (394 ft/s) across. Below that a coast can climb
into its top faster than the check allows, so a good top that a charge then drops fast may be
withheld. The check only withholds, so it never reads such a top wrong.

On a PerfectFlite logging 20 times a second, twice the noise and step is 6.6 m (22 ft), so a coast
climbs at most 16.42 m (54 ft) in its last second. In the tests, a step 20 m (66 ft) above a coast's
top and 0.6 s wide falls 29.0 m (95 ft) from the filtered top in one sample, where a body at rest
there falls at most 20.9 m (69 ft), so it is withheld.

Either half of a spike alone is read, as a good top makes each. A parachute open before apogee
slows the climb into the top harder than a coast does, but not the fall from it: the tests' chute,
opened at 40 m/s (131 ft/s), climbs 20 m (66 ft) in its last second, then falls at gravity. A
charge that raises the bay's pressure at the top drops the track fast, but only after the coast's
own climb: 50 m (164 ft) for half a second in the tests. Both read their apogee within its bound.

Two public logs of a charge's transient show both outcomes
([#702](https://github.com/nrdptel/fusionspace-eridanus/issues/702), a transient read as the
peak; [Public logs of faulty flights][faulty]). A TeleMetrum log from a forum thread, whose raw
peak was one sample of a pressure spike after the charge, now reads the coast's peak: the median
takes the one sample out. An MIT-licensed Blue Raven log of a breakup is withheld by the spike
check. The tests rebuild both shapes with invented numbers.

Another check guards a height that is above the pad only by a word. A column mapping can call a
column an altitude above the ground, and a column's header can say so, but neither proves it. Such
a track must start within 19.9 m (65 ft) of its zero (`starts_off_the_pad`): a logger zeroes on the
pad, and its zero moves with the weather and the noise before launch. The 19.9 m (65 ft) is an
hour on the pad at the National Weather Service's rate for *pressure rising rapidly* or *falling
rapidly*, 0.06 inHg (203.2 Pa) an hour, and the 3 m (9.8 ft) noise. At sea level's density,
1.225 kg/m³, an hour's 203.2 Pa is 16.9 m (55 ft) of height, and the noise makes up the rest. The
hour is HPR Sim's choice.

That check closes the first half of
[#646](https://github.com/nrdptel/fusionspace-eridanus/issues/646): an altitude above sea level
mapped as one above the ground. Its second half, a clock in milliseconds read as seconds, is caught
by the checks above: the samples come a thousand times too far apart, slower than every 2 s at any
logger rate below 500 a second, and a track on such a clock loses less than a tenth of a millimeter
in the 4 s before its top, where a coast loses 78.5 m (257 ft). A test holds both halves both ways: the wrong
unit withheld by name, the same flight in its right unit read.

### When a reading is withheld

A withheld reading has a code and a sentence with the log's own numbers. Only the readings that
need what is missing go: a log that starts in the air on the logger's own altitude loses its
liftoff, and with it the flight time, but keeps its apogee, top speed and landing. There are 26
reasons:

| code | when | readings withheld |
|---|---|---|
| `too_short` | the track has fewer than 3 samples; or the accelerometer has fewer than 3 before apogee | all; or the top acceleration, and burnout from the accelerometer |
| `no_climb` | the filtered track never climbs twice the allowance and a step above its start, or 3 m (9.8 ft) above the pad, or the allowance and a step if that is higher; or, for landing, falls too little past its step and noise to time the descent | all, or landing |
| `starts_off_the_pad` | a track above the pad only by a mapping's or a column's word starts more than 19.9 m (65 ft) from its zero | all |
| `short_pad` | the first samples can't rule out a rocket already more than 2 m (6.6 ft) up | liftoff, and a top speed at the log's start, on a track with a zero, the logger's own or a word's; all on any other track |
| `ends_before_landing` | the log ends before the track is back within the pad's allowance and stays down for a second | landing, the flight time and each canopy's descent |
| `faster_than_free_fall` | the track comes down to the landing sample sooner after apogee than a fall from rest in vacuum could lose that height, $\sqrt{2h/g}$, allowing a step in $h$ and a sample for the apogee's time | landing, the flight time and each canopy's descent |
| `implausible_speed` | the top speed is above 4,000 m/s (13,123 ft/s), Debrief's ceiling | the top speed, and burnout from its time |
| `noisy_speed` | from liftoff to apogee the speed swings below zero by more than 20% of its top | the top speed, and burnout from its time |
| `transonic` | the top speed reaches 227.7 m/s (747 ft/s) | the top speed, and burnout from its time |
| `sampled_too_slowly` | the samples come further apart than 2 s; or, for the top speed, than 0.125 s; or, for the canopies, too far apart for 5 samples in the descent's fit, every 0.25 s for a 1 s fit | all; or the top speed and burnout from its time; or both canopies' descents |
| `sampled_too_fast` | the samples come more often than every 2 ms, 500 a second, the fastest rate the bounds were checked at | all |
| `step_too_coarse` | the step is coarser than 10 m (33 ft) or 100 Pa; or so coarse for the descent near the ground that a fit holding four steps would span more than 10 s | all; or both canopies' descents |
| `slower_than_free_coast` | the track loses no more than twice its noise and step in the 4 s before its top | all |
| `disturbed_top` | the track falls more than twice its noise and step below a peak in the 4 s before its top, then climbs to that top; or, within a second either side, it climbs into its top faster than a coast and falls from it faster than a body at rest | apogee, the top speed, landing, and the readings that need them |
| `clock_out_of_order` | the height track's clock doesn't increase; or the accelerometer's runs back | all; or the top acceleration, and burnout from the accelerometer |
| `no_height_track` | the log holds no altitude, pressure or GNSS track | all |
| `unstated_sensor` | the log's only height doesn't say whether a barometer or a GNSS receiver measured it | all |
| `receiver_lag` | the only height is a GNSS receiver's, whose filter lags a climb and overshoots its top | apogee, the top speed, and burnout from its time |
| `receiver_dropout` | the only height is a GNSS receiver's, whose fix drops out or lags at liftoff | liftoff, and the flight time |
| `needs` | the reading needs another that was withheld, such as landing on a GNSS track | as the sentence says |
| `bad_record` | the record breaks what every reader guarantees: tracks as long as their clocks, on finite times that increase. No file reader produces one; only a record built by hand in code can | all |
| `no_accelerometer` | the log has no accelerometer | the top acceleration; burnout comes from the top speed's time |
| `axis_unstated` | the log's accelerometers don't say which lies along the rocket's axis, such as a board's three axes or a table's column read by its header | the top acceleration; burnout comes from the top speed's time |
| `gravity_unstated` | the axial accelerometer's readings may or may not hold gravity's 1 g, and neither its family, nor its pad, nor its coast to apogee settles which, or they disagree | the top acceleration; burnout comes from the top speed's time |
| `one_canopy` | the descent's rate never changes enough to tell a second canopy: the rocket came down under one, under two whose rates are closer than 1.4 to one, or under none | the drogue's descent; the main's is read from apogee |
| `short_leg` | a leg of the descent loses no more than 3 m (9.8 ft) or a tenth of its top's height; or the whole descent loses too little, or falls too slowly near the ground, to time a leg by | that leg's descent; or both |

"All" means liftoff, apogee, the top speed and landing, with the flight time, burnout and each
canopy's descent that need them. The top acceleration is searched for up to apogee, so a reason
that withholds the apogee withholds it too, unless the log has no axial accelerometer, which is
said first (`no_accelerometer` or `axis_unstated`). A burnout the accelerometer can't give falls
back to the top speed's time, and carries the top speed's reason where that is withheld too.

An apogee within half a window of the log's end is read, but marked `is_floor`: the log may have
stopped before the rocket did.

## Validation and limits

What the readings were checked against, how well they held, and what they leave out.

### Checked against

**The synthetic flights' tuning set.** [The synthetic flights](synthetic-flights.md) are flights
HPR Sim simulated, each with its truth and the log a simulated logger would write. Of the tuning
set's 200 flights, 171 carry a barometer; the other 29, a GNSS tracker's, give no reading by rule.
`cargo xtask synthetic` reads each of the 171 as its own logger wrote it, then again at every rate
from 500 to half a sample a second and every step from none to 10 m (33 ft) or 1 to 100 Pa, 36 ways
in all, and cut 6 ways as loggers that record from launch write them: 20 samples a second in feet
or in 10 Pa steps, starting 0.3 s before liftoff, 0.1 s after, or 0.5 s after with the zero first.
The 71 flights whose logger has an accelerometer, the AltOS-like and the IMU profiles, are read
again with it at 500, 100 and 20 samples a second. Every reading given stayed within its bound,
and the check fails the build if one doesn't; a floor holds while it is no more than its bound
above the truth. The largest share of its bound any error took, from the
[report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/synthetic-flights.md#readings-within-their-bounds)
and its JSON:

| reading | largest share of the bound | on the log | its error, against its bound |
|---|---:|---|---|
| liftoff | 0.603 | 500 samples a second, heights with no step | 0.387 s within 0.643 s |
| apogee height | 0.717 | 20 samples a second, heights with no step | −149 m (−489 ft) within 208 m (682 ft) |
| apogee time | 0.618 | 5 samples a second, heights with no step | −0.607 s within 0.982 s |
| top speed | 0.544 | 20 samples a second, pressure in 100 Pa steps | 13.7 m/s (44.9 ft/s) within 25.1 m/s (82.3 ft/s) |
| landing | 0.572 | 5 samples a second, pressure in 100 Pa steps | −3.89 s within 6.8 s |
| flight time | 0.51 | 500 samples a second, pressure in 10 Pa steps | −0.729 s within 1.43 s |
| descent rate | 0.609 | 20 samples a second, pressure in 1 Pa steps | −1.2 m/s (−3.9 ft/s) within 1.98 m/s (6.5 ft/s) |
| top acceleration | 0.715 | its accelerometer at 20 samples a second | −13.6 m/s² within 19.1 m/s² |
| burnout | 0.544 | 20 samples a second, pressure in 100 Pa steps, a floor from the top speed | 0.718 s within 1.32 s |
| drogue rate | 0.692 | 20 samples a second, heights with no step | −0.931 m/s (−3.05 ft/s) within 1.34 m/s (4.4 ft/s) |
| main opening | 0.642 | 20 samples a second, pressure in 10 Pa steps | 0.634 s within 0.987 s |
| main rate | 0.7 | 500 samples a second, heights with no step | −1.17 m/s (−3.84 ft/s) within 1.67 m/s (5.48 ft/s) |

The report also gives each reading's errors over every tuning flight and every log together,
beside the mean bound the readings stated
([Errors on the tuning set](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/synthetic-flights.md#errors-on-the-tuning-set)).
An error is the reading less the truth. A floor, a burnout from the top speed or a clipped top
acceleration, is counted apart, as it may lie any amount below the truth. The least, the most and
the mean bound are to three figures, nearest; a mean is given to the decimal place of its largest
error's third figure, the digits every platform shares:

| reading | given | mean error | least | most | mean bound |
|---|---:|---:|---:|---:|---:|
| flight time | 6,948 | −0.74 s | −5.06 s | 3.15 s | 5.83 s |
| top acceleration | 280 | 0.7 m/s² | −13.6 m/s² | 13 m/s² | 16.6 m/s² |
| top acceleration, a floor | 4 | −4.49 m/s² | −7.55 m/s² | −1.58 m/s² | 12.7 m/s² |
| burnout, from the accelerometer | 262 | −0.0118 s | −0.0943 s | 0.0476 s | 0.651 s |
| burnout, a floor | 1,236 | −0.24 s | −1.33 s | 0.718 s | 1.3 s |
| drogue rate | 2,827 | 0.04 m/s (0.13 ft/s) | −1.61 m/s (−5.28 ft/s) | 1.35 m/s (4.43 ft/s) | 1.79 m/s (5.87 ft/s) |
| main opening | 2,827 | 0.73 s | −0.778 s | 5.3 s | 2.04 s |
| main rate | 5,391 | 0.04 m/s (0.13 ft/s) | −5.47 m/s (−17.9 ft/s) | 4.83 m/s (15.8 ft/s) | 1.71 m/s (5.61 ft/s) |

Burnout from the accelerometer lands within 0.0943 s of the truth, against a mean bound of 0.651 s.
A floor from the top speed comes 0.24 s before burnout on average, as a speed's peak should, and up
to 0.718 s after it, within its bound. The main's opening is read late on average, by 0.73 s; its
bound holds the time a canopy takes to slow the fall and half the fit. No tuning flight's true top
acceleration reaches the range its part clips at: the nearest comes to 0.492 of it, the report says.
The 4 tops read as clipped all the same, each a floor, are all on the ADXL375 read at 20 Hz. That is
consistent with its noise there, 19.8 mg by the synthetic flights' noise model, being under half its
49 mg step, so that it writes most of a thrust's plateau as one value, but no test pins a quiet part
that reads as clipped ([ADR-283][adr-283]).

On their own logs, the main's opening was found on each of the 87 tuning flights with a barometer
that came down under two canopies, and on no log was a main named on a flight that opened one
canopy or none: those are counted as none to read. On logs sampled once a second or slower, the
canopies are read only where coarse steps widen the descent's fit enough to hold 5 samples;
elsewhere they are withheld, most as `sampled_too_slowly`, some as `ends_before_landing` (up to 14
of 171 at half a sample a second) and one as `step_too_coarse`.

The same report counts each reading withheld, by reason. At 20 samples a second and faster, the top
speed is withheld as `transonic` on 117 or 118 of the 171 flights, and at 5 samples a second and
slower as
`sampled_too_slowly` on all of them but the one withheld whole below. At half a sample a second, up to 14 flights end before their
landing is seen, and in 10 m (33 ft) steps one flight's track is withheld whole as
`step_too_coarse`. The logs written again are read as tables a column mapping states, so their
heights are above the pad only by the mapping's word. Of the cuts, heights from 0.1 s after liftoff
withhold liftoff as `short_pad` on 36 flights, and heights from 0.5 s after, the zero first, on all
171. Both give the apogee on all 171, within its bound with the zero's possible error in it, at up
to 0.695 and 0.674 of it. A pressure has no zero set on the pad, so a pressure cut after liftoff withholds every
reading on all 171, and one from 0.3 s before liftoff on 3. No tuning flight is withheld as
`disturbed_top` or `slower_than_free_coast`.

The tuning set shows the bounds hold on the flights they were tuned on. It doesn't show how often
they would hold on others, nor on real logs: the flights are HPR Sim's own, with a noise model
taken from datasheets and a few public logs ([Synthetic flights](synthetic-flights.md#in-short)).
The test sets, kept apart from any tuning, are for the 90% intervals of
[M7.2e](../decisions-and-roadmap.md#m7-2e).

**An invented flight.** The file
[`synthetic-pnut.pf2`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/logs/synthetic-pnut.pf2)
holds a flight made up for the tests. It sits on the pad for half a second, then accelerates at
50 m/s² for 1.6 s and coasts with no drag. It falls from rest to 25 m/s (82 ft/s), descends at 25 m/s (82 ft/s) to
150 m (492 ft) and at 6 m/s (20 ft/s) to the ground. It is written as a Pnut writes one, rounded to whole feet and
feet per second, with an ejection pulse of the public log's shape a second after apogee. Every
reading is known in closed form. The
[tests](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-flightdata/src/synthetic.rs)
hold each within its stated bound, and within a tighter one worked out beside it for this log
([evidence row G10](../evidence.md#g10)):

| reading | true | HPR Sim reads | its bound | the test's tighter bound |
|---|---|---|---:|---|
| liftoff | 0.50 s | 0.80 s | 0.81 s | from 0.50 s to 0.814 s, once the rocket has climbed 2 m (6.6 ft), a foot and the half foot that rounds it up |
| apogee | 390.31 m (1,280.5 ft) | 390.14 m (1,280 ft) | 31.9 m (105 ft) | 0.23 m (0.75 ft): half a foot of rounding and the median's 0.077 m (0.25 ft) |
| apogee time | 10.258 s | 10.275 s | 0.88 s | one sample, 0.05 s: the middle of the flat run at the top |
| top speed | 80.0 m/s (262 ft/s) at 2.10 s | 78.3 m/s (257 ft/s) at 2.20 s | 20.6 m/s (68 ft/s) | no higher than the truth, and no lower by more than the fit's cut across burnout, 3.7 m/s (12 ft/s), and half a foot a second |
| landing | 46.14 s, touchdown | 45.80 s | 2.11 s | up to 0.38 s early: 2 m (6.6 ft) at 6 m/s (20 ft/s) is 0.33 s, and a sample more |
| flight time | 45.64 s | 45.00 s | 2.91 s | follows from liftoff and landing |
| mean descent rate | 10.93 m/s (35.9 ft/s) over HPR Sim's span, 10.275 s to 45.80 s | 10.92 m/s (35.8 ft/s) | 2.1 m/s (6.9 ft/s) | the two heights' bounds over the span |
| burnout | 2.10 s | no sooner than 0.88 s: the top speed's time, 2.20 s, less its bound | 1.32 s | none tighter: a floor, no more than its bound after the truth |
| drogue rate | 21.74 m/s (71.3 ft/s) over HPR Sim's span, 10.275 s to 21.40 s, the fall from rest at apogee in it | 21.7 m/s (71 ft/s) | 2.2 m/s (7.2 ft/s) | the two heights' bounds over the span |
| main opening | 21.14 s, 150 m (492 ft) up | 21.40 s, 148.4 m (487 ft) up | 1.06 s | on this log, whose drogue falls far faster than the mark: no sooner than the truth, and no later by more than half the 1 s fit and a sample, 0.55 s |
| main rate | 6.0 m/s (20 ft/s) | 6.0 m/s (20 ft/s) | 0.8 m/s (2.6 ft/s) | the two heights' bounds over the span |

The middle of the flat run at the top falls between two samples here, so the apogee time is
10.275 s. [`hpr analyze`](../cli.md#hpr-analyze) prints times to two decimals, as 10.28 s and
9.47 s after liftoff. The first sample of the flat run at the top would read the apogee over 0.2 s
early; a test shows that too, so the rule of taking the run's middle is pinned. The file's highest
sample is the pulse's, 400.5 m (1,314 ft). The Hampel filter keeps it, and the median sets it
aside; a test holds both.

**A real flight.** Debrief ships a public PerfectFlite Pnut log, trimmed from a publicly shared
flight. Its terms upstream are unclear, so it isn't committed here, and CI doesn't have it. Where
it has been fetched (`cargo xtask refs fetch`), a
[test](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-cli/tests/cli.rs) reads it and
holds the apogee above the altimeter's zero, the highest sample, the pad, and each reading with
its bound:

| | the altimeter states | HPR Sim reads |
|---|---|---|
| apogee | 1,009 ft above the pad | 1,009 ft (307.5 m) above the pad, within 27.4 m (90 ft); 1,010 ft above the altimeter's zero, its pad a foot above it |
| highest sample | | 1,028 ft above the zero, the ejection pulse, set aside |
| top speed | | 264 ft/s (80.4 m/s), within 29.7 m/s (98 ft/s), derived from the heights |
| liftoff | | 0.40 s, within 0.98 s |
| landing | | 72.70 s, within 3.25 s |

The altimeter's apogee comes from the same pressure trace, so the agreement shows that HPR Sim
reads the trace as the altimeter does, not that either is the true height. A Featherweight Raven
flew on the same flight, and Debrief reports it agreeing at about 1,009 ft.

**The sample logs.** `cargo xtask logs` takes the readings from every record the project's sample
logs hold, other flyers' files kept outside the repository, and counts each reading given or
withheld, by logger family and by reason
([the survey's report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/log-import.md#readings)).
Of the 1,058 records, summed over the report's families, an apogee is given from 593, liftoff from
554, the top speed from 354, landing from 472, the flight time from 406, burnout from 435, the top
acceleration from 184, the drogue's descent from 267 and the main's from 459. Each of the 26
reasons has a case: for 22, the first file the survey withholds a reading of for it, by an
anonymous id; for the four that none of
the sample logs triggers (`bad_record`, `faster_than_free_fall`, `starts_off_the_pad` and
`step_too_coarse`), an invented file under
[`validation/fixtures/logs/readings/`](https://github.com/nrdptel/fusionspace-eridanus/tree/main/validation/fixtures/logs/readings)
that a test reads to its reason. The invented fall, for one, drops at 15 m/s (49 ft/s) for 3 s
after apogee, then at 300 m/s (984 ft/s) to the ground. These counts show what each logger can give. They don't show how
right a reading is: none of the sample logs has a truth beside it.

Three reasons stand out. The top speed is withheld as `transonic` on 205 records: many of these
flights are fast. Liftoff is withheld as `short_pad` on 157, 71 of them from loggers whose files
start close to liftoff: 37 of 51 RRC3 CSV records and 34 of 44 Eggtimer CSV records. And the
apogee, top speed and landing are withheld as `disturbed_top` on 76.

The new readings' reasons follow the loggers. The top acceleration is withheld as
`no_accelerometer` on 665 records, as `axis_unstated` on 136, whose boards or tables don't say
which accelerometer lies along the rocket, and as `gravity_unstated` on 10. The drogue's descent
is withheld as `short_leg` on 11, and as `one_canopy` on 185, where the descent's rate didn't show
a second canopy; with no truth beside the logs, that doesn't say how each rocket came down.

### What it leaves out

- **The day's air.** A logger's height is its own conversion of the pressure, which assumes a
  standard atmosphere. On a warm day a barometric altimeter reads low, 6.5% on a day 20 K warmer
  than standard ([Atmosphere](atmosphere.md#pressure-altitude-what-a-barometric-altimeter-reads)).
  HPR Sim doesn't correct it: it widens the bound by 6.92% of the height instead, which covers air
  up to 15 K from standard. On a day further off, a height can pass its bound.
- **Loggers unlike the tuning set's.** The bounds hold on the tuning set's
  [six simulated logger profiles](synthetic-flights.md#the-loggers), at their noise. A logger
  noisier than they are, or a rocket slower off the pad than 2 g, can be read outside its bound.
  A pad that drifts within the 2 m (6.6 ft) allowance, early one way and late the other, can read
  liftoff outside its bound: a review measured up to 1.22 times the bound at ±2 m (6.6 ft) on a
  synthetic climb. On a short pad, a zero only a word vouches for is placed by the first sample
  unfiltered, so its error can be understated by that sample's noise, up to 3 m (9.8 ft)
  ([#773](https://github.com/nrdptel/fusionspace-eridanus/issues/773), the pad error's noise
  budget).
- **Rockets slow off the pad.** The liftoff bound and the pad's error assume at least 2 g off the
  pad. A rocket slower than that is read later than its liftoff bound, and may already be further
  up, when a log starts, than its pad's error allows.
- **Fast flights' top speeds.** The top speed is withheld from 227.7 m/s (747 ft/s), and so is a
  burnout taken from its time where no accelerometer gives one. The heights
  aren't checked for the port's error near the speed of sound; the apogee, read near the top where
  the rocket is slow, stays within its bound on the tuning set, whose fastest flight reaches Mach
  1.12.
- **Wide pulses.** A pulse wider than half the median's window passes it. The bounds were held on
  the synthetic flights' pulses, which are shaped like the public Pnut log's and scaled by 0 to 2;
  a wider or taller one isn't covered.
- **The deployments, the Mach number and dynamic pressure.** The main's opening is read from the
  descent's rate, not from a deployment the log records, and the drogue's leg is read from apogee.
  The Mach number and dynamic pressure aren't read: they need the speed and the air all along the
  climb, which the readings don't reconstruct.
- **A canopy's filling time.** The opening's bound holds the time an open canopy takes to slow the
  fall, not the time it takes to [fill](../glossary.md#inflation-and-filling-time): the synthetic
  flights open each canopy at once, so a main that fills slowly can be read later than its bound
  ([#779](https://github.com/nrdptel/fusionspace-eridanus/issues/779)).
- **A fall under no canopy.** A rocket that came down under no canopy has its fall read as one
  canopy's `main_descent`: the rate alone doesn't tell a canopy from none. The tuning set's flights
  of Cavour, a design with no parachute, read that way
  ([#780](https://github.com/nrdptel/fusionspace-eridanus/issues/780)).
- **A descent from far above the tropopause.** The scaling to the ground's density slows a rate read
  high up, never quickens it, while the density from the pad up falls no faster than $\exp(-h/H)$.
  Through the troposphere it falls slower, as its scale height is at least 7,289 m (23,914 ft);
  above it, about 11 km (36,000 ft) up, the scale height is about 6,340 m (20,800 ft), 5,903 m
  (19,367 ft) on a day 15 K cold. From a pad at sea level, the 1976 standard atmosphere's density
  stays above $\exp(-h/H)$ to 34.6 km (113,500 ft) up, but on a day 15 K cold only to 21.3 km
  (69,900 ft); from a pad 1,500 m (4,921 ft) up, to 28.3 km (92,800 ft) and 17.7 km (58,100 ft)
  above it, each rounded down. A canopy from higher could read faster than it falls near the ground,
  and one canopy as two. The heights are the standard's geopotential heights, a little less than a
  height measured straight up, which puts the limits low, not high. The tuning set's apogees reach
  4,420 m (14,500 ft), so it can't show this.
- **More than one burn.** An air start's or a sustainer's second burn is searched for the top, but
  the tuning set's ten designs each fly one motor in one stage, so its bound isn't checked on one,
  and burnout then comes from the speed's peak, a floor. A second burn that holds past 2 g for less
  than 0.5 s is taken for a shock, so its top is missed and burnout is the first burn's end; a shock
  that holds there longer reads as a burn. Where liftoff is withheld, a knock on the pad wider than
  the cleaning takes out joins the burn, and can be taken for the top
  ([#789](https://github.com/nrdptel/fusionspace-eridanus/issues/789)). A part that reads above zero
  all through the coast, as a part that zeroed off its pad's level can, never ends its burn, so its
  top is searched to apogee and its burnout comes from the speed's peak.
- **Noise that runs from sample to sample.** Averaging $n$ samples cuts the noise by $\sqrt n$ only
  where each sample's noise is its own. A part whose noise runs on, as its own low-pass
  filter or a slow drift makes it, keeps more of it through the average, so $N$ comes out too
  small and burnout's bound too narrow. The synthetic flights draw each sample's noise apart, so
  they can't show this ([#787](https://github.com/nrdptel/fusionspace-eridanus/issues/787)).
- **Clipping at a part's range.** No tuning flight's true top reaches its part's stated range,
  ±32 g or ±180 g ([the loggers](synthetic-flights.md#the-loggers)): the nearest comes to 0.492 of
  it, the synthetic report says. So a top clipped at its range is held only by the unit tests. A
  clipped top is a floor either way.
- **The coast's witness.** It assumes the drag in the last second before apogee's earliest time is
  under 0.25 g. A rocket still fast at apogee, arcing over, reads its drag there: the witness then
  says nothing or, past 0.75 g, can disagree with a family's word and withhold the top
  acceleration ([ADR-283][adr-283]).
- **A board's three axes.** A log whose accelerometers don't say which lies along the rocket gives
  no top acceleration (`axis_unstated`), and its burnout comes from the top speed's time. The
  axes' magnitude could give a top acceleration, but not burnout, as it never falls below zero
  ([#781](https://github.com/nrdptel/fusionspace-eridanus/issues/781)).
- **Intervals.** A bound is a worst case held on the tuning set, not a 90% interval with a tested
  chance of holding the truth. Those come with [M7.2e](../decisions-and-roadmap.md#m7-2e), checked
  on the synthetic flights' test sets.
- **A GNSS track.** A log whose only height is a GNSS receiver's gives no reading: the
  receiver's lag and overshoot aren't modeled, so no bound can be stated for it.
- **Disturbed tops, both ways.** A bump less than about a second after apogee, before the track
  has fallen past its noise and step, can't be told from the top unless it is a spike, and its
  time may pass the apogee's time bound. A transient that only climbs in fast, or only falls out
  fast, is read as the top, since an early parachute and a charge make those shapes on a good
  top: a spike that climbs in no faster than a coast, at most 16.42 m (54 ft) in its last second
  at 20 samples a second, is read as the apogee, however high it stands. The other way, a brief dip in the coast in the last 4 s before the top
  withholds a good apogee; how many of the sample logs' 76 such tops are good isn't measured
  ([#762](https://github.com/nrdptel/fusionspace-eridanus/issues/762)). The synthetic flights
  model no charge's transient, so the bounds aren't tested against one
  ([#761](https://github.com/nrdptel/fusionspace-eridanus/issues/761)).

## Tests and references

The tests that pin the readings, and the sources they follow.

### Tests

- `hpr_flightdata::readings`: a plain flight read within its bounds; the same flight read alike
  from a pressure channel, an altitude channel above sea level and the logger's own track, and a
  GNSS altitude withheld by name; each withheld code on a log built to trigger it, only the
  readings that need it going; the pad's rules (a jitter in the first samples, noise up to the
  allowance, a slow noisy pad, a pad short by its samples and one read from the logger's zero);
  a vacuum fall rounded to feet landing at 10 to 100 samples a second and one at 1.3 g refused;
  [#646](https://github.com/nrdptel/fusionspace-eridanus/issues/646), a wrong unit or a wrong zero
  in a mapping, both ways; a mapped track's short pad read from its zero within the allowance, its
  apogee's bound grown by the distance, and withheld past it; a top held by a
  fall or a spike withheld, each by its own rule, and a top with an early parachute or a later
  charge read; the shapes of the two public logs of
  [#702](https://github.com/nrdptel/fusionspace-eridanus/issues/702), with invented numbers; the
  step and the clock at their edges; the top speed's guards, each on its own;
  and a property test of the median's peak bound for every $K$ from 1 to 6 and every place the
  peak can fall between samples.
- `hpr_flightdata::readings::accelerometer`: a noisy axial trace's top and burnout read within
  their bounds, burnout within a sample of the thrust's end; a part mounted nose down, with
  gravity taken out, read alike; a top held at the part's range marked clipped, the truth above
  the reading less its bound, and a quiet plateau not; gravity left unsettled withholding the
  top acceleration and sending burnout to the top speed's floor; the coast settling gravity where the log holds no pad; a repeated time keeping its
  first sample; a trace that never falls below zero giving burnout as a floor from the top speed;
  and the noise measured as the scatter about the curve, a straight line's as 0.
- `hpr_flightdata::readings::canopies`: a drogue to 150 m (492 ft) at 25 m/s (82 ft/s), then a
  main at 6 m/s (20 ft/s), each leg's rate and the opening read within their bounds, the opening's
  bound holding the main's slowing time to its 9.6 m/s (31.5 ft/s) mark beside half the fit; one canopy withholding the drogue and reading the
  main from apogee; two canopies whose rates are close read as one; a short drogue leg withheld;
  and a track too coarse for the main's fit withholding both.
- `hpr_flightdata::synthetic`: the invented flight, its readings against the truth (burnout, the
  main's opening and both legs among them), and the Hampel filter keeping the pulse the median
  removes.
- `hpr_flightdata::filter`: a pulse removed, a ramp untouched, the Hampel filter at zero equal to
  the median, and property tests that every output lies within its window and that the sliding
  median equals each window's own, bit for bit.
- `cargo xtask synthetic --check`: every reading on the tuning set within its bound, at every rate,
  step and cut and with each accelerometer at 500, 100 and 20 samples a second, a floor no more
  than its bound above the truth, its report reproduced on every platform.
- `cargo xtask logs` and its tests: each invented sample read to its reason, and, where the sample
  logs are fetched, every record's readings counted and the report reproduced.
- `crates/hpr-cli/tests/cli.rs`: `hpr analyze` on a log alone in a folder, its JSON checked against
  the published schema; the public Pnut log, where fetched. `crates/hpr-py/tests/test_logs.py`:
  Python's `FlightLog.readings()` equal to what `hpr analyze` prints, on each invented log.

### References

- R. K. Pearson, Y. Neuvo, J. Astola and M. Gabbouj, *The Class of Generalized Hampel Filters*,
  23rd European Signal Processing Conference (EUSIPCO), 2015, §2, eqs. 1 and 2. Pinned as
  `pearson-2015-generalized-hampel` in the
  [reference lock file](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/refs.lock.toml).
- Debrief, `lib/analyze/index.ts` and `lib/parsers/perfectflite.ts` (MIT, the project owner's
  own), for the landing's thresholds, the least climb, the speed's refusals and the format, and
  for the flat top of a clipped accelerometer, the fall through zero it takes as burnout, the
  main's opening found back from landing, the drogue's ratio and a leg's least drop;
  `lib/flight/build.ts` for the first sample kept at a repeated time; notes in
  [the readings note](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/debrief-flight-readings.md).
- The accelerometer's scale and bias: the ADXL375 datasheet's ±10% sensitivity (p. 3), and the
  bias the synthetic flights assume, which no source states; both in [the sensor noise note][noise]
  and [Synthetic flights](synthetic-flights.md#the-acceleration).
- A canopy's slowing after the opening: vertical motion under quadratic drag,
  $\dot v = -g(v^2/v_m^2 - 1)$, J. R. Taylor, *Classical Mechanics*, University Science Books,
  2005, §2.4; $\tau$ is its integral from any speed down to $v_1$, HPR Sim's step.
- A canopy's rate: drag balancing the weight, $v = \sqrt{2mg/(\rho C_D S)}$, so the rate goes as
  the density to the −½ ([Recovery](recovery.md#the-descent)).
- The density scale height: *U.S. Standard Atmosphere, 1976*, NOAA-S/T 76-1562, its
  troposphere's lapse of 6.5 K/km and its tropopause's 216.65 K
  ([Atmosphere](atmosphere.md)); $H = RT/(g - RL)$ follows from its pressure and temperature.
- The noise model and the range of air: [ADR-280, the synthetic flights first][adr-280] §2–3, and
  [Synthetic flights](synthetic-flights.md#the-noise-term-by-term).
- The port's error: [the Mach error note][mach] §1–2, from a wind-tunnel test of the static
  pressure along an ogive-cylinder through Mach 1.
- How loggers turn pressure into height: [the height formulas note][formulas].
- The least acceleration off the pad: the NAR's high-power safety code (a rocket's liftoff weight
  no more than a third of its motor's certified average thrust) and Tripoli's (below 3:1 only with
  an active stability system). The most a rail stands from vertical, 20°: the same codes.
- The rate of the day's pressure change: the National Weather Service's glossary, *pressure rising
  (falling) rapidly*, 0.06 inHg an hour or more, the threshold of METAR's `PRESRR` and `PRESFR`
  remarks.

[adr-108]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0108-a-flight-log-read-alone-perfectflites-pf2-first.md
[adr-280]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0280-m7-2-in-seven-the-synthetic-flights-first.md
[adr-281]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0281-readings-on-every-logger.md
[adr-283]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0283-the-rest-of-the-readings.md
[noise]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/altimeter-sensor-noise.md
[mach]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/altimeter-mach-error.md
[formulas]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/altimeter-height-formulas.md
[faulty]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/anomalous-public-flights.md
