//! The smoother held to the synthetic flights' truth ([M7.2d][m7-2d]): each flight's own log
//! smoothed, its height and vertical speed compared with the truth's at four moments, and the
//! share of flights whose truth falls outside the smoother's 90% interval on each side counted.
//!
//! **The moments**, on the truth's clock: burnout, the top vertical speed, apogee, and one moment
//! drawn per flight, uniformly between liftoff and landing, on a stream of its own
//! ([`moment_drawn`]). Each flight gives one comparison a moment, so the flights' comparisons are
//! independent. The truth is the center of mass's height above where it stood at launch and its
//! vertical speed (`truth`'s module).
//!
//! **Holding** ([ADR-299][adr-299]): a 90% interval misses about 5% of the time on each side. For
//! each state and moment, each side's misses are counted over a set's smoothed flights, and the
//! smoother holds there when that count's two-sided 99.9% Clopper–Pearson interval reaches down to
//! 5%: the flights don't show it misses more often than it says. An interval wider than it needs
//! to be holds; one that misses too often doesn't.
//!
//! [m7-2d]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-2d
//! [adr-299]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0299-the-smoother-and-its-check.md

use std::collections::BTreeMap;

use hpr_core::random::SeededRng;
use hpr_flightdata::smoother::{self, INTERVAL_Z, Options, Reason};
use serde::{Deserialize, Serialize};

use super::logger::Profile;
use super::{Flight, Set};

/// The moments the smoother is checked at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Moment {
    /// The motor's burnout.
    Burnout,
    /// The top vertical speed.
    TopSpeed,
    /// Apogee.
    Apogee,
    /// A moment drawn uniformly between liftoff and landing.
    Drawn,
}

impl Moment {
    /// Every moment.
    pub const ALL: [Self; 4] = [Self::Burnout, Self::TopSpeed, Self::Apogee, Self::Drawn];

    /// Its name.
    pub fn name(self) -> &'static str {
        match self {
            Self::Burnout => "burnout",
            Self::TopSpeed => "top speed",
            Self::Apogee => "apogee",
            Self::Drawn => "a drawn moment",
        }
    }
}

/// The smoothed states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StateName {
    /// The height above the pad.
    Height,
    /// The vertical speed.
    Speed,
}

impl StateName {
    /// Both.
    pub const ALL: [Self; 2] = [Self::Height, Self::Speed];

    /// Its name.
    pub fn name(self) -> &'static str {
        match self {
            Self::Height => "height",
            Self::Speed => "speed",
        }
    }
}

/// One comparison: the smoothed value less the truth, and the smoother's standard error.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Compared {
    /// The smoothed value less the truth.
    pub error: f64,
    /// The smoother's standard error.
    pub sd: f64,
}

impl Compared {
    /// Whether the truth lies below the 90% interval (the smoother reads high).
    pub fn truth_below(&self) -> bool {
        !self.finite() || self.error > INTERVAL_Z * self.sd
    }

    /// Whether it lies above.
    pub fn truth_above(&self) -> bool {
        !self.finite() || -self.error > INTERVAL_Z * self.sd
    }

    /// Whether the error and its standard error are both numbers: one that isn't misses on both
    /// sides, as a NaN compares false and would otherwise count as held.
    fn finite(&self) -> bool {
        self.error.is_finite() && self.sd.is_finite()
    }
}

/// One flight's check.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Checked {
    /// Its logger.
    pub profile: Profile,
    /// Why its log wasn't smoothed; `None` if it was.
    pub withheld: Option<Reason>,
    /// Each moment and state, compared.
    pub compared: Vec<(Moment, StateName, Compared)>,
}

/// The moment drawn for flight `index` of `set`, s on the truth's clock, uniformly between
/// `liftoff_s` and `landing_s`, on a stream keyed by the set's seed, the index and the
/// smoother's own key, so no other draw moves.
pub fn moment_drawn(set: Set, index: u64, liftoff_s: f64, landing_s: f64) -> f64 {
    let mut rng = SeededRng::for_stream(set.seed(), &[index, super::SMOOTHER_MOMENT_KEY]);
    liftoff_s + rng.uniform() * (landing_s - liftoff_s)
}

/// Checks one flight's smoothed log against its truth.
pub fn check(flight: &Flight, options: Options) -> Checked {
    let profile = flight.draw.logger.profile;
    let smoothed = match smoother::smooth_with(
        &flight.log,
        &hpr_flightdata::readings::read(&flight.log),
        options,
    ) {
        Ok(smoothed) => smoothed,
        Err(withheld) => {
            return Checked {
                profile,
                withheld: Some(withheld.reason),
                compared: Vec::new(),
            };
        }
    };
    let truth = &flight.truth;
    let events = &truth.events;
    let moments = [
        (Moment::Burnout, events.burnout_s),
        (Moment::TopSpeed, truth.top_speed().1),
        (Moment::Apogee, events.apogee_s),
        (
            Moment::Drawn,
            moment_drawn(
                flight.draw.set,
                flight.draw.index,
                events.liftoff_s,
                events.landing_s,
            ),
        ),
    ];
    let mut compared = Vec::new();
    for (moment, t) in moments {
        let (Some(true_state), Some(state)) =
            (truth.at(t), smoothed.at(t + flight.draw.logger.pad_s))
        else {
            continue;
        };
        compared.push((
            moment,
            StateName::Height,
            Compared {
                error: state.height_m - (true_state.height_m - truth.pad_height_m),
                sd: state.height_sd_m,
            },
        ));
        compared.push((
            moment,
            StateName::Speed,
            Compared {
                error: state.speed_m_s - true_state.vertical_speed_m_s,
                sd: state.speed_sd_m_s,
            },
        ));
    }
    Checked {
        profile,
        withheld: None,
        compared,
    }
}

/// One state at one moment over a set's flights.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cell {
    /// The moment.
    pub moment: Moment,
    /// The state.
    pub state: StateName,
    /// The flights compared.
    pub flights: u64,
    /// Those whose truth lay below the interval.
    pub below: u64,
    /// Those whose truth lay above it.
    pub above: u64,
    /// The share inside it.
    pub coverage: f64,
    /// The lower edge of the 99.9% Clopper–Pearson interval on each side's miss share: below,
    /// then above.
    pub lowest_miss_share: [f64; 2],
    /// Whether neither side is shown to miss more than 5%.
    pub holds: bool,
    /// The median standard error, in the state's unit.
    pub median_sd: f64,
}

/// A set's check.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCheck {
    /// The flights checked.
    pub flights: u64,
    /// Those withheld, by reason.
    pub withheld: BTreeMap<String, u64>,
    /// Every moment and state.
    pub cells: Vec<Cell>,
    /// The same, per logger.
    pub by_logger: BTreeMap<String, Vec<Cell>>,
}

impl SetCheck {
    /// Whether every cell holds.
    pub fn holds(&self) -> bool {
        self.cells.iter().all(|c| c.holds)
    }
}

/// The confidence of the band on a miss share.
pub const BAND_CONFIDENCE: f64 = 0.999;

/// The miss share each side of a 90% interval allows.
pub const MISS_SHARE: f64 = 0.05;

/// `ln k!`.
fn ln_factorial(k: u64) -> f64 {
    (2..=k).map(|i| (i as f64).ln()).sum()
}

/// `P(X ≥ x)` for `X ~ Binomial(n, p)`.
fn upper_tail(x: u64, n: u64, p: f64) -> f64 {
    if x == 0 {
        return 1.0;
    }
    if p <= 0.0 {
        return 0.0;
    }
    if p >= 1.0 {
        return 1.0;
    }
    let ln_n = ln_factorial(n);
    let (lp, lq) = (p.ln(), (1.0 - p).ln());
    let mut ln_k = 0.0;
    let mut below = 0.0;
    for k in 0..x {
        if k > 0 {
            ln_k += (k as f64).ln();
        }
        let ln_choose = ln_n - ln_k - ln_factorial(n - k);
        below += (ln_choose + k as f64 * lp + (n - k) as f64 * lq).exp();
    }
    (1.0 - below).max(0.0)
}

/// The lower edge of the two-sided Clopper–Pearson interval on a binomial share, at `confidence`:
/// the `p` at which `P(X ≥ x) = (1 − confidence)/2`, by bisection; 0 when `x` is 0 (C. J. Clopper
/// and E. S. Pearson, *Biometrika* 26(4), 404–413, 1934).
pub fn clopper_pearson_lower(x: u64, n: u64, confidence: f64) -> f64 {
    if x == 0 || n == 0 {
        return 0.0;
    }
    let target = 0.5 * (1.0 - confidence);
    let (mut lo, mut hi) = (0.0_f64, x as f64 / n as f64);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if upper_tail(x, n, mid) < target {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

fn cell(moment: Moment, state: StateName, compared: &[Compared]) -> Cell {
    let flights = compared.len() as u64;
    let below = compared.iter().filter(|c| c.truth_below()).count() as u64;
    let above = compared.iter().filter(|c| c.truth_above()).count() as u64;
    let lowest = [
        clopper_pearson_lower(below, flights, BAND_CONFIDENCE),
        clopper_pearson_lower(above, flights, BAND_CONFIDENCE),
    ];
    let mut sds: Vec<f64> = compared.iter().map(|c| c.sd).collect();
    sds.sort_by(f64::total_cmp);
    let median_sd = if sds.is_empty() {
        f64::NAN
    } else {
        sds[sds.len() / 2]
    };
    Cell {
        moment,
        state,
        flights,
        below,
        above,
        coverage: if flights == 0 {
            f64::NAN
        } else {
            (flights - below - above) as f64 / flights as f64
        },
        lowest_miss_share: lowest,
        holds: lowest.iter().all(|&l| l <= MISS_SHARE),
        median_sd,
    }
}

fn cells(checks: &[&Checked]) -> Vec<Cell> {
    let mut out = Vec::new();
    for moment in Moment::ALL {
        for state in StateName::ALL {
            let compared: Vec<Compared> = checks
                .iter()
                .flat_map(|c| c.compared.iter())
                .filter(|(m, s, _)| *m == moment && *s == state)
                .map(|(_, _, c)| *c)
                .collect();
            out.push(cell(moment, state, &compared));
        }
    }
    out
}

/// A set's check from its flights' checks.
pub fn tally(checks: &[Checked]) -> SetCheck {
    let mut withheld = BTreeMap::new();
    for check in checks {
        if let Some(reason) = check.withheld {
            *withheld.entry(format!("{reason:?}")).or_insert(0) += 1;
        }
    }
    let all: Vec<&Checked> = checks.iter().collect();
    let mut by_logger = BTreeMap::new();
    for profile in Profile::ALL {
        let mine: Vec<&Checked> = checks
            .iter()
            .filter(|c| c.profile == profile && c.withheld.is_none())
            .collect();
        if !mine.is_empty() {
            by_logger.insert(profile.name().to_owned(), cells(&mine));
        }
    }
    SetCheck {
        flights: checks.len() as u64,
        withheld,
        cells: cells(&all),
        by_logger,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clopper_pearson_matches_the_definition() {
        // The lower edges from the definition, `P(X ≥ x) = (1 − confidence)/2`, with exact integer
        // binomial coefficients in 60-digit decimal arithmetic (Python's `math.comb`, `decimal`).
        let lower = clopper_pearson_lower(5, 100, 0.95);
        assert!((lower - 0.016_431_879).abs() < 1e-8, "{lower}");
        let lower = clopper_pearson_lower(50, 1000, 0.999);
        assert!((lower - 0.030_238_692).abs() < 1e-8, "{lower}");
        let lower = clopper_pearson_lower(75, 1000, 0.999);
        assert!((lower - 0.050_351_908).abs() < 1e-8, "{lower}");
        assert_eq!(clopper_pearson_lower(0, 1000, 0.999), 0.0);
    }

    #[test]
    fn a_cell_holds_until_its_misses_show_more_than_five_percent() {
        let make = |misses: usize| -> Vec<Compared> {
            (0..1000)
                .map(|k| Compared {
                    error: if k < misses { 2.0 } else { 0.0 },
                    sd: 1.0,
                })
                .collect()
        };
        assert!(cell(Moment::Apogee, StateName::Height, &make(50)).holds);
        // 74 misses in 1,000: the band's lower edge 0.0495; 75: 0.0504.
        assert!(cell(Moment::Apogee, StateName::Height, &make(74)).holds);
        assert!(!cell(Moment::Apogee, StateName::Height, &make(75)).holds);
    }
}

#[cfg(test)]
mod debug_probe {
    use super::*;
    use crate::synthetic::designs::DesignFiles;
    use crate::synthetic::Flyer;

    #[test]
    #[ignore]
    fn port_truth() {
        let mut flyer = Flyer::new(DesignFiles::committed());
        for index in 0..200 {
            let Ok(flight) = flyer.flight(Set::Tuning, index) else { continue };
            let cp = crate::synthetic::noise::port_pressure_coefficient(flight.draw.logger.port_calibers, 0.5);
            println!("PORTTRUTH {index} {} {cp:.5}", flight.draw.logger.profile.name());
            let _ = smoother::smooth(&flight.log);
        }
    }

    #[test]
    #[ignore]
    fn list_misses() {
        let want = std::env::var("PROFILE").unwrap_or("imu-500hz".into());
        let moment = std::env::var("MOMENT").unwrap_or("Apogee".into());
        let mut flyer = Flyer::new(DesignFiles::committed());
        for index in 0..200 {
            let Ok(flight) = flyer.flight(Set::Tuning, index) else { continue };
            if flight.draw.logger.profile.name() != want { continue; }
            let c = check(&flight, Options::default());
            let line: Vec<String> = c.compared.iter().filter(|(m, _, _)| format!("{m:?}") == moment).map(|(_, st, cmp)| format!("{st:?} err {:8.2} sd {:6.2} z {:6.2}", cmp.error, cmp.sd, cmp.error / cmp.sd)).collect();
            println!("{index:3} T{:+5.1} port {:.2} {}", flight.truth.temperature_offset_k, flight.draw.logger.port_calibers, line.join(" | "));
        }
    }

    #[test]
    #[ignore]
    fn print_one() {
        let want = std::env::var("PROFILE").unwrap_or("imu-500hz".into());
        let only: Option<u64> = std::env::var("INDEX").ok().and_then(|i| i.parse().ok());
        let mut flyer = Flyer::new(DesignFiles::committed());
        for index in 0..200 {
            if only.is_some_and(|o| o != index) { continue; }
            let Ok(flight) = flyer.flight(Set::Tuning, index) else { continue };
            if flight.draw.logger.profile.name() != want { continue; }
            let s = smoother::smooth(&flight.log).unwrap();
            let pad = flight.draw.logger.pad_s;
            println!("index {index} liftoff {:?} truth liftoff {} apogee {} burnout {}", s.liftoff, flight.truth.events.liftoff_s + pad, flight.truth.events.apogee_s + pad, flight.truth.events.burnout_s+pad);
            for sh in &s.health.sensors { println!("{:?} n {} used {} rej {} unw {} resets {} largest {:?} nis {:?}", sh.sensor, sh.samples, sh.used, sh.rejected, sh.unweighed, sh.resets, sh.largest, sh.mean_nis); }
            println!("deweighted {:?} unsmoothed {}", s.health.deweighted, s.health.unsmoothed_steps);
            println!("accel scale {} bias {} temp {} port cal {}", flight.draw.logger.accel_scale, flight.draw.logger.accel_bias_m_s2, flight.truth.temperature_offset_k, flight.draw.logger.port_calibers);
            for tt in [2.0, 3.0, 3.5, 4.0, 5.0, 6.0, 7.0, 8.0, 10.0] {
                let a0 = flight.truth.at(tt - 0.05).unwrap();
                let a1 = flight.truth.at(tt + 0.05).unwrap();
                let acc = (a1.vertical_speed_m_s - a0.vertical_speed_m_s) / 0.1;
                let m = flight.truth.at(tt).unwrap();
                let vh = (m.velocity_enu_m_s[0].powi(2) + m.velocity_enu_m_s[1].powi(2)).sqrt();
                println!("truth t {tt} f {:.2} a+g {:.2} v {:.1} vh {:.1} mach {:.2}", m.axial_specific_force_m_s2, acc + 9.80665, m.vertical_speed_m_s, vh, m.mach);
            }
            let mut t = 0.0;
            while t < flight.truth.events.landing_s {
                let tr = flight.truth.at(t).unwrap();
                let st = s.at(t + pad).unwrap();
                println!("{t:6.2} h {:9.2} {:9.2} ±{:7.2}  v {:8.2} {:8.2} ±{:6.2}", tr.height_m - flight.truth.pad_height_m, st.height_m, st.height_sd_m, tr.vertical_speed_m_s, st.speed_m_s, st.speed_sd_m_s);
                let fine = std::env::var("FINE").is_ok() && (t - flight.truth.events.apogee_s).abs() < 1.5;
                t += if fine { 0.1 } else if t < flight.truth.events.apogee_s + 2.0 { 0.5 } else { 5.0 };
            }
            break;
        }
    }
}
