//! The drag part below Mach 0.8, by term (M1.14i2, ADR-359).
//!
//! [`crate::drag_bands`] puts the largest band of the drag part of the gap to OpenRocket below
//! Mach 0.8. This module says which term of the drag that band's part comes from. OpenRocket's
//! flight data splits its drag coefficient into friction, pressure and base
//! (`histories.py`'s last three columns); the rest is its coefficient less the three. On
//! OpenRocket's thrust, mass and center of mass, HPR Sim flies with OpenRocket's terms swapped in
//! for its own below Mach 0.8, one term after another, each added to the ones before
//! ([`TermSwap`]):
//!
//! ```text
//! C_D0' = C_D0 − Σ_swapped c_t + (A_OR / A_ref) Σ_swapped C_t,OR(M)   where M < 0.8
//! C_D0' = C_D0                                                         elsewhere
//!
//! friction = (A₂ − A_f)    / A_OR     A₂:   on OpenRocket's thrust and mass (the split's)
//! pressure = (A_f − A_fp)  / A_OR     A_f:  and on its friction below Mach 0.8
//! base     = (A_fp − A_fpb) / A_OR    A_fp: and on its pressure there too
//! rest     = (A_fpb − A_4) / A_OR     A_fpb: and on its base; A_4: and on its rest
//! ```
//!
//! `c_t` is HPR Sim's own term from its buildup ([`hpr_aero::Drag`]) and `C_t,OR(M)` OpenRocket's,
//! read as the split reads its whole coefficient ([`crate::gap_split::drag_table`]): power on or
//! power off as HPR Sim's motors burn, linear in Mach number between OpenRocket's rows, on the
//! area of a circle OpenRocket's reference length across. The four tables are taken on the same
//! rows, so they sum to the whole table at every Mach number, to rounding.
//!
//! **The two codes' terms, set against each other** ([`hpr_terms`]). OpenRocket books a launch
//! lug and rail buttons under pressure, and a stated drag coefficient under none of its three
//! terms, so in the rest; `drag_terms.py` measures that on a public design
//! (`validation/fixtures/ork/openrocket-drag-terms.json`), and a test holds the record to it.
//! So HPR Sim's four are its friction; its pressure with its parasitic drag (launch lugs and
//! rail buttons); its base; and its stated coefficients. Each code's four sum to its whole
//! coefficient, power on and power off.
//!
//! `A_4`, every term swapped, flies OpenRocket's whole coefficient below Mach 0.8 as the band
//! split's first flight does, bit for bit ([`TermSwap`]'s order). The run stops where the four
//! parts don't sum to that band's part ([`SUM_TOLERANCE_PERCENT`]).
//!
//! Each term is also swapped alone, first: on HPR Sim's own thrust and mass, with HPR Sim's own
//! drag in the other terms and above Mach 0.8 ([`Termed::alone`]). That is the flight set beside
//! the logged apogee, as the bands do.
//!
//! The parts are code against code: they say which term's difference moves the two programs'
//! apogees apart below Mach 0.8, not which program's term is the truer one.

use std::fmt::Write as _;

use hpr_aero::{AeroError, Drag, DragModel, DragQuery, DragTable};
use hpr_sim::Simulation;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::drag_bands::{Banded, FLOOR, LOG_ROWS, LogRow, Logs, band};
use crate::gap_split::{Air, LARGE_PERCENT, NotSplit, OpenRocketInputs, Size, SplitFlight};

/// The terms, in the order they are swapped, with the report's words for each.
pub(crate) const TERMS: [(&str, &str); 4] = [
    ("friction", "friction"),
    (
        "pressure",
        "pressure, launch lugs and rail buttons among it",
    ),
    ("base", "base"),
    ("rest", "the rest: stated drag coefficients"),
];

/// The columns of a record row that hold OpenRocket's three terms, after
/// [`crate::gap_split::COLUMNS`]' first seven.
const TERM_COLUMNS: [usize; 3] = [7, 8, 9];
const DRAG: usize = 4;
const MACH: usize = 5;
const THRUST: usize = 1;

/// How far from zero OpenRocket's rest may be and still be the rounding of its three terms'
/// sum, not a coefficient: three additions of numbers near 1 round by a few parts in 10^16,
/// and the smallest coefficient a design states is far above this.
const REST_ROUNDING: f64 = 1e-12;

/// The largest difference, per cent of OpenRocket's apogee, between the flight on every term
/// swapped and the band split's flight on OpenRocket's whole drag below Mach 0.8: the two are
/// the same flight ([`TermSwap`]), so the parts' sum differs from the band's part only by the
/// rounding of a few subtractions.
const SUM_TOLERANCE_PERCENT: f64 = 1e-9;

/// Why a flight split is not split by term, in the order they are tried, with the report's
/// words for each.
pub(crate) const NOT_TERMED: [(&str, &str); 3] = [
    (
        "not_banded",
        "HPR Sim failed a flight with a band swapped, so the flight has no part below Mach 0.8 \
         to split",
    ),
    (
        "terms_lacking",
        "OpenRocket's record lacks one of its three terms at a row where it gives the drag \
         coefficient",
    ),
    ("failed", "HPR Sim fails a flight with a term swapped"),
];

/// HPR Sim's four terms of `drag`, set against OpenRocket's: friction; pressure with the
/// parasitic drag of launch lugs and rail buttons; base; and stated coefficients. They sum to
/// `drag.zero_lift_coefficient` as the buildup sums its five parts, to rounding.
pub(crate) fn hpr_terms(drag: &Drag) -> [f64; 4] {
    [
        drag.friction,
        drag.pressure + drag.parasitic,
        drag.base,
        drag.stated,
    ]
}

/// OpenRocket's four terms along its flight as tables HPR Sim can fly, on the rows the split
/// takes its whole coefficient from ([`crate::gap_split::drag_table`]): friction, pressure,
/// base, and the rest, the coefficient less the three, taken as zero within
/// [`REST_ROUNDING`]. `Ok(Err(_))` when a term is not a number at a row where the coefficient
/// is.
///
/// # Errors
///
/// A record that is not `histories.py`'s: no rows, a cell that isn't a number or null, no
/// reference length, or rows that give no curve.
pub(crate) fn term_tables(flight: &Value) -> Result<Result<[DragTable; 4], String>, String> {
    let rows = flight["rows"]
        .as_array()
        .ok_or("a flight split has no rows")?;
    let reference = flight["reference_length_m"]
        .as_f64()
        .ok_or("a flight split has no reference length")?;
    let cell = |row: &Value, column: usize| -> Result<Option<f64>, String> {
        match &row[column] {
            Value::Null => Ok(None),
            value => value
                .as_f64()
                .filter(|x| x.is_finite())
                .map(Some)
                .ok_or_else(|| {
                    format!(
                        "a row's {} is not a number",
                        crate::gap_split::COLUMNS[column]
                    )
                }),
        }
    };
    // Per term, each row's thrust, Mach number and value.
    let mut terms: [Vec<Cell>; 4] = Default::default();
    for row in rows {
        let thrust = cell(row, THRUST)?.ok_or("a flight split has no thrust at a row")?;
        let (drag, mach) = (cell(row, DRAG)?, cell(row, MACH)?);
        let values = match drag {
            None => [None; 4],
            Some(drag) => {
                let mut three = [0.0; 3];
                for (value, column) in three.iter_mut().zip(TERM_COLUMNS) {
                    let Some(term) = cell(row, column)? else {
                        return Ok(Err(format!(
                            "OpenRocket's {} is not a number at a row where its drag \
                             coefficient is",
                            crate::gap_split::COLUMNS[column]
                        )));
                    };
                    *value = term;
                }
                let [friction, pressure, base] = three;
                let rest = drag - friction - pressure - base;
                let rest = if rest.abs() <= REST_ROUNDING {
                    0.0
                } else {
                    rest
                };
                [Some(friction), Some(pressure), Some(base), Some(rest)]
            }
        };
        for (term, value) in terms.iter_mut().zip(values) {
            term.push((thrust, value, mach));
        }
    }
    let mut tables = Vec::with_capacity(4);
    for term in &terms {
        tables.push(crate::gap_split::drag_table(term, reference)?);
    }
    tables
        .try_into()
        .map(Ok)
        .map_err(|_| "four terms make four tables".to_owned())
}

/// HPR Sim's drag with OpenRocket's terms in place of its own below Mach 0.8 for each term
/// `openrockets` marks. See the module's equations.
///
/// With friction, pressure and base all swapped, the coefficient starts from OpenRocket's whole
/// one, read as [`crate::drag_bands::BandSwap`] reads it, and takes OpenRocket's rest back out
/// when the rest is not swapped; otherwise it starts from HPR Sim's own. The two are the same
/// number to rounding, but an adaptive integrator can turn a difference in the last bit into a
/// visible one in the apogee, so this order makes the flight on every term swapped the band
/// swap's flight bit for bit. And a term both codes give as zero leaves the coefficient as it
/// was, bit for bit (x − 0 + 0 = x): on a flight with no stated coefficient the rest's part is
/// exactly zero.
#[derive(Debug, Clone)]
pub(crate) struct TermSwap {
    /// OpenRocket's whole coefficient, the split's table.
    whole: DragTable,
    tables: [DragTable; 4],
    /// Which of [`TERMS`] read OpenRocket's table.
    openrockets: [bool; 4],
    /// Whether the swap holds only while a motor burns: coasting, the drag is HPR Sim's own.
    powered_only: bool,
}

impl TermSwap {
    /// A swap of `tables`, which sum to `whole`, for the terms of [`TERMS`] that `openrockets`
    /// marks.
    pub(crate) fn new(whole: DragTable, tables: [DragTable; 4], openrockets: [bool; 4]) -> Self {
        Self {
            whole,
            tables,
            openrockets,
            powered_only: false,
        }
    }

    /// The same swap, held only while a motor burns.
    pub(crate) fn powered_only(mut self) -> Self {
        self.powered_only = true;
        self
    }
}

impl DragModel for TermSwap {
    fn zero_lift_drag(&self, query: &DragQuery<'_>) -> Result<f64, AeroError> {
        let own = query.buildup()?;
        let thrusting = query.conditions().thrusting;
        if band(query.mach()) != Some(0)
            || !self.openrockets.contains(&true)
            || (self.powered_only && !thrusting)
        {
            // HPR Sim's own drag, bit for bit.
            return Ok(own.zero_lift_coefficient);
        }
        let read = |table: &DragTable| -> Result<f64, AeroError> {
            let lookup = table.lookup(query.mach(), thrusting)?;
            if lookup.value < 0.0 && std::ptr::eq(table, &self.whole) {
                return Err(AeroError::Domain {
                    what: "zero-lift drag coefficient from a drag table",
                    value: lookup.value,
                });
            }
            let scale = match table.reference_diameter_m {
                Some(d) => 0.25 * std::f64::consts::PI * d * d / query.reference_area_m2(),
                None => 1.0,
            };
            Ok(lookup.value * scale)
        };
        let terms = hpr_terms(&own);
        let coefficient = if self.openrockets[..3] == [true; 3] {
            let whole = read(&self.whole)?;
            if self.openrockets[3] {
                whole
            } else {
                whole - read(&self.tables[3])? + terms[3]
            }
        } else {
            let mut coefficient = own.zero_lift_coefficient;
            for ((table, swapped), term) in self.tables.iter().zip(self.openrockets).zip(terms) {
                if swapped {
                    coefficient = coefficient - term + read(table)?;
                }
            }
            coefficient
        };
        if coefficient < 0.0 || !coefficient.is_finite() {
            return Err(AeroError::Domain {
                what: "zero-lift drag coefficient with OpenRocket's terms swapped in",
                value: coefficient,
            });
        }
        Ok(coefficient)
    }
}

/// One flight's part below Mach 0.8 by term, each per cent of OpenRocket's apogee, with the
/// heights the rows against the logs are taken from.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Termed {
    /// The terms' parts, each swapped after the earlier ones, in [`TERMS`]' order.
    pub(crate) parts: [f64; 4],
    /// Each term swapped alone, first: on HPR Sim's own thrust and mass.
    pub(crate) alone: [f64; 4],
    /// The base swapped alone as `alone`'s, but only while a motor burns.
    pub(crate) base_powered_alone: f64,
    /// The band's part below Mach 0.8, which `parts` sum to.
    pub(crate) band: f64,
    /// The band swapped alone, first, which `alone` is set beside.
    pub(crate) band_alone: f64,
    /// The heights climbed, m: HPR Sim's own flight, on each term alone, and on the whole band
    /// alone.
    pub(crate) heights_m: TermHeights,
}

/// The heights one flight's rows against its log are taken from, m.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct TermHeights {
    /// HPR Sim's own flight.
    pub(crate) own_m: f64,
    /// HPR Sim on OpenRocket's term below Mach 0.8 alone, in [`TERMS`]' order.
    pub(crate) alone_m: [f64; 4],
    /// HPR Sim on OpenRocket's whole drag below Mach 0.8.
    pub(crate) band_m: f64,
}

impl TermHeights {
    /// The six heights in [`TERM_LOG_ROWS`]' order.
    fn rows(&self) -> [f64; 6] {
        let [f, p, b, r] = self.alone_m;
        [self.own_m, f, p, b, r, self.band_m]
    }
}

impl Termed {
    /// The term that dominates the flight's part below Mach 0.8: the one whose absolute value
    /// is the largest, the earlier of two that tie.
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

/// One flight split, as this module counts it: split by term, or why not (an index into
/// [`NOT_TERMED`], with the detail kept on this machine); and its logged apogee as a height
/// climbed, m, when the report compares the flight with its log.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TermFlight {
    pub(crate) termed: TermResult,
    pub(crate) logged_m: Option<f64>,
}

/// One flight both simulators fly, as this module counts it: `None` when it is not split.
/// `build` and `climbed_m` are [`crate::gap_split::split`]'s; `histories` is OpenRocket's
/// record of the flight's inputs, `split` the flight's split, `banded` its split by band, and
/// `openrocket_m` OpenRocket's apogee.
///
/// # Errors
///
/// A split flight without its histories or its split by band; or the flight on every term
/// swapped failing, or missing the band split's flight below Mach 0.8 by more than
/// [`SUM_TOLERANCE_PERCENT`]. That flight is the band split's, which flew: either way the swap
/// no longer agrees with the tables, and the run stops.
pub(crate) fn flight(
    build: &dyn Fn(Air) -> Result<Simulation, String>,
    climbed_m: &dyn Fn(&Simulation) -> Result<f64, String>,
    histories: &Value,
    split: &Result<SplitFlight, NotSplit>,
    banded: Option<&Result<Banded, String>>,
    openrocket_m: f64,
) -> Result<Option<TermResult>, String> {
    let Ok(split) = split else {
        return Ok(None);
    };
    let banded = match banded {
        None => return Err("a flight that is split has no split by band".to_owned()),
        Some(Err(error)) => return Ok(Some(Err((0, error.clone())))),
        Some(Ok(banded)) => banded,
    };
    // The split flew these histories, so they are there.
    let Ok(inputs) = OpenRocketInputs::of(histories)? else {
        return Err("a flight that is split has no histories".to_owned());
    };
    let tables = match term_tables(histories)? {
        Ok(tables) => tables,
        Err(error) => return Ok(Some(Err((1, error)))),
    };
    let swapped_as =
        |openrockets: [bool; 4], histories: bool, powered_only: bool| -> Result<f64, String> {
            let mut simulation = build(Air::Day)?;
            if histories {
                simulation = simulation
                    .with_thrust_history(inputs.thrust.clone())
                    .map_err(|error| error.to_string())?
                    .with_mass_history(inputs.mass.clone())
                    .map_err(|error| error.to_string())?;
            }
            let swap = TermSwap::new(inputs.drag.clone(), tables.clone(), openrockets);
            climbed_m(&simulation.with_drag_model(if powered_only {
                swap.powered_only()
            } else {
                swap
            }))
        };
    let swapped =
        |openrockets: [bool; 4], histories: bool| swapped_as(openrockets, histories, false);
    let share = |before_m: f64, after_m: f64| 100.0 * (before_m - after_m) / openrocket_m;
    let apogees = split.apogees;
    // The flight on every term swapped is the band split's flight on OpenRocket's whole drag
    // below Mach 0.8, which flew. A failure here is the swap disagreeing with the tables.
    let every_m = swapped([true; 4], true).map_err(|error| {
        format!(
            "on every term swapped HPR Sim fails the flight it flew on OpenRocket's drag below \
             Mach 0.8: {error}"
        )
    })?;
    // A difference that isn't a number fails too.
    let same = (share(apogees.mass_m, every_m) - banded.parts[0]).abs() <= SUM_TOLERANCE_PERCENT;
    if !same {
        return Err(format!(
            "on every term swapped HPR Sim's part below Mach 0.8 is {}%, and the band's is {}%: \
             the terms don't sum to the band's part",
            share(apogees.mass_m, every_m),
            banded.parts[0]
        ));
    }
    let flown = || -> Result<[f64; 8], String> {
        let flown = [
            swapped([true, false, false, false], true)?,
            swapped([true, true, false, false], true)?,
            swapped([true, true, true, false], true)?,
            swapped([true, false, false, false], false)?,
            swapped([false, true, false, false], false)?,
            swapped([false, false, true, false], false)?,
            swapped([false, false, false, true], false)?,
            swapped_as([false, false, true, false], false, true)?,
        ];
        // A height that isn't finite is a failed flight, not a part.
        match flown.iter().find(|climbed_m| !climbed_m.is_finite()) {
            Some(climbed_m) => Err(format!("a flight with a term swapped climbs {climbed_m} m")),
            None => Ok(flown),
        }
    };
    let [f_m, fp_m, fpb_m, a_m, b_m, c_m, d_m, base_powered_m] = match flown() {
        Ok(flown) => flown,
        Err(error) => return Ok(Some(Err((2, error)))),
    };
    let alone_m = [a_m, b_m, c_m, d_m];
    Ok(Some(Ok(Termed {
        parts: [
            share(apogees.mass_m, f_m),
            share(f_m, fp_m),
            share(fp_m, fpb_m),
            share(fpb_m, every_m),
        ],
        alone: alone_m.map(|alone_m| share(apogees.own_m, alone_m)),
        base_powered_alone: share(apogees.own_m, base_powered_m),
        band: banded.parts[0],
        band_alone: banded.alone[0],
        heights_m: TermHeights {
            own_m: apogees.own_m,
            alone_m,
            band_m: banded.heights_m.alone_m[0],
        },
    })))
}

/// One term over the flights split by term. The sizes are there when the section is published
/// ([`DragTerms::published`]).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct Term {
    /// Its name, from [`TERMS`].
    pub name: String,
    /// The flights with a part in it: those whose part is not zero.
    pub flights: usize,
    /// Its part over every flight split by term.
    pub part: Option<Size>,
    /// The flights the term swapped alone changes: those whose part alone is not zero.
    pub alone_flights: usize,
    /// The term swapped alone, first, over every flight split by term.
    pub alone: Option<Size>,
    /// The flights it dominates: where its part's absolute value is the largest of the four.
    pub dominates: Option<usize>,
    /// The flights where its part's absolute value is over [`LARGE_PERCENT`].
    pub large: Option<usize>,
}

/// The rows against the logs, in order, with the report's words for the drag each flies.
pub(crate) const TERM_LOG_ROWS: [(&str, &str); 6] = [
    ("own", "its own"),
    ("friction", "OpenRocket's friction below Mach 0.8"),
    ("pressure", "OpenRocket's pressure below Mach 0.8"),
    ("base", "OpenRocket's base below Mach 0.8"),
    ("rest", "OpenRocket's rest below Mach 0.8"),
    ("subsonic", LOG_ROWS[1].1),
];

/// A table row's thrust, Mach number and one term's value.
type Cell = (f64, Option<f64>, Option<f64>);

/// A flight's terms, or why it has none: the index of its reason in [`NOT_TERMED`] and the
/// details.
pub(crate) type TermResult = Result<Termed, (usize, String)>;

/// A not-split reason's count.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NotTermed {
    /// Its name, from [`NOT_TERMED`].
    pub reason: String,
    pub flights: usize,
}

/// The report's section: the part below Mach 0.8 by term, as aggregates.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct DragTerms {
    /// The flights split.
    pub split: usize,
    /// Those split by term.
    pub termed: usize,
    /// The rest, by why: one per [`NOT_TERMED`], in its order.
    pub not_termed: Vec<NotTermed>,
    /// One per term, in [`TERMS`]' order.
    pub terms: Vec<Term>,
    /// The band's part below Mach 0.8 over the flights split by term, which the terms' parts
    /// sum to.
    pub band: Option<Size>,
    /// The band swapped alone, first, over the same flights.
    pub band_alone: Option<Size>,
    /// The flights the base swapped alone only while a motor burns changes.
    pub base_powered_flights: usize,
    /// The base swapped alone only while a motor burns, over every flight split by term.
    pub base_powered_alone: Option<Size>,
    /// The flights with a term swapped alone, beside the logs; the rows are
    /// [`TERM_LOG_ROWS`].
    pub logs: Logs,
    /// The flights the band section sets beside the logs. Sizes over that set and over this
    /// section's, given side by side, would give back the flights between them, so that
    /// difference is held to the floor too.
    pub band_logged: usize,
    /// The measured reference of the base term below Mach 0.8, from
    /// [`crate::aero_base_power::FIXTURE`].
    pub reference: Option<BaseReference>,
}

/// The base term's measured reference (NACA RM SL55F15, Fig. 20), with HPR Sim's base drag on
/// it, as [`crate::aero_base_power::generate`] writes it.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct BaseReference {
    /// The report's stated error of its faired base drag curves.
    pub stated_error: f64,
    /// The uncertainty of reading them off the figure.
    pub reading_uncertainty: f64,
    /// One row per Mach number, below Mach 0.8.
    pub rows: Vec<ReferenceRow>,
}

/// A row of the base term's measured reference (NACA RM SL55F15, Fig. 20): base drag
/// coefficients on the model's largest cross-section.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct ReferenceRow {
    pub mach: f64,
    pub measured_power_off: f64,
    pub hpr_power_off: f64,
    pub measured_power_on: f64,
    /// HPR Sim's rule given the nozzle exit's area.
    pub hpr_power_on: f64,
    /// HPR Sim's rule as it flies, the motor case's area off the base, down to zero.
    pub hpr_power_on_case: f64,
    /// The whole base under power, OpenRocket 24.12's rule: the power-off value.
    pub whole_base_under_power: f64,
}

/// The reference from [`crate::aero_base_power::generate`]'s fixture. Every value must be a
/// finite number, the measured power-off drag above zero and the measured power-on drag above
/// it: the report's text says the jet raises it.
pub(crate) fn base_reference(fixture: &Value) -> Result<BaseReference, String> {
    let fail = |what: String| format!("the base reference's fixture: {what}");
    let number = |value: &Value, key: &str| {
        value[key]
            .as_f64()
            .filter(|n| n.is_finite())
            .ok_or_else(|| fail(format!("{key} is not a finite number")))
    };
    let rows = fixture["rows"]
        .as_array()
        .filter(|rows| !rows.is_empty())
        .ok_or_else(|| fail("no rows".to_owned()))?
        .iter()
        .map(|row| {
            let r = ReferenceRow {
                mach: number(row, "mach")?,
                measured_power_off: number(row, "measured_power_off")?,
                hpr_power_off: number(row, "hpr_power_off")?,
                measured_power_on: number(row, "measured_power_on")?,
                hpr_power_on: number(row, "hpr_power_on")?,
                hpr_power_on_case: number(row, "hpr_power_on_case")?,
                whole_base_under_power: number(row, "whole_base_under_power")?,
            };
            if r.measured_power_off > 0.0 && r.measured_power_on > r.measured_power_off {
                Ok(r)
            } else {
                Err(fail(format!(
                    "at Mach {} the measured drag is {} power off and {} under power",
                    r.mach, r.measured_power_off, r.measured_power_on
                )))
            }
        })
        .collect::<Result<_, _>>()?;
    Ok(BaseReference {
        stated_error: number(fixture, "stated_error")?,
        reading_uncertainty: number(fixture, "reading_uncertainty")?,
        rows,
    })
}

/// Whether `count` flights are none, or enough for an aggregate over them to be given.
fn none_or_floor(count: usize) -> bool {
    count == 0 || count >= FLOOR
}

impl DragTerms {
    /// The section's numbers from each flight split, of which the report compares `compared`
    /// with a log. The sizes are given only when the section is published
    /// ([`DragTerms::published`]).
    /// `band_logged` is the band section's count of flights beside the logs, `None` for this
    /// section's own.
    pub(crate) fn of(flights: &[TermFlight], compared: usize, band_logged: Option<usize>) -> Self {
        let termed: Vec<(&Termed, Option<f64>)> = flights
            .iter()
            .filter_map(|flight| Some((flight.termed.as_ref().ok()?, flight.logged_m)))
            .collect();
        let logged: Vec<[f64; 6]> = termed
            .iter()
            .filter_map(|(t, logged_m)| {
                let logged_m = (*logged_m)?;
                Some(
                    t.heights_m
                        .rows()
                        .map(|m| 100.0 * (m - logged_m) / logged_m),
                )
            })
            .collect();
        let mut section = Self {
            split: flights.len(),
            termed: termed.len(),
            not_termed: NOT_TERMED
                .iter()
                .enumerate()
                .map(|(i, (reason, _))| NotTermed {
                    reason: (*reason).to_owned(),
                    flights: flights
                        .iter()
                        .filter(|f| matches!(&f.termed, Err((r, _)) if *r == i))
                        .count(),
                })
                .collect(),
            terms: TERMS
                .iter()
                .enumerate()
                .map(|(i, (name, _))| Term {
                    name: (*name).to_owned(),
                    flights: termed.iter().filter(|(t, _)| t.parts[i] != 0.0).count(),
                    alone_flights: termed.iter().filter(|(t, _)| t.alone[i] != 0.0).count(),
                    ..Term::default()
                })
                .collect(),
            band: None,
            band_alone: None,
            base_powered_flights: termed
                .iter()
                .filter(|(t, _)| t.base_powered_alone != 0.0)
                .count(),
            base_powered_alone: None,
            logs: Logs {
                flights: logged.len(),
                left_out: compared.saturating_sub(logged.len()),
                rows: TERM_LOG_ROWS
                    .iter()
                    .enumerate()
                    .map(|(row, (name, _))| LogRow {
                        name: (*name).to_owned(),
                        moved: logged.iter().filter(|e| e[row] != e[0]).count(),
                        ..LogRow::default()
                    })
                    .collect(),
            },
            band_logged: band_logged.unwrap_or(logged.len()),
            reference: None,
        };
        if !section.published() {
            return section;
        }
        let size = |of: &dyn Fn(&Termed) -> f64| Some(Size::of(termed.iter().map(|(t, _)| of(t))));
        for (i, term) in section.terms.iter_mut().enumerate() {
            term.part = size(&|t| t.parts[i]);
            term.alone = size(&|t| t.alone[i]);
            term.dominates = Some(termed.iter().filter(|(t, _)| t.dominant() == i).count());
            term.large = Some(
                termed
                    .iter()
                    .filter(|(t, _)| t.parts[i].abs() > LARGE_PERCENT)
                    .count(),
            );
        }
        for (row, log) in section.logs.rows.iter_mut().enumerate() {
            log.nearer = Some(logged.iter().filter(|e| e[row].abs() < e[0].abs()).count());
            log.farther = Some(logged.iter().filter(|e| e[row].abs() > e[0].abs()).count());
            // Summed from the least up, as the report's errors table and the bands sum theirs.
            let mut errors: Vec<f64> = logged.iter().map(|e| e[row]).collect();
            errors.sort_by(f64::total_cmp);
            log.error = Some(Size::of(errors.iter().copied()));
        }
        section.band = size(&|t| t.band);
        section.band_alone = size(&|t| t.band_alone);
        section.base_powered_alone = size(&|t| t.base_powered_alone);
        section
    }

    /// Whether the sizes are given: at least [`FLOOR`] flights are split by term, and each of
    /// these is none or at least [`FLOOR`]: the flights under each reason not split by term
    /// (the band's part is over them too), a term's flights, the flights a term alone changes,
    /// the flights compared with a log, those left out, and the flights each row against the
    /// logs moves. All or nothing, as the bands: the terms sum to the band's part, so one held
    /// back could be read off the others (ADR-151: aggregates only).
    pub(crate) fn published(&self) -> bool {
        self.termed >= FLOOR
            && self.not_termed.iter().all(|n| none_or_floor(n.flights))
            && self
                .terms
                .iter()
                .all(|term| none_or_floor(term.flights) && none_or_floor(term.alone_flights))
            && none_or_floor(self.base_powered_flights)
            && none_or_floor(self.logs.flights)
            && none_or_floor(self.logs.left_out)
            && self.logs.rows.iter().all(|row| none_or_floor(row.moved))
            && none_or_floor(self.band_logged.saturating_sub(self.logs.flights))
    }

    /// Holds the section to the band split's: the flights it starts from are those split, the
    /// first reason is the band split's failures, at least half the flights split by band are
    /// split by term (ADR-359), and with every flight split by band also split by term the band
    /// row and the last row against the logs are the band section's own.
    pub(crate) fn check_against_bands(
        &self,
        bands: &crate::drag_bands::DragBands,
    ) -> Result<(), String> {
        let fail = |why: String| Err(format!("the part below Mach 0.8 by term: {why}"));
        let not_banded = self.not_termed.first().map_or(0, |n| n.flights);
        if self.split != bands.split || not_banded != bands.failed {
            return fail(format!(
                "{} flights split and {not_banded} not split by band, where the bands have {} \
                 and {}",
                self.split, bands.split, bands.failed
            ));
        }
        if self.band_logged != bands.logs.flights {
            return fail(format!(
                "the bands set {} flights beside the logs, where this section counts {}",
                bands.logs.flights, self.band_logged
            ));
        }
        if self.termed.saturating_mul(2) < bands.banded {
            return fail(format!(
                "{} of the {} flights split by band are split by term, fewer than half",
                self.termed, bands.banded
            ));
        }
        if self.termed == bands.banded && self.published() && bands.published() {
            let theirs = bands.bands.first().and_then(|band| band.part);
            let theirs_alone = bands.bands.first().and_then(|band| band.alone);
            if (self.band, self.band_alone) != (theirs, theirs_alone) {
                return fail(format!(
                    "the band's part is {:?} and alone {:?}, where the bands have {theirs:?} and \
                     {theirs_alone:?}",
                    self.band, self.band_alone
                ));
            }
            if self.logs.flights == bands.logs.flights {
                let pairs = [(0, 0), (TERM_LOG_ROWS.len() - 1, 1)];
                for (ours, theirs) in pairs {
                    let (Some(a), Some(b)) =
                        (self.logs.rows.get(ours), bands.logs.rows.get(theirs))
                    else {
                        return fail("a row against the logs is missing".to_owned());
                    };
                    if (a.moved, a.nearer, a.farther, a.error)
                        != (b.moved, b.nearer, b.farther, b.error)
                    {
                        return fail(format!(
                            "the {} row against the logs is {a:?}, where the bands' is {b:?}",
                            a.name
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    /// Holds the section to itself: `compared` flights compared with a log.
    pub(crate) fn check_consistent(&self, compared: usize) -> Result<(), String> {
        let fail = |why: String| Err(format!("the part below Mach 0.8 by term: {why}"));
        let names: Vec<&str> = self.terms.iter().map(|t| t.name.as_str()).collect();
        let expected: Vec<&str> = TERMS.iter().map(|(name, _)| *name).collect();
        if names != expected {
            return fail(format!("the terms are {names:?}, not {expected:?}"));
        }
        let reasons: Vec<&str> = self.not_termed.iter().map(|n| n.reason.as_str()).collect();
        let expected: Vec<&str> = NOT_TERMED.iter().map(|(name, _)| *name).collect();
        if reasons != expected {
            return fail(format!("the reasons are {reasons:?}, not {expected:?}"));
        }
        let rows: Vec<&str> = self.logs.rows.iter().map(|r| r.name.as_str()).collect();
        let expected: Vec<&str> = TERM_LOG_ROWS.iter().map(|(name, _)| *name).collect();
        if rows != expected {
            return fail(format!(
                "the rows against the logs are {rows:?}, not {expected:?}"
            ));
        }
        // Saturating: a count read from a file can be any number.
        let not_termed = self
            .not_termed
            .iter()
            .fold(0_usize, |sum, n| sum.saturating_add(n.flights));
        if self.termed.saturating_add(not_termed) != self.split {
            return fail(format!(
                "{} split by term and {not_termed} not are not the {} flights split",
                self.termed, self.split
            ));
        }
        if self.base_powered_flights > self.termed {
            return fail(format!(
                "the base under thrust alone changes {} flights of {}",
                self.base_powered_flights, self.termed
            ));
        }
        if let Some(term) = self
            .terms
            .iter()
            .find(|term| term.flights.max(term.alone_flights) > self.termed)
        {
            return fail(format!(
                "{} flights have a part in {} and {} alone, more than the {} split by term",
                term.flights, term.name, term.alone_flights, self.termed
            ));
        }
        let logs = &self.logs;
        if logs.flights.saturating_add(logs.left_out) != compared || logs.flights > self.termed {
            return fail(format!(
                "{} flights against the logs and {} left out, of {compared} compared and {} \
                 split by term",
                logs.flights, logs.left_out, self.termed
            ));
        }
        if let Some(row) = logs.rows.iter().find(|row| row.moved > logs.flights) {
            return fail(format!(
                "the {} row moves {} flights of {}",
                row.name, row.moved, logs.flights
            ));
        }
        if logs.rows.first().is_some_and(|row| row.moved != 0) {
            return fail("HPR Sim's own row moves a flight".to_owned());
        }
        let published = self.published();
        let given = [
            self.band.is_some(),
            self.band_alone.is_some(),
            self.base_powered_alone.is_some(),
        ]
        .into_iter()
        .chain(self.terms.iter().flat_map(|term| {
            [
                term.part.is_some(),
                term.alone.is_some(),
                term.dominates.is_some(),
                term.large.is_some(),
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
                "a size is {}: they are given when no count the rule reads is under {FLOOR} \
                 flights but none (`DragTerms::published`)",
                if published { "missing" } else { "given" }
            ));
        }
        let (Some(band), Some(band_alone)) = (self.band, self.band_alone) else {
            return Ok(());
        };
        band.consistent("the band's part over the flights split by term")?;
        band_alone.consistent("the band alone over the flights split by term")?;
        if let Some(powered) = self.base_powered_alone {
            powered.consistent("the base under thrust alone")?;
            if self.base_powered_flights == 0 && powered != Size::default() {
                return fail(format!(
                    "the base under thrust alone is {powered:?} with no flight changed"
                ));
            }
        }
        let mut dominated = 0_usize;
        let mut sum = 0.0;
        for term in &self.terms {
            let (Some(part), Some(alone), Some(dominates), Some(large)) =
                (term.part, term.alone, term.dominates, term.large)
            else {
                return fail(format!("{} lacks a size", term.name));
            };
            part.consistent(&format!("the {} term", term.name))?;
            alone.consistent(&format!("the {} term alone", term.name))?;
            // A part over 1% is not zero, nor is one that dominates past the first term's.
            if large > term.flights || (term.name != TERMS[0].0 && dominates > term.flights) {
                return fail(format!(
                    "{} dominates {dominates} flights and is large on {large}, with a part in {}",
                    term.name, term.flights
                ));
            }
            if (term.flights == 0 && part != Size::default())
                || (term.alone_flights == 0 && alone != Size::default())
            {
                return fail(format!(
                    "{} has sizes {part:?} and {alone:?} with no flight changed",
                    term.name
                ));
            }
            dominated = dominated.saturating_add(dominates);
            sum += part.mean_percent;
        }
        if dominated != self.termed {
            return fail(format!(
                "{dominated} flights are dominated by a term, not the {} split by term",
                self.termed
            ));
        }
        // Each mean is rounded to 0.005: four of them, then the band's.
        if (sum - band.mean_percent).abs() > 0.025 + 1e-9 {
            return fail(format!(
                "the terms' means sum to {sum}, not the band's {}",
                band.mean_percent
            ));
        }
        for row in &logs.rows {
            let (Some(nearer), Some(farther), Some(error)) = (row.nearer, row.farther, row.error)
            else {
                return fail(format!("the {} row lacks a size", row.name));
            };
            error.consistent(&format!("{} against the logs", row.name))?;
            if nearer.saturating_add(farther) > row.moved {
                return fail(format!(
                    "the {} row has {nearer} flights nearer and {farther} farther, of {} moved",
                    row.name, row.moved
                ));
            }
        }
        Ok(())
    }

    /// The term in [`TERMS`]' first three, friction, pressure and base, with the largest mean
    /// absolute part, the earlier of two that tie: the one M1.14i2's measured reference is of.
    /// `None` when the sizes aren't given.
    pub(crate) fn largest_measurable(&self) -> Option<usize> {
        let mut largest: Option<(usize, f64)> = None;
        for (i, term) in self.terms.iter().take(3).enumerate() {
            let value = term.part?.mean_absolute_percent;
            if largest.is_none_or(|(_, best)| value > best) {
                largest = Some((i, value));
            }
        }
        largest.map(|(i, _)| i)
    }

    /// The base term's measured reference and HPR Sim's error on each row, given whether or not
    /// the sizes are: it holds no private number.
    fn reference_markdown(&self) -> String {
        let mut page = String::new();
        let Some(reference) = &self.reference else {
            return page;
        };
        let _ = writeln!(
            page,
            "\nThe measured reference of the base term below Mach 0.8 is the base drag of the \
             Hermes A-3A drag model A(1) in free flight, power on and power off (NACA RM \
             SL55F15, Fig. 20, its faired curves' error believed to be less than ±{}), read off \
             the figure to ±{} more ({}). Each coefficient is on the model's largest \
             cross-section. HPR Sim's rule as it flies takes the motor case's area off the base, \
             down to zero; the model's nominal 5 in case is wider than its 4.8 in base. Given \
             the nozzle exit's area instead, which is what the base holds of the motor, the \
             same rule leaves the annulus the report measured on. The whole base under power, \
             OpenRocket 24.12's rule, is the power-off value.\n",
            reference.stated_error,
            reference.reading_uncertainty,
            crate::aero_base_power::FIXTURE
        );
        let _ = writeln!(
            page,
            "| Mach | measured power off | HPR Sim power off | difference | measured under \
             power | HPR Sim under power, case off | difference | HPR Sim under power, nozzle \
             exit off | difference | the whole base under power | difference \
             |\n|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|"
        );
        for r in &reference.rows {
            let _ = writeln!(
                page,
                "| {:.2} | {:.4} | {:.4} | {:+.4} | {:.4} | {:.4} | {:+.4} | {:.4} | {:+.4} | \
                 {:.4} | {:+.4} |",
                r.mach,
                r.measured_power_off,
                r.hpr_power_off,
                r.hpr_power_off - r.measured_power_off,
                r.measured_power_on,
                r.hpr_power_on_case,
                r.hpr_power_on_case - r.measured_power_on,
                r.hpr_power_on,
                r.hpr_power_on - r.measured_power_on,
                r.whole_base_under_power,
                r.whole_base_under_power - r.measured_power_on,
            );
        }
        // The least and the most of a quantity over the rows, which `base_reference` holds to
        // finite numbers with the power-off drag above zero.
        let range = |of: &dyn Fn(&ReferenceRow) -> f64| {
            let values = reference.rows.iter().map(of);
            let least = values.clone().fold(f64::INFINITY, f64::min);
            (least, values.fold(f64::NEG_INFINITY, f64::max))
        };
        // "a to b", or one value where the two print the same.
        let span = |(least, most): (f64, f64), digits: usize| {
            let (a, b) = (format!("{least:.digits$}"), format!("{most:.digits$}"));
            if a == b { a } else { format!("{a} to {b}") }
        };
        let _ = writeln!(
            page,
            "\nPower off, HPR Sim's value is {} times the measured. The jet raises the measured \
             base drag to {} times its power-off value. HPR Sim's rule given the nozzle exit \
             gives {} of its power-off value, and the whole base 1. Kept on the measured \
             power-off value, the whole base would read low by the measured power-on less \
             power-off: {}.",
            span(range(&|r| r.hpr_power_off / r.measured_power_off), 1),
            span(range(&|r| r.measured_power_on / r.measured_power_off), 1),
            span(range(&|r| r.hpr_power_on / r.hpr_power_off), 2),
            span(range(&|r| r.measured_power_on - r.measured_power_off), 4)
        );
        page
    }

    /// The section, after the bands'.
    pub(crate) fn to_markdown(&self) -> String {
        let mut page = String::new();
        let flights = |n: usize| if n == 1 { "flight" } else { "flights" };
        let _ = writeln!(
            page,
            "\n### The part below Mach 0.8 by term\n\nFor M1.14i2 (ADR-359), the band's part \
             below Mach 0.8 is split by the term of the drag it comes from. OpenRocket's flight \
             data splits its drag coefficient into friction, pressure and base; the rest is \
             the coefficient less the three. OpenRocket books a launch lug and rail buttons \
             under pressure and a stated drag coefficient in the rest, so HPR Sim's pressure \
             takes its launch lugs and rail buttons with it, and its rest is its stated \
             coefficients. On OpenRocket's thrust and mass, HPR Sim flies with OpenRocket's \
             friction in place of its own below Mach 0.8, then its pressure too, then its \
             base, then its rest. A term's part is the apogee before its swap less the apogee \
             after it. The four sum to the band's part: the last of these flights flies \
             OpenRocket's whole drag below Mach 0.8, as the band's does, and the run stops on \
             a flight where they don't agree. Like the bands, the parts are code against \
             code.\n"
        );
        let _ = writeln!(
            page,
            "Of the {} {} split, {} {} split by term.",
            self.split,
            flights(self.split),
            self.termed,
            if self.termed == 1 { "is" } else { "are" },
        );
        for (n, (_, words)) in self.not_termed.iter().zip(NOT_TERMED) {
            if n.flights > 0 {
                let _ = writeln!(page, "Not split by term, {}: {words}.", n.flights);
            }
        }
        page.push('\n');
        let (Some(band), Some(band_alone)) = (self.band, self.band_alone) else {
            let _ = writeln!(page, "| term | flights with a part in it |\n|---|---:|");
            for (term, (_, words)) in self.terms.iter().zip(TERMS) {
                let _ = writeln!(page, "| {words} | {} |", term.flights);
            }
            let _ = writeln!(
                page,
                "\nThe terms' sizes are not given: an aggregate over one or two flights is a \
                 flight's own number, and the flights are private. The terms sum to the band's \
                 part, so one held back could be read off the others, and none is given."
            );
            page.push_str(&self.reference_markdown());
            return page;
        };
        let sizes = |size: Size| {
            format!(
                "{:+.2}% | {:+.2}% | {:.2}%",
                size.mean_percent, size.median_percent, size.mean_absolute_percent
            )
        };
        let _ = writeln!(
            page,
            "| term | flights with a part in it | mean | median | mean absolute | flights it \
             dominates | flights where it is over {LARGE_PERCENT}% |\n|---|---:|---:|---:|---:|\
             ---:|---:|"
        );
        for (term, (_, words)) in self.terms.iter().zip(TERMS) {
            let (Some(part), Some(dominates), Some(large)) =
                (term.part, term.dominates, term.large)
            else {
                continue;
            };
            let _ = writeln!(
                page,
                "| {words} | {} | {} | {dominates} | {large} |",
                term.flights,
                sizes(part)
            );
        }
        let _ = writeln!(
            page,
            "| the part below Mach 0.8 | {} | {} |  |  |",
            self.termed,
            sizes(band)
        );
        let _ = writeln!(
            page,
            "\nEach percentage is a share of OpenRocket's apogee, over the {} {} split by term. \
             A term dominates a flight when its part's absolute value is the largest of the \
             four, the earlier term if two tie. A positive part means HPR Sim climbs higher on \
             its own term than on OpenRocket's: its term is the lower of the two.\n",
            self.termed,
            flights(self.termed)
        );
        if let Some(largest) = self.largest_measurable() {
            let _ = writeln!(
                page,
                "Of friction, pressure and base, {} has the largest mean absolute part: M1.14i2's \
                 measured reference is of that term.\n",
                TERMS[largest].0
            );
        }
        let _ = writeln!(
            page,
            "Each term is also swapped alone, first: on HPR Sim's own thrust and mass, with its \
             own drag in the other terms and above Mach 0.8. The base is swapped alone a second \
             time only while a motor burns, HPR Sim's own base drag after burnout: the two codes \
             part on the base under thrust, where HPR Sim takes the burning motor's area off the \
             base and OpenRocket 24.12 keeps the whole base.\n"
        );
        let _ = writeln!(
            page,
            "| term, swapped alone | flights it changes | mean | median | mean absolute \
             |\n|---|---:|---:|---:|---:|"
        );
        for (term, (_, words)) in self.terms.iter().zip(TERMS) {
            if let Some(alone) = term.alone {
                let _ = writeln!(
                    page,
                    "| {words} | {} | {} |",
                    term.alone_flights,
                    sizes(alone)
                );
            }
        }
        if let Some(powered) = self.base_powered_alone {
            let _ = writeln!(
                page,
                "| base, only while a motor burns | {} | {} |",
                self.base_powered_flights,
                sizes(powered)
            );
        }
        let _ = writeln!(
            page,
            "| every term: the band below Mach 0.8, swapped alone |  | {} |",
            sizes(band_alone)
        );
        let logs = &self.logs;
        let _ = writeln!(
            page,
            "\nThe flights with a term swapped alone are also set beside the logs: of the \
             flights compared with a log, {} {} split by term. Each row is the apogee less the \
             logged one, over the logged one, and counts the flights it moves nearer their log \
             than HPR Sim's own flight and farther from it, as the bands' rows do.\n",
            logs.flights,
            if logs.flights == 1 { "is" } else { "are" },
        );
        let _ = writeln!(
            page,
            "| HPR Sim's drag | flights moved | mean | median | mean absolute | flights nearer \
             their log | flights farther from it |\n|---|---:|---:|---:|---:|---:|---:|"
        );
        for (i, (row, (_, words))) in logs.rows.iter().zip(TERM_LOG_ROWS).enumerate() {
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
            "\nAs with the bands, a row nearer the logs doesn't show that OpenRocket's term is \
             the truer one: both simulators climb above the logs on average, so more drag from \
             any cause moves the mean toward them. The terms say where to look for a \
             measurement; a measured reference says which code is nearer it."
        );
        page.push_str(&self.reference_markdown());
        page
    }
}

#[cfg(test)]
mod tests;
