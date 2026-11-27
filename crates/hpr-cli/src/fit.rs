//! `hpr fit`: a design's simulation fitted to its flight's log by least squares
//! (`hpr::fit`, [M7.3f][m7-3f], ADR-381).
//!
//! It prints each fitted parameter with its least-squares 90% interval, or why it states none;
//! each thing it read from the log with its source and provenance; the caveat an interval that
//! fails its test carries (ADR-368), beside the interval; and the known issues the fitted flight
//! meets, as `hpr sim` warns of them (ADR-163 §4).
//!
//! The log, the design and the launch are given as `hpr compare` takes them, and the launch
//! day's conditions come from the same places: Open-Meteo's archive at `--launch-time`, a weather
//! profile, or values entered. The wind is the one thing of them the fit moves: what is given is
//! what it is told of it.
//!
//! [m7-3f]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-3f

use std::path::Path;

use hpr::fit::{self as library, Fitted, LogFit, NoInterval, Parameter, Refused};
use hpr::hpr_flightdata::readings;
use hpr::hpr_forensics::residuals::HeightKind;

use crate::analyze;
use crate::compare::{self, CompareArgs, HeightReads};
use crate::console::Level;
use crate::output::{
    CompareConditions, CompareFlownIn, CompareSite, Fit, FitApogee, FitCaveat, FitLiftoff,
    FitNoInterval, FitParameter, FitParameterName, FitReadings, FitTrack, FitWindPastScan,
    HeightReadAs, IssueKind, ReadingInterval, ReadingProvenance, ReadingSource, SimIssue, SiteFrom,
};
use crate::sim::{FlightArgs, Setup, file_name, flight_refused};
use crate::units::{fixed, meters, meters_bound, rounded_down, rounded_up, seconds_bound};
use crate::{Failure, Out};

/// The widest line of the text form's own sentences, as the trust note is wrapped.
const WIDTH: usize = 100;

/// `hpr fit`'s command line.
#[derive(Debug, clap::Args)]
pub struct FitArgs {
    /// The flight log: a file in any format HPR Sim reads, or another logger's file read by
    /// --mapping or --layout
    pub log: String,
    /// The design, its motor and the launch, as `hpr sim` takes them. Give the launch day's air
    /// and wind, or the fit flies in the standard atmosphere and is told there was no wind.
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
    /// The launch's time in UTC, such as 2023-06-23T23:00Z, or `log` for the time the log's GPS
    /// receiver gives at liftoff (an AltosUI CSV's). The launch day's air and wind then come from
    /// Open-Meteo's archive of past forecasts at the site. The site is the log's GNSS fixes before
    /// liftoff unless --latitude or --longitude is given. With --offline, from the cache alone
    #[arg(
        long,
        value_name = "TIME",
        conflicts_with_all = ["weather", "wind", "temperature", "pressure"]
    )]
    pub launch_time: Option<String>,
    /// The air's temperature at the pad on the launch day, °C, as you measured it: the standard
    /// atmosphere is shifted to pass through it. Without it, the standard's at --elevation
    #[arg(
        long,
        value_name = "C",
        allow_hyphen_values = true,
        value_parser = crate::typed::number::<f64>(),
        conflicts_with = "weather"
    )]
    pub temperature: Option<f64>,
    /// The air's pressure at the pad on the launch day, hPa: the station's own, not one reduced to
    /// sea level as a weather report's is. Without it, the standard atmosphere's at --elevation
    #[arg(
        long,
        value_name = "HPA",
        allow_hyphen_values = true,
        value_parser = crate::typed::number::<f64>(),
        conflicts_with = "weather"
    )]
    pub pressure: Option<f64>,
}

impl FitArgs {
    /// The same log, design, launch and launch day as `hpr compare` takes them, with no figure:
    /// the two commands place the site and find the day's conditions by the same code.
    fn as_compare(&self) -> CompareArgs {
        CompareArgs {
            log: self.log.clone(),
            flight: self.flight.clone(),
            mapping: self.mapping.clone(),
            layout: self.layout.clone(),
            sheet: self.sheet,
            height_reads: self.height_reads,
            recording: self.recording,
            plot: None,
            launch_time: self.launch_time.clone(),
            temperature: self.temperature,
            pressure: self.pressure,
        }
    }
}

/// Runs `hpr fit`.
pub(crate) fn run(args: &FitArgs, to: &mut Out<'_>) -> Result<(), Failure> {
    let given = args.as_compare();
    let log = analyze::read_log(&given.log_args())?;
    let read = readings::read(&log);
    let (flight, site) = compare::sited(&given, &log, &read)?;
    let setup = Setup::read(&flight)?;
    let launch_day = compare::launch_day(&given, (&log, &read), &flight, &setup, &site)?;
    let environment = launch_day
        .as_ref()
        .map_or(&setup.designed, |day| &day.environment);
    let stated = args.height_reads.map(|reads| match reads {
        HeightReads::Barometer => HeightKind::Barometric,
        HeightReads::HeightClimbed => HeightKind::Geometric,
    });
    let builder = setup.builder_in(environment, &flight);
    let fitted = library::fit(&builder, &log, stated).map_err(|refused| match refused {
        Refused::Flight(error) => flight_refused(&error, &flight),
        Refused::Log(_) => Failure::helped(
            format!("{}: {refused}", args.log),
            "`hpr analyze` on the log says what it reads of the flight; a fit needs the climb \
             from liftoff to an apogee",
        ),
        Refused::Staged(_) => Failure::helped(
            format!("{}: {refused}", args.flight.design),
            "`hpr compare` sets a staged flight's log against its simulation; a fit of one isn't \
             available",
        ),
        Refused::SaysNothing { .. } => Failure::helped(
            format!("{}: {refused}", args.log),
            "`hpr compare` with the same options shows how far the log lies from the simulation; \
             check the design, its motor and --height-reads against the flight flown",
        ),
        _ => Failure::helped(
            format!("{}: {refused}", args.flight.design),
            "`hpr sim` with the same options shows what it flew; the flight must leave the pad, \
             in air that reaches its apogee",
        ),
    })?;
    drop(builder);

    let mut notes = setup.read.notes.clone();
    notes.extend(log.notes.iter().map(|note| format!("the log: {note}")));
    if args.mapping.is_some() || args.layout.is_some() {
        notes.push(
            "the log was read in the units your mapping or layout states, so the fit holds only \
             if those are right"
                .to_owned(),
        );
    }
    if stated.is_none() && fitted.height_read_as == HeightKind::Barometric {
        notes.push(
            "a log's main altitude, or a column you mapped, is taken as a barometer's: give \
             --height-reads height-climbed for a GPS receiver's"
                .to_owned(),
        );
    }
    if !setup.devices.is_empty() {
        notes.push(
            "the fit flies the climb with nothing open: a flight whose parachute opened before \
             apogee was slowed by it, and the fit puts that into the drag scale"
                .to_owned(),
        );
    }
    if !fitted.converged {
        notes.push("the fit stopped before it settled: take its numbers as rough".to_owned());
    }
    let document = Fit {
        kind: crate::trust::Kind::Fitted,
        trust: crate::trust::fitted(),
        log: args.log.clone(),
        logger: log.logger.clone(),
        design: file_name(&args.flight.design),
        configuration: setup.configuration_label.clone(),
        site: CompareSite {
            latitude_deg: flight.latitude,
            longitude_deg: flight.longitude,
            elevation_m: flight.elevation,
            from: site.from,
        },
        flown_in: CompareFlownIn {
            conditions: if launch_day.is_some() {
                CompareConditions::LaunchDay
            } else {
                CompareConditions::AsDesigned
            },
            detail: launch_day
                .as_ref()
                .map_or_else(|| compare::DESIGNED.to_owned(), |day| day.said.clone()),
        },
        height_read_as: match fitted.height_read_as {
            HeightKind::Barometric => HeightReadAs::Barometer,
            HeightKind::Geometric => HeightReadAs::HeightClimbed,
        },
        readings: readings_document(&fitted),
        parameters: fitted.parameters.iter().map(parameter_document).collect(),
        rms_m: fitted.rms_m,
        misfit_share: fitted.misfit_share,
        converged: fitted.converged,
        clock_offset_s: fitted.clock_offset_s,
        height_offset_m: fitted.height_offset_m,
        flights_flown: fitted.flights_flown,
        stated_apogee_m: fitted.stated_apogee_m,
        fitted_apogee_m: fitted.fitted_apogee_m,
        fitted_apogee_time_s: fitted.fitted_apogee_time_s,
        fitted_max_mach: fitted.fitted_max_mach,
        wind_past_scan: fitted.wind_past_scan.map(|past| {
            let (speed_m_s, from_deg) = past.told();
            FitWindPastScan {
                wind_east_m_s: past.wind_east_m_s,
                wind_north_m_s: past.wind_north_m_s,
                speed_m_s: past.speed_m_s(),
                from_deg: past.from_deg(),
                chi2: past.chi2,
                standing_chi2: past.standing_chi2,
                message: format!(
                    "{} On the command line: `--wind {speed_m_s:.1} --wind-from {from_deg:.0}`.",
                    past.message()
                ),
            }
        }),
        issues: fitted
            .issue_warnings
            .iter()
            .map(|warning| SimIssue {
                issue: warning.number(),
                kind: IssueKind::from(warning.issue.kind()),
                url: warning.issue.url(),
                max_mach: crate::sim::peak(warning.max_mach, fitted.fitted_apogee_time_s),
                parts: warning.parts.clone(),
                message: warning.message(),
            })
            .collect(),
        notes,
    };
    let lines: Vec<String> = text_lines(&document)
        .iter()
        .map(|line| crate::printable(line))
        .collect();
    to.emit(&document, |out, diagnostics| {
        out.lines(&lines, &[])?;
        crate::sim_text::trust_lines(&document.trust, out)?;
        for issue in &document.issues {
            let kind = serde_json::to_value(issue.kind)
                .ok()
                .and_then(|kind| kind.as_str().map(str::to_owned))
                .unwrap_or_default();
            diagnostics.line(
                Level::Warning,
                &format!("{kind}: {}", crate::sim_text::issue_text(issue)),
            )?;
        }
        crate::sim_text::issue_help(&document.issues, diagnostics)?;
        document
            .notes
            .iter()
            .try_for_each(|note| diagnostics.line(Level::Note, note))
    })
}

/// What the fit read, as the document holds it.
fn readings_document(fitted: &LogFit) -> FitReadings {
    let read = &fitted.readings;
    let track = |track: &library::TrackRead| FitTrack {
        source: analyze::source(track.source),
        provenance: analyze::provenance(track.provenance),
        bins: track.bins,
        bins_read: track.bins_read,
    };
    FitReadings {
        liftoff: read.liftoff.map(|liftoff| FitLiftoff {
            time_s: liftoff.time_s,
            bound_s: liftoff.bound_s,
            source: analyze::source(liftoff.source),
            provenance: analyze::provenance(liftoff.provenance),
        }),
        apogee: FitApogee {
            time_s: read.apogee.time_s,
            time_bound_s: read.apogee.time_bound_s,
            height_m: read.apogee.height_m,
            bound_m: read.apogee.bound_m,
            source: analyze::source(read.apogee.source),
            provenance: analyze::provenance(read.apogee.provenance),
        },
        height_track: track(&read.height_track),
        horizontal_track: read.horizontal_track.as_ref().map(track),
    }
}

/// A parameter, as the document holds it.
fn parameter_document(fitted: &Fitted) -> FitParameter {
    FitParameter {
        parameter: match fitted.parameter {
            Parameter::DragScale => FitParameterName::DragScale,
            Parameter::ImpulseScale => FitParameterName::ImpulseScale,
            Parameter::BurnTimeScale => FitParameterName::BurnTimeScale,
            Parameter::WindEast => FitParameterName::WindEastMS,
            Parameter::WindNorth => FitParameterName::WindNorthMS,
            Parameter::MassScale => FitParameterName::MassScale,
        },
        value: fitted.value,
        standard_error: fitted.standard_error,
        stated_mean: fitted.stated.mean,
        stated_sd: fitted.stated.sd,
        interval_90: fitted.interval_90.map(|interval| ReadingInterval {
            low: interval.low,
            high: interval.high,
        }),
        no_interval: fitted.no_interval.map(|why| match why {
            NoInterval::NotAsked => FitNoInterval::NotAsked,
            NoInterval::HeightAlone => FitNoInterval::HeightAlone,
            NoInterval::NoStandardError => FitNoInterval::NoStandardError,
        }),
        caveat: fitted.caveat.map(|caveat| FitCaveat {
            below: caveat.below,
            flights: caveat.flights,
            meant: caveat.meant,
            message: caveat.message(fitted.parameter),
        }),
        tested_range: ReadingInterval {
            low: fitted.tested_range.low,
            high: fitted.tested_range.high,
        },
        outside_tested_range: fitted.outside_tested_range,
    }
}

/// A parameter's name in a line of text.
fn parameter_name(name: FitParameterName) -> &'static str {
    match name {
        FitParameterName::DragScale => Parameter::DragScale.name(),
        FitParameterName::ImpulseScale => Parameter::ImpulseScale.name(),
        FitParameterName::BurnTimeScale => Parameter::BurnTimeScale.name(),
        FitParameterName::WindEastMS => Parameter::WindEast.name(),
        FitParameterName::WindNorthMS => Parameter::WindNorth.name(),
        FitParameterName::MassScale => Parameter::MassScale.name(),
    }
}

/// Whether a parameter is a speed; the others are factors with no unit.
fn is_speed(name: FitParameterName) -> bool {
    matches!(
        name,
        FitParameterName::WindEastMS | FitParameterName::WindNorthMS
    )
}

/// Where a reading came from: its provenance, then its track.
fn from_text(provenance: ReadingProvenance, source: ReadingSource) -> String {
    format!(
        "{}, from {}",
        analyze::together::provenance_words(provenance),
        analyze::source_words(source)
    )
}

/// The text form.
fn text_lines(document: &Fit) -> Vec<String> {
    let name = Path::new(&document.log).file_name().map_or_else(
        || document.log.clone(),
        |name| name.to_string_lossy().into_owned(),
    );
    let logger = match document.logger.as_str() {
        "unknown" => String::new(),
        logger => format!(" ({logger})"),
    };
    let mut lines = vec![
        format!("{name}{logger}, its simulation fitted to it"),
        format!("  flown: {}, {}", document.design, document.configuration),
        format!(
            "  {}: {}",
            match document.flown_in.conditions {
                CompareConditions::AsDesigned => "as designed",
                CompareConditions::LaunchDay => "on the launch day",
            },
            document.flown_in.detail
        ),
        format!(
            "  each simulated height is read as {}, as the log's is",
            match document.height_read_as {
                HeightReadAs::Barometer => "a barometer on the pad would read it",
                HeightReadAs::HeightClimbed => "the height climbed",
            }
        ),
    ];
    match document.site.from {
        SiteFrom::LogFixesBeforeLiftoff | SiteFrom::LogFirstFix => lines.push(format!(
            "  the site: {}, {}, from the log's {}",
            crate::sim_text::hemisphere(document.site.latitude_deg, "N", "S"),
            crate::sim_text::hemisphere(document.site.longitude_deg, "E", "W"),
            if document.site.from == SiteFrom::LogFirstFix {
                "first GNSS fix"
            } else {
                "GNSS fixes before liftoff"
            }
        )),
        SiteFrom::Given | SiteFrom::Default => {}
    }
    lines.push(String::new());
    lines.push("Read from the log".to_owned());
    let read = &document.readings;
    let from = |provenance, source| format!("           {}", from_text(provenance, source));
    match &read.liftoff {
        Some(liftoff) => {
            lines.push(format!(
                "  liftoff  {:.2} s, within {} s",
                liftoff.time_s,
                seconds_bound(liftoff.bound_s)
            ));
            lines.push(from(liftoff.provenance, liftoff.source));
        }
        None => lines.push("  liftoff  the log's first sample: it starts in flight".to_owned()),
    }
    lines.push(format!(
        "  apogee   {}, within {}, at {:.2} s, within {} s",
        meters(read.apogee.height_m),
        meters_bound(read.apogee.bound_m),
        read.apogee.time_s,
        seconds_bound(read.apogee.time_bound_s)
    ));
    lines.push(from(read.apogee.provenance, read.apogee.source));
    lines.push(format!(
        "  heights  {} of {} quarter-second bins from 1 s before liftoff to apogee",
        read.height_track.bins_read, read.height_track.bins
    ));
    lines.push(from(read.height_track.provenance, read.height_track.source));
    match &read.horizontal_track {
        Some(track) => {
            lines.push(format!(
                "  drift    {} quarter-second bins east and north of the pad",
                track.bins
            ));
            lines.push(format!(
                "           {}, from the GNSS fixes",
                analyze::together::provenance_words(track.provenance)
            ));
        }
        None => lines.push("  drift    none: the log holds no latitude and longitude".to_owned()),
    }
    lines.push(String::new());
    lines.push("Fitted by least squares".to_owned());
    // One row a parameter: its value, its 90% interval, and what the fit was told of it before
    // the log, so a scale the log pulled far from its stated range shows as one.
    let mut rows = vec![[
        String::new(),
        "fitted".to_owned(),
        "90% interval".to_owned(),
        "told before the log".to_owned(),
    ]];
    let feet = |value: f64| value / crate::units::FOOT_M;
    for fitted in &document.parameters {
        let mark = if fitted.caveat.is_some() { " *" } else { "" };
        let outside = if fitted.outside_tested_range {
            " †"
        } else {
            ""
        };
        if is_speed(fitted.parameter) {
            // A speed takes two rows, meters per second and under them feet per second, so the
            // table keeps its width whatever the wind. Intervals round outward: a printed
            // interval never claims less than it holds.
            let (si, us) = match &fitted.interval_90 {
                Some(interval) => (
                    format!(
                        "{} to {} m/s{mark}",
                        fixed(rounded_down(interval.low, 1), 1),
                        fixed(rounded_up(interval.high, 1), 1)
                    ),
                    format!(
                        "({} to {} ft/s)",
                        fixed(rounded_down(feet(interval.low), 0), 0),
                        fixed(rounded_up(feet(interval.high), 0), 0)
                    ),
                ),
                None => ("none".to_owned(), String::new()),
            };
            rows.push([
                parameter_name(fitted.parameter).to_owned(),
                format!("{} m/s{outside}", fixed(fitted.value, 1)),
                si,
                format!(
                    "{} ± {} m/s",
                    fixed(fitted.stated_mean, 1),
                    fixed(fitted.stated_sd, 1)
                ),
            ]);
            rows.push([
                String::new(),
                format!("({} ft/s)", fixed(feet(fitted.value), 0)),
                us,
                format!(
                    "({} ± {} ft/s)",
                    fixed(feet(fitted.stated_mean), 0),
                    fixed(feet(fitted.stated_sd), 0)
                ),
            ]);
        } else {
            let interval = match &fitted.interval_90 {
                Some(interval) => format!(
                    "{} to {}{mark}",
                    fixed(rounded_down(interval.low, 3), 3),
                    fixed(rounded_up(interval.high, 3), 3)
                ),
                None => "none".to_owned(),
            };
            rows.push([
                parameter_name(fitted.parameter).to_owned(),
                format!("{}{outside}", fixed(fitted.value, 3)),
                interval,
                format!(
                    "{} ± {}",
                    fixed(fitted.stated_mean, 3),
                    fixed(fitted.stated_sd, 3)
                ),
            ]);
        }
    }
    let width = |column: usize| {
        rows.iter()
            .map(|row| row[column].chars().count())
            .max()
            .unwrap_or(0)
    };
    let widths = [width(0), width(1), width(2)];
    for row in &rows {
        lines.push(
            format!(
                "  {:<a$}  {:<b$}  {:<c$}  {}",
                row[0],
                row[1],
                row[2],
                row[3],
                a = widths[0],
                b = widths[1],
                c = widths[2]
            )
            .trim_end()
            .to_owned(),
        );
    }
    let without = |why: FitNoInterval| {
        document
            .parameters
            .iter()
            .any(|fitted| fitted.no_interval == Some(why))
    };
    if document.parameters.iter().any(|f| f.caveat.is_some()) {
        lines.push("  * This interval fails its test: the caveat below.".to_owned());
    }
    // A value outside what the intervals were tested over; the wind's two components share a line.
    let mut wind_said = false;
    for fitted in document
        .parameters
        .iter()
        .filter(|f| f.outside_tested_range)
    {
        let tested = &fitted.tested_range;
        if is_speed(fitted.parameter) {
            if !wind_said {
                lines.push(format!(
                    "  † The wind is faster than {} m/s ({} ft/s), the fastest the intervals were \
                     tested in.",
                    fixed(rounded_down(tested.high, 1), 1),
                    fixed(rounded_down(feet(tested.high), 0), 0)
                ));
            }
            wind_said = true;
        } else {
            lines.push(format!(
                "  † The {} lies outside {} to {}, the range the intervals were tested over.",
                parameter_name(fitted.parameter),
                // Inward: a value just past an end isn't printed as lying on it.
                fixed(rounded_up(tested.low, 3), 3),
                fixed(rounded_down(tested.high, 3), 3)
            ));
        }
    }
    if without(FitNoInterval::NotAsked) {
        lines.push(format!(
            "  The burn time scale states no interval: {}.",
            NoInterval::NotAsked.words()
        ));
    }
    if without(FitNoInterval::HeightAlone) {
        lines.push(format!(
            "  The wind states no interval: {}.",
            NoInterval::HeightAlone.words()
        ));
        // A floor is rounded down: the fit is never told less than the line says.
        let least = library::height_alone_wind_m_s();
        lines.push(format!(
            "  So the fit is told a wind of at least {} m/s ({} ft/s), the wind its intervals were \
             sized in.",
            fixed(rounded_down(least, 2), 2),
            fixed(rounded_down(feet(least), 1), 1)
        ));
    }
    for fitted in &document.parameters {
        if fitted.no_interval == Some(FitNoInterval::NoStandardError) {
            lines.push(format!(
                "  The {} states no interval: {}.",
                parameter_name(fitted.parameter),
                NoInterval::NoStandardError.words()
            ));
        }
    }
    for fitted in &document.parameters {
        if let Some(caveat) = &fitted.caveat {
            lines.extend(
                crate::trust::wrap(&format!("Caveat. {}", caveat.message), WIDTH - 2)
                    .iter()
                    .map(|line| format!("  {line}")),
            );
        }
    }
    if let Some(past) = &document.wind_past_scan {
        lines.extend(
            crate::trust::wrap(&format!("Caveat. {}", past.message), WIDTH - 2)
                .iter()
                .map(|line| format!("  {line}")),
        );
    }
    lines.push(String::new());
    lines.push("The fitted flight".to_owned());
    lines.push(format!(
        "  heights' residual  {} RMS, {}% of the heights' spread",
        meters(document.rms_m),
        fixed(100.0 * document.misfit_share, 1)
    ));
    match (document.fitted_apogee_m, document.fitted_max_mach) {
        (Some(apogee_m), Some(mach)) => lines.push(format!(
            "  apogee             {}, top Mach {}; as stated, {}",
            meters(apogee_m),
            fixed(mach, 2),
            meters(document.stated_apogee_m)
        )),
        _ => lines.push(format!(
            "  apogee             none reached; as stated, {}",
            meters(document.stated_apogee_m)
        )),
    }
    lines.push(format!("  simulations flown  {}", document.flights_flown));
    lines
}
