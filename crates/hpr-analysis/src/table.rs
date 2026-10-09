//! A Monte Carlo [`Run`] as a table: one row a sample, in index order, and one column an input it
//! drew or a number its flight came to. `hpr mc` writes it as CSV ([`RunTable::csv`]), each
//! number in the shortest form that reads back to the same bits, so a program reading the file
//! gets the run's own numbers.
//!
//! The columns, in order:
//!
//! - `index`, a whole number, then `outcome` (`flown` or `failed`), `failed_at` (`inputs` or `flight`, empty for
//!   a flown sample) and `reason` (the error, empty for a flown sample): [`Sample`], [`Outcome`].
//! - What the sample drew ([`Draw`]): `drag_scale`, `wind_speed_scale`, `wind_turn_rad`,
//!   `rail_elevation_offset_rad` and `rail_azimuth_offset_rad`; then for each stage `n`, counted
//!   from 1 at the nose, `stage_n_dry_mass_scale` and `stage_n_cg_shift_m`; for each motor of the
//!   flown configuration, `motor_n_impulse_scale`, `motor_n_burn_time_scale` and
//!   `motor_n_ejection_delay_offset_s`; for each recovery device, `device_n_deployment_lag_offset_s`.
//! - What a flown sample's flight came to ([`FlightSummary`]), empty for a failed sample or a
//!   number its flight doesn't have: `termination`; `apogee_m` (height above the ground) and
//!   `apogee_time_s`; the peaks' values `rail_exit_speed_m_s`, `max_speed_m_s`, `max_mach`,
//!   `max_dynamic_pressure_pa`, `max_acceleration_m_s2`, `min_static_margin_cal`,
//!   `min_flight_margin_cal` and `max_angle_of_attack_rad`; the landing of the part that keeps
//!   the nose ([`FlightSummary::nose_landing`]), `landing_time_s`,
//!   `landing_east_m`, `landing_north_m`, `landing_distance_m`, `landing_latitude_deg`,
//!   `landing_longitude_deg` and `ground_hit_speed_m_s`; for each part `n` that a separation
//!   dropped, `part_n_landing_time_s`, `part_n_landing_east_m`, `part_n_landing_north_m` and
//!   `part_n_ground_hit_speed_m_s`; and `flags`, the flags the flight raised ([`EnvelopeFlag`]:
//!   the operating envelope's, and one for a rocket unstable under power), by name, separated by
//!   `;`.

use hpr_sim::metrics::Landing;
use hpr_sim::{EnvelopeFlag, FlightSummary};
use serde::{Deserialize, Serialize};

use crate::error::AnalysisError;
use crate::montecarlo::{Draw, FailedAt, Outcome, Run, Sample};

/// One of a draw's lists, by stage, motor or device.
type DrawList = fn(&Draw) -> &Vec<f64>;

/// One number of a landing.
type LandingField = fn(&Landing) -> f64;

/// One column of a [`RunTable`]: its name, with its unit where it has one, and a value for each
/// sample.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Column {
    /// The column's name, such as `apogee_m`.
    pub name: String,
    /// Its values, one a sample in index order.
    pub values: Values,
}

/// A column's values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "values", rename_all = "snake_case")]
pub enum Values {
    /// Whole numbers: the samples' indices.
    Integers(Vec<u64>),
    /// Numbers, `None` where a sample has none.
    Numbers(Vec<Option<f64>>),
    /// Words, empty where a sample has none.
    Text(Vec<String>),
}

/// A run as a table: the module's docs list its columns.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunTable {
    rows: usize,
    columns: Vec<Column>,
}

impl RunTable {
    /// `run` as a table. The lists of a draw (stages, motors, devices) take their lengths from the
    /// first sample, as every sample of a run draws the same; the parts' landings run to the
    /// highest part that landed in any sample.
    pub fn new(run: &Run) -> Self {
        let samples = &run.samples;
        let mut columns = Vec::new();
        columns.push(Column {
            name: "index".to_owned(),
            values: Values::Integers(samples.iter().map(|sample| sample.index).collect()),
        });
        let mut text = |name: &str, value: &dyn Fn(&Sample) -> String| {
            columns.push(Column {
                name: name.to_owned(),
                values: Values::Text(samples.iter().map(value).collect()),
            });
        };
        text("outcome", &|sample| match sample.outcome {
            Outcome::Flown { .. } => "flown".to_owned(),
            Outcome::Failed { .. } => "failed".to_owned(),
        });
        text("failed_at", &|sample| match &sample.outcome {
            Outcome::Flown { .. } => String::new(),
            Outcome::Failed { at, .. } => match at {
                FailedAt::Inputs => "inputs".to_owned(),
                FailedAt::Flight => "flight".to_owned(),
            },
        });
        text("reason", &|sample| match &sample.outcome {
            Outcome::Flown { .. } => String::new(),
            Outcome::Failed { reason, .. } => reason.clone(),
        });

        let mut add = |name: String, value: &dyn Fn(&Sample) -> Option<f64>| {
            columns.push(Column {
                name,
                values: Values::Numbers(samples.iter().map(value).collect()),
            });
        };
        let drawn = |pick: fn(&Draw) -> f64| move |sample: &Sample| Some(pick(&sample.draw));
        add("drag_scale".to_owned(), &drawn(|d| d.drag_scale));
        add(
            "wind_speed_scale".to_owned(),
            &drawn(|d| d.wind_speed_scale),
        );
        add("wind_turn_rad".to_owned(), &drawn(|d| d.wind_turn_rad));
        add(
            "rail_elevation_offset_rad".to_owned(),
            &drawn(|d| d.rail_elevation_offset_rad),
        );
        add(
            "rail_azimuth_offset_rad".to_owned(),
            &drawn(|d| d.rail_azimuth_offset_rad),
        );
        let lists: [(&str, &str, DrawList); 6] = [
            ("stage", "dry_mass_scale", |d| &d.dry_mass_scale),
            ("stage", "cg_shift_m", |d| &d.cg_shift_m),
            ("motor", "impulse_scale", |d| &d.impulse_scale),
            ("motor", "burn_time_scale", |d| &d.burn_time_scale),
            ("motor", "ejection_delay_offset_s", |d| {
                &d.ejection_delay_offset_s
            }),
            ("device", "deployment_lag_offset_s", |d| {
                &d.deployment_lag_offset_s
            }),
        ];
        // Stage by stage, each stage's two lists together; then motor by motor.
        for kind in ["stage", "motor", "device"] {
            let of_kind: Vec<_> = lists.iter().filter(|(k, _, _)| *k == kind).collect();
            let count = of_kind
                .first()
                .and_then(|(_, _, list)| samples.first().map(|s| list(&s.draw).len()))
                .unwrap_or(0);
            for copy in 0..count {
                for (_, name, list) in &of_kind {
                    add(format!("{kind}_{}_{name}", copy + 1), &|sample| {
                        list(&sample.draw).get(copy).copied()
                    });
                }
            }
        }

        columns.push(Column {
            name: "termination".to_owned(),
            values: Values::Text(
                samples
                    .iter()
                    .map(|sample| {
                        sample.summary().map_or_else(String::new, |s| {
                            snake_name(&format!("{:?}", s.termination))
                        })
                    })
                    .collect(),
            ),
        });
        let mut flown = |name: &str, value: fn(&FlightSummary) -> Option<f64>| {
            columns.push(Column {
                name: name.to_owned(),
                values: Values::Numbers(
                    samples
                        .iter()
                        .map(|sample| sample.summary().and_then(value))
                        .collect(),
                ),
            });
        };
        flown("apogee_m", |s| {
            s.apogee.as_ref().map(|a| a.height_above_ground_m)
        });
        flown("apogee_time_s", |s| s.apogee.as_ref().map(|a| a.time_s));
        flown("rail_exit_speed_m_s", |s| {
            s.rail_exit_speed_m_s.as_ref().map(|p| p.value)
        });
        flown("max_speed_m_s", |s| {
            s.max_speed_m_s.as_ref().map(|p| p.value)
        });
        flown("max_mach", |s| s.max_mach.as_ref().map(|p| p.value));
        flown("max_dynamic_pressure_pa", |s| {
            s.max_dynamic_pressure_pa.as_ref().map(|p| p.value)
        });
        flown("max_acceleration_m_s2", |s| {
            s.max_acceleration_m_s2.as_ref().map(|p| p.value)
        });
        flown("min_static_margin_cal", |s| {
            s.min_static_margin_cal.as_ref().map(|p| p.value)
        });
        flown("min_flight_margin_cal", |s| {
            s.min_flight_margin_cal.as_ref().map(|p| p.value)
        });
        flown("max_angle_of_attack_rad", |s| {
            s.max_angle_of_attack_rad.as_ref().map(|p| p.value)
        });
        flown("landing_time_s", |s| s.nose_landing().map(|l| l.time_s));
        flown("landing_east_m", |s| s.nose_landing().map(|l| l.east_m));
        flown("landing_north_m", |s| s.nose_landing().map(|l| l.north_m));
        flown("landing_distance_m", |s| {
            s.nose_landing().map(|l| l.distance_m)
        });
        flown("landing_latitude_deg", |s| {
            s.nose_landing().map(|l| l.latitude_deg)
        });
        flown("landing_longitude_deg", |s| {
            s.nose_landing().map(|l| l.longitude_deg)
        });
        flown("ground_hit_speed_m_s", |s| {
            s.nose_landing().map(|l| l.ground_hit_speed_m_s)
        });

        let parts = samples
            .iter()
            .filter_map(Sample::summary)
            .flat_map(|s| s.body_landings.iter().filter_map(|l| l.body))
            .filter(|body| *body > 0)
            .max()
            .unwrap_or(0);
        let fields: [(&str, LandingField); 4] = [
            ("landing_time_s", |l| l.time_s),
            ("landing_east_m", |l| l.east_m),
            ("landing_north_m", |l| l.north_m),
            ("ground_hit_speed_m_s", |l| l.ground_hit_speed_m_s),
        ];
        for part in 1..=parts {
            for (name, field) in fields {
                columns.push(Column {
                    name: format!("part_{part}_{name}"),
                    values: Values::Numbers(
                        samples
                            .iter()
                            .map(|sample| {
                                let landing = sample
                                    .summary()?
                                    .body_landings
                                    .iter()
                                    .find(|l| l.body == Some(part))?;
                                Some(field(landing))
                            })
                            .collect(),
                    ),
                });
            }
        }
        columns.push(Column {
            name: "flags".to_owned(),
            values: Values::Text(
                samples
                    .iter()
                    .map(|sample| {
                        sample.summary().map_or_else(String::new, |s| {
                            s.envelope_flags()
                                .iter()
                                .map(flag_name)
                                .collect::<Vec<_>>()
                                .join(";")
                        })
                    })
                    .collect(),
            ),
        });
        Self {
            rows: samples.len(),
            columns,
        }
    }

    /// The number of rows: the run's samples.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// The columns, in the module docs' order.
    pub fn columns(&self) -> &[Column] {
        &self.columns
    }

    /// The column called `name`, if there is one.
    pub fn column(&self, name: &str) -> Option<&Column> {
        self.columns.iter().find(|column| column.name == name)
    }

    /// The table as CSV (RFC 4180): a header of the columns in words with their units in
    /// brackets, as a recording's ([`hpr_sim::export::csv_header`]: `apogee [m]` for the column
    /// `apogee_m`), then one line a sample, lines ending in CRLF. A number is written in the shortest form that reads back to the same
    /// bits (Rust's `{:?}`, which Python's `float` and every IEEE-correct reader read exactly);
    /// a missing number is an empty cell. Words are quoted where they hold a comma, a quote or
    /// a line break, a quote doubled.
    ///
    /// # Errors
    ///
    /// [`AnalysisError::Domain`] for a number that isn't finite.
    pub fn csv(&self) -> Result<String, AnalysisError> {
        let header: Vec<String> = self
            .columns
            .iter()
            .map(|c| quoted(&hpr_sim::export::csv_header(&c.name)))
            .collect();
        let mut out = header.join(",");
        out.push_str("\r\n");
        for row in 0..self.rows {
            let mut cells = Vec::with_capacity(self.columns.len());
            for column in &self.columns {
                cells.push(match &column.values {
                    Values::Numbers(values) => match values.get(row).copied().flatten() {
                        Some(value) if value.is_finite() => format!("{value:?}"),
                        Some(value) => {
                            return Err(AnalysisError::Domain {
                                what: "number in a run's table",
                                value,
                            });
                        }
                        None => String::new(),
                    },
                    Values::Integers(values) => {
                        values.get(row).map_or_else(String::new, u64::to_string)
                    }
                    Values::Text(values) => values.get(row).map_or_else(String::new, |v| quoted(v)),
                });
            }
            out.push_str(&cells.join(","));
            out.push_str("\r\n");
        }
        Ok(out)
    }
}

/// `text` as a CSV cell: quoted, its quotes doubled, where it holds a comma, a quote or a line
/// break (RFC 4180 §2).
fn quoted(text: &str) -> String {
    if text.contains([',', '"', '\r', '\n']) {
        format!("\"{}\"", text.replace('"', "\"\""))
    } else {
        text.to_owned()
    }
}

/// A Rust name in snake case: `GroundHit` is `ground_hit`. Only the name before any `{` or `(`
/// of a variant's fields is kept.
fn snake_name(debug: &str) -> String {
    let name = debug
        .split(|c: char| c == '{' || c == '(' || c.is_whitespace())
        .next()
        .unwrap_or_default();
    let mut out = String::with_capacity(name.len() + 4);
    for (i, c) in name.chars().enumerate() {
        if c.is_ascii_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

/// A flag's name, as it is serialized.
fn flag_name(flag: &EnvelopeFlag) -> String {
    flag.name().to_owned()
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        reason = "tests state their expectations by unwrapping and panicking"
    )]

    use super::*;
    use crate::montecarlo::tests::{SEED, every_dispersion, nominal};
    use crate::montecarlo::{Dispersion, MonteCarlo};

    /// The cells of RFC 4180 `text`, line by line: quoted cells unquoted, their quotes undoubled.
    fn cells(text: &str) -> Vec<Vec<String>> {
        let mut lines = Vec::new();
        let mut line = Vec::new();
        let mut cell = String::new();
        let mut chars = text.chars().peekable();
        let mut quoted = false;
        while let Some(c) = chars.next() {
            match (quoted, c) {
                (true, '"') if chars.peek() == Some(&'"') => {
                    chars.next();
                    cell.push('"');
                }
                (true, '"') => quoted = false,
                (true, c) => cell.push(c),
                (false, '"') => quoted = true,
                (false, ',') => line.push(std::mem::take(&mut cell)),
                (false, '\r') => {
                    assert_eq!(chars.next(), Some('\n'), "a line ends in CRLF");
                    line.push(std::mem::take(&mut cell));
                    lines.push(std::mem::take(&mut line));
                }
                (false, c) => cell.push(c),
            }
        }
        assert!(cell.is_empty() && line.is_empty(), "the text ends in CRLF");
        lines
    }

    /// A run with flown and failed samples, its every dispersion on.
    fn run() -> Run {
        let dispersion = Dispersion {
            dry_mass_sd_fraction: 0.6,
            ..every_dispersion()
        };
        MonteCarlo::new(nominal(), dispersion)
            .unwrap()
            .run(SEED, 16)
    }

    /// M4.6a's bullet: every number of the run reads back from the CSV to the same bits, a
    /// missing one is an empty cell, and every word reads back as it was, quoted or not.
    #[test]
    fn the_csv_reads_back_to_the_runs_bits() {
        let run = run();
        let failed = run.failed().count();
        assert!(failed > 0 && failed < 16, "{failed} failed");
        let table = RunTable::new(&run);
        assert_eq!(table.rows(), 16);
        let lines = cells(&table.csv().unwrap());
        assert_eq!(lines.len(), 17);
        let names: Vec<&str> = table.columns().iter().map(|c| c.name.as_str()).collect();
        // The header gives each column in words with its unit in brackets, as the product
        // system's `data.md` writes one; written out by hand, so a unit read wrongly fails.
        assert_eq!(
            lines[0].join(","),
            "index,outcome,failed at,reason,drag scale,wind speed scale,wind turn [rad],\
             rail elevation offset [rad],rail azimuth offset [rad],stage 1 dry mass scale,\
             stage 1 cg shift [m],motor 1 impulse scale,motor 1 burn time scale,\
             motor 1 ejection delay offset [s],device 1 deployment lag offset [s],termination,\
             apogee [m],apogee time [s],rail exit speed [m/s],max speed [m/s],max mach,\
             max dynamic pressure [Pa],max acceleration [m/s^2],min static margin [cal],\
             min flight margin [cal],max angle of attack [rad],landing time [s],landing east [m],\
             landing north [m],landing distance [m],landing latitude [deg],\
             landing longitude [deg],ground hit speed [m/s],flags"
        );
        let headers: Vec<String> = names
            .iter()
            .map(|name| hpr_sim::export::csv_header(name))
            .collect();
        assert_eq!(lines[0], headers);
        for (row, line) in lines[1..].iter().enumerate() {
            assert_eq!(line.len(), names.len());
            for (column, cell) in table.columns().iter().zip(line) {
                match &column.values {
                    Values::Numbers(values) => match values[row] {
                        Some(value) => assert_eq!(
                            cell.parse::<f64>().unwrap().to_bits(),
                            value.to_bits(),
                            "{} of row {row}",
                            column.name
                        ),
                        None => assert!(cell.is_empty(), "{} of row {row}", column.name),
                    },
                    Values::Integers(values) => assert_eq!(*cell, values[row].to_string()),
                    Values::Text(values) => assert_eq!(*cell, values[row], "{}", column.name),
                }
            }
        }
        // The values are the run's own.
        let number = |name: &str, row: usize| match &table.column(name).unwrap().values {
            Values::Numbers(values) => values[row],
            Values::Text(_) | Values::Integers(_) => panic!("{name} isn't numbers"),
        };
        let word = |name: &str, row: usize| match &table.column(name).unwrap().values {
            Values::Text(values) => values[row].clone(),
            Values::Numbers(_) | Values::Integers(_) => panic!("{name} isn't words"),
        };
        for (row, sample) in run.samples.iter().enumerate() {
            assert_eq!(
                table.column("index").unwrap().values,
                Values::Integers((0..16).collect())
            );
            assert_eq!(sample.index, row as u64);
            let draw = &sample.draw;
            assert_eq!(number("drag_scale", row), Some(draw.drag_scale));
            assert_eq!(number("wind_turn_rad", row), Some(draw.wind_turn_rad));
            assert_eq!(
                number("stage_1_dry_mass_scale", row),
                Some(draw.dry_mass_scale[0])
            );
            assert_eq!(number("stage_1_cg_shift_m", row), Some(draw.cg_shift_m[0]));
            assert_eq!(
                number("motor_1_burn_time_scale", row),
                Some(draw.burn_time_scale[0])
            );
            assert_eq!(
                number("device_1_deployment_lag_offset_s", row),
                Some(draw.deployment_lag_offset_s[0])
            );
            match &sample.outcome {
                Outcome::Flown { summary } => {
                    assert_eq!(word("outcome", row), "flown");
                    assert_eq!(word("failed_at", row), "");
                    assert_eq!(word("termination", row), "ground_hit");
                    let apogee = summary.apogee.as_ref().unwrap();
                    assert_eq!(number("apogee_m", row), Some(apogee.height_above_ground_m));
                    let landing = summary.landing.as_ref().unwrap();
                    assert_eq!(number("landing_east_m", row), Some(landing.east_m));
                    assert_eq!(number("landing_north_m", row), Some(landing.north_m));
                    assert_eq!(
                        number("ground_hit_speed_m_s", row),
                        Some(landing.ground_hit_speed_m_s)
                    );
                }
                Outcome::Failed { at, reason } => {
                    assert_eq!(word("outcome", row), "failed");
                    assert_eq!(word("failed_at", row), "flight");
                    assert_eq!(*at, FailedAt::Flight);
                    assert_eq!(word("reason", row), *reason);
                    assert_eq!(number("apogee_m", row), None);
                    assert_eq!(word("termination", row), "");
                }
            }
        }
        // One stage, one motor, one device, and no part dropped.
        assert_eq!(names.iter().filter(|n| n.starts_with("stage_")).count(), 2);
        assert!(!names.iter().any(|n| n.starts_with("part_")));
    }

    /// A cell as the table should hold it, worked out from the sample's own fields.
    #[derive(Debug, PartialEq)]
    enum Cell {
        Integer(u64),
        Number(Option<u64>),
        Word(String),
    }

    /// Every column of `sample`'s row, by name, read field by field from the sample, not through
    /// the table's code; `parts` is how many dropped parts the run has.
    fn expected_row(sample: &Sample, parts: usize) -> Vec<(String, Cell)> {
        use hpr_sim::Termination;
        let number = |value: Option<f64>| Cell::Number(value.map(f64::to_bits));
        let word = |text: &str| Cell::Word(text.to_owned());
        let draw = &sample.draw;
        let summary = sample.summary();
        let (outcome, failed_at, reason) = match &sample.outcome {
            Outcome::Flown { .. } => ("flown", "", String::new()),
            Outcome::Failed { at, reason } => (
                "failed",
                match at {
                    FailedAt::Inputs => "inputs",
                    FailedAt::Flight => "flight",
                },
                reason.clone(),
            ),
        };
        let mut row = vec![
            ("index".to_owned(), Cell::Integer(sample.index)),
            ("outcome".to_owned(), word(outcome)),
            ("failed_at".to_owned(), word(failed_at)),
            ("reason".to_owned(), Cell::Word(reason)),
            ("drag_scale".to_owned(), number(Some(draw.drag_scale))),
            (
                "wind_speed_scale".to_owned(),
                number(Some(draw.wind_speed_scale)),
            ),
            ("wind_turn_rad".to_owned(), number(Some(draw.wind_turn_rad))),
            (
                "rail_elevation_offset_rad".to_owned(),
                number(Some(draw.rail_elevation_offset_rad)),
            ),
            (
                "rail_azimuth_offset_rad".to_owned(),
                number(Some(draw.rail_azimuth_offset_rad)),
            ),
        ];
        for (i, (mass, cg)) in draw.dry_mass_scale.iter().zip(&draw.cg_shift_m).enumerate() {
            row.push((
                format!("stage_{}_dry_mass_scale", i + 1),
                number(Some(*mass)),
            ));
            row.push((format!("stage_{}_cg_shift_m", i + 1), number(Some(*cg))));
        }
        for i in 0..draw.impulse_scale.len() {
            row.push((
                format!("motor_{}_impulse_scale", i + 1),
                number(Some(draw.impulse_scale[i])),
            ));
            row.push((
                format!("motor_{}_burn_time_scale", i + 1),
                number(Some(draw.burn_time_scale[i])),
            ));
            row.push((
                format!("motor_{}_ejection_delay_offset_s", i + 1),
                number(Some(draw.ejection_delay_offset_s[i])),
            ));
        }
        for (i, lag) in draw.deployment_lag_offset_s.iter().enumerate() {
            row.push((
                format!("device_{}_deployment_lag_offset_s", i + 1),
                number(Some(*lag)),
            ));
        }
        let termination = summary.map_or("", |s| match s.termination {
            Termination::GroundHit => "ground_hit",
            Termination::Separated => "separated",
            other => panic!("{other:?} isn't flown here"),
        });
        row.push(("termination".to_owned(), word(termination)));
        let peak = |pick: fn(&FlightSummary) -> Option<f64>| number(summary.and_then(pick));
        row.push((
            "apogee_m".to_owned(),
            number(summary.and_then(|s| s.apogee.map(|a| a.height_above_ground_m))),
        ));
        row.push((
            "apogee_time_s".to_owned(),
            number(summary.and_then(|s| s.apogee.map(|a| a.time_s))),
        ));
        row.push((
            "rail_exit_speed_m_s".to_owned(),
            peak(|s| s.rail_exit_speed_m_s.as_ref().map(|p| p.value)),
        ));
        row.push((
            "max_speed_m_s".to_owned(),
            peak(|s| s.max_speed_m_s.as_ref().map(|p| p.value)),
        ));
        row.push((
            "max_mach".to_owned(),
            peak(|s| s.max_mach.as_ref().map(|p| p.value)),
        ));
        row.push((
            "max_dynamic_pressure_pa".to_owned(),
            peak(|s| s.max_dynamic_pressure_pa.as_ref().map(|p| p.value)),
        ));
        row.push((
            "max_acceleration_m_s2".to_owned(),
            peak(|s| s.max_acceleration_m_s2.as_ref().map(|p| p.value)),
        ));
        row.push((
            "min_static_margin_cal".to_owned(),
            peak(|s| s.min_static_margin_cal.as_ref().map(|p| p.value)),
        ));
        row.push((
            "min_flight_margin_cal".to_owned(),
            peak(|s| s.min_flight_margin_cal.as_ref().map(|p| p.value)),
        ));
        row.push((
            "max_angle_of_attack_rad".to_owned(),
            peak(|s| s.max_angle_of_attack_rad.as_ref().map(|p| p.value)),
        ));
        // A split with nothing left to burn leaves the flight's landing empty: part 0's is the
        // nose's.
        let landing_of = |body: usize| {
            summary.and_then(|s| s.body_landings.iter().find(|l| l.body == Some(body)))
        };
        let nose = summary.and_then(|s| match s.termination {
            Termination::Separated => landing_of(0),
            _ => s.landing.as_ref(),
        });
        let field = |pick: fn(&Landing) -> f64| number(nose.map(pick));
        row.push(("landing_time_s".to_owned(), field(|l| l.time_s)));
        row.push(("landing_east_m".to_owned(), field(|l| l.east_m)));
        row.push(("landing_north_m".to_owned(), field(|l| l.north_m)));
        row.push(("landing_distance_m".to_owned(), field(|l| l.distance_m)));
        row.push(("landing_latitude_deg".to_owned(), field(|l| l.latitude_deg)));
        row.push((
            "landing_longitude_deg".to_owned(),
            field(|l| l.longitude_deg),
        ));
        row.push((
            "ground_hit_speed_m_s".to_owned(),
            field(|l| l.ground_hit_speed_m_s),
        ));
        for part in 1..=parts {
            let landing = landing_of(part);
            let field = |pick: fn(&Landing) -> f64| number(landing.map(pick));
            row.push((format!("part_{part}_landing_time_s"), field(|l| l.time_s)));
            row.push((format!("part_{part}_landing_east_m"), field(|l| l.east_m)));
            row.push((format!("part_{part}_landing_north_m"), field(|l| l.north_m)));
            row.push((
                format!("part_{part}_ground_hit_speed_m_s"),
                field(|l| l.ground_hit_speed_m_s),
            ));
        }
        let flags: Vec<&str> = summary.map_or_else(Vec::new, |s| {
            let mach = s.max_mach.map_or(0.0, |p| p.value);
            let aoa = s.max_angle_of_attack_rad.map_or(0.0, |p| p.value);
            let powered = s.min_powered_static_margin_cal.map_or(0.0, |p| p.value);
            let moment = s.max_powered_moment_slope_per_rad.map_or(0.0, |p| p.value);
            let mut flags = Vec::new();
            if powered < 0.0 {
                flags.push("unstable_under_power");
            } else if moment > 0.0 {
                flags.push("unstable_without_margin");
            }
            if mach > hpr_sim::envelope::VALIDATED_MACH {
                flags.push("beyond_validated_range");
            }
            if aoa > hpr_sim::envelope::HIGH_ANGLE_OF_ATTACK_RAD {
                flags.push("high_angle_of_attack");
            }
            if mach > hpr_sim::envelope::CORE_BAND_MACH {
                flags.push("outside_core_band");
            }
            flags
        });
        row.push(("flags".to_owned(), Cell::Word(flags.join(";"))));
        row
    }

    /// The table's row `row`, by name.
    fn table_row(table: &RunTable, row: usize) -> Vec<(String, Cell)> {
        table
            .columns()
            .iter()
            .map(|column| {
                let cell = match &column.values {
                    Values::Integers(values) => Cell::Integer(values[row]),
                    Values::Numbers(values) => Cell::Number(values[row].map(f64::to_bits)),
                    Values::Text(values) => Cell::Word(values[row].clone()),
                };
                (column.name.clone(), cell)
            })
            .collect()
    }

    /// M4.6a, found in review: every column holds its own field, in the documented order, on a
    /// run with two stages, two motors, two devices, two dropped parts with their own landings,
    /// a flight split with nothing left to burn (its landing part 0's), envelope flags, and
    /// failed flights. Each part's and each list's values differ, so a swap of two columns fails.
    #[test]
    fn every_column_is_its_own_field() {
        let mut run = run();
        let mut flown = 0;
        for sample in &mut run.samples {
            let draw = &mut sample.draw;
            draw.dry_mass_scale.push(1.11);
            draw.cg_shift_m.push(-0.012);
            draw.impulse_scale.push(0.97);
            draw.burn_time_scale.push(1.04);
            draw.ejection_delay_offset_s.push(0.3);
            draw.deployment_lag_offset_s.push(0.21);
            let Outcome::Flown { summary } = &mut sample.outcome else {
                continue;
            };
            flown += 1;
            let landing = summary.landing.unwrap();
            let part = |body: usize, shift: f64| Landing {
                body: Some(body),
                time_s: landing.time_s + 10.0 * shift,
                east_m: landing.east_m + shift,
                north_m: landing.north_m - 2.0 * shift,
                ground_hit_speed_m_s: landing.ground_hit_speed_m_s + 0.5 * shift,
                ..landing
            };
            summary.body_landings = vec![part(1, 1.0)];
            if flown % 2 == 0 {
                // Split with nothing left to burn: part 0's landing is the nose's, and the
                // second part landed too.
                summary.termination = hpr_sim::Termination::Separated;
                summary.landing = None;
                summary.body_landings = vec![part(0, 3.0), part(1, 1.0), part(2, 2.0)];
            }
            if flown == 1 {
                summary.max_mach.as_mut().unwrap().value = 3.0;
            }
            if flown == 3 {
                summary.max_angle_of_attack_rad.as_mut().unwrap().value = 0.5;
            }
        }
        assert!(flown >= 4, "{flown} flown");
        let table = RunTable::new(&run);
        for (row, sample) in run.samples.iter().enumerate() {
            assert_eq!(table_row(&table, row), expected_row(sample, 2), "row {row}");
        }
        let flags = table.column("flags").unwrap();
        let Values::Text(flags) = &flags.values else {
            panic!("flags aren't words");
        };
        assert!(
            flags
                .iter()
                .any(|f| f == "beyond_validated_range;outside_core_band")
        );
        assert!(flags.iter().any(|f| f == "high_angle_of_attack"));
    }

    /// A cell with a comma, a quote or a line break is quoted, its quotes doubled; a number that
    /// isn't finite is refused, not written.
    #[test]
    fn words_are_quoted_and_non_finite_numbers_refused() {
        assert_eq!(quoted("plain words"), "plain words");
        assert_eq!(quoted("a, b"), "\"a, b\"");
        assert_eq!(quoted("say \"hi\""), "\"say \"\"hi\"\"\"");
        assert_eq!(quoted("two\r\nlines"), "\"two\r\nlines\"");
        let table = RunTable {
            rows: 1,
            columns: vec![Column {
                name: "x".to_owned(),
                values: Values::Numbers(vec![Some(f64::NAN)]),
            }],
        };
        assert!(matches!(
            table.csv(),
            Err(AnalysisError::Domain { what, value }) if what.contains("run's table") && value.is_nan()
        ));
    }

    /// A Rust name in snake case, its fields dropped.
    #[test]
    fn names_are_in_snake_case() {
        assert_eq!(snake_name("GroundHit"), "ground_hit");
        assert_eq!(
            snake_name("BeyondEnvelope { mach: 3.6 }"),
            "beyond_envelope"
        );
        assert_eq!(snake_name("Separated"), "separated");
    }
}
