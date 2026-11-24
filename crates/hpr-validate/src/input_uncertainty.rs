//! The input part of a real flight's validation uncertainty: how far its apogee moves over the
//! stated uncertainties of its inputs ([M1.14b18][m1-14b18]; [the decision][adr-376]).
//!
//! ASME V&V 20 gives two ways to an input part `u_input`: locally, from each input's sensitivity
//! times its standard uncertainty, added in squares; and globally, by drawing the inputs from
//! their distributions and taking the spread of the result (Yeo, NISTIR 8298, 2020, pp. 18 to 19;
//! `docs/research/validation-uncertainty-vv20.md`). The report does both and prints them side by
//! side, as the standard's own example does. `u_input` is the global one: the sample standard
//! deviation of the apogee over [`SAMPLES`] flights, each with every input drawn from a normal
//! distribution about the value the report flies. The local one flies each input alone one
//! standard uncertainty either way, and says which input the spread comes from.
//!
//! The draws are Latin hypercubes, one of the two samplings the summary names (p. 19): for each
//! input the normal distribution is cut into as many strata of equal probability as there are
//! flights, each flight takes one stratum of each input, and the strata are paired across
//! inputs at random ([`deviates`]). Every part of each input's distribution is then flown once,
//! where plain random draws leave the spread of 200 flights known to about 5%. Two hypercubes
//! of [`SAMPLES_EACH`] flights are flown on two seeds, and the report prints each one's spread
//! beside the spread of both together: how far the two stand apart is how well the sample
//! knows its own answer.
//!
//! Five inputs are drawn ([`Stated`], one row a flight in [`INPUTS`]):
//!
//! | Input | What a draw changes |
//! |---|---|
//! | dry mass | every stage's mass without motors times `1 + σ z`, its inertia with it (`hpr_analysis::montecarlo::dry_mass_scaled`) |
//! | total impulse | every motor's thrust and propellant mass times `1 + σ z` (`hpr_analysis::montecarlo::dispersed_motor`) |
//! | wind | `σ z` added to the wind at every height, on each of the east and north axes |
//! | rail angle | the rail turned by `σ z` toward its heading's vertical plane and by `σ z` across it |
//! | air temperature | the whole column `σ z` warmer, its pressure on the pad kept ([`warmed`]) |
//!
//! The air is drawn once and used twice: the flight is flown in the warmed air, and the
//! altimeter that reads HPR Sim's apogee reads the same warmed air
//! ([`InputDraw::launch_day`]), since a barometric reading moves by about 0.35% for each kelvin
//! of the column while the height climbed moves the other way and by less ([the decision on the
//! log's part][adr-375], decision 10). The drag is not an input: it is the model under test.
//!
//! [m1-14b18]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m1-14b18
//! [adr-375]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0375-m1-14b17-the-logs-part-of-a-real-flights-validation-uncertainty.md
//! [adr-376]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0376-m1-14b18-the-inputs-part-and-the-band.md

use std::sync::Arc;

use hpr_analysis::AnalysisError;
use hpr_analysis::montecarlo::{dispersed_motor, dry_mass_scaled};
use hpr_analysis::statistics::normal_quantile;
use hpr_atmos::profile::{geometric_from_wmo_geopotential_m, wmo_geopotential_from_geometric_m};
use hpr_atmos::wind::velocity_from_speed_direction;
use hpr_atmos::{
    AtmosError, Atmosphere, ConstantWind, LayeredWind, SoundingLevel, SoundingProfile, Ussa76,
    WindInterpolation,
};
use hpr_core::random::SeededRng;
use hpr_design::Rocket;
use hpr_sim::{Environment, Rail};
use serde::{Deserialize, Serialize};

/// The flights of one Latin hypercube.
pub const SAMPLES_EACH: u64 = 100;

/// The hypercubes flown, each on a seed of its own: [`SEED`], then the next whole numbers.
pub const HYPERCUBES: u64 = 2;

/// The flights each public flight is flown for its input part, over all its hypercubes.
pub const SAMPLES: u64 = HYPERCUBES * SAMPLES_EACH;

/// The seed of the first hypercube. A flight's draws depend on the seeds and the flight's
/// stated inputs alone ([`draws`]), so the report reproduces whatever the number of threads.
pub const SEED: u64 = 376;

/// One input's standard uncertainty for a flight, and where it is stated.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stated {
    /// The standard uncertainty, in the input's own unit: a fraction for the mass and the
    /// impulse, m/s on each axis for the wind, radians on each axis for the rail, kelvins for
    /// the air.
    pub sigma: f64,
    /// What kind of figure the flight's own input is, in words: the reason it takes this row.
    pub basis: &'static str,
    /// Where the standard uncertainty is stated.
    pub from: &'static str,
}

/// A team's stated mass, taken as weighed.
const MASS_STATED: Stated = Stated {
    sigma: 0.005,
    basis: "the team's stated mass, taken as weighed",
    from: "docs/research/dispersion-sigmas.md, §3: a Student Launch team disperses its own \
           rocket's mass by 0.5% (NC State, 2023 flight readiness review, p. 53), assumed; \
           hobby sources assume 0% to 3.9%, and no notebook here says its mass was weighed",
};

/// A design file's mass, with no record of a weighing.
const MASS_DESIGN_FILE: Stated = Stated {
    sigma: 0.1075,
    basis: "a design file's mass, with no record that the rocket was weighed as flown",
    from: "docs/research/as-built-mass-growth.md: over 24 rockets with both a design-file mass \
           and a scale mass, the scale's stands −1.4% to +13.1% from the file's at the \
           quartiles, a standard deviation of 10.75% for a normal spread (the range over 1.349)",
};

/// A commercial motor's impulse about its file's.
const IMPULSE_COMMERCIAL: Stated = Stated {
    sigma: 0.025,
    basis: "a commercial motor on its maker's or certifier's curve",
    from: "docs/research/hpr-lot-impulse.md, §2: 21 AeroTech H to M motors that Tripoli Motor \
           Testing certified twice, in 2001 to 2004, differ by a standard deviation of 3.56 \
           points between the two tests, so one test stands 3.56/√2 = 2.5% from the motor's \
           mean; with a renamed 22nd motor, 3.1%",
};

/// The other end of the range the commercial motors' source gives: the 22 pairs with the
/// renamed motor, 4.41 points over √2. The report says what the band's count would be there.
pub const IMPULSE_OTHER_END: f64 = 0.031;

/// A curve that isn't a certifier's, on the commercial motors' figure.
const IMPULSE_BORROWED: Stated = Stated {
    sigma: 0.025,
    basis: "a curve of the team's own or of no stated origin: no spread was found for one, so \
            the commercial motors' is borrowed",
    from: IMPULSE_COMMERCIAL.from,
};

/// The reanalysis's wind at the flight's hour.
const WIND_REANALYSIS: Stated = Stated {
    sigma: 2.0,
    basis: "ERA5's wind at the flight's hour",
    from: "docs/research/era5-wind-direction-us.md, §6: about 2 m/s on each axis by day (1.8 \
           to 2.4), inferred from forecast models held to soundings and lidars; above 3.2 m/s \
           on the worst nights",
};

/// A wind that isn't the day's: anything a launch is allowed in.
const WIND_NOT_THE_DAYS: Stated = Stated {
    sigma: 4.47,
    basis: "not the day's wind",
    from: "the high-power safety code's limit of 20 mph (8.94 m/s) for a launch (NFPA 1127, as \
           docs/VALIDATION.md cites it), read here as a bound on a wind from any direction: \
           spread evenly over that disc, 8.94/2 on each axis",
};

/// The rail's angle about its stated setting, on each of two axes.
const RAIL: Stated = Stated {
    // One degree.
    sigma: std::f64::consts::PI / 180.0,
    basis: "the rail's stated angle",
    from: "docs/research/dispersion-sigmas.md, §5: 1° in five of the seven hobby sources that \
           disperse a launch angle, RocketPy's notebooks among them; each assumes it and none \
           measured it",
};

/// The reanalysis's air at the flight's hour.
const AIR_REANALYSIS: Stated = Stated {
    sigma: 0.9,
    basis: "ERA5's air at the flight's hour",
    from: "docs/research/era5-surface-temperature.md, §1: ERA5's mean temperature from the pad \
           to 1 to 3 km stands 0.9 to 1.35 K from soundings it never took in (Huang et al. \
           2021, Tazhong; this project's average of their levels), the lower end taken; at \
           stations it did take in, one level stands 0.72 to 0.85 K off (Taszarek et al. 2021, \
           Table 2)",
};

/// Air that isn't the day's: the standard atmosphere, or another year's reanalysis.
const AIR_NOT_THE_DAYS: Stated = Stated {
    // 15/√3.
    sigma: 8.660_254_037_844_386,
    basis: "not the day's air",
    from: "docs/physics/synthetic-flights.md, \"What a flight draws\": a day's air taken as \
           within 15 K of the 1976 standard atmosphere's, this project's own range with no \
           outside source; as bounds, 15/√3 = 8.66 K",
};

/// One public flight's five stated inputs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FlightInputs {
    /// The flight's id.
    pub id: &'static str,
    /// The mass without motors, as a fraction of it.
    pub dry_mass: Stated,
    /// Each motor's total impulse, as a fraction of it.
    pub impulse: Stated,
    /// The wind, m/s on each of the east and north axes, the same at every height.
    pub wind: Stated,
    /// The rail's direction, rad on each of two axes: toward the vertical plane of its heading,
    /// and across it.
    pub rail: Stated,
    /// The air column's temperature, K, the same at every height.
    pub air: Stated,
}

/// A RocketPy example flown in its day's ERA5 file on a commercial motor's curve.
const fn example(id: &'static str) -> FlightInputs {
    FlightInputs {
        id,
        dry_mass: MASS_STATED,
        impulse: IMPULSE_COMMERCIAL,
        wind: WIND_REANALYSIS,
        rail: RAIL,
        air: AIR_REANALYSIS,
    }
}

/// A design file's flight, flown in the standard atmosphere and its stored simulation's wind.
const fn design_file(id: &'static str) -> FlightInputs {
    FlightInputs {
        id,
        dry_mass: MASS_DESIGN_FILE,
        impulse: IMPULSE_COMMERCIAL,
        wind: WIND_NOT_THE_DAYS,
        rail: RAIL,
        air: AIR_NOT_THE_DAYS,
    }
}

/// Each public flight's stated inputs, in the report's order. A flight's row follows what the
/// report says of its inputs: Prometheus is flown in a weather file of a year after its flight
/// (its note in the report), Juno III and Andromeda on their teams' own thrust files, Erebus 11
/// on a file whose origin its notebook doesn't give, and the two Macie XL flights from a design
/// file in the standard atmosphere.
pub const INPUTS: [FlightInputs; 12] = [
    example("bella-lui"),
    example("ndrt-2020"),
    FlightInputs {
        wind: WIND_NOT_THE_DAYS,
        air: AIR_NOT_THE_DAYS,
        ..example("prometheus-2022")
    },
    FlightInputs {
        impulse: IMPULSE_BORROWED,
        ..example("juno-iii")
    },
    example("cavour"),
    example("genesis"),
    example("lince"),
    example("astra"),
    FlightInputs {
        impulse: IMPULSE_BORROWED,
        ..example("andromeda")
    },
    FlightInputs {
        impulse: IMPULSE_BORROWED,
        ..example("erebus-11")
    },
    design_file("macie-xl-1"),
    design_file("macie-xl-2"),
];

impl FlightInputs {
    /// The five inputs, each under the name the report lists it by, with the drawn number that
    /// stands for it (the first of two, for the wind and the rail).
    #[must_use]
    pub fn each(&self) -> [(&'static str, Drawn, Stated); 5] {
        [
            ("mass", Drawn::DryMass, self.dry_mass),
            ("impulse", Drawn::Impulse, self.impulse),
            ("wind", Drawn::WindEast, self.wind),
            ("rail", Drawn::RailUp, self.rail),
            ("air", Drawn::Air, self.air),
        ]
    }
}

/// The stated inputs of the flight `id`.
#[must_use]
pub fn stated(id: &str) -> Option<&'static FlightInputs> {
    INPUTS.iter().find(|inputs| inputs.id == id)
}

/// The seven numbers a draw holds, in the order the report lists them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Drawn {
    /// The mass without motors.
    DryMass,
    /// The motors' total impulse.
    Impulse,
    /// The wind's east component.
    WindEast,
    /// The wind's north component.
    WindNorth,
    /// The rail turned in the vertical plane of its heading.
    RailUp,
    /// The rail turned across that plane.
    RailAcross,
    /// The air column's temperature.
    Air,
}

impl Drawn {
    /// Every drawn number, in the report's order.
    pub const ALL: [Self; 7] = [
        Self::DryMass,
        Self::Impulse,
        Self::WindEast,
        Self::WindNorth,
        Self::RailUp,
        Self::RailAcross,
        Self::Air,
    ];

    /// Its name in the report.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::DryMass => "mass",
            Self::Impulse => "impulse",
            Self::WindEast => "wind east",
            Self::WindNorth => "wind north",
            Self::RailUp => "rail up",
            Self::RailAcross => "rail across",
            Self::Air => "air",
        }
    }

    /// Its standard uncertainty among a flight's stated inputs.
    #[must_use]
    pub fn sigma(self, inputs: &FlightInputs) -> f64 {
        match self {
            Self::DryMass => inputs.dry_mass.sigma,
            Self::Impulse => inputs.impulse.sigma,
            Self::WindEast | Self::WindNorth => inputs.wind.sigma,
            Self::RailUp | Self::RailAcross => inputs.rail.sigma,
            Self::Air => inputs.air.sigma,
        }
    }

    /// How the report writes one of its standard uncertainties: a percentage for the mass and
    /// the impulse, m/s for the wind, degrees for the rail, kelvins for the air.
    #[must_use]
    pub fn written(self, sigma: f64) -> String {
        match self {
            Self::DryMass | Self::Impulse => format!("{:.2}%", 100.0 * sigma),
            Self::WindEast | Self::WindNorth => format!("{sigma:.2} m/s on each axis"),
            Self::RailUp | Self::RailAcross => {
                format!("{:.2}° on each axis", sigma.to_degrees())
            }
            Self::Air => format!("{sigma:.2} K"),
        }
    }

    /// Its stream's key among a sample's draws: fixed numbers, so that adding an input later
    /// leaves the others' draws as they were.
    fn key(self) -> u64 {
        match self {
            Self::DryMass => 1,
            Self::Impulse => 2,
            Self::WindEast => 3,
            Self::WindNorth => 4,
            Self::RailUp => 5,
            Self::RailAcross => 6,
            Self::Air => 7,
        }
    }
}

/// One flight's inputs moved from the values the report flies: what a sample of the dispersion,
/// or one input moved alone, flies.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct InputDraw {
    /// Every stage's mass without motors, over the report's.
    pub dry_mass_scale: f64,
    /// Every motor's total impulse, over the report's.
    pub impulse_scale: f64,
    /// Added to the wind's east component at every height, m/s.
    pub wind_east_m_s: f64,
    /// Added to the wind's north component at every height, m/s.
    pub wind_north_m_s: f64,
    /// The rail turned upward in the vertical plane of its heading, rad: toward the vertical
    /// for a rail short of it.
    pub rail_up_rad: f64,
    /// The rail turned across that plane, rad: to the right, seen from behind the rail.
    pub rail_across_rad: f64,
    /// Added to the air's temperature at every height, K.
    pub air_k: f64,
}

impl InputDraw {
    /// Nothing moved: the report's own flight.
    pub const NONE: Self = Self {
        dry_mass_scale: 1.0,
        impulse_scale: 1.0,
        wind_east_m_s: 0.0,
        wind_north_m_s: 0.0,
        rail_up_rad: 0.0,
        rail_across_rad: 0.0,
        air_k: 0.0,
    };

    /// `which` alone moved `deviations` standard uncertainties of `inputs`, the rest as the
    /// report flies them.
    #[must_use]
    pub fn one(which: Drawn, deviations: f64, inputs: &FlightInputs) -> Self {
        Self::from_deviates(
            Drawn::ALL.map(|each| if each == which { deviations } else { 0.0 }),
            inputs,
        )
    }

    /// The draw whose seven numbers stand `z` standard uncertainties of `inputs` from the
    /// report's values, in [`Drawn::ALL`]'s order.
    #[must_use]
    pub fn from_deviates(z: [f64; 7], inputs: &FlightInputs) -> Self {
        let [mass, impulse, east, north, up, across, air] = z;
        Self {
            dry_mass_scale: 1.0 + mass * inputs.dry_mass.sigma,
            impulse_scale: 1.0 + impulse * inputs.impulse.sigma,
            wind_east_m_s: east * inputs.wind.sigma,
            wind_north_m_s: north * inputs.wind.sigma,
            rail_up_rad: up * inputs.rail.sigma,
            rail_across_rad: across * inputs.rail.sigma,
            air_k: air * inputs.air.sigma,
        }
    }

    /// `rocket` with the draw's mass and impulse, or `None` when it moves neither: the rocket
    /// as given.
    ///
    /// # Errors
    ///
    /// A scale that isn't a positive number, a design that can't be laid out, or a motor whose
    /// propellant can't be scaled.
    pub fn rocket(&self, rocket: &Rocket) -> Result<Option<Rocket>, AnalysisError> {
        if self.dry_mass_scale == 1.0 && self.impulse_scale == 1.0 {
            return Ok(None);
        }
        let mut rocket = if self.dry_mass_scale == 1.0 {
            rocket.clone()
        } else {
            dry_mass_scaled(rocket, self.dry_mass_scale)?
        };
        if self.impulse_scale != 1.0 {
            for configuration in &mut rocket.configurations {
                for mounted in &mut configuration.motors {
                    mounted.motor = dispersed_motor(&mounted.motor, self.impulse_scale, 1.0)?;
                }
            }
        }
        Ok(Some(rocket))
    }

    /// `rail` turned by the draw's two angles ([`turned`]).
    #[must_use]
    pub fn rail(&self, rail: Rail) -> Rail {
        turned(rail, self.rail_up_rad, self.rail_across_rad)
    }

    /// A sounding's day with the draw's air and wind: the environment a flight is flown in and
    /// the air its altimeter reads, both from the one warmed profile ([`warmed`]), the wind's
    /// two components added at every level. A draw that moves neither gives both back as they
    /// are.
    ///
    /// # Errors
    ///
    /// [`AtmosError`] if the warmed profile has no air, or its levels no longer rise.
    pub fn launch_day(
        &self,
        environment: Environment,
        air: SoundingProfile,
        pad_msl_m: f64,
    ) -> Result<(Environment, SoundingProfile), AtmosError> {
        if self.air_k == 0.0 && self.wind_east_m_s == 0.0 && self.wind_north_m_s == 0.0 {
            return Ok((environment, air));
        }
        let drawn = warmed(
            &air,
            pad_msl_m,
            self.air_k,
            (self.wind_east_m_s, self.wind_north_m_s),
        )?;
        let environment = match drawn.wind() {
            Some(wind) => Environment {
                atmosphere: Arc::new(drawn.clone()),
                wind: Arc::new(wind.clone()),
                ..environment
            },
            None => Environment {
                atmosphere: Arc::new(drawn.clone()),
                ..environment
            },
        };
        Ok((environment, drawn))
    }

    /// The standard atmosphere's day with the draw's air and wind, for a flight flown in the
    /// 1976 standard and a steady wind: the standard offset by the draw's kelvins at every
    /// height with its pressure on the pad kept ([`Ussa76::anchored`]), and the steady wind
    /// with the draw's two components added.
    ///
    /// # Errors
    ///
    /// [`AtmosError`] if the pad or the offset is out of the standard's range.
    pub fn standard_day(
        &self,
        pad_msl_m: f64,
        wind: ConstantWind,
    ) -> Result<(Ussa76, ConstantWind), AtmosError> {
        let standard = Ussa76::standard();
        let air = if self.air_k == 0.0 {
            standard
        } else {
            let pad = standard.air(pad_msl_m)?.air;
            Ussa76::anchored(pad_msl_m, pad.temperature_k + self.air_k, pad.pressure_pa)?
        };
        let wind = if self.wind_east_m_s == 0.0 && self.wind_north_m_s == 0.0 {
            wind
        } else {
            let (speed_m_s, from_rad) = wind_plus(
                wind.speed_m_s(),
                wind.direction_from_rad(),
                (self.wind_east_m_s, self.wind_north_m_s),
            );
            ConstantWind::new(speed_m_s, from_rad)?
        };
        Ok((air, wind))
    }
}

/// A wind of `speed_m_s` from `from_rad` with `(east, north)` m/s added to its velocity: its new
/// speed and the direction it blows from, clockwise from north.
fn wind_plus(speed_m_s: f64, from_rad: f64, (east_m_s, north_m_s): (f64, f64)) -> (f64, f64) {
    let velocity = velocity_from_speed_direction(speed_m_s, from_rad);
    let (east, north) = (velocity.x + east_m_s, velocity.y + north_m_s);
    // The air moves toward (east, north), so it blows from the opposite point.
    (east.hypot(north), (-east).atan2(-north))
}

/// `rail` turned by `up_rad` in the vertical plane of its heading (toward the vertical, for a
/// rail short of it) and by `across_rad` across that plane (to the right, seen from behind).
///
/// The rail points along `d = (cos E sin A, cos E cos A, sin E)` in east, north and up, for an
/// elevation `E` and a heading `A` clockwise from north. The two turns are small rotations about
/// two axes square to `d` and to each other, `e_up = ∂d/∂E` and the horizontal `e_across`. They
/// are applied as one rotation by `θ = √(up² + across²)` toward `(up·e_up + across·e_across)/θ`:
/// `d′ = d cos θ + ê sin θ`, from which the new elevation and heading are read. A vertical rail
/// has its two axes from its stated heading, so it leans along a plane and across it like any
/// other. The roll follows the heading, `φ′ = φ + (A′ − A)`, which keeps the rocket's own roll
/// attitude where a small turn of a near-vertical rail swings the heading far. No turn gives the
/// rail back as it is.
#[must_use]
pub fn turned(rail: Rail, up_rad: f64, across_rad: f64) -> Rail {
    let theta = up_rad.hypot(across_rad);
    if theta == 0.0 {
        return rail;
    }
    let (sin_e, cos_e) = rail.elevation_rad.sin_cos();
    let (sin_a, cos_a) = rail.azimuth_rad.sin_cos();
    let d = [cos_e * sin_a, cos_e * cos_a, sin_e];
    let e_up = [-sin_e * sin_a, -sin_e * cos_a, cos_e];
    let e_across = [cos_a, -sin_a, 0.0];
    let (sin_t, cos_t) = theta.sin_cos();
    let moved: [f64; 3] = std::array::from_fn(|i| {
        d[i] * cos_t + (up_rad * e_up[i] + across_rad * e_across[i]) / theta * sin_t
    });
    let elevation_rad = moved[2].clamp(-1.0, 1.0).asin();
    // Straight up has no heading of its own: the stated one stays.
    let azimuth_rad = if moved[0] == 0.0 && moved[1] == 0.0 {
        rail.azimuth_rad
    } else {
        moved[0].atan2(moved[1])
    };
    Rail {
        azimuth_rad,
        elevation_rad,
        roll_rad: rail.roll_rad + (azimuth_rad - rail.azimuth_rad),
        ..rail
    }
}

/// The log mean of two positive temperatures, `(b − a)/ln(b/a)`: the temperature whose
/// isothermal layer is as thick as one whose temperature runs linearly from `a` to `b` in
/// geopotential height (the hypsometric equation, WMO-No. 8 (2023), Vol. I, eqs. 12.17 and
/// 12.18, as `hpr_atmos::SoundingProfile` fills a pressure).
fn log_mean(a: f64, b: f64) -> f64 {
    let r = (b - a) / a;
    if r == 0.0 { a } else { r * a / r.ln_1p() }
}

/// `profile` with its whole column `delta_k` warmer, the pressure on a pad `pad_msl_m` above
/// sea level kept, and `(east, north)` m/s added to its wind at every level.
///
/// A sounding or a reanalysis gives a temperature and a height at each pressure. If the day's
/// air is `ΔT` warmer than the file says at every pressure, each layer between two pressures is
/// thicker by the ratio of its mean temperatures, warmed over as given (the hypsometric
/// equation: a layer's geopotential thickness is `(R_d/g₀) T̄ ln(p₁/p₂)`, with `T̄` the
/// logarithmic mean of its two temperatures). So every level keeps its pressure, takes `T + ΔT`,
/// and moves to the height that stacks the thicker layers up from the pad, where the pressure
/// is the one thing a reading is zeroed on. The pad's own temperature is the profile's there.
/// Heights are stacked in WMO geopotential at the profile's latitude and turned back to
/// geometric ones.
///
/// What it leaves out: the pad's pressure is kept only to second order in `ΔT/T`, since the pad
/// lies inside a layer whose interpolation isn't exactly a stack of two; on a 300 K day that is
/// under a pascal for each kelvin. A real error isn't the same at every height.
///
/// With no warming and no wind added it gives the profile back as it is.
///
/// # Errors
///
/// [`AtmosError`] if the pad has no air in the profile, a warmed temperature isn't positive, or
/// the warmed levels no longer rise.
pub fn warmed(
    profile: &SoundingProfile,
    pad_msl_m: f64,
    delta_k: f64,
    (east_m_s, north_m_s): (f64, f64),
) -> Result<SoundingProfile, AtmosError> {
    let no_wind = east_m_s == 0.0 && north_m_s == 0.0;
    if delta_k == 0.0 && no_wind {
        return Ok(profile.clone());
    }
    let latitude_rad = profile.latitude_rad();
    let levels = profile.levels();
    let pressures_pa = profile.pressures_pa();
    let mut heights_m: Vec<f64> = levels.iter().map(|level| level.height_msl_m).collect();
    if delta_k != 0.0 {
        let geopotential =
            |height_m: f64| wmo_geopotential_from_geometric_m(height_m, latitude_rad);
        let given = levels
            .iter()
            .map(|level| geopotential(level.height_msl_m))
            .collect::<Result<Vec<f64>, AtmosError>>()?;
        let pad = geopotential(pad_msl_m)?;
        let pad_k = profile.air(pad_msl_m)?.air.temperature_k;
        // A layer's thickness, warmed over as given.
        let thicker =
            |a_k: f64, b_k: f64| log_mean(a_k + delta_k, b_k + delta_k) / log_mean(a_k, b_k);
        let mut moved = given.clone();
        // Upward from the pad, then downward from it.
        let above = given.partition_point(|&z| z <= pad);
        let (mut below_z, mut below_k, mut below_moved) = (pad, pad_k, pad);
        for i in above..given.len() {
            let k = levels[i].temperature_k;
            below_moved += (given[i] - below_z) * thicker(below_k, k);
            moved[i] = below_moved;
            (below_z, below_k) = (given[i], k);
        }
        let (mut above_z, mut above_k, mut above_moved) = (pad, pad_k, pad);
        for i in (0..above).rev() {
            let k = levels[i].temperature_k;
            above_moved -= (above_z - given[i]) * thicker(k, above_k);
            moved[i] = above_moved;
            (above_z, above_k) = (given[i], k);
        }
        for (height_m, z) in heights_m.iter_mut().zip(moved) {
            *height_m = geometric_from_wmo_geopotential_m(z, latitude_rad)?;
        }
    }
    let levels = levels
        .iter()
        .zip(heights_m)
        .zip(pressures_pa)
        .map(|((level, height_msl_m), &pressure_pa)| {
            let wind = match (level.wind_speed_m_s, level.wind_direction_from_rad) {
                (Some(speed_m_s), Some(from_rad)) if !no_wind => {
                    let (speed_m_s, from_rad) =
                        wind_plus(speed_m_s, from_rad, (east_m_s, north_m_s));
                    (Some(speed_m_s), Some(from_rad))
                }
                given => given,
            };
            SoundingLevel {
                height_msl_m,
                temperature_k: level.temperature_k + delta_k,
                pressure_pa: Some(pressure_pa),
                relative_humidity: level.relative_humidity,
                wind_speed_m_s: wind.0,
                wind_direction_from_rad: wind.1,
            }
        })
        .collect();
    let interpolation = profile
        .wind()
        .map_or_else(WindInterpolation::default, LayeredWind::interpolation);
    SoundingProfile::new(levels, latitude_rad, interpolation)
}

/// The standard normal deviates of one Latin hypercube of `samples` flights seeded with
/// `seed`: one row a flight, its seven numbers in [`Drawn::ALL`]'s order.
///
/// For each number, the unit interval is cut into `samples` equal strata. A flight takes one
/// stratum and the standard normal deviate at its middle probability, the stratum's median
/// (`hpr_analysis::statistics::normal_quantile`). Which flight takes which
/// stratum is a random permutation, a new one for each number (a Fisher–Yates shuffle), so the
/// numbers are paired at random. This is the Latin hypercube sampling of McKay, Beckman and
/// Conover, "A comparison of three methods for selecting values of input variables in the
/// analysis of output from a computer code", *Technometrics* 21(2), 239–245 (1979), as the
/// summary of V&V 20 names it (Yeo, NISTIR 8298, p. 19); the construction here is the common
/// one, written from that description and not from the 1979 paper, which was not read.
///
/// The median stands in for a point drawn evenly inside the stratum, the usual choice. A drawn
/// point is unbiased, but the two end strata have no far edge, and one deep draw there moves a
/// hypercube's spread by several percent: over 20,000 hypercubes of 100, a number's standard
/// deviation scattered 1.3% about 1.005, and 1 in 120 stood over 1.05. On the medians every
/// hypercube's 100 deviates are the same set, mean 0 and standard deviation 0.99864: 0.14%
/// under the distribution's. On a flight that answers in a straight line that makes each
/// hypercube's spread 0.14% too small, and `u_input`, the spread of two hypercubes about one
/// mean, 0.39% too small. Only the pairing is left to the seed.
///
/// Each number's stream is keyed by the seed and the number alone
/// (`hpr_core::random::SeededRng::for_stream`), so adding a number later leaves the others'
/// deviates as they were.
#[must_use]
pub fn deviates(seed: u64, samples: u64) -> Vec<[f64; 7]> {
    let n = usize::try_from(samples).unwrap_or(usize::MAX);
    // A count of flights is far under 2^53.
    #[expect(
        clippy::cast_precision_loss,
        reason = "a count of flights is far under 2^53"
    )]
    let count = n as f64;
    let mut rows = vec![[0.0; 7]; n];
    for (column, which) in Drawn::ALL.iter().enumerate() {
        let mut rng = SeededRng::for_stream(seed, &[which.key()]);
        let mut strata: Vec<usize> = (0..n).collect();
        for last in (1..n).rev() {
            // An index evenly from 0 to `last`: a uniform deviate below 1 times `last + 1`,
            // floored, and so at most `last`.
            #[expect(
                clippy::cast_precision_loss,
                clippy::cast_possible_truncation,
                clippy::cast_sign_loss,
                reason = "a count of flights, and a product in [0, last + 1) floored"
            )]
            let other = ((rng.uniform() * (last + 1) as f64) as usize).min(last);
            strata.swap(last, other);
        }
        for (row, stratum) in rows.iter_mut().zip(strata) {
            #[expect(
                clippy::cast_precision_loss,
                reason = "a count of flights is far under 2^53"
            )]
            let lower = stratum as f64;
            row[column] = normal_quantile((lower + 0.5) / count);
        }
    }
    rows
}

/// What each sample of a flight's dispersion draws, in the order flown: [`HYPERCUBES`]
/// hypercubes of [`SAMPLES_EACH`] flights ([`deviates`]), the first on `seed` and each next one
/// on the next whole number, every deviate times its number's standard uncertainty among
/// `inputs`.
#[must_use]
pub fn draws(seed: u64, inputs: &FlightInputs) -> Vec<InputDraw> {
    (0..HYPERCUBES)
        .flat_map(|cube| deviates(seed.wrapping_add(cube), SAMPLES_EACH))
        .map(|z| InputDraw::from_deviates(z, inputs))
        .collect()
}

/// The mean and the sample standard deviation of `values`, the latter over `n − 1`: `None` with
/// fewer than two values or one that isn't finite. The sums run in the order given, so one
/// platform gives one answer.
#[must_use]
pub fn mean_and_deviation(values: &[f64]) -> Option<(f64, f64)> {
    if values.len() < 2 || values.iter().any(|value| !value.is_finite()) {
        return None;
    }
    // A count of flights is far under 2^53.
    #[expect(
        clippy::cast_precision_loss,
        reason = "a count of flights is far under 2^53"
    )]
    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;
    let squares: f64 = values
        .iter()
        .map(|value| (value - mean) * (value - mean))
        .sum();
    Some((mean, (squares / (n - 1.0)).sqrt()))
}

/// One drawn number flown alone, a standard uncertainty either way.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputComponent {
    /// Its name ([`Drawn::name`]).
    pub what: String,
    /// The standard uncertainty it was moved by, in its own unit: a fraction, m/s, rad or K.
    pub sigma: f64,
    /// The apogee with it one standard uncertainty below the report's value, m.
    pub low_m: f64,
    /// The apogee with it one standard uncertainty above, m.
    pub high_m: f64,
}

impl InputComponent {
    /// Half the difference of its two flights, m: the apogee's sensitivity to it times its
    /// standard uncertainty, by a central difference (Yeo, NISTIR 8298, p. 18, eq. 19), signed:
    /// positive where more of the input flies higher.
    #[must_use]
    pub fn half_difference_m(&self) -> f64 {
        0.5 * (self.high_m - self.low_m)
    }

    /// How far the mean of its two flights stands from the report's own `apogee_m`, m: what a
    /// straight line through the two leaves out. A rail turned either way from the vertical
    /// flies lower both times, and all of its effect is here.
    #[must_use]
    pub fn bend_m(&self, apogee_m: f64) -> f64 {
        0.5 * (self.high_m + self.low_m) - apogee_m
    }
}

/// The input part of one public real flight.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputPart {
    /// Each drawn number flown alone, in [`Drawn::ALL`]'s order.
    pub components: Vec<InputComponent>,
    /// The local method's part, m: the root sum of squares of the components' half differences.
    pub local_m: f64,
    /// The seed of the samples' draws.
    pub seed: u64,
    /// How many samples were flown, over all the hypercubes; every one flew.
    pub samples: u64,
    /// Each hypercube's own standard deviation, m, in the order flown.
    pub hypercubes_m: Vec<f64>,
    /// Each hypercube's own mean apogee, m, in the order flown.
    pub hypercube_means_m: Vec<f64>,
    /// The samples' mean apogee, m ([`together`]).
    pub mean_m: f64,
    /// `u_input`, m: the standard deviation of all the samples together ([`together`]).
    pub input_m: f64,
}

/// The mean and the standard deviation of all the samples of equal hypercubes of `each` flights,
/// from each hypercube's own mean and standard deviation: the mean of the means `m`, and
///
/// ```text
/// s² = [ (each − 1)·Σ s_c² + each·Σ (m_c − m)² ] / (N − 1),   N = each × the hypercubes
/// ```
///
/// which is the sum of squares about one mean split into the part inside each hypercube and
/// the part between them, over the degrees of freedom of all `N` samples. The report keeps each
/// hypercube's two figures and not its samples, so a reader can hold `u_input` to them. `None`
/// with no hypercube, with unequal lists, or with fewer than two flights in each.
#[must_use]
pub fn together(means_m: &[f64], deviations_m: &[f64], each: u64) -> Option<(f64, f64)> {
    if means_m.is_empty() || means_m.len() != deviations_m.len() || each < 2 {
        return None;
    }
    // Counts of flights are far under 2^53.
    #[expect(
        clippy::cast_precision_loss,
        reason = "counts of flights are far under 2^53"
    )]
    let (each, cubes) = (each as f64, means_m.len() as f64);
    let mean_m = means_m.iter().sum::<f64>() / cubes;
    let inside: f64 = deviations_m.iter().map(|s| s * s).sum();
    let between: f64 = means_m.iter().map(|m| (m - mean_m) * (m - mean_m)).sum();
    let squares = (each - 1.0) * inside + each * between;
    Some((mean_m, (squares / (each * cubes - 1.0)).sqrt()))
}

/// The root sum of squares of components' half differences, m.
#[must_use]
pub fn local_part_m(components: &[InputComponent]) -> f64 {
    components
        .iter()
        .map(|component| component.half_difference_m() * component.half_difference_m())
        .sum::<f64>()
        .sqrt()
}

/// A flight's input part from its flights: `alone`, each drawn number's two apogees (low, high)
/// in [`Drawn::ALL`]'s order, and `samples`, the dispersion's apogees in the order [`draws`]
/// gives them, one hypercube after another.
///
/// # Errors
///
/// In words: a count of samples that isn't a whole number of hypercubes of two flights or more,
/// or an apogee that isn't a finite number.
pub fn input_part(
    inputs: &FlightInputs,
    alone: [(f64, f64); 7],
    samples: &[f64],
) -> Result<InputPart, String> {
    let components: Vec<InputComponent> = Drawn::ALL
        .iter()
        .zip(alone)
        .map(|(which, (low_m, high_m))| InputComponent {
            what: which.name().to_owned(),
            sigma: which.sigma(inputs),
            low_m,
            high_m,
        })
        .collect();
    if components
        .iter()
        .any(|component| !(component.low_m.is_finite() && component.high_m.is_finite()))
    {
        return Err("an input moved alone flew to an apogee that isn't a number".to_owned());
    }
    let without = || {
        format!(
            "{} samples don't give a standard deviation for each of {HYPERCUBES} hypercubes: \
             too few, not a whole number of hypercubes, or one not finite",
            samples.len()
        )
    };
    let cubes = usize::try_from(HYPERCUBES).unwrap_or(usize::MAX);
    if samples.is_empty() || !samples.len().is_multiple_of(cubes) {
        return Err(without());
    }
    let each = samples.len() / cubes;
    let (hypercube_means_m, hypercubes_m): (Vec<f64>, Vec<f64>) = samples
        .chunks(each)
        .map(mean_and_deviation)
        .collect::<Option<Vec<(f64, f64)>>>()
        .ok_or_else(without)?
        .into_iter()
        .unzip();
    let (mean_m, input_m) = together(
        &hypercube_means_m,
        &hypercubes_m,
        u64::try_from(each).unwrap_or(u64::MAX),
    )
    .ok_or_else(without)?;
    if !(mean_m.is_finite() && input_m.is_finite()) {
        return Err(without());
    }
    Ok(InputPart {
        local_m: local_part_m(&components),
        components,
        seed: SEED,
        samples: u64::try_from(samples.len()).unwrap_or(u64::MAX),
        hypercubes_m,
        hypercube_means_m,
        mean_m,
        input_m,
    })
}

impl InputPart {
    /// Whether the part holds together for the flight `id`: its components are the flight's
    /// seven, each moved by its stated standard uncertainty, the local part is their root sum of
    /// squares, the samples are [`SAMPLES`] on [`SEED`] in [`HYPERCUBES`] hypercubes, the mean
    /// and `u_input` are the hypercubes' own means and spreads put together ([`together`]), and
    /// every number is finite and no spread negative. The samples themselves aren't kept: only
    /// the check that flies the flights again holds each hypercube's mean and spread.
    #[must_use]
    pub fn holds(&self, id: &str) -> bool {
        let Some(inputs) = stated(id) else {
            return false;
        };
        self.components.len() == Drawn::ALL.len()
            && self
                .components
                .iter()
                .zip(Drawn::ALL)
                .all(|(component, which)| {
                    component.what == which.name()
                        && component.sigma == which.sigma(inputs)
                        && component.low_m.is_finite()
                        && component.high_m.is_finite()
                })
            && self.local_m == local_part_m(&self.components)
            && self.seed == SEED
            && self.samples == SAMPLES
            && u64::try_from(self.hypercubes_m.len()) == Ok(HYPERCUBES)
            && self
                .hypercubes_m
                .iter()
                .all(|deviation_m| deviation_m.is_finite() && *deviation_m >= 0.0)
            && self
                .hypercube_means_m
                .iter()
                .all(|mean_m| mean_m.is_finite())
            && together(&self.hypercube_means_m, &self.hypercubes_m, SAMPLES_EACH)
                == Some((self.mean_m, self.input_m))
            && self.mean_m.is_finite()
            && self.input_m.is_finite()
            && self.input_m >= 0.0
    }
}

/// An input part for a test's row: Cavour's stated inputs (a RocketPy example's), each number
/// alone moving the apogee `halves_m` either side of 1000 m, both hypercubes' means at 1000 m,
/// and their spreads such that `u_input` is `input_m` to rounding. It holds for any flight with
/// those stated inputs.
#[cfg(test)]
pub(crate) fn part_with_for_tests(halves_m: [f64; 7], input_m: f64) -> InputPart {
    let components: Vec<InputComponent> = Drawn::ALL
        .iter()
        .zip(halves_m)
        .map(|(which, half_m)| InputComponent {
            what: which.name().to_owned(),
            sigma: which.sigma(&INPUTS[4]),
            low_m: 1000.0 - half_m,
            high_m: 1000.0 + half_m,
        })
        .collect();
    // Two hypercubes of one spread s about one mean have s·√(198/199) together.
    let hypercubes_m = vec![input_m * (199.0_f64 / 198.0).sqrt(); 2];
    let hypercube_means_m = vec![1000.0; 2];
    let (mean_m, input_m) = together(&hypercube_means_m, &hypercubes_m, SAMPLES_EACH)
        .expect("two hypercubes of a hundred");
    InputPart {
        local_m: local_part_m(&components),
        components,
        seed: SEED,
        samples: SAMPLES,
        hypercubes_m,
        hypercube_means_m,
        mean_m,
        input_m,
    }
}

/// [`part_with_for_tests`] with no number alone moving the apogee.
#[cfg(test)]
pub(crate) fn part_for_tests(input_m: f64) -> InputPart {
    part_with_for_tests([0.0; 7], input_m)
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    reason = "a test stops at the failure (clippy.toml)"
)]
mod tests {
    use std::f64::consts::SQRT_2;

    use hpr_atmos::{Barometer, Wind as _};

    use super::*;

    /// A spread worked by hand. The five apogees 1000, 1010, 990, 1020 and 980 m have a mean of
    /// 1000 m and squared distances 0, 100, 100, 400 and 400, which sum to 1000; over n − 1 = 4
    /// that is 250, and the standard deviation is √250 = 15.8114 m. The seven components'
    /// half differences 3, 4, 0, 0, 0, 0 and 12 m give a local part of √(9 + 16 + 144) = 13 m.
    #[test]
    fn the_input_part_on_a_case_worked_by_hand() {
        let samples = [1000.0, 1010.0, 990.0, 1020.0, 980.0];
        let (mean, deviation) = mean_and_deviation(&samples).unwrap();
        assert_eq!(mean, 1000.0);
        assert!((deviation * deviation - 250.0).abs() < 1e-9, "{deviation}");
        assert!((deviation - 15.811_388).abs() < 1e-6, "{deviation}");
        // The divisor is n − 1, not n: over n it would be √200 = 14.142 m, the smaller figure.
        assert!(deviation > 14.2);
        let inputs = stated("cavour").unwrap();
        let alone = [
            (997.0, 1003.0),
            (996.0, 1004.0),
            (1000.0, 1000.0),
            (1000.0, 1000.0),
            (999.0, 999.0),
            (1000.0, 1000.0),
            (1012.0, 988.0),
        ];
        // Two hypercubes of three flights: 1000, 1010 and 990 m (a mean of 1000 and squares of
        // 0, 100 and 100 over 2: 10 m), then 1020, 980 and 1000 m (400, 400 and 0 over 2: 20 m).
        // All six have a mean of 1000 m and squares summing to 1000 over 5: √200 = 14.142 m.
        let six = [1000.0, 1010.0, 990.0, 1020.0, 980.0, 1000.0];
        let part = input_part(inputs, alone, &six).unwrap();
        assert_eq!(part.local_m, 13.0);
        assert_eq!(part.mean_m, 1000.0);
        assert_eq!(part.hypercubes_m, [10.0, 20.0]);
        assert_eq!(part.hypercube_means_m, [1000.0, 1000.0]);
        assert!(
            (part.input_m * part.input_m - 200.0).abs() < 1e-9,
            "{}",
            part.input_m
        );
        assert_eq!(part.samples, 6);
        // Hypercubes about two means: 1000, 1010 and 990 m, then 1030, 1040 and 1020 m, have
        // means of 1000 and 1030 m and spreads of 10 m each. About the one mean of 1015 m the
        // six stand −15, −5, −25, +15, +25 and +5 m: squares of 1750 over 5, 350. Put together
        // from the hypercubes' figures: (2·(100 + 100) + 3·(225 + 225))/5 = 350, the same.
        let apart = [1000.0, 1010.0, 990.0, 1030.0, 1040.0, 1020.0];
        let part_apart = input_part(inputs, alone, &apart).unwrap();
        assert_eq!(part_apart.hypercube_means_m, [1000.0, 1030.0]);
        assert_eq!(part_apart.mean_m, 1015.0);
        assert!((part_apart.input_m * part_apart.input_m - 350.0).abs() < 1e-9);
        let (_, direct) = mean_and_deviation(&apart).unwrap();
        assert!((part_apart.input_m - direct).abs() < 1e-12);
        assert_eq!(together(&[], &[], 3), None);
        assert_eq!(together(&[1000.0], &[10.0, 20.0], 3), None);
        assert_eq!(together(&[1000.0], &[10.0], 1), None);
        // The air's component is signed: warmer air reads lower here.
        assert_eq!(part.components[6].half_difference_m(), -12.0);
        // A rail turned either way flies 1 m lower: no half difference, all bend.
        assert_eq!(part.components[4].half_difference_m(), 0.0);
        assert_eq!(part.components[4].bend_m(1000.0), -1.0);
        // Too few samples, an odd count for two hypercubes, or one that isn't a number, give
        // no part.
        assert!(input_part(inputs, alone, &[]).is_err());
        assert!(input_part(inputs, alone, &six[..2]).is_err());
        assert!(input_part(inputs, alone, &samples).is_err());
        assert!(input_part(inputs, alone, &[1000.0, 1001.0, 1002.0, f64::NAN]).is_err());
        let mut spoiled = alone;
        spoiled[2].0 = f64::INFINITY;
        assert!(input_part(inputs, spoiled, &six).is_err());
        assert_eq!(mean_and_deviation(&[5.0]), None);
        assert_eq!(mean_and_deviation(&[5.0, 5.0]), Some((5.0, 0.0)));
    }

    /// A part holds only as the flight's own: its seven components on the flight's stated
    /// standard uncertainties, their root sum of squares, and the report's count and seed.
    #[test]
    fn a_part_holds_only_as_its_flights_own() {
        let inputs = stated("macie-xl-1").unwrap();
        let alone = [(990.0, 1010.0); 7];
        let samples: Vec<f64> = (0..SAMPLES)
            .map(|i| if i % 2 == 0 { 1000.0 } else { 1003.0 })
            .collect();
        let part = input_part(inputs, alone, &samples).unwrap();
        assert!(part.holds("macie-xl-1"));
        // Another flight's inputs: a design file's mass isn't a team's.
        assert!(!part.holds("cavour"));
        assert!(!part.holds("no such flight"));
        let spoil = |change: &dyn Fn(&mut InputPart)| {
            let mut spoiled = part.clone();
            change(&mut spoiled);
            spoiled.holds("macie-xl-1")
        };
        assert!(!spoil(&|part| part.local_m *= 0.5));
        assert!(!spoil(&|part| part.components[1].sigma = 0.05));
        assert!(!spoil(&|part| part.components.swap(0, 1)));
        assert!(!spoil(&|part| {
            part.components.pop();
        }));
        assert!(!spoil(&|part| part.components[3].high_m += 1.0));
        assert!(!spoil(&|part| part.samples = 20));
        assert!(!spoil(&|part| {
            part.hypercubes_m.pop();
        }));
        assert!(!spoil(&|part| part.hypercubes_m[1] = f64::NAN));
        // u_input and the mean are the hypercubes' figures put together, to the bit.
        assert!(!spoil(&|part| part.input_m *= 1.001));
        assert!(!spoil(&|part| part.mean_m += 0.001));
        assert!(!spoil(&|part| part.hypercubes_m[0] *= 1.001));
        assert!(!spoil(&|part| part.hypercube_means_m[1] += 0.001));
        assert!(!spoil(&|part| {
            part.hypercube_means_m.pop();
        }));
        assert!(!spoil(&|part| part.seed += 1));
        assert!(!spoil(&|part| part.input_m = -1.0));
        assert!(!spoil(&|part| part.input_m = f64::NAN));
        assert!(!spoil(&|part| part.mean_m = f64::INFINITY));
    }

    /// Every public flight has its five inputs, each with a positive standard uncertainty and a
    /// source, and the classes are the report's: three flights on a curve that isn't a
    /// certifier's, three in air and wind that aren't their day's, two from a design file.
    #[test]
    fn every_public_flight_has_its_stated_inputs() {
        assert_eq!(INPUTS.len(), crate::data_uncertainty::INSTRUMENTS.len());
        for (inputs, instrument) in INPUTS.iter().zip(&crate::data_uncertainty::INSTRUMENTS) {
            assert_eq!(inputs.id, instrument.id, "the two tables keep one order");
            for which in Drawn::ALL {
                assert!(which.sigma(inputs) > 0.0, "{} {}", inputs.id, which.name());
            }
            for stated in [
                inputs.dry_mass,
                inputs.impulse,
                inputs.wind,
                inputs.rail,
                inputs.air,
            ] {
                assert!(!stated.basis.is_empty() && !stated.from.is_empty());
            }
        }
        let with = |pick: &dyn Fn(&FlightInputs) -> bool| -> Vec<&str> {
            INPUTS.iter().filter(|i| pick(i)).map(|i| i.id).collect()
        };
        assert_eq!(
            with(&|i| i.impulse == IMPULSE_BORROWED),
            ["juno-iii", "andromeda", "erebus-11"]
        );
        assert_eq!(
            with(&|i| i.air == AIR_NOT_THE_DAYS),
            ["prometheus-2022", "macie-xl-1", "macie-xl-2"]
        );
        assert_eq!(
            with(&|i| i.wind == WIND_NOT_THE_DAYS),
            ["prometheus-2022", "macie-xl-1", "macie-xl-2"]
        );
        assert_eq!(
            with(&|i| i.dry_mass == MASS_DESIGN_FILE),
            ["macie-xl-1", "macie-xl-2"]
        );
        // The figures the sources give: 14.5 points of interquartile range over 1.349, 3.56
        // points over √2, half the 20 mph limit, one degree, and 15 K as bounds.
        assert!((MASS_DESIGN_FILE.sigma - (0.131 + 0.014) / 1.349).abs() < 5e-5);
        assert!((IMPULSE_COMMERCIAL.sigma - 0.0356 / SQRT_2).abs() < 2e-4);
        assert!((WIND_NOT_THE_DAYS.sigma - 0.5 * 20.0 * 0.447_04).abs() < 1e-3);
        assert_eq!(RAIL.sigma.to_degrees(), 1.0);
        assert!((AIR_NOT_THE_DAYS.sigma - 15.0 / 3.0_f64.sqrt()).abs() < 1e-12);
    }

    /// A hypercube flies every stratum of every number once: sorted, a number's deviates fall
    /// one in each of the normal distribution's equal shares, at the share's median. So its
    /// mean is 0 and its spread 0.99864 on every seed, where 100 random draws' spread is known
    /// to 7%, and the numbers are paired at random, not in step.
    #[test]
    fn a_hypercube_flies_every_stratum_once() {
        let n = usize::try_from(SAMPLES_EACH).unwrap();
        let rows = deviates(SEED, SAMPLES_EACH);
        assert_eq!(rows.len(), n);
        for column in 0..7 {
            let mut sorted: Vec<f64> = rows.iter().map(|row| row[column]).collect();
            sorted.sort_by(f64::total_cmp);
            for (stratum, z) in sorted.iter().enumerate() {
                let (low, high) = (
                    normal_quantile(stratum as f64 / n as f64),
                    normal_quantile((stratum + 1) as f64 / n as f64),
                );
                assert!(low < *z && *z < high, "{z} in stratum {stratum}");
                // The median: as much of the share's probability under it as over it.
                assert_eq!(*z, normal_quantile((stratum as f64 + 0.5) / n as f64));
            }
            let (mean, deviation) = mean_and_deviation(&sorted).unwrap();
            assert!(mean.abs() < 1e-12, "{mean}");
            assert!((deviation - 0.998_64).abs() < 1e-5, "{deviation}");
        }
        // Two numbers' deviates are paired at random: their correlation over 100 flights is a
        // draw from about ±0.1, and none of the 21 pairs is in step.
        for a in 0..7 {
            for b in (a + 1)..7 {
                let product: f64 = rows.iter().map(|row| row[a] * row[b]).sum();
                assert!((product / n as f64).abs() < 0.35, "{a} with {b}: {product}");
            }
        }
        // The same seed gives the same hypercube; another seed, another.
        assert_eq!(rows, deviates(SEED, SAMPLES_EACH));
        assert_ne!(rows, deviates(SEED + 1, SAMPLES_EACH));
        assert!(deviates(SEED, 0).is_empty());
        assert_eq!(deviates(SEED, 1).len(), 1);
    }

    /// A flight's draws are its hypercubes' deviates on its own stated spreads, the same again
    /// for the same seed, and one number moved alone leaves the rest at the report's values.
    #[test]
    fn the_draws_follow_the_seed_and_the_stated_spreads() {
        let inputs = stated("prometheus-2022").unwrap();
        let drawn = draws(SEED, inputs);
        assert_eq!(drawn.len() as u64, SAMPLES);
        assert_eq!(drawn, draws(SEED, inputs));
        assert_ne!(drawn, draws(SEED + 7, inputs));
        // The second hypercube is the next seed's.
        let second = deviates(SEED + 1, SAMPLES_EACH);
        let each = usize::try_from(SAMPLES_EACH).unwrap();
        assert_eq!(drawn[each], InputDraw::from_deviates(second[0], inputs));
        // Another flight draws the same deviates on its own spreads.
        let other = draws(SEED, stated("cavour").unwrap());
        assert_eq!(drawn[3].dry_mass_scale, other[3].dry_mass_scale);
        assert!((drawn[3].air_k / inputs.air.sigma - other[3].air_k / 0.9).abs() < 1e-12);
        let spread = |pick: &dyn Fn(&InputDraw) -> f64| {
            mean_and_deviation(&drawn.iter().map(pick).collect::<Vec<f64>>()).unwrap()
        };
        let near = |(mean, deviation): (f64, f64), center: f64, sigma: f64| {
            assert!(
                (mean - center).abs() < 1e-9 * sigma,
                "{mean} about {center}"
            );
            // The medians' spread, 0.99864 (two hypercubes' 200 over 199 degrees of freedom).
            assert!(
                (deviation / sigma - 0.998_64 * (198.0_f64 / 199.0).sqrt()).abs() < 1e-5,
                "{deviation} for {sigma}"
            );
        };
        near(spread(&|d| d.dry_mass_scale), 1.0, 0.005);
        near(spread(&|d| d.impulse_scale), 1.0, 0.025);
        near(spread(&|d| d.wind_east_m_s), 0.0, 4.47);
        near(spread(&|d| d.wind_north_m_s), 0.0, 4.47);
        near(spread(&|d| d.rail_up_rad), 0.0, 1.0_f64.to_radians());
        near(spread(&|d| d.rail_across_rad), 0.0, 1.0_f64.to_radians());
        near(spread(&|d| d.air_k), 0.0, 15.0 / 3.0_f64.sqrt());
        let alone = InputDraw::one(Drawn::Air, -1.0, inputs);
        assert_eq!(
            alone,
            InputDraw {
                air_k: -inputs.air.sigma,
                ..InputDraw::NONE
            }
        );
        assert_eq!(InputDraw::one(Drawn::Impulse, 0.0, inputs), InputDraw::NONE);
    }

    fn rail(elevation_deg: f64, heading_deg: f64) -> Rail {
        Rail {
            azimuth_rad: heading_deg.to_radians(),
            elevation_rad: elevation_deg.to_radians(),
            ..Rail::vertical(5.0)
        }
    }

    /// The angle between two rails' directions, degrees.
    fn between_deg(a: Rail, b: Rail) -> f64 {
        let direction = |rail: Rail| {
            let (sin_e, cos_e) = rail.elevation_rad.sin_cos();
            let (sin_a, cos_a) = rail.azimuth_rad.sin_cos();
            [cos_e * sin_a, cos_e * cos_a, sin_e]
        };
        let (a, b) = (direction(a), direction(b));
        let dot: f64 = a.iter().zip(b).map(|(a, b)| a * b).sum();
        dot.clamp(-1.0, 1.0).acos().to_degrees()
    }

    /// A rail turns by the angle asked on either axis, whatever its elevation, a vertical one
    /// included; a turn up raises a leaning rail's elevation and keeps its heading, and a turn
    /// across swings its heading and nearly keeps its elevation.
    #[test]
    fn a_rail_turns_by_the_angle_asked() {
        let degree = 1.0_f64.to_radians();
        for start in [
            rail(84.0, 133.0),
            rail(80.0, 75.0),
            rail(90.0, 181.0),
            rail(89.0, 45.0),
        ] {
            for (up, across) in [(degree, 0.0), (0.0, degree), (-degree, 0.0), (0.0, -degree)] {
                let moved = turned(start, up, across);
                assert!((between_deg(start, moved) - 1.0).abs() < 1e-9, "{start:?}");
            }
            // Both at once: the two axes are square, so the turn is √2 degrees.
            let both = turned(start, degree, degree);
            assert!((between_deg(start, both) - SQRT_2).abs() < 1e-9);
            assert_eq!(turned(start, 0.0, 0.0), start);
        }
        let leaning = rail(84.0, 133.0);
        let up = turned(leaning, degree, 0.0);
        assert!((up.elevation_rad.to_degrees() - 85.0).abs() < 1e-9);
        assert!((up.azimuth_rad - leaning.azimuth_rad).abs() < 1e-12);
        let across = turned(leaning, 0.0, degree);
        // A degree across a rail 6° from the vertical swings its heading by
        // atan(tan 1°/cos 84°) = 9.48°.
        let swing = (across.azimuth_rad - leaning.azimuth_rad).to_degrees();
        assert!((swing - 9.48).abs() < 0.01, "{swing}");
        assert!((across.elevation_rad.to_degrees() - 84.0).abs() < 0.1);
        // The roll follows the heading.
        assert_eq!(across.roll_rad, across.azimuth_rad - leaning.azimuth_rad);
        // A vertical rail leans toward its stated heading's far side when turned "up" (past the
        // vertical), and never reads an elevation above 90°.
        let vertical = turned(rail(90.0, 181.0), degree, 0.0);
        assert!((vertical.elevation_rad.to_degrees() - 89.0).abs() < 1e-9);
        assert!(vertical.elevation_rad <= std::f64::consts::FRAC_PI_2);
    }

    /// A sounding of the standard atmosphere's own layers above a pad at 400 m, with a wind.
    fn sounding() -> SoundingProfile {
        let standard = Ussa76::standard();
        let levels = [0.0, 500.0, 1500.0, 3000.0, 5500.0, 9000.0]
            .iter()
            .map(|&height_msl_m| {
                let air = standard.air(height_msl_m).unwrap().air;
                SoundingLevel {
                    height_msl_m,
                    temperature_k: air.temperature_k,
                    pressure_pa: Some(air.pressure_pa),
                    relative_humidity: None,
                    wind_speed_m_s: Some(5.0),
                    wind_direction_from_rad: Some(270.0_f64.to_radians()),
                }
            })
            .collect();
        SoundingProfile::new(levels, 0.7, WindInterpolation::Components).unwrap()
    }

    /// Warming a column by a kelvin keeps the pad's pressure, warms every height by a kelvin,
    /// and makes a barometer on the pad read less at a given height by the ratio of the
    /// column's mean temperatures: about 0.35% for a kelvin, the figure the report quotes.
    #[test]
    fn a_warmer_column_reads_lower_by_its_temperature_ratio() {
        let (pad_m, top_m) = (400.0, 3400.0);
        let given = sounding();
        for delta_k in [1.0, -1.0, 8.66] {
            let warm = warmed(&given, pad_m, delta_k, (0.0, 0.0)).unwrap();
            let pad = |air: &SoundingProfile| air.air(pad_m).unwrap().air;
            // Under a pascal a kelvin on the pad.
            let moved_pa = (pad(&warm).pressure_pa - pad(&given).pressure_pa).abs();
            assert!(moved_pa < delta_k.abs(), "{moved_pa} Pa for {delta_k} K");
            assert!((pad(&warm).temperature_k - pad(&given).temperature_k - delta_k).abs() < 0.01);
            // The pressures at the levels are the given ones.
            assert_eq!(warm.pressures_pa(), given.pressures_pa());
            let reading = |air: &SoundingProfile| {
                Barometer::on_pad(air, 0.0, pad_m)
                    .unwrap()
                    .reading_m(top_m)
                    .unwrap()
            };
            // The column's mean temperature between the two heights, as the hypsometric
            // equation takes it: the standard's lapse is linear, so the log mean of the ends.
            let (low_k, high_k) = (
                pad(&given).temperature_k,
                given.air(top_m).unwrap().air.temperature_k,
            );
            let mean_k = log_mean(high_k, low_k);
            let ratio = reading(&warm) / reading(&given);
            assert!(
                (ratio - mean_k / (mean_k + delta_k)).abs() < 2e-4,
                "{ratio} against {} for {delta_k} K",
                mean_k / (mean_k + delta_k)
            );
        }
        // A kelvin is 0.36% of the reading over this column, whose mean is near 276 K.
        let warm = warmed(&given, pad_m, 1.0, (0.0, 0.0)).unwrap();
        let read = |air: &SoundingProfile| {
            Barometer::on_pad(air, 0.0, pad_m)
                .unwrap()
                .reading_m(top_m)
                .unwrap()
        };
        let share = 1.0 - read(&warm) / read(&given);
        assert!((share - 0.0036).abs() < 0.0002, "{share}");
        // No warming and no wind is the profile itself.
        assert_eq!(warmed(&given, pad_m, 0.0, (0.0, 0.0)).unwrap(), given);
    }

    /// A wind added to a column is added at every height and leaves its air alone; added to a
    /// steady wind it is the vector sum.
    #[test]
    fn a_wind_is_added_at_every_height() {
        let given = sounding();
        let windy = warmed(&given, 400.0, 0.0, (2.0, -3.0)).unwrap();
        for height_m in [400.0, 1000.0, 4000.0, 8000.0] {
            let was = given.wind().unwrap();
            let is = windy.wind().unwrap();
            let moved = is.wind(height_m).unwrap().velocity_enu_m_s
                - was.wind(height_m).unwrap().velocity_enu_m_s;
            assert!(
                (moved.x - 2.0).abs() < 1e-12 && (moved.y + 3.0).abs() < 1e-12,
                "{moved:?}"
            );
            assert_eq!(windy.air(height_m).unwrap(), given.air(height_m).unwrap());
        }
        // A 5 m/s wind from the west blows east at 5 m/s; 2 m/s more east and 3 m/s south make
        // it 7 east and 3 south: √58 m/s, from 270° + atan(3/7) = 293.2°.
        let (speed, from) = wind_plus(5.0, 270.0_f64.to_radians(), (2.0, -3.0));
        assert!((speed - 58.0_f64.sqrt()).abs() < 1e-12);
        assert!(
            (from.to_degrees().rem_euclid(360.0) - 293.198_59).abs() < 1e-4,
            "{from}"
        );
        // A calm with nothing added stays calm; the standard day takes both.
        let draw = InputDraw {
            wind_east_m_s: 2.0,
            wind_north_m_s: -3.0,
            air_k: 5.0,
            ..InputDraw::NONE
        };
        let west = ConstantWind::new(5.0, 270.0_f64.to_radians()).unwrap();
        let (air, wind) = draw.standard_day(300.0, west).unwrap();
        assert!((wind.speed_m_s() - 58.0_f64.sqrt()).abs() < 1e-12);
        let standard = Ussa76::standard();
        let (warm, cool) = (
            air.air(300.0).unwrap().air,
            standard.air(300.0).unwrap().air,
        );
        assert!((warm.temperature_k - cool.temperature_k - 5.0).abs() < 1e-9);
        assert!((warm.pressure_pa - cool.pressure_pa).abs() < 1e-6);
        let (same_air, same_wind) = InputDraw::NONE.standard_day(300.0, west).unwrap();
        assert_eq!(same_air, standard);
        assert_eq!(same_wind, west);
    }
}
