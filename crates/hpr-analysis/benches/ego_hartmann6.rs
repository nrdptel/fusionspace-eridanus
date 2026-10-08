//! M6.2d2's measurement: EGO on Hartmann's six-variable function, with Jones, Schonlau and
//! Welch's (1998) `−ln(−y)` transform, from seeds 1 to 20 in 250 evaluations each. It fails
//! unless every run's best value is within 1% of the minimum's size, the rule `tests/ego.rs`
//! holds Branin's and Hartmann's three-variable functions to (ADR-152).
//!
//! ```text
//! cargo bench -p fusionspace-hpr-analysis --bench ego_hartmann6
//! ```
//!
//! Not a test: a run of 250 evaluations refits the surrogate 190 times, each fit a few hundred
//! factorizations of a matrix of up to 250 × 250, so the 20 runs take minutes in a release build
//! and hours in the debug build `cargo test` makes. Not a criterion benchmark either: it prints
//! each run's best value, the evaluation that first came within 1%, and the run's time, the runs
//! spread over the machine's cores. It does nothing unless `cargo bench` runs it, so
//! `cargo test --all-targets` doesn't run it in a debug build.

#![expect(
    clippy::unwrap_used,
    clippy::print_stdout,
    clippy::disallowed_types,
    clippy::disallowed_methods,
    reason = "a measurement with invalid fixed inputs should stop at once, it exists to print, and \
              it reads the clock and its arguments to time the library, outside it"
)]

use std::time::Instant;

use hpr_analysis::optimize::Variable;
use hpr_analysis::optimize::benchmark::global::{HARTMANN6_MINIMUM, hartmann6};
use hpr_analysis::optimize::ego::{Ego, Stop, Transform};

/// The evaluations allowed, the initial design's 60 included.
const BUDGET: usize = 250;

/// The seeds.
const SEEDS: u64 = 20;

/// One run: the best value, the evaluation that first came within 1% of the minimum (counted
/// from 1), and the seconds it took.
fn run(seed: u64) -> (f64, Option<usize>, f64) {
    let variables = (0..6)
        .map(|k| {
            Variable::new(format!("x{k}"), 0.5, 0.2)
                .unwrap()
                .within(0.0, 1.0)
                .unwrap()
        })
        .collect();
    let ego = Ego::new(variables)
        .unwrap()
        .with_max_evaluations(BUDGET)
        .unwrap()
        .with_transform(Transform::NegativeLog);
    let start = Instant::now();
    let mut evaluations = 0;
    let mut first = None;
    let optimum = ego
        .minimize(seed, |x| {
            evaluations += 1;
            let value = hartmann6(x);
            if first.is_none() && value <= 0.99 * HARTMANN6_MINIMUM {
                first = Some(evaluations);
            }
            value
        })
        .unwrap();
    assert_eq!(
        (optimum.evaluations, optimum.stop),
        (BUDGET, Stop::Evaluations)
    );
    (optimum.value, first, start.elapsed().as_secs_f64())
}

fn main() {
    // `cargo bench` passes `--bench`; `cargo test` doesn't.
    if !std::env::args().any(|argument| argument == "--bench") {
        return;
    }
    let results: Vec<(f64, Option<usize>, f64)> = std::thread::scope(|scope| {
        let runs: Vec<_> = (1..=SEEDS)
            .map(|seed| scope.spawn(move || run(seed)))
            .collect();
        runs.into_iter().map(|run| run.join().unwrap()).collect()
    });
    println!("Hartmann 6, -ln(-y) transform, {BUDGET} evaluations, seeds 1 to {SEEDS}:");
    let mut within = 0;
    let mut worst = 0.0_f64;
    for (seed, (value, first, seconds)) in (1..).zip(&results) {
        let gap = (value - HARTMANN6_MINIMUM) / HARTMANN6_MINIMUM.abs();
        let first = first.map_or_else(|| "never".to_owned(), |e| e.to_string());
        println!(
            "  seed {seed:>2}: best {value:.5}, {gap:.2e} of the minimum away, within 1% from \
             evaluation {first}, {seconds:.0} s"
        );
        if gap <= 0.01 {
            within += 1;
        }
        worst = worst.max(gap);
    }
    let latest = results.iter().filter_map(|r| r.1).max().unwrap_or(0);
    println!(
        "{within} of {SEEDS} within 1%, the worst {worst:.2e} of the minimum away; the last to get \
         within 1% did so at evaluation {latest}"
    );
    assert_eq!(within, SEEDS, "every run within 1% of the minimum");
}
