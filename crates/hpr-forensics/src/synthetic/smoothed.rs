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
//! **Holding** ([ADR-300][adr-300]): a 90% interval misses about 5% of the time on each side. For
//! each state and moment, each side's misses are counted over a set's smoothed flights, and the
//! smoother holds there when that count's two-sided 99.9% Clopper–Pearson interval reaches down to
//! 5%: the flights don't show it misses more often than it says. An interval wider than it needs
//! to be holds; one that misses too often doesn't.
//!
//! [m7-2d]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-2d
//! [adr-300]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0300-the-smoother-and-its-check.md

use std::collections::BTreeMap;

use hpr_core::random::SeededRng;
use hpr_flightdata::log::{FlightLog, Quantity};
use hpr_flightdata::smoother::{self, Deweighted, Health, INTERVAL_Z, Options, Reason, Sensor};
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
    /// Its health card; `None` if it was withheld.
    pub health: Option<Health>,
    /// Whether its log has a GNSS track.
    pub has_gnss: bool,
}

/// The moment drawn for flight `index` of `set`, s on the truth's clock, uniformly between
/// `liftoff_s` and `landing_s`, on a stream keyed by the set's seed, the index and the
/// smoother's own key, so no other draw moves.
pub fn moment_drawn(set: Set, index: u64, liftoff_s: f64, landing_s: f64) -> f64 {
    let mut rng = SeededRng::for_stream(set.seed(), &[index, super::SMOOTHER_MOMENT_KEY]);
    liftoff_s + rng.uniform() * (landing_s - liftoff_s)
}

/// The flight's own log as a logger that writes only from launch would have written it ([#982]):
/// every sample of every sensor before the truth's liftoff is gone, the clock and the logger's zero
/// as they were. A track of pressures has no zero without its pad and is withheld, as a real one is.
///
/// [#982]: https://github.com/nrdptel/fusionspace-eridanus/issues/982
pub fn launch_started(flight: &Flight) -> FlightLog {
    let from_s = flight.draw.logger.pad_s + flight.truth.events.liftoff_s;
    let mut log = flight.log.clone();
    let first = log.time_s.partition_point(|t| *t < from_s);
    log.time_s.drain(..first);
    if !log.altitude_m.is_empty() {
        log.altitude_m.drain(..first);
    }
    for channel in &mut log.channels {
        let first = channel.time_s.partition_point(|t| *t < from_s);
        channel.time_s.drain(..first);
        channel.values.drain(..first);
    }
    log
}

/// Checks one flight's smoothed log against its truth.
pub fn check(flight: &Flight, options: Options) -> Checked {
    check_log(flight, &flight.log, options)
}

/// Checks a log of `flight`, its own or one written from it, against the flight's truth.
pub fn check_log(flight: &Flight, log: &FlightLog, options: Options) -> Checked {
    let profile = flight.draw.logger.profile;
    let has_gnss = log
        .channels
        .iter()
        .any(|c| c.quantity == Quantity::GnssAltitude);
    let smoothed = match smoother::smooth_with(log, &hpr_flightdata::readings::read(log), options) {
        Ok(smoothed) => smoothed,
        Err(withheld) => {
            return Checked {
                profile,
                withheld: Some(withheld.reason),
                compared: Vec::new(),
                health: None,
                has_gnss,
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
            // A moment the smoothed track doesn't reach misses on both sides, so every smoothed
            // flight counts in every cell.
            for state in StateName::ALL {
                compared.push((
                    moment,
                    state,
                    Compared {
                        error: f64::NAN,
                        sd: f64::NAN,
                    },
                ));
            }
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
        health: Some(smoothed.health),
        has_gnss,
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
    /// Whether it has flights and neither side is shown to miss more than 5%.
    pub holds: bool,
    /// The median standard error, in the state's unit.
    pub median_sd: f64,
    /// The largest, in the state's unit: an interval can hold by being too wide to say anything.
    pub largest_sd: f64,
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
    /// The smoothed flights' health cards, summed.
    pub health: HealthTally,
}

/// One sensor's health over a set's smoothed flights.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SensorTally {
    /// The logs that have it.
    pub logs: u64,
    /// Its samples.
    pub samples: u64,
    /// Those the gate rejected.
    pub rejected: u64,
    /// Those not weighed.
    pub unweighed: u64,
    /// How many times it was taken back after a second of rejections.
    pub resets: u64,
    /// The logs with at least one rejection.
    pub logs_rejecting: u64,
}

/// A set's health cards, summed.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct HealthTally {
    /// Each sensor, by [`Sensor::name`].
    pub sensors: BTreeMap<String, SensorTally>,
    /// The spans the barometer was de-weighted over as transonic.
    pub transonic_spans: u64,
    /// Those after a charge.
    pub charge_spans: u64,
    /// The logs whose GNSS receiver was trusted at some point.
    pub gnss_trusted: u64,
    /// The steps the pass back couldn't solve, over every log.
    pub unsmoothed_steps: u64,
    /// The logs smoothed.
    pub logs: u64,
    /// Those on an assumed pad.
    pub assumed_pad: u64,
    /// Those whose accelerometer was set aside.
    pub accelerometer_set_aside: u64,
    /// Those whose smoothed apogee passes the reading's bound.
    pub apogee_past_reading_bound: u64,
}

impl HealthTally {
    fn add(&mut self, health: &Health) {
        for s in &health.sensors {
            let t = self.sensors.entry(s.sensor.name().to_owned()).or_default();
            t.logs += 1;
            t.samples += s.samples as u64;
            t.rejected += s.rejected as u64;
            t.unweighed += s.unweighed as u64;
            t.resets += s.resets as u64;
            t.logs_rejecting += u64::from(s.rejected > 0);
        }
        for span in &health.deweighted {
            match span.why {
                Deweighted::Transonic => self.transonic_spans += 1,
                Deweighted::Charge => self.charge_spans += 1,
            }
        }
        let has_gnss = health.sensors.iter().any(|s| s.sensor == Sensor::Gnss);
        self.gnss_trusted += u64::from(has_gnss && health.gnss_trusted_s.is_some());
        self.unsmoothed_steps += health.unsmoothed_steps as u64;
        self.logs += 1;
        self.assumed_pad += u64::from(health.assumed_pad);
        self.accelerometer_set_aside += u64::from(health.accelerometer_set_aside);
        self.apogee_past_reading_bound += u64::from(health.apogee_past_reading_bound);
    }
}

impl SetCheck {
    /// The logs withheld for a reason other than having no barometer: a log the smoother can't
    /// finish would otherwise leave every cell, its misses with it, so each one fails the check.
    pub fn lost(&self) -> u64 {
        let no_barometer = format!("{:?}", Reason::NoBarometer);
        self.withheld
            .iter()
            .filter(|(reason, _)| **reason != no_barometer)
            .map(|(_, n)| n)
            .sum()
    }

    /// Whether every cell holds, the set's and each logger's, and no log was [`lost`](Self::lost).
    pub fn holds(&self) -> bool {
        self.lost() == 0
            && self
                .cells
                .iter()
                .chain(self.by_logger.values().flatten())
                .all(|c| c.holds)
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
/// and E. S. Pearson, *Biometrika* 26(4), 404–413, 1934). NaN, which no cell holds on, when `x`
/// exceeds `n` or `confidence` isn't strictly between 0 and 1.
pub fn clopper_pearson_lower(x: u64, n: u64, confidence: f64) -> f64 {
    if x > n || !(confidence > 0.0 && confidence < 1.0) {
        return f64::NAN;
    }
    if x == 0 {
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

/// The upper edge of the two-sided Clopper–Pearson interval on a binomial share at `confidence`,
/// `1 −` the lower edge of the share of the other outcome; 1 when `x` is `n`. NaN where
/// [`clopper_pearson_lower`] is.
pub fn clopper_pearson_upper(x: u64, n: u64, confidence: f64) -> f64 {
    if x > n {
        return f64::NAN;
    }
    1.0 - clopper_pearson_lower(n - x, n, confidence)
}

fn cell(moment: Moment, state: StateName, compared: &[Compared]) -> Cell {
    let flights = compared.len() as u64;
    let below = compared.iter().filter(|c| c.truth_below()).count() as u64;
    let above = compared.iter().filter(|c| c.truth_above()).count() as u64;
    // Counted apart: a comparison that isn't a number misses on both sides.
    let inside = compared
        .iter()
        .filter(|c| !c.truth_below() && !c.truth_above())
        .count() as u64;
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
    let largest_sd = sds.last().copied().unwrap_or(f64::NAN);
    Cell {
        moment,
        state,
        flights,
        below,
        above,
        coverage: if flights == 0 {
            f64::NAN
        } else {
            inside as f64 / flights as f64
        },
        lowest_miss_share: lowest,
        // A cell of no flights shows nothing.
        holds: flights > 0 && lowest.iter().all(|&l| l <= MISS_SHARE),
        median_sd,
        largest_sd,
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
    let mut health = HealthTally::default();
    for h in checks.iter().filter_map(|c| c.health.as_ref()) {
        health.add(h);
    }
    SetCheck {
        flights: checks.len() as u64,
        withheld,
        cells: cells(&all),
        by_logger,
        health,
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
        // More misses than flights, or a confidence that isn't one, has no edge.
        assert!(clopper_pearson_lower(5, 3, 0.999).is_nan());
        assert!(clopper_pearson_lower(1, 0, 0.999).is_nan());
        assert!(clopper_pearson_lower(5, 100, f64::NAN).is_nan());
        assert!(clopper_pearson_lower(5, 100, 1.0).is_nan());
        assert!(clopper_pearson_lower(5, 100, 0.0).is_nan());
    }

    #[test]
    fn the_upper_edge_mirrors_the_lower_one() {
        // ADR-257 §1: at 1,000 flights a tail passes with 29 to 74 misses, its band holding 5%.
        let holds = |x| {
            clopper_pearson_lower(x, 1000, 0.999) <= 0.05
                && 0.05 <= clopper_pearson_upper(x, 1000, 0.999)
        };
        assert!(!holds(28) && holds(29) && holds(74) && !holds(75));
        // ADR-308 §3: at set A's 820 readings, 22 to 63.
        let holds_820 = |x| {
            clopper_pearson_lower(x, 820, 0.999) <= 0.05
                && 0.05 <= clopper_pearson_upper(x, 820, 0.999)
        };
        assert!(!holds_820(21) && holds_820(22) && holds_820(63) && !holds_820(64));
        // And at set B's 845, 23 to 64.
        let holds_845 = |x| {
            clopper_pearson_lower(x, 845, 0.999) <= 0.05
                && 0.05 <= clopper_pearson_upper(x, 845, 0.999)
        };
        assert!(!holds_845(22) && holds_845(23) && holds_845(64) && !holds_845(65));
        assert_eq!(clopper_pearson_upper(1000, 1000, 0.999), 1.0);
        assert!(clopper_pearson_upper(5, 3, 0.999).is_nan());
    }

    #[test]
    fn a_log_withheld_with_a_barometer_fails_the_check() {
        let inside = Compared {
            error: 0.0,
            sd: 1.0,
        };
        let smoothed = |profile: Profile| Checked {
            profile,
            withheld: None,
            compared: Moment::ALL
                .iter()
                .flat_map(|&m| StateName::ALL.iter().map(move |&s| (m, s, inside)))
                .collect(),
            health: None,
            has_gnss: false,
        };
        let withheld = |profile: Profile, reason: Reason| Checked {
            profile,
            withheld: Some(reason),
            compared: Vec::new(),
            health: None,
            has_gnss: false,
        };
        let profile = Profile::ALL[0];
        let mut checks: Vec<Checked> = (0..100).map(|_| smoothed(profile)).collect();
        checks.push(withheld(profile, Reason::NoBarometer));
        let set = tally(&checks);
        assert_eq!(set.lost(), 0);
        assert!(set.holds());
        for reason in [Reason::Diverged, Reason::NoLiftoff, Reason::TooLong] {
            let mut lost = checks.clone();
            lost.push(withheld(profile, reason));
            let set = tally(&lost);
            assert_eq!(set.lost(), 1, "{reason:?}");
            // Every cell still holds: the log left them, which is what the check must catch.
            assert!(set.cells.iter().all(|c| c.holds), "{reason:?}");
            assert!(!set.holds(), "{reason:?}");
        }
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

    #[test]
    fn a_comparison_that_isnt_a_number_misses_and_an_empty_cell_shows_nothing() {
        let nan = Compared {
            error: f64::NAN,
            sd: 1.0,
        };
        let inside = Compared {
            error: 0.0,
            sd: 1.0,
        };
        let c = cell(Moment::Apogee, StateName::Height, &[nan, inside, inside]);
        assert_eq!((c.flights, c.below, c.above), (3, 1, 1));
        assert!((c.coverage - 2.0 / 3.0).abs() < 1e-12, "{}", c.coverage);
        assert!(!cell(Moment::Apogee, StateName::Height, &[]).holds);
    }
}
