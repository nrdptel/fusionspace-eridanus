//! Two or more recordings of one flight on one timeline, read side by side and never averaged.
//!
//! **What it is for.** A high-power rocket often carries two altimeters, and a staged one may carry
//! a logger in each stage. Each logger counts from a zero of its own: when it was switched on, or
//! when it saw the boost. To lay the logs side by side their clocks must be lined up.
//!
//! **How.** By a shared liftoff, the method [Debrief][debrief] uses: every board in a stack leaves
//! the pad at the same instant, so the time from one log's liftoff to the other's is the offset
//! between their clocks. There is no cross-correlation and no sweep over lags. What a timeline says
//! of a log is its liftoff's reading, with that reading's bound and interval. The offset's bound
//! is the sum of the two liftoffs' bounds, the most their errors can add to, and its 90% interval
//! is [`OFFSET_SHARES`] of that bound ([ADR-257][adr-257] §1).
//!
//! **Nothing is averaged.** Two boards in one airframe are two measurements. A [`SideBySide`] holds
//! each log's reading in its own cell, and each row's [`Row::spread`], the largest less the least,
//! describes how far apart they lie. It is not a check, and it isn't an error on either: two boards
//! can agree and both be wrong by the day's air. No field holds a mean, a weighted mean or a
//! fused track, and a test holds that to the serialized form.
//!
//! **Stages** ([`Flew::Stages`]) are laid out in the same way, each stage's logger leaving the pad
//! with the stack, and report no spread: a booster's apogee and a sustainer's are different
//! heights, not two measures of one. A logger that was still when the stack lifted off, or that
//! started in the air, has no liftoff, and sits on no timeline.
//!
//! **How far to trust it.** On 2,000 synthetic flights carrying two loggers, on seeds of their own,
//! 1,383 gave an offset (a log with no barometer has no liftoff); every one stayed within its
//! bound, by 0.13 of it at most and 0.03 s in the median, and its interval passed [the coverage test][adr-257]
//! (`cargo xtask two-logs`, [`validation/reports/two-logs.md`][report]). Each of the 135 of 200
//! two-stage flights that gave one stayed within its bound too; the interval missed none of them,
//! so it is wider than it need be there. One real pair checks it, weakly, against another clock: on
//! RocketPy's Prometheus flight the two boards' GPS time gives an offset about 0.08 s from this one,
//! where an offset of zero would miss by 0.13 s. The offset is one number per pair: a clock that runs fast or slow
//! isn't corrected, by about 15 ms over a 300 s flight at a crystal's 50 ppm. Nothing here can tell
//! that two logs are of one flight: that is the caller's word.
//!
//! [debrief]: https://github.com/nrdptel/fusionspace-debrief
//! [adr-257]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
//! [report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/two-logs.md

use serde::{Deserialize, Serialize};

use crate::log::FlightLog;
use crate::readings::interval::{Interval, Shares};
use crate::readings::{self, Provenance, Reading, Readings, Source, Withheld};
use crate::smoother::{self, INTERVAL_Z, Options};

/// How the clocks are lined up, in the result itself rather than implied.
pub const METHOD: &str = "shared liftoff";

/// The clock offset's shares of its bound ([`Offset::bound_s`]), fitted on the two-logger
/// calibration set: its 5% and 95% points of `(offset − truth)/bound`, on 2,000 flights on a seed
/// of their own. `cargo xtask two-logs` fits them again and fails if they differ.
pub const OFFSET_SHARES: Shares = Shares {
    low: -0.0526,
    high: 0.0515,
};

/// What the logs recorded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Flew {
    /// One airframe, the logs being measurements of one flight: each row reports its spread.
    OneAirframe,
    /// One logger to a stage, of a stack that left the pad together: no spread, the readings
    /// being of different bodies.
    Stages,
}

/// A log's place on the timeline.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Placed {
    /// Its liftoff, s on its own clock: the timeline's zero.
    pub zero_s: f64,
    /// The bound on that, s.
    pub bound_s: f64,
    /// Its 90% interval, s on its own clock.
    pub interval_s: Option<Interval>,
}

/// One log's clock against the first's.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Offset {
    /// The offset, s: this log's clock less the first log's at one instant.
    pub offset_s: f64,
    /// The bound on its error, s: the sum of the two liftoffs' bounds.
    pub bound_s: f64,
    /// Its 90% interval, s ([`OFFSET_SHARES`]).
    pub interval_s: Option<Interval>,
}

/// The clock offset of `other` against `first`, from their places.
pub fn offset(first: &Placed, other: &Placed) -> Offset {
    let offset_s = other.zero_s - first.zero_s;
    let bound_s = first.bound_s + other.bound_s;
    Offset {
        offset_s,
        bound_s,
        interval_s: OFFSET_SHARES.interval(offset_s, bound_s),
    }
}

/// A row of a [`SideBySide`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum What {
    /// Apogee's height above the log's own pad, m.
    ApogeeHeight,
    /// Apogee's time, s after the log's liftoff.
    ApogeeTime,
    /// The smoother's apogee height, m.
    SmoothedApogee,
    /// The top vertical speed, m/s.
    TopSpeed,
    /// The top acceleration, m/s².
    TopAcceleration,
    /// Burnout, s after the log's liftoff.
    Burnout,
    /// The flight time, liftoff to landing, s.
    FlightTime,
    /// The rate of descent under the drogue, m/s.
    DrogueRate,
    /// The main's opening, m above the log's own pad.
    MainOpening,
    /// The rate of descent under the main, m/s.
    MainRate,
}

/// One log's cell in a row.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Cell {
    /// What the log read.
    Read(Given),
    /// Why it read nothing, in words with the log's own numbers.
    Withheld {
        /// The words.
        detail: String,
    },
}

/// A reading in a cell.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Given {
    /// The reading.
    pub value: f64,
    /// The bound on its error; `None` for the smoother's apogee, which states a standard
    /// deviation and its interval instead, and the main's opening height, which states none.
    pub bound: Option<f64>,
    /// Its 90% interval, on the same scale; `None` for a time moved onto the timeline, whose
    /// interval on the log's own clock is the log's.
    pub interval: Option<Interval>,
    /// Whether the true value may lie any amount above it (a clipped sensor, a log ending early).
    pub is_floor: bool,
    /// The track it came from; `None` for the smoother's fused apogee.
    pub source: Option<Source>,
    /// Measured, derived or clipped; `None` for the smoother's fused apogee.
    pub provenance: Option<Provenance>,
}

/// One reading across the logs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Row {
    /// Which reading.
    pub what: What,
    /// Each log's, in the order the logs were given.
    pub cells: Vec<Cell>,
    /// The largest less the least: a description, not a check. `None` for fewer than two logs
    /// reading it whole, as a floor isn't the value, and for [`Flew::Stages`].
    pub spread: Option<f64>,
}

/// The logs' readings on one timeline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SideBySide {
    /// How the clocks were lined up: [`METHOD`].
    pub method: String,
    /// What the logs recorded.
    pub flew: Flew,
    /// Each log's place, in the order given, withheld as its liftoff is.
    pub placed: Vec<Reading<Placed>>,
    /// Each log after the first against the first, withheld as either's liftoff is.
    pub offsets: Vec<Reading<Offset>>,
    /// The rows.
    pub rows: Vec<Row>,
}

/// A log's place, from its readings.
pub fn place(read: &Readings) -> Reading<Placed> {
    match &read.liftoff {
        Reading::Read(liftoff) => Reading::Read(Placed {
            zero_s: liftoff.time_s,
            bound_s: liftoff.bound_s,
            interval_s: liftoff.interval_s,
        }),
        Reading::Withheld(withheld) => Reading::Withheld(withheld.clone()),
    }
}

/// Reads each of `logs` and lays them side by side.
pub fn side_by_side(logs: &[&FlightLog], flew: Flew) -> SideBySide {
    let reads: Vec<Readings> = logs.iter().map(|log| readings::read(log)).collect();
    let pairs: Vec<(&FlightLog, &Readings)> = logs.iter().copied().zip(&reads).collect();
    side_by_side_read(&pairs, flew)
}

/// Lays `logs` side by side, with the readings already taken from each.
pub fn side_by_side_read(logs: &[(&FlightLog, &Readings)], flew: Flew) -> SideBySide {
    let reads: Vec<&Readings> = logs.iter().map(|(_, read)| *read).collect();
    let placed: Vec<Reading<Placed>> = reads.iter().map(|read| place(read)).collect();
    let offsets = placed
        .iter()
        .skip(1)
        .map(|other| match (&placed[0], other) {
            (Reading::Read(first), Reading::Read(other)) => Reading::Read(offset(first, other)),
            (Reading::Withheld(withheld), _) | (_, Reading::Withheld(withheld)) => {
                Reading::Withheld(withheld.clone())
            }
        })
        .collect();
    let smoothed: Vec<Cell> = logs
        .iter()
        .map(|(log, read)| smoothed_apogee(log, read))
        .collect();
    let mut rows = vec![
        row(What::ApogeeHeight, &reads, |read| {
            cell(&read.apogee, |a| Given {
                value: a.height_m,
                bound: Some(a.bound_m),
                interval: a.height_interval_m,
                is_floor: a.is_floor,
                source: Some(a.source),
                provenance: Some(a.provenance),
            })
        }),
        row(What::ApogeeTime, &reads, |read| {
            after_liftoff(read, &read.apogee, |a, liftoff_bound_s| {
                a.time_after_liftoff_s.map(|value| Given {
                    value,
                    bound: Some(a.time_bound_s + liftoff_bound_s),
                    interval: None,
                    is_floor: a.is_floor,
                    source: Some(a.source),
                    provenance: Some(a.provenance),
                })
            })
        }),
    ];
    rows.push(with_spread(What::SmoothedApogee, smoothed));
    rows.extend([
        row(What::TopSpeed, &reads, |read| {
            cell(&read.max_speed, |s| Given {
                value: s.speed_m_s,
                bound: Some(s.bound_m_s),
                interval: if ends_climbing(read) {
                    None
                } else {
                    s.interval_m_s
                },
                is_floor: ends_climbing(read),
                source: Some(s.source),
                provenance: Some(s.provenance),
            })
        }),
        row(What::TopAcceleration, &reads, |read| {
            cell(&read.max_acceleration, |a| Given {
                value: a.acceleration_m_s2,
                bound: Some(a.bound_m_s2),
                interval: a.interval_m_s2,
                is_floor: a.is_floor,
                source: Some(a.source),
                provenance: Some(a.provenance),
            })
        }),
        row(What::Burnout, &reads, |read| {
            after_liftoff(read, &read.burnout, |b, liftoff_bound_s| {
                b.time_after_liftoff_s.map(|value| Given {
                    value,
                    bound: Some(b.bound_s + liftoff_bound_s),
                    interval: None,
                    is_floor: b.is_floor,
                    source: Some(b.source),
                    provenance: Some(b.provenance),
                })
            })
        }),
        row(What::FlightTime, &reads, |read| {
            cell(&read.flight_time, |f| Given {
                value: f.time_s,
                bound: Some(f.bound_s),
                interval: f.interval_s,
                is_floor: false,
                source: Some(f.source),
                provenance: Some(f.provenance),
            })
        }),
        row(What::DrogueRate, &reads, |read| {
            cell(&read.drogue_descent, |d| Given {
                value: d.rate_m_s,
                bound: Some(d.bound_m_s),
                interval: d.interval_m_s,
                is_floor: false,
                source: Some(d.source),
                provenance: Some(d.provenance),
            })
        }),
        row(What::MainOpening, &reads, |read| match &read.main_descent {
            Reading::Withheld(withheld) => Cell::Withheld {
                detail: withheld.detail.clone(),
            },
            Reading::Read(main) => match main.opening {
                Some(opening) => Cell::Read(Given {
                    value: opening.height_m,
                    bound: None,
                    interval: None,
                    is_floor: false,
                    source: Some(main.source),
                    provenance: Some(main.provenance),
                }),
                None => Cell::Withheld {
                    detail: "the descent shows one canopy".to_owned(),
                },
            },
        }),
        row(What::MainRate, &reads, |read| {
            cell(&read.main_descent, |m| Given {
                value: m.rate_m_s,
                bound: Some(m.bound_m_s),
                interval: m.interval_m_s,
                is_floor: false,
                source: Some(m.source),
                provenance: Some(m.provenance),
            })
        }),
    ]);
    if flew == Flew::Stages {
        for row in &mut rows {
            row.spread = None;
        }
    }
    SideBySide {
        method: METHOD.to_owned(),
        flew,
        placed,
        offsets,
        rows,
    }
}

/// `reading`'s cell.
fn cell<T>(reading: &Reading<T>, given: impl FnOnce(&T) -> Given) -> Cell {
    match reading {
        Reading::Read(value) => Cell::Read(given(value)),
        Reading::Withheld(Withheld { detail, .. }) => Cell::Withheld {
            detail: detail.clone(),
        },
    }
}

/// `reading`'s cell as a time after liftoff, withheld when liftoff is: a time on the log's clock
/// sits on no timeline without it.
fn after_liftoff<T>(
    read: &Readings,
    reading: &Reading<T>,
    given: impl FnOnce(&T, f64) -> Option<Given>,
) -> Cell {
    let liftoff = match &read.liftoff {
        Reading::Read(liftoff) => liftoff,
        Reading::Withheld(Withheld { detail, .. }) => {
            return Cell::Withheld {
                detail: detail.clone(),
            };
        }
    };
    match reading {
        Reading::Withheld(Withheld { detail, .. }) => Cell::Withheld {
            detail: detail.clone(),
        },
        Reading::Read(value) => match given(value, liftoff.bound_s) {
            Some(given) => Cell::Read(given),
            None => Cell::Withheld {
                detail: "liftoff is withheld".to_owned(),
            },
        },
    }
}

/// Whether the log ends on the climb, so that its top height is a floor, and what it read of the
/// climb's top speed may be too.
fn ends_climbing(read: &Readings) -> bool {
    read.apogee.value().is_some_and(|apogee| apogee.is_floor)
}

/// The smoother's apogee as a cell.
fn smoothed_apogee(log: &FlightLog, read: &Readings) -> Cell {
    let smoothed = match smoother::smooth_with(log, read, Options::default()) {
        Ok(smoothed) => smoothed,
        Err(withheld) => {
            return Cell::Withheld {
                detail: withheld.detail,
            };
        }
    };
    match smoothed.at(smoothed.apogee_time_s) {
        Some(state) => {
            let half = INTERVAL_Z * state.height_sd_m;
            // Where the log ends on the climb the track's top isn't the rocket's: a floor, its
            // interval a claim about the wrong height.
            let is_floor = ends_climbing(read);
            Cell::Read(Given {
                value: state.height_m,
                bound: None,
                interval: (!is_floor).then_some(Interval {
                    low: state.height_m - half,
                    high: state.height_m + half,
                }),
                is_floor,
                source: None,
                provenance: None,
            })
        }
        None => Cell::Withheld {
            detail: "the smoothed track holds no apogee".to_owned(),
        },
    }
}

/// A row from each log's readings.
fn row(what: What, reads: &[&Readings], cell: impl Fn(&Readings) -> Cell) -> Row {
    with_spread(what, reads.iter().map(|read| cell(read)).collect())
}

/// A row of `cells`, with its spread.
fn with_spread(what: What, cells: Vec<Cell>) -> Row {
    let spread = spread(&cells);
    Row {
        what,
        cells,
        spread,
    }
}

/// The largest whole reading less the least; `None` for fewer than two. A floor isn't the value,
/// so it joins no spread.
fn spread(cells: &[Cell]) -> Option<f64> {
    let values: Vec<f64> = cells
        .iter()
        .filter_map(|cell| match cell {
            Cell::Read(given) if !given.is_floor => Some(given.value),
            _ => None,
        })
        .collect();
    if values.len() < 2 {
        return None;
    }
    let least = values.iter().copied().fold(f64::INFINITY, f64::min);
    let largest = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    Some(largest - least)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::log::{LogFormat, Stated};

    /// A log climbing from a pad at `liftoff_s` on its own clock to `peak_m` and down again, with
    /// each height stepping up by a part in `tenths`/10 m, so that two logs differ.
    fn flown(liftoff_s: f64, peak_m: f64) -> FlightLog {
        let mut time_s = Vec::new();
        let mut altitude_m = Vec::new();
        for k in 0..6_000 {
            let since = f64::from(k) * 0.05 - 10.0;
            let t = since + liftoff_s;
            let climb = 10.0;
            let height = if since < 0.0 {
                0.0
            } else if since < climb {
                peak_m * (1.0 - (1.0 - since / climb).powi(2))
            } else {
                (peak_m - 12.0 * (since - climb)).max(0.0)
            };
            time_s.push(t);
            altitude_m.push(height);
        }
        FlightLog {
            format: LogFormat::GenericTable,
            logger: "invented".to_owned(),
            serial_number: None,
            firmware: None,
            flight_number: None,
            stated: Stated::default(),
            time_s,
            altitude_m,
            vertical_speed_m_s: None,
            temperature_k: None,
            battery_v: None,
            channels: Vec::new(),
            events: Vec::new(),
            notes: Vec::new(),
        }
    }

    fn zero_of(side: &SideBySide, log: usize) -> f64 {
        side.placed[log].value().unwrap().zero_s
    }

    #[test]
    fn the_offset_is_the_clocks_difference_with_the_bounds_added() {
        let (a, b) = (flown(3.0, 500.0), flown(103.0, 510.0));
        let side = side_by_side(&[&a, &b], Flew::OneAirframe);
        assert_eq!(side.method, "shared liftoff");
        let offset = side.offsets[0].value().unwrap();
        assert_eq!(offset.offset_s, zero_of(&side, 1) - zero_of(&side, 0));
        assert!((offset.offset_s - 100.0).abs() <= offset.bound_s);
        let (first, second) = (
            side.placed[0].value().unwrap(),
            side.placed[1].value().unwrap(),
        );
        assert_eq!(offset.bound_s, first.bound_s + second.bound_s);
        let interval = offset.interval_s.unwrap();
        assert_eq!(
            interval.low,
            offset.offset_s - OFFSET_SHARES.high * offset.bound_s
        );
        assert_eq!(
            interval.high,
            offset.offset_s - OFFSET_SHARES.low * offset.bound_s
        );
    }

    #[test]
    fn swapping_the_logs_turns_the_offset_round_and_keeps_every_spread() {
        let (a, b) = (flown(3.0, 500.0), flown(103.0, 510.0));
        let ab = side_by_side(&[&a, &b], Flew::OneAirframe);
        let ba = side_by_side(&[&b, &a], Flew::OneAirframe);
        let (ab_offset, ba_offset) = (
            ab.offsets[0].value().unwrap(),
            ba.offsets[0].value().unwrap(),
        );
        assert_eq!(ab_offset.offset_s, -ba_offset.offset_s);
        assert_eq!(ab_offset.bound_s, ba_offset.bound_s);
        for (x, y) in ab.rows.iter().zip(&ba.rows) {
            assert_eq!(x.spread, y.spread, "{:?}", x.what);
        }
    }

    #[test]
    fn each_log_keeps_its_own_reading_and_the_spread_is_the_largest_less_the_least() {
        let (a, b) = (flown(3.0, 500.0), flown(103.0, 520.0));
        let side = side_by_side(&[&a, &b], Flew::OneAirframe);
        let read_a = readings::read(&a);
        let read_b = readings::read(&b);
        let apogee = side
            .rows
            .iter()
            .find(|row| row.what == What::ApogeeHeight)
            .unwrap();
        let height = |read: &Readings| read.apogee.value().unwrap().height_m;
        assert_eq!(
            apogee.cells,
            [
                cell(&read_a.apogee, |a| Given {
                    value: a.height_m,
                    bound: Some(a.bound_m),
                    interval: a.height_interval_m,
                    is_floor: a.is_floor,
                    source: Some(a.source),
                    provenance: Some(a.provenance),
                }),
                cell(&read_b.apogee, |a| Given {
                    value: a.height_m,
                    bound: Some(a.bound_m),
                    interval: a.height_interval_m,
                    is_floor: a.is_floor,
                    source: Some(a.source),
                    provenance: Some(a.provenance),
                }),
            ]
        );
        assert_eq!(apogee.spread, Some(height(&read_b) - height(&read_a)));
    }

    #[test]
    fn nothing_in_the_result_is_a_mean() {
        let (a, b) = (flown(3.0, 500.0), flown(103.0, 520.0));
        let side = side_by_side(&[&a, &b], Flew::OneAirframe);
        let json = serde_json::to_string(&side).unwrap();
        for word in ["mean", "average", "fused", "weighted", "combined"] {
            assert!(!json.contains(word), "{word} in {json}");
        }
        // Nor is any value in it the two logs' mean.
        let apogee = side
            .rows
            .iter()
            .find(|row| row.what == What::ApogeeHeight)
            .unwrap();
        let (Cell::Read(first), Cell::Read(second)) = (&apogee.cells[0], &apogee.cells[1]) else {
            panic!("both logs read an apogee: {apogee:?}");
        };
        assert_ne!(first.value, second.value);
    }

    #[test]
    fn stages_report_no_spread_because_their_readings_are_of_different_bodies() {
        let (a, b) = (flown(3.0, 500.0), flown(103.0, 900.0));
        let side = side_by_side(&[&a, &b], Flew::Stages);
        assert!(side.rows.iter().all(|row| row.spread.is_none()));
        // Their clocks are still lined up.
        assert!(side.offsets[0].value().is_some());
    }

    #[test]
    fn a_log_with_no_liftoff_sits_on_no_timeline_and_says_why() {
        let a = flown(3.0, 500.0);
        let mut none = flown(103.0, 500.0);
        none.time_s.truncate(2);
        none.altitude_m.truncate(2);
        let side = side_by_side(&[&a, &none], Flew::OneAirframe);
        assert!(side.placed[0].value().is_some());
        let reason = side.placed[1].reason().unwrap();
        assert_eq!(side.offsets[0].reason(), Some(reason));
        // Whichever side is missing it, and the time rows go with it.
        let turned = side_by_side(&[&none, &a], Flew::OneAirframe);
        assert_eq!(turned.offsets[0].reason(), Some(reason));
        let apogee_time = side
            .rows
            .iter()
            .find(|row| row.what == What::ApogeeTime)
            .unwrap();
        assert!(matches!(apogee_time.cells[1], Cell::Withheld { .. }));
        assert_eq!(apogee_time.spread, None);
    }

    #[test]
    fn a_log_cut_on_the_climb_gives_floors_in_every_row_its_top_stands_for() {
        let whole = flown(3.0, 500.0);
        let mut cut = whole.clone();
        // Ending mid-climb, four seconds after liftoff.
        let keep = cut.time_s.iter().position(|t| *t > 3.0 + 4.0).unwrap();
        cut.time_s.truncate(keep);
        cut.altitude_m.truncate(keep);
        let read = readings::read(&cut);
        assert!(
            read.apogee.value().is_some_and(|a| a.is_floor),
            "{:?}",
            read.apogee
        );
        let side = side_by_side(&[&whole, &cut], Flew::OneAirframe);
        for what in [What::ApogeeHeight, What::SmoothedApogee, What::TopSpeed] {
            let row = side.rows.iter().find(|row| row.what == what).unwrap();
            assert_eq!(row.spread, None, "{what:?}: {row:?}");
            let Cell::Read(given) = &row.cells[1] else {
                panic!("{what:?}: {row:?}");
            };
            assert!(given.is_floor, "{what:?}: {given:?}");
            assert_eq!(given.interval, None, "{what:?}: {given:?}");
            // The whole log's reading stands: it says its top whole.
            let Cell::Read(whole) = &row.cells[0] else {
                panic!("{what:?}: {row:?}");
            };
            assert!(!whole.is_floor, "{what:?}: {whole:?}");
        }
    }

    #[test]
    fn a_floor_joins_no_spread() {
        let given = |value, is_floor| {
            Cell::Read(Given {
                value,
                bound: None,
                interval: None,
                is_floor,
                source: None,
                provenance: None,
            })
        };
        assert_eq!(spread(&[given(1.0, false), given(4.0, false)]), Some(3.0));
        assert_eq!(spread(&[given(1.0, true), given(4.0, false)]), None);
        assert_eq!(
            spread(&[given(1.0, false), given(9.0, true), given(4.0, false)]),
            Some(3.0)
        );
    }
}
