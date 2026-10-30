//! `hpr analyze`: a flight log's readings, taken from the log alone, with no design file and no
//! simulation (ADR-046).
//!
//! It reads the log with `hpr_flightdata`'s reader for its format, whichever of the formats HPR
//! Sim reads it is (`hpr_flightdata::import`), and prints what `hpr_flightdata::readings` takes
//! from it: liftoff, apogee, the top speed, the top acceleration and landing, each with the track
//! it came from, how it was read, the bound its error stays within and, where the reading has one,
//! its 90% interval, or withheld with the reason the log can't support it. What the file states
//! about itself, such as the logger's own apogee, is printed beside them, never in place of them.
//! Below them come `hpr_flightdata::smoother`'s apogee and health card ([ADR-300][adr-300]).
//!
//! A flyer's own logger is read by a file that says how ([ADR-276][adr-276]): a table by a
//! column mapping (`--mapping`), binary records or a serial console's capture of them by a
//! record layout (`--layout`), each a JSON file read by `hpr_flightdata::custom`, as the Python
//! package reads it. `--print-mapping` prints what a table's header states as a mapping to
//! edit, and `--record` prints the record a log reads into, every channel, in place of the
//! readings.
//!
//! [adr-300]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0300-the-smoother-and-its-check.md
//! [adr-276]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0276-the-2026-10-16-own-loggers-and-gpx-in-the-analyzer.md

use std::path::Path;

use hpr::hpr_flightdata::custom;
use hpr::hpr_flightdata::error::LogError;
use hpr::hpr_flightdata::import;
use hpr::hpr_flightdata::log::{FlightLog, Quantity};
use hpr::hpr_flightdata::readings::{
    self, Clip, Descent, Gravity, GravityBy, Method, Provenance, Reading, Readings, Reason, Source,
};
use hpr::hpr_flightdata::smoother::{self, INTERVAL_Z, Options};

use crate::console::Level;
use crate::output::{
    AccelerometerGravity, Analyze, AnalyzeMethod, AnalyzedAccelerometer, AnalyzedChannel,
    AnalyzedLog, ApogeeReading, BurnoutReading, DescentReading, FlightTimeReading,
    GravitySettledBy, HighestSample, LandingReading, LiftoffReading, LogReading, LoggerStated,
    MaxAccelerationReading, MaxSpeedReading, OpeningReading, ReadingClip, ReadingInterval,
    ReadingMethod, ReadingProvenance, ReadingSource, SmootherOutcome, SmootherReading,
    WithheldReading, WithheldReason,
};
use crate::units::{
    acceleration_bound, acceleration_interval, gravities, meters, meters_bound, meters_interval,
    seconds_bound, seconds_interval, speed, speed_bound, speed_interval,
};
use crate::{Failure, Out};

/// The column mapping's schema, as the help lines name it.
const MAPPING_SCHEMA: &str = "schema/cli/analyze-mapping.schema.json";
/// The record layout's schema, as the help lines name it.
const LAYOUT_SCHEMA: &str = "schema/cli/analyze-layout.schema.json";
/// The FIP header file's schema, as the help lines name it.
const FIP_HEADER_SCHEMA: &str = "schema/cli/analyze-fip-header.schema.json";

/// `hpr analyze`'s arguments.
#[derive(Debug, clap::Args)]
pub struct AnalyzeArgs {
    /// The flight log: a file in any format HPR Sim reads, or another logger's file read by
    /// --mapping or --layout
    pub log: String,
    /// Read the log as a table, by this column mapping: a JSON file, as --print-mapping prints
    /// one (schema/cli/analyze-mapping.schema.json)
    #[arg(long, value_name = "FILE", conflicts_with = "layout")]
    pub mapping: Option<String>,
    /// Read the log as binary records, or a serial console's capture of them, by this record
    /// layout: a JSON file (schema/cli/analyze-layout.schema.json)
    #[arg(long, value_name = "FILE")]
    pub layout: Option<String>,
    /// Which sheet of a workbook (.xlsx or .xls) to read, counting from 0. Without it, the one
    /// sheet that reads as a log is read. It works with the format's own reader and with
    /// --mapping, --print-mapping and --fip-header
    #[arg(
        long,
        value_name = "N",
        value_parser = crate::typed::number::<usize>(),
        conflicts_with = "layout"
    )]
    pub sheet: Option<usize>,
    /// Which recording to read from a file that holds several, such as an Entacore AIM raw dump,
    /// counting from 0
    #[arg(
        long,
        value_name = "N",
        value_parser = crate::typed::number::<usize>(),
        conflicts_with_all = ["mapping", "layout", "fip_header", "sheet"]
    )]
    pub recording: Option<usize>,
    /// Read a Featherweight Raven's FIP export pasted into a workbook without its header line.
    /// This JSON file states the missing header (schema/cli/analyze-fip-header.schema.json)
    #[arg(long, value_name = "FILE", conflicts_with_all = ["mapping", "layout"])]
    pub fip_header: Option<String>,
    /// Print what the table's header states, as a column mapping to edit and give back with
    /// --mapping, and read nothing more. The header must name a clock with a unit of time, such
    /// as `Time (s)`
    #[arg(long, conflicts_with_all = ["mapping", "layout", "record", "recording", "fip_header"])]
    pub print_mapping: bool,
    /// Print every channel of the log in SI units (degrees for angles), as one JSON document
    /// (schema/cli/analyze-record.schema.json), in place of the readings
    #[arg(long)]
    pub record: bool,
}

/// Runs `hpr analyze`.
pub(crate) fn run(args: &AnalyzeArgs, to: &mut Out<'_>) -> Result<(), Failure> {
    if args.print_mapping {
        return print_mapping(args, to);
    }
    let log = read_log(args)?;
    if args.record {
        // The record is JSON with `--json` or without: a file a program reads, its schema
        // committed.
        to.json = true;
        return to.emit(&log, |_, _| Ok(()));
    }
    let read = readings::read(&log);
    let document = document(args, &log, &read);
    // A column's name is the file's text: a control character in it, such as an escape, is
    // shown as `?` rather than reaching the terminal.
    let lines: Vec<String> = text_lines(&document)
        .iter()
        .map(|line| crate::printable(line))
        .collect();
    to.emit(&document, |out, diagnostics| {
        out.lines(&lines, &[])?;
        crate::sim_text::trust_lines(&document.trust, out)?;
        document
            .log
            .notes
            .iter()
            .try_for_each(|note| diagnostics.line(Level::Note, note))
    })
}

/// Prints what the log's header states as a mapping to edit, unstamped so that it reads back as
/// a mapping file; without `--json`, a note on each column kept as written and what to do next
/// follow on standard error.
fn print_mapping(args: &AnalyzeArgs, to: &mut Out<'_>) -> Result<(), Failure> {
    let path = &args.log;
    let bytes = crate::read_file(path)?;
    let (mapping, notes) = custom::header_mapping(&bytes, args.sheet).map_err(|error| {
        Failure::helped(
            format!("{path}: {error}"),
            format!(
                "write the mapping by hand, its fields as {MAPPING_SCHEMA} lists them; the guide's \
                 page on reading your own logger shows one"
            ),
        )
    })?;
    let text = serde_json::to_string_pretty(&mapping)
        .map_err(|error| Failure::Output(std::io::Error::from(error)))?;
    to.emit_file(&hpr::hpr_sim::export::ascii(&text), |diagnostics| {
        for note in &notes {
            diagnostics.line(Level::Note, note)?;
        }
        diagnostics.line(
            Level::Help,
            "save it to a file and check each column's quantity and unit. Set \"primary\" to \
             the column, from 0, that is the barometric altitude above the pad, mapped to \
             altitude_above_ground. Then read the log with --mapping and that file",
        )
    })
}

/// A mapping's or a layout's file, read as its JSON.
fn read_description<T>(
    path: &str,
    parse: fn(&str) -> Result<T, hpr::hpr_flightdata::error::LogError>,
    schema: &str,
) -> Result<T, Failure> {
    let bytes = crate::read_file(path)?;
    let text = custom::json_text(&bytes).ok_or_else(|| {
        Failure::helped(
            format!("{path}: not UTF-8 or UTF-16 text, as a JSON file is"),
            "save the file as UTF-8",
        )
    })?;
    parse(&text).map_err(|error| {
        Failure::helped(
            format!("{path}: {error}"),
            format!("fix what it names; {schema} lists every field"),
        )
    })
}

/// Reads the flight log: by the mapping, the layout or the FIP header given, or else by its
/// format's own reader, a workbook's sheet or a dump's recording as chosen.
fn read_log(args: &AnalyzeArgs) -> Result<FlightLog, Failure> {
    let path = args.log.as_str();
    if let Some(file) = &args.mapping {
        let mapping = read_description(file, custom::mapping_from_json, MAPPING_SCHEMA)?;
        let bytes = crate::read_file(path)?;
        return custom::read_by_mapping(&bytes, &mapping, args.sheet).map_err(|error| {
            Failure::helped(
                format!("{path}: {error}"),
                format!("check {file} against the log: the field or column the error names"),
            )
        });
    }
    if let Some(file) = &args.layout {
        let layout = read_description(file, custom::layout_from_json, LAYOUT_SCHEMA)?;
        let bytes = crate::read_file(path)?;
        return custom::read_by_layout(&bytes, &layout).map_err(|error| {
            Failure::helped(
                format!("{path}: {error}"),
                format!("check {file} against the logger's records: the field the error names"),
            )
        });
    }
    if let Some(file) = &args.fip_header {
        let stated = read_description(file, custom::fip_header_from_json, FIP_HEADER_SCHEMA)?;
        let bytes = crate::read_file(path)?;
        return custom::read_by_fip_header(&bytes, &stated, args.sheet).map_err(|error| {
            let help = match &error {
                LogError::Choice { .. } => {
                    "--sheet N, from 0, chooses the sheet the export is on, and only a \
                     workbook has sheets"
                        .to_owned()
                }
                _ => format!(
                    "check {file} against the export: the header line's columns, each channel's \
                     clock, value and status, and the pasted columns"
                ),
            };
            Failure::helped(format!("{path}: {error}"), help)
        });
    }
    let bytes = crate::read_file(path)?;
    import::read_one(&bytes, args.sheet, args.recording).map_err(|error| {
        let help = match &error {
            LogError::Choice { .. } => {
                "choose a workbook's sheet with --sheet N, or a dump's recording with \
                 --recording N, each from 0. A table on a sheet no reader takes is read by a \
                 column mapping (--mapping) and --sheet"
            }
            LogError::Unrecognized { .. } if bytes.iter().all(u8::is_ascii_whitespace) => {
                "download the log from the logger again with its maker's software: this file \
                 holds no readings"
            }
            LogError::Unrecognized { .. } => {
                "another logger's table is read by a column mapping (--mapping), and \
                 --print-mapping prints what its header states as one to edit. Another logger's \
                 binary records are read by a record layout (--layout). A Raven FIP export pasted \
                 below no header is read by the header you state (--fip-header)"
            }
            LogError::Archive { .. } => "unpack the archive and give the log inside it",
            LogError::Workbook { .. } => {
                "save the sheet as CSV and read that by a column mapping (--mapping)"
            }
            _ => {
                "download the log from the logger again with its maker's software, and give that \
                 file as it was saved"
            }
        };
        Failure::helped(format!("{path}: {error}"), help)
    })
}

/// The output document.
fn document(args: &AnalyzeArgs, log: &FlightLog, read: &Readings) -> Analyze {
    Analyze {
        kind: crate::trust::Kind::Measured,
        // A log read by the flyer's own description may hold no barometer at all, such as a GPS
        // tracker's table, and none of HPR Sim's readings.
        trust: if args.mapping.is_some() || args.layout.is_some() || args.fip_header.is_some() {
            crate::trust::own_log()
        } else {
            crate::trust::log()
        },
        log: AnalyzedLog {
            path: args.log.clone(),
            format: log.format,
            mapping: args.mapping.clone(),
            layout: args.layout.clone(),
            fip_header: args.fip_header.clone(),
            sheet: args.sheet,
            recording: args.recording,
            logger: log.logger.clone(),
            serial_number: log.serial_number.clone(),
            firmware: log.firmware.clone(),
            flight_number: log.flight_number,
            samples: log.time_s.len(),
            first_time_s: log.time_s.first().copied().unwrap_or(0.0),
            last_time_s: log.time_s.last().copied().unwrap_or(0.0),
            channels: log
                .channels
                .iter()
                .map(|channel| AnalyzedChannel {
                    column: channel.column.clone(),
                    quantity: channel.quantity,
                    samples: channel.values.len(),
                })
                .collect(),
            notes: log.notes.clone(),
        },
        stated: LoggerStated {
            apogee_m: log.stated.apogee_m,
            ground_elevation_msl_m: log.stated.ground_elevation_msl_m,
        },
        method: AnalyzeMethod {
            source: read.source.map(source),
            sample_interval_s: read.sample_interval_s,
            step_m: read.step_m,
            median_window_s: read.median_window_s,
            peak_bound_m: read.peak_bound_m,
            altitude_resolution_m: log.format.altitude_resolution_m(),
            pad_altitude_m: read.pad_altitude_m,
            accelerometer: read
                .accelerometer
                .as_ref()
                .map(|accelerometer| AnalyzedAccelerometer {
                    column: accelerometer.column.clone(),
                    gravity: gravity(accelerometer.gravity),
                    gravity_by: gravity_by(accelerometer.gravity_by),
                    turned_over: accelerometer.turned_over,
                    sample_interval_s: accelerometer.sample_interval_s,
                    step_m_s2: accelerometer.step_m_s2,
                }),
        },
        liftoff: reading(&read.liftoff, |liftoff| LiftoffReading {
            time_s: liftoff.time_s,
            bound_s: liftoff.bound_s,
            interval_s: liftoff.interval_s.map(interval),
            source: source(liftoff.source),
            provenance: provenance(liftoff.provenance),
            method: method(liftoff.method),
        }),
        apogee: reading(&read.apogee, |apogee| ApogeeReading {
            time_s: apogee.time_s,
            time_after_liftoff_s: apogee.time_after_liftoff_s,
            time_bound_s: apogee.time_bound_s,
            time_interval_s: apogee.time_interval_s.map(interval),
            height_m: apogee.height_m,
            bound_m: apogee.bound_m,
            height_interval_m: apogee.height_interval_m.map(interval),
            altitude_m: apogee.altitude_m,
            is_floor: apogee.is_floor,
            highest_sample: HighestSample {
                time_s: apogee.highest_sample.time_s,
                altitude_m: apogee.highest_sample.altitude_m,
            },
            source: source(apogee.source),
            provenance: provenance(apogee.provenance),
            method: method(apogee.method),
        }),
        max_speed: reading(&read.max_speed, |speed| MaxSpeedReading {
            speed_m_s: speed.speed_m_s,
            bound_m_s: speed.bound_m_s,
            interval_m_s: speed.interval_m_s.map(interval),
            time_s: speed.time_s,
            height_m: speed.height_m,
            source: source(speed.source),
            provenance: provenance(speed.provenance),
            method: method(speed.method),
        }),
        max_acceleration: reading(&read.max_acceleration, |acceleration| {
            MaxAccelerationReading {
                acceleration_m_s2: acceleration.acceleration_m_s2,
                bound_m_s2: acceleration.bound_m_s2,
                interval_m_s2: acceleration.interval_m_s2.map(interval),
                lower_end_m_s2: acceleration.lower_end_m_s2,
                time_s: acceleration.time_s,
                range_m_s2: acceleration.range_m_s2,
                is_floor: acceleration.is_floor,
                clip: acceleration.clip.map(clip),
                source: source(acceleration.source),
                provenance: provenance(acceleration.provenance),
                method: method(acceleration.method),
            }
        }),
        burnout: reading(&read.burnout, |burnout| BurnoutReading {
            time_s: burnout.time_s,
            bound_s: burnout.bound_s,
            interval_s: burnout.interval_s.map(interval),
            lower_end_s: burnout.lower_end_s,
            time_after_liftoff_s: burnout.time_after_liftoff_s,
            is_floor: burnout.is_floor,
            source: source(burnout.source),
            provenance: provenance(burnout.provenance),
            method: method(burnout.method),
        }),
        landing: reading(&read.landing, |landing| LandingReading {
            time_s: landing.time_s,
            bound_s: landing.bound_s,
            interval_s: landing.interval_s.map(interval),
            descent_time_s: landing.descent_time_s,
            mean_descent_rate_m_s: landing.mean_descent_rate_m_s,
            mean_descent_rate_bound_m_s: landing.mean_descent_rate_bound_m_s,
            mean_descent_rate_interval_m_s: landing.mean_descent_rate_interval_m_s.map(interval),
            source: source(landing.source),
            provenance: provenance(landing.provenance),
            method: method(landing.method),
        }),
        flight_time: reading(&read.flight_time, |flight| FlightTimeReading {
            time_s: flight.time_s,
            bound_s: flight.bound_s,
            interval_s: flight.interval_s.map(interval),
            source: source(flight.source),
            provenance: provenance(flight.provenance),
            method: method(flight.method),
        }),
        drogue_descent: reading(&read.drogue_descent, descent),
        main_descent: reading(&read.main_descent, descent),
        smoother: smoother_outcome(log, read),
    }
}

fn gravity(gravity: Gravity) -> AccelerometerGravity {
    match gravity {
        Gravity::In => AccelerometerGravity::In,
        Gravity::Out => AccelerometerGravity::Out,
        _ => AccelerometerGravity::Other,
    }
}

fn gravity_by(by: GravityBy) -> GravitySettledBy {
    match by {
        GravityBy::Family => GravitySettledBy::Family,
        GravityBy::Pad => GravitySettledBy::Pad,
        GravityBy::Coast => GravitySettledBy::Coast,
        _ => GravitySettledBy::Other,
    }
}

fn descent(descent: &Descent) -> DescentReading {
    DescentReading {
        rate_m_s: descent.rate_m_s,
        bound_m_s: descent.bound_m_s,
        interval_m_s: descent.interval_m_s.map(interval),
        from_s: descent.from_s,
        to_s: descent.to_s,
        from_height_m: descent.from_height_m,
        to_height_m: descent.to_height_m,
        opening: descent.opening.map(|opening| OpeningReading {
            time_s: opening.time_s,
            bound_s: opening.bound_s,
            interval_s: opening.interval_s.map(interval),
            height_m: opening.height_m,
        }),
        source: source(descent.source),
        provenance: provenance(descent.provenance),
        method: method(descent.method),
    }
}

fn reading<A, B>(reading: &Reading<A>, value: impl FnOnce(&A) -> B) -> LogReading<B> {
    match reading {
        Reading::Read(read) => LogReading::Read(value(read)),
        Reading::Withheld(withheld) => LogReading::Withheld(WithheldReading {
            reason: match withheld.reason {
                Reason::TooShort => WithheldReason::TooShort,
                Reason::NoClimb => WithheldReason::NoClimb,
                Reason::StartsOffThePad => WithheldReason::StartsOffThePad,
                Reason::EndsBeforeLanding => WithheldReason::EndsBeforeLanding,
                Reason::FasterThanFreeFall => WithheldReason::FasterThanFreeFall,
                Reason::ImplausibleSpeed => WithheldReason::ImplausibleSpeed,
                Reason::NoisySpeed => WithheldReason::NoisySpeed,
                Reason::NoAccelerometer => WithheldReason::NoAccelerometer,
                Reason::Needs => WithheldReason::Needs,
                Reason::BadRecord => WithheldReason::BadRecord,
                Reason::SampledTooFast => WithheldReason::SampledTooFast,
                Reason::NoHeightTrack => WithheldReason::NoHeightTrack,
                Reason::UnstatedSensor => WithheldReason::UnstatedSensor,
                Reason::SampledTooSlowly => WithheldReason::SampledTooSlowly,
                Reason::StepTooCoarse => WithheldReason::StepTooCoarse,
                Reason::Transonic => WithheldReason::Transonic,
                Reason::ReceiverLag => WithheldReason::ReceiverLag,
                Reason::ReceiverDropout => WithheldReason::ReceiverDropout,
                Reason::SlowerThanFreeCoast => WithheldReason::SlowerThanFreeCoast,
                Reason::ShortPad => WithheldReason::ShortPad,
                Reason::ClockOutOfOrder => WithheldReason::ClockOutOfOrder,
                Reason::DisturbedTop => WithheldReason::DisturbedTop,
                Reason::AxisUnstated => WithheldReason::AxisUnstated,
                Reason::GravityUnstated => WithheldReason::GravityUnstated,
                Reason::DirectionUnstated => WithheldReason::DirectionUnstated,
                Reason::OneCanopy => WithheldReason::OneCanopy,
                Reason::ShortLeg => WithheldReason::ShortLeg,
                _ => WithheldReason::Other,
            },
            detail: withheld.detail.clone(),
        }),
    }
}

/// A reading's 90% interval as the command writes it.
fn interval(interval: readings::Interval) -> ReadingInterval {
    ReadingInterval {
        low: interval.low,
        high: interval.high,
    }
}

fn source(source: Source) -> ReadingSource {
    match source {
        Source::Barometer => ReadingSource::Barometer,
        Source::Pressure => ReadingSource::Pressure,
        Source::AltitudeChannel => ReadingSource::AltitudeChannel,
        Source::Gnss => ReadingSource::Gnss,
        Source::Accelerometer => ReadingSource::Accelerometer,
        _ => ReadingSource::Other,
    }
}

fn provenance(provenance: Provenance) -> ReadingProvenance {
    match provenance {
        Provenance::Measured => ReadingProvenance::Measured,
        Provenance::Derived => ReadingProvenance::Derived,
        Provenance::Clipped => ReadingProvenance::Clipped,
        _ => ReadingProvenance::Other,
    }
}

fn clip(clip: Clip) -> ReadingClip {
    match clip {
        Clip::Range => ReadingClip::Range,
        Clip::Flat => ReadingClip::Flat,
        _ => ReadingClip::Other,
    }
}

fn method(method: Method) -> ReadingMethod {
    match method {
        Method::LastOnThePad => ReadingMethod::LastOnThePad,
        Method::HighestFiltered => ReadingMethod::HighestFiltered,
        Method::SteepestSlope => ReadingMethod::SteepestSlope,
        Method::BackOnThePad => ReadingMethod::BackOnThePad,
        Method::HighestReading => ReadingMethod::HighestReading,
        Method::LeastAfterThrust => ReadingMethod::LeastAfterThrust,
        Method::SpeedPeak => ReadingMethod::SpeedPeak,
        Method::LiftoffToLanding => ReadingMethod::LiftoffToLanding,
        Method::LegChord => ReadingMethod::LegChord,
        _ => ReadingMethod::Other,
    }
}

/// The track a reading came from, in words.
fn source_words(source: ReadingSource) -> &'static str {
    match source {
        ReadingSource::Barometer => "the logger's barometric altitude",
        ReadingSource::Pressure => "the pressure, by the 1976 standard atmosphere",
        ReadingSource::AltitudeChannel => "the barometric altitude channel",
        ReadingSource::Gnss => "the GNSS altitude",
        ReadingSource::Accelerometer => "the accelerometer",
        ReadingSource::Other => "another track",
    }
}

/// What `hpr analyze` prints on standard output without `--json`; the log's notes go to
/// standard error after it, as `note:` lines.
fn text_lines(document: &Analyze) -> Vec<String> {
    let log = &document.log;
    let mut about = vec![log.logger.clone()];
    if let Some(serial) = &log.serial_number {
        about.push(format!("serial {serial}"));
    }
    if let Some(flight) = log.flight_number {
        about.push(format!("flight {flight}"));
    }
    // The file's name, as `hpr convert` prints it: the folder adds nothing a reader needs.
    let name = Path::new(&log.path).file_name().map_or_else(
        || log.path.clone(),
        |name| name.to_string_lossy().into_owned(),
    );
    let by = |file: &str| {
        Path::new(file).file_name().map_or_else(
            || file.to_owned(),
            |name| name.to_string_lossy().into_owned(),
        )
    };
    if let Some(file) = &log.mapping {
        about = vec![format!("read by the mapping {}", by(file))];
    } else if let Some(file) = &log.layout {
        about = vec![format!("read by the layout {}", by(file))];
    }
    let mut lines = vec![format!("{name}: {}", about.join(", "))];
    let method = &document.method;
    lines.push(if log.samples == 0 {
        "no primary track: each channel on its own clock".to_owned()
    } else {
        format!(
            "{} samples, from {:.2} s to {:.2} s",
            log.samples, log.first_time_s, log.last_time_s
        )
    });
    if let (Some(source), Some(interval), Some(window)) = (
        method.source,
        method.sample_interval_s,
        method.median_window_s,
    ) {
        let step = match method.step_m {
            Some(step) if step > 0.0 => format!(" in steps of {step:.3} m"),
            _ => String::new(),
        };
        lines.push(format!(
            "readings from {}, sampled every {interval:.3} s{step}, after a {window:.2} s \
             running median",
            source_words(source)
        ));
    }
    let channels: Vec<String> = log
        .channels
        .iter()
        .map(|channel| format!("{} ({})", channel.column, quantity_words(channel.quantity)))
        .collect();
    lines.push(match channels.len() {
        0 => "no channels".to_owned(),
        1 => format!("1 channel: {}", channels.join(", ")),
        n => format!("{n} channels: {}", channels.join(", ")),
    });
    let mut stated = Vec::new();
    if let Some(apogee) = document.stated.apogee_m {
        stated.push(format!("apogee {}", meters(apogee)));
    }
    if let Some(ground) = document.stated.ground_elevation_msl_m {
        stated.push(format!(
            "ground elevation {} above sea level",
            meters(ground)
        ));
    }
    if !stated.is_empty() {
        lines.push(format!("the logger states: {}", stated.join("; ")));
    }
    lines.push(String::new());

    let row = |name: &str, value: String| format!("{name:<18}{value}");
    let withheld = |reading: &WithheldReading| format!("withheld: {}", reading.detail);
    // The four readings of the height track withheld for one reason, as a log with no height
    // track is: said once.
    let heights = [
        withheld_of(&document.liftoff),
        withheld_of(&document.apogee),
        withheld_of(&document.max_speed),
        withheld_of(&document.landing),
    ];
    // The flight time and each canopy's descent come from the heights alone, so they go unsaid
    // with them; burnout from the speed's peak too, unless the accelerometer gives another reason.
    let collapsed = match heights {
        [Some(first), ..] if heights.iter().all(|reading| reading == &Some(first)) => Some(first),
        _ => None,
    };
    if let Some(first) = collapsed {
        lines.push(row("readings", withheld(first)));
    } else {
        lines.extend(reading_rows(document, &row));
    }
    lines.push(row(
        "top acceleration",
        match &document.max_acceleration {
            LogReading::Read(top) => {
                let held = top
                    .interval_m_s2
                    .map(|i| format!(", 90% interval {}", acceleration_interval(i.low, i.high)))
                    .unwrap_or_default();
                let mut value = format!(
                    "{:.1} m/s² ({}), within {} m/s²{held}, at {:.2} s",
                    top.acceleration_m_s2,
                    gravities(top.acceleration_m_s2),
                    acceleration_bound(top.bound_m_s2),
                    top.time_s
                );
                // Why the top is a floor: each cause in its own words.
                value.push_str(&match (top.clip, top.range_m_s2) {
                    (Some(ReadingClip::Range), Some(range)) => format!(
                        "; clipped at the part's range of {}, so the top may have been higher",
                        gravities(range)
                    ),
                    (Some(ReadingClip::Flat), _) => "; held flat at its top, so it may have been \
                                                     clipped, and the top may have been higher"
                        .to_owned(),
                    (Some(_), _) => "; clipped, so the top may have been higher".to_owned(),
                    (None, _) if top.is_floor => "; the accelerometer holds the thrust only \
                                                  in part, with no start or no end to it, so \
                                                  the top may have been higher"
                        .to_owned(),
                    (None, _) => String::new(),
                });
                if let Some(lower) = top.lower_end_m_s2 {
                    value.push_str(&format!("; {LOWER_END} {lower:.1} m/s²"));
                }
                value
            }
            LogReading::Withheld(reading) => withheld(reading),
        },
    ));
    match (&document.burnout, collapsed) {
        (LogReading::Withheld(burnout), Some(first)) if burnout.reason == first.reason => {}
        (burnout, _) => lines.push(row(
            "burnout",
            match burnout {
                LogReading::Read(burnout) => {
                    if burnout.is_floor {
                        // A floor: no sooner than the time less its bound, rounded down so the
                        // printed time is as early as the bound allows; later by any amount.
                        let earliest = ((burnout.time_s - burnout.bound_s) * 100.0).floor() / 100.0;
                        let after = burnout
                            .time_after_liftoff_s
                            .map(|after| format!(" ({after:.2} s after liftoff)"))
                            .unwrap_or_default();
                        let lower = burnout
                            .lower_end_s
                            .map(|lower| format!("; {LOWER_END} {lower:.2} s"))
                            .unwrap_or_default();
                        format!(
                            "no sooner than {earliest:.2} s: the top speed's time, {:.2} s{after}, \
                             less its bound, {} s. Derived from the heights, burnout may have come \
                             later by any amount{lower}",
                            burnout.time_s,
                            seconds_bound(burnout.bound_s)
                        )
                    } else {
                        let after = burnout
                            .time_after_liftoff_s
                            .map(|after| format!(", {after:.2} s after liftoff"))
                            .unwrap_or_default();
                        format!(
                            "{:.2} s, within {} s{}{after}: from the accelerometer",
                            burnout.time_s,
                            seconds_bound(burnout.bound_s),
                            time_interval(burnout.interval_s)
                        )
                    }
                }
                LogReading::Withheld(reading) => withheld_after(
                    reading,
                    &[
                        (
                            "the top acceleration",
                            withheld_of(&document.max_acceleration),
                        ),
                        ("the top speed", withheld_of(&document.max_speed)),
                    ],
                ),
            },
        )),
    }
    match &document.smoother {
        SmootherOutcome::Read(read) => {
            lines.push(row(
                "smoothed apogee",
                format!(
                    "{}, 90% interval {}, at {:.2} s",
                    meters(read.apogee_height_m),
                    meters_interval(
                        read.apogee_height_interval_m.low,
                        read.apogee_height_interval_m.high
                    ),
                    read.apogee_time_s
                ),
            ));
            for (k, line) in read.card.iter().enumerate() {
                lines.push(row(if k == 0 { "health card" } else { "" }, line.clone()));
            }
        }
        // With no reading of the track at all, its row has said why.
        SmootherOutcome::Withheld(_) if collapsed.is_some() => {}
        SmootherOutcome::Withheld(withheld) => {
            lines.push(row("smoother", format!("withheld: {}", withheld.detail)));
        }
    }
    lines
}

/// How a floor's lower end is named in the text: on the synthetic flights, about 95% of truths lay
/// above it.
const LOWER_END: &str = "95% lower end";

/// A 90% interval on a time, after its reading's bound; nothing without one.
fn time_interval(interval: Option<ReadingInterval>) -> String {
    interval
        .map(|i| format!(", 90% interval {}", seconds_interval(i.low, i.high)))
        .unwrap_or_default()
}

/// A 90% interval on a height, after its reading's bound; nothing without one.
fn height_interval(interval: Option<ReadingInterval>) -> String {
    interval
        .map(|i| format!(", 90% interval {}", meters_interval(i.low, i.high)))
        .unwrap_or_default()
}

/// A 90% interval on a speed, after its reading's bound; nothing without one.
fn rate_interval(interval: Option<ReadingInterval>) -> String {
    interval
        .map(|i| format!(", 90% interval {}", speed_interval(i.low, i.high)))
        .unwrap_or_default()
}

/// The smoother's outcome as the document holds it.
fn smoother_outcome(log: &FlightLog, read: &Readings) -> SmootherOutcome {
    let smoothed = match smoother::smooth_with(log, read, Options::default()) {
        Ok(smoothed) => smoothed,
        Err(withheld) => return SmootherOutcome::Withheld(withheld),
    };
    let Some(state) = smoothed.at(smoothed.apogee_time_s) else {
        return SmootherOutcome::Withheld(smoother::Withheld {
            reason: smoother::Reason::Diverged,
            detail: "the smoothed track holds no apogee".to_owned(),
        });
    };
    let half = INTERVAL_Z * state.height_sd_m;
    let card = smoothed.health.card();
    SmootherOutcome::Read(Box::new(SmootherReading {
        apogee_time_s: smoothed.apogee_time_s,
        apogee_height_m: state.height_m,
        apogee_height_sd_m: state.height_sd_m,
        apogee_height_interval_m: ReadingInterval {
            low: state.height_m - half,
            high: state.height_m + half,
        },
        liftoff: smoothed.liftoff,
        card: card.lines().map(str::to_owned).collect(),
        health: smoothed.health,
    }))
}

/// A withheld reading's words, with what a row above already says left to it: a detail the
/// same as one above's is that reading's, and one that ends or starts with it points up.
fn withheld_after(reading: &WithheldReading, above: &[(&str, Option<&WithheldReading>)]) -> String {
    let mut detail = reading.detail.clone();
    for (name, earlier) in above {
        let Some(earlier) = earlier else { continue };
        if detail == earlier.detail {
            return format!("withheld, as {name} is");
        }
        if let Some(head) = detail.strip_suffix(&format!("withheld: {}", earlier.detail)) {
            detail = format!("{head}withheld above");
        }
        if let Some(tail) = detail.strip_prefix(&format!("{}; and ", earlier.detail)) {
            detail = format!("as for {name}; and {tail}");
        }
    }
    format!("withheld: {detail}")
}

/// A canopy's descent: its rate and the leg it was read over, from apogee or the main's opening
/// (found back from landing, so with its own bound) to the main's opening or landing.
fn descent_row(leg: &DescentReading, to: &str) -> String {
    let from = match &leg.opening {
        Some(opening) => format!(
            "the main's opening at {:.2} s, within {} s{},",
            opening.time_s,
            seconds_bound(opening.bound_s),
            time_interval(opening.interval_s)
        ),
        None => format!("apogee at {:.2} s", leg.from_s),
    };
    format!(
        "{}, within {}{}, from {from} to {to} at {:.2} s, {} to {} up",
        speed(leg.rate_m_s),
        speed_bound(leg.bound_m_s),
        rate_interval(leg.interval_m_s),
        leg.to_s,
        meters(leg.from_height_m),
        meters(leg.to_height_m)
    )
}

/// The rows of liftoff, apogee, the top speed and landing, each with the bound its error stays
/// within, rounded up.
fn reading_rows(document: &Analyze, row: &dyn Fn(&str, String) -> String) -> Vec<String> {
    let method = &document.method;
    let withheld = |reading: &WithheldReading| format!("withheld: {}", reading.detail);
    let mut lines = vec![row(
        "liftoff",
        match &document.liftoff {
            LogReading::Read(liftoff) => {
                format!(
                    "{:.2} s, within {} s{}",
                    liftoff.time_s,
                    seconds_bound(liftoff.bound_s),
                    time_interval(liftoff.interval_s)
                )
            }
            LogReading::Withheld(reading) => withheld(reading),
        },
    )];
    match &document.apogee {
        LogReading::Read(apogee) => {
            let mut value = format!(
                "{} above the pad, within {}{}; at {:.2} s, within {} s{}",
                meters(apogee.height_m),
                meters_bound(apogee.bound_m),
                height_interval(apogee.height_interval_m),
                apogee.time_s,
                seconds_bound(apogee.time_bound_s),
                time_interval(apogee.time_interval_s),
            );
            if let Some(after) = apogee.time_after_liftoff_s {
                value.push_str(&format!(", {after:.2} s after liftoff"));
            }
            if apogee.is_floor {
                value.push_str("; the log ends at its peak, so the rocket may have gone higher");
            }
            lines.push(row("apogee", value));
            // Above what the median's own rounding of the peak, the track's step and its noise
            // explain, the median set a pulse aside.
            let highest = &apogee.highest_sample;
            let explained = method.peak_bound_m.unwrap_or(0.0)
                + method
                    .step_m
                    .unwrap_or(0.0)
                    .max(method.altitude_resolution_m)
                + readings::NOISE_M;
            if highest.altitude_m - apogee.altitude_m > explained {
                lines.push(row(
                    "",
                    format!(
                        "highest sample {} above the pad at {:.2} s, set aside by the median",
                        meters(highest.altitude_m - method.pad_altitude_m.unwrap_or(0.0)),
                        highest.time_s
                    ),
                ));
            }
        }
        LogReading::Withheld(reading) => lines.push(row("apogee", withheld(reading))),
    }
    lines.push(row(
        "top speed",
        match &document.max_speed {
            LogReading::Read(top) => format!(
                "{}, within {}{}, at {:.2} s, {} up: derived from the heights",
                speed(top.speed_m_s),
                speed_bound(top.bound_m_s),
                rate_interval(top.interval_m_s),
                top.time_s,
                meters(top.height_m)
            ),
            LogReading::Withheld(reading) => withheld(reading),
        },
    ));
    lines.push(row(
        "landing",
        match &document.landing {
            LogReading::Read(landing) => {
                let mut value = format!(
                    "{:.2} s, within {} s{}",
                    landing.time_s,
                    seconds_bound(landing.bound_s),
                    time_interval(landing.interval_s)
                );
                value.push_str(&format!(
                    "; {:.2} s from apogee, at {} on average, within {}{}",
                    landing.descent_time_s,
                    speed(landing.mean_descent_rate_m_s),
                    speed_bound(landing.mean_descent_rate_bound_m_s),
                    rate_interval(landing.mean_descent_rate_interval_m_s)
                ));
                value
            }
            LogReading::Withheld(reading) => withheld(reading),
        },
    ));
    let above = [
        ("liftoff", withheld_of(&document.liftoff)),
        ("landing", withheld_of(&document.landing)),
    ];
    lines.push(row(
        "flight time",
        match &document.flight_time {
            LogReading::Read(flight) => format!(
                "{:.2} s, within {} s{}",
                flight.time_s,
                seconds_bound(flight.bound_s),
                time_interval(flight.interval_s)
            ),
            LogReading::Withheld(reading) => withheld_after(reading, &above),
        },
    ));
    lines.push(row(
        "drogue descent",
        match &document.drogue_descent {
            LogReading::Read(leg) => descent_row(leg, "the main's opening"),
            LogReading::Withheld(reading) => withheld_after(reading, &above),
        },
    ));
    lines.push(row(
        "main descent",
        match &document.main_descent {
            LogReading::Read(leg) => descent_row(leg, "landing"),
            LogReading::Withheld(reading) => withheld_after(reading, &above),
        },
    ));
    lines
}

/// The reading's withholding, if it was withheld.
fn withheld_of<T>(reading: &LogReading<T>) -> Option<&WithheldReading> {
    match reading {
        LogReading::Read(_) => None,
        LogReading::Withheld(withheld) => Some(withheld),
    }
}

/// What a channel measures, in words: its JSON name, its words spaced.
fn quantity_words(quantity: Quantity) -> String {
    match quantity {
        Quantity::GnssAltitude => "GNSS altitude".to_owned(),
        Quantity::AltitudeMsl => "altitude above sea level".to_owned(),
        _ => serde_json::to_value(quantity)
            .ok()
            .and_then(|value| value.as_str().map(|name| name.replace('_', " ")))
            .unwrap_or_default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A top acceleration reads in g too, standard gravity, in brackets after the m/s² (issue
    /// #392), and its bound rounded up. The reading is set by hand on a log with no
    /// accelerometer, at a value where a rounded 9.81 m/s² would print another tenth and a bound
    /// rounded to the nearest a tenth less.
    #[test]
    fn a_top_acceleration_gives_g() {
        let text = include_str!("../../../validation/fixtures/logs/synthetic-pnut.pf2");
        let log = hpr::hpr_flightdata::perfectflite::read(text).unwrap();
        let args = AnalyzeArgs {
            log: "synthetic-pnut.pf2".to_owned(),
            mapping: None,
            layout: None,
            sheet: None,
            recording: None,
            fip_header: None,
            print_mapping: false,
            record: false,
        };
        let mut document = document(&args, &log, &readings::read(&log));
        document.max_acceleration = LogReading::Read(MaxAccelerationReading {
            acceleration_m_s2: 1000.0,
            bound_m_s2: 101.04,
            interval_m_s2: None,
            lower_end_m_s2: None,
            time_s: 0.75,
            range_m_s2: None,
            is_floor: false,
            clip: None,
            source: ReadingSource::Accelerometer,
            provenance: ReadingProvenance::Measured,
            method: ReadingMethod::HighestReading,
        });
        let lines = text_lines(&document);
        assert!(
            lines.iter().any(|line| line
                == "top acceleration  1000.0 m/s² (102.0 g), within 101.1 m/s², at 0.75 s"),
            "{lines:#?}"
        );
    }

    /// Every reason, source, provenance and method the library has maps to the command's own, of
    /// the same name: a library variant added without a mapping would reach the JSON as `other`.
    #[test]
    fn every_library_reason_and_source_has_its_own_name() {
        for reason in Reason::ALL.iter().copied() {
            let withheld = Reading::<()>::Withheld(readings::Withheld {
                reason,
                detail: String::new(),
            });
            let LogReading::Withheld(mapped) = reading(&withheld, |()| ()) else {
                panic!("{reason:?} read");
            };
            assert_ne!(mapped.reason, WithheldReason::Other, "{reason:?}");
            assert_eq!(
                serde_json::to_value(mapped.reason).unwrap(),
                serde_json::to_value(reason).unwrap()
            );
        }
        for from in Source::ALL.iter().copied() {
            let mapped = source(from);
            assert_ne!(mapped, ReadingSource::Other, "{from:?}");
            assert_eq!(
                serde_json::to_value(mapped).unwrap(),
                serde_json::to_value(from).unwrap()
            );
        }
        for from in Provenance::ALL.iter().copied() {
            let mapped = provenance(from);
            assert_ne!(mapped, ReadingProvenance::Other, "{from:?}");
            assert_eq!(
                serde_json::to_value(mapped).unwrap(),
                serde_json::to_value(from).unwrap()
            );
        }
        for from in Method::ALL.iter().copied() {
            let mapped = method(from);
            assert_ne!(mapped, ReadingMethod::Other, "{from:?}");
            assert_eq!(
                serde_json::to_value(mapped).unwrap(),
                serde_json::to_value(from).unwrap()
            );
        }
        for from in Gravity::ALL.iter().copied() {
            let mapped = gravity(from);
            assert_ne!(mapped, AccelerometerGravity::Other, "{from:?}");
            assert_eq!(
                serde_json::to_value(mapped).unwrap(),
                serde_json::to_value(from).unwrap()
            );
        }
        for from in GravityBy::ALL.iter().copied() {
            let mapped = gravity_by(from);
            assert_ne!(mapped, GravitySettledBy::Other, "{from:?}");
            assert_eq!(
                serde_json::to_value(mapped).unwrap(),
                serde_json::to_value(from).unwrap()
            );
        }
        for from in Clip::ALL.iter().copied() {
            let mapped = clip(from);
            assert_ne!(mapped, ReadingClip::Other, "{from:?}");
            assert_eq!(
                serde_json::to_value(mapped).unwrap(),
                serde_json::to_value(from).unwrap()
            );
        }
    }
}
