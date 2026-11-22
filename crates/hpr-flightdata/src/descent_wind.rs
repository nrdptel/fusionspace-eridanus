//! The wind, reconstructed from a GPS-tracked descent ([M1.14b6][m1-14b6]).
//!
//! **What it is for.** A rocket falling under a parachute drifts with the wind, a little late,
//! like a dropsonde. Its GPS track then measures the wind at the flight's own place and minutes,
//! which a reanalysis such as ERA5, hourly on a 25 km grid, can't. The validation report
//! `validation/reports/descent-wind.md` sets this reconstruction beside ERA5 on the public
//! flights whose logs hold a GPS track of the descent.
//!
//! **The equation** (Hock and Franklin 1999, the appendix, after Lally and Leviton 1958; Kepert
//! 2008, eq. 4). With quadratic drag on the velocity relative to the air, and the fall much
//! faster than the horizontal slip, the body takes up a change in the wind with the time
//! constant `τ = w/g`, `w` the descent rate. So the wind is the ground velocity plus `τ` times
//! its rate of change:
//!
//! ```text
//! u = v + (w/g) dv/dt
//! ```
//!
//! with `v` the horizontal ground velocity. Under a 6 m/s main `τ` is 0.6 s and the correction
//! is a few centimeters per second; under a 25 m/s drogue `τ` is 2.5 s, and in a wind that
//! strengthens by 2 m/s per 100 m of height the uncorrected ground velocity lags it by
//! `(w²/g)(dU/dz)` = 1.3 m/s (`docs/research/wind-from-descent.md` §2).
//!
//! **Method.** At each fix, a least-squares quadratic in time is fitted to the east, north and up
//! positions of the fixes within [`Settings::window_s`] centered on it. The linear term is the
//! velocity, twice the quadratic term the acceleration. A window of 10 s spans about two and a
//! half periods of a 4 s swing on the shock cord, the note's §4 reading of GRUAN's filter. A fix
//! is kept where the body falls steadily: a descent rate inside
//! [`Settings::min_descent_m_s`] to [`Settings::max_descent_m_s`] and a vertical acceleration no
//! larger than [`Settings::max_vertical_acceleration_m_s2`]. That drops the fall from apogee
//! before a canopy holds, each canopy's opening and the landing, where the steady fall the
//! equation assumes does not hold.
//!
//! **How far to trust it.** The tests here pin it on synthetic descents, with the true wind
//! known, built on the method's own assumptions: quadratic drag, the same in every direction, and
//! no glide. On real descents nothing bounds what the canopy itself does: a canopy that glides
//! moves across the wind by `w tan γ` at a glide angle `γ`, up to the descent rate itself at
//! 45° (Knacke, NWC TP 6575, p. 5-16), and a swing on the shock cord adds up to `θ√(gL)` at an
//! angle `θ` on a line of length `L`, about 1 m/s at 10° on 3 m. On a synthetic swing of that
//! size, with a 2 to 5 s period and a fix a second, the window leaves under 0.15 m/s RMS (a test
//! here); a glide it leaves whole. Neither is measured for hobby canopies.
//!
//! [m1-14b6]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m1-14b6

use hpr_core::geodesy::{Ellipsoid, Geodetic, ecef_from_enu_rotation};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Standard gravity, m/s², the `g` of `τ = w/g`. The method's `g` needs no more: a 0.3% change
/// in `g` changes a 1 m/s correction by 3 mm/s.
pub const STANDARD_GRAVITY_M_S2: f64 = 9.806_65;

/// One GPS fix, in a local east, north and up frame, m.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TrackPoint {
    /// The fix's time, s, on any clock that increases.
    pub time_s: f64,
    /// East of the frame's origin, m.
    pub east_m: f64,
    /// North of the frame's origin, m.
    pub north_m: f64,
    /// Up from the frame's origin, m.
    pub up_m: f64,
}

impl TrackPoint {
    /// A fix at `point`, resolved in the east, north and up frame at `origin` on WGS 84: the
    /// difference of the two earth-centered positions, turned by the transpose of
    /// [`ecef_from_enu_rotation`] at `origin` (NGA.STND.0036 eq. 4-14). Over a drift of a few
    /// kilometers the tangent plane departs from the ellipsoid by under a meter, which moves no
    /// velocity.
    #[must_use]
    pub fn from_geodetic(time_s: f64, origin: Geodetic, point: Geodetic) -> Self {
        let ellipsoid = Ellipsoid::WGS84;
        let offset = ellipsoid.ecef_from_geodetic(point) - ellipsoid.ecef_from_geodetic(origin);
        let enu = ecef_from_enu_rotation(origin).transpose() * offset;
        Self {
            time_s,
            east_m: enu.x,
            north_m: enu.y,
            up_m: enu.z,
        }
    }
}

/// Which reading of the track gives the wind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Reading {
    /// The ground velocity plus `w/g` times its rate of change: the reconstruction.
    Lagged,
    /// The ground velocity taken as the wind, with no lag: the reading the tests show falls short
    /// under a fast drogue.
    GroundVelocity,
}

/// The reconstruction's settings. [`Settings::default`] gives the ones the report flies.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    /// The width of the window each fix's quadratic is fitted over, centered on the fix, s.
    pub window_s: f64,
    /// The fewest fixes a window must hold.
    pub min_points: usize,
    /// The slowest descent kept, m/s: slower is a landed or hanging body.
    pub min_descent_m_s: f64,
    /// The fastest descent kept, m/s: the range the tests pin, past the note's fastest drogue
    /// (30 m/s). A faster fall is not a canopy's, and the lag model is not shown to hold there.
    pub max_descent_m_s: f64,
    /// The largest vertical acceleration kept, m/s²: a steady fall's. In a steady fall the
    /// descent rate changes only with the air's density, `w²/(2H)` with `H` ≈ 8.5 km the density
    /// scale height, 0.04 m/s² at 25 m/s; a canopy's opening or the landing changes it by several
    /// m/s².
    pub max_vertical_acceleration_m_s2: f64,
    /// `g` in `τ = w/g`, m/s².
    pub gravity_m_s2: f64,
    /// Which reading gives the wind.
    pub reading: Reading,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            window_s: 10.0,
            min_points: 5,
            min_descent_m_s: 2.0,
            max_descent_m_s: 35.0,
            max_vertical_acceleration_m_s2: 0.5,
            gravity_m_s2: STANDARD_GRAVITY_M_S2,
            reading: Reading::Lagged,
        }
    }
}

/// The wind at one fix of a steady descent.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WindSample {
    /// The fix's time, s.
    pub time_s: f64,
    /// The fix's height in the track's frame, m.
    pub up_m: f64,
    /// The descent rate `w`, m/s, positive down.
    pub descent_m_s: f64,
    /// The vertical acceleration, m/s², positive up.
    pub vertical_acceleration_m_s2: f64,
    /// The ground velocity, east and north, m/s.
    pub ground_velocity_m_s: [f64; 2],
    /// Its rate of change, east and north, m/s².
    pub ground_acceleration_m_s2: [f64; 2],
    /// The lag `τ` applied, s: `w/g`, or zero for [`Reading::GroundVelocity`].
    pub lag_s: f64,
    /// The wind, east and north, m/s: the direction it blows toward.
    pub wind_m_s: [f64; 2],
}

/// Why a track can't be read.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum DescentWindError {
    /// A fix holds a value that is not finite.
    #[error("track point {index}: a value is not finite")]
    NonFinite {
        /// The fix's index.
        index: usize,
    },
    /// A fix's time does not follow the one before.
    #[error("track point {index}: its time, {time_s} s, does not follow the one before")]
    NotIncreasing {
        /// The fix's index.
        index: usize,
        /// Its time, s.
        time_s: f64,
    },
    /// A setting is out of its range.
    #[error("setting `{name}` = {value}: {why}")]
    Setting {
        /// The setting.
        name: &'static str,
        /// Its value.
        value: f64,
        /// What it must be.
        why: &'static str,
    },
}

fn check(settings: &Settings) -> Result<(), DescentWindError> {
    let positive = |name: &'static str, value: f64| {
        if value.is_finite() && value > 0.0 {
            Ok(())
        } else {
            Err(DescentWindError::Setting {
                name,
                value,
                why: "must be finite and positive",
            })
        }
    };
    positive("window_s", settings.window_s)?;
    positive("min_descent_m_s", settings.min_descent_m_s)?;
    positive("max_descent_m_s", settings.max_descent_m_s)?;
    positive(
        "max_vertical_acceleration_m_s2",
        settings.max_vertical_acceleration_m_s2,
    )?;
    positive("gravity_m_s2", settings.gravity_m_s2)?;
    if settings.max_descent_m_s < settings.min_descent_m_s {
        return Err(DescentWindError::Setting {
            name: "max_descent_m_s",
            value: settings.max_descent_m_s,
            why: "must be at least min_descent_m_s",
        });
    }
    if settings.min_points < 3 {
        return Err(DescentWindError::Setting {
            name: "min_points",
            // A count below 3 is exact as an f64.
            value: settings.min_points as f64,
            why: "must be at least 3, the quadratic's coefficients",
        });
    }
    Ok(())
}

/// A least-squares quadratic `a + b s + c s²` in the offset `s` from the window's center: `b` and
/// `c` for each of the three axes, or `None` where the normal equations are singular.
fn fit(points: &[TrackPoint], center_s: f64) -> Option<[(f64, f64); 3]> {
    // The normal equations' moments Σ sᵏ, k = 0..4, and Σ sᵏ y for each axis.
    let mut moments = [0.0_f64; 5];
    let mut weighted = [[0.0_f64; 3]; 3];
    for point in points {
        let s = point.time_s - center_s;
        let powers = [1.0, s, s * s, s * s * s, s * s * s * s];
        for (moment, power) in moments.iter_mut().zip(powers) {
            *moment += power;
        }
        for (axis, y) in [point.east_m, point.north_m, point.up_m]
            .into_iter()
            .enumerate()
        {
            for k in 0..3 {
                weighted[axis][k] += powers[k] * y;
            }
        }
    }
    let m = |i: usize, j: usize| moments[i + j];
    let determinant = |columns: [[f64; 3]; 3]| {
        columns[0][0] * (columns[1][1] * columns[2][2] - columns[2][1] * columns[1][2])
            - columns[1][0] * (columns[0][1] * columns[2][2] - columns[2][1] * columns[0][2])
            + columns[2][0] * (columns[0][1] * columns[1][2] - columns[1][1] * columns[0][2])
    };
    // Columns of the symmetric normal matrix.
    let matrix = [
        [m(0, 0), m(1, 0), m(2, 0)],
        [m(0, 1), m(1, 1), m(2, 1)],
        [m(0, 2), m(1, 2), m(2, 2)],
    ];
    let full = determinant(matrix);
    // The matrix is a Gram matrix, so its determinant is non-negative; relative to its diagonal's
    // product it measures how far the window's times are from fixing a quadratic.
    let scale = m(0, 0) * m(1, 1) * m(2, 2);
    // A NaN determinant fails the comparison too, and is refused.
    if full.is_nan() || full <= 1e-9 * scale {
        return None;
    }
    let mut out = [(0.0, 0.0); 3];
    for (axis, rhs) in weighted.iter().enumerate() {
        // Cramer's rule for b (column 1) and c (column 2).
        let mut with_b = matrix;
        with_b[1] = *rhs;
        let mut with_c = matrix;
        with_c[2] = *rhs;
        out[axis] = (determinant(with_b) / full, determinant(with_c) / full);
    }
    Some(out)
}

/// The wind at each fix of `track` where the body falls steadily, as [`Settings`] defines it.
///
/// A fix is read only when its window holds at least [`Settings::min_points`] fixes and reaches
/// a quarter of the window on each side. So a fix near the track's ends is not read, nor one with
/// no fix within a quarter of the window on one side, as beside a gap; a shorter gap inside a
/// window is fitted across.
///
/// # Errors
///
/// [`DescentWindError`]: a non-finite value, a time that does not increase, or a setting out of
/// its range.
pub fn reconstruct(
    track: &[TrackPoint],
    settings: &Settings,
) -> Result<Vec<WindSample>, DescentWindError> {
    check(settings)?;
    for (index, point) in track.iter().enumerate() {
        if ![point.time_s, point.east_m, point.north_m, point.up_m]
            .iter()
            .all(|value| value.is_finite())
        {
            return Err(DescentWindError::NonFinite { index });
        }
        if index > 0 && point.time_s <= track[index - 1].time_s {
            return Err(DescentWindError::NotIncreasing {
                index,
                time_s: point.time_s,
            });
        }
    }
    let half = settings.window_s / 2.0;
    let mut out = Vec::new();
    let mut start = 0;
    let mut end = 0;
    for point in track {
        let center = point.time_s;
        while track[start].time_s < center - half {
            start += 1;
        }
        while end < track.len() && track[end].time_s <= center + half {
            end += 1;
        }
        let window = &track[start..end];
        let reaches = |side: f64| {
            window
                .iter()
                .any(|other| side * (other.time_s - center) >= half / 2.0)
        };
        if window.len() < settings.min_points || !reaches(-1.0) || !reaches(1.0) {
            continue;
        }
        let Some([(east_v, east_c), (north_v, north_c), (up_v, up_c)]) = fit(window, center) else {
            continue;
        };
        let descent = -up_v;
        let vertical_acceleration = 2.0 * up_c;
        if !(settings.min_descent_m_s..=settings.max_descent_m_s).contains(&descent)
            || vertical_acceleration.abs() > settings.max_vertical_acceleration_m_s2
        {
            continue;
        }
        let ground_acceleration = [2.0 * east_c, 2.0 * north_c];
        let lag = match settings.reading {
            Reading::Lagged => descent / settings.gravity_m_s2,
            Reading::GroundVelocity => 0.0,
        };
        out.push(WindSample {
            time_s: center,
            up_m: point.up_m,
            descent_m_s: descent,
            vertical_acceleration_m_s2: vertical_acceleration,
            ground_velocity_m_s: [east_v, north_v],
            ground_acceleration_m_s2: ground_acceleration,
            lag_s: lag,
            wind_m_s: [
                east_v + lag * ground_acceleration[0],
                north_v + lag * ground_acceleration[1],
            ],
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hpr_core::random::SeededRng;

    /// The synthetic wind: blowing toward 60° east of north, 2 m/s at the ground and 2 m/s
    /// stronger for every 100 m of height (a shear of 0.02 s⁻¹, the note's strongest column).
    fn wind_at(up_m: f64) -> [f64; 2] {
        let speed = 2.0 + 0.02 * up_m;
        let toward = 60.0_f64.to_radians();
        [speed * toward.sin(), speed * toward.cos()]
    }

    /// Air density on an exponential profile with an 8.5 km scale height, kg/m³: enough for a
    /// descent rate that changes with height as a real one does.
    fn density(up_m: f64) -> f64 {
        1.225 * (-up_m / 8500.0).exp()
    }

    /// A body with quadratic drag on its velocity through the air, falling at `descent_m_s` at
    /// 500 m from a steady state at 1,200 m, integrated by fourth-order Runge–Kutta at 5 ms
    /// and sampled once a second down to the ground: each fix with the true wind at its height.
    fn descend(descent_m_s: f64) -> Vec<(TrackPoint, [f64; 2])> {
        let g = STANDARD_GRAVITY_M_S2;
        // w² = g/(k ρ), so k = g/(ρ w²) at 500 m.
        let k = g / (density(500.0) * descent_m_s * descent_m_s);
        let derivative = |state: [f64; 6]| {
            let wind = wind_at(state[2]);
            let relative = [state[3] - wind[0], state[4] - wind[1], state[5]];
            let speed = (relative[0].powi(2) + relative[1].powi(2) + relative[2].powi(2)).sqrt();
            let drag = k * density(state[2]) * speed;
            [
                state[3],
                state[4],
                state[5],
                -drag * relative[0],
                -drag * relative[1],
                -g - drag * relative[2],
            ]
        };
        let start = 1200.0;
        let wind = wind_at(start);
        let fall = (g / (k * density(start))).sqrt();
        let mut state = [0.0, 0.0, start, wind[0], wind[1], -fall];
        let dt = 0.005;
        let mut out = Vec::new();
        let mut step = 0_u64;
        while state[2] > 0.0 {
            if step.is_multiple_of(200) {
                // Step counts below 2^53 are exact.
                let time_s = step as f64 * dt;
                out.push((
                    TrackPoint {
                        time_s,
                        east_m: state[0],
                        north_m: state[1],
                        up_m: state[2],
                    },
                    wind_at(state[2]),
                ));
            }
            let add = |a: [f64; 6], b: [f64; 6], h: f64| std::array::from_fn(|i| a[i] + h * b[i]);
            let k1 = derivative(state);
            let k2 = derivative(add(state, k1, dt / 2.0));
            let k3 = derivative(add(state, k2, dt / 2.0));
            let k4 = derivative(add(state, k3, dt));
            state = std::array::from_fn(|i| {
                state[i] + dt / 6.0 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i])
            });
            step += 1;
        }
        out
    }

    /// The root-mean-square of the vector error between the read wind and the true wind at each
    /// fix from 100 m to 1,000 m, m/s, and how many fixes it holds.
    fn rms_error(
        truth: &[(TrackPoint, [f64; 2])],
        track: &[TrackPoint],
        reading: Reading,
    ) -> (f64, usize) {
        let settings = Settings {
            reading,
            ..Settings::default()
        };
        let samples = reconstruct(track, &settings).unwrap();
        let mut sum = 0.0;
        let mut count = 0;
        for sample in &samples {
            if !(100.0..=1000.0).contains(&sample.up_m) {
                continue;
            }
            let (_, true_wind) = truth
                .iter()
                .find(|(point, _)| point.time_s == sample.time_s)
                .unwrap();
            sum += (sample.wind_m_s[0] - true_wind[0]).powi(2)
                + (sample.wind_m_s[1] - true_wind[1]).powi(2);
            count += 1;
        }
        // Counts here are below 2^53, exact as f64.
        ((sum / count as f64).sqrt(), count)
    }

    fn track_of(truth: &[(TrackPoint, [f64; 2])]) -> Vec<TrackPoint> {
        truth.iter().map(|(point, _)| *point).collect()
    }

    /// The plan's line: at 5 m/s and at 25 m/s through a wind that changes by 2 m/s per 100 m,
    /// the reconstruction recovers the wind within 0.3 m/s RMS; the ground velocity taken as the
    /// wind fails that line at 25 m/s.
    #[test]
    fn the_lagged_reading_recovers_the_wind_and_the_ground_velocity_misses_it_under_a_drogue() {
        for descent in [5.0, 25.0, 33.0] {
            let truth = descend(descent);
            let track = track_of(&truth);
            let (lagged, count) = rms_error(&truth, &track, Reading::Lagged);
            let (ground, _) = rms_error(&truth, &track, Reading::GroundVelocity);
            // Every fix from 100 m to 1,000 m is a steady fall, so each is read.
            let expected = truth
                .iter()
                .filter(|(point, _)| (100.0..=1000.0).contains(&point.up_m))
                .count();
            assert_eq!(count, expected, "{descent} m/s");
            assert!(lagged < 0.3, "{descent} m/s: lagged {lagged}");
            // The lag the ground velocity leaves is (w²/g)(dU/dz), 0.05 m/s at 5 m/s and 1.27
            // m/s at 25 m/s, density's change in w aside.
            let lag = descent * descent / STANDARD_GRAVITY_M_S2 * 0.02;
            assert!(
                (ground - lag).abs() < 0.1 * lag,
                "{descent} m/s: ground {ground}, lag {lag}"
            );
            if descent >= 25.0 {
                assert!(ground > 0.3, "{descent} m/s: ground {ground}");
            }
        }
    }

    /// The reconstruction's error itself is small against the bar: a regression guard that
    /// would catch a lag applied with the wrong sign (which doubles the error rather than
    /// removing it) or a velocity taken from the wrong term of the fit.
    #[test]
    fn the_lagged_reading_is_near_exact_on_a_noiseless_track() {
        for descent in [5.0, 25.0] {
            let truth = descend(descent);
            let (lagged, _) = rms_error(&truth, &track_of(&truth), Reading::Lagged);
            assert!(lagged < 0.05, "{descent} m/s: {lagged}");
        }
    }

    /// With a GPS receiver's noise on every fix (0.5 m east and north, 1 m up, independent from
    /// fix to fix), the reconstruction still holds the line, on every one of 200 seeds.
    #[test]
    fn the_lagged_reading_holds_the_line_with_receiver_noise() {
        for descent in [5.0, 25.0] {
            let truth = descend(descent);
            let mut worst: f64 = 0.0;
            for seed in 0..200 {
                let mut rng = SeededRng::seed_from_u64(seed);
                let noisy: Vec<TrackPoint> = truth
                    .iter()
                    .map(|(point, _)| TrackPoint {
                        east_m: point.east_m + 0.5 * rng.standard_normal(),
                        north_m: point.north_m + 0.5 * rng.standard_normal(),
                        up_m: point.up_m + rng.standard_normal(),
                        ..*point
                    })
                    .collect();
                let (lagged, _) = rms_error(&truth, &noisy, Reading::Lagged);
                worst = worst.max(lagged);
            }
            assert!(
                worst.is_finite() && worst < 0.3,
                "{descent} m/s: worst {worst}"
            );
        }
    }

    #[test]
    fn the_steady_fall_rule_drops_an_opening_and_the_landing() {
        // A fall at 25 m/s that slows to 6 m/s over 4 s at 600 m, then lands at 0 m and stays.
        let mut track = Vec::new();
        let mut up = 1200.0;
        let mut time = 0.0;
        let mut rate: f64 = 25.0;
        while time < 400.0 {
            track.push(TrackPoint {
                time_s: time,
                east_m: 3.0 * time,
                north_m: 0.0,
                up_m: up,
            });
            if up < 600.0 {
                rate = (rate - 4.75).max(6.0);
            }
            up = (up - rate).max(0.0);
            time += 1.0;
        }
        let samples = reconstruct(&track, &Settings::default()).unwrap();
        assert!(!samples.is_empty());
        for sample in &samples {
            assert!(sample.descent_m_s >= 2.0, "{sample:?}");
            assert!(sample.vertical_acceleration_m_s2.abs() <= 0.5, "{sample:?}");
            assert!((sample.wind_m_s[0] - 3.0).abs() < 1e-9, "{sample:?}");
        }
        // Nothing is read on the ground or within 2 s of the opening's change.
        assert!(samples.iter().all(|s| s.up_m > 0.0));
        let opening = track.iter().find(|p| p.up_m < 600.0).unwrap().time_s;
        assert!(
            samples.iter().all(|s| (s.time_s - opening).abs() >= 2.0),
            "{samples:?}"
        );
    }

    /// A swing on the shock cord: the rocket's ground velocity is the wind's plus a sinusoid of
    /// 0.95 m/s amplitude (`θ√(gL)` at 10° on 3 m) and a 2 to 5 s period, fixes once a second
    /// (so a 2 s swing shows only in the positions, its velocity zero at every fix). The window
    /// cuts it to under 0.15 m/s RMS (0.10 at most on these cases), at 5 and at 25
    /// m/s, where the lag term multiplies the swing's acceleration by up to 2.5 s.
    #[test]
    fn the_window_cuts_a_swing_on_the_shock_cord() {
        let wind = 5.0;
        let amplitude = 0.95;
        for descent in [5.0, 25.0] {
            for period in [2.0, 3.5, 5.0] {
                let omega = std::f64::consts::TAU / period;
                let track: Vec<TrackPoint> = (0..300)
                    .map(|i| {
                        let t = f64::from(i);
                        TrackPoint {
                            time_s: t,
                            east_m: wind * t + amplitude / omega * (1.0 - (omega * t).cos()),
                            north_m: 0.0,
                            up_m: 2000.0 - descent * t,
                        }
                    })
                    .collect();
                let samples = reconstruct(&track, &Settings::default()).unwrap();
                assert_eq!(samples.len(), 294, "{descent} m/s, {period} s");
                let squares: f64 = samples
                    .iter()
                    .map(|s| (s.wind_m_s[0] - wind).powi(2) + s.wind_m_s[1].powi(2))
                    .sum();
                // Counts here are below 2^53, exact as f64.
                let rms = (squares / samples.len() as f64).sqrt();
                assert!(rms < 0.15, "{descent} m/s, {period} s: {rms}");
            }
        }
    }

    /// A gap: no fix from 10 s to 29 s. A fix with no fix within a quarter of the window on one
    /// side (7 to 9 s before the gap, 30 to 32 s after it) is not read; one that reaches both
    /// sides is.
    #[test]
    fn a_fix_beside_a_gap_is_not_read() {
        let track: Vec<TrackPoint> = (0..60)
            .filter(|i| !(10..30).contains(i))
            .map(|i| {
                let time_s = f64::from(i);
                TrackPoint {
                    time_s,
                    east_m: 4.0 * time_s,
                    north_m: 0.0,
                    up_m: 1000.0 - 10.0 * time_s,
                }
            })
            .collect();
        let read: Vec<f64> = reconstruct(&track, &Settings::default())
            .unwrap()
            .iter()
            .map(|sample| sample.time_s)
            .collect();
        let expected: Vec<f64> = (3..=6).chain(33..=56).map(f64::from).collect();
        assert_eq!(read, expected);
    }

    #[test]
    fn a_window_that_does_not_reach_both_sides_is_not_read() {
        let track: Vec<TrackPoint> = (0..30)
            .map(|i| {
                let time_s = f64::from(i);
                TrackPoint {
                    time_s,
                    east_m: 0.0,
                    north_m: 0.0,
                    up_m: 1000.0 - 10.0 * time_s,
                }
            })
            .collect();
        let samples = reconstruct(&track, &Settings::default()).unwrap();
        // The first and last 2.5 s (a quarter of the window) are not read.
        assert_eq!(samples.first().unwrap().time_s, 3.0);
        assert_eq!(samples.last().unwrap().time_s, 26.0);
    }

    #[test]
    fn bad_tracks_and_settings_are_refused() {
        let point = |time_s| TrackPoint {
            time_s,
            east_m: 0.0,
            north_m: 0.0,
            up_m: 0.0,
        };
        assert_eq!(
            reconstruct(&[point(0.0), point(0.0)], &Settings::default()),
            Err(DescentWindError::NotIncreasing {
                index: 1,
                time_s: 0.0
            })
        );
        assert_eq!(
            reconstruct(&[point(0.0), point(f64::NAN)], &Settings::default()),
            Err(DescentWindError::NonFinite { index: 1 })
        );
        let refused = |settings: Settings| match reconstruct(&[], &settings) {
            Err(DescentWindError::Setting { name, .. }) => name,
            other => panic!("{other:?}"),
        };
        assert_eq!(
            refused(Settings {
                window_s: 0.0,
                ..Settings::default()
            }),
            "window_s"
        );
        assert_eq!(
            refused(Settings {
                max_descent_m_s: 1.0,
                ..Settings::default()
            }),
            "max_descent_m_s"
        );
        assert_eq!(
            refused(Settings {
                min_points: 2,
                ..Settings::default()
            }),
            "min_points"
        );
        assert_eq!(
            refused(Settings {
                gravity_m_s2: f64::NAN,
                ..Settings::default()
            }),
            "gravity_m_s2"
        );
    }

    #[test]
    fn a_fix_north_of_the_origin_reads_north() {
        let origin = Geodetic::from_degrees(32.94, -106.91, 1400.0).unwrap();
        let point = Geodetic::from_degrees(32.95, -106.91, 1500.0).unwrap();
        let fix = TrackPoint::from_geodetic(1.0, origin, point);
        // 0.01° of latitude is about 1,109 m there; the up axis drops by the curvature.
        assert!((fix.north_m - 1109.0).abs() < 5.0, "{fix:?}");
        assert!(fix.east_m.abs() < 1e-6, "{fix:?}");
        assert!((fix.up_m - 100.0).abs() < 0.2, "{fix:?}");
    }
}
