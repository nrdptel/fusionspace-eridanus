//! `hpr compare`: a flight log against a simulation of it (`hpr_forensics::residuals`), each
//! residual the logged value less the simulated one.

use std::path::Path;

use hpr::hpr_flightdata::log::FlightLog;
use hpr::hpr_flightdata::readings;
use hpr::hpr_flightdata::timeline::What;
use hpr::hpr_forensics::residuals::{self, HeightKind, Phase, Residuals, Speed, Track};

use crate::analyze::{self, AnalyzeArgs};
use crate::console::Level;
use crate::output::{
    Compare, ComparePhase, ComparePoint, CompareResiduals, CompareRow, CompareSpeed, HeightReadAs,
    ResidualPhase, TimelineCell, TimelineWhat,
};
use crate::sim::{FlightArgs, Setup, flight_refused};
use crate::units::{meters, seconds_bound, speed};
use crate::{Failure, Out};

/// `hpr compare`'s command line.
#[derive(Debug, clap::Args)]
pub struct CompareArgs {
    /// The flight log: a file in any format HPR Sim reads, or another logger's file read by
    /// --mapping or --layout
    pub log: String,
    /// The design, its motor and the launch, as `hpr sim` takes them. Give the launch day's air
    /// and wind, or the simulation flies in the standard atmosphere with no wind.
    #[command(flatten)]
    pub flight: FlightArgs,
    /// Read the log as a table, by this column mapping: a JSON file, as `hpr analyze
    /// --print-mapping` prints one (schema/cli/analyze-mapping.schema.json)
    #[arg(long, value_name = "FILE", conflicts_with = "layout")]
    pub mapping: Option<String>,
    /// Read the log as binary records, or a serial console's capture of them, by this record
    /// layout: a JSON file (schema/cli/analyze-layout.schema.json)
    #[arg(long, value_name = "FILE")]
    pub layout: Option<String>,
    /// Which sheet of a workbook (.xlsx or .xls) to read, counting from 0
    #[arg(
        long,
        value_name = "N",
        value_parser = crate::typed::number::<usize>(),
        conflicts_with = "layout"
    )]
    pub sheet: Option<usize>,
    /// What the log's height stands for: a barometer's reading, or a height climbed. Without it, a
    /// log's main altitude, or a column you mapped, is taken as a barometer's
    #[arg(long, value_name = "WHAT", value_enum)]
    pub height_reads: Option<HeightReads>,
    /// Which recording to read from a file that holds several, counting from 0
    #[arg(
        long,
        value_name = "N",
        value_parser = crate::typed::number::<usize>(),
        conflicts_with_all = ["mapping", "layout", "sheet"]
    )]
    pub recording: Option<usize>,
}

/// What a log's height stands for, as typed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum HeightReads {
    /// A barometer's reading: the standard atmosphere's altitude of the pressure, less the pad's.
    Barometer,
    /// A height climbed: a GPS receiver's, or an accelerometer's integral.
    HeightClimbed,
}

impl CompareArgs {
    /// What `hpr analyze` needs to read the log.
    fn log_args(&self) -> AnalyzeArgs {
        AnalyzeArgs {
            mapping: self.mapping.clone(),
            layout: self.layout.clone(),
            sheet: self.sheet,
            recording: self.recording,
            ..analyze::together::one_log(&self.log)
        }
    }
}

/// Runs `hpr compare`.
pub(crate) fn run(args: &CompareArgs, to: &mut Out<'_>) -> Result<(), Failure> {
    let log = analyze::read_log(&args.log_args())?;
    let read = readings::read(&log);
    let setup = Setup::read(&args.flight)?;
    let builder = setup.builder(&args.flight);
    let mut track = Track::new();
    builder
        .fly_with(&mut track)
        .map_err(|error| flight_refused(&error, &args.flight))?;
    drop(builder);
    let refused = |error: residuals::ResidualsError| match error {
        residuals::ResidualsError::LogLiftoff(_) => Failure::helped(
            format!("{}: {error}", args.log),
            "`hpr analyze` on the log says why its liftoff can't be read; a log that doesn't \
             show a liftoff can't be put on the simulation's clock",
        ),
        _ => Failure::helped(
            format!("{}: {error}", args.flight.design),
            "`hpr sim` with the same options shows what it flew; the flight must leave the pad, \
             in air that reaches its apogee",
        ),
    };
    let simulated = track.simulated(setup.environment.sim()).map_err(refused)?;
    let stated = args.height_reads.map(|reads| match reads {
        HeightReads::Barometer => HeightKind::Barometric,
        HeightReads::HeightClimbed => HeightKind::Geometric,
    });
    let comparison = residuals::compare(&log, &read, &simulated, stated).map_err(refused)?;

    let notes = notes(args, &log, &setup, stated);
    let document = Compare {
        kind: crate::trust::Kind::Compared,
        trust: crate::trust::compared(),
        log: args.log.clone(),
        logger: log.logger.clone(),
        design: crate::sim::file_name(&args.flight.design),
        configuration: setup.configuration_label.clone(),
        method: comparison.method.clone(),
        height_read_as: match comparison.height_read_as {
            HeightKind::Barometric => HeightReadAs::Barometer,
            HeightKind::Geometric => HeightReadAs::HeightClimbed,
        },
        log_liftoff_s: comparison.log_liftoff_s,
        simulated_liftoff_s: comparison.simulated_liftoff_s,
        clock_bound_s: comparison.clock.bound_s,
        simulated_interval_s: comparison.simulated_interval_s,
        reaches_landing: comparison.reaches_landing,
        height_m: residuals_document(&comparison.height_m),
        speed_m_s: match &comparison.speed_m_s {
            Speed::Read(speed) => CompareSpeed::Read(residuals_document(speed)),
            Speed::Withheld { detail } => CompareSpeed::Withheld {
                detail: detail.clone(),
            },
        },
        rows: comparison.rows.iter().map(row_document).collect(),
        notes,
    };
    let lines: Vec<String> = text_lines(&document)
        .iter()
        .map(|line| crate::printable(line))
        .collect();
    to.emit(&document, |out, diagnostics| {
        out.lines(&lines, &[])?;
        crate::sim_text::trust_lines(&document.trust, out)?;
        document
            .notes
            .iter()
            .try_for_each(|note| diagnostics.line(Level::Note, note))
    })
}

/// The notes that go with the result, in the text and in the document alike.
fn notes(
    args: &CompareArgs,
    log: &FlightLog,
    setup: &Setup,
    stated: Option<HeightKind>,
) -> Vec<String> {
    let mut notes = setup.read.notes.clone();
    notes.extend(log.notes.iter().map(|note| format!("the log: {note}")));
    if args.mapping.is_some() || args.layout.is_some() {
        notes.push(
            "the log was read in the units your mapping or layout states, so its readings' bounds \
             hold only if those are right"
                .to_owned(),
        );
    }
    if stated.is_none() {
        notes.push(
            "a log's main altitude, or a column you mapped, is taken as a barometer's: give \
             --height-reads height-climbed for a GPS receiver's"
                .to_owned(),
        );
    }
    if setup.devices.is_empty() {
        notes.push(
            "the design flies no parachute, so the simulated descent is a fall: its residuals say \
             nothing of the flight's descent"
                .to_owned(),
        );
    }
    if !setup.separations.is_empty() {
        notes.push(
            "the simulation's track is the stack's, then the upper stage's: a log from a booster \
             compares only up to the split"
                .to_owned(),
        );
    }
    notes
}

/// A quantity's residuals as the document holds them.
fn residuals_document(residuals: &Residuals) -> CompareResiduals {
    CompareResiduals {
        phases: residuals
            .phases
            .iter()
            .map(|summary| ComparePhase {
                phase: match summary.phase {
                    Phase::Boost => ResidualPhase::Boost,
                    Phase::Coast => ResidualPhase::Coast,
                    Phase::Ascent => ResidualPhase::Ascent,
                    Phase::Descent => ResidualPhase::Descent,
                    Phase::Flight => ResidualPhase::Flight,
                },
                start_s: summary.start_s,
                end_s: summary.end_s,
                samples: summary.samples,
                mean: summary.mean,
                rms: summary.rms,
                largest: summary.largest,
                largest_at_s: summary.largest_at_s,
            })
            .collect(),
        past_simulation: residuals.past_simulation,
        points: residuals
            .points
            .iter()
            .map(|point| ComparePoint {
                time_after_liftoff_s: point.time_after_liftoff_s,
                logged: point.logged,
                logged_sd: point.logged_sd,
                simulated: point.simulated,
                residual: point.residual,
            })
            .collect(),
    }
}

/// A reading's row as the document holds it.
fn row_document(row: &residuals::Row) -> CompareRow {
    CompareRow {
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
        logged: analyze::together::cell(&row.logged),
        simulated: analyze::together::cell(&row.simulated),
        logged_minus_simulated: row.logged_minus_simulated,
    }
}

/// A phase's name in the text.
fn phase_name(phase: ResidualPhase) -> &'static str {
    match phase {
        ResidualPhase::Boost => "boost",
        ResidualPhase::Coast => "coast",
        ResidualPhase::Ascent => "ascent",
        ResidualPhase::Descent => "descent",
        ResidualPhase::Flight => "flight",
    }
}

/// A row's name in the text.
fn row_name(what: TimelineWhat) -> &'static str {
    match what {
        TimelineWhat::ApogeeHeight => "apogee, above the pad",
        TimelineWhat::ApogeeTime => "apogee's time, after liftoff",
        TimelineWhat::SmoothedApogee => "smoothed apogee",
        TimelineWhat::TopSpeed => "top speed",
        TimelineWhat::TopAcceleration => "top acceleration",
        TimelineWhat::Burnout => "burnout, after liftoff",
        TimelineWhat::FlightTime => "flight time",
        TimelineWhat::DrogueRate => "drogue descent",
        TimelineWhat::MainOpening => "main's opening, above the pad",
        TimelineWhat::MainRate => "main descent",
    }
}

/// The text form.
fn text_lines(document: &Compare) -> Vec<String> {
    let name = Path::new(&document.log).file_name().map_or_else(
        || document.log.clone(),
        |name| name.to_string_lossy().into_owned(),
    );
    let logger = match document.logger.as_str() {
        "unknown" => String::new(),
        logger => format!(" ({logger})"),
    };
    let mut lines = vec![
        format!("{name}{logger} against its simulation"),
        format!("  flown: {}, {}", document.design, document.configuration),
        format!(
            "  clocks: a {}, within {} s; the log's at {:.2} s, the simulation's {:.2} s after \
             ignition",
            document.method,
            seconds_bound(document.clock_bound_s),
            document.log_liftoff_s,
            document.simulated_liftoff_s
        ),
        format!(
            "  the simulation's height is read as {}, as the log's is",
            match document.height_read_as {
                HeightReadAs::Barometer => "a barometer on the pad would read it",
                HeightReadAs::HeightClimbed => "the height climbed",
            }
        ),
        String::new(),
        "Residuals, logged less simulated".to_owned(),
    ];
    residual_lines(&mut lines, "height", &document.height_m, meters_signed);
    match &document.speed_m_s {
        CompareSpeed::Read(residuals) => {
            residual_lines(&mut lines, "speed, up", residuals, speed_signed);
        }
        CompareSpeed::Withheld { detail } => {
            lines.push(format!("  speed: withheld: {detail}"));
        }
    }
    if !document.reaches_landing {
        lines.push("  the log reads no landing, so these stop at its end".to_owned());
    }
    if document.height_m.past_simulation > 0 {
        lines.push(format!(
            "  {} of the log's samples came after the simulated flight had ended",
            document.height_m.past_simulation
        ));
    }
    lines.push(String::new());
    lines.push("Readings, the simulation read as the log is".to_owned());
    for row in &document.rows {
        lines.push(format!("  {}", row_name(row.what)));
        if let (TimelineCell::Withheld { detail }, TimelineCell::Withheld { .. }) =
            (&row.logged, &row.simulated)
        {
            lines.push(format!("    withheld on both: {detail}"));
            continue;
        }
        lines.push(format!(
            "    logged     {}",
            analyze::together::cell_text(row.what, &row.logged)
        ));
        lines.push(format!(
            "    simulated  {}",
            simulated_text(row.what, &row.simulated)
        ));
        if let Some(difference) = row.logged_minus_simulated {
            lines.push(format!(
                "    logged less simulated  {}",
                analyze::together::signed_text(row.what, difference)
            ));
        }
    }
    lines
}

/// A simulated reading: its value only, as the reader's bound on noise-free samples says nothing
/// of the simulation's error.
fn simulated_text(what: TimelineWhat, cell: &TimelineCell) -> String {
    match cell {
        TimelineCell::Read {
            value, is_floor, ..
        } => {
            let at_least = if *is_floor { "at least " } else { "" };
            format!("{at_least}{}", analyze::together::value_text(what, *value))
        }
        TimelineCell::Withheld { detail } => format!("withheld: {detail}"),
    }
}

/// A height's residual, with its sign.
fn meters_signed(value: f64) -> String {
    let sign = if value < 0.0 { "" } else { "+" };
    format!("{sign}{}", meters(value))
}

/// A speed's residual, with its sign.
fn speed_signed(value: f64) -> String {
    let sign = if value < 0.0 { "" } else { "+" };
    format!("{sign}{}", speed(value))
}

/// A quantity's residuals by phase.
fn residual_lines(
    lines: &mut Vec<String>,
    name: &str,
    residuals: &CompareResiduals,
    signed: fn(f64) -> String,
) {
    lines.push(format!("  {name}"));
    for phase in &residuals.phases {
        lines.push(format!(
            "    {:<8} {:.1} to {:.1} s, {} samples",
            phase_name(phase.phase),
            phase.start_s,
            phase.end_s,
            phase.samples
        ));
        lines.push(format!(
            "             mean {}, RMS {}",
            signed(phase.mean),
            signed(phase.rms).trim_start_matches('+')
        ));
        lines.push(format!(
            "             largest {} at {:.2} s",
            signed(phase.largest),
            phase.largest_at_s
        ));
    }
}
