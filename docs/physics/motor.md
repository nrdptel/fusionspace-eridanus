# Solid motors

Learn how to fly a commercial solid motor in HPR Sim, and how it models the thrust curve, propellant
burn, mass and ejection delay.

## In short

- **What it models:** commercial solid motors: the thrust curve and its statistics (total
  impulse, burn time, average thrust, class), and how the propellant burns away. It also
  models the motor's mass, center of mass and inertia as it burns, thrust at altitude, and
  ejection delays.
- **Sources:** NASA SP-8039 (1971), the National Association of Rocketry's *Standard Motor
  Codes*, ThrustCurve.org's glossary, statistics page and code, and RocketPy 1.13.0's motor code.
- **How well it is validated:** by *analytic* tests (exact answers worked out by hand) and by
  *another code* ([code-to-code](../glossary.md#code-to-code-comparison)). Those are the first and
  third of the four [kinds of evidence][levels]. Small numbers here are in
  [scientific notation](../glossary.md#scientific-notation), and a
  [relative](../glossary.md#relative-and-absolute-difference) difference is a fraction of the value
  compared. ThrustCurve.org's own statistics code agrees to 1.8e-15, relative, on all 32 bundled
  curves. On three of them, RocketPy's total mass and inertias agree within 7.9e-5 relative, the
  centers of mass within 5.9e-6 of the motor's length. The propellant's own mass and
  inertias agree within 1e-4 of their values at ignition. OpenRocket 24.12 reads the same curve
  files. On all 32 bundled curves it gets exactly the same total impulse, peak thrust, burn-time
  window and curve duration. Its average thrust differs by definition: HPR Sim's is +0.0107% to
  +0.3147% higher. Nothing else in this model is
  compared with OpenRocket: not the propellant's burn-back, its mass and inertias, or the delays.
  Nothing here is compared with a real flight.
- **What it leaves out:** anything but [commercial off-the-shelf](../glossary.md#cots-motor)
  solids. Only 32 curves are bundled, none in class A. Propellant burns in proportion to the
  impulse delivered, an approximation. With only catalog data, the center of mass stays at
  mid-length. Commercial motor files give no nozzle exit size, so thrust at altitude goes
  uncorrected unless one is supplied.

## Using a motor

A flight needs a motor in the rocket's motor mount. There are two ways to get one:

- take one of the 32 motors that come with HPR Sim, or
- read a thrust-curve file, such as one downloaded from
  [ThrustCurve.org](../glossary.md#thrustcurveorg), by hand or through the library
  ([Matching motors to ThrustCurve's records](../motor-stock.md#matching-motors-to-thrustcurves-records)).

Either way, the motor then goes into one of the rocket design's configurations. A short program,
[`crates/hpr-sim/examples/motors.rs`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-sim/examples/motors.rs),
shows all of this: it lists the built-in motors, reads a `.eng` file, and flies that motor in a
rocket. Run it from anywhere in the repository (the setup is on
[Getting started](../getting-started.md)):

```bash
cargo run --example motors -p fusionspace-hpr-sim
```

It prints this. The output is committed in
[`motors.output.txt`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-sim/examples/motors.output.txt).
CI runs the program on macOS, Windows and Linux and fails if it prints anything else. So the list
below is always the list the code bundles.

<!-- quote: crates/hpr-sim/examples/motors.output.txt -->
```text
32 motors come built in:

                                           dia  length    loaded       total   average    burn
designation  maker     type        class    mm      mm    mass g impulse N·s  thrust N  time s
B4           Quest     single-use  B        18      80      19.8         4.9       4.4    1.11
C5           Estes     single-use  C        18      70      23.8         7.8       3.9    1.98
D5           Quest     single-use  D        20      96      45.1        17.6       3.8    4.59
E26W         AeroTech  single-use  E        24      88      44.0        27.6      23.5    1.18
26E31-15A    Cesaroni  reload      E        24      69      52.0        26.1      30.7    0.85
F52C         AeroTech  single-use  F        29     111      81.4        66.3      52.8    1.25
68F240-15A   Cesaroni  reload      F        24     133      91.8        68.0     236.6    0.29
F15          Estes     single-use  F        29     114     103.0        49.6      14.5    3.42
G69N         AeroTech  reload      G        38     106     201.0       136.3      72.1    1.89
131G84-10A   Cesaroni  reload      G        24     228     172.0       131.3      84.2    1.56
H170M        AeroTech  reload      H        38     191     330.0       318.0     165.4    1.92
168H54-10A   Cesaroni  reload      H        29     187     209.0       168.2      53.8    3.13
H125-CT      Loki      reload      H        38   177.8     342.0       241.7     125.0    1.93
I175WS       AeroTech  single-use  I        38     214     348.0       333.2     177.5    1.88
411I175-14A  Cesaroni  reload      I        38     245     437.5       411.4     174.1    2.36
I377-CT      Loki      reload      I        38     292     560.0       525.8     377.9    1.39
J450DM       AeroTech  single-use  J        54     359    1223.0      1061.6     465.6    2.28
1266J760-19A Cesaroni  reload      J        54     329    1076.8      1267.3     758.2    1.67
J300LR       Loki      reload      J        54     327    1315.0      1212.8     297.0    4.08
K400C        AeroTech  single-use  K        54     359    1194.0      1307.3     409.8    3.19
2245K1075-P  AMW       reload      K        54     728    2638.8      2255.1    1072.9    2.10
1633K940-18A Cesaroni  reload      K        54     404    1366.5      1636.1     936.9    1.75
L2500ST      AeroTech  reload      L        98     443    4989.5      4671.9    2501.7    1.87
3300L3200-P  Cesaroni  reload      L        75     486    3263.7      3299.6    3206.8    1.03
L1040LR      Loki      reload      L        54     736    2962.0      3722.5    1037.1    3.59
M1350W       AeroTech  single-use  M        75     622    4808.0      5179.5    1350.1    3.84
8187M1545-P  Cesaroni  reload      M        75    1025    7878.3      8181.8    1547.7    5.29
M1378LR      Loki      reload      M        54    1108    4331.0      5362.3    1375.6    3.90
N3300R       AeroTech  reload      N        98    1060   12269.0     14035.1    3189.9    4.40
13628N5600-P Cesaroni  reload      N        98    1010   11280.0     13633.5    5626.1    2.42
O6000W       AeroTech  single-use  O       152    1120   31524.6     39687.2    5828.8    6.81
21062O3400-P Cesaroni  reload      O        98    1239   16842.0     21041.0    3424.3    6.14

From the .eng file: I377CT by Loki, 38 mm by 292 mm, 560 g loaded, 250 g of propellant
  525.8 N·s (class I), 377.9 N average over 1.39 s, 657.0 N peak
  effective exhaust velocity 2103 m/s

Valetudo on the I377CT: 8.82 kg at liftoff
  leaves the 3 m rail at 17.7 m/s
  apogee 151.7 m above the pad, at 6.09 s
Not yet validated: see the Accuracy page before trusting these numbers.
```

The whole program is [at the end of this page](#the-example-program).

### The bundled motors

Each curve is a ThrustCurve.org file marked public domain, bundled unchanged. The size, masses and
delays come from its header, and the rest is worked out from its curve. Its ThrustCurve.org
record's impulse, thrusts and burn time are bundled too, as published facts with
credit ([third-party notices](https://github.com/nrdptel/fusionspace-eridanus/blob/main/THIRD-PARTY-NOTICES.md)).
They aren't the maker's data. Each file is checked against them: its total impulse, average
thrust and burn time must each be within 1% of the record's to pass. A gap is the file's figure
less the record's, as a share of the record's, so `+` means the file reads high. A motor flies a
passing file where it has one, else the first by format and by who measured it. Today each of
the 32 has one file, and it passes. Flying a failing file in `hpr sim` warns, naming each failing figure:
`burn time +2.00%`. Check such a file against the maker's data.
`hpr motors show` lists each file with its check. `cargo xtask motor-catalog` writes the catalog,
and a test fails if the committed one differs.

| column | what it is | where it comes from |
|---|---|---|
| designation | The motor's full name ([motor designation](../glossary.md#motor-designation)) | the motor's name, as ThrustCurve.org lists the file |
| maker | The manufacturer, abbreviated | as ThrustCurve.org lists the file |
| type | `single-use`, or `reload`: a propellant load for a reusable case | as ThrustCurve.org lists the file |
| class | The [impulse class](../glossary.md#impulse-class) letter | HPR Sim, from the curve |
| dia, length | The case's diameter and length, mm | the file's header |
| loaded mass | The motor ready to fly, g. For a reload this includes the case | the file's header |
| total impulse | [Total impulse](../glossary.md#total-impulse), N·s | HPR Sim, from the curve |
| average thrust | [Average thrust](../glossary.md#average-thrust), N | HPR Sim, from the curve |
| burn time | [Burn time](../glossary.md#burn-time), s | HPR Sim, from the curve |

How the 32 were chosen:

- They are public-domain ThrustCurve.org curves of motors in production.
- When they were chosen (September 17, 2026), each curve's total impulse, burn time and average thrust
  were within 1% of the values ThrustCurve.org publishes for the motor. Peak thrust is not held to it: it runs from 16.7% below
  (Cesaroni 26E31-15A) to 2.1% above (Loki M1378LR).
- The files' headers differ from ThrustCurve.org's records in a few places, and the header is
  what the simulator flies. Five lengths differ, from 1.2% shorter (the B4) to 1.3% longer (the
  N3300R). So do 7 propellant masses, from 2.7% lighter (the C5) to 52% heavier (the 26E31-15A).
  So do 6 loaded masses, from 4.0% lighter (the C5) to 2.3% heavier (the D5). For the 26E31-15A
  the header is the likelier: the record's 11.1 g of propellant would need an effective exhaust
  velocity of 2,350 m/s (7,710 ft/s) for its 26.1 N·s, more than every bundled motor but the
  K400C. The delays are the header's too, which often
  lists fewer settings than the maker sells.
- There are up to three per impulse class, from different manufacturers. None is in class A.
- The selection, and the curves left out, are in
  [the ThrustCurve.org data notes](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/thrustcurve-data.md).

In code, `Catalog::bundled()` returns the list
([`Catalog`](../api/hpr_motor/catalog/struct.Catalog.html)). `catalog.find("I377")` finds a motor
by its designation or common name, ignoring case, spaces and hyphens. It returns every match:
`I175` finds both the AeroTech I175WS and the Cesaroni 411I175-14A. Then
`entry.bundled_motor()` builds the motor
([`CatalogMotor`](../api/hpr_motor/catalog/struct.CatalogMotor.html)), with the catalog's size and
masses. The catalog doesn't say where inside the motor its mass sits, so the simulator uses a
rough guess, the *envelope default*. The propellant and the rest of the motor (case, nozzle and
closures) are each spread evenly along its length. So the center of mass stays at mid-length as
it burns ([The whole motor](#the-whole-motor) gives the details).

### A motor from a file

Motor files come in two formats, and ThrustCurve.org serves both:

- [RASP `.eng`](../format/eng.md), a plain-text format named after RASP, the rocket simulation
  program it comes from. It is the more common of the two: 889 of the 1,712 solid-motor files
  ThrustCurve.org held when surveyed.
- [RockSim `.rse`](../format/rse.md), an XML format from the RockSim simulator.

The example reads a `.eng` file in four steps:

1. Read the text. For a file you downloaded, use `std::fs::read_to_string("my-motor.eng")?`.
   The motor crate never opens files itself. The example builds one of the bundled files into
   the program with `include_str!` instead, so that it runs anywhere.
2. Parse it. `eng::parse(&text)?` ([`eng::parse`](../api/hpr_motor/eng/fn.parse.html))
   returns the file's entries, one per motor (a file can hold several), and a list of warnings.
   The reader is lenient: it accepts the oddities real files have and reports each one, with its
   line number ([reader policy](../format/eng.md#reader-policy-lenient-with-diagnostics)). This
   file reads with no warnings.
3. Take the curve. `entry.thrust_curve()?` makes the [thrust curve](#thrust-curve), starting
   it from zero thrust at ignition.
4. Build the motor with `SolidMotor::from_envelope(curve, diameter_m, length_m, propellant_kg,
   loaded_kg)` ([`SolidMotor`](../api/hpr_motor/motor/struct.SolidMotor.html)). A `.eng` header
   gives the size in millimeters and the masses in kilograms. So the example multiplies the size
   by 0.001 and passes the masses as they are. **Check that step yourself:** the motor can't tell
   a size left in millimeters. The [units check](#the-effective-exhaust-velocity-is-a-units-check)
   looks at the impulse and the propellant mass, not the size, so any positive size is accepted.
   If the same size goes into the rocket, the design checks stop the flight for a diameter too
   wide for the mount. A length too long for it only draws a warning
   ([Putting it in a rocket](#putting-it-in-a-rocket)).

For a `.rse` file, use `rse::parse` ([`rse::parse`](../api/hpr_motor/rse/fn.parse.html)) instead.
Its motors are in `engines` rather than `entries`. Its masses are in grams (`initial_mass_g`,
`propellant_mass_g`), so multiply those by 0.001 too. Forget it for the propellant, and the units
check catches it: the exhaust velocity is 1,000 times too small, and the motor is refused. The
check sees only the propellant mass. Forget it only for the loaded mass, and the motor is accepted
as 1,000 times heavier.

The file here is Loki Research's I377. The program prints what it read, `38 mm by 292 mm` (1.50 by 11.5 in), 560 g
loaded with 250 g of propellant. From the curve it works out 525.8 N·s, an I motor, averaging
377.9 N over 1.39 s. Its effective exhaust velocity, 2,103 m/s (6,900 ft/s), is well inside the check's range.

Three things to know about a motor built from a file:

- The header can be wrong. Whoever made the file typed its size and masses. Of ThrustCurve.org's
  554 public-domain files, 66 headers give a length more than 1 mm (0.039 in) off the catalog's.
  Five give a diameter more than 0.5 mm (0.020 in) off
  ([data notes](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/thrustcurve-data.md),
  section 4). The 32 bundled motors fly their headers' values too. `cargo xtask motor-catalog` refuses a reload whose case
  names another diameter than its header. In Loft, one header listed a 54 mm motor as a 75 mm
  motor ([Loft lesson L43](../decisions-and-roadmap.md#l43)). For any other motor, check the
  header against the motor's page on ThrustCurve.org.

  A motor fetched from ThrustCurve.org gets one more check, against its record: a propellant mass
  over 5% below the record's draws a warning naming both
  ([`PROPELLANT_SHORT_BOUND`](../api/hpr_net/on_demand/constant.PROPELLANT_SHORT_BOUND.html)).
  With the file's loaded mass right, that flies too heavy a burnout mass and too low an apogee,
  the side that flatters a waiver. A heavier one draws none, and the loaded mass isn't checked
  ([survey](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/fetched-mass-survey.md),
  section 5).
- Where its mass sits is a rough guess, the *envelope default*
  ([The whole motor](#the-whole-motor)). If you know the grain sizes, `SolidMotor::new` with
  `Propellant::Grains` does better
  ([Where the propellant is](#where-the-propellant-is)).
- Its nozzle size is unknown, so its thrust is not corrected for altitude
  ([Thrust at altitude](#thrust-at-altitude)).

### Putting it in a rocket

A rocket design keeps its motors in *configurations*: named sets of motors, at most one in each
motor mount ([Motors and configurations](design.md#motors-and-configurations)). The example adds
one to Valetudo, the rocket of [Getting started](../getting-started.md): a
[`Configuration`](../api/hpr_design/config/struct.Configuration.html) with the id `from-file`,
holding one [`MountedMotor`](../api/hpr_design/config/struct.MountedMotor.html).

| field of `MountedMotor` | what it holds |
|---|---|
| `mount` | The id of the mount in the design, here `motor-mount` |
| `designation` | A name, for display |
| `diameter_m`, `length_m` | The case's size, in meters. The design checks compare the diameter with the mount's bore: a motor wider than its mount is an error, and the flight refuses to start, unless a nominal size's real case fits ([a nominal motor in its matching tube](design.md#a-nominal-motor-in-its-matching-tube)). A case that reaches forward past the mount's top is only a warning |
| `motor` | The motor built above |
| `delay` | The [ejection delay](../glossary.md#ejection-delay) chosen, if any. A parachute can fire on it, with the `MotorDelay` trigger ([Recovery](recovery.md#triggers-lag-and-release)) |

Valetudo's mount is 43 mm (1.7 in) inside, so this 38 mm motor fits. `Simulation::new(&rocket,
"from-file", ...)` then flies the configuration by its id: straight up from a 3 m (9.8 ft) rail, in calm
air, with no parachutes. The rocket weighs 8.82 kg at liftoff and leaves the rail at 17.7 m/s
(58 ft/s). It reaches 151.7 m (498 ft) above the pad at 6.09 s. These flight numbers are not
validated ([evidence row G24](../evidence.md#g24)). See [Accuracy](../accuracy.md) before
trusting them.

A design file holds configurations in the same form, as JSON, with the motor written out in full.
See the `configurations` list at the end of
[the Valetudo design](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/designs/rocketpy-valetudo.json).
Every motor lights at launch, $t = 0$, unless its `ignition` says otherwise: at a time, after
another motor's burnout, or after its stage separates ([Staging](staging.md)). It then burns on its
own clock from that moment.

## Code and conventions

Where the motor code lives, its sources, and the units and axes it uses.

### Code and sources

Code: `hpr_motor` ([API reference](../api/hpr_motor/index.html)), in its modules `curve`, `class`,
`motor`, `grains`, `mass`, `delay` and `catalog`. The file formats are described in
[`.eng` files](../format/eng.md) and [`.rse` files](../format/rse.md).

Each source has a short key in square brackets, used below. A source that can be downloaded is
*pinned*. Its address and a checksum of its bytes are in the
[reference lock file](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/refs.lock.toml),
under the name given. So anyone can fetch the same copy ([Checking a claim](../checking-a-claim.md)).

- **[SP]** NASA SP-8039, *Solid Rocket Motor Performance Analysis and Prediction* (1971),
  [free from NASA][sp], pinned as `nasa-sp-8039`. Page notes:
  [nasa-sp-8039-motor-definitions.md](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/nasa-sp-8039-motor-definitions.md).
- **[NAR]** National Association of Rocketry, *Standard Motor Codes*, as
  [archived on February 5, 2014][nar], pinned as `nar-standard-motor-codes`.
- **[TC-G]** ThrustCurve.org's [glossary][tc-g], pinned as `thrustcurve-glossary`.
- **[TC-S]** ThrustCurve.org's ["Motor Statistics" page][tc-s], pinned as `thrustcurve-motorstats`.
- **[TC-A]** ThrustCurve.org's statistics code, [`simulate/analyze/analyze.js`][tc-a] in the
  site's source at commit `577afa6`, pinned as `thrustcurve3-analyze`. It is under the ISC
  license, a permissive open-source license like MIT, so this project may read it and run it.
- **[RP]** RocketPy 1.13.0 (MIT), [`rocketpy/motors/motor.py`][rp-motor] and
  [`solid_motor.py`][rp-solid], pinned as `rocketpy`. Notes:
  [rocketpy-solid-motor.md](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/rocketpy-solid-motor.md).

Line numbers below, such as "lines 40–91", link to those lines of the source.

### Conventions

- Time $t$ is seconds from ignition.
- Positions are meters along the motor's axis **from the nozzle exit plane (where the exhaust
  leaves) toward the forward closure (the motor's front end)**. So $+z$ points toward the nose,
  like the body frame ([Frames](frames.md)). The design model places the motor's nozzle exit in
  the body frame ([its milestone](../decisions-and-roadmap.md#m1-4)).
- Every part is symmetric about the axis. $I_a$ is about the axis and $I_t$ about a transverse
  axis, both through the part's own center of mass (RocketPy's `I_33` and `I_11`).
- Motor files keep their own units (mm, kg or g). The model is SI: meters, kilograms and seconds.

## The model

How a motor's thrust, propellant, mass, altitude effect and delay are worked out.

### Thrust curve

A [thrust curve](../glossary.md#thrust-curve) is the motor's thrust against time, as its file
lists it.

- Samples $(t_i, F_i)$ joined by straight lines, as [RP] reads them (`interpolation_method`
  `"linear"`). Before the first sample the curve starts from $(0, 0)$ when $t_0 > 0$, as the RASP
  format specifies ([`.eng` files](../format/eng.md#thrust-curve-r-data-r-problems)) and [TC-A]
  integrates. From the last sample on $F = 0$, that instant included. Like any step, the end
  takes the later value. So at burnout the thrust, the mass flow and the propellant left are all
  zero together.
- Equal consecutive times are a **step**: the later value holds from that time on. Real files use
  them for an abrupt burnout. Decreasing times and negative thrust are rejected, and so is a curve
  with no impulse or no burn time (defined below).
- A sample is **delivered** when it starts or ends a stretch of the curve that lasts some time
  ($t_i < t_{i+1}$ or $t_{i-1} < t_i$). The thrust then really takes its value. A sample that
  starts a stretch is reached at its own instant. One that ends a stretch is approached ever
  closer as time runs up to its instant. A sample strictly inside a run of three or more equal
  times is never delivered: the step jumps straight past it. Nor is the first sample of a step at
  the curve's start, or the last of a step at its end. **Peak thrust** is the largest delivered
  sample, which is exactly the most the curve's thrust reaches. The burn-time crossings below
  compare only delivered samples with the threshold. For example, times `0, 1, 1, 1, 2`&nbsp;s and
  thrusts `10, 10, 1000, 10, 0`&nbsp;N hold 15 N·s. The step skips the 1,000 N sample, so the thrust
  never exceeds 10 N, and 10 N is the peak. The burn runs from 0 to 1.95 s. Counting the skipped sample would set the
  5% threshold at 50 N, above anything the curve delivers. The curve would then be refused
  ([the decision record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0150-peak-thrust-and-burn-time-crossings-from-delivered.md),
  [issue #10](https://github.com/nrdptel/fusionspace-eridanus/issues/10), the report of the refusal). A one-off
  run over the surveyed ThrustCurve.org files found no file whose statistics change. Its scripts
  are not committed; the [decision record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0150-peak-thrust-and-burn-time-crossings-from-delivered.md)
  has the counts.
- Before integrating, [TC-A] drops leading points below 500 µN and averages points closer than
  50 µs ([lines 40–91][tc-a-40]). HPR Sim keeps leading zeros and treats equal times as steps. The
  results are identical on every bundled curve. On the 1,710 of ThrustCurve.org's 1,712 surveyed
  solid-motor files that read, the two agree to 1e-9, except 17 with repeated times.
  On those they differ by up to 1.1% in average thrust. HPR Sim keeps the step because a vertical
  drop is what the file draws.
- **Total impulse:** the exact integral of the lines
  ([SP] glossary p. 96: $I = \int F\,\mathrm{d}t$),

  $$
  \begin{aligned}
  I &= \sum_i \frac{1}{2} \left(F_i + F_{i+1}\right) \\
  &\quad \times \left(t_{i+1} - t_i\right).
  \end{aligned}
  $$

  $I(t)$ is the same sum up to $t$, with the partial interval.
- **Burn time** follows the rule of [NFPA 1125](../glossary.md#nfpa-1125), as ThrustCurve.org
  applies it. NFPA 1125 is the US National Fire Protection Association's code for making model and
  high-power rocket motors. The burn runs from the moment the thrust first reaches 5% of peak to
  the moment it last falls to 5% of peak. Each crossing is interpolated on its segment ([TC-G]
  "Burn Time"; [TC-S]; [TC-A] [lines 165–203][tc-a-165]). The peak and the crossings use
  delivered samples only (above). The last sample's time is not the burn time
  ([Loft lesson L39](../decisions-and-roadmap.md#l39): Loft took it as the burn time).
- **Average thrust:** total impulse over the NFPA 1125 burn time ([TC-G] "Average Thrust"; [TC-A]
  [line 231][tc-a-231]).
  - [TC-S] says instead "the total impulse during the 5%-defined burn time". The two differ by the
    impulse outside the window, a median 0.13% on ThrustCurve's public-domain curves
    ([data notes](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/thrustcurve-data.md)).
    HPR Sim follows the glossary and the site's code.
- **Impulse class:** each letter covers twice the total impulse of the one before. `A` goes up to
  2.5 N·s, `B` to 5, `C` to 10, and so on. In general $\mathrm{upper}(k) = 1.25 \cdot 2^k\,\mathrm{N\,s}$, from `1/8A`
  ($k = -2$) to `O` ($k = 15$), and on to `Z` by doubling. **Upper limits are inclusive:** [NAR]
  states `C` as "5.01 to 10.0 N-sec". Loft gave `B` for 2.5 N·s ([Loft lesson L38](../decisions-and-roadmap.md#l38)).

### Propellant consumption

The [effective exhaust velocity](../glossary.md#effective-exhaust-velocity) $c$ is the thrust
divided by the rate at which propellant mass leaves, $c = F/\dot{m}$ ([SP] glossary p. 95). **Held
constant** through the burn, it makes the propellant burned proportional to the impulse delivered
so far. When half the impulse has been delivered, half the propellant is gone. With $I$ the total
impulse, $I(t)$ the impulse delivered by time $t$, and $m_{p0}$ the propellant at ignition:

$$
\begin{aligned}
c &= \frac{I}{m_{p0}}, \\
\dot{m}(t) &= \frac{F(t)}{c}, \\
m_p(t) &= m_{p0} \left(1 - \frac{I(t)}{I}\right)
\end{aligned}
$$

- This is [RP] `SolidMotor` ([`solid_motor.py:401-418`][rp-401]; [`motor.py:483-524`][rp-483]).
  ThrustCurve's `.rse` files tabulate their mass column the same way
  ([`.rse` files](../format/rse.md)).
- Holding $c$ constant is an approximation. [SP] defines $c$ only instantaneously. Measured
  [specific impulse](../glossary.md#specific-impulse) ($I_{\mathrm{sp}}$, the same quantity divided by
  standard gravity) drifts during a burn. It drifts as the pressure inside the motor changes and
  the nozzle erodes ([SP] p. 14), so real consumption is not exactly proportional. No data
  published for commercial motors resolves the difference.
- All propellant is gone at the last sample. The whole burned mass leaves as exhaust. Inert
  material that also leaves (bits of liner and igniter) is not modeled.

### Where the propellant is

- **Column** (`Propellant::Column`): a hollow cylinder $(R, r, L)$ (outer radius, inner radius,
  length) at a fixed center. It keeps its shape and loses density as it burns, so

  $$
  \begin{aligned}
  I_a &= \frac{1}{2} m_p \left(R^2 + r^2\right), \\
  I_t &= m_p \left(\frac{R^2 + r^2}{4} + \frac{L^2}{12}\right)
  \end{aligned}
  $$

  This is [RP] `GenericMotor`'s
  model ([`motor.py:1580-1649`][rp-1580], a solid cylinder).
- **BATES grains** (`Propellant::Grains`): $N$ identical [BATES grains](../glossary.md#bates-grain)
  $(R, r_0, h_0)$ (outer radius, starting bore radius, starting height), spaced $h_0 + s$ apart, with
  a gap $s$ between them. Each burns on its bore, the hole along its axis. It also burns on both
  ends unless the ends are inhibited: coated so that they can't burn ([RP]
  [`solid_motor.py:487-632`][rp-487]).
  - Every grain needs a bore ($r_0 > 0$). A solid end burner is a grain with no hole that burns
    only on one face. It shortens without widening. This regression (the way the grain burns
    back) doesn't describe that, and neither does RocketPy's.
  - Facing ends burn even with no gap, as in [RP].
  - Every burning surface recedes by the same depth $x$, the web burned so far (the web is the
    thickness of propellant between the burning surfaces). The first formula holds with the ends
    burning, for $x \le \min(R - r_0, h_0/2)$; the second with the ends inhibited, for
    $x \le R - r_0$:

  $$
  \begin{aligned}
  V(x) &= \pi \left(R^2 - (r_0 + x)^2\right) \\
  &\quad \times (h_0 - 2x), \\
  V(x) &= \pi \left(R^2 - (r_0 + x)^2\right) \\
  &\quad \times h_0,
  \end{aligned}
  $$

  and

  $$
  N \rho\, V(x) = m_p(t)
  $$

  Here $V$ is one grain's volume and $\rho$ the propellant's density, so the last line says the $N$
  grains hold the propellant left.

  - [RP] integrates $\dot{r} = -\dot{V}/A_b$, $\dot{h} = -2\dot{r}$ in time, with $A_b$ the burning area. It uses
    LSODA, a general-purpose solver for ordinary differential equations (ODEs), from the Python
    library SciPy. Because $\mathrm{d}V/\mathrm{d}x = -A_b$, that ODE is the relation above. HPR Sim solves it for $x$ by
    safeguarded Newton iteration, exactly at any time. That is Newton's method, which improves a
    guess using the slope, kept inside an interval known to hold the answer. A step that would
    leave the interval halves it instead.
  - $m_{p0} = N \rho\, V(0)$ comes from the geometry, as in [RP].
  - About the stack's center, which doesn't move:

    $$
    \begin{aligned}
    I_a &= \frac{1}{2} m_p \left(R^2 + r^2\right), \\
    I_t &= m_p \frac{R^2 + r^2}{4} \\
    &\quad + m_p \frac{h^2}{12} \\
    &\quad + \frac{m_p}{N} (h_0 + s)^2 \\
    &\quad \times \frac{N (N^2 - 1)}{12}
    \end{aligned}
    $$

    The last term sums $m_g d_k^2$ over the grain offsets ([RP] [`solid_motor.py:724-740`][rp-724],
    [`784-789`][rp-784]).

### The whole motor

- The dry mass (case, closures, liner, nozzle: everything but the propellant) is one element:
  mass, center and inertias about its own center. `with_added_dry_mass` joins more hardware, such
  as a retainer (the cap or clip that holds the motor in its mount). It must be positive: at
  burnout it is the whole motor, and with no mass it has no center.
- The motor is the dry mass and the propellant combined about the instantaneous center with the
  [parallel-axis theorem](../glossary.md#parallel-axis-theorem):

  $$
  \begin{aligned}
  z &= \frac{1}{m} \sum_k m_k z_k, \\
  I_a &= \sum_k I_{a,k}, \\
  I_t &= \sum_k \left(I_{t,k} + m_k (z_k - z)^2\right)
  \end{aligned}
  $$

  This is [RP] `Motor.I_11`
  ([`motor.py:585-666`][rp-585]).
- The *envelope default* (`SolidMotor::from_envelope`) applies when only the motor's size and
  masses are known, as from a catalog entry or a motor file's header:
  - the propellant is a solid column of radius $D/2$ filling the length, centered at $L/2$;
  - the dry mass (loaded minus propellant) is a thin tube, $I_a = m r^2$ and
    $I_t = m (r^2/2 + L^2/12)$, also centered at $L/2$.

  **These are crude:** the center of mass stays at $L/2$ throughout. Loft fixed the CG at the
  midpoint with no inertia of its own ([Loft lesson L40](../decisions-and-roadmap.md#l40)). HPR Sim
  gives the parts inertia, and moves the CG as soon as the dry and propellant centers differ.
  - ThrustCurve's loaded mass includes the reusable case ([TC-G] "Total Weight": "propellant and
    case").
  - [RP] `GenericMotor.load_from_eng` sets its chamber radius to the motor **diameter**
    ([`motor.py:1759-1761`][rp-1759]). That quadruples the $r^2$ inertia terms (and the default
    nozzle area it derives from that radius). HPR Sim uses $D/2$.
- A catalog entry's diameter, length and masses are its curve file's header values
  ([A motor from a file](#a-motor-from-a-file)). ThrustCurve.org's own records differ in a few
  places ([the bundled motors](#the-bundled-motors)).

#### The effective exhaust velocity is a units check

Both constructors, `SolidMotor::new` and `SolidMotor::from_envelope`, work out the effective
exhaust velocity $c = I/m_p$ from the curve and the propellant mass. They refuse a motor whose $c$
is outside **200 to 5,000 m/s (656 to 16,404 ft/s)**. It is a guard against a slip in the units
of the propellant mass, not a filter on propellant:

- It catches a propellant mass in grams typed where kilograms belong, such as a `.rse` file's
  `propellant_mass_g` passed unconverted. That moves $c$ by a factor of 1,000. Nothing else in the
  API notices. Take the 411I175's envelope typed in millimeters and grams,
  `from_envelope(curve, 38.0, 245.0, 228.9, 437.5)`. It has positive, finite dimensions and a
  propellant mass below the loaded mass, and an exhaust velocity of 1.8 m/s (5.9 ft/s). The grams
  give it away, and the check refuses it.
- It can't catch a size in the wrong unit. $c$ doesn't depend on the diameter or the length, and
  the motor accepts any positive size. The example's I377 with its size left in millimeters,
  `from_envelope(curve, 38.0, 292.0, 0.250, 0.560)`, is accepted, at the same
  2,103 m/s (6,900 ft/s). Only the design checks see a size, and only the one given to the `MountedMotor`
  ([Checks](design.md#checks)). A diameter wider than the mount's bore is an error,
  `motor_wider_than_mount`, and the flight refuses to start. Within a nominal size's slack for its
  narrower case it only warns, `motor_tight_in_mount`. A case that runs forward past the
  mount's top is only a warning, `motor_past_mount_top`, so a length slip alone still flies.
  Nor does $c$ involve the loaded mass, so a loaded mass in grams passes too.
- It lets through every real motor checked. The 1,708 ThrustCurve.org files with a catalog
  propellant mass run from 236 to 3,031 m/s (774 to 9,944 ft/s). Their median is 1,867, and 90% of them are
  between 928 and 2,210. The 32 bundled motors run from **708.59 m/s** (2,324.8 ft/s, a
  black-powder C) to **2,651.64 m/s** (8,699.6 ft/s, a K). They use their files' own
  propellant masses.
- It is tightest at the floor: the lowest real value is 1.2x above it. Estes and Quest normally
  count the delay grain and the ejection charge as propellant, so their small black-powder motors
  read low. [Issue #11](https://github.com/nrdptel/fusionspace-eridanus/issues/11) records the
  fallback if one ever falls below 200 m/s (656 ft/s).

$I$ here is the curve's own impulse, uncorrected for air pressure. The reasoning in full, the
table of percentiles, and two figures that were once got wrong are in
[the exhaust-velocity note](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/exhaust-velocity-guard.md).

### Thrust at altitude

A motor pushes harder in thinner air. Its thrust has two parts. One is the momentum of the
exhaust. The other is a pressure term: the exhaust's pressure at the nozzle exit less the air's,
times the exit area ([SP] eq. 2, p. 3; SP-8039's unit constant $g_c$ is 1 in SI):

$$
F = \dot{m}\, u_e + (p_e - p_a)\, A_e
$$

Here $\dot{m}$ is the mass flow and $u_e$ the exhaust speed at the exit. $p_e$ is the exit
pressure, $p_a$ the ambient (air) pressure and $A_e$ the exit area. With the flow unchanged, a
curve measured at reference pressure $p_{\mathrm{ref}}$ gives, at ambient $p_a$,

$$
\begin{aligned}
F(p_a) &= F_{\mathrm{curve}} + (p_{\mathrm{ref}} - p_a)\, A_e, \\
A_e &= \pi r_e^2
\end{aligned}
$$

- [SP] doesn't print this conversion; it follows from eq. 2. [RP] `Motor.pressure_thrust`
  ([`motor.py:1173-1191`][rp-1173]) applies the same term.
- $p_{\mathrm{ref}}$ is the air pressure at the static test site, where the motor was fired on a test stand
  to measure its curve. It is stored with the nozzle (`Nozzle::reference_pressure_pa`). Motor
  files and catalogs don't record it, so a design gives it, or gives none (below). Standard
  sea-level pressure (101 325 Pa) suits a motor tested near sea level, but it is a guess. Take a
  test at 1,500 m (4,921 ft) elevation, 84.6 kPa in the 1976
  [standard atmosphere](../glossary.md#standard-atmosphere). For that motor, sea-level pressure
  makes the term 16.8 kPa × $A_e$ too large at every altitude.
- It holds while the exhaust fills the nozzle to its exit. A nozzle made for high altitude, tested
  at sea level, can have its flow come away from the nozzle wall (separate). Then the conversion
  doesn't hold ([SP] pp. 32–34).
- HPR Sim applies it strictly inside the burn, $0 < t < t_{\mathrm{end}}$ ($t_{\mathrm{end}}$ the curve's last time), as
  RocketPy's flight does ([`simulation/flight.py:1936-1956`][rp-flight]). HPR Sim applies the
  term only where the curve's thrust is positive; RocketPy also adds it inside zero-thrust gaps,
  where nothing flows. HPR Sim never lets thrust go negative. Without a known nozzle it returns the
  curve. Commercial motor files carry no exit diameter: `.eng` has no field for one, and the
  `.rse` format's `exitDia` attribute is always 0 ([`.rse` files](../format/rse.md#engine-attributes)).
  So a motor read from a file or the catalog gets no correction.
- A nozzle may leave `reference_pressure_pa` empty (`None`, `null` in a design file). The curve is
  then flown as it is at every pressure, which is what RocketPy does by default: its
  `Motor(reference_pressure=None)` makes `pressure_thrust` zero ([`motor.py:1188-1189`][rp-1173]).
  The designs transcribed from RocketPy's examples say `None` for that reason
  ([ADR-021][adr-021]). Take the Valetudo of [Getting started](../getting-started.md), at a
  1,400 m (4,593 ft) site. With the sea-level stand-in it carried about 23 N of thrust
  (15.7 kPa × 1.47e-3 m²) that RocketPy's example never flies. It reached 874 m (2,867 ft), where it now
  reaches 779 m (2,556 ft). A design must say which it means. The key is required, `null` for none, so
  leaving it out is an error rather than a silent choice. The unit test
  `hpr_motor::motor::tests::pressure_correction_uses_the_exit_area` pins both forms and the
  missing key.
- The full-flow term steps in just after ignition and steps to zero at $t_{\mathrm{end}}$ (the
  integrator should treat both as [events](../glossary.md#event)). The ignition transient is the
  first moments, while the pressure inside the motor builds. The tail-off is the end of the burn,
  while that pressure falls. In both, the real exit pressure is far from its full-flow value, so
  the term misstates thrust there. In the tail-off, where the exit pressure falls with the
  chamber's, the term overstates thrust. Take the 411I175, with a 9.5 mm (0.37 in) exit, in vacuum. In the 0.14 s
  after the NFPA 1125 burn ends, the term adds 3.9 N·s, and the curve itself delivers 0.45 N·s.
  The 3.9 N·s is 0.95% of total impulse.

### Delays

A motor's [ejection delay](../glossary.md#ejection-delay) is the time from burnout to its ejection
charge, which deploys the recovery. [TC-G] lists every achievable delay, adjustable ones included.
A plugged motor has no ejection charge, and files mark it `P`. See
[`hpr_motor::delay`](../api/hpr_motor/delay/index.html) and [`.eng` files](../format/eng.md) for
the markers files use. A `0` is read as its own "zero or plugged" setting. The RASP spec says it
means ejection at burnout, but most files mean plugged. HPR Sim never turns it into an ejection
event by itself: the user has to decide.

## Validation and example

The tests that pin the motor model, and the whole example program.

### Validation

- **Catalog** (`catalog::tests`): every figure the catalog lists is its curve file's own, bit for
  bit, so what `hpr motors` lists is what flies. Each file's stored check is worked out again from
  the file and its motor's committed record. All 32 pass: 1% was **the bundling rule**. In Loft,
  a mis-sourced curve flew about 26% high
  ([Loft lesson L42](../decisions-and-roadmap.md#l42)). Of 554 public-domain curves, 196 passed
  ([data notes](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/thrustcurve-data.md),
  which also list the four motors whose stored values are printed coarser than 1%). Each bundled
  file is public domain, its bytes matching the SHA-256 checksum (a fingerprint of a file's exact
  bytes) recorded at download. These checks answer Loft lessons
  [L41](../decisions-and-roadmap.md#l41) to [L43](../decisions-and-roadmap.md#l43), on curve
  licenses, loose impulse checks and wrong headers.
- **ThrustCurve's code** (`catalog::tests::every_bundled_curve_matches_thrustcurve_statistics_code`):
  [`validation/oracles/thrustcurve/analyze_stats.js`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/oracles/thrustcurve/analyze_stats.js)
  runs [TC-A] unchanged on every bundled curve, and writes its results to a
  [fixture](../glossary.md#reference-value-and-fixture),
  [`validation/fixtures/motor/thrustcurve-analyze-stats.json`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/motor/thrustcurve-analyze-stats.json).
  HPR Sim's impulse, burn window, burn time, average and peak thrust agree to 1.8e-15. So the
  definitions are checked, not only the 1% rule.
- **OpenRocket** (`catalog::tests::openrocket_s_total_impulse_matches_every_bundled_curve`):
  [`validation/oracles/openrocket/motors.py`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/oracles/openrocket/motors.py)
  hands each bundled file to OpenRocket 24.12's own motor loader. It records what the loader makes
  of it in
  [`validation/fixtures/motor/openrocket-curve-stats.json`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/motor/openrocket-curve-stats.json),
  tied to each file by its SHA-256. The two codes are compared on the **same bytes**, so what is
  measured is the arithmetic, not two catalogs' data for one motor. Four numbers are **identical**,
  to the last bit of an `f64`, on all 32 curves. They are total impulse, peak thrust, the
  5%-of-peak burn-time window (OpenRocket's `getBurnTimeEstimate`) and the curve's whole duration.
  That duration is OpenRocket's `getBurnTime`, the last listed time rather than a window. The test
  holds the impulse to the 0.1% that [M2.2c](../decisions-and-roadmap.md#m2-2c) asks for, met
  with everything to spare. It also asserts the bit-for-bit equality, so this paragraph cannot go
  stale while the suite stays green. Both codes integrate the listed points as straight lines.
  Both prepend `(0, 0)` to a file whose first point is after ignition (29 of the 32 start between
  1 and 40 ms). So the point counts match as well, and nothing rounds differently.
  - One definition genuinely differs, and is recorded rather than held ([ADR-066][adr-066]): the
    **average thrust**. Both codes divide by the same 5% window. OpenRocket's numerator is the
    impulse *inside* it, while HPR Sim's, following [TC-A], is the whole curve's. The tails below
    5% are the difference. So HPR Sim's average is the higher on every one of the 32, by
    **+0.0107% to +0.3147%** (median +0.0965%). The test prints all three. Carrying an average
    thrust between the two codes means saying which numerator it used.
  - Curves outside this repository are held to the same bound. They are not committed, so
    `cargo xtask ork` checks them on the machine that has them. It fails if one is outside 0.1%
    ([M2.2c2](../decisions-and-roadmap.md#m2-2c2), [ADR-067][adr-067]):
    - The curves the reference library's designs embed, which are other people's data. HPR Sim
      parses each file itself, so these are real checks. All 3 agree to the last bit.
    - Every solid curve in the motor database OpenRocket 24.12 ships: 1,288 of its 1,452
      motors, the rest hybrids. The survey supplies them to designs that name a curve by
      [digest](../glossary.md#digest) without carrying it. Both codes integrate the samples
      OpenRocket has already parsed, so agreement to the last bit proves the hand-off, not two
      independent readings.

    The counts, and what the database motors' masses leave out, are on the
    [`.ork` format page](../format/ork.md#motors-in-the-reference-library).
- **RocketPy** (`motor::tests::matches_rocketpy_solid_motor_for_three_bundled_motors`): three
  bundled curves with BATES loads. They cover grains that burn out radially (the bore reaches the
  outer wall first) and axially (the burning ends meet first). They also cover inhibited ends, and
  both of RocketPy's axis directions (positions measured from the nozzle forward, or toward the
  nozzle).
  [`validation/oracles/rocketpy/solid_motor.py`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/oracles/rocketpy/solid_motor.py)
  writes
  [`validation/fixtures/motor/rocketpy-solid-motor.json`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/motor/rocketpy-solid-motor.json),
  and regenerates it byte for byte.
  - On 203 times per motor, total mass, $I_t$ and $I_a$ agree within 7.9e-5 of
    RocketPy's own values. At the same times the center of mass agrees within 5.9e-6 of the motor
    length.
  - Quantities that go to zero are compared against a fixed scale. Propellant mass and inertias
    use their ignition values, grain height its initial height, and centers the motor length. On
    that scale they agree within 1e-4. Before burnout the propellant mass differs by up to 69% of
    RocketPy's own value, where RocketPy holds 6.9 mg in the 38 mm motor. That comes from
    RocketPy's interpolation between the points its ODE solver stored.
  - The test holds 0.1% on these scales.
- **Files:** 1,710 of ThrustCurve's 1,712 solid-motor files read, and write back out and read again
  with every value identical to the last bit. The other two have times that go backwards
  ([`.rse` files](../format/rse.md#checked-against-real-files)).
- **Unit and property tests** cover exact impulse integration, burn windows, grain volume
  inversion, the parallel-axis theorem, and the impulse-fraction flow integrating to $m_{p0}$.
  Property tests check a rule on many random inputs.

### The example program

This is the whole of
[`crates/hpr-sim/examples/motors.rs`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-sim/examples/motors.rs),
line for line the file CI runs. [Using a motor](#using-a-motor) walks through what it does.

<!-- quote: crates/hpr-sim/examples/motors.rs -->
```rust
//! Motors: the thrust curves that come with HPR Sim, and a motor read from a RASP `.eng` file
//! like the ones ThrustCurve.org serves, put in a rocket's motor mount and flown.
//!
//! Run it from anywhere in the repository:
//!
//! ```text
//! cargo run --example motors -p fusionspace-hpr-sim
//! ```
//!
//! The documentation site's *Solid motors* page (`docs/physics/motor.md`, "Using a motor") walks
//! through it. What it prints is kept next to it in `motors.output.txt`, and CI checks that the
//! two still agree (`cargo xtask examples --check`).

#![allow(
    clippy::print_stdout,
    reason = "the project's lints forbid printing in library code, and this program exists to print"
)]

use std::error::Error;

use hpr_core::geodesy::Geodetic;
use hpr_design::{Configuration, Ignition, MountedMotor, Rocket};
use hpr_motor::catalog::MotorType;
use hpr_motor::{Catalog, ImpulseClass, SolidMotor, eng};
use hpr_sim::{Environment, EventKind, FlightSettings, Rail, Simulation, Termination};

fn main() -> Result<(), Box<dyn Error>> {
    // 1. The motors that come built in. Size and loaded mass are each curve file's header values;
    //    impulse, class, average thrust and burn time are worked out here from each motor's curve.
    let catalog = Catalog::bundled()?;
    println!("{} motors come built in:", catalog.motors.len());
    println!();
    let heading = [
        [
            "", "", "", "", "dia", "length", "loaded", "total", "average", "burn",
        ],
        [
            "designation",
            "maker",
            "type",
            "class",
            "mm",
            "mm",
            "mass g",
            "impulse N·s",
            "thrust N",
            "time s",
        ],
    ];
    for [a, b, c, d, e, f, g, h, i, j] in heading {
        println!("{a:<12} {b:<8}  {c:<10}  {d:<5} {e:>5} {f:>7} {g:>9} {h:>11} {i:>9} {j:>7}");
    }
    for entry in &catalog.motors {
        let motor = entry.bundled_motor()?;
        let curve = motor.curve();
        let kind = match entry.motor_type {
            MotorType::SingleUse => "single-use",
            MotorType::Reload => "reload",
            _ => "other",
        };
        let class = ImpulseClass::from_total_impulse(curve.total_impulse_ns())?;
        // An entry the catalog gives no loaded mass for would show a dash; every bundled one has
        // a loaded mass.
        let loaded_g = entry
            .total_mass_g
            .map_or_else(|| "-".to_owned(), |mass_g| format!("{mass_g:.1}"));
        println!(
            "{:<12} {:<8}  {kind:<10}  {:<5} {:>5} {:>7} {loaded_g:>9} {:>11.1} {:>9.1} {:>7.2}",
            entry.designation,
            entry.manufacturer_abbrev,
            class.label(),
            entry.diameter_mm,
            entry.length_mm,
            curve.total_impulse_ns(),
            curve.average_thrust_n(),
            curve.burn_time_s(),
        );
    }

    // 2. A motor from a file. This is one of the bundled files, built into the program so that it
    //    runs anywhere. For a file you downloaded, read its text instead with
    //    `let text = std::fs::read_to_string("path/to/your-motor.eng")?;`
    let text = include_str!("../../hpr-motor/data/thrustcurve/curves/5f4294d20002e90000000863.eng");
    let parsed = eng::parse(text)?;
    for warning in &parsed.warnings {
        println!("warning, line {}: {}", warning.line, warning.message);
    }
    let [entry] = &parsed.value.entries[..] else {
        return Err("expected one motor in the file".into());
    };
    // The file gives the size in millimeters and the masses in kilograms; the motor takes meters
    // and kilograms. Only the size needs converting, and the motor can't tell if it isn't: its
    // units check looks at the impulse and the propellant mass, not the size.
    let diameter_m = entry.diameter_mm * 1e-3;
    let length_m = entry.length_mm * 1e-3;
    let motor = SolidMotor::from_envelope(
        entry.thrust_curve()?,
        diameter_m,
        length_m,
        entry.propellant_mass_kg,
        entry.total_mass_kg,
    )?;
    let curve = motor.curve();
    println!();
    println!(
        "From the .eng file: {} by {}, {} mm by {} mm, {:.0} g loaded, {:.0} g of propellant",
        entry.name,
        entry.manufacturer,
        entry.diameter_mm,
        entry.length_mm,
        entry.total_mass_kg * 1e3,
        entry.propellant_mass_kg * 1e3,
    );
    println!(
        "  {:.1} N·s (class {}), {:.1} N average over {:.2} s, {:.1} N peak",
        curve.total_impulse_ns(),
        ImpulseClass::from_total_impulse(curve.total_impulse_ns())?,
        curve.average_thrust_n(),
        curve.burn_time_s(),
        curve.peak_thrust_n(),
    );
    println!(
        "  effective exhaust velocity {:.0} m/s",
        motor.exhaust_velocity_m_s()
    );

    // 3. Into a rocket: a new configuration that puts the motor in the design's motor mount. The
    //    rocket is Valetudo, the rocket of `first_flight.rs`. Its mount is 43 mm inside, so this
    //    38 mm motor fits: the design checks compare the case diameter given here with the bore.
    let mut rocket: Rocket = serde_json::from_str(include_str!(
        "../../../validation/designs/rocketpy-valetudo.json"
    ))?;
    rocket.configurations.push(Configuration {
        id: "from-file".to_owned(),
        name: format!("{} from its .eng file", entry.name),
        motors: vec![MountedMotor {
            mount: "motor-mount".to_owned(),
            designation: entry.name.clone(),
            diameter_m,
            length_m,
            motor,
            // No ejection delay is chosen: this flight carries no parachutes.
            delay: None,
            ignition: Ignition::Launch,
            failed_tubes: Vec::new(),
        }],
    });

    // Fly it, straight up from a 3 m rail in calm air, with no parachutes.
    let site = Geodetic::from_degrees(32.99, -106.97, 1400.0)?;
    let simulation = Simulation::new(
        &rocket,
        "from-file",
        Environment::standard(site)?,
        Rail::vertical(3.0),
        FlightSettings::default(),
    )?;
    let flight = simulation.run(&mut ())?;
    if flight.termination != Termination::GroundHit {
        return Err(format!("the flight ended with {:?}", flight.termination).into());
    }
    let sample = |kind| {
        flight
            .event(kind)
            .map(|event| event.sample)
            .ok_or(format!("the flight has no {kind:?}"))
    };
    let (liftoff, rail_exit, apogee) = (
        sample(EventKind::Liftoff)?,
        sample(EventKind::RailExit)?,
        sample(EventKind::Apogee)?,
    );
    println!();
    println!(
        "Valetudo on the {}: {:.2} kg at liftoff",
        entry.name, liftoff.mass_kg
    );
    println!(
        "  leaves the 3 m rail at {:.1} m/s",
        rail_exit.cg_velocity_enu_m_s.length()
    );
    println!(
        "  apogee {:.1} m above the pad, at {:.2} s",
        apogee.height_above_ground_m, apogee.time_s,
    );
    println!("Not yet validated: see the Accuracy page before trusting these numbers.");
    Ok(())
}
```

[adr-066]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0066-every-curve-hpr-flies-is-integrated-as.md
[adr-067]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0067-curves-come-from-openrockets-own-database-by.md
[levels]: ../accuracy.md#four-kinds-of-evidence
[sp]: https://ntrs.nasa.gov/api/citations/19720011135/downloads/19720011135.pdf
[nar]: https://web.archive.org/web/20140205181530/http://www.nar.org/NARmotors.html
[tc-g]: https://www.thrustcurve.org/info/glossary.html
[tc-s]: https://www.thrustcurve.org/info/motorstats.html
[tc-a]: https://github.com/JohnCoker/thrustcurve3/blob/577afa62302f70c6b2ba04e97a39240638cd704b/simulate/analyze/analyze.js
[tc-a-40]: https://github.com/JohnCoker/thrustcurve3/blob/577afa62302f70c6b2ba04e97a39240638cd704b/simulate/analyze/analyze.js#L40-L91
[tc-a-165]: https://github.com/JohnCoker/thrustcurve3/blob/577afa62302f70c6b2ba04e97a39240638cd704b/simulate/analyze/analyze.js#L165-L203
[tc-a-231]: https://github.com/JohnCoker/thrustcurve3/blob/577afa62302f70c6b2ba04e97a39240638cd704b/simulate/analyze/analyze.js#L231
[rp-motor]: https://github.com/RocketPy-Team/RocketPy/blob/v1.13.0/rocketpy/motors/motor.py
[rp-solid]: https://github.com/RocketPy-Team/RocketPy/blob/v1.13.0/rocketpy/motors/solid_motor.py
[rp-401]: https://github.com/RocketPy-Team/RocketPy/blob/v1.13.0/rocketpy/motors/solid_motor.py#L401-L418
[rp-483]: https://github.com/RocketPy-Team/RocketPy/blob/v1.13.0/rocketpy/motors/motor.py#L483-L524
[rp-1580]: https://github.com/RocketPy-Team/RocketPy/blob/v1.13.0/rocketpy/motors/motor.py#L1580-L1649
[rp-487]: https://github.com/RocketPy-Team/RocketPy/blob/v1.13.0/rocketpy/motors/solid_motor.py#L487-L632
[rp-724]: https://github.com/RocketPy-Team/RocketPy/blob/v1.13.0/rocketpy/motors/solid_motor.py#L724-L740
[rp-784]: https://github.com/RocketPy-Team/RocketPy/blob/v1.13.0/rocketpy/motors/solid_motor.py#L784-L789
[rp-585]: https://github.com/RocketPy-Team/RocketPy/blob/v1.13.0/rocketpy/motors/motor.py#L585-L666
[rp-1759]: https://github.com/RocketPy-Team/RocketPy/blob/v1.13.0/rocketpy/motors/motor.py#L1759-L1761
[rp-1173]: https://github.com/RocketPy-Team/RocketPy/blob/v1.13.0/rocketpy/motors/motor.py#L1173-L1191
[rp-flight]: https://github.com/RocketPy-Team/RocketPy/blob/v1.13.0/rocketpy/simulation/flight.py#L1936-L1956
[adr-021]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0021-whole-flights-against-rocketpy-what-is-compared.md
