//! `cargo xtask fit --halves <file>`: rules for the least-squares shares tried on the calibration
//! set's kept fits alone ([ADR-384][adr-384] decision 4). The calibration set alternates set A's
//! model and set B's ([`Set::Calibration`]), so each rule's shares, fitted on the whole set, are
//! counted on each model's half as the report counts set A and set B: the drag, impulse and mass
//! on every fit with something to fit, the wind on the fits of flights with a track logger. A
//! half's counts, scaled to a test set's size, are that rule's prediction for the set; the test
//! sets' own fits are never read here.
//!
//! [adr-384]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0384-m7-3g-in-three-the-split-by-the-residual-measured.md

use std::collections::HashMap;
use std::fmt::Write as _;

use hpr_flightdata::readings::interval::{self, Interval, Shares};
use hpr_forensics::synthetic::fits::{self, ASKED, Asked, FlightFit};
use hpr_forensics::synthetic::intervals::{Counts, INSIDE_SHARE, Probe, Test, three_figures};
use hpr_forensics::synthetic::smoothed::MISS_SHARE;
use hpr_forensics::synthetic::{self, Model, Set};

/// A rule for the least-squares shares: beside the log's track, which class of fits a fit's
/// shares are fitted on.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rule {
    /// One class: the shares in force.
    InForce,
    /// The heights' RMS residual under [`fits::RESIDUAL_SPLIT_M`], or not: the split ADR-384
    /// measured.
    Residual,
    /// The heights' residual as a share of their spread ([`FlightFit::misfit_share`]) under the
    /// share given, or not.
    MisfitShare(f64),
}

impl Rule {
    /// The class of `fit`: 0, or 1 for the worse fit. A fit with no residual, or one that isn't a
    /// number, is in the worse class.
    pub fn class(self, fit: &FlightFit) -> usize {
        let within = match self {
            Self::InForce => true,
            Self::Residual => fit.rms_m.is_some_and(|r| r < fits::RESIDUAL_SPLIT_M),
            Self::MisfitShare(at) => fit.misfit_share.is_some_and(|s| s < at),
        };
        usize::from(!within)
    }

    /// How many classes it has.
    pub fn classes(self) -> usize {
        if self == Self::InForce { 1 } else { 2 }
    }

    /// Its name.
    pub fn name(self) -> String {
        match self {
            Self::InForce => "the shares in force".to_owned(),
            Self::Residual => "the split by the residual at 1 m".to_owned(),
            Self::MisfitShare(at) => format!("the split by the misfit's share at {at:.3e}"),
        }
    }
}

/// Whether flight `index` of the calibration set flies set B's model.
fn set_b_model(index: u64) -> bool {
    matches!(
        synthetic::draw(Set::Calibration, index).model,
        Model::Other { .. }
    )
}

/// Counts `truth` against `interval` as the report does: a truth that isn't a number, or an
/// interval that can't be stated, misses on both sides.
fn add(counts: &mut Counts, interval: Option<Interval>, truth: f64) {
    counts.given += 1;
    let Some(interval) = interval else {
        counts.below += 1;
        counts.above += 1;
        return;
    };
    let below = truth.is_nan() || truth < interval.low;
    let above = truth.is_nan() || truth > interval.high;
    counts.below += u64::from(below);
    counts.above += u64::from(above);
    counts.inside += u64::from(!below && !above);
}

/// One parameter's counts on one half.
#[derive(Debug, Clone, Default)]
pub struct Cell {
    /// Every fit counted.
    pub row: Counts,
    /// By the rule's class.
    pub classes: Vec<Counts>,
    /// Each probe of the row's intervals, in [`Probe::ALL`]'s order.
    pub probes: [Counts; 3],
}

/// One rule tried: its shares and each half's counts.
#[derive(Debug, Clone)]
pub struct Tried {
    /// The rule.
    pub rule: Rule,
    /// Each parameter's shares by track (height alone, then a track) and class; `None` where too
    /// few fits.
    pub shares: Vec<(Asked, [Vec<Option<Shares>>; 2])>,
    /// Each parameter's cells on set A's model's half, then set B's.
    pub cells: Vec<(Asked, [Cell; 2])>,
}

/// The fits a parameter is counted on: on a log of height alone too, or only with a track.
fn counted_on<'a>(
    asked: Asked,
    made: &'a [FlightFit],
    tracked: &'a [FlightFit],
) -> &'a [FlightFit] {
    if asked.stated_on_height_alone() {
        made
    } else {
        tracked
    }
}

/// `rule` tried on the calibration set's kept fits: `made`, every fit of the set, which the shares
/// on a log of height alone are fitted on; `tracked`, its fits of flights with a track logger,
/// which the shares on a log with a track are fitted on. The shares are taken at three figures,
/// as the library holds them.
pub fn tried(rule: Rule, made: &[FlightFit], tracked: &[FlightFit]) -> Tried {
    let mut shares = Vec::new();
    let mut cells = Vec::new();
    for (slot, asked) in ASKED.into_iter().enumerate() {
        let fitted = |drift: bool| -> Vec<Option<Shares>> {
            let on = if drift { tracked } else { made };
            (0..rule.classes())
                .map(|class| {
                    let points: Vec<f64> = on
                        .iter()
                        .filter(|f| f.fitted && f.drift == drift && rule.class(f) == class)
                        .map(|f| f.compared[slot].share())
                        .collect();
                    // At three figures, as the library holds the shares in force.
                    interval::fit(&points).map(|s| Shares {
                        low: three_figures(s.low),
                        high: three_figures(s.high),
                    })
                })
                .collect()
        };
        let by_track = [fitted(false), fitted(true)];
        let mut halves: [Cell; 2] = Default::default();
        for half in &mut halves {
            half.classes = vec![Counts::default(); rule.classes()];
        }
        for fit in counted_on(asked, made, tracked) {
            if !fit.fitted || (!fit.drift && !asked.stated_on_height_alone()) {
                continue;
            }
            let class = rule.class(fit);
            let compared = fit.compared[slot];
            let interval = by_track[usize::from(fit.drift)][class]
                .and_then(|s| s.interval(compared.value, compared.standard_error));
            let half = &mut halves[usize::from(set_b_model(fit.index))];
            add(&mut half.row, interval, compared.truth);
            add(&mut half.classes[class], interval, compared.truth);
            for (counts, probe) in half.probes.iter_mut().zip(Probe::ALL) {
                add(counts, interval.map(|i| probe.apply(i)), compared.truth);
            }
        }
        shares.push((asked, by_track));
        cells.push((asked, halves));
    }
    Tried {
        rule,
        shares,
        cells,
    }
}

/// The fits a test set counts a parameter on: [`super::COUNTED_FLIGHTS`] for the drag, impulse
/// and mass, the [`super::TRACK_FLIGHTS`] flights with a track logger for the wind.
fn set_size(asked: Asked) -> u64 {
    if asked.stated_on_height_alone() {
        super::COUNTED_FLIGHTS as u64
    } else {
        super::TRACK_FLIGHTS
    }
}

/// `counts` scaled by `to / from`, below and above each rounded to the nearest, inside the rest.
pub fn scaled(counts: Counts, from: u64, to: u64) -> Counts {
    let scale = |x: u64| ((x as f64) * (to as f64) / (from.max(1) as f64)).round() as u64;
    let given = scale(counts.given);
    let (below, above) = (scale(counts.below), scale(counts.above));
    Counts {
        given,
        below,
        inside: given.saturating_sub(below + above),
        above,
    }
}

/// What fails: rows and classes that fail [ADR-257][adr-257] §1's test, and probes that pass, on
/// each half.
///
/// [adr-257]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
pub fn failures(tried: &Tried) -> Vec<String> {
    let mut out = Vec::new();
    for (asked, halves) in &tried.cells {
        for (half, cell) in ["A", "B"].iter().zip(halves) {
            if !Test::of(cell.row).passes() {
                out.push(format!("{half}, {}: the row", asked.name()));
            }
            for (class, counts) in cell.classes.iter().enumerate() {
                if tried.rule.classes() > 1 && !Test::of(*counts).passes() {
                    out.push(format!("{half}, {}: class {class}", asked.name()));
                }
            }
            for (counts, probe) in cell.probes.iter().zip(Probe::ALL) {
                if Test::of(*counts).passes() {
                    out.push(format!(
                        "{half}, {}: probe {} passes",
                        asked.name(),
                        probe.name()
                    ));
                }
            }
        }
    }
    out
}

/// The text `--halves` prints for one rule, in full: shares, then each half's counts and what
/// they predict at a test set's size.
pub fn table(tried: &Tried) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "{}", tried.rule.name());
    let mark = |c: Counts| if Test::of(c).passes() { "" } else { " FAILS" };
    for ((asked, shares), (_, halves)) in tried.shares.iter().zip(&tried.cells) {
        let _ = writeln!(
            out,
            "  {}: shares, height alone {:?}; with a track {:?}",
            asked.name(),
            shares[0],
            shares[1]
        );
        for (half, cell) in ["A", "B"].iter().zip(halves) {
            let to = set_size(*asked);
            let row = scaled(cell.row, cell.row.given, to);
            let _ = write!(
                out,
                "    {half}'s half: {} / {} / {} of {}{}; as set {half}: {} / {} / {}{}",
                cell.row.below,
                cell.row.inside,
                cell.row.above,
                cell.row.given,
                mark(cell.row),
                row.below,
                row.inside,
                row.above,
                mark(row)
            );
            if tried.rule.classes() > 1 {
                for (class, counts) in cell.classes.iter().enumerate() {
                    let as_set = scaled(*counts, cell.row.given, to);
                    let _ = write!(
                        out,
                        "; class {class}: {} / {} / {} of {}{}, as set {half}: {} / {} / {}{}",
                        counts.below,
                        counts.inside,
                        counts.above,
                        counts.given,
                        mark(*counts),
                        as_set.below,
                        as_set.inside,
                        as_set.above,
                        mark(as_set)
                    );
                }
            }
            let passing: Vec<&str> = cell
                .probes
                .iter()
                .zip(Probe::ALL)
                .filter(|(c, _)| Test::of(**c).passes())
                .map(|(_, p)| p.name())
                .collect();
            if !passing.is_empty() {
                let _ = write!(out, "; probes passing: {}", passing.join(", "));
            }
            let _ = writeln!(out);
        }
    }
    out
}

/// The `q`-quantiles of the misfit's share over the fits with something to fit of `made` and
/// `tracked` together, each fit once, for `q` in `at`: where a split by it may fall.
pub fn misfit_quantiles(made: &[FlightFit], tracked: &[FlightFit], at: &[f64]) -> Vec<f64> {
    let mut seen = std::collections::BTreeSet::new();
    let mut shares: Vec<f64> = made
        .iter()
        .chain(tracked)
        .filter(|f| f.fitted && seen.insert(f.index))
        .filter_map(|f| f.misfit_share)
        .filter(|s| s.is_finite())
        .collect();
    shares.sort_by(f64::total_cmp);
    at.iter()
        .map(|q| {
            let rank = ((shares.len() as f64 - 1.0) * q).round() as usize;
            shares.get(rank).copied().unwrap_or(f64::NAN)
        })
        .collect()
}

/// A draw of a test set's counts for one class: `from` as the half counted them, at `to` fits.
/// Below and above are drawn jointly normal about the half's shares (each taken as `(x + ½)/(n +
/// 3/2)`, so a count of 0 still varies), their covariance the multinomial's times `1 + to/n`: the
/// set's own sampling and the half's error in its shares, together.
fn draw_counts(from: Counts, to: f64, rng: &mut hpr_core::random::SeededRng) -> Counts {
    let n = from.given as f64;
    let given = to.round().max(0.0) as u64;
    if from.given == 0 || given == 0 {
        return Counts {
            given,
            ..Counts::default()
        };
    }
    let share = |x: u64| (x as f64 + 0.5) / (n + 1.5);
    let (pb, pa) = (share(from.below), share(from.above));
    let inflate = to * (1.0 + to / n);
    let (vb, va, cba) = (
        pb * (1.0 - pb) * inflate,
        pa * (1.0 - pa) * inflate,
        -pb * pa * inflate,
    );
    let l11 = vb.sqrt();
    let l21 = cba / l11;
    let l22 = (va - l21 * l21).max(0.0).sqrt();
    let (z1, z2) = (rng.standard_normal(), rng.standard_normal());
    let count = |x: f64| x.round().clamp(0.0, given as f64) as u64;
    let below = count(to * pb + l11 * z1);
    let above = count(to * pa + l21 * z1 + l22 * z2).min(given - below);
    Counts {
        given,
        below,
        inside: given - below - above,
        above,
    }
}

/// Draws of a test set a chance is taken over.
pub const CHANCE_DRAWS: u32 = 20_000;

/// The chance that `tried`'s rule passes on a test set like each half, as [`draw_counts`] draws
/// it, at the set's size ([`set_size`]): each parameter's row and classes pass, and each probe
/// fails. The parameters are drawn as if apart, though one flight's errors in them are tied.
/// Returns the whole chance, then each half and parameter's.
pub fn pass_chance(tried: &Tried) -> (f64, Vec<(String, f64)>) {
    let mut rng = hpr_core::random::SeededRng::seed_from_u64(2_026_112_700);
    // The test's bands are Clopper–Pearson's, slow to work out: each count's verdict once.
    let mut verdicts: HashMap<(u64, u64), (bool, bool)> = HashMap::new();
    let mut passes = |c: Counts| -> bool {
        let mut holds = |x: u64| {
            *verdicts.entry((x, c.given)).or_insert_with(|| {
                let test = Test::of(Counts {
                    given: c.given,
                    below: x,
                    inside: x,
                    above: x,
                });
                (
                    test.below.holds(MISS_SHARE),
                    test.inside.holds(INSIDE_SHARE),
                )
            })
        };
        holds(c.below).0 && holds(c.above).0 && holds(c.inside).1
    };
    let mut whole = 1.0;
    let mut each = Vec::new();
    for (asked, halves) in &tried.cells {
        for (half, cell) in ["A", "B"].iter().zip(halves) {
            let to = set_size(*asked) as f64;
            let row_n = cell.row.given.max(1) as f64;
            let mut passed = 0_u32;
            for _ in 0..CHANCE_DRAWS {
                let mut row = Counts::default();
                let mut ok = true;
                for class in &cell.classes {
                    let drawn = draw_counts(*class, to * class.given as f64 / row_n, &mut rng);
                    row.given += drawn.given;
                    row.below += drawn.below;
                    row.inside += drawn.inside;
                    row.above += drawn.above;
                    ok &= tried.rule.classes() == 1 || passes(drawn);
                }
                ok &= passes(row);
                for probe in cell.probes {
                    ok &= !passes(draw_counts(probe, to, &mut rng));
                }
                passed += u32::from(ok);
            }
            let chance = f64::from(passed) / f64::from(CHANCE_DRAWS);
            whole *= chance;
            each.push((format!("{half}, {}", asked.name()), chance));
        }
    }
    (whole, each)
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    reason = "a test stops at the failure (clippy.toml)"
)]
mod tests {
    use super::*;
    use crate::fit::tests::a_fit;

    /// A rule's classes: the better fit is class 0, and a fit with no misfit or residual, or one
    /// that isn't a number, is in the worse.
    #[test]
    fn a_rules_classes_put_a_missing_misfit_in_the_worse() {
        let mut fit = a_fit(0);
        assert_eq!(Rule::InForce.class(&fit), 0);
        fit.misfit_share = Some(0.000_5);
        assert_eq!(Rule::MisfitShare(0.000_634).class(&fit), 0);
        fit.misfit_share = Some(0.000_634);
        assert_eq!(Rule::MisfitShare(0.000_634).class(&fit), 1);
        for missing in [None, Some(f64::NAN)] {
            fit.misfit_share = missing;
            fit.rms_m = missing;
            assert_eq!(Rule::MisfitShare(0.000_634).class(&fit), 1);
            assert_eq!(Rule::Residual.class(&fit), 1);
        }
        fit.rms_m = Some(0.999);
        assert_eq!(Rule::Residual.class(&fit), 0);
        fit.rms_m = Some(fits::RESIDUAL_SPLIT_M);
        assert_eq!(Rule::Residual.class(&fit), 1);
        assert_eq!(
            [Rule::InForce, Rule::Residual, Rule::MisfitShare(0.1)].map(Rule::classes),
            [1, 2, 2]
        );
    }

    /// A count is the report's: a truth on an end is inside, and one that isn't a number, or an
    /// interval that can't be stated, misses on both sides.
    #[test]
    fn a_truth_is_counted_as_the_report_counts_it() {
        let mut counts = Counts::default();
        let interval = Some(Interval {
            low: 1.0,
            high: 2.0,
        });
        for truth in [0.5, 1.0, 1.5, 2.0, 2.5] {
            add(&mut counts, interval, truth);
        }
        add(&mut counts, interval, f64::NAN);
        add(&mut counts, None, 1.5);
        assert_eq!(
            counts,
            Counts {
                given: 7,
                below: 3,
                inside: 3,
                above: 3,
            }
        );
    }

    /// Scaling keeps the three counts adding up to the set's size.
    #[test]
    fn scaled_counts_add_up() {
        let half = Counts {
            given: 838,
            below: 27,
            inside: 765,
            above: 46,
        };
        let set = scaled(half, 838, 1_000);
        assert_eq!((set.given, set.below, set.above), (1_000, 32, 55));
        assert_eq!(set.below + set.inside + set.above, set.given);
        assert_eq!(scaled(half, 838, 838), half);
    }

    /// Whether flight `index` of the calibration set flies set B's model: the odd ones.
    #[test]
    fn the_calibration_sets_halves_alternate() {
        assert!((0..40).all(|k| set_b_model(k) == (k % 2 == 1)));
    }

    /// Fits made up so that the shares and the halves are known: the share of flight `k` is drawn
    /// evenly, set A's model's halfway as wide as set B's, so one pair of shares covers set A's
    /// half too well and set B's too little, and its classes split by the misfit at `k`.
    fn made_up(count: u64) -> Vec<FlightFit> {
        (0..count)
            .map(|k| {
                let mut fit = a_fit(k);
                let spread = if k % 2 == 1 { 2.0 } else { 1.0 };
                // An even spread of shares over each half, in a scrambled order.
                let u = ((k / 2) * 7_919 % (count / 2)) as f64 / (count / 2) as f64;
                for c in &mut fit.compared {
                    c.standard_error = 1.0;
                    c.truth = 0.0;
                    c.value = spread * (2.0 * u - 1.0);
                }
                fit.misfit_share = Some(k as f64 / count as f64);
                fit
            })
            .collect()
    }

    /// The halves are counted apart, each against shares fitted on both: set B's model's half,
    /// twice as spread, misses more often, and set A's model's fewer than a 90% interval allows.
    #[test]
    fn each_half_is_counted_on_the_shares_of_both() {
        let made = made_up(2_000);
        let tracked: Vec<FlightFit> = Vec::new();
        let tried = tried(Rule::InForce, &made, &tracked);
        let (asked, [a, b]) = &tried.cells[0];
        assert_eq!(*asked, Asked::DragScale);
        assert_eq!((a.row.given, b.row.given), (1_000, 1_000));
        // The shares cover 10% of both halves together.
        let missed = a.row.below + a.row.above + b.row.below + b.row.above;
        assert!((190..=210).contains(&missed), "{missed}");
        // Set A's model's half sits inside them, set B's half misses for both.
        assert_eq!(a.row.below + a.row.above, 0);
        assert!(!Test::of(a.row).passes() && !Test::of(b.row).passes());
        assert!(failures(&tried).contains(&"A, drag scale: the row".to_owned()));
        // The wind is counted on fits with a track alone, and there are none.
        let (_, [wind, _]) = &tried.cells[2];
        assert_eq!(wind.row.given, 0);
    }

    /// A cell that holds its test with room passes nearly always; one far outside it, nearly
    /// never; one counted on a half far smaller than the set, much less surely, as the half's own
    /// error in its shares widens the draws; and the chance is the same on every run.
    #[test]
    fn the_chance_follows_the_room_a_cell_has() {
        let counts = |given: u64, below: u64, above: u64| Counts {
            given,
            below,
            inside: given - below - above,
            above,
        };
        let cell = |given: u64, below: u64, above: u64| Cell {
            row: counts(given, below, above),
            classes: vec![counts(given, below, above)],
            // Probes far off, which fail on every draw.
            probes: [counts(given, given * 3 / 10, given * 3 / 10); 3],
        };
        let of = |given: u64, below: u64, above: u64| Tried {
            rule: Rule::InForce,
            shares: Vec::new(),
            cells: vec![(
                Asked::DragScale,
                [cell(given, below, above), cell(given, below, above)],
            )],
        };
        let (good, _) = pass_chance(&of(2_000, 100, 100));
        let (bad, _) = pass_chance(&of(2_000, 200, 100));
        // 5% each side, but of 200 fits for a set of 1,000: the draws' spread is the set's times
        // 1 + 1,000/200 = 6. Without that factor the chance is above 0.9 here too.
        let (small, _) = pass_chance(&of(200, 10, 10));
        assert!(good > 0.9, "{good}");
        assert!(bad < 0.01, "{bad}");
        assert!(small < 0.75, "{small}");
        assert_eq!(pass_chance(&of(2_000, 100, 100)).0, good);
    }
}
