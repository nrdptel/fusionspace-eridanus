//! The drag part of the gap to OpenRocket, by Mach band (M1.14i1, ADR-359).
//!
//! [`crate::gap_split`] sizes what OpenRocket's whole drag does to HPR Sim's apogee: the drag
//! part. This module says where along the flight's speeds that part comes from. On OpenRocket's
//! thrust, mass and center of mass, HPR Sim flies with OpenRocket's drag coefficient swapped in
//! for its own one band of Mach number after another, each band added to the ones before
//! ([`BandSwap`]):
//!
//! ```text
//! C_D0' = C_D,OR(M) · A_OR / A_ref     where M is in a band swapped
//! C_D0' = C_D0                         elsewhere: HPR Sim's own buildup
//!
//! below 0.8  = (A₂ − A_s)  / A_OR      A₂:  on OpenRocket's thrust and mass (the split's)
//! 0.8 to 1.2 = (A_s − A_st) / A_OR     A_s: and on its drag below Mach 0.8
//! above 1.2  = (A_st − A_b) / A_OR     A_st: and on its drag below Mach 1.2
//! drag       = (A₂ − A₃)  / A_OR       A_b: and on its drag at every Mach number
//! ```
//!
//! `C_D,OR(M)` is the split's table ([`crate::gap_split::drag_table`]): OpenRocket's coefficient
//! along its own flight, power on or power off as HPR Sim's motors burn, on the area of a circle
//! OpenRocket's reference length across, which `A_OR / A_ref` puts on HPR Sim's reference area.
//! `A_b` flies the same numbers as the split's `A₃`, through this module's swap, and the run
//! stops unless the two agree ([`SUM_TOLERANCE_PERCENT`]); so the three bands sum to the drag
//! part.
//!
//! A band that a flight never enters takes no part in it: the flight before its swap and the
//! one after it are the same flight, bit for bit. So a flight has a part in a band exactly when
//! that part is not zero.
//!
//! Each band is also swapped alone, first: on HPR Sim's own thrust and mass, with the other two
//! bands left HPR Sim's ([`Banded::alone`]). That is the flight [`Logs`] sets beside the logged
//! apogee, for each flight compared with a log:
//!
//! ```text
//! error = (A − A_log) / A_log      A: HPR Sim's own flight; on OpenRocket's drag in one band
//!                                     alone; on its whole drag; or OpenRocket's own flight
//! ```
//!
//! At a band's edge the coefficient steps from one program's to the other's. The step is part of
//! what is measured, not a model of anything.
//!
//! The parts are code against code, as the split is: they say where the two programs' drag moves
//! the apogee apart. The rows against the logs say which band's difference the comparison with
//! real flights feels. Neither shows which program's drag is the truer one in a band: both
//! simulators climb above the logs on average, so more drag from any cause moves the mean toward
//! them.

use std::fmt::Write as _;

use hpr_aero::{AeroError, DragModel, DragQuery, DragTable};
use hpr_sim::Simulation;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::gap_split::{Air, LARGE_PERCENT, NotSplit, OpenRocketInputs, Size, SplitFlight};

/// The bands, slowest first, with the report's words for each. They are the Mach numbers
/// M1.14's increments divide the core band by: M1.14c owns Mach 0.8 to 1.2 and M1.14d Mach 1.2
/// to 2.5.
pub(crate) const BANDS: [(&str, &str); 3] = [
    ("subsonic", "below Mach 0.8"),
    ("transonic", "Mach 0.8 to 1.2"),
    ("supersonic", "above Mach 1.2"),
];

/// Where one band ends and the next begins: a Mach number at an edge is in the faster band.
pub(crate) const EDGES: [f64; 2] = [0.8, 1.2];

/// The fewest flights with a part in a band whose sizes are published: an aggregate over one or
/// two flights is a flight's own number (ADR-349 §3's floor, under ADR-151's terms).
pub(crate) const FLOOR: usize = 3;

/// The largest difference, per cent of OpenRocket's apogee, between the flight on every band
/// swapped and the split's flight on OpenRocket's whole drag: the two fly the same coefficient.
const SUM_TOLERANCE_PERCENT: f64 = 1e-9;

/// The band a Mach number is in, an index into [`BANDS`]. One that isn't a number is in none:
/// the buildup refuses it.
pub(crate) fn band(mach: f64) -> Option<usize> {
    if mach.is_nan() {
        return None;
    }
    Some(EDGES.iter().filter(|edge| mach >= **edge).count())
}

/// HPR Sim's drag with OpenRocket's coefficient in place of its own wherever the Mach number is
/// in a band of `openrockets`. See the module's equations.
#[derive(Debug, Clone)]
pub(crate) struct BandSwap {
    table: DragTable,
    /// Which of [`BANDS`] read OpenRocket's table.
    openrockets: [bool; 3],
}

impl BandSwap {
    pub(crate) fn new(table: DragTable, openrockets: [bool; 3]) -> Self {
        Self { table, openrockets }
    }
}

impl DragModel for BandSwap {
    fn zero_lift_drag(&self, query: &DragQuery<'_>) -> Result<f64, AeroError> {
        let mach = query.mach();
        if !band(mach).is_some_and(|band| self.openrockets[band]) {
            // HPR Sim's own drag, bit for bit.
            return Ok(query.buildup()?.zero_lift_coefficient);
        }
        // As a flight on the table alone reads it (`hpr_aero::AeroModel::drag`), so the flight
        // on every band swapped is that flight.
        let lookup = self.table.lookup(mach, query.conditions().thrusting)?;
        if lookup.value < 0.0 {
            return Err(AeroError::Domain {
                what: "zero-lift drag coefficient from a drag table",
                value: lookup.value,
            });
        }
        let scale = match self.table.reference_diameter_m {
            Some(d) => 0.25 * std::f64::consts::PI * d * d / query.reference_area_m2(),
            None => 1.0,
        };
        Ok(lookup.value * scale)
    }
}

/// One flight's drag part by band, each per cent of OpenRocket's apogee, with the heights the
/// rows against the logs are taken from.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Banded {
    /// The bands' parts, each swapped after the slower ones, in [`BANDS`]' order.
    pub(crate) parts: [f64; 3],
    /// Each band swapped alone, first: on HPR Sim's own thrust and mass.
    pub(crate) alone: [f64; 3],
    /// The split's drag part, which `parts` sum to.
    pub(crate) drag: f64,
    /// The split's drag part with the whole drag swapped first.
    pub(crate) drag_first: f64,
    /// The heights climbed, m: HPR Sim's own flight, on each band alone, on OpenRocket's whole
    /// drag swapped first, and OpenRocket's own flight.
    pub(crate) heights_m: Heights,
}

/// The heights one flight's rows against its log are taken from, m.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Heights {
    pub(crate) own_m: f64,
    pub(crate) alone_m: [f64; 3],
    pub(crate) drag_first_m: f64,
    pub(crate) openrocket_m: f64,
}

impl Heights {
    /// The six heights in [`LOG_ROWS`]' order.
    fn rows(&self) -> [f64; 6] {
        let [slow, mid, fast] = self.alone_m;
        [
            self.own_m,
            slow,
            mid,
            fast,
            self.drag_first_m,
            self.openrocket_m,
        ]
    }
}

impl Banded {
    /// The band that dominates the flight's drag part: the one whose absolute value is the
    /// largest, the slower of two that tie.
    pub(crate) fn dominant(&self) -> usize {
        let mut dominant = 0;
        for (i, part) in self.parts.iter().enumerate() {
            if part.abs() > self.parts[dominant].abs() {
                dominant = i;
            }
        }
        dominant
    }
}

/// One flight split, as this module counts it: split by band, or why HPR Sim failed a flight
/// with a band swapped (kept on this machine); and its logged apogee as a height climbed, m,
/// when the report compares the flight with its log.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BandFlight {
    pub(crate) banded: Result<Banded, String>,
    pub(crate) logged_m: Option<f64>,
}

/// One flight both simulators fly, as this module counts it: `None` when it is not split;
/// `Some(Err(_))` when HPR Sim fails a flight with a band swapped, with why. `build` and
/// `climbed_m` are [`crate::gap_split::split`]'s; `histories` is OpenRocket's record of the
/// flight's inputs, `split` the flight's split, and `openrocket_m` OpenRocket's apogee.
///
/// # Errors
///
/// A split flight without its histories, or the flight on every band swapped missing the
/// split's flight on OpenRocket's whole drag by more than [`SUM_TOLERANCE_PERCENT`].
pub(crate) fn flight(
    build: &dyn Fn(Air) -> Result<Simulation, String>,
    climbed_m: &dyn Fn(&Simulation) -> Result<f64, String>,
    histories: &Value,
    split: &Result<SplitFlight, NotSplit>,
    openrocket_m: f64,
) -> Result<Option<Result<Banded, String>>, String> {
    let Ok(split) = split else {
        return Ok(None);
    };
    // The split flew these histories, so they are there.
    let Ok(inputs) = OpenRocketInputs::of(histories)? else {
        return Err("a flight that is split has no histories".to_owned());
    };
    let swapped = |openrockets: [bool; 3], histories: bool| -> Result<f64, String> {
        let mut simulation = build(Air::Day)?;
        if histories {
            simulation = simulation
                .with_thrust_history(inputs.thrust.clone())
                .map_err(|error| error.to_string())?
                .with_mass_history(inputs.mass.clone())
                .map_err(|error| error.to_string())?;
        }
        climbed_m(&simulation.with_drag_model(BandSwap::new(inputs.drag.clone(), openrockets)))
    };
    let flown = || -> Result<[f64; 6], String> {
        Ok([
            swapped([true, false, false], true)?,
            swapped([true, true, false], true)?,
            swapped([true, true, true], true)?,
            swapped([true, false, false], false)?,
            swapped([false, true, false], false)?,
            swapped([false, false, true], false)?,
        ])
    };
    let [slow_m, slow_mid_m, every_m, alone_m @ ..] = match flown() {
        Ok(flown) => flown,
        Err(error) => return Ok(Some(Err(error))),
    };
    let apogees = split.apogees;
    let share = |before_m: f64, after_m: f64| 100.0 * (before_m - after_m) / openrocket_m;
    // A difference that isn't a number fails too.
    let same = share(every_m, apogees.drag_m).abs() <= SUM_TOLERANCE_PERCENT;
    if !same {
        return Err(format!(
            "on every band swapped HPR Sim climbs {every_m} m, and on OpenRocket's whole drag \
             {} m: the bands don't sum to the drag part",
            apogees.drag_m
        ));
    }
    Ok(Some(Ok(Banded {
        parts: [
            share(apogees.mass_m, slow_m),
            share(slow_m, slow_mid_m),
            share(slow_mid_m, every_m),
        ],
        alone: alone_m.map(|alone_m| share(apogees.own_m, alone_m)),
        drag: split.parts.drag,
        drag_first: split.parts.drag_first,
        heights_m: Heights {
            own_m: apogees.own_m,
            alone_m,
            drag_first_m: apogees.drag_first_m,
            openrocket_m,
        },
    })))
}

/// One band over the flights split by band. The sizes are there when the section is published
/// ([`DragBands::published`]).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct Band {
    /// Its name, from [`BANDS`].
    pub name: String,
    /// The flights with a part in it: those whose part is not zero.
    pub flights: usize,
    /// Its part over every flight split by band.
    pub part: Option<Size>,
    /// Its part over the flights with a part in it.
    pub part_on_its_flights: Option<Size>,
    /// The band swapped alone, first, over every flight split by band.
    pub alone: Option<Size>,
    /// The flights it dominates: where its part's absolute value is the largest of the three.
    pub dominates: Option<usize>,
    /// The flights where its part's absolute value is over [`LARGE_PERCENT`].
    pub large: Option<usize>,
}

/// The rows against the logs, in order, with the report's words for the drag each flies.
pub(crate) const LOG_ROWS: [(&str, &str); 6] = [
    ("own", "its own"),
    ("subsonic", "OpenRocket's below Mach 0.8, its own above"),
    (
        "transonic",
        "OpenRocket's from Mach 0.8 to 1.2, its own elsewhere",
    ),
    ("supersonic", "OpenRocket's above Mach 1.2, its own below"),
    ("every", "OpenRocket's at every Mach number"),
    ("openrocket", "OpenRocket's own flight, for comparison"),
];

/// One row against the logs: the apogee less the logged one, over the logged one, per cent.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct LogRow {
    /// Its name, from [`LOG_ROWS`].
    pub name: String,
    /// The flights whose apogee is not HPR Sim's own.
    pub moved: usize,
    /// Of those, the flights nearer their log than HPR Sim's own flight, and the ones farther.
    pub nearer: Option<usize>,
    pub farther: Option<usize>,
    pub error: Option<Size>,
}

/// The flights compared with a log that are split by band, and their rows.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct Logs {
    pub flights: usize,
    /// One per [`LOG_ROWS`], in its order.
    pub rows: Vec<LogRow>,
}

/// The report's section: the drag part by Mach band, as aggregates.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct DragBands {
    /// The flights split.
    pub split: usize,
    /// Those split by band.
    pub banded: usize,
    /// The rest: HPR Sim failed a flight with a band swapped.
    pub failed: usize,
    /// One per band, in [`BANDS`]' order.
    pub bands: Vec<Band>,
    /// The drag part over the flights split by band, which the bands' parts sum to.
    pub drag: Option<Size>,
    /// The same with the whole drag swapped first, which the bands alone are set beside.
    pub drag_first: Option<Size>,
    pub logs: Logs,
}

/// Whether `count` flights are none, or enough for an aggregate over them to be given.
fn none_or_floor(count: usize) -> bool {
    count == 0 || count >= FLOOR
}

impl DragBands {
    /// The section's numbers from each flight split. The sizes are given only when every
    /// aggregate is over [`FLOOR`] flights or more (see [`DragBands::published`]).
    pub(crate) fn of(flights: &[BandFlight]) -> Self {
        let banded: Vec<(&Banded, Option<f64>)> = flights
            .iter()
            .filter_map(|flight| Some((flight.banded.as_ref().ok()?, flight.logged_m)))
            .collect();
        let entered = |i: usize| banded.iter().filter(|(b, _)| b.parts[i] != 0.0).count();
        // Against the logs: each flight's six errors, per cent of its logged apogee.
        let logged: Vec<[f64; 6]> = banded
            .iter()
            .filter_map(|(b, logged_m)| {
                let logged_m = (*logged_m)?;
                Some(
                    b.heights_m
                        .rows()
                        .map(|m| 100.0 * (m - logged_m) / logged_m),
                )
            })
            .collect();
        let moved = |row: usize| logged.iter().filter(|e| e[row] != e[0]).count();
        let published = banded.len() >= FLOOR
            && (0..BANDS.len()).all(|i| none_or_floor(entered(i)))
            && none_or_floor(logged.len())
            && (0..LOG_ROWS.len()).all(|row| none_or_floor(moved(row)));
        let size = |of: &dyn Fn(&Banded) -> f64| {
            published.then(|| Size::of(banded.iter().map(|(b, _)| of(b))))
        };
        let bands = BANDS
            .iter()
            .enumerate()
            .map(|(i, (name, _))| Band {
                name: (*name).to_owned(),
                flights: entered(i),
                part: size(&|b| b.parts[i]),
                part_on_its_flights: published.then(|| {
                    Size::of(
                        banded
                            .iter()
                            .map(move |(b, _)| b.parts[i])
                            .filter(|part| *part != 0.0),
                    )
                }),
                alone: size(&|b| b.alone[i]),
                dominates: published
                    .then(|| banded.iter().filter(|(b, _)| b.dominant() == i).count()),
                large: published.then(|| {
                    banded
                        .iter()
                        .filter(|(b, _)| b.parts[i].abs() > LARGE_PERCENT)
                        .count()
                }),
            })
            .collect();
        let rows = LOG_ROWS
            .iter()
            .enumerate()
            .map(|(row, (name, _))| LogRow {
                name: (*name).to_owned(),
                moved: moved(row),
                nearer: published
                    .then(|| logged.iter().filter(|e| e[row].abs() < e[0].abs()).count()),
                farther: published
                    .then(|| logged.iter().filter(|e| e[row].abs() > e[0].abs()).count()),
                error: published.then(|| Size::of(logged.iter().map(move |e| e[row]))),
            })
            .collect();
        Self {
            split: flights.len(),
            banded: banded.len(),
            failed: flights.len() - banded.len(),
            bands,
            drag: size(&|b| b.drag),
            drag_first: size(&|b| b.drag_first),
            logs: Logs {
                flights: logged.len(),
                rows,
            },
        }
    }

    /// Whether the sizes are given: at least [`FLOOR`] flights are split by band, and each
    /// band's flights, the flights compared with a log, and the flights each row against the
    /// logs moves are none or at least [`FLOOR`]. It is all or nothing, since the bands sum to
    /// the drag part: one band held back could be read off the others (ADR-151: aggregates only).
    pub(crate) fn published(&self) -> bool {
        self.banded >= FLOOR
            && self.bands.iter().all(|band| none_or_floor(band.flights))
            && none_or_floor(self.logs.flights)
            && self.logs.rows.iter().all(|row| none_or_floor(row.moved))
    }

    /// Holds the section to itself and to the report: `split` flights split and `compared`
    /// compared with a log, whose errors over every flight compared are `hpr`'s and
    /// `openrocket`'s `(mean, median, mean absolute)`.
    pub(crate) fn check_consistent(
        &self,
        split: usize,
        compared: usize,
        (hpr, openrocket): ([f64; 3], [f64; 3]),
    ) -> Result<(), String> {
        let fail = |why: String| Err(format!("the drag by Mach band: {why}"));
        let names: Vec<&str> = self.bands.iter().map(|b| b.name.as_str()).collect();
        let expected: Vec<&str> = BANDS.iter().map(|(name, _)| *name).collect();
        if names != expected {
            return fail(format!("the bands are {names:?}, not {expected:?}"));
        }
        let rows: Vec<&str> = self.logs.rows.iter().map(|r| r.name.as_str()).collect();
        let expected: Vec<&str> = LOG_ROWS.iter().map(|(name, _)| *name).collect();
        if rows != expected {
            return fail(format!(
                "the rows against the logs are {rows:?}, not {expected:?}"
            ));
        }
        // Saturating: a count read from a file can be any number.
        if self.split != split || self.banded.saturating_add(self.failed) != split {
            return fail(format!(
                "{} split by band and {} failed are not the {split} flights split",
                self.banded, self.failed
            ));
        }
        // M1.14i1's floor (ADR-359): the measurement covers at least half the flights split.
        if self.banded.saturating_mul(2) < split {
            return fail(format!(
                "{} of the {split} flights split are split by band, fewer than half",
                self.banded
            ));
        }
        if let Some(band) = self.bands.iter().find(|band| band.flights > self.banded) {
            return fail(format!(
                "{} flights have a part in the {} band, more than the {} split by band",
                band.flights, band.name, self.banded
            ));
        }
        let logs = &self.logs;
        if logs.flights > compared || logs.flights > self.banded {
            return fail(format!(
                "{} flights against the logs, of {compared} compared and {} split by band",
                logs.flights, self.banded
            ));
        }
        if let Some(row) = logs.rows.iter().find(|row| row.moved > logs.flights) {
            return fail(format!(
                "the {} row moves {} flights of {}",
                row.name, row.moved, logs.flights
            ));
        }
        if logs.rows[0].moved != 0 {
            return fail("HPR Sim's own row moves a flight".to_owned());
        }
        let published = self.published();
        let given = [self.drag.is_some(), self.drag_first.is_some()]
            .into_iter()
            .chain(self.bands.iter().flat_map(|band| {
                [
                    band.part.is_some(),
                    band.part_on_its_flights.is_some(),
                    band.alone.is_some(),
                    band.dominates.is_some(),
                    band.large.is_some(),
                ]
            }))
            .chain(logs.rows.iter().flat_map(|row| {
                [
                    row.nearer.is_some(),
                    row.farther.is_some(),
                    row.error.is_some(),
                ]
            }));
        if given.into_iter().any(|given| given != published) {
            return fail(format!(
                "a size is {}: they are given when every aggregate is over none or {FLOOR} \
                 flights or more",
                if published { "missing" } else { "given" }
            ));
        }
        let (Some(drag), Some(drag_first)) = (self.drag, self.drag_first) else {
            return Ok(());
        };
        drag.consistent("drag over the flights split by band")?;
        drag_first.consistent("drag swapped first over the flights split by band")?;
        let mut dominated = 0_usize;
        let mut sum = 0.0;
        for (i, band) in self.bands.iter().enumerate() {
            let (Some(part), Some(own), Some(alone), Some(dominates), Some(large)) = (
                band.part,
                band.part_on_its_flights,
                band.alone,
                band.dominates,
                band.large,
            ) else {
                return fail(format!("the {} band lacks a size", band.name));
            };
            part.consistent(&format!("{} band", band.name))?;
            own.consistent(&format!("{} band on its flights", band.name))?;
            alone.consistent(&format!("{} band alone", band.name))?;
            // A part over 1% is not zero; a faster band dominates only where its part is
            // larger than the slowest band's, so not zero either. The slowest band takes the
            // flights whose parts are all zero.
            if large > band.flights || (i > 0 && dominates > band.flights) {
                return fail(format!(
                    "the {} band dominates {dominates} flights and is large on {large}, with a \
                     part in {}",
                    band.name, band.flights
                ));
            }
            // The same sum of absolute values over fewer flights, to the rounding of each.
            if own.mean_absolute_percent + 0.01 + 1e-9 < part.mean_absolute_percent {
                return fail(format!(
                    "the {} band's mean absolute value is {} over its own flights, below {} over \
                     every flight",
                    band.name, own.mean_absolute_percent, part.mean_absolute_percent
                ));
            }
            dominated = dominated.saturating_add(dominates);
            sum += part.mean_percent;
        }
        if dominated != self.banded {
            return fail(format!(
                "{dominated} flights are dominated by a band, not the {} split by band",
                self.banded
            ));
        }
        // Each mean is rounded to 0.005: three of them, then the drag's.
        if (sum - drag.mean_percent).abs() > 0.02 + 1e-9 {
            return fail(format!(
                "the bands' means sum to {sum}, not the drag part's {}",
                drag.mean_percent
            ));
        }
        for row in &logs.rows {
            let (Some(nearer), Some(farther), Some(error)) = (row.nearer, row.farther, row.error)
            else {
                return fail(format!("the {} row lacks a size", row.name));
            };
            error.consistent(&format!("{} against the logs", row.name))?;
            // A flight nearer or farther has moved.
            if nearer.saturating_add(farther) > row.moved {
                return fail(format!(
                    "the {} row has {nearer} flights nearer and {farther} farther, of {} moved",
                    row.name, row.moved
                ));
            }
        }
        // Over every flight compared, the first and last rows are the report's own.
        if compared > 0 && logs.flights == compared {
            for (row, theirs) in [(0, hpr), (LOG_ROWS.len() - 1, openrocket)] {
                let ours = logs.rows[row].error.map(|size| {
                    [
                        size.mean_percent,
                        size.median_percent,
                        size.mean_absolute_percent,
                    ]
                });
                if ours != Some(theirs) {
                    return fail(format!(
                        "the {} row is {ours:?} over every flight compared, where the errors' \
                         table has {theirs:?}",
                        logs.rows[row].name
                    ));
                }
            }
        }
        Ok(())
    }

    /// The section, for the end of the split's.
    pub(crate) fn to_markdown(&self) -> String {
        let mut page = String::new();
        let flights = |n: usize| if n == 1 { "flight" } else { "flights" };
        let _ = writeln!(
            page,
            "\n### The drag part by Mach band\n\nFor M1.14i1 (ADR-359), the drag part is split \
             by the Mach number the drag acts at. On OpenRocket's thrust and mass, HPR Sim \
             flies with OpenRocket's drag coefficient in place of its own below Mach {} only, \
             then below Mach {} too, then at every Mach number. Outside the bands swapped the \
             drag is HPR Sim's own, so at a band's edge the coefficient steps from one \
             program's to the other's. A band's part is the apogee before its swap less the \
             apogee after it. The three sum to the drag part: the last of these flights is the \
             split's flight on OpenRocket's whole drag, and the run stops on a flight where it \
             isn't. A flight that never reaches a band is the same flight before and after \
             that band's swap, so a flight has a part in a band when its part there is not \
             zero. Like the split, the parts are code against code.\n",
            EDGES[0], EDGES[1]
        );
        let _ = writeln!(
            page,
            "Of the {} {} split, {} {} split by band; on {} HPR Sim failed a flight with a band \
             swapped.\n",
            self.split,
            flights(self.split),
            self.banded,
            if self.banded == 1 { "is" } else { "are" },
            self.failed
        );
        let (Some(drag), Some(drag_first)) = (self.drag, self.drag_first) else {
            let _ = writeln!(page, "| band | flights with a part in it |\n|---|---:|");
            for (band, (_, words)) in self.bands.iter().zip(BANDS) {
                let _ = writeln!(page, "| {words} | {} |", band.flights);
            }
            let _ = writeln!(
                page,
                "\nThe bands' sizes are not given: an aggregate over one or two flights is a \
                 flight's own number, and the flights are private. The bands sum to the drag \
                 part, so one held back could be read off the others, and none is given."
            );
            return page;
        };
        let _ = writeln!(
            page,
            "| band | flights with a part in it | mean | median | mean absolute | mean absolute \
             over those flights | flights it dominates | flights where it is over \
             {LARGE_PERCENT}% |\n|---|---:|---:|---:|---:|---:|---:|---:|"
        );
        let sizes = |size: Size| {
            format!(
                "{:+.2}% | {:+.2}% | {:.2}%",
                size.mean_percent, size.median_percent, size.mean_absolute_percent
            )
        };
        for (band, (_, words)) in self.bands.iter().zip(BANDS) {
            let (Some(part), Some(own), Some(dominates), Some(large)) = (
                band.part,
                band.part_on_its_flights,
                band.dominates,
                band.large,
            ) else {
                continue;
            };
            let _ = writeln!(
                page,
                "| {words} | {} | {} | {:.2}% | {dominates} | {large} |",
                band.flights,
                sizes(part),
                own.mean_absolute_percent
            );
        }
        let _ = writeln!(
            page,
            "| the drag part | {} | {} |  |  |  |",
            self.banded,
            sizes(drag)
        );
        let _ = writeln!(
            page,
            "\nEach percentage is a share of OpenRocket's apogee, over the {} {} split by band, \
             but for the column over the flights with a part in the band. A band dominates a \
             flight when its part's absolute value is the largest of the three, the slower band \
             if two tie. The last column counts the flights where that absolute value is over \
             {LARGE_PERCENT}%.\n",
            self.banded,
            flights(self.banded)
        );
        let _ = writeln!(
            page,
            "A band's part depends on the speed the slower bands' swaps leave the flight with. \
             So each band is also swapped alone, first: on HPR Sim's own thrust and mass, with \
             its own drag in the other two bands.\n"
        );
        let _ = writeln!(
            page,
            "| band, swapped alone | mean | median | mean absolute |\n|---|---:|---:|---:|"
        );
        for (band, (_, words)) in self.bands.iter().zip(BANDS) {
            if let Some(alone) = band.alone {
                let _ = writeln!(page, "| {words} | {} |", sizes(alone));
            }
        }
        let _ = writeln!(
            page,
            "| every band: the drag part, swapped first | {} |",
            sizes(drag_first)
        );
        let logs = &self.logs;
        let _ = writeln!(
            page,
            "\nThe flights with a band swapped alone are also set beside the logs. Of the \
             flights compared with a log above, {} {} split by band. Each row is the apogee \
             less the logged one, over the logged one, on those flights: HPR Sim's own \
             flight; HPR Sim on OpenRocket's drag in one band alone; HPR Sim on OpenRocket's \
             whole drag; and OpenRocket's own flight. A row moves a flight when its apogee \
             is not that of HPR Sim's own flight, and the last two columns count the moved \
             flights that come nearer their log than HPR Sim's own flight, and the ones that \
             go farther.\n",
            logs.flights,
            if logs.flights == 1 { "is" } else { "are" },
        );
        let _ = writeln!(
            page,
            "| HPR Sim's drag | flights moved | mean | median | mean absolute | flights nearer \
             their log | flights farther from it |\n|---|---:|---:|---:|---:|---:|---:|"
        );
        for (i, (row, (_, words))) in logs.rows.iter().zip(LOG_ROWS).enumerate() {
            let (Some(nearer), Some(farther), Some(error)) = (row.nearer, row.farther, row.error)
            else {
                continue;
            };
            if i == 0 {
                let _ = writeln!(page, "| {words} |  | {} |  |  |", sizes(error));
            } else {
                let _ = writeln!(
                    page,
                    "| {words} | {} | {} | {nearer} | {farther} |",
                    row.moved,
                    sizes(error)
                );
            }
        }
        let _ = writeln!(
            page,
            "\nA row nearer the logs doesn't show that OpenRocket's drag is the truer one in \
             that band. Both simulators climb above the logs on average, so more drag from any \
             cause moves the mean toward them, and a wrong mass or thrust would read the same \
             way. The rows aren't a model HPR Sim could fly either: the coefficient is read \
             along OpenRocket's own flight of the same design. They say which band's \
             difference the comparison with the logs feels."
        );
        page
    }
}

#[cfg(test)]
mod tests;
