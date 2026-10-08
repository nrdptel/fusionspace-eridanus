//! EGO held to the known minima of the Branin and Hartmann 3 functions in a stated budget.
//!
//! The rule is Jones et al.'s (1998) measure of success (their Table 1): the best value within
//! 1% of the minimum's size. The budget is 50 evaluations, the initial design's 10 per variable
//! included, from each of seeds 1 to 20. The budget was set from a probe on seeds 1 to 10 at 40
//! evaluations, where Branin's worst was 1.03%; at 50 the worst gaps of the 20 seeds were
//! 1.18e-3 (Branin) and 1.24e-3 (Hartmann 3) of the minimum on a macOS debug build (shown with
//! `--nocapture`; other platforms' maths libraries may move the last digits).
//!
//! Hartmann's six-variable function is held to the same rule by
//! `benches/ego_hartmann6.rs`, with Jones et al.'s `−ln(−y)` transform, from seeds 1 to 20 in
//! 250 evaluations (M6.2d2, ADR-152): 20 runs take hours in a debug build, so `cargo bench` runs
//! them, in a release build. Here one run of 100 evaluations shows the transform at work, by an
//! outcome that doesn't move with a platform's maths library: the run finds the deepest well.

#![allow(
    clippy::unwrap_used,
    reason = "the helpers stop at the failure, as `#[test]` functions may (clippy.toml)"
)]
#![allow(
    clippy::print_stderr,
    reason = "the worst gaps the guide quotes, shown with `--nocapture`"
)]

use hpr_analysis::AnalysisError;
use hpr_analysis::optimize::Variable;
use hpr_analysis::optimize::benchmark::global::{
    BRANIN_MINIMA, BRANIN_MINIMUM, HARTMANN3_MINIMUM, HARTMANN6_MINIMUM, branin, hartmann3,
    hartmann6,
};
use hpr_analysis::optimize::cmaes::Cmaes;
use hpr_analysis::optimize::ego::{Ego, MAX_EGO_VARIABLES, MAX_POINTS, Stop, Transform};

/// Hartmann 6's second-lowest minimum, −3.20316 at (0.4047, 0.8824, 0.8461, 0.5740, 0.1389,
/// 0.0385), rounded down. A value below it can only be in the global minimum's well, as every
/// other well bottoms out higher. Checked by polishing in `minima_as_printed`; that no other
/// minimum lies between it and the global one rests on uncommitted surveys (ADR-152 item 5).
const HARTMANN6_SECOND_MINIMUM_BELOW: f64 = -3.2032;

/// The evaluations allowed.
const BUDGET: usize = 50;

/// The seeds.
const SEEDS: u64 = 20;

/// Branin's two variables.
fn branin_variables() -> Vec<Variable> {
    vec![
        Variable::new("x0", 2.5, 3.0)
            .unwrap()
            .within(-5.0, 10.0)
            .unwrap(),
        Variable::new("x1", 7.5, 3.0)
            .unwrap()
            .within(0.0, 15.0)
            .unwrap(),
    ]
}

/// `n` variables in `[0, 1]`.
fn unit(n: usize) -> Vec<Variable> {
    (0..n)
        .map(|k| {
            Variable::new(format!("x{k}"), 0.5, 0.2)
                .unwrap()
                .within(0.0, 1.0)
                .unwrap()
        })
        .collect()
}

/// Runs `ego` from every seed and holds each best value within 1% of `minimum`'s size.
fn within_one_percent(name: &str, ego: &Ego, model: fn(&[f64]) -> f64, minimum: f64) {
    let mut worst = 0.0_f64;
    for seed in 1..=SEEDS {
        let optimum = ego.minimize(seed, model).unwrap();
        assert_eq!(optimum.evaluations, BUDGET);
        assert_eq!(optimum.stop, Stop::Evaluations);
        assert_eq!(optimum.value, model(&optimum.point));
        let gap = (optimum.value - minimum) / minimum.abs();
        assert!(
            gap > -1e-6,
            "{name} seed {seed}: below the minimum by {gap}"
        );
        assert!(
            gap <= 0.01,
            "{name} seed {seed}: {gap:.2e} of the minimum away"
        );
        worst = worst.max(gap);
    }
    eprintln!("{name}: worst gap {worst:.2e} of the minimum in {BUDGET} evaluations");
}

#[test]
fn branin_within_one_percent() {
    let ego = Ego::new(branin_variables())
        .unwrap()
        .with_max_evaluations(BUDGET)
        .unwrap();
    within_one_percent("Branin", &ego, branin, BRANIN_MINIMUM);
}

#[test]
fn hartmann3_within_one_percent() {
    let ego = Ego::new(unit(3))
        .unwrap()
        .with_max_evaluations(BUDGET)
        .unwrap();
    within_one_percent("Hartmann 3", &ego, hartmann3, HARTMANN3_MINIMUM);
}

/// Hartmann 6 from seed 10 in 100 evaluations, the initial design's 60 included: with the
/// `−ln(−y)` transform the run finds the global minimum's well, and without it the same run ends
/// at the second-lowest minimum near −3.20, 3.6% away. The best value is returned.
///
/// How close the run gets inside the well moves with the platform's maths: 0.45% above the
/// minimum on macOS, 1.01% on Linux and 1.47% on Windows (CI), most likely as each
/// seeded run amplifies last-bit differences in `exp` and `ln`. So the 1% rule is held by the
/// 20-run program (ADR-152), and this test holds only which well the run ends in: it accepts a
/// run up to 3.59% above the minimum.
fn hartmann6_seed_10(transform: Transform) -> f64 {
    let optimum = Ego::new(unit(6))
        .unwrap()
        .with_max_evaluations(100)
        .unwrap()
        .with_transform(transform)
        .minimize(10, hartmann6)
        .unwrap();
    assert_eq!(
        (optimum.evaluations, optimum.stop),
        (100, Stop::Evaluations)
    );
    assert_eq!(optimum.value, hartmann6(&optimum.point));
    optimum.value
}

#[test]
fn hartmann6_with_the_transform() {
    let best = hartmann6_seed_10(Transform::NegativeLog);
    eprintln!("Hartmann 6, seed 10, with the transform: best {best:.5}");
    assert!(
        best > HARTMANN6_MINIMUM - 1e-5 && best < HARTMANN6_SECOND_MINIMUM_BELOW,
        "{best}: not in the global minimum's well"
    );
}

#[test]
fn hartmann6_without_the_transform() {
    let best = hartmann6_seed_10(Transform::None);
    eprintln!("Hartmann 6, seed 10, without the transform: best {best:.5}");
    let gap = (best - HARTMANN6_MINIMUM) / HARTMANN6_MINIMUM.abs();
    assert!(gap > 0.03, "{gap:.2e} of the minimum away");
}

/// The transforms on their domains, and a value outside one refused with the value it got,
/// before any fit: a positive value under `−ln(−y)`, zero and a negative one under `ln y`.
#[test]
fn transforms_and_their_domains() {
    assert_eq!(Transform::NegativeLog.apply(-1.0), Some(0.0));
    assert_eq!(
        Transform::NegativeLog.apply(-std::f64::consts::E),
        Some(-1.0)
    );
    assert_eq!(Transform::NegativeLog.apply(-0.0), None);
    assert_eq!(Transform::NegativeLog.apply(0.5), None);
    assert_eq!(Transform::Log.apply(std::f64::consts::E), Some(1.0));
    assert_eq!(Transform::Log.apply(0.0), None);
    assert_eq!(Transform::Log.apply(-2.0), None);
    assert_eq!(Transform::None.apply(-2.0), Some(-2.0));
    for transform in [Transform::None, Transform::Log, Transform::NegativeLog] {
        assert_eq!(transform.apply(f64::INFINITY), Some(f64::INFINITY));
    }
    // Both increase with the value, so the least is in the same place.
    let a = Transform::NegativeLog.apply(-3.3).unwrap();
    let b = Transform::NegativeLog.apply(-3.2).unwrap();
    assert!(a < b);
    let ego = Ego::new(branin_variables())
        .unwrap()
        .with_max_evaluations(30)
        .unwrap();
    let mut calls = 0;
    let error = ego
        .clone()
        .with_transform(Transform::NegativeLog)
        .minimize(1, |x| {
            calls += 1;
            if calls == 25 { 0.25 } else { -branin(x) - 1.0 }
        })
        .unwrap_err();
    assert!(
        matches!(error, AnalysisError::Domain { what, value }
            if what.contains("-ln(-y)") && value == 0.25),
        "{error}"
    );
    let error = ego
        .clone()
        .with_transform(Transform::Log)
        .minimize(1, |_| 0.0)
        .unwrap_err();
    assert!(
        matches!(error, AnalysisError::Domain { what, value }
            if what.contains("ln(y)") && value == 0.0),
        "{error}"
    );
    // A failed evaluation is still fitted as the worst value under a transform.
    let mut calls = 0;
    let optimum = ego
        .with_transform(Transform::Log)
        .minimize(2, |x| {
            calls += 1;
            if calls % 4 == 0 {
                f64::INFINITY
            } else {
                branin(x)
            }
        })
        .unwrap();
    assert_eq!((optimum.evaluations, optimum.stop), (30, Stop::Evaluations));
    assert!(optimum.value.is_finite());
}

/// The minima the test holds EGO to: Branin's three points give its closed form, and the
/// Hartmann functions' printed values are their least values to the printing, found by CMA-ES
/// polishing from the printed points.
#[test]
fn minima_as_printed() {
    for x in BRANIN_MINIMA {
        assert!((branin(&x) - BRANIN_MINIMUM).abs() < 1e-14, "{x:?}");
    }
    let polish = |start: &[f64], model: fn(&[f64]) -> f64| {
        let variables = start
            .iter()
            .enumerate()
            .map(|(k, s)| {
                Variable::new(format!("x{k}"), *s, 1e-3)
                    .unwrap()
                    .within(0.0, 1.0)
                    .unwrap()
            })
            .collect();
        Cmaes::new(variables)
            .unwrap()
            .minimize(1, model)
            .unwrap()
            .value
    };
    let h3 = polish(&[0.114614, 0.555649, 0.852547], hartmann3);
    assert!((h3 - HARTMANN3_MINIMUM).abs() < 5e-6, "{h3}");
    let h6 = polish(
        &[0.20169, 0.150011, 0.476874, 0.275332, 0.311652, 0.6573],
        hartmann6,
    );
    assert!((h6 - HARTMANN6_MINIMUM).abs() < 5e-6, "{h6}");
    let second = polish(&[0.4047, 0.8824, 0.8461, 0.574, 0.1389, 0.0385], hartmann6);
    assert!((second - -3.20316).abs() < 5e-6, "{second}");
    assert!(HARTMANN6_SECOND_MINIMUM_BELOW < second);
}

/// One seed gives the same run twice; a target stops it early.
#[test]
fn reproducible_and_stops_at_target() {
    let ego = Ego::new(branin_variables())
        .unwrap()
        .with_max_evaluations(30)
        .unwrap();
    let first = ego.minimize(3, branin).unwrap();
    assert_eq!(first, ego.minimize(3, branin).unwrap());
    let target = ego
        .clone()
        .with_target(1.0)
        .unwrap()
        .minimize(3, branin)
        .unwrap();
    assert_eq!(target.stop, Stop::Target);
    assert!(target.value <= 1.0);
    assert_eq!(target.evaluation, target.evaluations);
    let short = ego
        .with_max_evaluations(5)
        .unwrap()
        .minimize(3, branin)
        .unwrap();
    assert_eq!((short.evaluations, short.stop), (5, Stop::Evaluations));
}

/// A failed evaluation (`+∞`) is fitted as the worst value and the run goes on; NaN is an
/// error naming its evaluation; a flat model runs to the budget, exploring.
#[test]
fn failures_and_flat_models() {
    let ego = Ego::new(branin_variables())
        .unwrap()
        .with_max_evaluations(30)
        .unwrap();
    let mut calls = 0;
    let optimum = ego
        .minimize(2, |x| {
            calls += 1;
            if calls % 4 == 0 {
                f64::INFINITY
            } else {
                branin(x)
            }
        })
        .unwrap();
    assert_eq!((optimum.evaluations, optimum.stop), (30, Stop::Evaluations));
    assert!(optimum.value.is_finite());
    let mut calls = 0;
    let error = ego
        .minimize(2, |x| {
            calls += 1;
            if calls == 25 { f64::NAN } else { branin(x) }
        })
        .unwrap_err();
    assert!(
        matches!(error, AnalysisError::Output { index: 24, .. }),
        "{error}"
    );
    let flat = ego.minimize(2, |_| 7.3).unwrap();
    assert_eq!((flat.evaluations, flat.stop), (30, Stop::Evaluations));
    let failed = ego.minimize(2, |_| f64::INFINITY).unwrap();
    assert_eq!(
        (failed.value, failed.stop),
        (f64::INFINITY, Stop::Evaluations)
    );
}

/// The improvement tolerance stops a run early; Jones et al.'s 1% of the best value's size.
#[test]
fn stops_on_small_improvement() {
    let ego = Ego::new(branin_variables())
        .unwrap()
        .with_max_evaluations(50)
        .unwrap()
        .with_tolerance_improvement(0.01 * BRANIN_MINIMUM)
        .unwrap();
    let optimum = ego.minimize(1, branin).unwrap();
    assert_eq!(optimum.stop, Stop::Improvement);
    assert!(optimum.evaluations < 50);
}

/// Settings are checked, and read back through the same checks.
#[test]
fn settings_checked_and_read_back() {
    let ego = || Ego::new(branin_variables()).unwrap();
    let what = |e: AnalysisError| match e {
        AnalysisError::TooFew { what, .. }
        | AnalysisError::Count { what, .. }
        | AnalysisError::Domain { what, .. } => what,
        other => panic!("{other}"),
    };
    assert_eq!(
        what(ego().with_initial(1).unwrap_err()),
        "initial design's points"
    );
    assert_eq!(
        what(ego().with_initial(MAX_POINTS + 1).unwrap_err()),
        "initial design's points"
    );
    assert_eq!(
        what(ego().with_max_evaluations(0).unwrap_err()),
        "evaluations"
    );
    assert_eq!(
        what(ego().with_max_evaluations(MAX_POINTS + 1).unwrap_err()),
        "evaluations"
    );
    assert_eq!(
        what(ego().with_target(f64::NAN).unwrap_err()),
        "target (finite)"
    );
    assert_eq!(
        what(ego().with_tolerance_improvement(-1.0).unwrap_err()),
        "improvement tolerance"
    );
    assert_eq!(
        what(Ego::new(unit(MAX_EGO_VARIABLES + 1)).unwrap_err()),
        "EGO variables"
    );
    assert_eq!(what(Ego::new(Vec::new()).unwrap_err()), "variables");
    let set = ego()
        .with_initial(12)
        .unwrap()
        .with_target(0.5)
        .unwrap()
        .with_tolerance_improvement(1e-3)
        .unwrap();
    let json = serde_json::to_string(&set).unwrap();
    assert_eq!(serde_json::from_str::<Ego>(&json).unwrap(), set);
    let bad = json.replace("\"initial\":12", "\"initial\":1");
    assert!(serde_json::from_str::<Ego>(&bad).is_err());
    // The transform reads back, and a document without one (written before it existed) reads
    // as none.
    let transformed = set.clone().with_transform(Transform::NegativeLog);
    let json = serde_json::to_string(&transformed).unwrap();
    assert!(json.contains("\"transform\":\"NegativeLog\""), "{json}");
    assert_eq!(serde_json::from_str::<Ego>(&json).unwrap(), transformed);
    let without = serde_json::to_string(&set)
        .unwrap()
        .replace(",\"transform\":\"None\"", "");
    assert!(!without.contains("transform"), "{without}");
    assert_eq!(serde_json::from_str::<Ego>(&without).unwrap(), set);
    let optimum = set.minimize(4, branin).unwrap();
    let json = serde_json::to_string(&optimum).unwrap();
    assert_eq!(
        serde_json::from_str::<hpr_analysis::optimize::ego::Optimum>(&json).unwrap(),
        optimum
    );
}
