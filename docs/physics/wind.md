# Wind

Set the steady wind a flight sees at each height: constant, grown from a single measured wind, or
read from a forecast's table of levels.

## In short

- **What it models:** the steady wind's speed and direction at each height. The wind can be
  constant, or grow with height from one measured wind (a power or log law). It can also come
  from a table of levels, such as a forecast.
- **Sources:** the US military flying-qualities specification MIL-F-8785C (1980), NASA's climatic
  criteria for aerospace vehicles NASA/TM-2008-215633 (2008), and the World Meteorological
  Organization's observing guide WMO-No. 8 (2023).
- **How well it is validated:** tests with exact answers pin each model. By default a table
  interpolates speed and direction separately. Halfway between 4 m/s (13 ft/s) from 350° and 12 m/s (39 ft/s) from 30°, it
  gives 8 m/s (26 ft/s) from 10°. In [RocketPy](../glossary.md#rocketpy)'s parachute descents,
  HPR Sim's wind is interpolated by components, as RocketPy does. It matches RocketPy's samples
  to 1e-9 m/s (in [scientific notation](../glossary.md#scientific-notation): a
  billionth of a meter per second). That includes the wind of NDRT 2020, one of the
  [example rockets](../glossary.md#example-rockets), which changes with height. In the four
  descents with wind, the total [drift](../glossary.md#drift) agrees within 0.28%, and each east or
  north part within 2.9% ([Recovery](recovery.md#against-rocketpy)). Ten real flights have been
  flown in ERA5 winds, nine of them in their own day's. Only their heights are compared, not their drift
  ([Accuracy: real flights](../accuracy.md#real-flights)). A wind read back from a GPS-tracked
  descent is pinned on synthetic descents. It is set beside ERA5 on five public flights
  ([Wind from a GPS-tracked descent](#wind-from-a-gps-tracked-descent)).
- **What it leaves out:** wind varying in time or across the field, vertical wind, terrain, and
  gusts (no flight uses [Turbulence](turbulence.md)). A flight takes one wind model, so a power or
  log law, which keeps growing with height, can't hand over to winds aloft.

## Sources and conventions

Where the code and its sources are, and the conventions every wind model follows.

### Code and sources

Code: [`hpr_atmos::wind`](../api/hpr_atmos/wind/index.html) in the API reference. Random gusts on
top of the steady wind are in [Turbulence](turbulence.md). The example program under
[An example of each](#an-example-of-each) builds every model.

Sources. [Checking a claim](../checking-a-claim.md#the-trail) explains how to fetch and check
each one. Each is pinned under the id given, in the [reference lock file][lock], which records
where to download it and its SHA-256 hash:

- **[8785C]** MIL-F-8785C, *Flying Qualities of Piloted Airplanes* (1980), §3.7.3.2, pinned as
  `mil-f-8785c`.
- **[TM]** NASA/TM-2008-215633, *Terrestrial Environment (Climatic) Criteria Guidelines for Use
  in Aerospace Vehicle Development* (2008), §2.2.5.2 and Table 2-21, `nasa-tm-2008-215633`.
- **[WMO]** WMO-No. 8 (2023), Vol. I, chapter 5 and its annex, `wmo-no8-vol1-2023`.

### Conventions

- A wind model returns the velocity of the air, in m/s, along the east, north and up
  axes of the [launch frame](../glossary.md#launch-frame-enu). The flight engine's airspeed is the
  rocket's velocity minus this. Mean wind (the steady wind, without gusts) is horizontal, so its up
  part is zero.
- The direction is [meteorological](../glossary.md#wind-direction): the direction the wind blows
  **from**, clockwise from true north, in radians. For a wind of speed $V$ from direction $\theta$, the
  east and north parts are $v_E = -V \sin\theta$ and $v_N = -V \cos\theta$. A wind from the west ($3\pi/2$,
  270°) blows toward the east, $+E$. RocketPy's *heading* is the direction the wind blows toward,
  $\theta + \pi$.
- A flight asks every model for the wind at a height above mean sea level, as it asks
  the [atmosphere](atmosphere.md#height-datum). Some models are built from heights above the
  ground instead: [Which height?](#which-height) says which.
- Each answer a wind model gives carries a flag, which is set when the height is outside what
  the model covers. That is below a table's lowest level or above its highest, or
  below the ground for a power or log law.
  - The model still answers: a table holds its end level's wind, and a law gives calm below the
    ground. The flag says that the answer was filled in, not taken from the model's data.
  - In code it is the answer's `extrapolated` field: `None` normally, and `Some(Side::Below)` or
    `Some(Side::Above)` when flagged.
  - The power and log laws are not flagged high above the ground, although they describe only
    the air near it.
  - **A flight doesn't read the flag yet** ([issue #45](https://github.com/nrdptel/fusionspace-eridanus/issues/45)).
    A rocket that climbs above a table's top level flies on the held wind, and nothing in its
    result says so. Ask the model yourself before flying a table, as
    [the example](#an-example-of-each) does: it marks each flagged answer with a star.

## Wind models

The wind laws, which height each input is measured from, and an example program that builds each
one.

### Models

In the laws below, $V$ is the wind speed at height $z$ above the ground. $V_{\mathrm{ref}}$ is the speed
measured at a reference height $z_{\mathrm{ref}}$ above the ground, such as the 10 m (33 ft) mast of a weather
station. Each law blows from one direction at every height.

- **`ConstantWind`:** one velocity at every height.
- **`PowerLawWind`:** the speed grows as a power of height, $V = V_{\mathrm{ref}}\, (z/z_{\mathrm{ref}})^\alpha$ for $z > 0$,
  and is zero at and below the ground. A height below the ground is [flagged](#conventions).
  - $\alpha$ is the exponent: the larger it is, the faster the wind grows with height.
  - [TM] eq. 2.1 gives the law for peak winds below 150 m (492 ft), with $z_{\mathrm{ref}} = 18.3\,\mathrm{m}$. A peak wind is
    the strongest speed over a period, gusts included, not the steady mean wind the simulator flies.
  - [TM] Table 2-1: $\alpha = 0.2$ for 7–22 m/s (23–72 ft/s) and 0.14 above 22 m/s (72 ft/s). Eq. 2.22 gives $1/7$ with
    $z_{\mathrm{ref}} = 10\,\mathrm{m}$ for strong 10 m (33 ft) winds.
  - Those exponents describe profiles of peak winds, and of strong 10 m (33 ft) winds. No single value
    fits mean winds everywhere, so treat any exponent as a starting point, not a measurement of
    your field.
  - [The example](#an-example-of-each) uses $\alpha = 1/7$ (about 0.14) with a 10 m (33 ft) reference height,
    the pairing of [TM] eq. 2.22. Its *power law* column shows what that does to a 5.0 m/s (16 ft/s) wind at
    10 m (33 ft). The wind is 4.0 m/s (13 ft/s) at 2 m (6.6 ft) and 6.9 m/s (23 ft/s) at 100 m (328 ft). At 2,000 m (6,562 ft) it is
    10.7 m/s (35 ft/s), and still growing.
  - The law describes the surface layer, the air nearest the ground. There friction with the
    ground sets how fast the wind grows with height ([TM] fits it below 150 m (492 ft)). Above that layer the
    law keeps growing, so pair it with measured winds aloft.
- **`LogLawWind`:** the speed grows with the logarithm of height,

  $$
  V = V_{\mathrm{ref}}\, \frac{\ln(z/z_0)}{\ln(z_{\mathrm{ref}}/z_0)}
  $$

  for $z > z_0$, and is zero from the ground up to $z_0$.
  - $z_0$ is the roughness length: the height at which the law's wind falls to zero. It is set by
    how rough the ground is (see the values below).
  - This is the neutral surface-layer law $V = (u_*/\kappa) \ln(z/z_0)$ ([WMO] ch. 5 annex; [TM] eq.
    2.8 with $\Psi = 0$), written through a reference wind. There $u_*$ is the friction velocity, a
    speed that measures how hard the wind drags on the ground. The von Kármán constant $\kappa$
    is a fixed number of the theory. Writing the law through the wind measured at $z_{\mathrm{ref}}$
    cancels both, so the simulator needs neither.
  - $\Psi$ in [TM] eq. 2.8 corrects for the air's stability. Air warmed from below mixes more, and
    air cooled from below mixes less, which changes how the wind grows with height. $\Psi = 0$ is
    neutral air, where neither happens. The simulator has only the neutral law.
  - [8785C] §3.7.3.2 uses it with $z_{\mathrm{ref}} = 20\,\mathrm{ft}$.
  - [WMO] gives a roughness length of 0.03 m (0.098 ft) for open flat terrain with grass. That is class 3 of the
    Davenport-Wieringa classification there, which gives a roughness length for each kind of
    terrain. [TM] Table 2-21 gives 0.001–0.01 m (0.0033–0.033 ft) for mown grass and 0.01–0.04 m (0.033–0.13 ft) for low grass or
    steppe.
- **`LayeredWind`:** speed and direction tabulated at heights, as from a
  [sounding](../glossary.md#sounding) or a forecast. Between levels it interpolates in one of two
  ways, set by `WindInterpolation`.
  - `SpeedDirection` (the default) interpolates speed linearly and turns the direction along
    the shorter arc. Between exactly opposite directions it turns clockwise with height. A
    turning wind keeps its speed.
  - With `SpeedDirection`, a calm level (speed 0) takes the other level's direction, because
    reports give calm as "0 from 0°". Without that rule a wind growing out of calm would swing
    through a meaningless direction and create a crosswind neither level has.
  - A near-calm level that isn't exactly 0 still turns the direction
    ([issue #8](https://github.com/nrdptel/fusionspace-eridanus/issues/8)). Take 0.1 m/s (0.33 ft/s) from 0° under 10 m/s (33 ft/s)
    from 270°. At the midpoint that gives (3.57, −3.57) m/s: 3.57 m/s (11.7 ft/s) blowing south. The levels
    blow 0.1 m/s (0.33 ft/s) and nothing that way. Where
    the other levels share one direction the swing likely shortens the drift. Where they turn it
    can lengthen it. A flight through such a span warns of it
    ([the flight-path warnings](../VALIDATION.md#the-flight-path-warnings)). The warning is for a
    level at or below 1.5 m/s (4.9 ft/s) turning by any angle against its neighbor. 1.5 m/s (4.9 ft/s) is the top of
    Beaufort force 1, light air. Force 1 spans 0.3 to 1.5 m/s (0.98 to 4.9 ft/s) in
    [MeteoSwiss's Beaufort table](https://www.meteoswiss.admin.ch/dam/jcr:c1064242-4a6e-4df2-8d21-96e68d1def7a/Beaufort_Tabelle_en.pdf),
    and 1 to 3 knots in the [Met Office's scale](https://weather.metoffice.gov.uk/guides/coast-and-sea/beaufort-scale).
    Light air's direction is "shown by smoke drift, but not by wind vanes" in the
    [Hong Kong Observatory's land criteria](https://my.weather.gov.hk/en/education/weather/wind-and-pressure/00112a-Beaufort_Land_Sea_Criterion.html).
  - `Components` interpolates the east and north parts linearly, as RocketPy does. Between levels
    90° apart the speed dips by up to 29%.
  - Beyond the end levels the end wind is held and the sample is flagged.
  - Put the surface wind (for example the 10 m (33 ft) observation) in as the lowest level, so the
    profile blends up from it.
- **`WindModel`:** any one of the four, as a flight or a file takes it. In JSON its `model` field
  names which one ([In JSON](#in-json)).

Loft, the project before FusionSpace HPR, got this wrong
([Loft lesson L6](../decisions-and-roadmap.md#l6)). Its design-file runs used one wind vector, and
its forecast profiles stepped at the lowest level instead of blending from the surface.

### Which height?

A flight asks the wind model for the wind at the rocket's
[height above mean sea level](../glossary.md#height-above-sea-level-msl). The laws are written in
height above the ground, so they take the ground's height above sea level as well. Each input's
name says which height it is: `_msl_` is above mean sea level, and `_agl_` is above the ground
([AGL](../glossary.md#agl-above-ground-level)).

| Model | Heights you give it | Measured from |
|---|---|---|
| `ConstantWind` | none: the same wind at every height | n/a |
| `PowerLawWind` | the reference height, `reference_height_agl_m` | the ground |
| | the ground's height, `ground_msl_m` | sea level |
| `LogLawWind` | the reference height, `reference_height_agl_m`, and the roughness length, `roughness_length_m` | the ground |
| | the ground's height, `ground_msl_m` | sea level |
| `LayeredWind` | each level's `height_msl_m` | sea level |
| A sounding's wind (`SoundingProfile`) | each level's `height_msl_m` | sea level |

So a table's levels, from a forecast or a sounding, are **above sea level**. At a site 1,400 m (4,593 ft)
above sea level, a forecast's 10 m (33 ft) wind goes in at 1,410 m (4,626 ft). Entered at 10 m (33 ft) instead, the whole
profile sits 1,400 m (4,593 ft) too low. The [example](#an-example-of-each)'s last column shows what the
flight would then see. Near the pad it sees 11.6 m/s (38 ft/s) from 298° instead of 5.0 m/s (16 ft/s) from 270°.
From 100 m (328 ft) up it sees the top level's 12.0 m/s (39 ft/s) from 300°. Above that, every sample is
[flagged](#conventions) as beyond the table, which is the sign to look for.

**What to enter for your field's elevation.** The launch site's height is the third number in
`Geodetic::from_degrees(latitude, longitude, height)`. It is an
[ellipsoidal height](../glossary.md#ellipsoidal-height): height above the
[WGS 84](../glossary.md#wgs-84) ellipsoid, the smooth shape the simulator gives the Earth. A field's
elevation, as a map gives it, is height above sea level instead. The two differ by the geoid
undulation $N$, the height of sea level above the ellipsoid at that place. $N$ can be up to
about 100 m (328 ft). The simulator has no map of $N$, so there are two ways to set up a site:

- **The simple way, which the examples take.** Enter your field's elevation above sea level as
  the site's height, and leave $N$ at 0, as `Environment::standard` sets it. The air and the wind
  are then looked up at the right heights above sea level. What is off is the height above the
  ellipsoid, by your field's $N$. A flight uses that only to work out gravity, which changes by
  about 0.003% over 100 m (328 ft) of height.
- **The exact way, if you know $N$ at your field.** Enter the elevation plus $N$ as the site's
  height, and give the environment $N$ with `with_geoid_undulation_m`
  ([`Environment`](../api/hpr_sim/environment/struct.Environment.html)).

Either way, the ground is the site's `height_m` less the environment's `geoid_undulation_m` above
sea level, which is how the example works it out. A power or log law needs that number as its
`ground_msl_m`, and doesn't take it from the site by itself. If its `ground_msl_m` is wrong, its
whole profile is shifted up or down by the error.

Three more things to watch:

- Soundings and forecasts often give heights in geopotential meters above sea level, not
  geometric meters (the meters a tape measure would give).
  - A geopotential meter measures height by the work done lifting a mass against gravity. It is
    the climb that takes as much work as one meter does where gravity is 9.80665 m/s².
  - Gravity varies with height and latitude, so the two differ by an amount that depends on
    both. 30 km (19 mi) above sea level is 29.7785 km (18.504 mi) of geopotential at the equator and 29.932 km (18.60 mi) at
    80° N.
  - Convert them first with
    [`geometric_from_wmo_geopotential_m`](../api/hpr_atmos/profile/fn.geometric_from_wmo_geopotential_m.html)
    ([Atmosphere](atmosphere.md#sounding-and-forecast-profiles) explains).
- A sounding's wind is a `LayeredWind` (`SoundingProfile::wind`). A flight flies it only if
  you also pass it to `with_wind`: the atmosphere and the wind are separate parts of
  `Environment`.
- A table flags a height below or above its levels, and a law flags one below the ground.
  [Conventions](#conventions) says how to read a flag. A flight doesn't report flags yet
  ([issue #45](https://github.com/nrdptel/fusionspace-eridanus/issues/45)), so ask the model yourself before
  flying a table, as the example does.

### An example of each

The example program
[`wind_profiles.rs`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-sim/examples/wind_profiles.rs)
builds each model for the launch site of [Getting started](../getting-started.md), 1,400 m (4,593 ft) above
sea level. It prints the wind each one gives at a few heights above the ground, and flies nothing.
Run it from anywhere in the repository:

```bash
cargo run --example wind_profiles -p fusionspace-hpr-sim
```

It prints this:

<!-- quote: crates/hpr-sim/examples/wind_profiles.output.txt -->
```text
Wind at a site 1400 m above sea level: speed in m/s, from a direction in °

above ground (m)      constant      power law        log law        layered    wrong datum
               2  5.0 from 270   4.0 from 270   3.6 from 270   5.0 from 270* 11.6 from 298
              10  5.0 from 270   5.0 from 270   5.0 from 270   5.0 from 270  11.6 from 298
              50  5.0 from 270   6.3 from 270   6.4 from 270   5.2 from 271  11.8 from 299
             100  5.0 from 270   6.9 from 270   7.0 from 270   5.6 from 272  12.0 from 300
             500  5.0 from 270   8.7 from 270   8.4 from 270   8.0 from 280  12.0 from 300*
            1000  5.0 from 270   9.7 from 270   9.0 from 270  10.0 from 290  12.0 from 300*
            2000  5.0 from 270  10.7 from 270   9.6 from 270  12.0 from 300* 12.0 from 300*

* beyond the table's levels: the end level's wind, held, and the sample flagged

{
  "model": "layered",
  "levels": [
    {
      "height_msl_m": 1410.0,
      "speed_m_s": 5.0,
      "direction_from_rad": 4.71238898038469
    },
    {
      "height_msl_m": 1900.0,
      "speed_m_s": 8.0,
      "direction_from_rad": 4.886921905584122
    },
    {
      "height_msl_m": 2900.0,
      "speed_m_s": 12.0,
      "direction_from_rad": 5.235987755982989
    }
  ],
  "interpolation": "speed_direction"
}
```

Reading it:

- Each law passes through its reference wind, 5.0 m/s (16 ft/s) at 10 m (33 ft), and blows from 270° at every
  height.
- The power and log laws agree near their reference height and part away from it. At 2 m (6.6 ft)
  they give 4.0 against 3.6 m/s (12 ft/s). At 2,000 m (6,562 ft) they give 10.7 against 9.6 m/s (31 ft/s). Both keep growing far above the surface
  layer they describe, which is why winds aloft should come from a table.
- The layered wind blends up from 5.0 m/s (16 ft/s) from 270° at 10 m (33 ft), turning with height. At 500 m (1,640 ft) it
  reaches 8.0 m/s (26 ft/s) from 280°. At 2 m (6.6 ft), below its lowest level, it holds the 10 m (33 ft) wind. At 2,000 m (6,562 ft), above
  its top level, it holds 12.0 m/s (39 ft/s) from 300°. The star marks both as flagged.
- The *wrong datum* column is the same table with its heights entered above the ground by
  mistake ([Which height?](#which-height)).
- The JSON at the end is the layered wind as a file holds it ([In JSON](#in-json)).

To fly one of them, give it to the flight's environment, as the program does for each column:
`Environment::standard(site)?.with_wind(wind)`. Then build the `Simulation` from that environment,
as [Getting started](../getting-started.md) does with its constant wind.

The program, which CI compiles and runs on macOS, Windows and Linux:

<!-- quote: crates/hpr-sim/examples/wind_profiles.rs -->
```rust
//! Wind profiles: each kind of steady wind hpr can fly, built for a launch site 1,400 m above sea
//! level, with the wind each one gives at a few heights above the ground, and how a flight takes
//! one. Nothing is flown.
//!
//! Run it from anywhere in the repository:
//!
//! ```text
//! cargo run --example wind_profiles -p fusionspace-hpr-sim
//! ```
//!
//! The documentation site's *Wind* page (`docs/physics/wind.md`) walks through it. What it prints
//! is kept next to it in `wind_profiles.output.txt`, and CI checks that the two still agree
//! (`cargo xtask examples --check`).

#![allow(
    clippy::print_stdout,
    reason = "the project's lints forbid printing in library code, and this program exists to print"
)]

use std::error::Error;

use hpr_atmos::{
    ConstantWind, LayeredWind, LogLawWind, PowerLawWind, WindInterpolation, WindLevel, WindModel,
};
use hpr_core::DVec3;
use hpr_core::geodesy::Geodetic;
use hpr_sim::Environment;

fn main() -> Result<(), Box<dyn Error>> {
    // The launch site of Getting started: New Mexico, 1,400 m up.
    let site = Geodetic::from_degrees(32.99, -106.97, 1400.0)?;
    let calm = Environment::standard(site)?; // no wind until it is given one

    // A flight asks for the wind at its height above mean sea level: its height above the WGS 84
    // ellipsoid, which is how hpr takes the site's height too, less the geoid undulation N (0
    // unless you set it). So the ground is this high above sea level:
    let ground_msl_m = site.height_m - calm.geoid_undulation_m;
    let from_west = 270_f64.to_radians();

    // 1. The same wind at every height: 5 m/s from the west. It takes no height.
    let constant = ConstantWind::new(5.0, from_west)?;

    // 2. A power law through 5 m/s at 10 m above the ground, with exponent α = 1/7. The
    //    reference height is above the ground; the ground's height is above sea level.
    let power_law = PowerLawWind::new(5.0, 10.0, 1.0 / 7.0, from_west, ground_msl_m)?;

    // 3. A log law through the same 5 m/s at 10 m, over grass: roughness length z₀ = 0.03 m.
    let log_law = LogLawWind::new(5.0, 10.0, 0.03, from_west, ground_msl_m)?;

    // 4. A table of levels, as from a forecast. Its heights are above sea level, so a level given
    //    above the ground goes in at the ground's height plus its own. The 10 m wind is the
    //    lowest level, so the profile blends up from it.
    let level = |height_agl_m: f64, speed_m_s: f64, from_deg: f64| WindLevel {
        height_msl_m: ground_msl_m + height_agl_m,
        speed_m_s,
        direction_from_rad: from_deg.to_radians(),
    };
    let forecast = vec![
        level(10.0, 5.0, 270.0),
        level(500.0, 8.0, 280.0),
        level(1500.0, 12.0, 300.0),
    ];
    let layered = LayeredWind::new(forecast.clone(), WindInterpolation::SpeedDirection)?;

    // The same table with its heights entered above the ground by mistake: 1,400 m too low.
    let wrong_levels = forecast
        .iter()
        .map(|level| WindLevel {
            height_msl_m: level.height_msl_m - ground_msl_m,
            ..*level
        })
        .collect();
    let wrong_datum = LayeredWind::new(wrong_levels, WindInterpolation::SpeedDirection)?;

    // Any of them goes to a flight the same way, through the flight's environment. Each
    // environment here is the one a `Simulation` from this site would fly in.
    let columns = [
        ("constant", WindModel::Constant(constant)),
        ("power law", WindModel::PowerLaw(power_law)),
        ("log law", WindModel::LogLaw(log_law)),
        ("layered", WindModel::Layered(layered.clone())),
        ("wrong datum", WindModel::Layered(wrong_datum)),
    ];
    let mut environments = Vec::new();
    for (_, wind) in &columns {
        environments.push(Environment::standard(site)?.with_wind(wind.clone()));
    }

    println!(
        "Wind at a site {ground_msl_m:.0} m above sea level: speed in m/s, from a direction in °"
    );
    println!();
    let mut header = "above ground (m)".to_owned();
    for (name, _) in &columns {
        header += &format!("{name:>14} ");
    }
    println!("{}", header.trim_end());
    for height_agl_m in [2.0, 10.0, 50.0, 100.0, 500.0, 1000.0, 2000.0_f64] {
        let mut row = format!("{height_agl_m:>16.0}");
        for environment in &environments {
            // What the flight engine asks for: the wind at a height above sea level.
            let sample = environment.wind.wind(ground_msl_m + height_agl_m)?;
            let (speed_m_s, from_deg) = speed_and_direction_from(sample.velocity_enu_m_s);
            let cell = format!("{speed_m_s:.1} from {from_deg:.0}");
            // A star marks a height beyond a table's levels, where the model holds the end
            // level's wind and flags the sample.
            let flag = if sample.extrapolated.is_some() {
                "*"
            } else {
                " "
            };
            row += &format!("{cell:>14}{flag}");
        }
        println!("{}", row.trim_end());
    }
    println!();
    println!("* beyond the table's levels: the end level's wind, held, and the sample flagged");
    println!();

    // The table as JSON, tagged by `model`. Directions are in radians.
    println!(
        "{}",
        serde_json::to_string_pretty(&WindModel::Layered(layered))?
    );
    Ok(())
}

/// The speed of a wind velocity (east, north, up), m/s, and the direction it blows from,
/// clockwise from north, rounded to a whole degree in `[0, 360)`.
fn speed_and_direction_from(velocity_enu_m_s: DVec3) -> (f64, f64) {
    let (east, north) = (velocity_enu_m_s.x, velocity_enu_m_s.y);
    let from_deg = (-east).atan2(-north).to_degrees().rem_euclid(360.0);
    // `% 360.0` turns a 359.6° into 0°, and `+ 0.0` turns a -0 into 0.
    (east.hypot(north), from_deg.round() % 360.0 + 0.0)
}
```

## Wind from a GPS-tracked descent

A rocket under a parachute drifts with the wind, a little late. So its GPS track measures the
wind at the flight's own place and minutes. This section gives how HPR Sim reads that wind from a
log, and how it compares with ERA5, the [reanalysis](../glossary.md#era5) the real flights fly in.
A *fix* is one position the receiver reported.

> **How far to trust it.** The wind it reads is a measurement's reconstruction. On synthetic
> descents built on the method's own assumptions, whose wind is known, it is within 0.3 m/s
> (0.7 mph) RMS. On real descents nothing bounds what the canopy itself does (below). Five public
> flights are compared with ERA5, but one of them gives three fixes in four. Read the differences
> as five examples of a reconstructed wind beside ERA5's, not as ERA5's measured miss. For ERA5's
> own miss, rely on the soundings below.

### The method

A body under a parachute takes up a change in the wind with a lag: its descent rate over gravity.
Under a 6 m/s (20 ft/s) main the lag is 0.6 s; under a 25 m/s (82 ft/s) drogue it is 2.5 s. The
wind is therefore the ground velocity plus the lag times the ground velocity's rate of change.
This is the dropsonde's equation, from Hock and Franklin (1999, the appendix, after Lally and
Leviton 1958; Kepert 2008, eq. 4):

$$
\mathbf{u} = \mathbf{v} + \frac{w}{g}\,\frac{\mathrm{d}\mathbf{v}}{\mathrm{d}t}
$$

with $\mathbf{u}$ the horizontal wind, $\mathbf{v}$ the horizontal ground velocity, $w$ the
descent rate and $g$ gravity. It assumes a steady fall, much faster than the body slips sideways.
It also assumes the same drag in every direction
([research note](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/wind-from-descent.md),
§1 and §2).

At each fix, a quadratic in time is fitted to the positions of the fixes within 10 s around it.
Its slope is the velocity, and twice its curvature the acceleration. A fix is read only where its
window holds at least five fixes and reaches 2.5 s to each side, so a fix beside a gap or at the
track's end is not read. It is
read only where the body falls steadily: at 2 to 35 m/s (7 to 115 ft/s), with a vertical
acceleration of at most 0.5 m/s². That drops the fall from apogee before a canopy holds, each
canopy's opening and the landing. Code:
[`hpr_flightdata::descent_wind`](../api/hpr_flightdata/descent_wind/index.html).

**A worked example.** Take a wind that strengthens by 2 m/s (4.5 mph) for every 100 m (328 ft) of
height. A body falling through it at 25 m/s (82 ft/s) meets 0.02 × 25 = 0.5 m/s less wind each
second. Its lag is 25 / 9.807 = 2.55 s, so its ground velocity runs 2.55 × 0.5 = 1.27 m/s
(2.8 mph) above the wind it is in. Under a 5 m/s (16 ft/s) main, the same wind puts the ground
velocity only 0.05 m/s (0.1 mph) off the wind.

On a real flight, Juno III fell under its drogue at 24.071 m/s (79 ft/s). This is one fix from
the report's data file (`descent-wind.json`, Juno III at 124.4 s).
At 929 m (3,048 ft) above the pad, its lag was 24.071 / 9.807 = 2.455 s. Its ground velocity was
13.033 m/s (29.2 mph) east and 5.153 m/s (11.5 mph) north. Each second it slowed by 0.552 m/s
(1.2 mph) east and 0.166 m/s (0.4 mph) north. The rate of change is negative, so the correction
lowers the ground velocity, and the wind was:

- east: 13.033 − 2.455 × 0.552 = 11.678 m/s;
- north: 5.153 − 2.455 × 0.166 = 4.745 m/s.

That is 12.605 m/s (28.2 mph), against a ground speed of 14.01 m/s (31.3 mph). ERA5's wind there
was 14.74 m/s (33.0 mph).

### Against ERA5

The [descent-wind report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/descent-wind.md)
reads every public flight HPR Sim flies or predicts, and RocketPy's other example logs: the seven
the real-flights report flies, the five predicted first, and Camões, Halcyon, Hedy and Valkyrie.
It also reads, or tries, the three public team logs with GPS fixes in the descent that a search
of public repositories found: Hyperion-II's (a TeleMega) and Itzamna's (UTRGV's, a Featherweight
GPS) in the [public-flights note](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/public-flights.md),
and RIT's Testbed 2 (a Featherweight GPS) in the
[smoother-truth note](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/smoother-truth-flights.md).
A log not published, or not found, is not here.

- **Compared:** Juno III, under its drogue from 3,210 m (10,531 ft) down to 421 m (1,381 ft) above
  the pad. Genesis, under its main below 666 m (2,185 ft); its receiver once went 79 s without a
  new fix. Camões, under its drogue from 2,677 m (8,783 ft) down. Halcyon, under its main from
  3,336 m (10,945 ft) down, at 1,951 fixes. Itzamna, under its drogue and main from 2,731 m
  (8,960 ft) down to 24 m (79 ft). Its GPS log dates the launch 30 March, but its Blue Raven
  altimeter's summary gives 29 March. ERA5's pressure at the pad's height agrees with the 29th:
  it is 20 Pa from the 97,119 Pa the team's flight computer logged on the pad, against 406.5 Pa on
  the 30th. The comparison uses the 29th (the report's note on the flight).
- **Reconstructed, not compared:** Prometheus, whose ERA5 file is of the year after the flight.
  Hedy, whose ERA5 file HPR Sim can't read without a checked conversion. Testbed 2, whose ERA5
  file is not fetched yet; its date, 16 May 2026, is its GPS log's alone, as its Blue Raven's
  clock was not set.
- **Not run:** Cavour fell at 75 m/s (246 ft/s), Astra at 98 m/s (321 ft/s) and Hyperion-II at
  85 m/s (279 ft/s), faster than any canopy the method is held to: Hyperion-II's drogue never
  slowed it. Eight more logged no position.

Over the five compared flights and 2,558 fixes, the reconstructed speed is 0.81 m/s (1.8 mph)
below ERA5's on average. Root-mean-square, it is 3.49 m/s (7.8 mph) from it. Each flight alone
differs on average by 0.6 to 1.5 m/s (1.4 to 3.4 mph): above ERA5's on four, below it on
Halcyon. Halcyon gives three fixes in four, so the pooled mean is mostly its own. ERA5 misses
soundings by 1.4 to 1.9 m/s (3.1 to 4.3 mph) RMS in speed through the lowest 3 km (1.9 mi)
(Taszarek et al. 2021, in the research note's §5). The pooled RMS here is 1.8 to 2.5 times that.
Nothing here tells ERA5's miss at a launch field apart from the canopy's.

Prometheus carried two GPS receivers, whose highest fixes are 1 m (3.3 ft) apart in height and whose last
fixes are 2.4 m (8 ft) apart. Read apart, their winds differ by 0.51 m/s (1.1 mph) RMS over 170
shared fixes, and 1.35 m/s (3.0 mph) at most. That is a floor on the noise between receivers. It
can't show the lag model's error or a motion both shared, which cancel in the difference.

### What it leaves unbounded

- **Glide.** A canopy that glides at an angle $\gamma$ moves across the wind by $w\tan\gamma$.
  Model canopies at 10 ft/s (3.0 m/s) glided at up to 45° (Knacke, NWC TP 6575, p. 5-16): a
  speed as large as the descent itself. A turning canopy partly averages its glide out; how much
  is left, no source measures for hobby canopies.
- **Swing.** A rocket swinging by $\theta$ on a shock cord of length $L$ moves at up to
  $\theta\sqrt{gL}$. That is about 0.95 m/s (2.1 mph) at 10° on 3 m (10 ft). On a synthetic
  swing that size, with a 2 to 5 s period, the 10 s window leaves under 0.15 m/s (0.3 mph) RMS.
  A slower swing is not tested.
- **Drogues and tumbling bodies.** The lag assumes the same drag in every direction; a tumbling
  airframe or a fluttering streamer has no measured response.
- **ERA5's hour and place.** ERA5 is read at the launch hour and site; the descent is minutes
  later and away from the site.

### Sources

- Hock, T. F., and Franklin, J. L. (1999). The NCAR GPS dropwindsonde. *Bulletin of the American
  Meteorological Society* 80, pp. 407–420
  ([DOI](https://doi.org/10.1175/1520-0477%281999%29080%3C0407%3ATNGD%3E2.0.CO%3B2)).
- Kepert, J. D. (2008). Interpreting dropsonde measurements of turbulence in the tropical cyclone
  boundary layer. AMS 28th Conference on Hurricanes and Tropical Meteorology, paper P1D.7, eq. 4.
- Taszarek, M., and others (2021). Comparison of convective parameters derived from ERA5 and
  MERRA-2 with rawinsonde data over Europe and North America. *Journal of Climate* 34,
  pp. 3211–3237, doi:10.1175/JCLI-D-20-0484.1.
- Knacke, T. W. (1991). *Parachute Recovery Systems Design Manual*, NWC TP 6575, p. 5-16.

## Input and limits

How a wind model is written in JSON, and what is not modeled yet.

### In JSON

`WindModel` reads and writes JSON as one object. Its `model` field names the kind of wind, and the
other fields are that model's inputs:

| `model` | Other fields |
|---|---|
| `constant` | `speed_m_s`, `direction_from_rad` |
| `power_law` | `reference_speed_m_s`, `reference_height_agl_m`, `exponent`, `direction_from_rad`, `ground_msl_m` |
| `log_law` | `reference_speed_m_s`, `reference_height_agl_m`, `roughness_length_m`, `direction_from_rad`, `ground_msl_m` |
| `layered` | `levels`, each with `height_msl_m`, `speed_m_s` and `direction_from_rad`; and `interpolation`, `speed_direction` (the default if left out) or `components` |

- Directions are in radians, the direction the wind blows from: 270° is 4.71238898038469, as the
  example's JSON shows.
- Reading a file checks it as the model's constructor does: a negative speed, or a field the model
  doesn't have, is an error.

### Not yet modeled

- Wind that changes with time, or across the field.
- Vertical mean wind.
- Terrain effects.
- A blend below the lowest tabulated level (the table holds the lowest level's wind).
- Reporting during a flight that the wind was held beyond a table's levels.

A flight takes one wind model, so it can't join a power or log law near the ground to a table of
levels aloft. How to join them isn't decided yet
([issue #890](https://github.com/nrdptel/fusionspace-eridanus/issues/890)).

## Tests that pin this

- `wind::tests::layered_wind_interpolates_speed_and_heading` ([Loft lesson L6](../decisions-and-roadmap.md#l6), the
  forecast profile that stepped at its lowest level):
  - Halfway between 4 m/s (13 ft/s) from 350° and 12 m/s (39 ft/s) from 30°, the wind is 8 m/s (26 ft/s) from 10°. It turns
    through north.
  - There is no step just above the surface level.
- `components_interpolation_averages_the_vectors`, `opposite_directions_turn_clockwise`
  and `wind_grows_out_of_calm_without_turning`.
- `layered_wind_holds_and_flags_beyond_its_levels`.
- `meteorological_direction_convention`, plus the power and log laws through their
  references, below ground, and at $z_0$.
- `invalid_inputs_are_rejected`, and `wind_models_round_trip_through_json`: each model
  written to JSON and read back gives the same wind.
- Against RocketPy, in the recovery test `descent_matches_rocketpy_examples`
  ([Recovery](recovery.md#against-rocketpy)):
  - At every height its parachute descents sample, HPR Sim's wind matches RocketPy's within 1e-9 m/s
    in its east and north parts.
  - Four of the five descents fly in wind: Calisto, NDRT 2020, Prometheus and Juno III. Their total
    drift agrees within 0.28%, and each east or north part within 2.9%. The largest gap is the
    north part of NDRT 2020's drift, +2.865% (the
    [validation report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/latest.md)).
- The example above, which CI runs and compares with its committed output.
- The wind from a GPS-tracked descent, in `descent_wind::tests`:
  - `the_lagged_reading_recovers_the_wind_and_the_ground_velocity_misses_it_under_a_drogue`. A
    body with quadratic drag falls at 5 m/s (16 ft/s), 25 m/s (82 ft/s) and 33 m/s (108 ft/s), sampled once a
    second. Its wind strengthens by 2 m/s (4.5 mph) every 100 m (328 ft). The method recovers it
    within 0.3 m/s (0.7 mph) RMS at each. The ground velocity alone misses it by the worked
    example's lag, within 10%: past 0.3 m/s (0.7 mph) at the two faster descents.
  - `the_lagged_reading_is_near_exact_on_a_noiseless_track`: within 0.05 m/s (0.1 mph).
  - `the_lagged_reading_holds_the_line_with_receiver_noise`. Each fix gets 0.5 m (1.6 ft) of noise
    east and north and 1 m (3.3 ft) up; the method holds 0.3 m/s (0.7 mph) on each of 200 seeds.
  - `the_window_cuts_a_swing_on_the_shock_cord`: a 0.95 m/s (2.1 mph) swing of 2 to 5 s period,
    a fix a second, leaves under 0.15 m/s (0.3 mph) RMS at 5 m/s (16 ft/s) and 25 m/s (82 ft/s).
  - `the_steady_fall_rule_drops_an_opening_and_the_landing`, `a_fix_beside_a_gap_is_not_read`,
    and the refusals of a bad track or setting.
- `cargo xtask descent-wind --check`, in the gate's `refs` step, reproduces the report from the
  logs. In CI, `the_descent_wind_report_holds_together` recomputes its bands from each flight's
  committed sums and its page from its data, and checks the page's ERA5 credit.

[lock]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/refs.lock.toml
