//! A validation uncertainty for each real flight, after ASME V&V 20-2009 ([M1.14b16][m1-14b16],
//! the combination and the numerical part; [the decision that planned it][adr-351], decision 2,
//! and [the one that splits it in three][adr-373]).
//!
//! V&V 20 writes a comparison error `E = S − D`, the simulation's value less the data's, as the
//! model's own error and three others: `E = δ_model + δ_num + δ_input − δ_D`. Each of the three
//! others has a standard uncertainty, and while they are independent the validation uncertainty
//! is their root sum of squares, `u_val = √(u_num² + u_input² + u_D²)`; the model's error then
//! lies in `E ± k·u_val`, with `k` typically 2 to 3. Read in Eça, Dowding and Roache, "On the
//! interpretation and scope of the V&V 20 standard", ASME V&V Symposium 2020, SAND2020-3788C
//! (OSTI 1774746), p. 4, eq. 10; the standard's own text is paywalled and was not read
//! (`docs/research/validation-uncertainty-vv20.md`).
//!
//! The numerical part follows Roache's grid convergence index on two runs, the second with the
//! adaptive stepper's tolerances divided by 32 so that its steps are about half as long. `S` in
//! `E` is the first, coarser run, the report's own flight, so the index is the coarse run's:
//! Richardson's estimate puts the finer run's error at `|ε|/(r^p − 1)` (the note's sources S4
//! p. 3 and S6 p. 7), and so the coarser run's at `|ε|·r^p/(r^p − 1)`, which the safety factor
//! `F_s = 3` for two runs scales to `GCI = 3·|ε|·r^p/(r^p − 1)`. The order is `p = 4`, assumed:
//! the formal order of Dormand–Prince's dense output, which places the apogee (measured 4.8 to 5
//! on a model problem, `docs/physics/integration.md`). The flights' own order isn't measured,
//! and a lower one would make the index larger. The refinement ratio
//! `r` is measured, as the second run's accepted steps to apogee over the first's, not assumed
//! to be 2. The index is a band of about 95%; it is divided by `k = 2` for a standard
//! uncertainty, the divisor NIST's summary gives for a Gaussian (NISTIR 8298, 2020, §4, p. 17; it
//! gives 1.1 to 1.15 for a shifted one, and neither was checked against the standard).
//!
//! The data part is the log's own ([M1.14b17][m1-14b17], [`crate::data_uncertainty`]): each
//! row carries its log apogee's standard uncertainty where its altimeter, or the pressure
//! sensor its maker names, has a published accuracy, and the correction the log's apogee takes
//! for what is known to make it read low or high ([`UncertaintyRow::corrected_error_m`]). Both
//! apogees are barometric readings, so the standard atmosphere's distance from
//! the day's air is in both and not in the data part; each row measures that conversion on
//! both sides ([`UncertaintyRow::conversion_m`], [`UncertaintyRow::log_conversion_m`]).
//!
//! The input part is the apogee's spread over the flight's stated input uncertainties
//! ([M1.14b18][m1-14b18], [`crate::input_uncertainty`]): each row carries its seven drawn
//! numbers flown alone and the standard deviation of 200 flights with all of them drawn. With
//! all three parts a row has its validation uncertainty and the band's call
//! ([`UncertaintyRow::within_band`]), about the error against the corrected log; a flight with
//! no data part has neither.
//!
//! [m1-14b16]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m1-14b16
//! [m1-14b17]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m1-14b17
//! [m1-14b18]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m1-14b18
//! [adr-351]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0351-m1-14b-in-eight-the-basic-finners-normal-force-first.md
//! [adr-373]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0373-m1-14b7-in-three-the-numerical-part-first.md

use std::path::Path;

use std::sync::atomic::{AtomicUsize, Ordering};

use serde::{Deserialize, Serialize};

use crate::correction::Variant;
use crate::data_uncertainty::{DataPart, Measured, Term, data_part, instrument};
use crate::input_uncertainty::{Drawn, InputDraw, InputPart, SEED, draws, input_part, stated};
use crate::predicted_first::ComparedPrediction;
use crate::real_flight::{FLIGHTS, FlightRow, RealFlightError, Varied, fly_varied};

/// Roache's safety factor for a grid convergence index from two runs.
pub const SAFETY_FACTOR_TWO_RUNS: f64 = 3.0;

/// The order taken for the apogee's convergence: 4, assumed, the formal order of
/// Dormand–Prince 5(4)'s dense output, which places it (measured 4.8 to 5 on a model problem,
/// `docs/physics/integration.md`). The flights' own order isn't measured, and a lower one would
/// make the index larger. RK4's event state is third order there, so a flight on RK4 would need
/// 3; the report flies Dormand–Prince. `r^p` is [`fourth_power`], products alone, so that every
/// platform rounds it the same.
pub const APOGEE_ORDER: u32 = 4;

/// The coverage factor between a standard uncertainty and a band of about 95%: `k = 2`, the
/// divisor NIST's summary gives for a Gaussian (NISTIR 8298, p. 17). V&V 20 takes `k` typically
/// 2 to 3 (Eça, Dowding and Roache 2020, p. 4), and the paper's own reading draws `E ± 2·u_val`
/// (p. 7).
pub const COVERAGE_K: f64 = 2.0;

/// `x⁴` as `(x·x)·(x·x)`: two products, which IEEE 754 rounds the same on every platform, where a
/// library's `powf` may not.
#[must_use]
pub fn fourth_power(x: f64) -> f64 {
    let square = x * x;
    square * square
}

/// The numerical part `u_num`, m, of the coarser run: the standard uncertainty of an apogee
/// flown at `coarse_m`, from the same flight flown again on steps `refinement_ratio` times
/// shorter at `fine_m`, as Roache's two-run grid convergence index for the coarser run over
/// [`COVERAGE_K`]: `3·|fine − coarse|·r⁴/(r⁴ − 1)/2`. `None` unless the ratio is a finite number
/// above 1: the second run must be the finer.
#[must_use]
pub fn numerical_uncertainty_m(coarse_m: f64, fine_m: f64, refinement_ratio: f64) -> Option<f64> {
    if !(refinement_ratio.is_finite() && refinement_ratio > 1.0) {
        return None;
    }
    let r_p = fourth_power(refinement_ratio);
    let index = SAFETY_FACTOR_TWO_RUNS * (fine_m - coarse_m).abs() * r_p / (r_p - 1.0);
    Some(index / COVERAGE_K)
}

/// The refinement ratio of two runs of one flight: the finer run's accepted steps to apogee over
/// the coarser's, the ratio of their mean steps over the climb. `None` without steps in the
/// coarser run.
#[must_use]
pub fn refinement_ratio(coarse_steps: u64, fine_steps: u64) -> Option<f64> {
    // Step counts are far under 2^53 (a flight's step limit is in the millions), so each is an
    // exact f64.
    #[expect(clippy::cast_precision_loss, reason = "step counts are far under 2^53")]
    let ratio = (coarse_steps > 0).then(|| fine_steps as f64 / coarse_steps as f64);
    ratio
}

/// V&V 20's validation uncertainty, m: the root sum of squares of the numerical, input and data
/// parts, each a standard uncertainty (Eça, Dowding and Roache 2020, eq. 10). It holds while the
/// three errors are independent; where one source feeds both the simulation and the data, the
/// standard combines them otherwise.
#[must_use]
pub fn validation_uncertainty_m(numerical_m: f64, input_m: f64, data_m: f64) -> f64 {
    (numerical_m * numerical_m + input_m * input_m + data_m * data_m).sqrt()
}

/// One public real flight's comparison error and the numerical and data parts of its validation
/// uncertainty.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UncertaintyRow {
    /// The flight's id, as its row in the report gives it.
    pub id: String,
    /// The comparison error `E = S − D`, m: HPR Sim's apogee as its row gives it less the log's.
    pub comparison_error_m: f64,
    /// The apogee as the report flies it, m, as the log's altimeter would read it, unrounded.
    pub apogee_m: f64,
    /// The same flight with the time step halved, m ([`Variant::STEP_HALVED`]).
    pub apogee_step_halved_m: f64,
    /// The integrator's accepted steps that start before the apogee, as the report flies it.
    pub steps: u64,
    /// The same with the time step halved.
    pub steps_step_halved: u64,
    /// The numerical part `u_num`, m ([`numerical_uncertainty_m`] on the
    /// [`refinement_ratio`] of the two runs' steps).
    pub numerical_m: f64,
    /// The log's apogee `D`, m, as its row gives it.
    pub log_apogee_m: f64,
    /// HPR Sim's apogee as a height, m above the start: [`UncertaintyRow::apogee_m`] before the
    /// altimeter reads it.
    pub height_apogee_m: f64,
    /// The height above the start, m, at which the same altimeter, in the air the flight is
    /// flown in, reads the log's apogee: the log's apogee as a height, if that air is the day's.
    pub log_height_m: f64,
    /// The data part: the log apogee's correction, its standard uncertainty `u_D` and their
    /// components, or why its altimeter has none ([`crate::data_uncertainty`]).
    pub data: DataPart,
    /// The input part: the apogee's spread `u_input` over the flight's stated input
    /// uncertainties, and each input moved alone ([`crate::input_uncertainty`]).
    pub input: InputPart,
    /// The validation uncertainty `u_val`, m ([`validation_uncertainty_m`] of the three parts).
    /// `None` for a flight with no data part: without the log's part there is no `u_val`.
    pub validation_m: Option<f64>,
    /// Whether the error against the corrected log lies within the band, [`COVERAGE_K`] times
    /// `u_val` either side of zero ([`UncertaintyRow::within_band`]). `None` with no `u_val`.
    pub within: Option<bool>,
}

impl UncertaintyRow {
    /// The refinement ratio of its two runs ([`refinement_ratio`]).
    #[must_use]
    pub fn refinement_ratio(&self) -> Option<f64> {
        refinement_ratio(self.steps, self.steps_step_halved)
    }

    /// The barometer's conversion in HPR Sim's apogee, m: the reading less the height. It is
    /// how far the standard atmosphere's altitude of the day's pressure stands from the height
    /// climbed, at HPR Sim's apogee.
    #[must_use]
    pub fn conversion_m(&self) -> f64 {
        self.apogee_m - self.height_apogee_m
    }

    /// The same conversion in the log's apogee, m, in the air the flight is flown in: the log's
    /// reading less the height at which that reading is made ([`UncertaintyRow::log_height_m`]).
    #[must_use]
    pub fn log_conversion_m(&self) -> f64 {
        self.log_apogee_m - self.log_height_m
    }

    /// The comparison error as heights, m: HPR Sim's apogee less the log's, neither read through
    /// the barometer. It differs from [`UncertaintyRow::comparison_error_m`] by the part of the
    /// conversion that the two readings don't share.
    #[must_use]
    pub fn height_error_m(&self) -> f64 {
        self.height_apogee_m - self.log_height_m
    }

    /// The comparison error as a share of the log's apogee, both as readings: how the report
    /// scores a flight.
    #[must_use]
    pub fn error_share(&self) -> f64 {
        self.comparison_error_m / self.log_apogee_m
    }

    /// [`UncertaintyRow::height_error_m`] as a share of the log's apogee as a height.
    #[must_use]
    pub fn height_error_share(&self) -> f64 {
        self.height_error_m() / self.log_height_m
    }

    /// The comparison error against the log's apogee corrected, m: HPR Sim's apogee less the
    /// log's with its data part's correction added
    /// ([`DataPart::correction_m`](crate::data_uncertainty::DataPart::correction_m)), so
    /// [`UncertaintyRow::comparison_error_m`] less the correction. `None` for a flight with no
    /// data part. The report's own error stays on the log as logged; a band is drawn about this
    /// one.
    #[must_use]
    pub fn corrected_error_m(&self) -> Option<f64> {
        self.data
            .correction_m
            .map(|correction_m| self.comparison_error_m - correction_m)
    }

    /// The validation uncertainty its three parts give, m: their root sum of squares
    /// ([`validation_uncertainty_m`]). `None` for a flight with no data part.
    #[must_use]
    pub fn validation_of_parts_m(&self) -> Option<f64> {
        self.data
            .data_m
            .map(|data_m| validation_uncertainty_m(self.numerical_m, self.input.input_m, data_m))
    }

    /// Whether the error against the corrected log lies within [`COVERAGE_K`] times the
    /// validation uncertainty of zero, `|E corrected| ≤ 2·u_val`. Inside, the three parts could
    /// account for the whole error, and the model's own error is known only to be smaller than
    /// `|E| + 2·u_val`, its sign not at all; outside, the model's error has the sign of `E`
    /// (Eça, Dowding and Roache 2020, p. 5). `None` for a flight with no data part.
    #[must_use]
    pub fn within_band(&self) -> Option<bool> {
        let corrected_m = self.corrected_error_m()?;
        let validation_m = self.validation_of_parts_m()?;
        Some(corrected_m.abs() <= COVERAGE_K * validation_m)
    }

    /// Whether the numerical part is its two runs' ([`numerical_uncertainty_m`] on their
    /// [`refinement_ratio`]).
    #[must_use]
    pub fn numerical_holds(&self) -> bool {
        self.refinement_ratio().and_then(|ratio| {
            numerical_uncertainty_m(self.apogee_m, self.apogee_step_halved_m, ratio)
        }) == Some(self.numerical_m)
    }

    /// Whether the row holds together: its numerical part is its two runs', its data part is its
    /// altimeter's on its log's apogee, from the part's own conversions and formula gap
    /// ([`DataPart::holds`]; the report's check holds those to the flight's site and row), and
    /// everything is a finite number.
    #[must_use]
    pub fn holds(&self) -> bool {
        [
            self.comparison_error_m,
            self.apogee_m,
            self.apogee_step_halved_m,
            self.numerical_m,
            self.log_apogee_m,
            self.height_apogee_m,
            self.log_height_m,
        ]
        .iter()
        .all(|x| x.is_finite())
            && self.numerical_holds()
            && self.data.holds(&self.id, self.log_apogee_m)
            && self.input.holds(&self.id)
            && self.validation_m == self.validation_of_parts_m()
            && self.validation_m.is_none_or(f64::is_finite)
            && self.within == self.within_band()
    }
}

/// Each public real flight's row: the flights compared first, in `flights`' order, then those
/// predicted first, in `predicted`'s. Each is flown twice more, as the report flies it and with
/// the time step halved.
///
/// # Errors
///
/// [`RealFlightError`] if a flight doesn't fly, a row has no flight, or the second run of a
/// flight took no more steps than the first.
pub fn rows(
    root: &Path,
    flights: &[FlightRow],
    predicted: &[ComparedPrediction],
) -> Result<Vec<UncertaintyRow>, RealFlightError> {
    let missing = |id: &str| RealFlightError::Input {
        flight: id.to_owned(),
        what: "a row with no flight to fly again".to_owned(),
    };
    let input = |id: &str, what: String| RealFlightError::Input {
        flight: id.to_owned(),
        what,
    };
    // The data part: the flight's altimeter on its log's apogee. A pressure term is turned into
    // a height at the site's elevation plus that apogee; only flights with a site carry one.
    let data =
        |id: &str, log_apogee_m: f64, site_elevation_m: Option<f64>, formula_gap_m: Option<f64>| {
            let instrument = instrument(id)
                .ok_or_else(|| input(id, "a flight with no altimeter in the table".to_owned()))?;
            let measured = match site_elevation_m {
                Some(elevation_m) if instrument.terms.iter().any(Term::is_pressure) => {
                    Measured::at(elevation_m, log_apogee_m, formula_gap_m)
                        .map_err(|error| input(id, format!("the standard atmosphere: {error}")))?
                }
                _ => Measured {
                    formula_gap_m,
                    ..Measured::default()
                },
            };
            data_part(instrument, log_apogee_m, &measured).map_err(|what| input(id, what))
        };
    let row = |id: &str,
               (error_m, log_apogee_m): (f64, f64),
               coarse: Varied,
               fine: Varied,
               data: DataPart|
     -> Result<Unfinished, RealFlightError> {
        let log_height_m = coarse.log_height_m.ok_or_else(|| {
            input(
                id,
                "the flight was flown without its log's apogee asked of its altimeter".to_owned(),
            )
        })?;
        let numerical_m = refinement_ratio(coarse.steps_to_apogee, fine.steps_to_apogee)
            .and_then(|ratio| numerical_uncertainty_m(coarse.apogee_m, fine.apogee_m, ratio))
            .ok_or_else(|| RealFlightError::Input {
                flight: id.to_owned(),
                what: format!(
                    "the run with the step halved took {} steps, the report's {}: not finer",
                    fine.steps_to_apogee, coarse.steps_to_apogee
                ),
            })?;
        Ok(Unfinished {
            id: id.to_owned(),
            comparison_error_m: error_m,
            apogee_m: coarse.apogee_m,
            apogee_step_halved_m: fine.apogee_m,
            steps: coarse.steps_to_apogee,
            steps_step_halved: fine.steps_to_apogee,
            numerical_m,
            log_apogee_m,
            height_apogee_m: coarse.height_apogee_m,
            log_height_m,
            data,
        })
    };
    let mut out = Vec::with_capacity(flights.len() + predicted.len());
    for compared in flights {
        let flight = FLIGHTS
            .iter()
            .find(|f| f.id == compared.id)
            .ok_or_else(|| missing(&compared.id))?;
        let coarse = fly_varied(root, flight, Variant::TIP_OFF)?;
        let fine = fly_varied(root, flight, Variant::STEP_HALVED)?;
        let part = data(
            &compared.id,
            compared.log_apogee_m,
            Some(flight.site.2),
            compared.pressure_reading_apogee_m,
        )?;
        out.push(row(
            &compared.id,
            (
                compared.hpr_apogee_m - compared.log_apogee_m,
                compared.log_apogee_m,
            ),
            coarse,
            fine,
            part,
        )?);
    }
    let readings = predicted
        .iter()
        .map(|compared| (compared.id.clone(), compared.log_apogee_m))
        .collect::<Vec<_>>();
    let mut coarse =
        crate::public_predictions::varied_apogees_at(root, Variant::TIP_OFF, &readings)?;
    let mut fine = crate::public_predictions::varied_apogees(root, Variant::STEP_HALVED)?;
    let take = |list: &mut Vec<crate::public_predictions::VariedApogee>, id: &str| {
        let at = list
            .iter()
            .position(|(each, _)| each == id)
            .ok_or_else(|| missing(id))?;
        list.swap_remove(at).1
    };
    for compared in predicted {
        let coarse_run = take(&mut coarse, &compared.id)?;
        let fine_run = take(&mut fine, &compared.id)?;
        let part = data(&compared.id, compared.log_apogee_m, None, None)?;
        out.push(row(
            &compared.id,
            (
                compared.hpr_apogee_m - compared.log_apogee_m,
                compared.log_apogee_m,
            ),
            coarse_run,
            fine_run,
            part,
        )?);
    }
    with_input_parts(root, out)
}

/// A row before its input part.
struct Unfinished {
    id: String,
    comparison_error_m: f64,
    apogee_m: f64,
    apogee_step_halved_m: f64,
    steps: u64,
    steps_step_halved: u64,
    numerical_m: f64,
    log_apogee_m: f64,
    height_apogee_m: f64,
    log_height_m: f64,
    data: DataPart,
}

/// `work` on every item, on as many threads as the machine has, the results in the items'
/// order. Each result depends on its item alone, so the threads change nothing but the time.
fn in_parallel<T: Sync, R: Send>(items: &[T], work: impl Fn(&T) -> R + Sync) -> Vec<R> {
    // A flight's stack frames are large unoptimized: a thread gets 8 MB, the main thread's on
    // Linux and macOS.
    const STACK_BYTES: usize = 8 << 20;
    let threads = std::thread::available_parallelism()
        .map_or(1, usize::from)
        .min(items.len().max(1));
    let next = AtomicUsize::new(0);
    let mut done: Vec<(usize, R)> = std::thread::scope(|scope| {
        let workers: Vec<_> = (0..threads)
            .map(|_| {
                let worker = || {
                    let mut done = Vec::new();
                    loop {
                        let at = next.fetch_add(1, Ordering::Relaxed);
                        let Some(item) = items.get(at) else {
                            return done;
                        };
                        done.push((at, work(item)));
                    }
                };
                std::thread::Builder::new()
                    .stack_size(STACK_BYTES)
                    .spawn_scoped(scope, worker)
            })
            .collect();
        let mut done = Vec::with_capacity(items.len());
        for worker in workers {
            match worker.map(std::thread::ScopedJoinHandle::join) {
                Ok(Ok(results)) => done.extend(results),
                // A worker that panicked: the panic goes on, as it would on one thread.
                Ok(Err(panic)) => std::panic::resume_unwind(panic),
                // A thread that couldn't start does no work; the others take its share, and the
                // items it would have taken are flown below.
                Err(_) => {}
            }
        }
        done
    });
    // With no thread at all, the work is done here.
    if done.is_empty() {
        done = items.iter().map(&work).enumerate().collect();
    }
    done.sort_by_key(|&(at, _)| at);
    done.into_iter().map(|(_, result)| result).collect()
}

/// Each row with its input part, its validation uncertainty and whether its error lies within
/// the band. Every flight is flown [`crate::input_uncertainty::SAMPLES`] times with all its inputs drawn, twice for each
/// drawn number moved alone, and once with nothing moved, which must be the report's own flight
/// bit for bit: the flights of a draw are then the report's flight with its inputs moved and
/// nothing else.
fn with_input_parts(
    root: &Path,
    rows: Vec<Unfinished>,
) -> Result<Vec<UncertaintyRow>, RealFlightError> {
    let input = |id: &str, what: String| RealFlightError::Input {
        flight: id.to_owned(),
        what,
    };
    let weather = crate::public_predictions::checked_conversion(root)?;
    // Each flight's draws: nothing moved, each number alone (low, then high), then the samples.
    let mut jobs: Vec<(usize, InputDraw)> = Vec::new();
    for (at, row) in rows.iter().enumerate() {
        let inputs = stated(&row.id)
            .ok_or_else(|| input(&row.id, "a flight with no stated inputs".to_owned()))?;
        jobs.push((at, InputDraw::NONE));
        for which in Drawn::ALL {
            jobs.push((at, InputDraw::one(which, -1.0, inputs)));
            jobs.push((at, InputDraw::one(which, 1.0, inputs)));
        }
        jobs.extend(draws(SEED, inputs).into_iter().map(|moved| (at, moved)));
    }
    let apogees = in_parallel(&jobs, |&(at, moved)| {
        let id = rows[at].id.as_str();
        let variant = Variant::drawn(moved);
        match FLIGHTS.iter().find(|flight| flight.id == id) {
            Some(flight) => fly_varied(root, flight, variant),
            None => crate::public_predictions::varied_apogee_of(root, id, &weather, variant),
        }
        .map(|varied| varied.apogee_m)
    });
    let mut apogees = apogees.into_iter();
    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        let inputs = stated(&row.id)
            .ok_or_else(|| input(&row.id, "a flight with no stated inputs".to_owned()))?;
        let mut next = || {
            apogees
                .next()
                .ok_or_else(|| input(&row.id, "a flight of its input part is missing".to_owned()))?
        };
        let unmoved_m = next()?;
        if unmoved_m != row.apogee_m {
            return Err(input(
                &row.id,
                format!(
                    "with no input moved the apogee is {unmoved_m}, the report's {}: the input \
                     part's flights aren't the report's flight",
                    row.apogee_m
                ),
            ));
        }
        let mut alone = [(0.0, 0.0); 7];
        for pair in &mut alone {
            *pair = (next()?, next()?);
        }
        // Every sample must fly: one that doesn't fails the report, so none is left out of
        // the spread.
        let samples = (0..crate::input_uncertainty::SAMPLES)
            .map(|_| next())
            .collect::<Result<Vec<f64>, _>>()?;
        let part = input_part(inputs, alone, &samples).map_err(|what| input(&row.id, what))?;
        let mut finished = UncertaintyRow {
            id: row.id,
            comparison_error_m: row.comparison_error_m,
            apogee_m: row.apogee_m,
            apogee_step_halved_m: row.apogee_step_halved_m,
            steps: row.steps,
            steps_step_halved: row.steps_step_halved,
            numerical_m: row.numerical_m,
            log_apogee_m: row.log_apogee_m,
            height_apogee_m: row.height_apogee_m,
            log_height_m: row.log_height_m,
            data: row.data,
            input: part,
            validation_m: None,
            within: None,
        };
        finished.validation_m = finished.validation_of_parts_m();
        finished.within = finished.within_band();
        out.push(finished);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The worked case, by hand. Validation: `u_num` = 1 m, `u_input` = 20 m, `u_D` = 10 m give
    /// `u_val` = √(1 + 400 + 100) = √501 = 22.3830 m. Numerical: an apogee of 1000.0 m flown
    /// again at 1000.3 m on 1000 steps for the first run's 500 (r = 2) differs by 0.3 m; the
    /// coarser run's index is 3 × 0.3 × 16 / (16 − 1) = 0.96 m, and over k = 2 the standard
    /// uncertainty is 0.48 m. On 1414 steps for 1000 (r ≈ √2, r⁴ ≈ 4) the index is
    /// 3 × 0.3 × 4 / 3 ≈ 1.2 m, and the standard uncertainty about 0.6 m.
    #[test]
    fn the_combination_on_a_case_worked_by_hand() {
        let u_val = validation_uncertainty_m(1.0, 20.0, 10.0);
        assert!((u_val - 22.383_029).abs() < 1e-6, "{u_val}");
        assert!((u_val * u_val - 501.0).abs() < 1e-9);
        let halved = refinement_ratio(500, 1000).expect("500 steps is some");
        assert_eq!(halved, 2.0);
        let u_num = numerical_uncertainty_m(1000.0, 1000.3, halved).expect("r = 2 is finer");
        assert!((u_num - 0.48).abs() < 1e-12, "{u_num}");
        let root_two = refinement_ratio(1000, 1414).expect("1000 steps is some");
        let wide = numerical_uncertainty_m(1000.0, 1000.3, root_two).expect("r > 1 is finer");
        assert!((wide - 0.6).abs() < 1e-3, "{wide}");
        // The fourth power is products alone, exact for a power of two.
        assert_eq!(fourth_power(2.0), 16.0);
        assert_eq!(fourth_power(1.5), 5.0625);
        // Which apogee is the coarse one doesn't matter; no difference, no part.
        assert_eq!(Some(u_num), numerical_uncertainty_m(1000.3, 1000.0, 2.0));
        assert_eq!(numerical_uncertainty_m(512.0, 512.0, 2.0), Some(0.0));
        // A second run no finer than the first, or no steps, gives no index.
        assert_eq!(numerical_uncertainty_m(1000.0, 1000.3, 1.0), None);
        assert_eq!(numerical_uncertainty_m(1000.0, 1000.3, 0.5), None);
        assert_eq!(numerical_uncertainty_m(1000.0, 1000.3, f64::NAN), None);
        assert_eq!(refinement_ratio(0, 1000), None);
        // A part alone is itself, and the parts add in squares, not in sums.
        assert_eq!(validation_uncertainty_m(0.0, 3.0, 0.0), 3.0);
        assert_eq!(validation_uncertainty_m(3.0, 4.0, 0.0), 5.0);
    }

    /// A row holds only with its own numerical part.
    #[test]
    fn a_row_holds_with_its_own_numerical_part() {
        let own = numerical_uncertainty_m(1000.0, 1000.3, 2.0).expect("r = 2 is finer");
        let mut row = UncertaintyRow {
            id: "astra".to_owned(),
            comparison_error_m: -12.0,
            apogee_m: 1000.0,
            apogee_step_halved_m: 1000.3,
            steps: 500,
            steps_step_halved: 1000,
            numerical_m: 0.48,
            log_apogee_m: 1012.0,
            height_apogee_m: 1030.0,
            log_height_m: 1042.5,
            data: data_part(
                instrument("astra").expect("the flight is in the table"),
                1012.0,
                &Measured::default(),
            )
            .expect("no term to compute"),
            input: crate::input_uncertainty::part_for_tests(20.0),
            validation_m: None,
            within: None,
        };
        assert!(!row.holds(), "0.48 isn't the float the formula gives");
        row.numerical_m = own;
        assert!(row.holds());
        row.numerical_m *= 2.0;
        assert!(!row.holds());
        row.numerical_m = own;
        row.steps_step_halved = 900;
        assert!(!row.holds(), "the part follows the steps' ratio");
        row.steps_step_halved = 500;
        assert!(!row.holds(), "a second run no finer holds no part");
        row.steps_step_halved = 1000;
        row.comparison_error_m = f64::NAN;
        assert!(!row.holds());
        row.comparison_error_m = -12.0;
        assert!(row.holds());
        // The conversions: 1000 − 1030 in HPR Sim's reading, 1012 − 1042.5 in the log's, and
        // the error as heights, 1030 − 1042.5.
        assert_eq!(row.conversion_m(), -30.0);
        assert_eq!(row.log_conversion_m(), -30.5);
        assert_eq!(row.height_error_m(), -12.5);
        // A data part that isn't the flight's altimeter's doesn't hold.
        row.id = "macie-xl-1".to_owned();
        assert!(!row.holds(), "a PerfectFlite has a part; this row has none");
        row.id = "astra".to_owned();
        row.log_height_m = f64::INFINITY;
        assert!(!row.holds());
    }
}
