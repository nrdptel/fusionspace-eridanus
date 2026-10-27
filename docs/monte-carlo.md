# Monte Carlo dispersion

Fly a rocket many times with its uncertain inputs scattered, see how far its apogee and landing
spread, and choose each input's scatter.

No two flights of one rocket are the same. The motor burns a little hotter or cooler than its
label, and the rocket weighs a few grams more than the plan. The wind isn't the forecast's.

A [Monte Carlo](glossary.md#monte-carlo) run flies the rocket hundreds or thousands of times. Each
time it draws the uncertain inputs afresh around their planned (nominal) values. The run shows how
far the apogee and the landing spread, and draws the ellipse the landings fall in.

This page runs one, says what each [dispersion](glossary.md#dispersion) does to a flight, and how
to choose the numbers. Its examples need some Rust and follow on from [The builder](the-builder.md).
[`hpr mc`](cli.md#hpr-mc) runs the same from the command line, with no code
([below](#from-the-command-line)), and `hpr.MonteCarlo` from Python
([From Python](#from-python)).

> **How far to trust it.** The spread a run gives is only as good as the uncertainties you give it
> and HPR Sim's flight models. Those models aren't yet validated against real flights
> ([Accuracy](accuracy.md)).
>
> - [`montecarlo.rs`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-analysis/src/montecarlo.rs)'s
>   tests pin the sampling (below).
> - The [landing ellipses](#landing-ellipses) are tested against normal spreads with known answers.
> - Nothing checks whether the spread matches that of real flights. No measured set of repeated
>   flights has been compared yet.
> - A run leaves out correlations between inputs, distributions other than the
>   [normal](glossary.md#normal-distribution), and the inputs listed under
>   [What is not dispersed](#what-is-not-dispersed). It flies what `Flight::builder` sets up, a
>   staged flight's separations included. Events of your own can't be part of one yet.

Sampling tests check that:

- the same [seed](glossary.md#seed) gives the same run, bit for bit, however many flights it
  has and however many threads fly them;
- a run with no dispersion flies the nominal flight in every sample;
- each dispersion moves its input as the table below says;
- a dispersed motor keeps its [specific impulse](glossary.md#specific-impulse);
- failed flights are counted.

## Running a dispersion

Rust, the command line and Python fly the same flights for the same seed.

### A run of 200 flights

The example program
[`crates/hpr/examples/monte_carlo.rs`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr/examples/monte_carlo.rs)
takes the 54 mm (2.1 in) rocket of [The builder](the-builder.md) on a Cesaroni H54
([motor designation](glossary.md#motor-designation) 168H54-10A). It flies at Spaceport America in
a forecast wind of 4 m/s (8.9 mph) from the west, off a rail leaned 5° into the wind. The program
disperses the rocket's mass, its center of mass, its drag, the motor's impulse and burn time, the
wind and the rail. It flies 200 flights. Run it from a copy of the repository with:

```text
cargo run --example monte_carlo -p fusionspace-hpr
```

The heart of it:

```rust,ignore
let launch = Flight::builder(&rocket, &environment, 1.8)
    .inclination_deg(85.0)
    .heading_deg(270.0);
let dispersion = Dispersion {
    dry_mass_sd_fraction: 0.02,              // 2% of the mass without the motor
    cg_sd_m: 0.005,                          // 5 mm
    drag_sd_fraction: 0.05,                  // 5% of the drag coefficient
    impulse_sd_fraction: 0.03,               // 3% of the motor's total impulse
    burn_time_sd_fraction: 0.02,             // 2% of its burn time
    wind_speed_sd_fraction: 0.25,            // 25% of the wind's speed
    wind_heading_sd_rad: 15_f64.to_radians(),
    rail_elevation_sd_rad: 1_f64.to_radians(),
    rail_azimuth_sd_rad: 2_f64.to_radians(),
    ..Dispersion::default()
};
let monte_carlo = MonteCarlo::new(launch.inputs()?, dispersion)?;
let run = monte_carlo.run(2026, 200);          // seed 2026, 200 flights
let apogee = run.apogee()?;                    // the apogees' spread
let landing = run.landing()?;                  // where they landed
let ellipse = landing
    .prediction_ellipse(0.95)?                 // where the next flight lands, 95 times in 100
    .ok_or("too few landings")?;
```

It prints:

<!-- quote: crates/hpr/examples/monte_carlo.output.txt -->
```text
My 54 mm rocket on a 168H54-10A, from a 1.8 m rail at 85°, heading west into a 4 m/s west wind
200 flights, seed 2026: 0 failed
Not yet validated: see the Accuracy page before trusting these numbers.

                       nominal     mean  std dev       5%   median      95%
apogee (m)              1113.3   1119.8     47.4   1040.5   1121.3   1198.8
landing distance (m)     662.4    660.3    208.9    329.3    670.5    996.5
landing east (m)         662.4    626.1    209.6    274.9    635.5    957.7

Landing ellipses, centered 626 m east and 23 m south of the pad:
                       semi-major  semi-minor  heading  landings inside
50%                         253 m       239 m     131°            48.0%
95%                         525 m       497 m     131°            94.5%
95%, the next flight        532 m       503 m     131°            95.5%

Reached 1,100 m: 65.5% of the flights
Apogee with 5% less drag: +29 m; with 5% more: -27 m
```

How to read it:

- **Nominal** is the one flight with every input at its planned value.
- **Landing distance** is how far from the pad the rocket lands, and **landing east** is the
  eastward, downwind, part of it. The nominal flight lands due east, so the two are equal. In the
  run the wind's heading has a standard deviation of 15°, which pushes some flights north or
  south. So the mean landing east is smaller than the mean distance.
- **Std dev** is the [standard deviation](glossary.md#standard-deviation). If the spread is normal,
  about two values in three lie within one standard deviation of the mean. **5%** and **95%** are
  [percentiles](glossary.md#percentile): one flight in twenty went lower than the 5% value, one in
  twenty higher than the 95% value. So nine flights in ten reached between about 1,040 m (3,412 ft)
  and 1,200 m (3,937 ft).

**The mean isn't the nominal flight.** The mean apogee is 6.5 m (21 ft) above the nominal one.
With 200 flights the mean itself is uncertain by about $47.4/\sqrt{200} = 3.4\,\mathrm{m}$ (its
[standard error](glossary.md#standard-error)). So 6.5 m (21 ft) is 1.9 standard errors, and chance
can account for most of it.

Part of the gap is that the apogee doesn't respond evenly to drag. With 5% less drag it gains 29 m
(95 ft), and with 5% more it loses 27 m (89 ft). So an even spread of drag lifts the mean apogee by
about 1 m (3.3 ft).

**The landing spreads far more than the apogee.** The landing distance's standard deviation is
209 m (686 ft), about a third of its mean of 660 m (2,165 ft). The apogee's is 47 m (154 ft), 4% of
1,120 m (3,675 ft). Under the parachute the drift is the wind's speed times the time in the air,
and the wind's speed is the most uncertain input here.

### From the command line

[`hpr mc`](cli.md#hpr-mc) flies any design [`hpr sim`](cli.md#hpr-sim) flies, a staged `.ork`
included. Each dispersion is an option in the units the command line uses elsewhere: degrees for
angles, a fraction for a percentage (`--impulse-sd 0.03` is 3%). It prints the apogee's and the
landing distance's spreads, the landing ellipses below and the failed flights. `--export runs.csv`
writes every flight's draw and outcome, one row a flight. The command's runs are the library's for
the same seed, bit for bit: a test compares them (`mc_flies_a_public_ork_as_the_library_does`).

```text
hpr mc my-rocket.ork --runs 200 --seed 2026 --wind 4 --wind-from 270 \
  --mass-sd 0.02 --drag-sd 0.05 --impulse-sd 0.03 --wind-sd 0.25 --wind-from-sd 15
```

### From Python

`hpr.MonteCarlo` in the [Python package](python.md#monte-carlo) flies the same run and returns it as
NumPy arrays. Each array is one column of the table `hpr mc --export` writes. Failed flights are
counted, and their figures are NaN. `hpr.MonteCarlo` takes the same eleven dispersions as `hpr mc`'s
options, named for their units (`impulse_sd_fraction`, `wind_from_sd_deg`).

A test runs both on 40 flights with every input scattered and finds every number equal, bit for
bit. That holds on one platform, with both built in release mode
([`test_a_run_is_hpr_mcs_bit_for_bit`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-py/tests/test_montecarlo.py)).
Landing ellipses and the summary tables aren't in Python yet; NumPy finds the spreads from the
arrays. A configuration that drops a stage can't be read from Python yet, so it runs only from
Rust and `hpr mc`.

## Landing ellipses

A [landing ellipse](glossary.md#landing-ellipse) is the outline a range safety officer or a
competition asks for. It's an area the rocket lands inside, say, 95 times in 100. Comparing it with
the field's boundary says whether the field is big enough for this rocket in this wind. For that
check, use the *next-flight* ellipse below ([`Scatter::prediction_ellipse`]).

The ellipse assumes the landings follow a [normal distribution](glossary.md#normal-distribution).
It's only as good as the run's landings, which carry the flight models' errors and those of the
dispersions you chose. Neither has been compared with real flights yet.

The simulator draws the ellipse from the run's landing points ([`Run::landing`], then
[`Scatter::ellipse`]) in three steps. Its *semi-major* and *semi-minor* axes are its half-lengths
along its long and short directions.

1. The center is the landings' mean: here 626 m (2,054 ft) east and 23 m (75 ft) south of the
   pad.
2. The axes come from the landings' [covariance](glossary.md#covariance). It gives the direction
   they spread most, the *major axis*, and the direction across it, the *minor axis*, with a
   standard deviation along each. Here those are about 214.6 m (704 ft) and 203.0 m (666 ft). The
   spread is nearly round: the wind's uncertain heading (15°) spreads the landings sideways about as
   much as its uncertain speed (25%) spreads them downwind.
3. The size is how many standard deviations, $k$, the ellipse reaches along each axis. If the
   landings follow a two-dimensional [normal distribution](glossary.md#normal-distribution), that
   ellipse holds the share

   $$
   p = 1 - \exp(-k^2/2)
   $$

   of them. So the ellipse of level $p$ has

   $$
   k = \sqrt{-2 \ln(1 - p)}
   $$


   | Level $p$ | Scale factor $k$ |
   |--:|--:|
   | 50% | 1.177 |
   | 90% | 2.146 |
   | 95% | 2.448 |
   | 99% | 3.035 |

   The 95% ellipse's semi-axes are $2.448 \times 214.6\,\mathrm{m} = 525\,\mathrm{m}$ and
   $2.448 \times 203.0\,\mathrm{m} = 497\,\mathrm{m}$.

In two dimensions it takes more standard deviations to hold 95% than in one: 2.448 against 1.960.
That's because a landing can stray in two directions at once.

**Heading** is the major axis's direction, clockwise from north, between 0° and 180°: here 131°,
running from north-west to south-east. With axes this close to equal the heading means little: a few
landings more or less could turn it a long way. A circle has no heading; the simulator then reports
90°, east.

**The next flight.** The mean and covariance of 200 flights are only estimates. So an ellipse
drawn from them holds a little less than its level of the flights still to come. For normal
landings [`Scatter::prediction_ellipse`] allows for that exactly.

The next-flight ellipse's $k$ comes from Hotelling's $T^2$ distribution, which accounts for the
mean and the spread both being estimated from the same flights. For $n$ landings:

$$
k^2 = \frac{n^2 - 1}{n}\left[(1 - p)^{-2/(n - 2)} - 1\right]
$$

With 200 landings the next-flight ellipse's axes are 1.3% longer: 532 m (1,745 ft) against 525 m
(1,722 ft). With 10 they would be 36% longer.

**Landings inside** counts the run's landings each ellipse really holds
([`Scatter::share_inside`]). Here they are 48.0%, 94.5% and 95.5%. Those are within the
[standard error](glossary.md#standard-error) of a share $p$ from a run of 200, $\sqrt{p(1 - p)/200}$:
about 3.5 percentage points at 50%, 1.5 at 95%. So the landings are consistent with a normal
spread, though that doesn't prove it.

With few landings the shares run high, because the ellipse is fitted to the same points. With three
landings, even the 50% ellipse holds all three. If the share is far from the level in a run of
hundreds of flights, the landings aren't normal. An uncertain wind heading in a strong wind spreads
them along an arc, and an ellipse is then the wrong shape. Look at the points ([`Scatter::points`]).

A failed flight has no landing. It counts as outside for the lower bound and inside for the upper
([Failed flights are counted](#failed-flights-are-counted)).

> **How far to trust it.** The ellipse math is tested against normal spreads whose answers are
> known exactly
> ([`ellipse.rs`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-analysis/src/ellipse.rs)'s
> tests):
>
> - The scale matches the NIST/SEMATECH handbook's chi-square table and its closed form.
> - The axes and heading of turned, stretched covariances come back to 1e-14.
> - Integrating a normal density over its ellipse gives the level to 1e-12.
> - 100,000 points drawn from a known normal spread give back its covariance, and land inside each
>   ellipse at its level, within five standard errors.
> - A new point lands inside the next-flight ellipse of 3, 5 or 20 others at its level, within
>   five standard errors. It lands inside the plain ellipse visibly less often.

The tests can catch a wrong ellipse. Take a spread 200 m (656 ft) long and 30 m (98 ft) wide,
one standard deviation each way. Its 95% ellipse, turned 6° off the spread's axes, holds 90.6%,
and the test pins that.

## What each dispersion does

Each dispersion is a standard deviation: zero, the default, leaves its input at the nominal value.
For each flight the simulator draws a standard normal number $z$ (mean 0, standard deviation 1) for
each input. It moves the input by $\sigma z$, with $\sigma$ the standard deviation you gave. A
rocket of more than one stage gets a draw for each stage, whether its stages fly together or
separate. Each motor and each parachute gets its own draw.

| Field | What each flight flies |
|---|---|
| `dry_mass_sd_fraction` | Each stage's mass without motors times $1 + \sigma z$. Its moments of inertia scale with it. |
| `cg_sd_m` | Each stage's [center of mass](glossary.md#center-of-gravity-cg) moved $\sigma z$ meters towards the tail (towards the nose when negative). |
| `drag_sd_fraction` | The rocket's zero-lift [drag coefficient](glossary.md#drag-coefficient) times $1 + \sigma z$, whether HPR Sim's own, a drag table's or a [drag model's](custom-models.md). |
| `impulse_sd_fraction` | Each motor's thrust and propellant mass, both times $1 + \sigma z$. Its [total impulse](glossary.md#total-impulse) changes and its specific impulse doesn't, as for a motor that holds a little more or less of the same propellant. |
| `burn_time_sd_fraction` | Each motor's thrust curve stretched in time by $1 + \sigma z$ and its thrust divided by the same: a longer, softer burn of the same impulse. |
| `ejection_delay_sd_s` | Each motor's [ejection delay](glossary.md#ejection-delay) plus $\sigma z$ seconds, never below zero. |
| `wind_speed_sd_fraction` | The wind at every height times $1 + \sigma z$, never below calm. A calm forecast stays calm. |
| `wind_heading_sd_rad` | The wind at every height turned $\sigma z$ clockwise: the forecast's direction, give or take. |
| `rail_elevation_sd_rad` | The rail's angle above the horizon plus $\sigma z$. Past vertical, it leans the other way. |
| `rail_azimuth_sd_rad` | The rail's heading plus $\sigma z$, clockwise. |
| `deployment_lag_sd_s` | Each recovery device's lag after its trigger plus $\sigma z$ seconds, never below zero. A part's [tumble](glossary.md#tumble-recovery), which the simulator adds to a separated part with no device open, starts at the split and has no lag to scatter. |

Five cases need a word:

- A draw that makes an input impossible fails that flight. With a 60% mass spread, a draw of $z$
  below −1.67 gives a negative mass; a rail drawn below the horizon is another. That flight is
  kept in the run as failed, with its reason, and counted (next section). The two delays and the
  wind's speed are the exceptions. A charge can't fire before its event and a wind can't blow at
  less than calm, so a draw below zero is flown as zero.
- A [cluster](glossary.md#cluster) is one draw. The simulator holds a cluster as one motor in a
  mount with several tubes, so all its motors get the same impulse and burn time.
- A vertical rail leans along one line. The elevation is dispersed in the plane of the rail's
  heading. So on a vertical rail with only its elevation dispersed, every flight leans towards or
  away from that heading, never sideways. RocketPy disperses its rail the same way, an inclination
  and a heading (`rocketpy/stochastic/stochastic_flight.py:21-24`, version 1.13.0). Disperse the
  heading too for leans in every direction.
- Every flight of a staged run separates where the nominal one does. It separates at the same
  event, or at the same time for a split the file times in seconds. A split timed by a burnout
  follows the dispersed burn; one timed in seconds doesn't. If a booster still burns at a split
  timed in seconds, the flight stops with its error and counts as failed. The exception is a split
  that may drop a burning motor, as OpenRocket's first burnout of a stage does
  ([the decision record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0172-a-stage-s-first-burnout.md)).
  `--delay-sd` moves a booster's ejection charge, and the parachute it fires, but not a split the
  file times in seconds.
- A part dropped on the way up needs its device open. After a split with nothing left to burn, each
  part flies as a point with only its open devices' drag. A part whose device fired by the split but
  waits out a dispersed lag would climb through the lag with no drag. So the simulator refuses that
  flight, and it counts as failed.

## How a run behaves

### Failed flights are counted

A run never drops a flight. [`Run::failed`] lists those that failed, saying whether the draw made
an impossible input or the flight refused it. A spread like [`Run::apogee`] counts every flight
tried: `attempted()` is the run's size, `count()` the flights that gave a value, and `missing()`
the rest. Its mean and percentiles are over the flights that gave a value, so many failures can
bias them: check `missing()` first.

A share such as "reached 1,100 m (3,609 ft)" is given as two bounds, `share_at_least(1100.0)`:

- `low` counts a failed flight as not reaching it;
- `high` counts it as reaching it.

With no failures the two are equal. With some, the truth lies between them, and a wide gap says the
run can't answer the question.

### The same seed, the same run

Every number a flight draws comes from its own stream of random numbers. The stream is picked out
by the run's seed, the flight's number in the run and the input it's for. So:

- flight 37 of a run is the same flight in a run of 100 or of 10,000;
- a run flown on one thread or on twelve is the same, bit for bit (with the `parallel` feature,
  [`run_parallel`]);
- turning a dispersion on or off doesn't change what the other inputs draw.

To fly on several threads, turn on the `parallel` feature where your program depends on the `hpr`
library. [Using it from your own program](api.md#using-it-from-your-own-program) describes how.
Then call `run_parallel` instead of `run`:

```toml
[dependencies]
fusionspace-hpr = { git = "https://github.com/nrdptel/fusionspace-eridanus", rev = "<commit>", features = ["parallel"] }
```

On another platform (operating system and processor) a draw can differ in its last binary digit,
because the normal numbers use the platform's logarithm. A run then agrees to many digits, not to
the bit.
[The decision record on Monte Carlo dispersion](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0134-monte-carlo-dispersion-independent-normals-one.md)
sets out the draws.

### How long a run takes

On the development machine, an Apple M5 with 10 cores, these are the times for 10,000 flights of
a Level 2 rocket. That's a rocket on a J, K or L [impulse class](glossary.md#impulse-class) motor.
Each flight runs from ignition to the ground, under a drogue and a main:

| rocket | peak Mach | 10,000 flights, 10 threads | one flight, one thread |
|---|--:|--:|--:|
| Valetudo, one of RocketPy's examples: 9.7 kg on a K400C | 0.3 to 0.4 | 3.0 s | 1.9 ms |
| a 66 mm (2.6 in) rocket with a 54 mm motor mount on a K940 | 1.6 to 2.0 | 9.4 s | 5.7 ms |

No flight failed. These are release builds, with optimization on: add `--release` to
`cargo run`, or build your program with it. A debug build, what plain `cargo run` gives, is
many times slower. To time it on your own machine, from a copy of the repository:

```text
cargo bench -p fusionspace-hpr --features parallel --bench ten_thousand
```

It takes about a minute. Its output, the before-and-after numbers and where the time goes are
on [Performance](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/perf.md). Other machines
haven't been timed.

A rocket that passes Mach 1.2 flies on a supersonic table. The table holds the body's lift and
center of pressure by the shock-expansion method
([The body faster than sound in a flight](physics/aero.md#the-body-faster-than-sound-in-a-flight)).
They're worked out once at every 0.05 Mach and then looked up. Building the table takes about a
fifth of a second for the rocket above.

A dispersion changes the rocket's masses, its motor, the weather and a scale on its drag, never
its shape. So the flights of a run share the nominal flight's table: the run builds it once. Each
flight also reuses the nominal rocket's layout, its parts placed and weighed. Every flight flies
exactly as it would alone, bit for bit. A unit test holds a supersonic flight to that, on one, two
and five threads (`samples_share_the_nominal_table_and_fly_as_alone`).

Before this speed-up ([sharing the table](decisions-and-roadmap.md#m6-1d), October 2026), each
flight built its own table. The supersonic run above then took 264 s. To fly draws of your own, as a
[sensitivity analysis](sensitivity.md) does, use `monte_carlo.fly(&draw)`, which shares the work.
Flying a draw's inputs yourself, `monte_carlo.inputs(&draw)?.fly()`, builds its own table and
layout.

A two-stage rocket's sustainer, built when the stages separate, still builds its own table in every
flight. Issue [#285](https://github.com/nrdptel/fusionspace-eridanus/issues/285) is about making
each flight itself faster.

The times say nothing about how accurate a Mach 2 flight is. No validation flight goes past
Mach 1.06
([The body faster than sound in a flight](physics/aero.md#the-body-faster-than-sound-in-a-flight)).

## Choosing the numbers

The spread a run gives is the spread you put in. Some places to start:

- **Motor impulse** has its limit in NFPA 1125, the code commercial motors are certified to
  (§8.1.7 and §8.2.7). Over a motor type's certification firings, the "standard deviation of the
  total impulse data shall be no greater than 6.7 percent of the mean". Four National Association
  of Rocketry (NAR) certification reports measured 1.3% to 3.1%: an Estes C6 (2.0%), D12 (3.1%),
  E12 (1.28%) and an AeroTech G80 (2.3%). The G80's report is hosted on
  [ThrustCurve.org](https://www.thrustcurve.org/motors/cert/60c63bfcb5bc370004713e8f/G80-20071207.pdf).
  So 2% to 3% is a fair guess, and 6.7% the most the code allows.
- **Ejection delay.** The code lets a measured delay differ from its label by "1.5 seconds or
  20 percent (whichever is greater, but not to exceed 3 seconds)". The E12 and G80 reports'
  firings of one delay scatter by 0.2 to 0.9 s. A delay's average can sit more than a second from
  its label: the G80's 7 s delay averaged 5.88 s. A dispersion is about the nominal value, so give
  the delay you expect, not only the one printed on the motor.
- **Burn time.** The code sets no limit of its own. The four reports disagree: the E12's and G80's
  burn times scatter by 1.5% and 1.8%, the older C6's and D12's by 17% and 18%. Why isn't recorded;
  the older sheets may time the burn differently. Start from a few percent for a composite motor,
  and try a larger value to see whether it matters to your flight.
- **Mass, center of mass and drag** depend on how well you know your rocket. A rocket weighed
  ready to fly needs a smaller mass dispersion than one weighed on paper. Drag is usually the least
  certain of the three: [Accuracy](accuracy.md) shows how far HPR Sim's drag sits from other
  programs' and from wind-tunnel data.
- **Wind** depends on the forecast, its age and the hour. A [sounding](glossary.md#sounding) of the
  day, or a forecast's spread between models, is a better guide than a guess.

The NFPA wording is the 2019 edition's. It's quoted in the public first-draft documents of the
code's next revision
([public inputs 4 and 5](https://docinfofiles.nfpa.org/files/AboutTheCodes/1125/1125_A2021_PYR_AAA_FD_PIResponses.pdf)).
The draft's first revisions, FR-7 and FR-8, keep both sentences
([`1125_A2021_PYR_AAA_FD_FRStatements.pdf`](https://docinfofiles.nfpa.org/files/AboutTheCodes/1125/1125_A2021_PYR_AAA_FD_FRStatements.pdf)).
The edition in force today hasn't been checked.

### What is not dispersed

- Inputs are independent: a heavier rocket isn't also draggier.
- Every dispersion is normal; there are no uniform or skewed ones.
- Moving a stage's center of mass keeps its inertia about the center.
- The drag dispersion scales the zero-lift drag only, not the [normal force](glossary.md#normal-force)
  or the moments; a recovery device's drag isn't dispersed.
- A stretched thrust curve keeps its shape.
- The atmosphere's temperature and pressure, a motor's ignition time, a separation's trigger or
  time, and events of your own.

## What comes next

The run gives each flight's whole [`FlightSummary`](api/hpr_sim/metrics/struct.FlightSummary.html),
so any number a flight reports can be spread with `run.distribution(...)`, as the example does for
the landing. To find which input moves the apogee most, see
[Sensitivity analysis](sensitivity.md).

The API reference is [`hpr_analysis::montecarlo`](api/hpr_analysis/montecarlo/index.html),
[`hpr_analysis::statistics`](api/hpr_analysis/statistics/index.html) and
[`hpr_analysis::ellipse`](api/hpr_analysis/ellipse/index.html).

[`Run::failed`]: api/hpr_analysis/montecarlo/struct.Run.html#method.failed
[`Run::apogee`]: api/hpr_analysis/montecarlo/struct.Run.html#method.apogee
[`run_parallel`]: api/hpr_analysis/montecarlo/struct.MonteCarlo.html#method.run_parallel
[`Run::landing`]: api/hpr_analysis/montecarlo/struct.Run.html#method.landing
[`Scatter::ellipse`]: api/hpr_analysis/ellipse/struct.Scatter.html#method.ellipse
[`Scatter::prediction_ellipse`]: api/hpr_analysis/ellipse/struct.Scatter.html#method.prediction_ellipse
[`Scatter::share_inside`]: api/hpr_analysis/ellipse/struct.Scatter.html#method.share_inside
[`Scatter::points`]: api/hpr_analysis/ellipse/struct.Scatter.html#method.points
