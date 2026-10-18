//! A set's census: how many flights each design and logger flew, the ranges they span, and every
//! draw that didn't fly, with its reason. `cargo xtask synthetic` commits it.
//!
//! Values are kept to three significant figures, the digits every platform shares: a flight's
//! numbers match across platforms to many digits, not to the bit (`docs/physics/monte-carlo.md`).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::designs::{DESIGNS, DesignFiles};
use super::{Flight, Flyer, Model, Set};

/// The least, middle and greatest of a quantity over a set's flights.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Range {
    /// The least.
    pub min: f64,
    /// The median.
    pub median: f64,
    /// The greatest.
    pub max: f64,
}

/// `x` to three significant figures, nearest.
pub fn three_figures(x: f64) -> f64 {
    to_three_figures(x, Toward::Nearest)
}

/// Which way [`to_three_figures`] rounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Toward {
    Nearest,
    Down,
    Up,
}

/// `x` to three significant figures. Rounded down or up, the result is stepped once more if the
/// division by the scale left it a hair on the wrong side of `x`, so a range's ends hold every
/// value exactly.
fn to_three_figures(x: f64, toward: Toward) -> f64 {
    if x == 0.0 || !x.is_finite() {
        return x;
    }
    let scale = 10_f64.powi(2 - x.abs().log10().floor() as i32);
    let scaled = x * scale;
    let whole = match toward {
        Toward::Nearest => scaled.round(),
        Toward::Down => scaled.floor(),
        Toward::Up => scaled.ceil(),
    };
    let rounded = whole / scale;
    match toward {
        Toward::Down if rounded > x => (whole - 1.0) / scale,
        Toward::Up if rounded < x => (whole + 1.0) / scale,
        _ => rounded,
    }
}

impl Range {
    /// The range of `values` to three significant figures, its ends rounded outward (the least
    /// down, the greatest up) so that the range holds every value, and its median to the nearest;
    /// `None` if there are none.
    fn of(mut values: Vec<f64>) -> Option<Self> {
        if values.is_empty() {
            return None;
        }
        values.sort_by(f64::total_cmp);
        let n = values.len();
        let median = if n % 2 == 1 {
            values[n / 2]
        } else {
            0.5 * (values[n / 2 - 1] + values[n / 2])
        };
        Some(Self {
            min: to_three_figures(values[0], Toward::Down),
            median: three_figures(median),
            max: to_three_figures(values[n - 1], Toward::Up),
        })
    }
}

/// A draw that didn't fly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotFlown {
    /// Its index in the set.
    pub index: u64,
    /// Its design.
    pub design: String,
    /// Why.
    pub reason: String,
}

/// One set's census.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCensus {
    /// The set.
    pub set: Set,
    /// Its seed.
    pub seed: u64,
    /// The flights drawn.
    pub drawn: u64,
    /// The flights flown to the ground and logged.
    pub flown: u64,
    /// Of those, the flights of set B's model.
    pub other_model: u64,
    /// Flights flown, by design.
    pub by_design: BTreeMap<String, u64>,
    /// Flights flown, by logger profile.
    pub by_logger: BTreeMap<String, u64>,
    /// The apogee above where the logger sat on the pad, m.
    pub apogee_m: Option<Range>,
    /// The top Mach number.
    pub top_mach: Option<Range>,
    /// The top vertical speed, m/s.
    pub top_speed_m_s: Option<Range>,
    /// The flight's time, liftoff to landing, s.
    pub flight_time_s: Option<Range>,
    /// The wind's speed, m/s.
    pub wind_m_s: Option<Range>,
    /// The rail's angle from vertical, degrees.
    pub tilt_deg: Option<Range>,
    /// The site's elevation, m.
    pub elevation_m: Option<Range>,
    /// The air's offset from the standard, K.
    pub temperature_offset_k: Option<Range>,
    /// Every draw that didn't fly.
    pub not_flown: Vec<NotFlown>,
}

/// What a census keeps of one flight: a few numbers, not its log, so that a census of 1,000
/// flights holds 1,000 rows rather than 1,000 logs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Row {
    /// The design's name.
    pub design: String,
    /// The logger profile's name.
    pub logger: String,
    /// Whether it flew set B's model.
    pub other_model: bool,
    /// The apogee above where the logger sat on the pad, m.
    pub apogee_m: f64,
    /// The top Mach number.
    pub top_mach: f64,
    /// The top vertical speed, m/s.
    pub top_speed_m_s: f64,
    /// Liftoff to landing, s.
    pub flight_time_s: f64,
    /// The wind's speed, m/s.
    pub wind_m_s: f64,
    /// The rail's angle from vertical, degrees.
    pub tilt_deg: f64,
    /// The site's elevation, m.
    pub elevation_m: f64,
    /// The air's offset from the standard, K.
    pub temperature_offset_k: f64,
}

impl Row {
    /// A flight's row; a reason instead if any of its numbers isn't finite, so that a census
    /// counts the flight as not flown rather than ranking around a NaN.
    pub fn of(flight: &Flight) -> Result<Self, String> {
        let draw = &flight.draw;
        let truth = &flight.truth;
        let row = Self {
            design: design_name(draw.design).to_owned(),
            logger: draw.logger.profile.name().to_owned(),
            other_model: matches!(draw.model, Model::Other { .. }),
            apogee_m: truth.apogee_above_pad_m(),
            top_mach: truth.top_mach(),
            top_speed_m_s: truth.top_speed().0,
            flight_time_s: truth.events.landing_s - truth.events.liftoff_s,
            wind_m_s: draw.wind_speed_m_s,
            tilt_deg: draw.tilt_rad.to_degrees(),
            elevation_m: draw.elevation_m,
            temperature_offset_k: draw.temperature_offset_k,
        };
        let numbers = [
            row.apogee_m,
            row.top_mach,
            row.top_speed_m_s,
            row.flight_time_s,
            row.wind_m_s,
            row.tilt_deg,
            row.elevation_m,
            row.temperature_offset_k,
        ];
        if numbers.iter().all(|x| x.is_finite()) {
            Ok(row)
        } else {
            Err("a number in its row isn't finite".to_owned())
        }
    }
}

/// Design `design`'s name, or a placeholder for an index that names none.
fn design_name(design: usize) -> &'static str {
    DESIGNS.get(design).map_or("not one of the ten", |d| d.name)
}

/// Flight `index` of `set` as a census row, or why it didn't fly.
pub fn row(flyer: &mut Flyer, set: Set, index: u64) -> Result<Row, String> {
    flyer
        .flight(set, index)
        .map_err(|error| error.to_string())
        .and_then(|flight| Row::of(&flight))
}

/// The census of `set` from every flight's row, `rows[k]` flight `k`'s.
pub fn from_rows(set: Set, rows: &[Result<Row, String>]) -> SetCensus {
    let mut by_design = BTreeMap::new();
    let mut by_logger = BTreeMap::new();
    let mut not_flown = Vec::new();
    let mut other_model = 0;
    let mut columns: [Vec<f64>; 8] = Default::default();
    for (index, row) in rows.iter().enumerate() {
        match row {
            Ok(row) => {
                *by_design.entry(row.design.clone()).or_insert(0) += 1;
                *by_logger.entry(row.logger.clone()).or_insert(0) += 1;
                other_model += u64::from(row.other_model);
                let values = [
                    row.apogee_m,
                    row.top_mach,
                    row.top_speed_m_s,
                    row.flight_time_s,
                    row.wind_m_s,
                    row.tilt_deg,
                    row.elevation_m,
                    row.temperature_offset_k,
                ];
                for (column, value) in columns.iter_mut().zip(values) {
                    column.push(value);
                }
            }
            Err(reason) => {
                let index = index as u64;
                not_flown.push(NotFlown {
                    index,
                    design: design_name(super::draw(set, index).design).to_owned(),
                    reason: reason.clone(),
                });
            }
        }
    }
    let drawn = rows.len() as u64;
    let [apogee, mach, speed, time, wind, tilt, elevation, offset] = columns;
    SetCensus {
        set,
        seed: set.seed(),
        drawn,
        flown: drawn - not_flown.len() as u64,
        other_model,
        by_design,
        by_logger,
        apogee_m: Range::of(apogee),
        top_mach: Range::of(mach),
        top_speed_m_s: Range::of(speed),
        flight_time_s: Range::of(time),
        wind_m_s: Range::of(wind),
        tilt_deg: Range::of(tilt),
        elevation_m: Range::of(elevation),
        temperature_offset_k: Range::of(offset),
        not_flown,
    }
}

/// Flies the first `count` flights of `set` one after another and counts what came of them;
/// `cargo xtask synthetic` flies whole sets on several threads and gives each flight's row to
/// [`from_rows`] in order, which comes to the same.
pub fn census(set: Set, count: u64, files: DesignFiles) -> SetCensus {
    let mut flyer = Flyer::new(files);
    let rows: Vec<_> = (0..count)
        .map(|index| row(&mut flyer, set, index))
        .collect();
    from_rows(set, &rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a_row(design: &str, logger: &str, other_model: bool, apogee_m: f64) -> Row {
        Row {
            design: design.to_owned(),
            logger: logger.to_owned(),
            other_model,
            apogee_m,
            top_mach: 0.5,
            top_speed_m_s: 170.0,
            flight_time_s: 100.0,
            wind_m_s: 3.0,
            tilt_deg: 5.0,
            elevation_m: 200.0,
            temperature_offset_k: 0.0,
        }
    }

    #[test]
    fn a_census_counts_the_flown_rows_and_names_each_draw_not_flown() {
        let rows = vec![
            Ok(a_row("x", "altos", false, 1_000.0)),
            Err("it hit a wall".to_owned()),
            Ok(a_row("y", "altos", true, 3_000.0)),
            Ok(a_row("x", "rrc3", true, 2_000.0)),
            Err("its row isn't finite".to_owned()),
        ];
        let census = from_rows(Set::B, &rows);
        assert_eq!((census.drawn, census.flown, census.other_model), (5, 3, 2));
        assert_eq!(census.by_design.get("x"), Some(&2));
        assert_eq!(census.by_design.get("y"), Some(&1));
        assert_eq!(census.by_logger.get("altos"), Some(&2));
        assert_eq!(census.by_logger.get("rrc3"), Some(&1));
        let apogee = census.apogee_m.unwrap();
        assert_eq!(
            (apogee.min, apogee.median, apogee.max),
            (1_000.0, 2_000.0, 3_000.0)
        );
        // Each draw not flown keeps its own index, its drawn design and its reason.
        let missing: Vec<_> = census
            .not_flown
            .iter()
            .map(|n| (n.index, n.design.as_str(), n.reason.as_str()))
            .collect();
        let name = |index| DESIGNS[super::super::draw(Set::B, index).design].name;
        assert_eq!(
            missing,
            [
                (1, name(1), "it hit a wall"),
                (4, name(4), "its row isn't finite")
            ]
        );
        // With nothing flown, every range is empty.
        let none = from_rows(Set::A, &[Err("no".to_owned())]);
        assert_eq!((none.drawn, none.flown), (1, 0));
        assert!(none.apogee_m.is_none() && none.top_mach.is_none());
    }

    #[test]
    fn three_figures_rounds_to_three_significant_digits() {
        assert_eq!(three_figures(1234.5), 1230.0);
        assert_eq!(three_figures(0.012_345), 0.0123);
        assert_eq!(three_figures(-8.886), -8.89);
        assert_eq!(three_figures(0.0), 0.0);
    }

    #[test]
    fn a_range_has_its_least_median_and_greatest() {
        let r = Range::of(vec![3.0, 1.0, 2.0, 10.0]).unwrap();
        assert_eq!((r.min, r.median, r.max), (1.0, 2.5, 10.0));
        // The ends round outward, so the range holds every value; the median to the nearest.
        let r = Range::of(vec![1.174_9, 412.6, -14.62, 1.0]).unwrap();
        assert_eq!((r.min, r.max), (-14.7, 413.0));
        assert!(r.median <= 1.18 && r.median >= 1.08);
        let r = Range::of(vec![1.174_9]).unwrap();
        assert_eq!((r.min, r.median, r.max), (1.17, 1.17, 1.18));
        assert_eq!(Range::of(Vec::new()), None);
    }
}
