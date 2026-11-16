//! The gap between HPR Sim's apogee and OpenRocket's, split by cause (M2.3c12, ADR-349).
//!
//! A flight both simulators fly is flown again by HPR Sim with OpenRocket's inputs swapped in for
//! its own, one after another, each added to the ones before (ADR-226, ADR-349 §3):
//!
//! 1. the thrust OpenRocket flew, against time ([`hpr_sim::ThrustHistory`]);
//! 2. its mass and center of mass, against time ([`hpr_sim::MassHistory`]);
//! 3. its drag coefficient, against Mach number ([`hpr_aero::DragTable`], [`drag_table`]);
//! 4. its air: the standard atmosphere through the pad's temperature and pressure.
//!
//! A part is the apogee before its swap less the apogee after it, as a share of OpenRocket's
//! apogee:
//!
//! ```text
//! thrust = (A₀ − A₁) / A_OR        A₀: HPR Sim's own flight
//! mass   = (A₁ − A₂) / A_OR        A₁: on OpenRocket's thrust
//! drag   = (A₂ − A₃) / A_OR        A₂: and on its mass and center of mass
//! air    = (A₃ − A₄) / A_OR        A₃: and on its drag
//! left   = (A₄ − A_OR) / A_OR      A₄: and in its air
//! other  = air + left
//! whole  = (A₀ − A_OR) / A_OR = thrust + mass + drag + other
//! ```
//!
//! So the parts sum to the whole by construction, and [`Parts::check_sum`] holds every flight
//! split to it. The parts depend on the order: the drag part is also taken with the drag swapped
//! first, on HPR Sim's own thrust and mass ([`Parts::drag_first`]).
//!
//! What no swap replaces stays HPR Sim's in every flight: its normal force, its center of
//! pressure, its damping of a turn and its inertia. So after the mass swap HPR Sim flies
//! OpenRocket's center of mass against its own center of pressure, which moves its stability
//! margin. What that does to the apogee is in the mass part, and no later swap undoes it: the
//! last flight keeps it, and the rest is measured from that flight to OpenRocket's own. So the
//! rest can take back what the mass part holds ([`GapSplit::mass_opposed_by_rest`] counts the
//! flights where both are large with opposite signs; it doesn't show they are one effect).
//!
//! A flight is not split, and is counted under the first of [`NOT_SPLIT`] that applies, when
//! OpenRocket's record lacks an input for it, when HPR Sim refuses one of the histories or fails
//! any of the flights the split needs (its own sampled at OpenRocket's times among them), or when
//! the flight on its own thrust and mass, sampled at OpenRocket's times, doesn't come back within
//! [`OWN_HISTORY_BOUND`] of its own apogee (M2.3c11's test).
//!
//! Inside the drag part, [`crate::nose_line`] splits off the pressure drag of a tangent-ogive
//! nose above Mach 1 (M2.3c13).
//!
//! The split sizes a difference between two programs, code against code. It says nothing about
//! which is nearer a real flight.

use std::fmt::Write as _;

use hpr_aero::DragTable;
use hpr_core::interp::{Extrapolation, Interpolation, Table1D};
use hpr_sim::{MassHistory, Simulation, ThrustHistory};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::fixture_flights::Tally;
use crate::histories::own::{OWN_HISTORY_BOUND, own_history};

/// The columns of a record flight's rows, as `histories.py` writes them.
pub(crate) const COLUMNS: [&str; 7] = [
    "time_s",
    "thrust_n",
    "mass_kg",
    "cg_m",
    "drag_coefficient",
    "mach",
    "altitude_m",
];
pub(crate) const TIME: usize = 0;
const THRUST: usize = 1;
const MASS: usize = 2;
const CG: usize = 3;
const DRAG: usize = 4;
const MACH: usize = 5;

/// The parts a flight's gap is split into, in the order a part dominates a tie.
pub(crate) const PARTS: [&str; 4] = ["thrust", "mass", "drag", "other"];

/// Why a flight both simulators fly is not split, in the order they are tried, with the report's
/// words for each.
pub(crate) const NOT_SPLIT: [(&str, &str); 3] = [
    (
        "lacks_input",
        "OpenRocket's record lacks an input for the flight",
    ),
    (
        "refused",
        "HPR Sim refuses a history or fails the swapped flight",
    ),
    (
        "own_history",
        "the flight on its own thrust and mass, sampled at OpenRocket's times, misses its own \
         apogee by more than 0.05%",
    ),
];

/// The largest difference, per cent of OpenRocket's apogee, between a flight's parts summed and
/// its whole: each is a few floating-point operations on apogees of the same size.
const SUM_TOLERANCE_PERCENT: f64 = 1e-9;

/// The air a flight is flown in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Air {
    /// HPR Sim's: the day's whole profile.
    Day,
    /// OpenRocket's: the standard atmosphere through the pad's temperature and pressure.
    OpenRocket,
}

/// Why a flight is not split: an index into [`NOT_SPLIT`], and the detail, kept on this machine.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NotSplit {
    pub(crate) reason: usize,
    pub(crate) detail: String,
}

impl NotSplit {
    fn lacks_input(detail: String) -> Self {
        Self { reason: 0, detail }
    }
    fn refused(detail: String) -> Self {
        Self { reason: 1, detail }
    }
    fn own_history(detail: String) -> Self {
        Self { reason: 2, detail }
    }
}

/// OpenRocket's inputs along one flight, as HPR Sim flies them.
#[derive(Debug, Clone)]
pub(crate) struct OpenRocketInputs {
    /// OpenRocket's times, launch to apogee, s.
    pub(crate) times_s: Vec<f64>,
    pub(crate) thrust: ThrustHistory,
    pub(crate) mass: MassHistory,
    pub(crate) drag: DragTable,
}

/// One record row's number in `column`: `None` for OpenRocket's not-a-number.
fn cell(row: &Value, column: usize) -> Result<Option<f64>, String> {
    match &row[column] {
        Value::Null => Ok(None),
        value => value
            .as_f64()
            .filter(|x| x.is_finite())
            .map(Some)
            .ok_or_else(|| format!("a row's {} is not a number", COLUMNS[column])),
    }
}

/// OpenRocket's drag coefficient along its flight as a table HPR Sim can fly, by ADR-097's two
/// curves in Mach number, each linear between its points and held at its ends, on the area of
/// a circle OpenRocket's reference length across:
///
/// - power on: the rows with thrust, each at a Mach number above every earlier one's;
/// - power off: the rows after the last one with thrust, to apogee, each at a Mach number below
///   every earlier one's.
///
/// Which curve a swapped flight reads follows HPR Sim's own motors, which still set the burnout
/// under a thrust history; both programs fly the same thrust curve for each motor.
///
/// A row whose Mach number is zero or whose coefficient OpenRocket gives as not a number is
/// passed over. `rows` are `(thrust, drag coefficient, Mach number)`.
///
/// # Errors
///
/// A curve of fewer than two points, or a reference length that isn't positive: no table.
pub(crate) fn drag_table(
    rows: &[(f64, Option<f64>, Option<f64>)],
    reference_length_m: f64,
) -> Result<DragTable, String> {
    let last_burning = rows.iter().rposition(|(thrust, _, _)| *thrust > 0.0);
    let mut on: Vec<(f64, f64)> = Vec::new();
    let mut off: Vec<(f64, f64)> = Vec::new();
    for (i, (thrust, drag, mach)) in rows.iter().enumerate() {
        let (Some(drag), Some(mach)) = (*drag, *mach) else {
            continue;
        };
        if mach <= 0.0 {
            continue;
        }
        if *thrust > 0.0 && on.last().is_none_or(|(last, _)| mach > *last) {
            on.push((mach, drag));
        }
        if last_burning.is_none_or(|last| i > last)
            && off.last().is_none_or(|(last, _)| mach < *last)
        {
            off.push((mach, drag));
        }
    }
    off.reverse();
    let curve = |points: Vec<(f64, f64)>, what: &str| {
        if points.len() < 2 {
            return Err(format!(
                "OpenRocket's {what} drag curve has {} points",
                points.len()
            ));
        }
        let (xs, ys) = points.into_iter().unzip();
        Table1D::new(xs, ys, Interpolation::Linear, Extrapolation::Clamp)
            .map_err(|error| format!("OpenRocket's {what} drag curve: {error}"))
    };
    DragTable::new(curve(off, "power-off")?, Some(curve(on, "power-on")?))
        .with_reference_diameter_m(reference_length_m)
        .map_err(|error| format!("OpenRocket's drag curves: {error}"))
}

impl OpenRocketInputs {
    /// A record flight's inputs: `Ok(Err(_))` when the record lacks one or HPR Sim refuses a
    /// history built from it.
    ///
    /// # Errors
    ///
    /// A record that is not `histories.py`'s: no rows, a cell that isn't a number or null, no
    /// reference length, or rows that give no drag curve.
    pub(crate) fn of(flight: &Value) -> Result<Result<Self, NotSplit>, String> {
        if let Some(lacks) = flight.get("lacks") {
            return Ok(Err(NotSplit::lacks_input(lacks.to_string())));
        }
        let rows = flight["rows"]
            .as_array()
            .ok_or("a flight holding every input has no rows")?;
        let column = |index: usize| -> Result<Vec<f64>, String> {
            rows.iter()
                .map(|row| {
                    cell(row, index)?.ok_or_else(|| {
                        format!(
                            "a flight holding every input has no {} at a row",
                            COLUMNS[index]
                        )
                    })
                })
                .collect()
        };
        let times_s = column(TIME)?;
        let thrust_n = column(THRUST)?;
        let drag_rows = rows
            .iter()
            .zip(&thrust_n)
            .map(|(row, thrust)| Ok((*thrust, cell(row, DRAG)?, cell(row, MACH)?)))
            .collect::<Result<Vec<_>, String>>()?;
        let reference = flight["reference_length_m"]
            .as_f64()
            .ok_or("a flight holding every input has no reference length")?;
        // The histories first: a thrust HPR Sim refuses (one that doesn't end at zero, say) is
        // counted, where the rows it would leave the drag curves stop the run.
        let thrust = match ThrustHistory::new(times_s.clone(), thrust_n) {
            Ok(thrust) => thrust,
            Err(error) => return Ok(Err(NotSplit::refused(error.to_string()))),
        };
        let mass = match MassHistory::new(times_s.clone(), column(MASS)?, column(CG)?) {
            Ok(mass) => mass,
            Err(error) => return Ok(Err(NotSplit::refused(error.to_string()))),
        };
        let drag = drag_table(&drag_rows, reference)?;
        Ok(Ok(Self {
            times_s,
            thrust,
            mass,
            drag,
        }))
    }
}

/// The apogees one flight's split is taken from, each the height climbed, m.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Apogees {
    /// HPR Sim's own flight, in the day's air.
    pub(crate) own_m: f64,
    /// On OpenRocket's thrust.
    pub(crate) thrust_m: f64,
    /// On OpenRocket's thrust, mass and center of mass.
    pub(crate) mass_m: f64,
    /// On OpenRocket's thrust, mass, center of mass and drag.
    pub(crate) drag_m: f64,
    /// The same in OpenRocket's air.
    pub(crate) air_m: f64,
    /// On OpenRocket's drag alone: HPR Sim's own thrust and mass, in the day's air.
    pub(crate) drag_first_m: f64,
    /// OpenRocket's flight.
    pub(crate) openrocket_m: f64,
}

/// One flight's gap and its parts, each per cent of OpenRocket's apogee.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Parts {
    /// HPR Sim's apogee less OpenRocket's.
    pub(crate) whole: f64,
    pub(crate) thrust: f64,
    pub(crate) mass: f64,
    pub(crate) drag: f64,
    /// Inside other: what flying OpenRocket's air moves, after the three swaps.
    pub(crate) air: f64,
    /// Inside other: the flight on every swap less OpenRocket's own.
    pub(crate) left: f64,
    /// The drag part with the drag swapped first.
    pub(crate) drag_first: f64,
}

impl Parts {
    /// The parts of a flight from its apogees.
    pub(crate) fn of(apogees: Apogees) -> Self {
        let Apogees {
            own_m,
            thrust_m,
            mass_m,
            drag_m,
            air_m,
            drag_first_m,
            openrocket_m,
        } = apogees;
        let share = |before_m: f64, after_m: f64| 100.0 * (before_m - after_m) / openrocket_m;
        Self {
            whole: share(own_m, openrocket_m),
            thrust: share(own_m, thrust_m),
            mass: share(thrust_m, mass_m),
            drag: share(mass_m, drag_m),
            air: share(drag_m, air_m),
            left: share(air_m, openrocket_m),
            drag_first: share(own_m, drag_first_m),
        }
    }

    /// What thrust, mass and drag leave: the air's line and what is left after it.
    pub(crate) fn other(&self) -> f64 {
        self.air + self.left
    }

    /// Thrust, mass, drag and other, in [`PARTS`]' order.
    pub(crate) fn four(&self) -> [f64; 4] {
        [self.thrust, self.mass, self.drag, self.other()]
    }

    /// The part that dominates the flight, an index into [`PARTS`]: the one whose absolute
    /// value is the largest, the earlier of two that tie.
    pub(crate) fn dominant(&self) -> usize {
        let four = self.four();
        let mut dominant = 0;
        for (i, part) in four.iter().enumerate() {
            if part.abs() > four[dominant].abs() {
                dominant = i;
            }
        }
        dominant
    }

    /// Holds the parts, the air's among them, to sum to the whole.
    pub(crate) fn check_sum(&self) -> Result<(), String> {
        let sum = self.thrust + self.mass + self.drag + self.air + self.left;
        if !sum.is_finite() || (sum - self.whole).abs() > SUM_TOLERANCE_PERCENT {
            return Err(format!(
                "the parts sum to {sum}%, not the whole's {}%",
                self.whole
            ));
        }
        Ok(())
    }
}

/// One flight split: its parts, and how far the flight on its own histories came from its own
/// apogee, relative.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct SplitFlight {
    pub(crate) parts: Parts,
    pub(crate) own_history: f64,
    /// The apogees the parts are taken from, which the nose's line reads
    /// ([`crate::nose_line`]).
    pub(crate) apogees: Apogees,
}

/// Splits one flight both simulators fly. `build` gives its simulation in an air, as the
/// comparison flies it; `climbed_m` flies one and gives the height it climbed; `flight` is
/// OpenRocket's record of its inputs; `own_m` and `openrocket_m` are the two apogees the whole
/// is taken from; `bound` is the own-history test's, relative.
///
/// # Errors
///
/// A record that is not `histories.py`'s ([`OpenRocketInputs::of`]), or parts that don't sum to
/// the whole ([`Parts::check_sum`]).
pub(crate) fn split(
    build: &dyn Fn(Air) -> Result<Simulation, String>,
    climbed_m: &dyn Fn(&Simulation) -> Result<f64, String>,
    flight: &Value,
    (own_m, openrocket_m): (f64, f64),
    bound: f64,
) -> Result<Result<SplitFlight, NotSplit>, String> {
    let positive = openrocket_m > 0.0 && openrocket_m.is_finite();
    if !positive {
        return Err(format!(
            "OpenRocket's apogee of {openrocket_m} m is not a height to take shares of"
        ));
    }
    let inputs = match OpenRocketInputs::of(flight)? {
        Ok(inputs) => inputs,
        Err(not_split) => return Ok(Err(not_split)),
    };
    // HPR Sim's refusals and failures: the flight is counted, and the run goes on.
    let flown = || -> Result<(f64, [f64; 5]), String> {
        let own = own_history(|| build(Air::Day), &inputs.times_s)?;
        let history = |simulation: Simulation, mass: bool| -> Result<Simulation, String> {
            let simulation = simulation
                .with_thrust_history(inputs.thrust.clone())
                .map_err(|error| error.to_string())?;
            if mass {
                simulation
                    .with_mass_history(inputs.mass.clone())
                    .map_err(|error| error.to_string())
            } else {
                Ok(simulation)
            }
        };
        let thrust_m = climbed_m(&history(build(Air::Day)?, false)?)?;
        let mass_m = climbed_m(&history(build(Air::Day)?, true)?)?;
        let drag_m =
            climbed_m(&history(build(Air::Day)?, true)?.with_drag_table(inputs.drag.clone()))?;
        let air_m = climbed_m(
            &history(build(Air::OpenRocket)?, true)?.with_drag_table(inputs.drag.clone()),
        )?;
        let drag_first_m = climbed_m(&build(Air::Day)?.with_drag_table(inputs.drag.clone()))?;
        Ok((
            own.relative(),
            [thrust_m, mass_m, drag_m, air_m, drag_first_m],
        ))
    };
    let (own_history, [thrust_m, mass_m, drag_m, air_m, drag_first_m]) = match flown() {
        Ok(flown) => flown,
        Err(error) => return Ok(Err(NotSplit::refused(error))),
    };
    // A difference or a bound that isn't a number fails the test too.
    let within = own_history.abs() <= bound;
    if !within {
        return Ok(Err(NotSplit::own_history(format!(
            "{:+.5}%",
            100.0 * own_history
        ))));
    }
    let apogees = Apogees {
        own_m,
        thrust_m,
        mass_m,
        drag_m,
        air_m,
        drag_first_m,
        openrocket_m,
    };
    let parts = Parts::of(apogees);
    parts.check_sum()?;
    Ok(Ok(SplitFlight {
        parts,
        own_history,
        apogees,
    }))
}

/// A mean, a median and a mean absolute value over some flights, per cent of OpenRocket's
/// apogee.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct Size {
    pub mean_percent: f64,
    /// The middle value, or the mean of the middle two.
    pub median_percent: f64,
    pub mean_absolute_percent: f64,
}

/// `value` rounded to two decimals, for a report that reads the same on every run. A sum of no
/// flights is 0, not −0.
fn round2(value: f64) -> f64 {
    (value * 100.0).round() / 100.0 + 0.0
}

impl Size {
    pub(crate) fn of(values: impl Iterator<Item = f64> + Clone) -> Self {
        let n = values.clone().count();
        if n == 0 {
            return Self::default();
        }
        let mut sorted: Vec<f64> = values.clone().collect();
        sorted.sort_by(f64::total_cmp);
        let median = (sorted[(n - 1) / 2] + sorted[n / 2]) / 2.0;
        let n = n as f64;
        Self {
            mean_percent: round2(values.clone().fold(0.0, |sum, v| sum + v) / n),
            median_percent: round2(median),
            mean_absolute_percent: round2(values.fold(0.0, |sum, v| sum + v.abs()) / n),
        }
    }

    /// A mean absolute value is no smaller than the mean's own, to the rounding of each.
    pub(crate) fn consistent(&self, what: &str) -> Result<(), String> {
        if !self.mean_percent.is_finite()
            || !self.median_percent.is_finite()
            || !self.mean_absolute_percent.is_finite()
            || self.mean_absolute_percent + 0.01 + 1e-9 < self.mean_percent.abs()
        {
            return Err(format!(
                "the split's {what}: a mean of {}, a median of {} and a mean absolute value of {}",
                self.mean_percent, self.median_percent, self.mean_absolute_percent
            ));
        }
        Ok(())
    }
}

/// One part over the flights split.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct Part {
    /// Its name, from [`PARTS`].
    pub name: String,
    pub mean_percent: f64,
    pub median_percent: f64,
    pub mean_absolute_percent: f64,
    /// The flights it dominates: where its absolute value is the largest of the four.
    pub dominates: usize,
    /// The flights where its absolute value is over [`LARGE_PERCENT`].
    pub large: usize,
}

/// A part counts as large on a flight past this share of OpenRocket's apogee, per cent: a mean
/// carried by a few flights shows as a small median beside a count of them.
pub(crate) const LARGE_PERCENT: f64 = 1.0;

impl Part {
    fn size(&self) -> Size {
        Size {
            mean_percent: self.mean_percent,
            median_percent: self.median_percent,
            mean_absolute_percent: self.mean_absolute_percent,
        }
    }
}

/// The report's section: the gap split over the flights both simulators fly, as aggregates.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct GapSplit {
    /// The flights both simulators fly.
    pub flown_by_both: usize,
    /// Those split.
    pub split: usize,
    /// The rest, under each reason of [`NOT_SPLIT`], in its order.
    pub not_split: Vec<Tally>,
    /// HPR Sim's apogee less OpenRocket's on the flights split.
    pub whole_on_split: Size,
    /// The same on every flight both fly.
    pub whole_on_both: Size,
    /// Thrust, mass, drag and other over the flights split, in [`PARTS`]' order.
    pub parts: Vec<Part>,
    /// Inside other: the air's line.
    pub air: Size,
    /// Inside other: what is left after the air's line.
    pub left: Size,
    /// The drag part with the drag swapped first.
    pub drag_first: Size,
    /// The largest miss of a split flight on its own histories from its own apogee, per cent,
    /// to two significant digits; the test's bound is 0.05.
    pub largest_own_history_percent: f64,
    /// The flights where the mass part and the rest are each over [`LARGE_PERCENT`] with
    /// opposite signs. It fits a change in stability that the mass swap makes and the rest
    /// takes back; it doesn't show the two are one effect.
    pub mass_opposed_by_rest: usize,
}

impl GapSplit {
    /// The section's numbers from each flight both fly: its whole, per cent of OpenRocket's
    /// apogee, and its split or why it has none.
    pub(crate) fn of(flights: &[(f64, Result<SplitFlight, NotSplit>)]) -> Self {
        let split: Vec<&SplitFlight> = flights
            .iter()
            .filter_map(|(_, flight)| flight.as_ref().ok())
            .collect();
        let over = |of: &dyn Fn(&Parts) -> f64| Size::of(split.iter().map(|s| of(&s.parts)));
        let parts = PARTS
            .iter()
            .enumerate()
            .map(|(i, name)| {
                let size = over(&|parts| parts.four()[i]);
                Part {
                    name: (*name).to_owned(),
                    mean_percent: size.mean_percent,
                    median_percent: size.median_percent,
                    mean_absolute_percent: size.mean_absolute_percent,
                    dominates: split.iter().filter(|s| s.parts.dominant() == i).count(),
                    large: split
                        .iter()
                        .filter(|s| s.parts.four()[i].abs() > LARGE_PERCENT)
                        .count(),
                }
            })
            .collect();
        let largest = split
            .iter()
            .fold(0.0_f64, |largest, s| largest.max(s.own_history.abs()));
        Self {
            flown_by_both: flights.len(),
            split: split.len(),
            not_split: NOT_SPLIT
                .iter()
                .enumerate()
                .map(|(i, (name, _))| Tally {
                    reason: (*name).to_owned(),
                    flights: flights
                        .iter()
                        .filter(|(_, f)| f.as_ref().is_err_and(|not| not.reason == i))
                        .count(),
                })
                .collect(),
            whole_on_split: over(&|parts| parts.whole),
            whole_on_both: Size::of(flights.iter().map(|(whole, _)| *whole)),
            parts,
            air: over(&|parts| parts.air),
            left: over(&|parts| parts.left),
            drag_first: over(&|parts| parts.drag_first),
            largest_own_history_percent: crate::histories::significant(100.0 * largest, 2),
            mass_opposed_by_rest: split
                .iter()
                .filter(|s| {
                    let (mass, left) = (s.parts.mass, s.parts.left);
                    mass.abs() > LARGE_PERCENT && left.abs() > LARGE_PERCENT && mass * left < 0.0
                })
                .count(),
        }
    }

    /// Holds the section to itself and to the report's `flown_by_both` flights: each flight
    /// split or under a reason, at least half of them split, each split flight dominated by one
    /// part, and the parts' means summing to the whole's to the rounding of each.
    pub(crate) fn check_consistent(&self, flown_by_both: usize) -> Result<(), String> {
        let fail = |why: String| Err(format!("the split: {why}"));
        let reasons: Vec<&str> = self.not_split.iter().map(|t| t.reason.as_str()).collect();
        let expected: Vec<&str> = NOT_SPLIT.iter().map(|(name, _)| *name).collect();
        if reasons != expected {
            return fail(format!("the reasons are {reasons:?}, not {expected:?}"));
        }
        let names: Vec<&str> = self.parts.iter().map(|p| p.name.as_str()).collect();
        if names != PARTS {
            return fail(format!("the parts are {names:?}, not {PARTS:?}"));
        }
        // Saturating: a count read from a file can be any number.
        let total =
            |counts: &mut dyn Iterator<Item = usize>| counts.fold(0_usize, usize::saturating_add);
        let not_split = total(&mut self.not_split.iter().map(|t| t.flights));
        if self.flown_by_both != flown_by_both
            || self.split.saturating_add(not_split) != flown_by_both
        {
            return fail(format!(
                "{} split and {not_split} not split are not the {flown_by_both} flights both fly",
                self.split
            ));
        }
        if self.split.saturating_mul(2) < flown_by_both {
            return fail(format!(
                "{} of the {flown_by_both} flights both fly are split, fewer than half (ADR-349 §3)",
                self.split
            ));
        }
        let dominated = total(&mut self.parts.iter().map(|p| p.dominates));
        if dominated != self.split {
            return fail(format!(
                "{dominated} flights are dominated by a part, not the {} split",
                self.split
            ));
        }
        if let Some(part) = self.parts.iter().find(|p| p.large > self.split) {
            return fail(format!(
                "{} is large on {} flights, more than the {} split",
                part.name, part.large, self.split
            ));
        }
        for (what, size) in [
            ("whole on the flights split", self.whole_on_split),
            ("whole on the flights both fly", self.whole_on_both),
            ("air", self.air),
            ("rest", self.left),
            ("drag swapped first", self.drag_first),
        ]
        .into_iter()
        .chain(self.parts.iter().map(|p| (p.name.as_str(), p.size())))
        {
            size.consistent(what)?;
        }
        if self.mass_opposed_by_rest > self.parts[1].large {
            return fail(format!(
                "the rest opposes a large mass part on {} flights, more than the {} where the \
                 mass part is large",
                self.mass_opposed_by_rest, self.parts[1].large
            ));
        }
        // Each mean is rounded to 0.005: five of them, then three.
        let sum: f64 = self.parts.iter().map(|p| p.mean_percent).sum();
        if (sum - self.whole_on_split.mean_percent).abs() > 0.025 + 1e-9 {
            return fail(format!(
                "the parts' means sum to {sum}, not the whole's {}",
                self.whole_on_split.mean_percent
            ));
        }
        let other = self.parts[3].mean_percent;
        if (self.air.mean_percent + self.left.mean_percent - other).abs() > 0.015 + 1e-9 {
            return fail(format!(
                "the air's mean and the rest's don't sum to other's {other}"
            ));
        }
        if self.split == flown_by_both && self.whole_on_split != self.whole_on_both {
            return fail("every flight is split, yet the two wholes differ".to_owned());
        }
        if !(0.0..=100.0 * OWN_HISTORY_BOUND).contains(&self.largest_own_history_percent) {
            return fail(format!(
                "a split flight missed its own apogee by {}%, past the bound",
                self.largest_own_history_percent
            ));
        }
        Ok(())
    }

    /// The report's section.
    pub(crate) fn to_markdown(&self) -> String {
        let mut page = String::new();
        let _ = writeln!(page, "\n## The gap to OpenRocket, by cause\n");
        let _ = writeln!(
            page,
            "For M2.3c12 (ADR-349), HPR Sim flies each flight again with OpenRocket's inputs from \
             the section above swapped in for its own, one after another, each added to the ones \
             before: OpenRocket's thrust against time, then its mass and center of mass against \
             time, then its drag coefficient against Mach number, then its air (the standard \
             atmosphere through the pad's temperature and pressure, in place of the day's \
             profile). A part is the apogee before its swap less the apogee after it. Other is \
             what thrust, mass and drag leave: the air's line, and the last flight's apogee less \
             OpenRocket's own. So the parts sum to the whole, HPR Sim's apogee less OpenRocket's, \
             and the run stops if they don't on a flight. Every number is a share of \
             OpenRocket's apogee. This is code against code: it sizes where the two simulators \
             differ and says nothing about which is nearer a flight.\n"
        );
        let _ = writeln!(
            page,
            "Of the {} flights both simulators fly, {} are split. These include the flights \
             left out of the errors above, whose logs are no fair test: the split needs no \
             log. A part dominates a flight when its absolute value is the largest of thrust, \
             mass, drag and other, the earlier in that order if two tie. The last column counts \
             the flights where a part's absolute value is over {LARGE_PERCENT}%.\n",
            self.flown_by_both, self.split
        );
        let _ = writeln!(
            page,
            "| part | mean | median | mean absolute | flights it dominates | flights where it is \
             over {LARGE_PERCENT}% |\n|---|---:|---:|---:|---:|---:|"
        );
        let row = |page: &mut String, name: &str, size: Size, counts: Option<(usize, usize)>| {
            let count = |n: Option<usize>| n.map_or_else(String::new, |n| n.to_string());
            let _ = writeln!(
                page,
                "| {name} | {:+.2}% | {:+.2}% | {:.2}% | {} | {} |",
                size.mean_percent,
                size.median_percent,
                size.mean_absolute_percent,
                count(counts.map(|c| c.0)),
                count(counts.map(|c| c.1))
            );
        };
        for part in &self.parts {
            row(
                &mut page,
                &part.name,
                part.size(),
                Some((part.dominates, part.large)),
            );
            if part.name == PARTS[3] {
                row(&mut page, "other: the air", self.air, None);
                row(&mut page, "other: the rest", self.left, None);
            }
        }
        row(
            &mut page,
            "the whole, on the flights split",
            self.whole_on_split,
            None,
        );
        row(
            &mut page,
            "the whole, on every flight both fly",
            self.whole_on_both,
            None,
        );
        row(&mut page, "drag, swapped first", self.drag_first, None);
        let _ = writeln!(
            page,
            "\nThe parts depend on the order. The last row is the drag part with the drag \
             swapped first, on HPR Sim's own thrust and mass, beside the drag row's, which \
             comes after thrust and mass. The drag swap is a table in Mach number, power on and \
             power off, read from OpenRocket's own flight, so it leaves out how OpenRocket's \
             drag depends on the Reynolds number and the angle of attack away from that \
             flight's path. The mass swap leaves HPR Sim's inertia and its center of pressure in \
             place, and no swap replaces its normal force, its center of pressure or its damping \
             of a turn: those are in the rest. So where the two simulators put the center of \
             mass apart, HPR Sim flies OpenRocket's center of mass against its own center of \
             pressure from the mass swap on, which moves its stability margin. What that does \
             to the apogee is in the mass row, and no later swap undoes it: the last flight \
             keeps it, and the rest is measured from that flight to OpenRocket's own. So the \
             rest can take back what the mass row holds. Of the {} {} where the mass part \
             is over {LARGE_PERCENT}%, the rest is over {LARGE_PERCENT}% with the other sign on \
             {}, which fits that reading and doesn't prove it. The air's line is taken with the \
             drag table already in place, so it holds the air's density and speed of sound and \
             not what the air does to friction. Each flight split also passed M2.3c11's test: \
             flown on its own thrust and mass sampled at OpenRocket's times, it came back to \
             its own apogee within 0.05%; the largest miss was {}%.\n",
            self.parts[1].large,
            if self.parts[1].large == 1 {
                "flight"
            } else {
                "flights"
            },
            self.mass_opposed_by_rest,
            self.largest_own_history_percent
        );
        let _ = writeln!(
            page,
            "A flight both fly that is not split is counted under the first reason that \
             applies:\n"
        );
        let _ = writeln!(page, "| why a flight is not split | flights |\n|---|---:|");
        for (tally, (_, words)) in self.not_split.iter().zip(NOT_SPLIT) {
            let _ = writeln!(page, "| {words} | {} |", tally.flights);
        }
        page
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::histories::tests::{Public, public_flights};
    use serde_json::json;

    fn apogees() -> Apogees {
        Apogees {
            own_m: 1030.0,
            thrust_m: 1025.0,
            mass_m: 1032.0,
            drag_m: 1004.0,
            air_m: 1001.5,
            drag_first_m: 1003.0,
            openrocket_m: 1000.0,
        }
    }

    /// Each part is the apogee before its swap less the one after, over OpenRocket's; they sum
    /// to the whole, and a part moved by more than the tolerance is caught.
    #[test]
    fn the_parts_sum_to_the_whole() {
        let parts = Parts::of(apogees());
        let near = |a: f64, b: f64| (a - b).abs() < 1e-12;
        assert!(near(parts.whole, 3.0));
        assert!(near(parts.thrust, 0.5));
        assert!(near(parts.mass, -0.7));
        assert!(near(parts.drag, 2.8));
        assert!(near(parts.air, 0.25));
        assert!(near(parts.left, 0.15));
        assert!(near(parts.other(), 0.4));
        assert!(near(parts.drag_first, 2.7));
        assert_eq!(parts.check_sum(), Ok(()));
        assert_eq!(parts.dominant(), 2);
        for moved in [
            Parts {
                thrust: parts.thrust + 1e-8,
                ..parts
            },
            Parts {
                air: parts.air - 1e-8,
                ..parts
            },
            Parts {
                left: f64::NAN,
                ..parts
            },
            Parts {
                whole: -parts.whole,
                ..parts
            },
        ] {
            assert!(moved.check_sum().unwrap_err().contains("not the whole's"));
        }
    }

    /// The part with the largest absolute value dominates, whatever its sign, other included,
    /// and the earlier of two that tie.
    #[test]
    fn the_largest_part_dominates() {
        let parts = |thrust: f64, mass: f64, drag: f64, air: f64, left: f64| Parts {
            whole: thrust + mass + drag + air + left,
            thrust,
            mass,
            drag,
            air,
            left,
            drag_first: 0.0,
        };
        assert_eq!(parts(-3.0, 1.0, 2.0, 0.5, 0.5).dominant(), 0);
        assert_eq!(parts(0.1, -0.4, 0.3, 0.0, 0.0).dominant(), 1);
        assert_eq!(parts(0.1, 0.2, 0.3, -0.2, -0.2).dominant(), 3);
        // The air and the rest cancel: other is their sum, not either alone.
        assert_eq!(parts(0.1, 0.2, 0.3, 5.0, -5.0).dominant(), 2);
        assert_eq!(parts(0.5, -0.5, 0.5, 0.25, 0.25).dominant(), 0);
        assert_eq!(parts(0.1, 0.5, -0.5, 0.25, 0.25).dominant(), 1);
    }

    /// The two curves of ADR-097: power on from the rows with thrust as the Mach number rises,
    /// power off from the rows after them as it falls, on OpenRocket's reference length.
    #[test]
    fn the_drag_table_is_openrockets_two_curves() {
        let rows = [
            (0.0, Some(1.2), Some(0.0)),
            (50.0, Some(1.3), Some(0.0)),
            (50.0, Some(0.9), Some(0.1)),
            (50.0, Some(0.6), Some(0.4)),
            (50.0, None, Some(0.5)),
            (50.0, Some(0.61), Some(0.39)),
            (20.0, Some(0.5), Some(0.8)),
            (0.0, Some(0.45), Some(0.7)),
            (0.0, Some(0.44), Some(0.71)),
            (0.0, Some(0.5), Some(0.3)),
            (0.0, Some(0.9), None),
            (0.0, Some(1.1), Some(0.01)),
        ];
        let table = drag_table(&rows, 0.06).unwrap();
        assert_eq!(table.reference_diameter_m, Some(0.06));
        let on = table.power_on.as_ref().unwrap();
        let at = |table: &Table1D, mach: f64| table.lookup(mach).unwrap().value;
        // On: 0.1, 0.4 and 0.8; the row at Mach 0, the null row and the slower one are passed
        // over.
        assert_eq!(at(on, 0.0), 0.9);
        assert_eq!(at(on, 0.1), 0.9);
        assert_eq!(at(on, 0.4), 0.6);
        assert!((at(on, 0.6) - 0.55).abs() < 1e-12);
        assert_eq!(at(on, 0.8), 0.5);
        assert_eq!(at(on, 2.0), 0.5);
        // Off: 0.7, 0.3 and 0.01; the faster row and the one with no Mach number are passed over.
        let off = &table.power_off;
        assert_eq!(at(off, 0.7), 0.45);
        assert!((at(off, 0.5) - 0.475).abs() < 1e-12);
        assert_eq!(at(off, 0.3), 0.5);
        assert_eq!(at(off, 0.01), 1.1);
        assert_eq!(at(off, 0.0), 1.1);
        // While it thrusts, the table gives the power-on curve.
        assert_eq!(table.lookup(0.4, true).unwrap().value, 0.6);
        assert!((table.lookup(0.4, false).unwrap().value - 0.4875).abs() < 1e-12);

        let coast_only = [(0.0, Some(1.0), Some(0.5)), (0.0, Some(1.0), Some(0.2))];
        assert!(
            drag_table(&coast_only, 0.06)
                .unwrap_err()
                .contains("power-on drag curve has 0 points")
        );
        let burn_only = [(5.0, Some(1.0), Some(0.2)), (5.0, Some(1.0), Some(0.5))];
        assert!(
            drag_table(&burn_only, 0.06)
                .unwrap_err()
                .contains("power-off drag curve has 0 points")
        );
        assert!(drag_table(&rows, 0.0).is_err());
    }

    fn fly(simulation: &Simulation) -> Result<f64, String> {
        crate::fixture_flights::climbed_m(simulation)
    }

    /// A public flight split against OpenRocket's own flight of it: both in the standard air, so
    /// the air's line is the same flight twice.
    fn split_public(public: &Public, flight: &Value, bound: f64) -> Result<SplitFlight, NotSplit> {
        let own_m = fly(&public.build().unwrap()).unwrap();
        let openrocket_m = flight["apogee_m"].as_f64().unwrap();
        split(
            &|_air| public.build(),
            &fly,
            flight,
            (own_m, openrocket_m),
            bound,
        )
        .unwrap()
    }

    /// What the documentation quotes for the two public designs, per cent of OpenRocket's apogee:
    /// the whole and the drag part, each held within [`PUBLIC_WITHIN`]. On `pods-none` the whole
    /// is 0.81% and the drag part 0.79%; on `mach-probe`, 3.06% and 3.03%.
    const PUBLIC_PARTS: [(&str, f64, f64); 2] = [
        ("pods-none.ork", 0.81, 0.79),
        ("mach-probe.ork", 3.06, 3.03),
    ];
    const PUBLIC_WITHIN: f64 = 0.01;

    /// The two public designs split: the parts sum to the whole, the air's line is zero in one
    /// air, each flight passes the own-history test, drag carries the gap, and flown on all
    /// three of OpenRocket's inputs HPR Sim comes within 0.05% of OpenRocket's apogee.
    #[test]
    fn the_public_flights_split() {
        let flights = public_flights();
        assert_eq!(flights.len(), PUBLIC_PARTS.len());
        for (public, (file, whole, drag)) in flights.iter().zip(PUBLIC_PARTS) {
            assert!(public.file.ends_with(file), "{}", public.file);
            let split = split_public(public, &public.flight, OWN_HISTORY_BOUND).unwrap();
            let parts = split.parts;
            eprintln!(
                "{}: whole {:+.3}% = thrust {:+.3}% + mass {:+.3}% + drag {:+.3}% + other \
                 {:+.3}% (air {:+.3}%, rest {:+.3}%); drag first {:+.3}%; own history {:+.5}%",
                public.file,
                parts.whole,
                parts.thrust,
                parts.mass,
                parts.drag,
                parts.other(),
                parts.air,
                parts.left,
                parts.drag_first,
                100.0 * split.own_history
            );
            assert_eq!(parts.check_sum(), Ok(()), "{file}");
            assert_eq!(parts.air, 0.0, "{file}");
            assert!(split.own_history.abs() <= OWN_HISTORY_BOUND);
            assert!((parts.whole - whole).abs() <= PUBLIC_WITHIN, "{file}");
            assert!((parts.drag - drag).abs() <= PUBLIC_WITHIN, "{file}");
            assert!((parts.drag_first - drag).abs() <= PUBLIC_WITHIN, "{file}");
            assert_eq!(parts.dominant(), 2, "{file}");
            assert!(
                parts.thrust.abs() < 0.005 && parts.mass.abs() < 0.005,
                "{file}"
            );
            assert!(parts.left.abs() < 0.05, "{file}: {}", parts.left);
        }
    }

    /// Each part is its own swap, added to the ones before, and only the air's line flies
    /// OpenRocket's air. OpenRocket's inputs are scaled so each swap moves the apogee by far more
    /// than a slip could hide in: 5% more thrust, 20% more mass, 30% more drag, and an
    /// "OpenRocket's air" 1,500 m higher than the day's. Each step is flown here on its own and
    /// the split's parts must be those flights', bit for bit.
    #[test]
    fn each_part_is_its_own_swap() {
        let flights = public_flights();
        let public = &flights[0];
        let mut scaled = public.flight.clone();
        for row in scaled["rows"].as_array_mut().unwrap() {
            for (column, factor) in [(THRUST, 1.05), (MASS, 1.2), (DRAG, 1.3)] {
                if let Some(value) = row[column].as_f64() {
                    row[column] = json!(factor * value);
                }
            }
        }
        let high_m = 1500.0;
        let build = |air: Air| match air {
            Air::Day => public.build(),
            Air::OpenRocket => public.build_at(high_m),
        };
        let own_m = fly(&public.build().unwrap()).unwrap();
        let openrocket_m = scaled["apogee_m"].as_f64().unwrap();
        let split = split(
            &build,
            &fly,
            &scaled,
            (own_m, openrocket_m),
            OWN_HISTORY_BOUND,
        )
        .unwrap()
        .unwrap();

        let inputs = OpenRocketInputs::of(&scaled).unwrap().unwrap();
        let thrust = |s: Simulation| s.with_thrust_history(inputs.thrust.clone()).unwrap();
        let mass = |s: Simulation| s.with_mass_history(inputs.mass.clone()).unwrap();
        let drag = |s: Simulation| s.with_drag_table(inputs.drag.clone());
        let day = || public.build().unwrap();
        let expected = Parts::of(Apogees {
            own_m,
            thrust_m: fly(&thrust(day())).unwrap(),
            mass_m: fly(&mass(thrust(day()))).unwrap(),
            drag_m: fly(&drag(mass(thrust(day())))).unwrap(),
            air_m: fly(&drag(mass(thrust(public.build_at(high_m).unwrap())))).unwrap(),
            drag_first_m: fly(&drag(day())).unwrap(),
            openrocket_m,
        });
        let parts = split.parts;
        eprintln!(
            "scaled {}: thrust {:+.2}% mass {:+.2}% drag {:+.2}% air {:+.2}% drag first {:+.2}%",
            public.file, parts.thrust, parts.mass, parts.drag, parts.air, parts.drag_first
        );
        assert_eq!(parts, expected);
        assert_eq!(parts.check_sum(), Ok(()));
        // More thrust raises the apogee, more drag lowers it, thinner air raises it; each by more
        // than 1% of OpenRocket's apogee. The apogee is a weak probe of the mass, which can raise
        // it or lower it: 20% more moves it by more than 0.2% here.
        assert!(parts.thrust < -1.0, "{}", parts.thrust);
        assert!(parts.mass.abs() > 0.2, "{}", parts.mass);
        assert!(parts.drag > 1.0, "{}", parts.drag);
        assert!(parts.air < -1.0, "{}", parts.air);
        assert!(parts.drag_first > 1.0, "{}", parts.drag_first);
        // The drag part depends on the order once thrust and mass differ.
        assert!((parts.drag - parts.drag_first).abs() > 0.05);
    }

    /// Each reason a flight is not split, on a public flight: the record lacks an input; HPR Sim
    /// refuses a history (times that don't rise, a negative thrust); the own-history test fails
    /// (here, against a bound of zero).
    #[test]
    fn a_flight_not_split_names_its_reason() {
        let flights = public_flights();
        let public = &flights[0];
        let reason = |flight: &Value, bound: f64| {
            let not = split_public(public, flight, bound).unwrap_err();
            (NOT_SPLIT[not.reason].0, not.detail)
        };

        let mut lacking = public.flight.clone();
        lacking["lacks"] = json!("cg_not_finite");
        lacking.as_object_mut().unwrap().remove("rows");
        assert_eq!(reason(&lacking, OWN_HISTORY_BOUND).0, "lacks_input");

        let mut twice = public.flight.clone();
        let rows = twice["rows"].as_array_mut().unwrap();
        let repeated = rows[10].clone();
        rows.insert(10, repeated);
        let (name, detail) = reason(&twice, OWN_HISTORY_BOUND);
        assert_eq!(name, "refused");
        assert!(detail.contains("rise"), "{detail}");

        let mut pulling = public.flight.clone();
        pulling["rows"][5][THRUST] = json!(-1.0);
        let (name, detail) = reason(&pulling, OWN_HISTORY_BOUND);
        assert_eq!(name, "refused");
        assert!(detail.contains("a thrust is negative"), "{detail}");

        let mut weightless = public.flight.clone();
        weightless["rows"][5][MASS] = json!(0.0);
        let (name, detail) = reason(&weightless, OWN_HISTORY_BOUND);
        assert_eq!(name, "refused");
        assert!(detail.contains("a mass isn't positive"), "{detail}");

        assert_eq!(reason(&public.flight, 0.0).0, "own_history");
        assert_eq!(reason(&public.flight, f64::NAN).0, "own_history");
        // A miss below the flight's own apogee counts as one above does: the second public
        // flight comes back 0.00063% low, past a bound of 0.0001%.
        let low = &flights[1];
        let own = split_public(low, &low.flight, OWN_HISTORY_BOUND).unwrap();
        assert!(own.own_history < -1e-6, "{}", own.own_history);
        let not = split_public(low, &low.flight, 1e-6).unwrap_err();
        assert_eq!(NOT_SPLIT[not.reason].0, "own_history");

        // An apogee of OpenRocket's that is no height stops the run.
        let own_m = fly(&public.build().unwrap()).unwrap();
        for apogee in [0.0, -5.0, f64::NAN, f64::INFINITY] {
            let stopped = split(
                &|_air| public.build(),
                &fly,
                &public.flight,
                (own_m, apogee),
                OWN_HISTORY_BOUND,
            );
            assert!(stopped.unwrap_err().contains("not a height"), "{apogee}");
        }

        // A record that is not the oracle's stops the run instead.
        let mut broken = public.flight.clone();
        broken["rows"][3][MASS] = json!("heavy");
        let own_m = fly(&public.build().unwrap()).unwrap();
        assert!(
            split(
                &|_air| public.build(),
                &fly,
                &broken,
                (own_m, 700.0),
                OWN_HISTORY_BOUND
            )
            .unwrap_err()
            .contains("mass_kg is not a number")
        );
    }

    fn flight(four: [f64; 4], air: f64, own_history: f64) -> (f64, Result<SplitFlight, NotSplit>) {
        let [thrust, mass, drag, other] = four;
        let parts = Parts {
            whole: thrust + mass + drag + other,
            thrust,
            mass,
            drag,
            air,
            left: other - air,
            drag_first: drag + 0.1,
        };
        (
            parts.whole,
            Ok(SplitFlight {
                parts,
                own_history,
                apogees: NO_APOGEES,
            }),
        )
    }

    /// Apogees no test of the report's numbers reads.
    const NO_APOGEES: Apogees = Apogees {
        own_m: 0.0,
        thrust_m: 0.0,
        mass_m: 0.0,
        drag_m: 0.0,
        air_m: 0.0,
        drag_first_m: 0.0,
        openrocket_m: 0.0,
    };

    /// A large mass part with a large rest of the other sign is counted: on the first flight,
    /// not where they share a sign, nor where the rest or the mass part is small.
    #[test]
    fn a_large_mass_part_the_rest_opposes_is_counted() {
        let of = |mass: f64, left: f64| {
            let parts = Parts {
                whole: 0.3 + mass + left,
                thrust: 0.0,
                mass,
                drag: 0.3,
                air: 0.0,
                left,
                drag_first: 0.3,
            };
            (
                parts.whole,
                Ok(SplitFlight {
                    parts,
                    own_history: 0.0,
                    apogees: NO_APOGEES,
                }),
            )
        };
        let s = GapSplit::of(&[of(4.0, -3.9), of(2.0, 1.5), of(-1.5, 0.5), of(0.2, -2.0)]);
        assert_eq!(s.parts[1].large, 3);
        assert_eq!(s.mass_opposed_by_rest, 1);
        assert_eq!(s.check_consistent(4), Ok(()));
        let page = s.to_markdown();
        assert!(
            page.contains(
                "Of the 3 flights where the mass part is over 1%, the rest is over 1% with the \
                 other sign on 1,"
            ),
            "{page}"
        );
        let mut more = s.clone();
        more.mass_opposed_by_rest = 4;
        assert!(
            more.check_consistent(4)
                .unwrap_err()
                .contains("opposes a large mass part")
        );
    }

    fn section() -> GapSplit {
        GapSplit::of(&[
            flight([0.1, -0.2, 1.5, 0.4], 0.3, 1e-5),
            flight([-0.3, 0.2, -2.5, 0.2], 0.1, -2.5e-4),
            flight([0.2, 0.1, 0.3, -0.9], -0.5, 3e-6),
            (4.0, Err(NotSplit::refused("a separation".to_owned()))),
            (-1.0, Err(NotSplit::lacks_input("not_flown".to_owned()))),
            (2.0, Err(NotSplit::refused("a release".to_owned()))),
        ])
    }

    #[test]
    fn the_section_counts_and_averages() {
        let s = section();
        assert_eq!((s.flown_by_both, s.split), (6, 3));
        let counts: Vec<usize> = s.not_split.iter().map(|t| t.flights).collect();
        assert_eq!(counts, [1, 2, 0]);
        let dominates: Vec<usize> = s.parts.iter().map(|p| p.dominates).collect();
        assert_eq!(dominates, [0, 0, 2, 1]);
        // Thrust: (0.1 − 0.3 + 0.2) / 3 = 0 and 0.6 / 3 = 0.2.
        assert_eq!(s.parts[0].mean_percent, 0.0);
        assert_eq!(s.parts[0].mean_absolute_percent, 0.2);
        // Drag: (1.5 − 2.5 + 0.3) / 3 = −0.2333 and 4.3 / 3 = 1.4333.
        assert_eq!(s.parts[2].mean_percent, -0.23);
        assert_eq!(s.parts[2].mean_absolute_percent, 1.43);
        // The wholes: 1.8, −2.4 and −0.3 on the three split; with 4, −1 and 2 on all six.
        assert_eq!(s.whole_on_split.mean_percent, -0.3);
        assert_eq!(s.whole_on_split.mean_absolute_percent, 1.5);
        assert_eq!(s.whole_on_both.mean_percent, 0.68);
        assert_eq!(s.whole_on_both.mean_absolute_percent, 1.92);
        assert_eq!(s.air.mean_percent, -0.03);
        assert_eq!(s.left.mean_percent, -0.07);
        assert_eq!(s.drag_first.mean_percent, -0.13);
        assert_eq!(s.largest_own_history_percent, 0.025);
        assert_eq!(s.check_consistent(6), Ok(()));
        let page = s.to_markdown();
        assert!(page.contains("Of the 6 flights both simulators fly, 3 are split."));
        // The medians: drag's of 1.5, −2.5 and 0.3; the whole's of all six, between −0.3 and 1.8.
        assert_eq!(s.parts[2].median_percent, 0.3);
        assert_eq!(s.whole_on_both.median_percent, 0.75);
        let large: Vec<usize> = s.parts.iter().map(|p| p.large).collect();
        assert_eq!(large, [0, 0, 2, 0]);
        assert!(page.contains("| drag | -0.23% | +0.30% | 1.43% | 2 | 2 |"));
        assert!(page.contains("| other: the air | -0.03% | +0.10% | 0.30% |  |  |"));
        assert!(page.contains("the largest miss was 0.025%"));
        for (_, words) in NOT_SPLIT {
            assert!(page.contains(&format!("| {words} |")), "{words}");
        }
    }

    /// No flight split: zeros, not a division by nothing, and under the floor.
    #[test]
    fn no_flight_split_is_zeros_and_fails_the_floor() {
        let s = GapSplit::of(&[(1.0, Err(NotSplit::refused(String::new())))]);
        assert_eq!(s.whole_on_split, Size::default());
        assert!(s.parts.iter().all(|p| p.mean_percent.to_bits() == 0));
        assert!(
            s.check_consistent(1)
                .unwrap_err()
                .contains("fewer than half")
        );
    }

    #[test]
    fn an_inconsistent_section_is_caught() {
        let good = section();
        let caught = |change: &dyn Fn(&mut GapSplit), flown: usize, why: &str| {
            let mut s = good.clone();
            change(&mut s);
            let error = s.check_consistent(flown).unwrap_err();
            assert!(error.contains(why), "{error}");
        };
        caught(&|_| {}, 7, "not the 7 flights both fly");
        caught(&|s| s.split += 1, 6, "not the 6 flights both fly");
        // Counts no file should hold don't overflow: they are inconsistent.
        caught(&|s| s.split = usize::MAX, 6, "not the 6 flights both fly");
        caught(
            &|s| s.not_split[0].flights = usize::MAX,
            6,
            "not the 6 flights both fly",
        );
        caught(
            &|s| s.parts[0].dominates = usize::MAX,
            6,
            "dominated by a part",
        );
        caught(&|s| s.not_split.swap(0, 1), 6, "the reasons are");
        caught(&|s| s.parts.swap(0, 1), 6, "the parts are");
        caught(&|s| s.parts[2].dominates += 1, 6, "dominated by a part");
        caught(&|s| s.parts[0].large = 4, 6, "more than the 3 split");
        caught(&|s| s.left.median_percent = f64::NAN, 6, "a median of");
        caught(&|s| s.parts[2].mean_percent += 0.04, 6, "sum to");
        caught(&|s| s.air.mean_percent -= 0.03, 6, "don't sum to other's");
        caught(&|s| s.parts[1].mean_absolute_percent = 0.0, 6, "a mean of");
        caught(
            &|s| s.largest_own_history_percent = 0.051,
            6,
            "past the bound",
        );
        caught(
            &|s| s.largest_own_history_percent = f64::NAN,
            6,
            "past the bound",
        );
        // Under half: three of seven.
        caught(
            &|s| {
                s.flown_by_both = 7;
                s.not_split[2].flights = 1;
            },
            7,
            "fewer than half",
        );
        // Every flight split, yet the wholes differ.
        let mut all = GapSplit::of(&[
            flight([0.1, -0.2, 1.5, 0.4], 0.3, 1e-5),
            flight([-0.3, 0.2, -2.5, 0.2], 0.1, 1e-5),
        ]);
        assert_eq!(all.check_consistent(2), Ok(()));
        all.whole_on_both.mean_percent += 0.01;
        assert!(
            all.check_consistent(2)
                .unwrap_err()
                .contains("wholes differ")
        );
    }
}
