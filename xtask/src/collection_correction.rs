//! `cargo xtask fixture-flights` fits the correction of the bias the simulators share on the
//! calibration half and judges it on the held-out half and the public flights (M7.5b, ADR-346).
//!
//! **The correction** is two scales, fitted as ADR-346 §4 fixes them before any was fitted. The
//! drag scale $k_D$ comes from the calibration flights' coasts: for each flight without a
//! deployment before apogee, the drag $d_i$ at which its boost, refitted with the drag held at $d$
//! and flown on, reaches the logged apogee ([`solve_drag`]); $k_D$ is their mean, with a Student's
//! $t$ interval. The impulse scale $k_I$ comes from the boosts, refitted at that drag: a weighted
//! mean with weights $1/(s_i^2 + \tau^2)$, its interval from the scales' own weighted spread
//! ([`ScaleFit::impulse`]). Each enters only if its interval excludes 1 over at least [`MIN_FITS`]
//! values, rounded to two decimals ([`two_decimals`]). Two fits are made: over every calibration
//! flight predicted, and without the flagged ones (ADR-346 §3).
//!
//! **The test** flies each fit's correction on six sets: the held-out flights the forward report
//! compares, those without a deployment before apogee, the public flights the public report
//! compares, and each less its flagged flights ([`SetJudged`]). On the four held-out sets the
//! mean absolute error and the absolute bias must each be lower than the forward simulation's; on
//! the two public sets neither may be higher. Both fits must pass every set.
//!
//! **What is published** follows ADR-346 §7 ([`Judged::published`]): where fewer than
//! [`SMALL_SET`] calibration flights are flagged, the fit without them is given as flown (its
//! scales to two decimals, its counts, whether each interval excludes 1) and the other fit by its
//! pass or fail on each set; a collection set less fewer than [`SMALL_SET`] flagged flights is
//! given by its count and its pass or fail. The whole is written to the gitignored [`HELD_BACK`],
//! and its SHA-256 committed in [`HELD_BACK_SHA`].

use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use hpr_forensics::boost;
use hpr_forensics::fit::Stated;
use hpr_forensics::synthetic::fits::Asked;
use hpr_validate::correction::Correction;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// The bootstrap's seed beside the rule: the record's number (ADR-346 §5).
pub const SEED: u64 = 346;
/// The lower end of the drag's bisection.
pub const DRAG_LOW: f64 = 0.25;
/// Its upper end.
pub const DRAG_HIGH: f64 = 4.0;
/// How close the bisection's two ends come before it stops.
pub const DRAG_WITHIN: f64 = 0.001;
/// How many times narrower than its stated range the drag is told, to hold it at a trial value.
pub const DRAG_HELD: f64 = 1000.0;
/// The fewest values a scale may enter on.
pub const MIN_FITS: usize = 3;
/// Fewer flagged flights than this between two published sets would give those flights' own
/// values by subtraction (ADR-346 §7).
pub const SMALL_SET: usize = 3;
/// Everything the test computed, the numbers the page holds back included (private,
/// gitignored).
pub const HELD_BACK: &str = "corpus-out/hpr-sim-fixtures/correction-held-back.json";
/// The SHA-256 of [`HELD_BACK`], committed.
pub const HELD_BACK_SHA: &str = "validation/reports/correction-held-back.sha256";
/// The fit over every calibration flight predicted.
pub const EVERY: &str = "every calibration flight";
/// The fit without the flagged ones: the one that would ship (ADR-346 §8).
pub const UNFLAGGED: &str = "without the flagged flights";

/// $P(|T| < t)$ for Student's $t$ with `nu` degrees of freedom, by the closed forms for an
/// integer `nu` (Abramowitz and Stegun, *Handbook of Mathematical Functions*, 1964, 26.7.3 for
/// `nu` odd and 26.7.4 for `nu` even), with $\theta = \arctan(t/\sqrt{\nu})$:
/// odd, $\frac{2}{\pi}\left[\theta + \sin\theta\left(\cos\theta + \frac{2}{3}\cos^3\theta +
/// \dots + \frac{2\cdot4\cdots(\nu-3)}{1\cdot3\cdots(\nu-2)}\cos^{\nu-2}\theta\right)\right]$
/// ($2\theta/\pi$ for $\nu = 1$); even, $\sin\theta\left(1 + \frac{1}{2}\cos^2\theta + \dots +
/// \frac{1\cdot3\cdots(\nu-3)}{2\cdot4\cdots(\nu-2)}\cos^{\nu-2}\theta\right)$.
fn within_t(t: f64, nu: u32) -> f64 {
    let theta = (t / f64::from(nu).sqrt()).atan();
    let (sin, cos) = theta.sin_cos();
    let cos2 = cos * cos;
    if nu % 2 == 1 {
        let mut sum = 0.0;
        let mut term = cos;
        for k in 1..=(nu - 1) / 2 {
            sum += term;
            term *= cos2 * f64::from(2 * k) / f64::from(2 * k + 1);
        }
        2.0 / std::f64::consts::PI * (theta + sin * sum)
    } else {
        let mut sum = 0.0;
        let mut term = 1.0;
        for k in 0..nu / 2 {
            sum += term;
            term *= cos2 * f64::from(2 * k + 1) / f64::from(2 * k + 2);
        }
        sin * sum
    }
}

/// The 97.5% point of Student's $t$ with `nu` degrees of freedom: the $t$ with
/// $P(|T| < t) = 0.95$, by bisection on [`within_t`]; `None` for no degrees of freedom.
pub fn t_975(nu: u32) -> Option<f64> {
    if nu == 0 {
        return None;
    }
    let (mut low, mut high) = (0.0_f64, 1000.0_f64);
    for _ in 0..200 {
        let mid = 0.5 * (low + high);
        if within_t(mid, nu) < 0.95 {
            low = mid;
        } else {
            high = mid;
        }
    }
    Some(0.5 * (low + high))
}

/// `value` to two decimals, a tie going up (ADR-346 §4): the scales are positive, so half away
/// from zero is up.
pub fn two_decimals(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

/// What a calibration flight's boost is told for a trial drag `d` (ADR-346 §4): the log alone's
/// fit (`stated_alone`, the impulse's range widened), the drag held at `d` to a
/// [`DRAG_HELD`]-th of its stated standard deviation.
pub fn stated_at_drag(stated: Stated, d: f64) -> Stated {
    let mut told = crate::collection_errors::stated_alone(stated);
    let drag = &mut told[Asked::DragScale.index()];
    drag.mean = d;
    drag.sd /= DRAG_HELD;
    told
}

/// What a calibration flight's boost is told for the impulse scale at a drag scale `k_d`
/// (ADR-346 §4): the log alone's fit, its drag centered on `k_d` to its stated standard
/// deviation.
pub fn stated_told_drag(stated: Stated, k_d: f64) -> Stated {
    let mut told = crate::collection_errors::stated_alone(stated);
    told[Asked::DragScale.index()].mean = k_d;
    told
}

/// The drag $d$ at which a flight's boost, fitted with the drag held at $d$ and flown on,
/// reaches `logged_m` (ADR-346 §4): bisection from [`DRAG_LOW`] and [`DRAG_HIGH`] until the ends
/// are within [`DRAG_WITHIN`], the midpoint of the last pair. `apogee_at(d)` is that flight's
/// apogee above the pad, or why a fit came to none or didn't converge. The apogee falls as the
/// drag rises, so the ends must lie on opposite sides of the logged apogee.
///
/// # Errors
///
/// Ends on one side of the logged apogee, or a fit along the way that came to nothing.
pub fn solve_drag(
    mut apogee_at: impl FnMut(f64) -> Result<f64, String>,
    logged_m: f64,
) -> Result<f64, String> {
    let miss = |apogee_m: f64| apogee_m - logged_m;
    let (mut low, mut high) = (DRAG_LOW, DRAG_HIGH);
    let low_miss = miss(apogee_at(low)?);
    let high_miss = miss(apogee_at(high)?);
    if !(low_miss.is_finite() && high_miss.is_finite()) {
        return Err("an apogee that is no number".to_owned());
    }
    if low_miss == 0.0 {
        return Ok(low);
    }
    if high_miss == 0.0 {
        return Ok(high);
    }
    if low_miss.signum() == high_miss.signum() {
        return Err(format!(
            "its apogees at drag scales {DRAG_LOW} and {DRAG_HIGH} lie on one side of the logged \
             apogee"
        ));
    }
    while high - low > DRAG_WITHIN {
        let mid = 0.5 * (low + high);
        let at = miss(apogee_at(mid)?);
        if !at.is_finite() {
            return Err("an apogee that is no number".to_owned());
        }
        if at == 0.0 {
            return Ok(mid);
        }
        if at.signum() == low_miss.signum() {
            low = mid;
        } else {
            high = mid;
        }
    }
    Ok(0.5 * (low + high))
}

/// One scale of a fit across the calibration flights, as computed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScaleFit {
    /// The flights it is taken over: the fit's flights without a deployment before apogee for the
    /// drag, every one for the impulse.
    pub flights: usize,
    /// Those with a value: a drag solved, or an impulse fit that converged.
    pub with_value: usize,
    /// The scale before rounding; `None` with no value, or where the page holds it back.
    pub mean: Option<f64>,
    /// Its 95% interval's lower end.
    pub low: Option<f64>,
    /// Its upper end.
    pub high: Option<f64>,
    /// Whether the interval excludes 1; `None` where there is no interval.
    pub excludes_one: Option<bool>,
    /// The scale as flown: two decimals where it entered, else 1; `None` where held back.
    pub flown: Option<f64>,
    /// Why it didn't enter, if it didn't; `None` where held back.
    pub why_not: Option<String>,
}

impl ScaleFit {
    /// The scale from its values' mean and interval half-width, by ADR-346 §4's entry rule.
    /// `every_needed`: the drag's rule, under which a flight with no value keeps the scale at 1.
    fn of(
        flights: usize,
        with_value: usize,
        center_and_half_width: Option<(f64, f64)>,
        every_needed: bool,
    ) -> Self {
        let (mean, low, high) = match center_and_half_width {
            Some((center, half)) => (Some(center), Some(center - half), Some(center + half)),
            None => (None, None, None),
        };
        let excludes_one = low.zip(high).map(|(low, high)| low > 1.0 || high < 1.0);
        let why_not = if every_needed && with_value < flights {
            Some(format!(
                "{} of its {flights} flights have none",
                flights - with_value
            ))
        } else if with_value < MIN_FITS {
            Some(format!("fewer than {MIN_FITS} values"))
        } else if excludes_one != Some(true) {
            Some("its interval holds 1".to_owned())
        } else if mean.map(two_decimals) == Some(1.0) {
            Some("it rounds to 1.00".to_owned())
        } else {
            None
        };
        let flown = match (why_not.is_none(), mean) {
            (true, Some(mean)) => two_decimals(mean),
            _ => 1.0,
        };
        Self {
            flights,
            with_value,
            mean,
            low,
            high,
            excludes_one,
            flown: Some(flown),
            why_not,
        }
    }

    /// The drag scale from each coast's solved drag (`None`: none was found): the mean, give or
    /// take $t$ times the standard deviation over $\sqrt{n}$, $t$ at $n - 1$.
    pub fn drag(solved: &[Option<f64>]) -> Self {
        let values: Vec<f64> = solved.iter().flatten().copied().collect();
        let n = values.len();
        let center = mean(&values);
        let spread = sample_sd(&values);
        let half = spread
            .zip(u32::try_from(n).ok().and_then(|n| t_975(n.checked_sub(1)?)))
            .map(|(sd, t)| t * sd / (n as f64).sqrt());
        Self::of(solved.len(), n, center.zip(half), true)
    }

    /// The impulse scale from each boost's `(scale, standard error)`, the converged fits only
    /// (`None`: not converged), with the between-flight standard deviation `tau`: the weighted
    /// mean with $w_i = 1/(s_i^2 + \tau^2)$, give or take $t$ times
    /// $\sqrt{\sum w_i (k_i - k_I)^2 / ((n - 1) \sum w_i)}$, $t$ at $n - 1$.
    ///
    /// The weights need only be right relative to each other: the spread is the weighted mean's
    /// variance $1/\sum w_i$ scaled by the Birge ratio squared,
    /// $\sum w_i (k_i - k_I)^2/(n - 1)$ (Merkatas, Toman, Possolo and Schlamminger, "Shades of
    /// dark uncertainty and consensus value for the Newtonian constant of gravitation",
    /// *Metrologia* 56, 054001, 2019, §3.1). With normal errors and weights right up to a common
    /// factor, the mean less the true value over that spread follows Student's $t$ at $n - 1$
    /// (Bodnar and Eriksson, arXiv:2104.01977, 2021, the proof of Theorem 1, as the posterior of
    /// the mean under a reference prior).
    pub fn impulse(fits: &[Option<(f64, f64)>], tau: f64) -> Self {
        let values: Vec<(f64, f64)> = fits.iter().flatten().copied().collect();
        let n = values.len();
        let weights: Vec<f64> = values
            .iter()
            .map(|(_, s)| 1.0 / (s * s + tau * tau))
            .collect();
        let total: f64 = weights.iter().sum();
        let center = (n > 0).then(|| {
            values
                .iter()
                .zip(&weights)
                .map(|((k, _), w)| w * k)
                .sum::<f64>()
                / total
        });
        let half = center.zip(u32::try_from(n).ok()).and_then(|(center, n32)| {
            let t = t_975(n32.checked_sub(1)?)?;
            let scatter: f64 = values
                .iter()
                .zip(&weights)
                .map(|((k, _), w)| w * (k - center).powi(2))
                .sum();
            Some(t * (scatter / ((n as f64 - 1.0) * total)).sqrt())
        });
        Self::of(fits.len(), n, center.zip(half), false)
    }

    /// Whether the scale entered: flown at other than 1.
    pub fn entered(&self) -> bool {
        self.why_not.is_none() && self.flown.is_some_and(|flown| flown != 1.0)
    }

    /// The page's form where fewer than [`SMALL_SET`] calibration flights are flagged
    /// (ADR-346 §7): the scale as flown, the counts and whether the interval excludes 1, not the
    /// scale before rounding or its interval.
    fn as_flown_only(&self) -> Self {
        Self {
            mean: None,
            low: None,
            high: None,
            ..self.clone()
        }
    }

    /// Everything held back but the counts of flights.
    fn held_back(&self) -> Self {
        Self {
            flights: self.flights,
            with_value: self.with_value,
            mean: None,
            low: None,
            high: None,
            excludes_one: None,
            flown: None,
            why_not: None,
        }
    }
}

/// The mean of `values`; `None` for none.
fn mean(values: &[f64]) -> Option<f64> {
    let count = u32::try_from(values.len()).ok().filter(|n| *n > 0)?;
    Some(values.iter().fold(0.0, |sum, v| sum + v) / f64::from(count))
}

/// The sample standard deviation of `values`; `None` for fewer than 2.
fn sample_sd(values: &[f64]) -> Option<f64> {
    let center = mean(values)?;
    let count = u32::try_from(values.len()).ok().filter(|n| *n > 1)?;
    Some((values.iter().map(|v| (v - center).powi(2)).sum::<f64>() / f64::from(count - 1)).sqrt())
}

/// One flight of a set: the forward simulation's apogee error, and the corrected flight's (`None`
/// where it failed to fly), %.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pair {
    /// The forward simulation's error.
    pub forward: f64,
    /// The corrected flight's.
    pub corrected: Option<f64>,
}

/// One set's test, for one fit (ADR-346 §5).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetJudged {
    /// The set's name.
    pub set: String,
    /// Its flights.
    pub flights: usize,
    /// Whether the set is less its flagged flights.
    pub less_flagged: bool,
    /// Its flights whose corrected flight failed to fly, which fails the set.
    pub failed_to_fly: Option<usize>,
    /// The forward simulation's mean absolute apogee error, %.
    pub forward_mean_absolute_percent: Option<f64>,
    /// Its mean error with its sign, %.
    pub forward_mean_percent: Option<f64>,
    /// The corrected flights' mean absolute error, %, over those that flew.
    pub corrected_mean_absolute_percent: Option<f64>,
    /// Their mean error with its sign, %.
    pub corrected_mean_percent: Option<f64>,
    /// Whether the corrected mean absolute error is lower (a held-out set) or no higher (public).
    pub mean_absolute_holds: Option<bool>,
    /// Whether the corrected absolute bias is lower (held out) or no higher (public).
    pub bias_holds: Option<bool>,
    /// Whether the set passes.
    pub passes: bool,
    /// The mean of each flight's corrected absolute error less its forward one, points.
    pub difference_points: Option<f64>,
    /// Its paired bootstrap 95% interval ([`SEED`]).
    pub bootstrap_interval: Option<(f64, f64)>,
    /// Its exact sign-flip 95% interval, each end `None` where unbounded.
    pub sign_flip_interval: Option<(Option<f64>, Option<f64>)>,
    /// The flights whose absolute error grew.
    pub grew: Option<usize>,
}

impl SetJudged {
    /// The set `pairs`; `strict`: a held-out set, which must improve, not merely hold.
    pub fn of(set: &str, less_flagged: bool, pairs: &[Pair], strict: bool) -> Self {
        let failed = pairs.iter().filter(|p| p.corrected.is_none()).count();
        let flew: Vec<(f64, f64)> = pairs
            .iter()
            .filter_map(|p| Some((p.forward, p.corrected?)))
            .collect();
        let forward: Vec<f64> = pairs.iter().map(|p| p.forward).collect();
        let forward_abs: Vec<f64> = forward.iter().map(|e| e.abs()).collect();
        let corrected: Vec<f64> = flew.iter().map(|(_, c)| *c).collect();
        let corrected_abs: Vec<f64> = corrected.iter().map(|e| e.abs()).collect();
        let forward_mae = mean(&forward_abs);
        let forward_mean = mean(&forward);
        let corrected_mae = mean(&corrected_abs);
        let corrected_mean = mean(&corrected);
        let holds = |corrected: Option<f64>, forward: Option<f64>| match (corrected, forward) {
            (Some(c), Some(f)) if failed == 0 => Some(if strict { c < f } else { c <= f }),
            _ => Some(false),
        };
        let mean_absolute_holds = holds(corrected_mae, forward_mae);
        let bias_holds = holds(corrected_mean.map(f64::abs), forward_mean.map(f64::abs));
        let differences: Vec<f64> = flew.iter().map(|(f, c)| c.abs() - f.abs()).collect();
        let finite = |x: f64| x.is_finite().then_some(x);
        Self {
            set: set.to_owned(),
            flights: pairs.len(),
            less_flagged,
            failed_to_fly: Some(failed),
            forward_mean_absolute_percent: forward_mae,
            forward_mean_percent: forward_mean,
            corrected_mean_absolute_percent: corrected_mae,
            corrected_mean_percent: corrected_mean,
            passes: !pairs.is_empty()
                && mean_absolute_holds == Some(true)
                && bias_holds == Some(true),
            mean_absolute_holds,
            bias_holds,
            difference_points: mean(&differences),
            bootstrap_interval: crate::collection_errors::resolves(
                differences.len(),
                crate::collection_errors::LEVEL,
            )
            .then(|| {
                boost::paired_bootstrap_interval(
                    &differences,
                    crate::collection_errors::LEVEL,
                    crate::collection_errors::RESAMPLES,
                    SEED,
                )
            })
            .flatten(),
            sign_flip_interval: boost::sign_flip_interval(
                &differences,
                crate::collection_errors::LEVEL,
            )
            .map(|(low, high)| (finite(low), finite(high))),
            grew: Some(differences.iter().filter(|d| **d > 0.0).count()),
        }
    }

    /// Only its count and its pass or fail (ADR-346 §7).
    fn count_and_verdict(&self) -> Self {
        Self {
            set: self.set.clone(),
            flights: self.flights,
            less_flagged: self.less_flagged,
            failed_to_fly: None,
            forward_mean_absolute_percent: None,
            forward_mean_percent: None,
            corrected_mean_absolute_percent: None,
            corrected_mean_percent: None,
            mean_absolute_holds: None,
            bias_holds: None,
            passes: self.passes,
            difference_points: None,
            bootstrap_interval: None,
            sign_flip_interval: None,
            grew: None,
        }
    }

    /// The set's name in a clause: its own, and whether it is less its flagged flights.
    fn named(&self) -> String {
        if self.less_flagged {
            format!("{}, less their flagged flights", self.set)
        } else {
            self.set.clone()
        }
    }

    /// The clauses it failed, named for the verdict's record.
    fn failed_clauses(&self, fit: &str) -> Vec<String> {
        let mut failed = Vec::new();
        let set = self.named();
        if self.failed_to_fly.is_some_and(|n| n > 0) {
            failed.push(format!("{fit}, {set}: a corrected flight failed to fly"));
        }
        if self.mean_absolute_holds == Some(false) {
            failed.push(format!(
                "{fit}, {set}: the mean absolute error is not {}",
                if self.is_public() {
                    "at most the forward simulation's"
                } else {
                    "lower than the forward simulation's"
                }
            ));
        }
        if self.bias_holds == Some(false) {
            failed.push(format!(
                "{fit}, {set}: the absolute bias is not {}",
                if self.is_public() {
                    "at most the forward simulation's"
                } else {
                    "lower than the forward simulation's"
                }
            ));
        }
        failed
    }

    fn is_public(&self) -> bool {
        self.set.starts_with(PUBLIC)
    }
}

/// The held-out set's name.
pub const HELD_OUT_SET: &str = "held-out flights the forward report compares";
/// The held-out set without a deployment before apogee.
pub const HELD_OUT_BALLISTIC: &str = "… without a deployment before apogee";
/// The public set's name.
pub const PUBLIC: &str = "public flights the public report compares";
/// The calibration half's flights, beside the rule.
pub const CALIBRATION_SET: &str = "calibration flights the forward report compares";

/// One fit across the calibration flights and its test.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FitJudged {
    /// [`EVERY`] or [`UNFLAGGED`].
    pub name: String,
    /// The calibration flights predicted that it is fitted on.
    pub calibration_flights: usize,
    /// The drag scale.
    pub drag: ScaleFit,
    /// The impulse scale, at that drag.
    pub impulse: ScaleFit,
    /// The six sets, in ADR-346 §5's order.
    pub sets: Vec<SetJudged>,
    /// The calibration half's corrected errors, beside the rule: the flights it was fitted on.
    pub calibration: SetJudged,
    /// Whether it passes every set.
    pub passes: bool,
}

/// The test's whole result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Judged {
    /// The bootstrap's resamples.
    pub resamples: u32,
    /// Its seed.
    pub seed: u64,
    /// The impulse's between-flight standard deviation, the fit's stated one.
    pub tau: f64,
    /// The calibration flights predicted and flagged.
    pub flagged_calibration: usize,
    /// The held-out set's flagged flights.
    pub flagged_held_out: usize,
    /// The held-out set's flagged flights without a deployment before apogee.
    pub flagged_held_out_ballistic: usize,
    /// The public set's flagged flights.
    pub flagged_public: usize,
    /// The held-out flights predicted whose log alone was fitted, for the flag.
    pub held_out_log_fits: usize,
    /// The public flights whose boost was fitted alone, for the flag: those the public boost
    /// report cuts and fits.
    pub public_log_fits: usize,
    /// The largest distance, m, of a collection flight flown with both scales at 1 from its
    /// forward apogee.
    pub largest_uncorrected_collection_m: Option<f64>,
    /// The same over the public flights.
    pub largest_uncorrected_public_m: Option<f64>,
    /// The two fits: without the flagged flights, then every calibration flight.
    pub fits: Vec<FitJudged>,
    /// Whether both fits pass every set.
    pub passes: bool,
    /// Each clause failed, by fit and set.
    pub failed: Vec<String>,
    /// Whether the page holds numbers back (ADR-346 §7).
    pub held_back: bool,
}

impl Judged {
    /// The verdict over `fits`, both passing every set or a fail naming each clause.
    pub fn verdict(fits: &[FitJudged]) -> (bool, Vec<String>) {
        let mut failed = Vec::new();
        for fit in fits {
            if !fit.drag.entered() && !fit.impulse.entered() {
                failed.push(format!(
                    "{}: neither scale entered, so no correction",
                    fit.name
                ));
            }
            for set in &fit.sets {
                failed.extend(set.failed_clauses(&fit.name));
            }
        }
        let passes = failed.is_empty() && fits.iter().all(|fit| fit.passes);
        (passes, failed)
    }

    /// What the page gives (ADR-346 §7).
    pub fn published(&self) -> Self {
        let small = self.flagged_calibration < SMALL_SET;
        let flagged_in = |set: &SetJudged| {
            if set.set.starts_with(HELD_OUT_SET) {
                self.flagged_held_out
            } else if set.set.starts_with(HELD_OUT_BALLISTIC) {
                self.flagged_held_out_ballistic
            } else {
                self.flagged_public
            }
        };
        let fits = self
            .fits
            .iter()
            .map(|fit| {
                let unflagged = fit.name == UNFLAGGED;
                let sets = fit
                    .sets
                    .iter()
                    .map(|set| {
                        let public = set.is_public();
                        // The public flights' numbers are given either way.
                        let withheld = (small && !unflagged)
                            || (set.less_flagged && flagged_in(set) < SMALL_SET);
                        if public || !withheld {
                            set.clone()
                        } else {
                            set.count_and_verdict()
                        }
                    })
                    .collect();
                let (drag, impulse, calibration) = match (small, unflagged) {
                    (false, _) => (
                        fit.drag.clone(),
                        fit.impulse.clone(),
                        fit.calibration.clone(),
                    ),
                    (true, true) => (
                        fit.drag.as_flown_only(),
                        fit.impulse.as_flown_only(),
                        fit.calibration.clone(),
                    ),
                    (true, false) => (
                        fit.drag.held_back(),
                        fit.impulse.held_back(),
                        fit.calibration.count_and_verdict(),
                    ),
                };
                FitJudged {
                    name: fit.name.clone(),
                    calibration_flights: fit.calibration_flights,
                    drag,
                    impulse,
                    sets,
                    calibration,
                    passes: fit.passes,
                }
            })
            .collect();
        Self {
            fits,
            held_back: true,
            ..self.clone()
        }
    }

    /// Its counts and verdict hold together.
    pub fn adds_up(&self) -> bool {
        let finite = |v: Option<f64>| v.is_none_or(f64::is_finite);
        self.fits.len() == 2
            && self.fits.iter().all(|fit| {
                fit.sets.len() == 6
                    && fit.passes == fit.sets.iter().all(|s| s.passes)
                    && fit.drag.with_value <= fit.drag.flights
                    && fit.impulse.with_value <= fit.impulse.flights
                    && fit.impulse.flights == fit.calibration_flights
                    && fit.sets.iter().chain([&fit.calibration]).all(|s| {
                        s.failed_to_fly.is_none_or(|n| n <= s.flights)
                            && s.grew.is_none_or(|n| n <= s.flights)
                            && finite(s.forward_mean_absolute_percent)
                            && finite(s.corrected_mean_absolute_percent)
                            && finite(s.forward_mean_percent)
                            && finite(s.corrected_mean_percent)
                            && finite(s.difference_points)
                    })
            })
            && self.passes == (self.failed.is_empty() && self.fits.iter().all(|f| f.passes))
    }
}

/// [`HELD_BACK`]'s text.
pub fn held_back_text(judged: &Judged) -> Result<String, String> {
    Ok(serde_json::to_string_pretty(judged).map_err(|error| error.to_string())? + "\n")
}

/// [`HELD_BACK_SHA`]'s text: the digest of `text`, then what it is.
pub fn held_back_sha_text(text: &str) -> String {
    let digest = Sha256::digest(text.as_bytes());
    let hex: String = digest.iter().map(|b| format!("{b:02x}")).collect();
    format!(
        "{hex}  SHA-256 of the private collection's correction test as computed, the numbers its \
         page holds back included (ADR-346 §7)\n"
    )
}

/// Writes, or with `check` checks, [`HELD_BACK`] and its digest.
pub fn seal_held_back(root: &Path, judged: &Judged, check: bool) -> Result<(), String> {
    let text = held_back_text(judged)?;
    let sealed = held_back_sha_text(&text);
    if check {
        let committed = fs::read_to_string(root.join(HELD_BACK_SHA))
            .map_err(|error| format!("reading {HELD_BACK_SHA}: {error}"))?;
        if committed != sealed {
            return Err(format!(
                "{HELD_BACK_SHA} doesn't hold the SHA-256 of the correction's test as computed; \
                 run `cargo xtask fixture-flights`"
            ));
        }
        return Ok(());
    }
    let private = root.join(HELD_BACK);
    if let Some(parent) = private.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("{HELD_BACK}: {error}"))?;
    }
    fs::write(&private, text).map_err(|error| format!("writing {HELD_BACK}: {error}"))?;
    fs::write(root.join(HELD_BACK_SHA), sealed)
        .map_err(|error| format!("writing {HELD_BACK_SHA}: {error}"))
}

/// Runs `f` on each item, in parallel, keeping the items' order.
pub fn parallel<T: Sync, R: Send>(items: &[T], f: impl Fn(&T) -> R + Sync) -> Vec<R> {
    let threads = std::thread::available_parallelism().map_or(4, usize::from);
    let chunk = items.len().div_ceil(threads).max(1);
    let f = &f;
    std::thread::scope(|scope| {
        let handles: Vec<_> = items
            .chunks(chunk)
            .map(|part| scope.spawn(move || part.iter().map(f).collect::<Vec<R>>()))
            .collect();
        handles
            .into_iter()
            .flat_map(|handle| match handle.join() {
                Ok(part) => part,
                Err(panic) => std::panic::resume_unwind(panic),
            })
            .collect()
    })
}

/// One public flight of the test.
#[derive(Debug, Clone, PartialEq)]
pub struct PublicFlight {
    /// Its id, public.
    pub id: String,
    /// The forward simulation's apogee error in the committed public report, %.
    pub forward_percent: f64,
    /// Each correction's error, in the order given; `None` where the flight failed to fly.
    pub corrected_percent: Vec<Option<f64>>,
    /// Whether its log alone was fitted: the public boost report cuts and fits its log.
    pub fitted: bool,
    /// Whether that fit flags it (ADR-346 §3).
    pub flagged: bool,
}

/// The public flights of the test.
#[derive(Debug, Clone, PartialEq)]
pub struct PublicFlights {
    /// The flights the public report compares, in its order.
    pub flights: Vec<PublicFlight>,
    /// The largest distance, m, of a flight flown with both scales at 1 from its committed apogee.
    pub largest_uncorrected_m: Option<f64>,
}

/// The public flights the committed `real-flights` report compares, each flown as that report
/// flies it with no correction (it must land within `SAME_FLIGHT_WITHIN_M` of the committed
/// apogee) and with each of `corrections`, and each flight whose boost the public boost report
/// cuts fitted alone for its flag. A predicted-first flight's error is taken against the log
/// apogee its comparison read; one whose corrected flight fails to fly counts as failed.
///
/// # Errors
///
/// A committed report that doesn't read, a flight that doesn't fly with no correction, or one
/// that lands farther than the check allows from its committed apogee.
pub fn public_flights(root: &Path, corrections: &[Correction]) -> Result<PublicFlights, String> {
    use hpr_validate::real_flight::{self, FLIGHTS, RealFlightReport};
    let within = crate::collection_errors::SAME_FLIGHT_WITHIN_M;
    let text = fs::read_to_string(root.join(real_flight::REPORT_JSON))
        .map_err(|error| format!("reading {}: {error}", real_flight::REPORT_JSON))?;
    let report: RealFlightReport = serde_json::from_str(&text)
        .map_err(|error| format!("reading {}: {error}", real_flight::REPORT_JSON))?;
    let percent = |apogee_m: f64, log_m: f64| 100.0 * (apogee_m - log_m) / log_m;
    let mut largest: Option<f64> = None;
    let mut note = |distance: f64, id: &str| -> Result<(), String> {
        if !distance.is_finite() || distance > within {
            return Err(format!(
                "{id}: flown with both scales at 1, it lands {distance:.3} m from its committed \
                 apogee"
            ));
        }
        largest = Some(largest.map_or(distance, |l| l.max(distance)));
        Ok(())
    };
    let rows: Vec<_> = FLIGHTS
        .iter()
        .map(|flight| {
            report
                .flights
                .iter()
                .find(|row| row.id == flight.id)
                .map(|row| (flight, row))
                .ok_or_else(|| format!("{}: not in {}", flight.id, real_flight::REPORT_JSON))
        })
        .collect::<Result<_, String>>()?;
    type Flown = Result<(f64, Vec<Option<f64>>, Option<bool>), String>;
    let flown: Vec<Flown> = parallel(&rows, |(flight, row)| {
        let none = real_flight::fly_corrected(root, flight, Correction::NONE)
            .map_err(|error| error.to_string())?;
        let corrected = corrections
            .iter()
            .map(|c| {
                real_flight::fly_corrected(root, flight, *c)
                    .ok()
                    .map(|flown| percent(flown.hpr_apogee_m, row.log_apogee_m))
            })
            .collect();
        let flag = crate::from_boost::log_alone(root, flight, row)?
            .map(|fit| crate::collection_errors::Implied::of(&fit).disagrees());
        Ok((
            (none.hpr_apogee_m - row.hpr_apogee_m).abs(),
            corrected,
            flag,
        ))
    });
    let mut flights = Vec::new();
    for ((_, row), flown) in rows.iter().zip(flown) {
        let (distance, corrected_percent, flag) = flown?;
        note(distance, &row.id)?;
        flights.push(PublicFlight {
            id: row.id.clone(),
            forward_percent: row.apogee_error_percent,
            corrected_percent,
            fitted: flag.is_some(),
            flagged: flag == Some(true),
        });
    }
    // The flights predicted first, flown as their predictions are.
    let none = hpr_validate::public_predictions::run_corrected(root, Correction::NONE)
        .map_err(|error| error.to_string())?;
    let corrected: Vec<_> = corrections
        .iter()
        .map(|c| hpr_validate::public_predictions::run_corrected(root, *c).ok())
        .collect();
    let apogee_of = |report: &hpr_validate::public_predictions::PredictionsReport, id: &str| {
        report
            .flights
            .iter()
            .find(|row| row.id == id)
            .map(|row| row.predicted_apogee_m)
            .or_else(|| {
                report
                    .design_file_flights
                    .iter()
                    .find(|row| row.id == id)
                    .map(|row| row.predicted_apogee_m)
            })
    };
    for compared in &report.predicted_first {
        let id = compared.id.as_str();
        let uncorrected = apogee_of(&none, id)
            .ok_or_else(|| format!("{id}: not flown by the public predictions"))?;
        note((uncorrected - compared.hpr_apogee_m).abs(), id)?;
        flights.push(PublicFlight {
            id: id.to_owned(),
            forward_percent: compared.apogee_error_percent,
            corrected_percent: corrected
                .iter()
                .map(|report| {
                    report
                        .as_ref()
                        .and_then(|r| apogee_of(r, id))
                        .map(|a| percent(a, compared.log_apogee_m))
                })
                .collect(),
            fitted: false,
            flagged: false,
        });
    }
    Ok(PublicFlights {
        flights,
        largest_uncorrected_m: largest,
    })
}

/// A number with its sign, or a dash.
fn signed(value: Option<f64>, unit: &str) -> String {
    value.map_or_else(|| "–".to_owned(), |v| format!("{v:+.2}{unit}"))
}

/// A number without a sign, or a dash.
fn unsigned(value: Option<f64>, unit: &str) -> String {
    value.map_or_else(|| "–".to_owned(), |v| format!("{v:.2}{unit}"))
}

/// Yes or no, or a dash.
fn yes_no(value: Option<bool>) -> &'static str {
    match value {
        Some(true) => "yes",
        Some(false) => "no",
        None => "–",
    }
}

/// An interval rounded outward, as the boost report gives them.
fn interval(value: Option<(Option<f64>, Option<f64>)>, few: &str) -> String {
    let end = |v: Option<f64>, outward: fn(f64) -> f64| {
        v.map_or_else(
            || "unbounded".to_owned(),
            |v| format!("{:.2}", outward(v * 100.0) / 100.0),
        )
    };
    match value {
        None => few.to_owned(),
        Some((None, None)) => "unbounded".to_owned(),
        Some((low, high)) => format!("{} to {}", end(low, f64::floor), end(high, f64::ceil)),
    }
}

impl Judged {
    /// The page's section, written from the published form.
    pub fn write_markdown(&self, page: &mut String, adr: &str) {
        let [unflagged, every] = [&self.fits[0], &self.fits[1]];
        let _ = writeln!(
            page,
            "\n**The correction of the shared bias, fitted and judged (M7.5b).** [ADR-346]({adr}) \
             fixed, before any of this was computed, how a correction of the bias the simulators \
             share is fitted on the calibration half and judged on the held-out half and the \
             public flights. It is two scales: one on every motor's impulse, one on the drag. \
             The drag scale comes from the calibration flights' coasts: for each flight without a \
             deployment before apogee, the drag at which its boost, refitted with the drag held \
             there, flies on to the logged apogee (a bisection from {DRAG_LOW} to {DRAG_HIGH}, to \
             {DRAG_WITHIN}); the scale is their mean, its 95% interval from Student's t. The \
             impulse scale comes from the same flights' boosts, every one this time, refitted at \
             that drag: their scales' mean weighted by one over each fit's variance plus a motor's \
             own spread ({tau:.4}, what the fit is told of one motor's impulse), its interval from \
             the scales' weighted spread. A scale enters only if at least {MIN_FITS} values give an \
             interval that excludes 1, rounded to two decimals. Two fits are made: over every \
             calibration flight predicted, and without the ones flagged above.",
            tau = self.tau,
        );
        let _ = writeln!(
            page,
            "\nEach fit's correction is flown on every flight the forward report compares, and \
             on the public flights as [their report](real-flights.md) flies them: every motor's \
             thrust and propellant scaled by the impulse scale at its rated burn time, the drag \
             by the drag scale, nothing else changed. Flown with both scales at 1, a collection \
             flight lands at most {} m from its forward apogee and a public flight at most {} m \
             (the rule allows 0.5 m). It passes only if, on the held-out flights and on those \
             without a deployment before apogee, each with and without their flagged flights, \
             its mean absolute error and the size of its mean error are both lower than the \
             forward simulation's, and on the public flights, with and without theirs, neither \
             is higher; both fits must pass. The fits rest on boosts below Mach 0.8, the cut's \
             limit, and are flown on faster flights too.",
            unsigned(self.largest_uncorrected_collection_m, ""),
            unsigned(self.largest_uncorrected_public_m, ""),
        );
        let _ = writeln!(
            page,
            "\n**The verdict: {}.** {}",
            if self.passes { "pass" } else { "fail" },
            if self.passes {
                "Both fits pass on all six sets.".to_owned()
            } else {
                format!(
                    "{} clause{} failed: {}.",
                    self.failed.len(),
                    if self.failed.len() == 1 { "" } else { "s" },
                    self.failed.join("; ")
                )
            }
        );
        let small = self.flagged_calibration < SMALL_SET;
        let _ = writeln!(
            page,
            "\nFlagged flights: {} of the calibration half's predicted flights, {} of the \
             held-out set's ({} of those without a deployment before apogee), from {} held-out \
             logs fitted alone for the flag and for nothing else, and {} of the public flights, \
             from the {} whose boost the [public boost report](from-boost.md) fits. {}",
            self.flagged_calibration,
            self.flagged_held_out,
            self.flagged_held_out_ballistic,
            self.held_out_log_fits,
            self.flagged_public,
            self.public_log_fits,
            if small {
                format!(
                    "Fewer than {SMALL_SET} calibration flights are flagged, so, as ADR-346 §7 \
                     says, the fit without them is given by its scales as flown, its counts and \
                     whether each interval excludes 1, and the fit over every calibration flight \
                     by its pass or fail on each set, with the public flights' numbers in full. \
                     A collection set less fewer than {SMALL_SET} flagged flights is given by its \
                     count and its pass or fail. What is held back is in a private file whose \
                     SHA-256 is committed in [`correction-held-back.sha256`](correction-held-back.sha256)."
                )
            } else {
                "Both fits' scales and intervals are given.".to_owned()
            }
        );
        let _ = writeln!(page, "\n| | {UNFLAGGED} | {EVERY} |\n|---|---:|---:|");
        let both = |page: &mut String, label: &str, value: &dyn Fn(&FitJudged) -> String| {
            let _ = writeln!(
                page,
                "| {label} | {} | {} |",
                value(unflagged),
                value(every)
            );
        };
        let held = |v: String| {
            if v == "–" {
                "held back".to_owned()
            } else {
                v
            }
        };
        both(page, "calibration flights predicted", &|f| {
            f.calibration_flights.to_string()
        });
        for (label, scale) in [
            (
                "drag scale",
                (|f: &FitJudged| &f.drag) as fn(&FitJudged) -> &ScaleFit,
            ),
            ("impulse scale", |f: &FitJudged| &f.impulse),
        ] {
            both(page, &format!("**{label}**"), &|_| String::new());
            both(
                page,
                if label == "drag scale" {
                    "… coasts solved, of those without a deployment before apogee"
                } else {
                    "… fits converged, of the flights predicted"
                },
                &|f| format!("{} of {}", scale(f).with_value, scale(f).flights),
            );
            both(page, "… the values' mean", &|f| {
                held(
                    scale(f)
                        .mean
                        .map_or_else(|| "–".to_owned(), |v| format!("{v:.4}")),
                )
            });
            both(page, "… 95% interval", &|f| {
                held(match (scale(f).low, scale(f).high) {
                    (Some(low), Some(high)) => format!("{low:.4} to {high:.4}"),
                    _ => "–".to_owned(),
                })
            });
            both(page, "… the interval excludes 1", &|f| {
                held(yes_no(scale(f).excludes_one).to_owned())
            });
            both(page, "… as flown", &|f| {
                held(
                    scale(f)
                        .flown
                        .map_or_else(|| "–".to_owned(), |v| format!("{v:.2}")),
                )
            });
            both(page, "… entered, or why not", &|f| {
                held(match (&scale(f).why_not, scale(f).flown) {
                    (Some(why), _) => why.clone(),
                    (None, Some(_)) => "it entered".to_owned(),
                    (None, None) => "–".to_owned(),
                })
            });
        }
        let _ = writeln!(
            page,
            "\nThe test. Each set's last rows give each flight's absolute error corrected less \
             forward, averaged, in percentage points (below zero, the correction came closer), \
             with its paired bootstrap interval ({} resamples, seed {}) and exact sign-flip \
             interval, and how many flights' errors grew: beside the rule, not in it.",
            crate::collection_errors::grouped(self.resamples),
            self.seed,
        );
        let _ = writeln!(page, "\n| | {UNFLAGGED} | {EVERY} |\n|---|---:|---:|");
        // Each set beside itself less its flagged flights: ADR-346 §5's order is all three, then
        // all three again less theirs.
        let sets = [0, 3, 1, 4, 2, 5]
            .into_iter()
            .filter_map(|k| Some((unflagged.sets.get(k)?, every.sets.get(k)?)))
            .chain([(&unflagged.calibration, &every.calibration)]);
        for (a, b) in sets {
            let title = if a.set == CALIBRATION_SET {
                format!("{} (fitted on these: not a test)", a.set)
            } else {
                a.named()
            };
            let _ = writeln!(
                page,
                "| **{title}** | **{}** | **{}** |",
                a.flights, b.flights
            );
            let row = |page: &mut String, label: &str, value: &dyn Fn(&SetJudged) -> String| {
                let _ = writeln!(
                    page,
                    "| {label} | {} | {} |",
                    held(value(a)),
                    held(value(b))
                );
            };
            row(page, "forward simulation: mean absolute error", &|s| {
                unsigned(s.forward_mean_absolute_percent, "%")
            });
            row(page, "corrected: mean absolute error", &|s| {
                unsigned(s.corrected_mean_absolute_percent, "%")
            });
            row(page, "forward simulation: mean error with its sign", &|s| {
                signed(s.forward_mean_percent, "%")
            });
            row(page, "corrected: mean error with its sign", &|s| {
                signed(s.corrected_mean_percent, "%")
            });
            row(page, "corrected flights that failed to fly", &|s| {
                s.failed_to_fly
                    .map_or_else(|| "–".to_owned(), |n| n.to_string())
            });
            if a.set != CALIBRATION_SET {
                let _ = writeln!(
                    page,
                    "| passes | {} | {} |",
                    yes_no(Some(a.passes)),
                    yes_no(Some(b.passes))
                );
            }
            row(
                page,
                "absolute errors' difference, corrected less forward (points)",
                &|s| signed(s.difference_points, ""),
            );
            row(page, "… paired bootstrap 95% interval", &|s| {
                if s.failed_to_fly.is_none() {
                    "–".to_owned()
                } else {
                    interval(
                        s.bootstrap_interval.map(|(l, h)| (Some(l), Some(h))),
                        "too few flights to resample",
                    )
                }
            });
            row(page, "… exact sign-flip 95% interval", &|s| {
                if s.failed_to_fly.is_none() {
                    "–".to_owned()
                } else {
                    interval(
                        s.sign_flip_interval,
                        if s.flights > 20 {
                            "not counted past 20 flights"
                        } else {
                            "none"
                        },
                    )
                }
            });
            row(page, "flights whose error grew", &|s| {
                s.grew.map_or_else(|| "–".to_owned(), |n| n.to_string())
            });
        }
        let _ = writeln!(
            page,
            "\n\"Held back\" marks a number ADR-346 §7 keeps off this page, where a set or a fit \
             differs from a published one by fewer than {SMALL_SET} flights. The calibration \
             flights are the ones the correction was fitted on, so their errors say how well it \
             fits, not how well it predicts. The impulse scale stands for the boost's whole \
             shortfall: a motor that delivered less, a rocket built heavier than its files, or \
             the model of the boost; the drag scale carries what the as-built mass does to the \
             coast too."
        );
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    reason = "a test stops at the failure (clippy.toml)"
)]
mod tests {
    use super::*;

    /// The 97.5% points of Student's t, as NIST/SEMATECH's e-Handbook of Statistical Methods
    /// prints them to three decimals (§1.3.6.7.2, the table of the t distribution's critical
    /// values).
    #[test]
    fn t_points_match_the_printed_table() {
        for (nu, printed) in [
            (1, 12.706),
            (2, 4.303),
            (3, 3.182),
            (4, 2.776),
            (5, 2.571),
            (6, 2.447),
            (7, 2.365),
            (8, 2.306),
            (9, 2.262),
            (10, 2.228),
            (11, 2.201),
            (12, 2.179),
        ] {
            let t = t_975(nu).unwrap();
            assert!((t - printed).abs() < 5e-4, "{nu}: {t} against {printed}");
        }
        assert!(t_975(0).is_none());
    }

    /// The closed forms agree with the two that have a simple one: one degree of freedom is the
    /// Cauchy distribution, two give t/√(2 + t²).
    #[test]
    fn the_closed_forms_hold_where_simple() {
        for t in [0.1_f64, 1.0, 3.0, 12.0] {
            let cauchy = 2.0 / std::f64::consts::PI * t.atan();
            assert!((within_t(t, 1) - cauchy).abs() < 1e-14);
            let two = t / (2.0 + t * t).sqrt();
            assert!((within_t(t, 2) - two).abs() < 1e-14);
        }
    }

    #[test]
    fn a_scale_rounds_to_two_decimals_a_tie_up() {
        assert_eq!(two_decimals(0.934_9), 0.93);
        assert_eq!(two_decimals(0.935_1), 0.94);
        assert_eq!(two_decimals(1.004_9), 1.0);
        assert_eq!(two_decimals(0.875), 0.88);
    }

    /// The bisection finds where a falling apogee crosses the logged one, to the stated width,
    /// and refuses ends on one side or a fit that came to nothing.
    #[test]
    fn the_drag_is_solved_where_the_apogee_is_the_logged_one() {
        // An apogee falling with the drag: 1000 m at a drag of 1.2.
        let apogee = |d: f64| Ok(1000.0 * (1.2 / d).sqrt());
        let d = solve_drag(apogee, 1000.0).unwrap();
        assert!((d - 1.2).abs() <= DRAG_WITHIN, "{d}");
        assert!(solve_drag(apogee, 10.0).unwrap_err().contains("one side"));
        let mut calls = 0;
        let failing = |_| {
            calls += 1;
            if calls > 3 {
                Err("the fit didn't converge".to_owned())
            } else {
                Ok(500.0)
            }
        };
        assert!(solve_drag(failing, 1000.0).is_err());
    }

    /// A held trial drag is the mean at a thousandth of the stated spread, the impulse widened;
    /// the impulse's fit at the drag scale keeps the drag's spread.
    #[test]
    fn the_trial_drag_is_held_and_the_impulse_free() {
        let stated = hpr_forensics::synthetic::fits::stated();
        let drag = Asked::DragScale.index();
        let impulse = Asked::ImpulseScale.index();
        let held = stated_at_drag(stated, 1.7);
        assert_eq!(held[drag].mean, 1.7);
        assert!((held[drag].sd * DRAG_HELD - stated[drag].sd).abs() < 1e-15);
        assert!((held[impulse].sd - 100.0 * stated[impulse].sd).abs() < 1e-12);
        let told = stated_told_drag(stated, 0.9);
        assert_eq!(told[drag].mean, 0.9);
        assert_eq!(told[drag].sd.to_bits(), stated[drag].sd.to_bits());
    }

    /// The drag enters only when every coast is solved, at least three, and the interval
    /// excludes 1.
    #[test]
    fn the_drag_enters_by_its_rule() {
        let solved = [Some(0.80), Some(0.82), Some(0.79), Some(0.81)];
        let fit = ScaleFit::drag(&solved);
        assert!(fit.entered());
        assert_eq!(fit.flown, Some(0.81));
        let censored = ScaleFit::drag(&[Some(0.80), Some(0.82), Some(0.79), None]);
        assert!(!censored.entered());
        assert_eq!(censored.flown, Some(1.0));
        assert!(censored.why_not.unwrap().contains("1 of its 4"));
        let two = ScaleFit::drag(&[Some(0.80), Some(0.82)]);
        assert!(!two.entered());
        let holds_one = ScaleFit::drag(&[Some(0.8), Some(1.2), Some(0.9), Some(1.1)]);
        assert_eq!(holds_one.excludes_one, Some(false));
        assert!(!holds_one.entered());
    }

    /// The impulse's weighted mean and interval, against a hand computation: two fits of
    /// standard error 0 and one of 0.0328·√3, τ = 0.0328, give weights 3, 3 and 1 over τ².
    #[test]
    fn the_impulse_is_the_weighted_mean_with_its_spread() {
        let tau = 0.0328;
        let s3 = tau * (3.0_f64).sqrt();
        let fits = [Some((0.90, 0.0)), Some((0.94, 0.0)), Some((0.86, s3)), None];
        let fit = ScaleFit::impulse(&fits, tau);
        assert_eq!(fit.flights, 4);
        assert_eq!(fit.with_value, 3);
        // Weights 1, 1 and 1/4 of 1/τ²: (0.90 + 0.94 + 0.86/4)/2.25.
        let center = (0.90 + 0.94 + 0.86 / 4.0) / 2.25;
        assert!((fit.mean.unwrap() - center).abs() < 1e-12);
        let scatter =
            (0.90 - center).powi(2) + (0.94 - center).powi(2) + (0.86 - center).powi(2) / 4.0;
        let half = t_975(2).unwrap() * (scatter / (2.0 * 2.25)).sqrt();
        assert!((fit.high.unwrap() - center - half).abs() < 1e-12);
        assert_eq!(fit.excludes_one, Some(center + half < 1.0));
    }

    /// The impulse enters on three converged fits whose interval excludes 1, rounded; not on
    /// two, nor where its interval holds 1.
    #[test]
    fn the_impulse_enters_by_its_rule() {
        let tau = 0.0328;
        let low = ScaleFit::impulse(
            &[Some((0.93, 0.03)), Some((0.94, 0.03)), Some((0.95, 0.03))],
            tau,
        );
        assert!(low.entered(), "{low:?}");
        assert_eq!(low.flown, Some(0.94));
        let two = ScaleFit::impulse(&[Some((0.93, 0.03)), Some((0.94, 0.03)), None], tau);
        assert!(!two.entered());
        assert_eq!(two.flown, Some(1.0));
        let wide = ScaleFit::impulse(
            &[Some((0.8, 0.03)), Some((1.2, 0.03)), Some((1.0, 0.03))],
            tau,
        );
        assert_eq!(wide.excludes_one, Some(false));
        assert!(!wide.entered());
    }

    fn pairs(values: &[(f64, f64)]) -> Vec<Pair> {
        values
            .iter()
            .map(|&(forward, corrected)| Pair {
                forward,
                corrected: Some(corrected),
            })
            .collect()
    }

    /// A held-out set must improve both numbers; a public one must not worsen either, and an
    /// equal number passes there. A flight that fails to fly fails its set.
    #[test]
    fn the_pass_rule_is_strict_on_the_held_out_sets_only() {
        let same = pairs(&[(10.0, 10.0), (-4.0, -4.0)]);
        assert!(!SetJudged::of(HELD_OUT_SET, false, &same, true).passes);
        assert!(SetJudged::of(PUBLIC, false, &same, false).passes);
        let better = pairs(&[(10.0, 6.0), (4.0, 1.0)]);
        assert!(SetJudged::of(HELD_OUT_SET, false, &better, true).passes);
        // Lower mean absolute error, larger bias: a fail.
        let biased = pairs(&[(10.0, 5.0), (-10.0, 6.0)]);
        let judged = SetJudged::of(HELD_OUT_SET, false, &biased, true);
        assert_eq!(judged.mean_absolute_holds, Some(true));
        assert_eq!(judged.bias_holds, Some(false));
        assert!(!judged.passes);
        let mut lost = better.clone();
        lost[1].corrected = None;
        let judged = SetJudged::of(HELD_OUT_SET, false, &lost, true);
        assert!(!judged.passes);
        assert_eq!(judged.failed_to_fly, Some(1));
    }

    fn fit(name: &str) -> FitJudged {
        let set = |set: &str, less: bool| {
            SetJudged::of(
                set,
                less,
                &pairs(&[(10.0, 6.0), (4.0, 1.0), (6.0, 2.0)]),
                true,
            )
        };
        let sets = vec![
            set(HELD_OUT_SET, false),
            set(HELD_OUT_BALLISTIC, false),
            set(PUBLIC, false),
            set(HELD_OUT_SET, true),
            set(HELD_OUT_BALLISTIC, true),
            set(PUBLIC, true),
        ];
        FitJudged {
            name: name.to_owned(),
            calibration_flights: 4,
            drag: ScaleFit::drag(&[Some(0.80), Some(0.82), Some(0.79), Some(0.81)]),
            impulse: ScaleFit::impulse(&[Some((0.9, 0.03)); 4], 0.0328),
            passes: sets.iter().all(|s| s.passes),
            sets,
            calibration: set(CALIBRATION_SET, false),
        }
    }

    /// With fewer than three calibration flights flagged, the page gives the unflagged fit as
    /// flown and the other by its verdicts, and a collection set less fewer than three flagged
    /// flights by its count and verdict; the public flights in full.
    #[test]
    fn the_page_holds_back_what_a_small_set_would_give_away() {
        let fits = vec![fit(UNFLAGGED), fit(EVERY)];
        let (passes, failed) = Judged::verdict(&fits);
        let judged = Judged {
            resamples: 10_000,
            seed: SEED,
            tau: 0.0328,
            flagged_calibration: 1,
            flagged_held_out: 0,
            flagged_held_out_ballistic: 0,
            flagged_public: 0,
            held_out_log_fits: 9,
            public_log_fits: 4,
            largest_uncorrected_collection_m: Some(0.0),
            largest_uncorrected_public_m: Some(0.0),
            fits,
            passes,
            failed,
            held_back: false,
        };
        assert!(judged.adds_up());
        let page = judged.published();
        let [unflagged, every] = [&page.fits[0], &page.fits[1]];
        assert_eq!(unflagged.drag.flown, Some(0.81));
        assert_eq!(unflagged.drag.mean, None);
        assert_eq!(unflagged.drag.low, None);
        assert_eq!(unflagged.drag.excludes_one, Some(true));
        assert_eq!(every.drag.flown, None);
        assert_eq!(every.impulse.mean, None);
        assert_eq!(every.sets[0].corrected_mean_absolute_percent, None);
        assert!(every.sets[0].passes);
        // The public set is given in full by either fit.
        assert!(every.sets[2].corrected_mean_absolute_percent.is_some());
        // A collection set less no flagged flight: its count and verdict only.
        assert_eq!(unflagged.sets[3].corrected_mean_absolute_percent, None);
        assert!(unflagged.sets[0].corrected_mean_absolute_percent.is_some());
        // Three flagged: nothing held back of the fits.
        let open = Judged {
            flagged_calibration: 3,
            ..judged.clone()
        }
        .published();
        assert!(open.fits[1].drag.mean.is_some());
        // The private file is sealed by its text.
        let text = held_back_text(&judged).unwrap();
        let hex: String = Sha256::digest(text.as_bytes())
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        assert!(held_back_sha_text(&text).starts_with(&hex));
    }

    /// A failed set less its flagged flights is named apart from the whole set.
    #[test]
    fn each_failed_clause_names_its_set() {
        let worse = pairs(&[(4.0, 9.0), (-2.0, 2.0)]);
        let whole = SetJudged::of(PUBLIC, false, &worse, false).failed_clauses(UNFLAGGED);
        let less = SetJudged::of(PUBLIC, true, &worse, false).failed_clauses(UNFLAGGED);
        assert_eq!(whole.len(), 2);
        assert!(less[0].contains("less their flagged flights"), "{less:?}");
        assert!(whole.iter().all(|clause| !less.contains(clause)));
    }

    /// Neither scale entering is a fail, named.
    #[test]
    fn no_correction_is_a_fail() {
        let mut none = fit(UNFLAGGED);
        none.drag = ScaleFit::drag(&[Some(0.8), None, Some(0.8)]);
        none.impulse = ScaleFit::impulse(&[Some((1.0, 0.03)); 4], 0.0328);
        let (passes, failed) = Judged::verdict(&[none, fit(EVERY)]);
        assert!(!passes);
        assert!(failed[0].contains("neither scale entered"), "{failed:?}");
    }
}
