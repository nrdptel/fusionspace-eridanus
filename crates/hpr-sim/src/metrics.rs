//! What a flight comes to: its peaks, its apogee, its stability margins, the ejection delay that
//! would fire at apogee, and where each body landed ([the flight-metrics milestone][m1-10a] and
//! [its decision record][adr-077]; the guide's [Flight metrics][page] page).
//!
//! - [`FlightMetrics`] is an [`Observer`]: fly a [`Simulation`] with it, then ask it for a
//!   [`FlightSummary`]. It finds each peak on the integrator's dense output, not in a recorded
//!   table, so a peak between two rows is not missed ([Loft lesson L34][l34]).
//! - [`stability`] gives the static margin and the margin at the flight's Mach number at one
//!   instant, and [`margin`] the margin of any flow. A margin that would be a ratio of nearly cancelling
//!   slopes is `None`, with the pitch-moment slope beside it ([L33][l33]).
//! - [`optimum_delays`] flies the rocket with its charges held and gives, for each motor, the delay
//!   from its burnout to that apogee, so it doesn't depend on the delay flown ([L94][l94]).
//! - Anything that did not happen is `None`, never a zero, and every height names its datum
//!   ([L35][l35]).
//!
//! **Heights.** A height here is the center of mass's ellipsoidal height above the launch site's,
//! [`Sample::height_above_ground_m`]. The center of mass starts above the site (the rocket stands
//! on the rail), at [`FlightSummary::launch_height_m`]; [`Apogee::gain_m`] counts from there, as
//! OpenRocket's altitude does.
//!
//! [l33]: https://hpr.fusionspace.co/decisions-and-roadmap.html#l33
//! [l34]: https://hpr.fusionspace.co/decisions-and-roadmap.html#l34
//! [l35]: https://hpr.fusionspace.co/decisions-and-roadmap.html#l35
//! [l94]: https://hpr.fusionspace.co/decisions-and-roadmap.html#l94
//! [m1-10a]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m1-10a
//! [adr-077]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/DECISIONS.md#adr-077-flight-metrics-peaks-on-the-dense-output-margins-only-where-they-mean-something-and-none-for-what-didnt-happen-2026-09-26
//! [page]: https://hpr.fusionspace.co/physics/metrics.html

use hpr_aero::{AeroModel, Flow};
use serde::{Deserialize, Serialize};

use crate::dynamics::Phase;
use crate::envelope::{self, EnvelopeFlag, HIGH_ANGLE_GRACE_S, HIGH_ANGLE_OF_ATTACK_RAD};
use crate::environment::Environment;
use crate::error::SimError;
use crate::flight::{EventKind, FlightEvent, FlightResult, Simulation, Termination};
use crate::recorder::{FlightStep, Observer, Sample};
use crate::recovery::BodySample;

/// The largest ratio `κ = Σ |C_Nα,i| / C_Nα` at which a margin is given: `√10`.
///
/// The center of pressure is `x_cp = Σ C_Nα,i x_i / C_Nα`, over the parts the model adds up, with
/// each part's station `x_i` on the rocket, in `[0, L]`. Since `x_cp − x_j = Σ C_Nα,i (x_i − x_j) / C_Nα`, it lies within `κ L` of every
/// station. An error `ε C_Nα,j` in one component's slope moves it by
/// `ε C_Nα,j (x_j − x_cp) / C_Nα`, so by at most `ε κ² L`. A rocket whose slopes all push the same
/// way has `κ = 1`, and a 1% error in one slope moves its center of pressure by at most 1% of its
/// length. As the net slope goes to zero, `κ` runs away: the loads become a pure couple with no
/// line of action, and the quotient is noise (Loft published ±12 to 15 calibres so,
/// [lesson L33][l33]). At `κ = √10` a 1% error in one slope can move the center of pressure by a
/// tenth of the rocket's length; past it hpr gives no margin, only the pitch-moment slope, which
/// stays finite. The limit is a chosen bound on that sensitivity, not a measurement. The bound
/// holds for parts that each carry a force at a station: a part that is a pure couple (a step and
/// a flare of equal slopes) has none, and `κ` doesn't count it.
///
/// [l33]: https://hpr.fusionspace.co/decisions-and-roadmap.html#l33
pub const MARGIN_CONDITION_LIMIT: f64 = 3.162_277_660_168_379_5;

/// The Mach number of the static margin: the air at rest, as RocketPy's `static_margin` takes it.
pub const STATIC_MARGIN_MACH: f64 = 0.0;

/// A peak over the flight, with where and when it came.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Peak {
    /// The peak value, in the quantity's own unit.
    pub value: f64,
    /// When it came, s after launch.
    pub time_s: f64,
    /// The center of mass's height above the launch site then, m.
    pub height_above_ground_m: f64,
}

/// The stability margin of one flow.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Margin {
    /// The Mach number.
    pub mach: f64,
    /// The total angle of attack, rad.
    pub angle_of_attack_rad: f64,
    /// The direction the air crosses the rocket, rad from `x_B` toward `y_B`
    /// ([`hpr_aero::Flow::roll_rad`]): for [`weakest_margin`], its weakest plane's.
    #[serde(default)]
    pub roll_rad: f64,
    /// The rocket's normal-force slope `C_Nα` on the reference area, per radian (at an angle of
    /// attack, the secant `C_N/α`, [`hpr_aero::NormalForce::slope_per_rad`]).
    pub normal_force_slope_per_rad: f64,
    /// `Σ |C_Nα,i|` over the components, per radian: the scale against which the net slope is
    /// judged ([`MARGIN_CONDITION_LIMIT`]). With a normal-force table it is the table's own slope's
    /// magnitude.
    pub slope_magnitude_sum_per_rad: f64,
    /// The pitch-moment slope about the center of mass on the reference area and diameter, per
    /// radian: `C_mα = −(Σ C_Nα,i x_i − C_Nα x_cg) / d`, stations `x` aft of the nose tip, from
    /// [`hpr_aero::NormalForce::moment_slope_m`]. Negative restores. It is finite when the margin
    /// is not, and with a margin it is `−C_Nα · margin`.
    pub pitch_moment_slope_per_rad: f64,
    /// The center of pressure, m aft of the nose tip; `None` when the margin is.
    pub cp_station_m: Option<f64>,
    /// The margin `(x_cp − x_cg) / d`, calibres: positive with the center of pressure aft of the
    /// center of mass. `None` when the net slope is not positive or is below
    /// `Σ |C_Nα,i| / MARGIN_CONDITION_LIMIT`, where the quotient would be noise.
    pub margin_cal: Option<f64>,
}

/// The rocket's stability at one instant.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Stability {
    /// When, s after launch.
    pub time_s: f64,
    /// The center of mass's height above the launch site then, m.
    pub height_above_ground_m: f64,
    /// The dynamic pressure then, Pa: how hard the air presses on the margin's moment.
    pub dynamic_pressure_pa: f64,
    /// The center of mass, m aft of the nose tip.
    pub cg_station_m: f64,
    /// The reference diameter `d` the margins are counted in, m (the sustainer's after a powered
    /// separation).
    pub reference_diameter_m: f64,
    /// The static margin: the air along the axis at [`STATIC_MARGIN_MACH`], with the center of
    /// mass of this instant (RocketPy's `static_margin`), in the rocket's weakest plane
    /// ([`weakest_margin`]).
    pub static_margin: Margin,
    /// The flight margin: the air along the axis at the flight's Mach number, with the center of
    /// mass of this instant (RocketPy's `stability_margin`), in the rocket's weakest plane
    /// ([`weakest_margin`]). The angle of attack is left out: near
    /// apogee it swings toward 90° as the rocket slows and tips over, and a least margin that
    /// followed it would land wherever large angles stopped being counted.
    pub flight_margin: Margin,
}

/// The margin of `aero` in `flow` with the center of mass at `cg_station_m` (m aft of the nose
/// tip). See [`Margin`] and [`MARGIN_CONDITION_LIMIT`] for when it is `None`.
///
/// # Errors
///
/// The aerodynamic model's, for a flow outside its range.
pub fn margin(aero: &AeroModel, flow: &Flow, cg_station_m: f64) -> Result<Margin, SimError> {
    let normal = aero.normal_force(flow)?;
    let d = aero.reference_diameter_m();
    let slope = normal.slope_per_rad;
    let scale = if aero.normal_force_table().is_some() {
        // The table gives one force at one station.
        slope.abs()
    } else {
        aero.components(flow)?
            .iter()
            .map(|c| c.normal_force.slope_per_rad.abs())
            .sum()
    };
    let conditioned = slope > 0.0 && slope * MARGIN_CONDITION_LIMIT >= scale;
    let cp_station_m = normal.cp_station_m.filter(|_| conditioned);
    Ok(Margin {
        mach: flow.mach,
        angle_of_attack_rad: flow.alpha_rad,
        roll_rad: flow.roll_rad,
        normal_force_slope_per_rad: slope,
        slope_magnitude_sum_per_rad: scale,
        pitch_moment_slope_per_rad: -(normal.moment_slope_m - slope * cg_station_m) / d,
        cp_station_m,
        margin_cal: cp_station_m.map(|cp| (cp - cg_station_m) / d),
    })
}

/// The least margin of `aero` at Mach `mach`, the air along the axis, over the direction it
/// crosses the rocket, with the center of mass at `cg_station_m`: [`margin`] in the weakest plane,
/// whose direction it gives ([`Margin::roll_rad`]); or a plane where no margin can be given, if
/// there is one (#329).
///
/// A fin set of one or two fins carries `Σ sin²(φ − θ_k)` of its force in the plane at `φ`
/// (Niskanen 2009 eq. 3.51, [`hpr_aero::roll_sum`]), so a rocket with one ([`AeroModel::rolls`])
/// has a margin in each plane. Each of its terms is then `a + b cos 2φ + c sin 2φ`, and so are the
/// net slope `S(φ)`, its moment `M(φ) = Σ C_Nα,i x_i`, and the sum of the slopes' magnitudes
/// `Σ(φ)` while each fin's slope is positive: three planes, `φ = 0, π/3, 2π/3`, give each sum's
/// `a`, `b` and `c`. A quotient `N/D` of two such sums is stationary where
/// `P sin 2φ + Q cos 2φ + R = 0`, with `P = n₀d₁ − n₁d₀`, `Q = n₂d₀ − n₀d₂` and
/// `R = n₂d₁ − n₁d₂` (its derivative's numerator, `N′D − ND′`, expanded). The center of pressure
/// `M/S` is foremost at one of its two roots, and the margin's conditioning `S/Σ`
/// ([`MARGIN_CONDITION_LIMIT`]) worst at one of its. Each root, and the axial plane, is evaluated
/// through the model: a plane with no margin wins, else the least margin. A rocket whose fin sets
/// are each of three or more fins, or that flies a normal-force table, has one margin, the axial
/// plane's, returned bit for bit.
///
/// # Errors
///
/// As [`margin`].
pub fn weakest_margin(aero: &AeroModel, mach: f64, cg_station_m: f64) -> Result<Margin, SimError> {
    let axial = margin(aero, &Flow::axial(mach), cg_station_m)?;
    if !aero.rolls() {
        return Ok(axial);
    }
    // Each sum's `[a, b, c]` in `a + b cos u + c sin u`, `u = 2φ`, from `u = 0, 2π/3, 4π/3`.
    let mut samples = [[0.0; 3]; 3];
    for (k, column) in [0.0, 1.0, 2.0].into_iter().enumerate() {
        let flow = Flow::new(mach, 0.0, column * std::f64::consts::FRAC_PI_3);
        let normal = aero.normal_force(&flow)?;
        let scale: f64 = aero
            .components(&flow)?
            .iter()
            .map(|c| c.normal_force.slope_per_rad.abs())
            .sum();
        samples[0][k] = normal.slope_per_rad;
        samples[1][k] = normal.moment_slope_m;
        samples[2][k] = scale;
    }
    let fit = |[f0, f1, f2]: [f64; 3]| {
        [
            (f0 + f1 + f2) / 3.0,
            (2.0 * f0 - f1 - f2) / 3.0,
            (f1 - f2) / 3.0_f64.sqrt(),
        ]
    };
    let [slope, moment, scale] = samples.map(fit);
    let mut least = axial;
    for numerator_denominator in [(moment, slope), (slope, scale)] {
        for u in stationary_angles(numerator_denominator) {
            let candidate = margin(aero, &Flow::new(mach, 0.0, 0.5 * u), cg_station_m)?;
            least = match (least.margin_cal, candidate.margin_cal) {
                (None, _) => least,
                (Some(_), None) => candidate,
                (Some(a), Some(b)) if b < a => candidate,
                _ => least,
            };
        }
    }
    Ok(least)
}

/// The angles `u` in `[0, 2π)` where `N(u)/D(u)` is stationary, `N = n₀ + n₁ cos u + n₂ sin u`
/// and `D` alike: the roots of `P sin u + Q cos u + R = 0` ([`weakest_margin`]). `P sin u + Q cos u`
/// is `A sin(u + ψ)` with `A = √(P² + Q²)` and `ψ = atan2(Q, P)`. None when the quotient is
/// constant (`A` is zero); a root just past `|R| = A` by rounding is taken at the touch point.
fn stationary_angles(([n0, n1, n2], [d0, d1, d2]): ([f64; 3], [f64; 3])) -> Vec<f64> {
    let (p, q, r) = (n0 * d1 - n1 * d0, n2 * d0 - n0 * d2, n2 * d1 - n1 * d2);
    let a = p.hypot(q);
    if a.is_nan() || a <= 0.0 {
        return Vec::new();
    }
    let ratio = -r / a;
    if ratio.is_nan() || ratio.abs() > 1.0 + 1e-9 {
        return Vec::new();
    }
    let (root, psi) = (ratio.clamp(-1.0, 1.0).asin(), q.atan2(p));
    [root - psi, std::f64::consts::PI - root - psi]
        .into_iter()
        .map(|u| u.rem_euclid(std::f64::consts::TAU))
        .collect()
}

/// The static margin and the flight margin of `aero` at `time_s`, with the center of mass
/// `height_above_ground_m` above the launch site and at `cg_station_m` on the airframe, and the
/// flight's air at Mach `mach` pressing with `dynamic_pressure_pa`.
///
/// # Errors
///
/// As [`margin`].
pub fn stability(
    aero: &AeroModel,
    time_s: f64,
    height_above_ground_m: f64,
    dynamic_pressure_pa: f64,
    cg_station_m: f64,
    mach: f64,
) -> Result<Stability, SimError> {
    Ok(Stability {
        time_s,
        height_above_ground_m,
        dynamic_pressure_pa,
        cg_station_m,
        reference_diameter_m: aero.reference_diameter_m(),
        static_margin: weakest_margin(aero, STATIC_MARGIN_MACH, cg_station_m)?,
        flight_margin: weakest_margin(aero, mach, cg_station_m)?,
    })
}

/// The apogee of the flight's center of mass.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Apogee {
    /// When, s after launch.
    pub time_s: f64,
    /// The center of mass's ellipsoidal height above the launch site, m.
    pub height_above_ground_m: f64,
    /// The rise of the center of mass from where it stood at launch, m: the height above the site
    /// less [`FlightSummary::launch_height_m`], as OpenRocket's altitude counts. `None` for a
    /// flight started in the air ([`Simulation::run_free`]).
    pub gain_m: Option<f64>,
}

/// Where something landed: the center of mass as it reached the launch site's ellipsoidal
/// height.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Landing {
    /// Which separated body ([`crate::BodyFlight::body`]), or `None` for the flight itself: the
    /// stack, or after a powered separation the sustainer.
    pub body: Option<usize>,
    /// When, s after launch.
    pub time_s: f64,
    /// Geodetic latitude, degrees north (WGS 84).
    pub latitude_deg: f64,
    /// Longitude, degrees east (WGS 84).
    pub longitude_deg: f64,
    /// East of the launch site in its local frame, m.
    pub east_m: f64,
    /// North of the launch site, m.
    pub north_m: f64,
    /// The horizontal distance from the launch site, m.
    pub distance_m: f64,
    /// The speed of the center of mass at the ground hit, relative to the ground, m/s.
    pub ground_hit_speed_m_s: f64,
    /// Its downward speed then, m/s.
    pub descent_rate_m_s: f64,
}

impl Landing {
    /// The landing at `sample`, for `body`, located on `environment`'s ellipsoid.
    ///
    /// # Errors
    ///
    /// [`SimError::Core`] if the position has no geodetic coordinates.
    pub fn at(
        body: Option<usize>,
        sample: &BodySample,
        environment: &Environment,
    ) -> Result<Self, SimError> {
        let place = environment
            .earth
            .frame()
            .geodetic_from_enu(sample.cg_enu_m)?;
        let (east_m, north_m) = (sample.cg_enu_m.x, sample.cg_enu_m.y);
        Ok(Self {
            body,
            time_s: sample.time_s,
            latitude_deg: place.latitude_rad.to_degrees(),
            longitude_deg: place.longitude_rad.to_degrees(),
            east_m,
            north_m,
            distance_m: east_m.hypot(north_m),
            ground_hit_speed_m_s: sample.cg_velocity_enu_m_s.length(),
            descent_rate_m_s: -sample.vertical_speed_m_s,
        })
    }
}

/// The metrics of one flight. Every field that may not have happened is an `Option`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlightSummary {
    /// Why the flight ended.
    pub termination: Termination,
    /// The center of mass's height above the launch site as the rocket stood at launch, m. `None`
    /// for a flight started in the air ([`Simulation::run_free`]).
    pub launch_height_m: Option<f64>,
    /// The speed as the last rail guide left the rail, m/s, with its time and height.
    pub rail_exit_speed_m_s: Option<Peak>,
    /// The apogee of the center of mass. A stack that comes apart at a separation before its
    /// apogee ([`Termination::Separated`]) has the apogee of the part that keeps the nose, body 0;
    /// one that comes apart at an ejection ([`crate::Ejection`]) has none.
    pub apogee: Option<Apogee>,
    /// The largest speed of the center of mass relative to the ground after liftoff, m/s.
    pub max_speed_m_s: Option<Peak>,
    /// The largest Mach number after liftoff.
    pub max_mach: Option<Peak>,
    /// The largest dynamic pressure after liftoff ("max q"), Pa.
    pub max_dynamic_pressure_pa: Option<Peak>,
    /// The largest acceleration of the nose tip (the body origin) relative to the launch frame
    /// from liftoff until a recovery device opens, m/s²: the boost and the coast, from the
    /// equations of motion.
    pub max_acceleration_m_s2: Option<Peak>,
    /// The largest acceleration under the recovery devices, m/s²: the opening shock, which
    /// follows the inflation model, kept apart from the boost's.
    pub max_descent_acceleration_m_s2: Option<Peak>,
    /// The smallest static margin from the rail exit to apogee or the first deployment,
    /// calibres, where it is defined, found inside steps as a peak is. After a powered separation
    /// the sustainer's margins count its own diameter.
    pub min_static_margin_cal: Option<Peak>,
    /// The smallest flight margin over the same span, calibres, found the same way. RocketPy's
    /// `min_stability_margin` takes its least over the whole flight, so it can differ.
    pub min_flight_margin_cal: Option<Peak>,
    /// The smallest static margin over the same span while a motor burns, calibres, found the
    /// same way: below zero, the rocket is unstable under power
    /// ([`EnvelopeFlag::UnstableUnderPower`]; [#335][i335]). A step counts when the thrust at its
    /// middle is positive. Every ignition, thrust-curve knot and burnout ends a step, so a motor's
    /// thrust then holds through the step, and a step that ends at a burnout counts up to that
    /// instant. A NaN margin at a step's start, middle or end while a motor burns is kept, and
    /// then nothing replaces it, so it can't hide the flag; a NaN thrust counts as burning. `None`
    /// when no motor burns in the span, the margin is nowhere defined while one does, and in a
    /// summary written before this field was.
    ///
    /// [i335]: https://github.com/nrdptel/fusionspace-eridanus/issues/335
    #[serde(default)]
    pub min_powered_static_margin_cal: Option<Peak>,
    /// The largest static pitch-moment slope `C_mα` per radian
    /// ([`Margin::pitch_moment_slope_per_rad`], `C_mα = −(Σ C_Nα,i x_i − C_Nα x_cg) / d`) over the
    /// same span while a motor burns, taken only at instants where the static margin is
    /// undefined: above zero, the air turns the rocket away from its path, so it is unstable under
    /// power although hpr can give no margin ([`EnvelopeFlag::UnstableWithoutMargin`]). The
    /// margin is undefined where the net normal-force slope is not positive, or is too small
    /// against `Σ |C_Nα,i|` for the quotient to mean anything ([`MARGIN_CONDITION_LIMIT`]); the
    /// moment slope stays finite there and still says which way the air turns the rocket.
    /// Taken at the start, middle and end of each step where a motor burns, counted as for
    /// [`min_powered_static_margin_cal`](Self::min_powered_static_margin_cal), not searched
    /// between. A NaN slope is kept, and then nothing replaces it. `None` when the margin is
    /// defined at every such instant, no motor burns in the span, and in a summary written
    /// before this field was.
    #[serde(default)]
    pub max_powered_moment_slope_per_rad: Option<Peak>,
    /// The stability as the last rail guide left the rail.
    pub rail_exit_stability: Option<Stability>,
    /// The largest angle of attack more than [`HIGH_ANGLE_GRACE_S`] after the rail exit (or the
    /// start of a flight begun in the air), until apogee or the first deployment, rad. Only
    /// instants where a 15° angle would give a normal force of at least
    /// [`HIGH_ANGLE_MIN_FORCE_SHARE`](envelope::HIGH_ANGLE_MIN_FORCE_SHARE) of the rocket's weight count: as the path turns over near
    /// apogee the angle swings past 15° on an ordinary flight, with too little air to move it ([the envelope flags' decision
    /// record][adr-179]). Taken at each step's start, middle and end, not searched between.
    /// `None` when no instant counted, and in a summary written before this field was.
    ///
    /// [adr-179]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0179-the-envelope-flags.md
    #[serde(default)]
    pub max_angle_of_attack_rad: Option<Peak>,
    /// Where the flight itself landed: the stack, or after a powered separation the sustainer.
    pub landing: Option<Landing>,
    /// Where each separated body landed, in body order; a body that didn't land is left out.
    pub body_landings: Vec<Landing>,
}

impl FlightSummary {
    /// The operating envelope's flags this flight raises, in [`EnvelopeFlag`]'s order.
    #[must_use]
    pub fn envelope_flags(&self) -> Vec<EnvelopeFlag> {
        crate::envelope::flags(
            self.max_mach,
            self.max_angle_of_attack_rad,
            self.min_powered_static_margin_cal,
            self.max_powered_moment_slope_per_rad,
        )
    }

    /// The flight's own ground-hit speed, m/s, if it landed.
    #[must_use]
    pub fn ground_hit_speed_m_s(&self) -> Option<f64> {
        self.landing.map(|landing| landing.ground_hit_speed_m_s)
    }

    /// Where the part that keeps the nose landed, as [`Self::apogee`] is its apogee: the
    /// flight's own landing ([`Self::landing`]), or, after the stack comes apart with nothing
    /// left to burn ([`Termination::Separated`], which leaves [`Self::landing`] empty), part 0's
    /// from [`Self::body_landings`].
    #[must_use]
    pub fn nose_landing(&self) -> Option<&Landing> {
        self.landing
            .as_ref()
            .or_else(|| self.body_landings.iter().find(|l| l.body == Some(0)))
    }
}

/// Watches a flight and keeps its peaks, and its stability from the rail exit to apogee or the
/// first deployment.
///
/// Each step is sampled at its start, middle and end. When the parabola through the three has its
/// top inside the step, a golden-section search on the dense output finds the peak there; the
/// largest of what it finds and the three samples is kept. The least margins are found the same
/// way, with the parabola turned over. Thrust-curve knots and events end steps, so a thrust
/// spike's peak is a step's end. Nothing is kept on the pad.
///
/// A watcher keeps one flight: [`FlightMetrics::clear`] it before watching another, and
/// [`FlightMetrics::summary`] refuses a flight whose steps it didn't all see, once each. Like
/// [`crate::Recorder`], it serializes what it holds for inspection and is built with
/// [`FlightMetrics::new`], not deserialized.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct FlightMetrics {
    launch_height_m: Option<f64>,
    max_speed: Option<Peak>,
    max_mach: Option<Peak>,
    max_q: Option<Peak>,
    max_acceleration: Option<Peak>,
    max_descent_acceleration: Option<Peak>,
    /// Whether any step was seen, and the last one's end.
    last_end_s: Option<f64>,
    /// How many steps it saw: a flight's accepted steps, one each.
    steps: u64,
    on_rail: bool,
    past_apogee: bool,
    /// Whether a separation came since the last step: the next starts on another rocket.
    separated: bool,
    /// The top Mach number of each vehicle the flight flew, as [`Self::max_mach_by_vehicle`].
    vehicle_max_mach: Vec<Option<Peak>>,
    rail_exit: Option<Stability>,
    stability: Vec<Stability>,
    min_static_margin: Option<Peak>,
    min_flight_margin: Option<Peak>,
    /// The least static margin over the steps where a motor burns.
    min_powered_static_margin: Option<Peak>,
    /// The largest static pitch-moment slope over the steps where a motor burns, at instants
    /// without a static margin.
    max_powered_moment_slope: Option<Peak>,
    /// When the free flight began (the rail exit, or the start of a flight begun in the air), s:
    /// where the angle of attack's window opens.
    free_start_s: Option<f64>,
    max_angle_of_attack: Option<Peak>,
}

/// What a peak is of.
#[derive(Debug, Clone, Copy)]
enum Quantity {
    Speed,
    Mach,
    DynamicPressure,
    Acceleration,
}

impl Quantity {
    const ALL: [Quantity; 4] = [
        Quantity::Speed,
        Quantity::Mach,
        Quantity::DynamicPressure,
        Quantity::Acceleration,
    ];

    fn of(self, sample: &Sample) -> f64 {
        match self {
            Quantity::Speed => sample.cg_velocity_enu_m_s.length(),
            Quantity::Mach => sample.mach,
            Quantity::DynamicPressure => sample.dynamic_pressure_pa,
            Quantity::Acceleration => sample.acceleration_enu_m_s2.length(),
        }
    }
}

impl FlightMetrics {
    /// A watcher with nothing seen.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Forgets what it saw, ready for another flight.
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    /// The stability from the rail exit (or the start of a flight begun in the air) to apogee or
    /// the first deployment, whichever comes first: at the rail exit and at every step's end, in
    /// time order. A powered separation adds the sustainer's own entry at the split, after the
    /// stack's and at the same time, so the watcher needs the flight's events as well as its
    /// steps.
    #[must_use]
    pub fn stability(&self) -> &[Stability] {
        &self.stability
    }

    /// The top Mach number of each vehicle the flight flew in six degrees of freedom, as the
    /// flight's own is found ([`FlightSummary::max_mach`]) but over that vehicle's steps alone:
    /// the launch stack's first, then after each separation the vehicle that flew on (the
    /// sustainer after a powered one, [`crate::Simulation::flown_vehicles`]). A separation that
    /// ended the flight leaves a last entry of `None`, as does a vehicle that took no step off
    /// the pad; a flight that took no step has none. The bodies that fall as points after a
    /// separation aren't among them, as they aren't in the flight's.
    #[must_use]
    pub fn max_mach_by_vehicle(&self) -> &[Option<Peak>] {
        &self.vehicle_max_mach
    }

    /// The summary of `result`, the flight this watched, flown in `environment`.
    ///
    /// # Errors
    ///
    /// [`SimError::Domain`] if it didn't see each of `result`'s accepted steps once, or its last
    /// step ended after `result` did (it watched another flight, or wasn't cleared between two);
    /// [`SimError::Core`] if a landing has no geodetic coordinates.
    pub fn summary(
        &self,
        result: &FlightResult,
        environment: &Environment,
    ) -> Result<FlightSummary, SimError> {
        // A flight can end without a step (before its first, or on a stop a few ulps ahead that
        // the clock just moves to), so the steps are counted rather than its end matched.
        if self.steps != result.stats.accepted_steps
            || self
                .last_end_s
                .is_some_and(|end| end > result.final_sample.time_s)
        {
            return Err(SimError::Domain {
                what: "steps this watcher saw (it must watch each step of the flight it sums up, \
                       and be cleared before another)",
                value: self.steps as f64,
            });
        }
        // A stack that came apart at a separation on its way up has the apogee of the part that
        // keeps the nose, body 0, which flies on as a point mass ([`Termination::Separated`]).
        // An ejection's body 0 is its lead piece, often the nose cone alone, so a stack that
        // came apart at one has none.
        let stack = result
            .event(EventKind::Apogee)
            .map(|event| (event.sample.time_s, event.sample.height_above_ground_m));
        let nose = || {
            result
                .bodies
                .iter()
                .find(|body| body.body == 0)
                .and_then(|body| body.event(EventKind::Apogee))
                .map(|event| (event.sample.time_s, event.sample.height_above_ground_m))
        };
        let apogee = stack
            .or_else(|| {
                let separated = result.termination == Termination::Separated
                    && result.event(EventKind::Separation).is_some()
                    && !result
                        .events
                        .iter()
                        .any(|event| matches!(event.kind, EventKind::Ejection(_)));
                separated.then(nose).flatten()
            })
            .map(|(time_s, height)| Apogee {
                time_s,
                height_above_ground_m: height,
                gain_m: self.launch_height_m.map(|start| height - start),
            });
        let rail_exit = result.event(EventKind::RailExit).map(|event| event.sample);
        let landing = if result.termination == Termination::GroundHit {
            let s = &result.final_sample;
            Some(Landing::at(
                None,
                &BodySample {
                    time_s: s.time_s,
                    cg_enu_m: s.cg_enu_m,
                    cg_velocity_enu_m_s: s.cg_velocity_enu_m_s,
                    height_above_ground_m: s.height_above_ground_m,
                    vertical_speed_m_s: s.vertical_speed_m_s,
                    airspeed_m_s: s.airspeed_m_s,
                    recovery_drag_area_m2: s.recovery_drag_area_m2,
                    mass_kg: s.mass_kg,
                },
                environment,
            )?)
        } else {
            None
        };
        let body_landings = result
            .bodies
            .iter()
            .filter(|body| body.termination == Termination::GroundHit)
            .map(|body| Landing::at(Some(body.body), &body.final_sample, environment))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(FlightSummary {
            termination: result.termination,
            launch_height_m: self.launch_height_m,
            rail_exit_speed_m_s: rail_exit.map(|s| Peak {
                value: s.cg_velocity_enu_m_s.length(),
                time_s: s.time_s,
                height_above_ground_m: s.height_above_ground_m,
            }),
            apogee,
            max_speed_m_s: self.max_speed,
            max_mach: self.max_mach,
            max_dynamic_pressure_pa: self.max_q,
            max_acceleration_m_s2: self.max_acceleration,
            max_descent_acceleration_m_s2: self.max_descent_acceleration,
            min_static_margin_cal: self.min_static_margin,
            min_flight_margin_cal: self.min_flight_margin,
            min_powered_static_margin_cal: self.min_powered_static_margin,
            max_powered_moment_slope_per_rad: self.max_powered_moment_slope,
            rail_exit_stability: self.rail_exit,
            max_angle_of_attack_rad: self.max_angle_of_attack,
            landing,
            body_landings,
        })
    }

    fn slot(&mut self, quantity: Quantity, phase: Phase) -> &mut Option<Peak> {
        match quantity {
            Quantity::Speed => &mut self.max_speed,
            Quantity::Mach => &mut self.max_mach,
            Quantity::DynamicPressure => &mut self.max_q,
            Quantity::Acceleration if phase == Phase::Descent => &mut self.max_descent_acceleration,
            Quantity::Acceleration => &mut self.max_acceleration,
        }
    }
}

impl Observer for FlightMetrics {
    fn step(&mut self, step: &dyn FlightStep) -> Result<(), SimError> {
        let phase = step.phase();
        let (a, b) = (step.start_s(), step.end_s());
        let start = step.sample(a)?;
        if self.last_end_s.is_none() {
            match phase {
                Phase::Pad | Phase::Rail => {
                    self.launch_height_m = Some(start.height_above_ground_m)
                }
                // A flight begun in the air on its way down has no apogee to come.
                _ if start.vertical_speed_m_s <= 0.0 => self.past_apogee = true,
                _ => {}
            }
        }
        self.steps += 1;
        self.last_end_s = Some(b);
        if phase == Phase::Pad {
            return Ok(());
        }
        let samples = [start, step.sample(0.5 * (a + b))?, step.sample(b)?];
        for quantity in Quantity::ALL {
            let best = step_peak(step, quantity, &samples)?;
            let slot = self.slot(quantity, phase);
            if slot.is_none_or(|peak| best.value > peak.value) {
                *slot = Some(best);
            }
            if matches!(quantity, Quantity::Mach) {
                if self.vehicle_max_mach.is_empty() {
                    self.vehicle_max_mach.push(None);
                }
                if let Some(slot) = self.vehicle_max_mach.last_mut()
                    && slot.is_none_or(|peak| best.value > peak.value)
                {
                    *slot = Some(best);
                }
            }
        }
        if phase == Phase::Rail {
            self.on_rail = true;
        }
        // On the rail the rail holds the rocket, so stability starts at the rail exit.
        if !self.past_apogee && phase == Phase::Free {
            let from_s = *self.free_start_s.get_or_insert(samples[0].time_s);
            // A step starts where the last ended, on the same rocket, unless a separation came
            // between: then the sustainer's own margins at the split start the step.
            let first = match self.stability.last() {
                Some(last) if !self.separated => *last,
                _ => {
                    let first = step.stability(a)?;
                    if self.stability.is_empty() && self.on_rail {
                        self.rail_exit = Some(first);
                    }
                    self.stability.push(first);
                    first
                }
            };
            let end = step.stability(b)?;
            let entries = [first, step.stability(0.5 * (a + b))?, end];
            for (sample, entry) in samples.iter().zip(&entries) {
                // The normal force a 15° angle would give here: whether the air could turn the
                // rocket at all.
                let force_n = envelope::normal_force_n(
                    sample.dynamic_pressure_pa,
                    entry.reference_diameter_m,
                    entry.flight_margin.normal_force_slope_per_rad,
                    HIGH_ANGLE_OF_ATTACK_RAD,
                );
                // A NaN angle is kept, and then nothing replaces it: it can't hide a flag.
                if sample.time_s - from_s > HIGH_ANGLE_GRACE_S
                    && envelope::force_counts(force_n, sample.mass_kg)
                    && self.max_angle_of_attack.is_none_or(|peak| {
                        !peak.value.is_nan()
                            && (sample.angle_of_attack_rad.is_nan()
                                || sample.angle_of_attack_rad > peak.value)
                    })
                {
                    self.max_angle_of_attack = Some(Peak {
                        value: sample.angle_of_attack_rad,
                        time_s: sample.time_s,
                        height_above_ground_m: sample.height_above_ground_m,
                    });
                }
            }
            let least_static = least_margin(step, a, b, &entries, static_of)?;
            for (least, slot) in [
                (least_static, &mut self.min_static_margin),
                (
                    least_margin(step, a, b, &entries, flight_of)?,
                    &mut self.min_flight_margin,
                ),
            ] {
                if let Some(least) = least
                    && slot.is_none_or(|peak| replaces(least.value, peak.value))
                {
                    *slot = Some(least);
                }
            }
            // Every ignition, thrust-curve knot and burnout ends a step, so the thrust at the
            // middle says whether a motor burns through the step. A NaN counts as burning.
            let thrust_n = samples[1].thrust_n;
            if thrust_n > 0.0 || thrust_n.is_nan() {
                keep_powered(&mut self.min_powered_static_margin, &entries, least_static);
                keep_moment_without_margin(&mut self.max_powered_moment_slope, &entries);
            }
            self.stability.push(end);
        }
        self.separated = false;
        Ok(())
    }

    fn event(&mut self, event: &FlightEvent) {
        match event.kind {
            EventKind::Apogee => self.past_apogee = true,
            EventKind::Separation => {
                self.separated = true;
                // The vehicle that flies on starts its own peak; the stack's is the first.
                if self.vehicle_max_mach.is_empty() {
                    self.vehicle_max_mach.push(None);
                }
                self.vehicle_max_mach.push(None);
            }
            _ => {}
        }
    }
}

/// The golden ratio's inverse, `(√5 − 1)/2`.
const INVERSE_PHI: f64 = 0.618_033_988_749_894_9;

/// How finely a peak's time is found, relative to the flight's clock (at least 1 s): 1e-9. The
/// value's error goes as the square of the time's, so it is far below the integration's.
const PEAK_TIME_RESOLUTION: f64 = 1e-9;

/// The largest of `value` on `[a, b]`, where it has one top: a golden-section search (Kiefer
/// 1953) on a step's dense output, to [`PEAK_TIME_RESOLUTION`].
fn golden_max(
    mut a: f64,
    mut b: f64,
    mut value: impl FnMut(f64) -> Result<Peak, SimError>,
) -> Result<Peak, SimError> {
    let mut c = b - INVERSE_PHI * (b - a);
    let mut d = a + INVERSE_PHI * (b - a);
    let mut fc = value(c)?;
    let mut fd = value(d)?;
    while b - a > PEAK_TIME_RESOLUTION * b.abs().max(1.0) {
        if fc.value >= fd.value {
            b = d;
            (d, fd) = (c, fc);
            c = b - INVERSE_PHI * (b - a);
            fc = value(c)?;
        } else {
            a = c;
            (c, fc) = (d, fd);
            d = a + INVERSE_PHI * (b - a);
            fd = value(d)?;
        }
    }
    Ok(if fc.value >= fd.value { fc } else { fd })
}

/// Where the parabola through `(−1, va)`, `(0, vm)` and `(1, vb)` has its top inside: it bends
/// down, and its top `x = (va − vb) / (2 (va − 2 vm + vb))` has `|x| < 1`.
fn top_inside(va: f64, vm: f64, vb: f64) -> bool {
    let bend = va - 2.0 * vm + vb;
    bend < 0.0 && (va - vb).abs() < -2.0 * bend
}

/// The peak of `quantity` on the step, from its start, middle and end samples and, when the
/// parabola through them has its top inside, a search between.
fn step_peak(
    step: &dyn FlightStep,
    quantity: Quantity,
    samples: &[Sample; 3],
) -> Result<Peak, SimError> {
    let peak_of = |sample: &Sample| Peak {
        value: quantity.of(sample),
        time_s: sample.time_s,
        height_above_ground_m: sample.height_above_ground_m,
    };
    let [start, middle, end] = samples.each_ref().map(peak_of);
    let mut best = [middle, end]
        .into_iter()
        .fold(start, |x, y| if y.value > x.value { y } else { x });
    if top_inside(start.value, middle.value, end.value) {
        let found = golden_max(step.start_s(), step.end_s(), |t| {
            step.sample(t).map(|sample| peak_of(&sample))
        })?;
        if found.value > best.value {
            best = found;
        }
    }
    Ok(best)
}

fn static_of(s: &Stability) -> &Margin {
    &s.static_margin
}

fn flight_of(s: &Stability) -> &Margin {
    &s.flight_margin
}

/// How far below a least margin another must be to replace it: this fraction of the least, or of
/// 1 calibre below 1 calibre. A flat margin, which rounding can nudge by an ulp, keeps its first
/// time.
const MARGIN_TIE: f64 = 1e-12;

/// Whether margin `a` is below `b` by more than [`MARGIN_TIE`].
fn clearly_below(a: f64, b: f64) -> bool {
    b - a > MARGIN_TIE * b.abs().max(1.0)
}

/// Whether margin `a` replaces the least so far, `b`: clearly below it, or below zero where `b`
/// is not, so a tie can't keep a margin on the stable side of zero over one past it.
fn replaces(a: f64, b: f64) -> bool {
    clearly_below(a, b) || (a < 0.0 && b >= 0.0)
}

/// Keeps the least static margin of a step where a motor burns in `slot`: a NaN among the step's
/// `entries` first, which then stays, so a broken number can't hide the flag; else `least`, the
/// step's least, when it [`replaces`] the slot's.
fn keep_powered(slot: &mut Option<Peak>, entries: &[Stability; 3], least: Option<Peak>) {
    if slot.is_some_and(|peak| peak.value.is_nan()) {
        return;
    }
    let broken = entries.iter().find_map(|s| {
        s.static_margin
            .margin_cal
            .filter(|margin| margin.is_nan())
            .map(|value| Peak {
                value,
                time_s: s.time_s,
                height_above_ground_m: s.height_above_ground_m,
            })
    });
    if let Some(broken) = broken {
        *slot = Some(broken);
    } else if let Some(least) = least
        && slot.is_none_or(|peak| replaces(least.value, peak.value))
    {
        *slot = Some(least);
    }
}

/// Keeps in `slot` the largest static pitch-moment slope among a powered step's `entries` where
/// the static margin is undefined: a NaN first, which then stays, so a broken number can't hide
/// the flag; else a larger slope.
fn keep_moment_without_margin(slot: &mut Option<Peak>, entries: &[Stability; 3]) {
    for entry in entries
        .iter()
        .filter(|s| s.static_margin.margin_cal.is_none())
    {
        let slope = entry.static_margin.pitch_moment_slope_per_rad;
        if slot.is_none_or(|peak| !peak.value.is_nan() && (slope.is_nan() || slope > peak.value)) {
            *slot = Some(Peak {
                value: slope,
                time_s: entry.time_s,
                height_above_ground_m: entry.height_above_ground_m,
            });
        }
    }
}

/// The least of one margin on the step `[a, b]`, from its `entries` at the start, middle and end
/// and, when all three are defined and the parabola through them has its bottom inside, a search
/// between; `None` if it is nowhere defined among them.
fn least_margin(
    step: &dyn FlightStep,
    a: f64,
    b: f64,
    entries: &[Stability; 3],
    pick: fn(&Stability) -> &Margin,
) -> Result<Option<Peak>, SimError> {
    let at = |s: &Stability| {
        pick(s).margin_cal.map(|value| Peak {
            value,
            time_s: s.time_s,
            height_above_ground_m: s.height_above_ground_m,
        })
    };
    let mut best: Option<Peak> = None;
    let mut keep = |candidate: Peak| {
        if best.is_none_or(|peak| replaces(candidate.value, peak.value)) {
            best = Some(candidate);
        }
    };
    let points = entries.each_ref().map(at);
    points.into_iter().flatten().for_each(&mut keep);
    if let [Some(start), Some(middle), Some(end)] = points
        && top_inside(-start.value, -middle.value, -end.value)
    {
        // Searched as the largest of its negative; an undefined margin is never the least.
        let found = golden_max(a, b, |t| {
            step.stability(t).map(|s| {
                let peak = at(&s);
                Peak {
                    value: peak.map_or(f64::NEG_INFINITY, |p| -p.value),
                    time_s: s.time_s,
                    height_above_ground_m: s.height_above_ground_m,
                }
            })
        })?;
        if found.value.is_finite() {
            keep(Peak {
                value: -found.value,
                ..found
            });
        }
    }
    Ok(best)
}

/// The ejection delay that would fire a motor's charge at apogee.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct OptimumDelay {
    /// The motor, by its index in [`hpr_design::Assembly::motors`].
    pub motor: usize,
    /// When it burned out, s after launch.
    pub burnout_s: f64,
    /// When the rocket reached apogee with its charges held, s after launch.
    pub apogee_s: f64,
    /// How high its center of mass was then, m above the launch site.
    pub apogee_height_above_ground_m: f64,
    /// The delay from its burnout to that apogee, s.
    pub delay_s: f64,
}

/// The optimum ejection delay of each motor that burns out before apogee: the time from its
/// burnout to the apogee of the same flight with every recovery charge held, so that the answer
/// is a property of the rocket, its motors and its air, and not of the delay flown
/// ([Loft lesson L94][l94]).
///
/// - The held flight holds the stack's devices and any separation with nothing ahead of it left
///   to burn, which is part of the recovery, and a mass shift or release fired by a motor's delay,
///   whose charge is held. A powered separation still happens, and so does a shift or release
///   on any other trigger.
/// - A motor that burns out after that apogee, or never lights, has none. Nor does a motor in a
///   body a powered separation drops: its charge fires in that body, which never reaches the
///   stack's apogee.
///
/// It is `None` when the held flight has no apogee (it never lifted off, or hit its time cap).
///
/// # Errors
///
/// The flight's.
///
/// [l94]: https://hpr.fusionspace.co/decisions-and-roadmap.html#l94
pub fn optimum_delays(simulation: &Simulation) -> Result<Option<Vec<OptimumDelay>>, SimError> {
    let held = simulation.with_recovery_held();
    let result = held.run(&mut ())?;
    let Some((apogee_s, apogee_height_above_ground_m)) = result
        .event(EventKind::Apogee)
        .map(|event| (event.sample.time_s, event.sample.height_above_ground_m))
    else {
        return Ok(None);
    };
    let assembly = simulation.assembly();
    // A motor lit by a separation has its time only from the flight's ignition event.
    let known = assembly.ignition_times_s(|_| None);
    let dropped = |stage: usize| {
        result
            .bodies
            .iter()
            .any(|body| (body.stages.0..=body.stages.1).contains(&stage))
    };
    Ok(Some(
        assembly
            .motors
            .iter()
            .enumerate()
            .filter(|(_, placed)| !dropped(placed.stage))
            .filter_map(|(motor, placed)| {
                let ignition_s = result
                    .event(EventKind::Ignition(motor))
                    .map(|event| event.sample.time_s)
                    .or(known.get(motor).copied().flatten())?;
                let burnout_s = ignition_s + placed.mounted.motor.burnout_time_s();
                (burnout_s <= apogee_s).then_some(OptimumDelay {
                    motor,
                    burnout_s,
                    apogee_s,
                    apogee_height_above_ground_m,
                    delay_s: apogee_s - burnout_s,
                })
            })
            .collect(),
    ))
}

#[cfg(test)]
mod tests {
    use hpr_aero::{AeroModel, Flow};
    use hpr_atmos::ConstantWind;
    use hpr_design::Rocket;
    use serde_json::json;

    use super::*;
    use crate::envelope::HIGH_ANGLE_MIN_FORCE_SHARE;
    use crate::flight::FlightSettings;
    use crate::integrator::{Adaptive, Method};
    use crate::rail::Rail;
    use crate::recorder::{Channel, Recorder};
    use crate::recovery::{Device, DeviceDrag, Trigger};
    use crate::testing::{UniformAir, analytic_environment, design, site, windy_environment};

    const G: f64 = 9.806_65;

    fn valetudo(environment: Environment, settings: FlightSettings) -> Simulation {
        Simulation::new(
            &design("rocketpy-valetudo"),
            "example",
            environment,
            Rail::vertical(3.0),
            settings,
        )
        .unwrap()
    }

    fn capped(max_time_s: f64) -> FlightSettings {
        FlightSettings {
            max_time_s,
            ..FlightSettings::default()
        }
    }

    fn fly(simulation: &Simulation) -> (FlightResult, FlightMetrics) {
        let mut metrics = FlightMetrics::new();
        let result = simulation.run(&mut metrics).unwrap();
        (result, metrics)
    }

    /// A conical nose `0.3` m long of radius `R = 0.05` m, a `0.5` m tube, and a conical boattail
    /// `0.4` m long down to `r_m`, with no fins: Barrowman gives the nose a slope of 2 at
    /// `2/3` of its length and the boattail `2((r/R)² − 1)` at `L/3 · (1 + 1/(1 + R/r))` aft of
    /// its fore end (Barrowman's eq. 44, `hpr_aero`'s hand values, L89).
    fn nose_and_boattail(r_m: f64) -> AeroModel {
        let material = json!({"name": "test", "density": {"kind": "bulk", "kg_m3": 1000.0}});
        let rocket: Rocket = serde_json::from_value(json!({
            "name": "",
            "stages": [{
                "id": "stage",
                "name": "",
                "components": [
                    {"id": "nose", "name": "", "part": {"nose_cone": {
                        "shape": {"kind": "conical"}, "length_m": 0.3, "base_radius_m": 0.05,
                        "wall": {"kind": "filled"}, "shoulder": null, "material": material}}},
                    {"id": "tube", "name": "", "part": {"body_tube": {
                        "length_m": 0.5, "outer_radius_m": 0.05, "thickness_m": 0.005,
                        "material": material}}},
                    {"id": "boattail", "name": "", "part": {"transition": {
                        "shape": {"kind": "conical"}, "length_m": 0.4, "fore_radius_m": 0.05,
                        "aft_radius_m": r_m, "wall": {"kind": "filled"}, "material": material}}}
                ]
            }],
            "reference_diameter": {"kind": "maximum"},
            "configurations": []
        }))
        .unwrap();
        AeroModel::new(&rocket.layout().unwrap()).unwrap()
    }

    /// The hand values of [`nose_and_boattail`]: `(slope, Σ |slopes|, C_mα about x_cg, x_cp)`.
    fn nose_and_boattail_by_hand(r_m: f64, cg_m: f64) -> (f64, f64, f64, f64) {
        let (big_r, d) = (0.05, 0.1);
        let (nose_slope, nose_x) = (2.0, 0.3 * 2.0 / 3.0);
        let tail_slope = 2.0 * ((r_m / big_r).powi(2) - 1.0);
        let tail_x = 0.8 + 0.4 / 3.0 * (1.0 + 1.0 / (1.0 + big_r / r_m));
        let slope = nose_slope + tail_slope;
        let moment = -(nose_slope * (nose_x - cg_m) + tail_slope * (tail_x - cg_m)) / d;
        let cp = (nose_slope * nose_x + tail_slope * tail_x) / slope;
        (slope, nose_slope + tail_slope.abs(), moment, cp)
    }

    fn close(got: f64, want: f64, rel: f64, what: &str) {
        let err = ((got - want) / want).abs();
        assert!(
            err <= rel,
            "{what}: got {got}, want {want}, rel err {err:e}"
        );
    }

    #[test]
    fn static_margin_undefined_when_cn_alpha_near_zero() {
        // L33: a boattail down to a tenth of the radius all but cancels the nose's slope
        // (2 − 1.98 = 0.02 against a sum of 3.98). The quotient still exists, and it is the kind
        // of number Loft published: hundreds of calibres.
        let cg_m = 0.5;
        let aero = nose_and_boattail(0.005);
        let (slope, sum, moment, cp) = nose_and_boattail_by_hand(0.005, cg_m);
        let raw = aero.normal_force(&Flow::axial(0.0)).unwrap();
        close(raw.slope_per_rad, slope, 1e-9, "net slope");
        close(raw.cp_station_m.unwrap(), cp, 1e-9, "the quotient");
        assert!(
            ((cp - cg_m) / 0.1).abs() > 100.0,
            "a margin of {} cal",
            (cp - cg_m) / 0.1
        );
        let static_margin = stability(&aero, 0.0, 0.0, 0.0, cg_m, 0.3)
            .unwrap()
            .static_margin;
        assert_eq!(static_margin.margin_cal, None);
        assert_eq!(static_margin.cp_station_m, None);
        close(
            static_margin.slope_magnitude_sum_per_rad,
            sum,
            1e-12,
            "Σ |slopes|",
        );
        // The couple stays: finite, and destabilising (positive).
        close(
            static_margin.pitch_moment_slope_per_rad,
            moment,
            1e-9,
            "C_mα",
        );
        assert!(moment > 0.0);

        // Both sides of the limit: κ = Σ|C_Nα,i| / C_Nα = 2/ρ² − 1 with ρ = r/R, so the margin
        // is given from κ = √10, ρ = √(2/(1 + √10)) = 0.6932, up.
        for (r_m, defined) in [
            (0.035, true),
            (0.0345, false),
            (0.04, true),
            (0.0215, false),
        ] {
            let (slope, sum, moment, cp) = nose_and_boattail_by_hand(r_m, cg_m);
            assert_eq!(sum / slope <= MARGIN_CONDITION_LIMIT, defined, "r = {r_m}");
            let m = margin(&nose_and_boattail(r_m), &Flow::axial(0.0), cg_m).unwrap();
            close(m.pitch_moment_slope_per_rad, moment, 1e-9, "C_mα");
            if defined {
                close(m.margin_cal.unwrap(), (cp - cg_m) / 0.1, 1e-9, "margin");
                // C_mα = −C_Nα · margin.
                close(
                    m.pitch_moment_slope_per_rad,
                    -slope * m.margin_cal.unwrap(),
                    1e-9,
                    "C_mα",
                );
            } else {
                assert_eq!(m.margin_cal, None, "r = {r_m}");
            }
        }
    }

    /// A rocket with one fin at 0° and a pair at `pair_rad`: the pair is edge-on to air crossing
    /// in its own plane, and the lone fin to air crossing in its. A fin set of one or two fins
    /// carries `Σ sin²(φ − θ_k)` of its force in the plane at `φ` (Niskanen 2009 eq. 3.51).
    fn uneven_fins(single_m: f64, pair_m: f64, pair_rad: f64) -> AeroModel {
        let material = json!({"name": "test", "density": {"kind": "bulk", "kg_m3": 1000.0}});
        let fins = |id: &str, count: u32, angle_rad: f64, span_m: f64| {
            json!({"id": id, "name": "", "part": {"fin_set": {
                "count": count, "base_angle_rad": angle_rad,
                "planform": {"kind": "trapezoidal", "root_chord_m": 0.12, "tip_chord_m": 0.06,
                    "span_m": span_m, "sweep_m": 0.05},
                "thickness_m": 0.003, "cross_section": "rounded", "tab": null,
                "cant_rad": 0.0, "material": material}},
                "position": {"from": "bottom", "aft_offset_m": 0.0}})
        };
        let rocket: Rocket = serde_json::from_value(json!({
            "name": "",
            "stages": [{"id": "stage", "name": "", "components": [
                {"id": "nose", "name": "", "part": {"nose_cone": {
                    "shape": {"kind": "conical"}, "length_m": 0.2, "base_radius_m": 0.025,
                    "wall": {"kind": "filled"}, "shoulder": null, "material": material}}},
                {"id": "tube", "name": "", "part": {"body_tube": {"length_m": 0.6,
                    "outer_radius_m": 0.025, "thickness_m": 0.001, "material": material}},
                    "children": [
                        fins("single", 1, 0.0, single_m),
                        fins("pair", 2, pair_rad, pair_m)
                    ]}
            ]}],
            "reference_diameter": {"kind": "maximum"},
            "configurations": []
        }))
        .unwrap();
        AeroModel::new(&rocket.layout().unwrap()).unwrap()
    }

    /// The least margin over the direction the air crosses the rocket (#329): never above any
    /// plane's, met in a plane a fine scan can't beat, and found in closed form wherever it is.
    /// A rocket whose fin sets are each of three or more keeps its axial margin bit for bit.
    #[test]
    fn the_least_margin_is_the_weakest_plane_s() {
        let scan = |aero: &AeroModel, mach: f64, cg_m: f64| {
            (0..3600)
                .map(|k| {
                    let roll = std::f64::consts::PI * f64::from(k) / 3600.0;
                    margin(aero, &Flow::new(mach, 0.0, roll), cg_m).unwrap()
                })
                .collect::<Vec<_>>()
        };
        // Spans either way round, and centers of mass either side of a margin that vanishes in
        // some planes. With the pair at right angles to the lone fin every sum is even in `φ`
        // (`c = 0`), and the weak plane lies on a fin's plane; with it at 0.6 rad the weak plane
        // lies between them, where a wrong sign on the `sin 2φ` terms would land on a strong one.
        let right = std::f64::consts::FRAC_PI_2;
        let mut off_fins = 0;
        for (single_m, pair_m, pair_rad) in [
            (0.06, 0.03, right),
            (0.03, 0.06, right),
            (0.05, 0.05, right),
            (0.07, 0.02, right),
            (0.06, 0.03, 0.6),
            (0.03, 0.06, 0.6),
        ] {
            let aero = uneven_fins(single_m, pair_m, pair_rad);
            assert!(aero.rolls());
            for mach in [0.1, 0.6, 1.5] {
                for cg_m in [0.35, 0.45, 0.55] {
                    let least = weakest_margin(&aero, mach, cg_m).unwrap();
                    let planes = scan(&aero, mach, cg_m);
                    let at = margin(&aero, &Flow::new(mach, 0.0, least.roll_rad), cg_m).unwrap();
                    assert_eq!(at, least, "the least margin is its plane's");
                    // Distance, in the margin's period of π, to the nearest plane of a fin or
                    // square to one.
                    let off = [0.0, right, pair_rad, pair_rad + right]
                        .iter()
                        .map(|plane| {
                            let d = (least.roll_rad - plane).rem_euclid(std::f64::consts::PI);
                            d.min(std::f64::consts::PI - d)
                        })
                        .fold(f64::INFINITY, f64::min);
                    if off > 0.05 {
                        off_fins += 1;
                    }
                    match least.margin_cal {
                        None => assert!(
                            planes.iter().any(|m| m.margin_cal.is_none()) || {
                                // The plane where no margin can be given lies between two scan lines.
                                let near = |m: &Margin| {
                                    m.slope_magnitude_sum_per_rad
                                        / m.normal_force_slope_per_rad.max(f64::MIN_POSITIVE)
                                };
                                planes.iter().map(near).fold(0.0, f64::max)
                                    > 0.99 * MARGIN_CONDITION_LIMIT
                            }
                        ),
                        Some(cal) => {
                            for m in &planes {
                                let plane = m.margin_cal.expect("a margin in every plane");
                                assert!(
                                    cal <= plane + 1e-12,
                                    "{cal} above {plane} at {}",
                                    m.roll_rad
                                );
                            }
                            let scanned = planes
                                .iter()
                                .filter_map(|m| m.margin_cal)
                                .fold(f64::INFINITY, f64::min);
                            assert!(scanned - cal < 1e-5, "{cal} against the scan's {scanned}");
                        }
                    }
                }
            }
        }
        assert!(off_fins > 0, "no weak plane off the fins' planes");
        // With the pair the larger, the axial plane, where it works whole, is the strong one: the
        // reported margin is well below it.
        let aero = uneven_fins(0.03, 0.06, right);
        let axial = margin(&aero, &Flow::axial(0.3), 0.45)
            .unwrap()
            .margin_cal
            .unwrap();
        let least = weakest_margin(&aero, 0.3, 0.45)
            .unwrap()
            .margin_cal
            .unwrap();
        assert!(least < axial - 0.5, "{least} against {axial}");
        // Even fins: the axial margin, bit for bit.
        for name in [
            "synthetic-54mm-three-fin",
            "rocketpy-valetudo",
            "rocketpy-juno-iii",
        ] {
            let aero = AeroModel::new(&design(name).layout().unwrap()).unwrap();
            assert!(!aero.rolls(), "{name}");
            for mach in [0.0, 0.3, 1.2] {
                assert_eq!(
                    weakest_margin(&aero, mach, 1.0).unwrap(),
                    margin(&aero, &Flow::axial(mach), 1.0).unwrap(),
                    "{name}"
                );
            }
        }
    }

    #[test]
    fn ordinary_rockets_keep_their_margin() {
        // The other side of the limit: every design in `validation/designs/`, at Mach 0 to 2 and
        // angles of attack of 0° to 20° (the points below), is far from it. Its slopes nearly all push one way.
        let mut worst: f64 = 0.0;
        for name in [
            "synthetic-54mm-three-fin",
            "rocketpy-valetudo",
            "rocketpy-calisto-tests-motor-at-minus-1.373",
            "rocketpy-ndrt-2020-nose-to-tail",
            "rocketpy-prometheus-2022-generic-motor",
            "rocketpy-juno-iii",
            "synthetic-two-stage-75mm-54mm",
            "mil-hdbk-762-sample-rocket",
            "rocketpy-bella-lui",
            "rocketpy-calisto-getting-started-motor-at-minus-1.255",
            "rocketpy-cavour",
            "wind-tunnel-arcas-robin-long",
            "wind-tunnel-arcas-robin-short",
        ] {
            let aero = AeroModel::new(&design(name).layout().unwrap()).unwrap();
            for mach in [0.0, 0.3, 0.8, 1.2, 2.0] {
                for alpha_deg in [0.0, 5.0, 10.0, 20.0] {
                    let flow = Flow::new(mach, f64::to_radians(alpha_deg), 0.0);
                    let m = margin(&aero, &flow, 0.0).unwrap();
                    assert!(m.margin_cal.is_some(), "{name} at {mach}, {alpha_deg}°");
                    worst = worst.max(m.slope_magnitude_sum_per_rad / m.normal_force_slope_per_rad);
                }
            }
        }
        // 1.35 at worst.
        assert!(worst < 1.5, "{worst}");
    }

    #[test]
    fn a_pure_couple_keeps_its_moment() {
        // A nose (slope 2 at 0.2 m), then a step down from R = 0.05 m to 0.03 m at 0.8 m and a
        // conical flare back to 0.05 m over 0.2 m: the step's −2(1 − 0.36) = −1.28 at 0.8 m and
        // the flare's +1.28 at 0.8 + (0.2/3)(1 + 1/1.6) m cancel, leaving that component a pure
        // couple. It still turns the rocket.
        let material = json!({"name": "test", "density": {"kind": "bulk", "kg_m3": 1000.0}});
        let tube = |length_m: f64| {
            json!({"body_tube": {"length_m": length_m, "outer_radius_m": 0.05,
                "thickness_m": 0.005, "material": material}})
        };
        let rocket: Rocket = serde_json::from_value(json!({
            "name": "",
            "stages": [{"id": "stage", "name": "", "components": [
                {"id": "nose", "name": "", "part": {"nose_cone": {
                    "shape": {"kind": "conical"}, "length_m": 0.3, "base_radius_m": 0.05,
                    "wall": {"kind": "filled"}, "shoulder": null, "material": material}}},
                {"id": "tube", "name": "", "part": tube(0.5)},
                {"id": "flare", "name": "", "part": {"transition": {
                    "shape": {"kind": "conical"}, "length_m": 0.2, "fore_radius_m": 0.03,
                    "aft_radius_m": 0.05, "wall": {"kind": "filled"}, "material": material}}},
                {"id": "tail", "name": "", "part": tube(0.3)}
            ]}],
            "reference_diameter": {"kind": "maximum"},
            "configurations": []
        }))
        .unwrap();
        let aero = AeroModel::new(&rocket.layout().unwrap()).unwrap();
        let cg_m = 0.6;
        let couple = -1.28 * 0.8 + 1.28 * (0.8 + 0.2 / 3.0 * (1.0 + 1.0 / 1.6));
        let m = margin(&aero, &Flow::axial(0.0), cg_m).unwrap();
        close(m.normal_force_slope_per_rad, 2.0, 1e-12, "net slope");
        let cp = (2.0 * 0.2 + couple) / 2.0;
        close(m.margin_cal.unwrap(), (cp - cg_m) / 0.1, 1e-9, "margin");
        let moment = -(2.0 * 0.2 + couple - 2.0 * cg_m) / 0.1;
        close(m.pitch_moment_slope_per_rad, moment, 1e-9, "C_mα");
        close(
            m.pitch_moment_slope_per_rad,
            -2.0 * m.margin_cal.unwrap(),
            1e-12,
            "−C_Nα · margin",
        );
    }

    #[test]
    fn a_normal_force_table_gives_its_own_margin() {
        // A table of one column: C_Nα = 10 on the rocket's reference area, at 1.2 m at every Mach
        // number. The margin is the table's, and κ is 1.
        use hpr_aero::{NormalForceColumn, NormalForceTable};
        use hpr_core::interp::{Extrapolation, Interpolation, Table1D};
        let constant = |y: f64| {
            Table1D::new(
                vec![0.0, 2.0],
                vec![y, y],
                Interpolation::Linear,
                Extrapolation::Clamp,
            )
            .unwrap()
        };
        let table = NormalForceTable::new(vec![NormalForceColumn::new(
            0.0,
            constant(10.0),
            constant(1.2),
        )])
        .unwrap();
        let sim = valetudo(Environment::standard(site()).unwrap(), capped(60.0))
            .with_normal_force_table(table)
            .unwrap();
        let d = sim.aero().reference_diameter_m();
        let m = margin(sim.aero(), &Flow::axial(0.3), 0.8).unwrap();
        close(m.normal_force_slope_per_rad, 10.0, 1e-12, "slope");
        assert_eq!(m.slope_magnitude_sum_per_rad, m.normal_force_slope_per_rad);
        close(m.margin_cal.unwrap(), (1.2 - 0.8) / d, 1e-12, "margin");
        close(
            m.pitch_moment_slope_per_rad,
            -10.0 * (1.2 - 0.8) / d,
            1e-12,
            "C_mα",
        );
    }

    #[test]
    fn peak_acceleration_is_analytic_and_excludes_opening_shock() {
        // L34, first half: a vertical boost in a vacuum under uniform gravity, with no rotation.
        // The nose tip's acceleration is (T − m r″ − 2ṁ r′ + m̈(n − r))/m − g along the axis, from
        // the motor and the assembly directly (as the powered-climb test integrates it). Its
        // largest value on a fine scan, and at every thrust-curve knot from both sides, is the
        // peak.
        let sim = valetudo(analytic_environment(UniformAir::vacuum(), G), capped(4.0));
        let (_, metrics) = fly(&sim);
        let assembly = sim.assembly();
        let placed = &assembly.motors[0];
        let motor = &placed.mounted.motor;
        let burnout_s = motor.burnout_time_s();
        let h = 1e-5;
        let mut knots: Vec<f64> = motor
            .curve()
            .times_s()
            .iter()
            .copied()
            .filter(|t| *t > 0.0 && *t < burnout_s)
            .collect();
        knots.insert(0, 0.0);
        knots.push(burnout_s);
        let props = |t: f64| assembly.mass_properties(t);
        // On the knot interval [a, b], derivatives taken inside it and extended to t.
        let by_hand = |t: f64, a: f64, b: f64| {
            let c = t.clamp(a + h, b - h);
            let (m, r) = (props(t).mass_kg, props(t).cg_m.z);
            let r_mid = props(c).cg_m.z;
            let r2 = (props(c + h).cg_m.z - 2.0 * r_mid + props(c - h).cg_m.z) / (h * h);
            let r1 = (props(c + h).cg_m.z - props(c - h).cg_m.z) / (2.0 * h) + (t - c) * r2;
            let mdot = -motor.state(t).mass_flow_kg_s;
            let mddot = -(motor.state(c + h).mass_flow_kg_s - motor.state(c - h).mass_flow_kg_s)
                / (2.0 * h);
            let thrust = motor.thrust_at_pressure_n(t, 0.0);
            (thrust - m * r2 - 2.0 * mdot * r1 + mddot * (placed.nozzle_m.z - r)) / m - G
        };
        let mut expected: f64 = 0.0;
        for pair in knots.windows(2) {
            let (a, b) = (pair[0], pair[1]);
            let n = 2000;
            for i in 0..=n {
                let t = a + (b - a) * f64::from(i) / f64::from(n);
                expected = expected.max(by_hand(t, a, b));
            }
        }
        let peak = metrics.max_acceleration.unwrap();
        close(peak.value, expected, 1e-6, "peak acceleration");
        assert!(peak.time_s < burnout_s);
        // What Loft did: a finite difference of the speed recorded at 100 Hz, which averages the
        // acceleration over each interval and reads the thrust spike low.
        let mut recorder =
            Recorder::new(vec![Channel::Time, Channel::Velocity], Some(0.01)).unwrap();
        sim.run(&mut recorder).unwrap();
        let rows = recorder.rows();
        let differenced = rows
            .windows(2)
            .map(|w| (w[1][3] - w[0][3]) / (w[1][0] - w[0][0]))
            .fold(0.0, f64::max);
        // 1.3% low on this motor's first ramp.
        assert!(
            differenced < 0.99 * peak.value,
            "{differenced} vs {}",
            peak.value
        );

        // Second half: the same boost in air, with a large canopy opening at once while the rocket
        // still climbs fast. The boost's peak is the flight's without the canopy, and the opening
        // shock, several times larger, is kept apart.
        let air = || analytic_environment(UniformAir::sea_level(), G);
        let (_, plain) = fly(&valetudo(air(), capped(60.0)));
        let canopy = Device::new(
            "main",
            DeviceDrag::DragArea { cd_s_m2: 4.0 },
            Trigger::Time { time_s: 4.0 },
        );
        let early = valetudo(air(), capped(60.0))
            .with_recovery(vec![canopy])
            .unwrap();
        let (result, shocked) = fly(&early);
        let deployed = result.event(EventKind::Deployment(0)).unwrap().sample;
        assert!(deployed.airspeed_m_s > 100.0, "{}", deployed.airspeed_m_s);
        let boost = shocked.max_acceleration.unwrap();
        close(
            boost.value,
            plain.max_acceleration.unwrap().value,
            1e-12,
            "boost peak",
        );
        let shock = shocked.max_descent_acceleration.unwrap();
        assert!(
            shock.value > 3.0 * boost.value,
            "{} vs {}",
            shock.value,
            boost.value
        );
        assert_eq!(shock.time_s, deployed.time_s);
        assert_eq!(plain.max_descent_acceleration, None);
    }

    #[test]
    fn unlanded_flight_has_no_ground_hit_speed_and_outputs_name_datum() {
        // L35: a flight stopped by its time cap at 5 s has no landing, no ground-hit speed and no
        // apogee, and says so with `None`, not a zero.
        let environment = || Environment::standard(site()).unwrap();
        let sim = valetudo(environment(), capped(5.0));
        let (result, metrics) = fly(&sim);
        assert_eq!(result.termination, Termination::TimeCap);
        let summary = metrics.summary(&result, sim.environment()).unwrap();
        assert_eq!(summary.landing, None);
        assert_eq!(summary.ground_hit_speed_m_s(), None);
        assert_eq!(summary.apogee, None);
        assert_eq!(summary.max_descent_acceleration_m_s2, None);
        let json = serde_json::to_value(&summary).unwrap();
        assert_eq!(json["landing"], serde_json::Value::Null);
        assert_eq!(json["apogee"], serde_json::Value::Null);

        // The whole flight: the heights name their datum. The center of mass starts above the
        // site, the apogee's gain counts from there, and the landing is on the site's ellipsoidal
        // height, where the flight stops.
        let sim = valetudo(environment(), capped(600.0));
        let (result, metrics) = fly(&sim);
        let summary = metrics.summary(&result, sim.environment()).unwrap();
        // On a vertical rail the nose tip starts at the aft guide's station above the site, and
        // the center of mass that far less its own station.
        let start = summary.launch_height_m.unwrap();
        let cg_station_m = -sim.assembly().mass_properties(0.0).cg_m.z;
        close(
            start,
            sim.guides().aft_station_m - cg_station_m,
            1e-9,
            "start",
        );
        let apogee = summary.apogee.unwrap();
        assert_eq!(apogee.gain_m, Some(apogee.height_above_ground_m - start));
        let landing = summary.landing.unwrap();
        assert_eq!(
            summary.ground_hit_speed_m_s(),
            Some(landing.ground_hit_speed_m_s)
        );
        assert!(result.final_sample.height_above_ground_m.abs() < 1e-6);
        let json = serde_json::to_value(&summary).unwrap();
        for field in ["launch_height_m", "apogee"] {
            assert!(
                json[field].is_number() || json[field].is_object(),
                "{field}"
            );
        }
        assert!(json["apogee"]["height_above_ground_m"].is_number());
        assert!(json["apogee"]["gain_m"].is_number());
    }

    #[test]
    fn optimum_delay_independent_of_flown_delay() {
        // L94: a delay of 1 s opens the canopy while the rocket climbs; one of 20 s opens it long
        // after apogee. The optimum is the same for both, and is the time from burnout to the
        // apogee of a flight with no recovery at all.
        let environment = || Environment::standard(site()).unwrap();
        let flown = |delay_s: f64| {
            valetudo(environment(), capped(600.0))
                .with_recovery(vec![Device::new(
                    "main",
                    DeviceDrag::DragArea { cd_s_m2: 1.0 },
                    Trigger::Burnout { motor: 0, delay_s },
                )])
                .unwrap()
        };
        let (early, late) = (flown(1.0), flown(20.0));
        let bare = valetudo(environment(), capped(600.0));
        let bare_apogee_s = bare
            .run(&mut ())
            .unwrap()
            .event(EventKind::Apogee)
            .unwrap()
            .sample
            .time_s;
        let early_result = early.run(&mut ()).unwrap();
        let opened_s = early_result
            .event(EventKind::Deployment(0))
            .unwrap()
            .sample
            .time_s;
        assert!(
            opened_s < bare_apogee_s - 5.0,
            "{opened_s} vs {bare_apogee_s}"
        );

        let a = optimum_delays(&early).unwrap().unwrap();
        let b = optimum_delays(&late).unwrap().unwrap();
        assert_eq!(a, b);
        assert_eq!(a.len(), 1);
        let burnout_s = bare.assembly().motors[0].mounted.motor.burnout_time_s();
        assert_eq!(a[0].burnout_s, burnout_s);
        close(a[0].apogee_s, bare_apogee_s, 1e-9, "apogee");
        close(a[0].delay_s, bare_apogee_s - burnout_s, 1e-9, "delay");
        // It doesn't touch the simulation it was given.
        assert_eq!(early.run(&mut ()).unwrap(), early_result);
    }

    #[test]
    fn peaks_are_refined_inside_steps() {
        // Max q and max Mach come between the integrator's steps, some in a step's outer quarter,
        // where its middle sample is below an end. On each rocket every peak is at least every
        // sample of a millisecond record, and within the record's spacing of the best.
        for (name, configuration) in [
            ("rocketpy-valetudo", "example"),
            ("rocketpy-juno-iii", "example"),
            ("rocketpy-calisto-tests-motor-at-minus-1.373", "example"),
        ] {
            let sim = Simulation::new(
                &design(name),
                configuration,
                Environment::standard(site()).unwrap(),
                Rail::vertical(5.0),
                capped(20.0),
            )
            .unwrap();
            let mut recorder = Recorder::new(
                vec![Channel::Time, Channel::DynamicPressure, Channel::Mach],
                Some(1e-3),
            )
            .unwrap();
            sim.run(&mut recorder).unwrap();
            let (result, metrics) = fly(&sim);
            let summary = metrics.summary(&result, sim.environment()).unwrap();
            for (column, peak) in [
                (1, summary.max_dynamic_pressure_pa.unwrap()),
                (2, summary.max_mach.unwrap()),
            ] {
                let recorded = recorder
                    .rows()
                    .iter()
                    .map(|row| row[column])
                    .fold(0.0, f64::max);
                assert!(
                    peak.value >= recorded,
                    "{name}: {} < {recorded}",
                    peak.value
                );
                assert!(
                    peak.value - recorded < 1e-4 * peak.value,
                    "{name}: {}",
                    peak.value
                );
            }
            if name == "rocketpy-valetudo" {
                // At the speed's peak the acceleration is zero, so q = ½ρv² falls there with the
                // density (dq/dt = ½v² dρ/dt < 0) and peaked earlier; the Mach number still rises
                // with the falling speed of sound, and peaks later.
                let (q, mach) = (
                    summary.max_dynamic_pressure_pa.unwrap(),
                    summary.max_mach.unwrap(),
                );
                let speed = summary.max_speed_m_s.unwrap();
                assert!(q.time_s < speed.time_s, "{} vs {}", q.time_s, speed.time_s);
                assert!(
                    speed.time_s < mach.time_s,
                    "{} vs {}",
                    speed.time_s,
                    mach.time_s
                );
                assert!(q.height_above_ground_m > 0.0);
            }
        }
    }

    #[test]
    fn landings_are_placed_on_the_ellipsoid() {
        // A 6 m/s wind from the west carries the rocket east. Over a few hundred meters the
        // landing's latitude and longitude are the site's plus the local displacement over the
        // radii of curvature M and N at the site's height, to second order in distance/radius.
        let wind = ConstantWind::new(6.0, 1.5 * std::f64::consts::PI).unwrap();
        let canopy = Device::new(
            "main",
            DeviceDrag::DragArea { cd_s_m2: 0.5 },
            Trigger::Apogee,
        );
        let sim = Simulation::new(
            &design("rocketpy-valetudo"),
            "example",
            windy_environment(wind),
            Rail::vertical(3.0),
            capped(600.0),
        )
        .unwrap()
        .with_recovery(vec![canopy])
        .unwrap();
        let (result, metrics) = fly(&sim);
        let summary = metrics.summary(&result, sim.environment()).unwrap();
        let landing = summary.landing.unwrap();
        assert!(landing.east_m > 100.0, "{}", landing.east_m);
        let place = site();
        let (a, f) = (6_378_137.0, 1.0 / 298.257_223_563);
        let e2 = f * (2.0 - f);
        let s = place.latitude_rad.sin();
        let w = (1.0 - e2 * s * s).sqrt();
        let n = a / w + place.height_m;
        let m = a * (1.0 - e2) / (w * w * w) + place.height_m;
        let lat = place.latitude_rad + landing.north_m / m;
        let lon = place.longitude_rad + landing.east_m / (n * place.latitude_rad.cos());
        let second_order = (landing.distance_m / a).powi(2);
        assert!((landing.latitude_deg.to_radians() - lat).abs() < 10.0 * second_order);
        assert!((landing.longitude_deg.to_radians() - lon).abs() < 10.0 * second_order);
        close(
            landing.distance_m,
            landing.east_m.hypot(landing.north_m),
            1e-15,
            "distance",
        );
        assert_eq!(landing.body, None);
        assert!(landing.descent_rate_m_s > 0.0);
    }

    #[test]
    fn stability_is_kept_from_rail_exit_to_apogee() {
        let sim = valetudo(Environment::standard(site()).unwrap(), capped(600.0));
        let (result, metrics) = fly(&sim);
        let summary = metrics.summary(&result, sim.environment()).unwrap();
        let series = metrics.stability();
        let exit_s = result.event(EventKind::RailExit).unwrap().sample.time_s;
        let apogee_s = summary.apogee.unwrap().time_s;
        assert_eq!(series.first().unwrap().time_s, exit_s);
        assert_eq!(series.last().unwrap().time_s, apogee_s);
        assert!(series.windows(2).all(|w| w[0].time_s < w[1].time_s));
        // The static margin at the rail exit: the center of pressure at Mach 0 against the center
        // of mass then, from the assembly and the aerodynamic model directly.
        let first = series[0];
        let lit = sim.assembly().ignition_times_s(|_| None);
        let cg_m = -sim.assembly().mass_properties_lit(exit_s, &lit).cg_m.z;
        close(first.cg_station_m, cg_m, 1e-12, "center of mass");
        let cp_m = sim
            .aero()
            .normal_force(&Flow::axial(0.0))
            .unwrap()
            .cp_station_m
            .unwrap();
        let d = sim.aero().reference_diameter_m();
        close(
            first.static_margin.margin_cal.unwrap(),
            (cp_m - cg_m) / d,
            1e-12,
            "static margin",
        );
        assert_eq!(summary.rail_exit_stability, Some(first));
        let min = summary.min_static_margin_cal.unwrap();
        assert!(
            series
                .iter()
                .all(|s| s.static_margin.margin_cal.unwrap() >= min.value)
        );
        // The least margins are found inside steps, so they are at or below every entry.
        let least = summary.min_flight_margin_cal.unwrap();
        assert!(
            series
                .iter()
                .all(|s| s.flight_margin.margin_cal.unwrap() >= least.value)
        );
        assert!(least.value > 3.0, "{least:?}");

        // The flight margin is the model's at the flight's own Mach number with the air along the
        // axis, whatever the angle of attack: at the rail exit in a crosswind the rocket meets the
        // air more than 0.2 rad off its axis, and the margin leaves that out.
        let windy = valetudo(
            windy_environment(ConstantWind::new(5.0, 1.5 * std::f64::consts::PI).unwrap()),
            capped(600.0),
        );
        let (result, metrics) = fly(&windy);
        let summary = metrics.summary(&result, windy.environment()).unwrap();
        let exit = result.event(EventKind::RailExit).unwrap().sample;
        assert!(
            exit.angle_of_attack_rad > 0.2,
            "{}",
            exit.angle_of_attack_rad
        );
        let cg_m = -windy
            .assembly()
            .mass_properties_lit(exit.time_s, &lit)
            .cg_m
            .z;
        let expected = margin(windy.aero(), &Flow::axial(exit.mach), cg_m).unwrap();
        let got = summary.rail_exit_stability.unwrap().flight_margin;
        assert_eq!(got.angle_of_attack_rad, 0.0);
        close(got.mach, exit.mach, 1e-12, "Mach number");
        close(
            got.margin_cal.unwrap(),
            expected.margin_cal.unwrap(),
            1e-12,
            "flight margin",
        );
    }

    #[test]
    fn least_margins_do_not_depend_on_where_steps_end() {
        // The same flight flown in steps of at most 1 ms gives the same least margins, to a
        // millionth of a calibre: Valetudo in calm air off a vertical and an 84° rail and in a
        // crosswind, and Prometheus. On all four both leasts are at the rail exit, where the rocket
        // is heaviest and its center of mass furthest aft: the search inside steps is checked on a
        // two-stage flight (`staging::tests::metrics_follow_a_powered_separation`) instead.
        let fine = |settings: FlightSettings| FlightSettings {
            method: Method::DormandPrince54(Adaptive {
                max_step_s: Some(1e-3),
                ..Adaptive::default()
            }),
            ..settings
        };
        let tilted = Rail {
            elevation_rad: 84.0_f64.to_radians(),
            ..Rail::vertical(2.0)
        };
        let calm = || Environment::standard(site()).unwrap();
        let wind =
            || windy_environment(ConstantWind::new(5.0, 1.5 * std::f64::consts::PI).unwrap());
        let cases = [
            ("rocketpy-valetudo", calm(), Rail::vertical(3.0), 15.0),
            ("rocketpy-valetudo", calm(), tilted, 15.0),
            ("rocketpy-valetudo", wind(), Rail::vertical(3.0), 15.0),
            (
                "rocketpy-prometheus-2022-generic-motor",
                calm(),
                Rail::vertical(5.0),
                60.0,
            ),
        ];
        for (name, environment, rail, max_time_s) in cases {
            let least = |settings: FlightSettings| {
                let sim = Simulation::new(
                    &design(name),
                    "example",
                    environment.clone(),
                    rail,
                    settings,
                )
                .unwrap();
                let (result, metrics) = fly(&sim);
                let summary = metrics.summary(&result, sim.environment()).unwrap();
                let series = metrics.stability().to_vec();
                (
                    summary.min_static_margin_cal.unwrap(),
                    summary.min_flight_margin_cal.unwrap(),
                    series,
                )
            };
            let (coarse_static, coarse_flight, series) = least(capped(max_time_s));
            assert_eq!(coarse_flight.time_s, series[0].time_s, "{name}");
            assert_eq!(coarse_static.time_s, series[0].time_s, "{name}");
            let (fine_static, fine_flight, _) = least(fine(capped(max_time_s)));
            for (coarse, fine, what) in [
                (coarse_static, fine_static, "static"),
                (coarse_flight, fine_flight, "flight"),
            ] {
                assert!(
                    (coarse.value - fine.value).abs() < 1e-6,
                    "{name} {what}: {coarse:?} against {fine:?}"
                );
            }
        }
    }

    /// A step whose only use is its margins: both are `margin_cal(t)`.
    struct MarginStep {
        margin_cal: fn(f64) -> Option<f64>,
    }

    impl FlightStep for MarginStep {
        fn phase(&self) -> Phase {
            Phase::Free
        }
        fn start_s(&self) -> f64 {
            0.0
        }
        fn end_s(&self) -> f64 {
            1.0
        }
        fn state_at(&self, _t_s: f64) -> crate::state::State {
            unreachable!("the margin search reads only the stability")
        }
        fn sample(&self, _t_s: f64) -> Result<Sample, SimError> {
            unreachable!("the margin search reads only the stability")
        }
        fn stability(&self, t_s: f64) -> Result<Stability, SimError> {
            let margin_cal = (self.margin_cal)(t_s);
            let margin = Margin {
                mach: 0.0,
                angle_of_attack_rad: 0.0,
                roll_rad: 0.0,
                normal_force_slope_per_rad: 1.0,
                slope_magnitude_sum_per_rad: 1.0,
                pitch_moment_slope_per_rad: -margin_cal.unwrap_or(0.0),
                cp_station_m: margin_cal,
                margin_cal,
            };
            Ok(Stability {
                time_s: t_s,
                height_above_ground_m: 100.0 * t_s,
                dynamic_pressure_pa: 0.0,
                cg_station_m: 0.0,
                reference_diameter_m: 1.0,
                static_margin: margin,
                flight_margin: margin,
            })
        }
    }

    fn least_of(margin_cal: fn(f64) -> Option<f64>) -> Option<Peak> {
        let step = MarginStep { margin_cal };
        let entries = [0.0, 0.5, 1.0].map(|t| step.stability(t).unwrap());
        least_margin(&step, 0.0, 1.0, &entries, flight_of).unwrap()
    }

    #[test]
    fn a_least_margin_between_step_ends_is_found() {
        // 2 + (t − 0.37)² reads 2.137, 2.017 and 2.397 at the step's start, middle and end; the
        // parabola through them has its bottom inside, and the search finds 2 at 0.37 s.
        let least = least_of(|t| Some(2.0 + (t - 0.37).powi(2))).unwrap();
        close(least.value, 2.0, 1e-15, "least");
        assert!((least.time_s - 0.37).abs() < 1e-7, "{least:?}");
        close(least.height_above_ground_m, 37.0, 1e-5, "height");
        // Falling across the step, it is least at the end, with no search.
        let least = least_of(|t| Some(3.0 - t)).unwrap();
        assert_eq!((least.value, least.time_s), (2.0, 1.0));
        // Undefined at the middle: the least of the defined ends, and no search through the gap.
        let least = least_of(|t| (t != 0.5).then_some(2.0 + (t - 0.37).powi(2))).unwrap();
        assert_eq!(least.time_s, 0.0);
        assert_eq!(least_of(|_| None), None);
    }

    #[test]
    fn a_summary_needs_the_flight_it_watched() {
        // A watcher that saw nothing, or saw another flight too, refuses to sum one up; cleared, it
        // sums up the next. A flight started in the air has no launch height, and no climb.
        // The refusal names the steps the watcher saw.
        let refused = |summary: Result<FlightSummary, SimError>, steps: u64| match summary {
            Err(SimError::Domain { what, value }) => {
                assert!(what.starts_with("steps this watcher saw"), "{what}");
                assert_eq!(value, steps as f64);
            }
            other => panic!("{other:?}"),
        };
        let sim = valetudo(Environment::standard(site()).unwrap(), capped(20.0));
        let (result, mut metrics) = fly(&sim);
        let steps = result.stats.accepted_steps;
        refused(FlightMetrics::new().summary(&result, sim.environment()), 0);
        // The same steps, but a flight that ended before the watcher's last step did.
        let mut early = result.clone();
        early.final_sample.time_s -= 1.0;
        refused(metrics.summary(&early, sim.environment()), steps);
        let other = valetudo(Environment::standard(site()).unwrap(), capped(15.0));
        let other_result = other.run(&mut metrics).unwrap();
        refused(
            metrics.summary(&other_result, other.environment()),
            steps + other_result.stats.accepted_steps,
        );
        metrics.clear();
        let other_result = other.run(&mut metrics).unwrap();
        let summary = metrics.summary(&other_result, other.environment()).unwrap();
        assert!(summary.rail_exit_stability.is_some());

        let burnout = result.event(EventKind::Burnout).unwrap().sample;
        let mut aloft = FlightMetrics::new();
        let free = sim
            .run_free(burnout.time_s, burnout.state, &mut aloft)
            .unwrap();
        let summary = aloft.summary(&free, sim.environment()).unwrap();
        assert_eq!(summary.launch_height_m, None);
        assert_eq!(summary.apogee.unwrap().gain_m, None);
        assert_eq!(summary.rail_exit_stability, None);

        // Reused forward in time, not cleared: the first flight's steps are still counted.
        let short = valetudo(Environment::standard(site()).unwrap(), capped(5.0));
        let (short_result, mut reused) = fly(&short);
        let apogee = result.event(EventKind::Apogee).unwrap().sample;
        let free = sim
            .run_free(apogee.time_s, apogee.state, &mut reused)
            .unwrap();
        refused(
            reused.summary(&free, sim.environment()),
            short_result.stats.accepted_steps + free.stats.accepted_steps,
        );

        // Begun in the air on its way down, it keeps no stability: its apogee is behind it.
        let mut falling = FlightMetrics::new();
        let mut state = apogee.state;
        state.velocity_enu_m_s.z = -1.0;
        let free = sim.run_free(apogee.time_s, state, &mut falling).unwrap();
        falling.summary(&free, sim.environment()).unwrap();
        assert!(falling.stability().is_empty());

        // A flight can end without a step, or on a stop so close ahead that the clock just moves
        // there; it is still summed up.
        let stepless = valetudo(
            Environment::standard(site()).unwrap(),
            FlightSettings {
                step_limit: 0,
                ..capped(20.0)
            },
        );
        let (result, metrics) = fly(&stepless);
        assert_eq!(result.stats.accepted_steps, 0);
        let summary = metrics.summary(&result, stepless.environment()).unwrap();
        assert_eq!(summary.termination, Termination::StepLimit);
        assert_eq!(summary.max_speed_m_s, None);
        let cap_s = f64::from_bits(5.0_f64.to_bits() + 3);
        let at_cap = valetudo(Environment::standard(site()).unwrap(), capped(cap_s))
            .with_recovery(vec![Device::new(
                "main",
                DeviceDrag::canopy(crate::recovery::CanopyType::FlatCircular, 1.0),
                Trigger::Time { time_s: 5.0 },
            )])
            .unwrap();
        let (result, metrics) = fly(&at_cap);
        assert_eq!(result.termination, Termination::TimeCap);
        assert_eq!(result.final_sample.time_s, cap_s);
        metrics.summary(&result, at_cap.environment()).unwrap();
    }

    /// A free-flight step from `start_s` to `end_s` whose samples copy `template` but for the
    /// time, the dynamic pressure and the angle of attack, both functions of time.
    struct AngleStep {
        template: Sample,
        span: (f64, f64),
        phase: Phase,
        dynamic_pressure_pa: fn(f64) -> f64,
        angle_of_attack_rad: fn(f64) -> f64,
    }

    impl FlightStep for AngleStep {
        fn phase(&self) -> Phase {
            self.phase
        }
        fn start_s(&self) -> f64 {
            self.span.0
        }
        fn end_s(&self) -> f64 {
            self.span.1
        }
        fn state_at(&self, _t_s: f64) -> crate::state::State {
            unreachable!("the watcher reads samples and stability")
        }
        fn sample(&self, t_s: f64) -> Result<Sample, SimError> {
            Ok(Sample {
                time_s: t_s,
                dynamic_pressure_pa: (self.dynamic_pressure_pa)(t_s),
                angle_of_attack_rad: (self.angle_of_attack_rad)(t_s),
                ..self.template
            })
        }
        fn stability(&self, t_s: f64) -> Result<Stability, SimError> {
            MarginStep {
                margin_cal: |_| Some(2.0),
            }
            .stability(t_s)
        }
    }

    /// The largest counted angle of attack, rad, after watching free-flight steps that end at
    /// each of `ends` in turn (the first starting at 0 s), then an apogee and a step after it.
    fn largest_angle(
        ends: &[f64],
        dynamic_pressure_pa: fn(f64) -> f64,
        angle_of_attack_rad: fn(f64) -> f64,
    ) -> Option<Peak> {
        let sim = valetudo(Environment::standard(site()).unwrap(), capped(2.0));
        let template = fly(&sim).0.final_sample;
        let mut metrics = FlightMetrics::new();
        let mut start = 0.0;
        for &end in ends {
            let step = AngleStep {
                template,
                span: (start, end),
                phase: Phase::Free,
                dynamic_pressure_pa,
                angle_of_attack_rad,
            };
            metrics.step(&step).unwrap();
            start = end;
        }
        // Nothing after apogee counts, however steep.
        metrics.event(&FlightEvent {
            kind: EventKind::Apogee,
            sample: template,
        });
        let after = AngleStep {
            template,
            span: (start, start + 1.0),
            phase: Phase::Free,
            dynamic_pressure_pa: |_| 1e5,
            angle_of_attack_rad: |_| 3.0,
        };
        metrics.step(&after).unwrap();
        metrics.max_angle_of_attack
    }

    #[test]
    fn the_angle_of_attack_counts_from_just_past_one_second_after_the_rail() {
        // The free flight starts at 0 s. 40° exactly 1 s after it is the transient and doesn't
        // count; 20° at 1.5 s, the next step's middle, does.
        let spike = |t: f64| if t == 1.0 { 40_f64 } else { 20.0 }.to_radians();
        let peak = largest_angle(&[0.5, 1.0, 2.0], |_| 1e3, spike).unwrap();
        assert_eq!((peak.value, peak.time_s), (20_f64.to_radians(), 1.5));
        // The same spike a step end later, at the next time after 1 s, counts.
        let spike = |t: f64| if t == 1.0_f64.next_up() { 40_f64 } else { 20.0 }.to_radians();
        let ends = [0.5, 1.0_f64.next_up(), 2.0];
        let peak = largest_angle(&ends, |_| 1e3, spike).unwrap();
        assert_eq!(
            (peak.value, peak.time_s),
            (40_f64.to_radians(), 1.0_f64.next_up())
        );
        // Nothing past 1 s: no peak.
        assert_eq!(largest_angle(&[0.5, 1.0], |_| 1e3, spike), None);
    }

    #[test]
    fn the_angle_of_attack_counts_only_with_a_fifth_of_the_weight_in_normal_force() {
        // The step's normal-force slope is 1 on a diameter of 1 m: at 1000 Pa a 15° angle gives
        // 206 N, far past a fifth of the rocket's weight; at 1 mPa, far short of it. The edge
        // itself is `envelope::force_counts`'s, tested there.
        let angle = |t: f64| t / 10.0;
        let peak = largest_angle(&[1.0, 2.0, 3.0], |_| 1000.0, angle).unwrap();
        assert_eq!(peak.time_s, 3.0);
        let fading = |t: f64| if t > 2.0 { 1e-3 } else { 1000.0 };
        let peak = largest_angle(&[1.0, 2.0, 3.0], fading, angle).unwrap();
        assert_eq!(peak.time_s, 2.0);
        // A NaN angle that counts stays the largest, so it can't hide a flag.
        let broken = |t: f64| if t == 1.5 { f64::NAN } else { t / 10.0 };
        let peak = largest_angle(&[1.0, 2.0, 3.0], |_| 1000.0, broken).unwrap();
        assert!(peak.value.is_nan() && peak.time_s == 1.5, "{peak:?}");
    }

    /// No wind below `height_msl_m`, and `speed_m_s` from the north above it.
    #[derive(Debug)]
    struct Layer {
        height_msl_m: f64,
        speed_m_s: f64,
    }

    impl hpr_atmos::Wind for Layer {
        fn wind(&self, height_msl_m: f64) -> Result<hpr_atmos::WindSample, hpr_atmos::AtmosError> {
            let speed = if height_msl_m < self.height_msl_m {
                0.0
            } else {
                self.speed_m_s
            };
            Ok(hpr_atmos::WindSample {
                velocity_enu_m_s: hpr_atmos::wind::velocity_from_speed_direction(speed, 0.0),
                extrapolated: None,
            })
        }
    }

    /// Every step end's angle of attack before apogee, with its time and the normal force a 15°
    /// angle would give then, as a share of the rocket's weight.
    #[derive(Default)]
    struct Angles {
        climbing: Vec<(f64, f64, f64)>,
        past_apogee: bool,
    }

    impl Observer for Angles {
        fn step(&mut self, step: &dyn FlightStep) -> Result<(), SimError> {
            let sample = step.sample(step.end_s())?;
            if !self.past_apogee && step.phase() == Phase::Free {
                let stability = step.stability(step.end_s())?;
                let force_n = envelope::normal_force_n(
                    sample.dynamic_pressure_pa,
                    stability.reference_diameter_m,
                    stability.flight_margin.normal_force_slope_per_rad,
                    HIGH_ANGLE_OF_ATTACK_RAD,
                );
                let weight_n = sample.mass_kg * hpr_core::gravity::STANDARD_GRAVITY_MPS2;
                self.climbing.push((
                    sample.time_s,
                    sample.angle_of_attack_rad,
                    force_n / weight_n,
                ));
            }
            Ok(())
        }
        fn event(&mut self, event: &FlightEvent) {
            self.past_apogee |= event.kind == EventKind::Apogee;
        }
    }

    #[test]
    fn a_tilted_calm_climb_passes_15_degrees_only_with_too_little_force() {
        // Off an 84° rail in calm air the path turns over near apogee faster than the slowing
        // rocket can follow, and its angle of attack passes 15° there, at about the rail exit's
        // airspeed; a 15° angle then gives a normal force of 1.0% to 1.5% of its weight, far short of
        // a fifth, so the flight raises no flag.
        let tilted = Rail {
            elevation_rad: 84.0_f64.to_radians(),
            ..Rail::vertical(3.0)
        };
        let sim = Simulation::new(
            &design("rocketpy-valetudo"),
            "example",
            Environment::standard(site()).unwrap(),
            tilted,
            FlightSettings::default(),
        )
        .unwrap();
        let mut watchers = (FlightMetrics::new(), Angles::default());
        let result = sim.run(&mut watchers).unwrap();
        let (metrics, angles) = watchers;
        let summary = metrics.summary(&result, sim.environment()).unwrap();
        let steep: Vec<_> = angles
            .climbing
            .iter()
            .filter(|(_, angle, _)| *angle > HIGH_ANGLE_OF_ATTACK_RAD)
            .collect();
        assert!(!steep.is_empty(), "the climb passes 15° before apogee");
        for (_, _, share) in &steep {
            assert!(*share < 0.15 * HIGH_ANGLE_MIN_FORCE_SHARE, "{steep:?}");
        }
        let counted = summary.max_angle_of_attack_rad.unwrap();
        assert!(counted.value < HIGH_ANGLE_OF_ATTACK_RAD, "{counted:?}");
        assert!(summary.envelope_flags().is_empty());
    }

    #[test]
    fn a_wind_layer_met_at_speed_raises_the_high_angle_flag() {
        // Valetudo climbs into a 30 m/s wind 300 m up, fast, well after its first second: the
        // crosswind turns its angle of attack past 15° until it weathercocks.
        let environment = Environment::standard(site()).unwrap().with_wind(Layer {
            height_msl_m: site().height_m + 300.0,
            speed_m_s: 30.0,
        });
        let sim = valetudo(environment, FlightSettings::default());
        let (result, metrics) = fly(&sim);
        let summary = metrics.summary(&result, sim.environment()).unwrap();
        let flags = summary.envelope_flags();
        let exit = summary.rail_exit_speed_m_s.unwrap();
        let [
            EnvelopeFlag::HighAngleOfAttack {
                angle_of_attack_rad,
            },
        ] = flags[..]
        else {
            panic!("{flags:?}");
        };
        assert!(angle_of_attack_rad.value > HIGH_ANGLE_OF_ATTACK_RAD);
        assert!(angle_of_attack_rad.time_s > exit.time_s + HIGH_ANGLE_GRACE_S);
        assert!(
            (angle_of_attack_rad.height_above_ground_m - 300.0).abs() < 50.0,
            "{angle_of_attack_rad:?}"
        );
        // Calm, the same flight raises nothing.
        let calm = valetudo(
            Environment::standard(site()).unwrap(),
            FlightSettings::default(),
        );
        let (result, metrics) = fly(&calm);
        let summary = metrics.summary(&result, calm.environment()).unwrap();
        assert!(summary.envelope_flags().is_empty(), "{summary:?}");
    }

    #[test]
    fn a_wind_layer_met_in_a_fast_coast_raises_the_high_angle_flag() {
        // The synthetic two-stage rocket, flown as one stack, burns out within 2 s and coasts at
        // several thousand pascals into a 40 m/s wind 2000 m up, long after its boost's peak:
        // the crosswind turns its angle of attack past 15° with a normal force larger than its
        // weight. A floor at a tenth of the boost's dynamic pressure missed it.
        let environment = Environment::standard(site()).unwrap().with_wind(Layer {
            height_msl_m: site().height_m + 2000.0,
            speed_m_s: 40.0,
        });
        let sim = Simulation::new(
            &design("synthetic-two-stage-75mm-54mm"),
            "j760-i175",
            environment,
            Rail::vertical(3.0),
            FlightSettings::default(),
        )
        .unwrap();
        let (result, metrics) = fly(&sim);
        let summary = metrics.summary(&result, sim.environment()).unwrap();
        let flags: Vec<_> = summary
            .envelope_flags()
            .iter()
            .map(EnvelopeFlag::name)
            .collect();
        assert_eq!(flags, ["beyond_validated_range", "high_angle_of_attack"]);
        let angle = summary.max_angle_of_attack_rad.unwrap();
        assert!(
            (angle.height_above_ground_m - 2000.0).abs() < 100.0,
            "{angle:?}"
        );
    }

    /// A free-flight step from `span.0` to `span.1` whose samples copy `template` but for the
    /// time and the thrust, and whose margins are both `margin_cal(t)`, with their pitch-moment
    /// slopes `moment_slope_per_rad(t)` when given (else [`MarginStep`]'s).
    struct PoweredStep {
        template: Sample,
        span: (f64, f64),
        thrust_n: fn(f64) -> f64,
        margin_cal: fn(f64) -> Option<f64>,
        moment_slope_per_rad: Option<fn(f64) -> f64>,
    }

    impl FlightStep for PoweredStep {
        fn phase(&self) -> Phase {
            Phase::Free
        }
        fn start_s(&self) -> f64 {
            self.span.0
        }
        fn end_s(&self) -> f64 {
            self.span.1
        }
        fn state_at(&self, _t_s: f64) -> crate::state::State {
            unreachable!("the watcher reads samples and stability")
        }
        fn sample(&self, t_s: f64) -> Result<Sample, SimError> {
            Ok(Sample {
                time_s: t_s,
                thrust_n: (self.thrust_n)(t_s),
                ..self.template
            })
        }
        fn stability(&self, t_s: f64) -> Result<Stability, SimError> {
            let mut stability = MarginStep {
                margin_cal: self.margin_cal,
            }
            .stability(t_s)?;
            if let Some(slope) = self.moment_slope_per_rad {
                stability.static_margin.pitch_moment_slope_per_rad = slope(t_s);
                stability.flight_margin.pitch_moment_slope_per_rad = slope(t_s);
            }
            Ok(stability)
        }
    }

    /// The least static margin while a motor burns, and the least over the whole span, after
    /// watching free-flight steps from 0 s that end at 0.5, 1 and 2 s, with a motor that burns
    /// out at 1 s, a step's end.
    fn least_powered(margin_cal: fn(f64) -> Option<f64>) -> (Option<Peak>, Option<Peak>) {
        let metrics = watch_powered(margin_cal, None);
        (metrics.min_powered_static_margin, metrics.min_static_margin)
    }

    /// The watcher after [`least_powered`]'s steps, with margins `margin_cal(t)` and, when given,
    /// pitch-moment slopes `moment_slope_per_rad(t)`.
    fn watch_powered(
        margin_cal: fn(f64) -> Option<f64>,
        moment_slope_per_rad: Option<fn(f64) -> f64>,
    ) -> FlightMetrics {
        let sim = valetudo(Environment::standard(site()).unwrap(), capped(2.0));
        let template = fly(&sim).0.final_sample;
        let mut metrics = FlightMetrics::new();
        let mut start = 0.0;
        for end in [0.5, 1.0, 2.0] {
            let step = PoweredStep {
                template,
                span: (start, end),
                thrust_n: |t| if t < 1.0 { 50.0 } else { 0.0 },
                margin_cal,
                moment_slope_per_rad,
            };
            metrics.step(&step).unwrap();
            start = end;
        }
        metrics
    }

    #[test]
    fn the_powered_margin_counts_only_while_a_motor_burns() {
        let tiny = 2_f64.powi(-30);
        let names = |least: Option<Peak>| {
            envelope::flags(None, None, least, None)
                .iter()
                .map(EnvelopeFlag::name)
                .collect::<Vec<_>>()
        };
        // The margin falls by 1 calibre a second. From 1 − 2⁻³⁰ it is −2⁻³⁰ at the burnout:
        // unstable under power, however little.
        let (powered, least) = least_powered(|t| Some(1.0 - 2_f64.powi(-30) - t));
        let powered = powered.unwrap();
        assert_eq!((powered.value, powered.time_s), (-tiny, 1.0));
        assert_eq!(names(Some(powered)), ["unstable_under_power"]);
        assert_eq!(least.unwrap().time_s, 2.0);
        // From 1 + 2⁻³⁰ it is +2⁻³⁰ at the burnout, and falls to −1 calibre in the coast after:
        // no flag, as only the margin under power counts.
        let (powered, least) = least_powered(|t| Some(1.0 + 2_f64.powi(-30) - t));
        let powered = powered.unwrap();
        assert_eq!((powered.value, powered.time_s), (tiny, 1.0));
        assert!(names(Some(powered)).is_empty());
        assert!(least.unwrap().value < -0.99, "{least:?}");
        // Least inside a powered step, below zero, found by the search there.
        let (powered, _) = least_powered(|t| Some((t - 0.37).powi(2) - 0.01));
        let powered = powered.unwrap();
        assert!((powered.value + 0.01).abs() < 1e-15, "{powered:?}");
        assert!((powered.time_s - 0.37).abs() < 1e-7, "{powered:?}");
        // A NaN margin under power is kept, so it raises the flag; one in the coast doesn't count.
        let (powered, _) = least_powered(|t| Some(if t == 0.25 { f64::NAN } else { 2.0 - t }));
        let powered = powered.unwrap();
        assert!(
            powered.value.is_nan() && powered.time_s == 0.25,
            "{powered:?}"
        );
        assert_eq!(names(Some(powered)), ["unstable_under_power"]);
        let (powered, _) = least_powered(|t| Some(if t == 1.5 { f64::NAN } else { 2.0 - t }));
        assert_eq!(powered.unwrap().value, 1.0);
        // A margin defined nowhere under power: none.
        let (powered, _) = least_powered(|t| (t > 1.0).then_some(-1.0));
        assert_eq!(powered, None);
        // A tie can't keep a margin on the stable side of zero: +10⁻¹³ at the start, then
        // −10⁻¹³ (closer than the tie of 10⁻¹² calibres) from 0.25 s on, all under power.
        let (powered, _) = least_powered(|t| Some(if t < 0.2 { 1e-13 } else { -1e-13 }));
        let powered = powered.unwrap();
        assert_eq!((powered.value, powered.time_s), (-1e-13, 0.25));
        assert_eq!(names(Some(powered)), ["unstable_under_power"]);
    }

    #[test]
    fn a_moment_slope_without_a_margin_counts_only_while_a_motor_burns() {
        // 2⁻³⁰, a const so the closures below stay fn pointers.
        const TINY: f64 = 1.0 / 1_073_741_824.0;
        let flag_names = |metrics: &FlightMetrics| {
            envelope::flags(
                None,
                None,
                metrics.min_powered_static_margin,
                metrics.max_powered_moment_slope,
            )
            .iter()
            .map(EnvelopeFlag::name)
            .collect::<Vec<_>>()
        };
        // No margin anywhere; C_mα is +2⁻³⁰ per radian at 0.75 s, while the motor burns, and
        // −1 elsewhere: unstable under power, however little.
        let metrics = watch_powered(|_| None, Some(|t| if t == 0.75 { TINY } else { -1.0 }));
        let peak = metrics.max_powered_moment_slope.unwrap();
        assert_eq!((peak.value, peak.time_s), (TINY, 0.75));
        assert_eq!(flag_names(&metrics), ["unstable_without_margin"]);
        // −2⁻³⁰ there instead: the air turns it back, no flag.
        let metrics = watch_powered(|_| None, Some(|t| if t == 0.75 { -TINY } else { -1.0 }));
        let peak = metrics.max_powered_moment_slope.unwrap();
        assert_eq!((peak.value, peak.time_s), (-TINY, 0.75));
        assert!(flag_names(&metrics).is_empty());
        // +2⁻³⁰ only after the burnout at 1 s: not under power, no flag.
        let metrics = watch_powered(|_| None, Some(|t| if t > 1.0 { TINY } else { -1.0 }));
        assert_eq!(metrics.max_powered_moment_slope.unwrap().value, -1.0);
        assert!(flag_names(&metrics).is_empty());
        // A NaN slope under power is kept, so it raises the flag; nothing larger replaces it, nor
        // does a later NaN.
        let metrics = watch_powered(
            |_| None,
            Some(|t| {
                if t == 0.25 || t == 0.75 {
                    f64::NAN
                } else {
                    5.0 * t
                }
            }),
        );
        let peak = metrics.max_powered_moment_slope.unwrap();
        assert!(peak.value.is_nan() && peak.time_s == 0.25, "{peak:?}");
        assert_eq!(flag_names(&metrics), ["unstable_without_margin"]);
        // Where the margin is defined, its slope doesn't count, even above zero: the margin says
        // it all there.
        let metrics = watch_powered(|t| (t != 0.75).then_some(1.0), Some(|_| 3.0));
        let peak = metrics.max_powered_moment_slope.unwrap();
        assert_eq!((peak.value, peak.time_s), (3.0, 0.75));
        let metrics = watch_powered(|_| Some(1.0), Some(|_| 3.0));
        assert_eq!(metrics.max_powered_moment_slope, None);
        assert!(flag_names(&metrics).is_empty());
        // A margin below zero under power and a slope above zero where it is undefined: one flag.
        let metrics = watch_powered(|t| (t != 0.75).then_some(-1.0), Some(|_| 3.0));
        assert_eq!(flag_names(&metrics), ["unstable_under_power"]);
    }

    #[test]
    fn a_rocket_without_fins_is_unstable_under_power() {
        // Valetudo, its fins taken off, flown for 2 s: its nose alone carries the normal force,
        // ahead of the center of mass, so the margin is below zero while the motor burns. With
        // its fins it raises nothing (above, and in the angle-of-attack tests).
        let mut rocket = serde_json::to_value(design("rocketpy-valetudo")).unwrap();
        let children = rocket["stages"][0]["components"][1]["children"]
            .as_array_mut()
            .unwrap();
        let before = children.len();
        children.retain(|child| child["id"] != "fins-1");
        assert_eq!(children.len(), before - 1);
        let rocket: Rocket = serde_json::from_value(rocket).unwrap();
        let sim = Simulation::new(
            &rocket,
            "example",
            Environment::standard(site()).unwrap(),
            Rail::vertical(3.0),
            capped(2.0),
        )
        .unwrap();
        let (result, metrics) = fly(&sim);
        let summary = metrics.summary(&result, sim.environment()).unwrap();
        let powered = summary.min_powered_static_margin_cal.unwrap();
        assert!(powered.value < -1.0, "{powered:?}");
        let burnout_s = sim.assembly().motors[0].mounted.motor.burnout_time_s();
        assert!(powered.time_s <= burnout_s, "{powered:?}");
        let flags = summary.envelope_flags();
        assert_eq!(
            flags[0],
            EnvelopeFlag::UnstableUnderPower {
                static_margin_cal: powered
            }
        );
        // Stock, its least margin under power is the least of all, and above zero.
        let sim = valetudo(Environment::standard(site()).unwrap(), capped(2.0));
        let (result, metrics) = fly(&sim);
        let summary = metrics.summary(&result, sim.environment()).unwrap();
        let powered = summary.min_powered_static_margin_cal.unwrap();
        assert_eq!(Some(powered), summary.min_static_margin_cal);
        assert!(powered.value > 1.0, "{powered:?}");
        assert!(summary.envelope_flags().is_empty());
    }
}
