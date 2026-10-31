//! `hpr analyze` with more than one log: the logs on one timeline by a shared liftoff, read side
//! by side and never averaged (`hpr_flightdata::timeline`).

use std::path::Path;

use hpr::hpr_flightdata::log::FlightLog;
use hpr::hpr_flightdata::readings::{self, Reading, Readings};
use hpr::hpr_flightdata::timeline::{self, Cell, Flew, Offset, SideBySide, What};

use super::{AnalyzeArgs, document, interval, provenance, read_log, reading, source, source_words};
use super::{height_interval, rate_interval, time_interval};
use crate::console::Level;
use crate::output::{
    AnalyzeTogether, ClockOffset, LogReading, ReadingInterval, TimelineCell, TimelineFlew,
    TimelineRow, TimelineWhat,
};
use crate::units::{
    acceleration_bound, acceleration_interval, gravities, meters, meters_bound, seconds_bound,
    speed, speed_bound,
};
use crate::{Failure, Out};

/// Runs `hpr analyze` on `args.log` and `args.with`.
pub(super) fn run(args: &AnalyzeArgs, to: &mut Out<'_>) -> Result<(), Failure> {
    let paths: Vec<&String> = std::iter::once(&args.log).chain(&args.with).collect();
    let each: Vec<AnalyzeArgs> = paths.iter().map(|path| one_log(path)).collect();
    let logs = each
        .iter()
        .map(read_log)
        .collect::<Result<Vec<FlightLog>, Failure>>()?;
    let reads: Vec<Readings> = logs.iter().map(readings::read).collect();
    let pairs: Vec<(&FlightLog, &Readings)> = logs.iter().zip(&reads).collect();
    let flew = if args.stages {
        Flew::Stages
    } else {
        Flew::OneAirframe
    };
    let side = timeline::side_by_side_read(&pairs, flew);
    let together = AnalyzeTogether {
        kind: crate::trust::Kind::Measured,
        trust: crate::trust::logs_together(),
        method: side.method.clone(),
        flew: match side.flew {
            Flew::OneAirframe => TimelineFlew::OneAirframe,
            Flew::Stages => TimelineFlew::Stages,
        },
        logs: each
            .iter()
            .zip(&logs)
            .zip(&reads)
            .map(|((args, log), read)| document(args, log, read))
            .collect(),
        offsets: side
            .offsets
            .iter()
            .map(|offset| reading(offset, clock_offset))
            .collect(),
        rows: side.rows.iter().map(row).collect(),
    };
    let lines: Vec<String> = text_lines(&together, &side)
        .iter()
        .map(|line| crate::printable(line))
        .collect();
    to.emit(&together, |out, diagnostics| {
        out.lines(&lines, &[])?;
        crate::sim_text::trust_lines(&together.trust, out)?;
        for (index, log) in together.logs.iter().enumerate() {
            for note in &log.log.notes {
                diagnostics.line(Level::Note, &format!("log {}: {note}", index + 1))?;
            }
        }
        Ok(())
    })
}

/// The arguments that read one log by its format's own reader.
fn one_log(path: &str) -> AnalyzeArgs {
    AnalyzeArgs {
        log: path.to_owned(),
        with: Vec::new(),
        stages: false,
        mapping: None,
        layout: None,
        sheet: None,
        recording: None,
        fip_header: None,
        print_mapping: false,
        record: false,
    }
}

/// An offset as the document holds it.
fn clock_offset(offset: &Offset) -> ClockOffset {
    ClockOffset {
        offset_s: offset.offset_s,
        bound_s: offset.bound_s,
        interval_s: offset.interval_s.map(interval),
    }
}

/// A row as the document holds it.
fn row(row: &timeline::Row) -> TimelineRow {
    TimelineRow {
        what: match row.what {
            What::ApogeeHeight => TimelineWhat::ApogeeHeight,
            What::ApogeeTime => TimelineWhat::ApogeeTime,
            What::SmoothedApogee => TimelineWhat::SmoothedApogee,
            What::TopSpeed => TimelineWhat::TopSpeed,
            What::TopAcceleration => TimelineWhat::TopAcceleration,
            What::Burnout => TimelineWhat::Burnout,
            What::FlightTime => TimelineWhat::FlightTime,
            What::DrogueRate => TimelineWhat::DrogueRate,
            What::MainOpening => TimelineWhat::MainOpening,
            What::MainRate => TimelineWhat::MainRate,
        },
        cells: row
            .cells
            .iter()
            .map(|cell| match cell {
                Cell::Read(given) => TimelineCell::Read {
                    value: given.value,
                    bound: given.bound,
                    interval: given.interval.map(interval),
                    is_floor: given.is_floor,
                    source: given.source.map(source),
                    provenance: given.provenance.map(provenance),
                },
                Cell::Withheld { detail } => TimelineCell::Withheld {
                    detail: detail.clone(),
                },
            })
            .collect(),
        spread: row.spread,
    }
}

/// A row's name in the text.
fn row_name(what: TimelineWhat) -> &'static str {
    match what {
        TimelineWhat::ApogeeHeight => "apogee, above each log's own pad",
        TimelineWhat::ApogeeTime => "apogee's time, after each log's liftoff",
        TimelineWhat::SmoothedApogee => "smoothed apogee",
        TimelineWhat::TopSpeed => "top speed",
        TimelineWhat::TopAcceleration => "top acceleration",
        TimelineWhat::Burnout => "burnout, after each log's liftoff",
        TimelineWhat::FlightTime => "flight time",
        TimelineWhat::DrogueRate => "drogue descent",
        TimelineWhat::MainOpening => "main's opening, above each log's own pad",
        TimelineWhat::MainRate => "main descent",
    }
}

/// A value of a row, in its unit.
fn value_text(what: TimelineWhat, value: f64) -> String {
    match what {
        TimelineWhat::ApogeeHeight | TimelineWhat::SmoothedApogee | TimelineWhat::MainOpening => {
            meters(value)
        }
        TimelineWhat::TopSpeed | TimelineWhat::DrogueRate | TimelineWhat::MainRate => speed(value),
        TimelineWhat::TopAcceleration => {
            format!("{value:.1} m/s² ({})", gravities(value))
        }
        TimelineWhat::ApogeeTime | TimelineWhat::Burnout | TimelineWhat::FlightTime => {
            format!("{value:.2} s")
        }
    }
}

/// A bound of a row, in its unit.
fn bound_text(what: TimelineWhat, bound: f64) -> String {
    match what {
        TimelineWhat::ApogeeHeight | TimelineWhat::SmoothedApogee | TimelineWhat::MainOpening => {
            meters_bound(bound)
        }
        TimelineWhat::TopSpeed | TimelineWhat::DrogueRate | TimelineWhat::MainRate => {
            speed_bound(bound)
        }
        TimelineWhat::TopAcceleration => format!("{} m/s²", acceleration_bound(bound)),
        TimelineWhat::ApogeeTime | TimelineWhat::Burnout | TimelineWhat::FlightTime => {
            format!("{} s", seconds_bound(bound))
        }
    }
}

/// An interval of a row, in its unit, after what comes before it.
fn interval_text(what: TimelineWhat, interval: Option<ReadingInterval>) -> String {
    match what {
        TimelineWhat::ApogeeHeight | TimelineWhat::SmoothedApogee | TimelineWhat::MainOpening => {
            height_interval(interval)
        }
        TimelineWhat::TopSpeed | TimelineWhat::DrogueRate | TimelineWhat::MainRate => {
            rate_interval(interval)
        }
        TimelineWhat::TopAcceleration => interval
            .map(|i| format!(", 90% interval {}", acceleration_interval(i.low, i.high)))
            .unwrap_or_default(),
        TimelineWhat::ApogeeTime | TimelineWhat::Burnout | TimelineWhat::FlightTime => {
            time_interval(interval)
        }
    }
}

/// The text form.
fn text_lines(together: &AnalyzeTogether, side: &SideBySide) -> Vec<String> {
    let count = together.logs.len();
    let mut lines = vec![match together.flew {
        TimelineFlew::OneAirframe => format!(
            "{count} logs of one flight on one timeline, by a {}: read side by side, never \
             averaged",
            together.method
        ),
        TimelineFlew::Stages => format!(
            "{count} stages' logs of one flight on one timeline, by a {}: their readings are of \
             different bodies, so no spread",
            together.method
        ),
    }];
    for (index, document) in together.logs.iter().enumerate() {
        let log = &document.log;
        let name = Path::new(&log.path).file_name().map_or_else(
            || log.path.clone(),
            |name| name.to_string_lossy().into_owned(),
        );
        let mut about = format!("log {}  {name}: {}", index + 1, log.logger);
        if let Some(serial) = &log.serial_number {
            about.push_str(&format!(", serial {serial}"));
        }
        if let Some(flight) = log.flight_number {
            about.push_str(&format!(", flight {flight}"));
        }
        lines.push(about);
        if let Some(track) = document.method.source {
            lines.push(format!("       readings from {}", source_words(track)));
        }
        lines.push(match &side.placed[index] {
            Reading::Read(placed) => format!(
                "       liftoff {:.2} s on its clock, within {} s{}",
                placed.zero_s,
                seconds_bound(placed.bound_s),
                time_interval(placed.interval_s.map(interval))
            ),
            Reading::Withheld(withheld) => format!(
                "       on no timeline: liftoff withheld: {}",
                withheld.detail
            ),
        });
    }
    lines.push(String::new());
    for (index, offset) in together.offsets.iter().enumerate() {
        let name = format!("log {}'s clock less log 1's", index + 2);
        lines.push(match offset {
            LogReading::Read(offset) => format!(
                "{name}: {:.2} s, within {} s{}",
                offset.offset_s,
                seconds_bound(offset.bound_s),
                time_interval(offset.interval_s)
            ),
            LogReading::Withheld(withheld) => format!("{name}: withheld: {}", withheld.detail),
        });
    }
    // A reason already given for a log isn't printed again: a log with no track says the same of
    // every reading.
    let mut said: Vec<Vec<&str>> = vec![Vec::new(); count];
    if let [first, ..] = together.offsets.as_slice() {
        if let LogReading::Withheld(withheld) = first {
            said.iter_mut()
                .for_each(|log| log.push(withheld.detail.as_str()));
        }
    }
    for row in &together.rows {
        lines.push(String::new());
        lines.push(row_name(row.what).to_owned());
        for (index, cell) in row.cells.iter().enumerate() {
            let text = match cell {
                TimelineCell::Read {
                    value,
                    bound,
                    interval,
                    is_floor,
                    ..
                } => {
                    let within = bound
                        .map(|bound| format!(", within {}", bound_text(row.what, bound)))
                        .unwrap_or_default();
                    let floor = if *is_floor {
                        ", a floor: the true value may lie any amount above it"
                    } else {
                        ""
                    };
                    format!(
                        "{}{within}{}{floor}",
                        value_text(row.what, *value),
                        interval_text(row.what, *interval)
                    )
                }
                TimelineCell::Withheld { detail } if said[index].contains(&detail.as_str()) => {
                    "withheld, as above".to_owned()
                }
                TimelineCell::Withheld { detail } => {
                    said[index].push(detail);
                    format!("withheld: {detail}")
                }
            };
            lines.push(format!("  log {}  {text}", index + 1));
        }
        if let Some(spread) = row.spread {
            lines.push(format!("  spread {}", value_text(row.what, spread)));
        }
    }
    lines
}
