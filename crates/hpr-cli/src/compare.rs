//! `hpr compare`: a flight log against a simulation of it (`hpr_forensics::residuals`), each
//! residual the logged value less the simulated one; with `--plot`, a figure of the log beside the
//! simulation as designed and, where they are given, under the launch day's conditions
//! ([M7.3e][m7-3e], ADR-379).
//!
//! The launch day's conditions come from one place: Open-Meteo's archive of past forecasts at the
//! launch's time (`--launch-time`), a weather profile (`--weather`), or values entered (`--wind`,
//! `--temperature`, `--pressure`), so the command works offline from values entered or a cached
//! answer. The site is the one given, or,
//! where none is, the log's GNSS fixes before liftoff, as `hpr analyze --ghost` places its pad.
//!
//! [m7-3e]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m7-3e

use std::path::Path;

use hpr::Environment;
use hpr::hpr_atmos::Ussa76;
use hpr::hpr_flightdata::ghost::{self, PadFrom};
use hpr::hpr_flightdata::log::FlightLog;
use hpr::hpr_flightdata::readings::{self, Readings};
use hpr::hpr_flightdata::timeline::What;
use hpr::hpr_forensics::residuals::{
    self, Comparison, HeightKind, Phase, Residuals, Simulated, Speed, Track,
};
use hpr::hpr_net::open_meteo;

use crate::analyze::{self, AnalyzeArgs};
use crate::console::Level;
use crate::output::{
    Compare, CompareConditions, CompareFlownIn, ComparePhase, ComparePlot, ComparePoint,
    CompareResiduals, CompareRow, CompareSite, CompareSpeed, ExportMeta, HeightReadAs,
    ResidualPhase, SiteFrom, TimelineCell, TimelineWhat,
};
use crate::sim::{FlightArgs, Setup, file_name, flight_refused, same_file};
use crate::units::{fixed, meters, seconds_bound, speed};
use crate::weather::{Fetching, OpenMeteoArgs};
use crate::{Failure, Out};

/// The coldest and hottest air at a pad that `--temperature` takes, °C: past the coldest and
/// hottest air measured at the ground, −89.2 °C (Vostok, 1983) and 56.7 °C (Death Valley, 1913),
/// as the World Meteorological Organization's archive of weather extremes lists them.
const TEMPERATURE_C: (f64, f64) = (-90.0, 60.0);
/// How far a pad's pressure may lie from the 1976 standard atmosphere's at its height before
/// `--pressure` is refused, as a fraction: past the deepest low and the highest high measured at
/// sea level, 870 hPa (Typhoon Tip, 1979) and 1,083.8 hPa (Agata, 1968, the highest below 750 m),
/// 14% under and 7% over the standard's 1,013.25 hPa, as the World Meteorological Organization's
/// archive lists them. A
/// pressure reduced to sea level, as a weather report gives it, lies about 18% over the
/// standard's 856 hPa at a 1,400 m field (1,013 hPa), and is refused there; nearer sea level the
/// two differ less, and so does the air flown.
const PRESSURE_FROM_STANDARD: f64 = 0.15;

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
    /// Draw the log's altitude and vertical speed, liftoff to landing, into this .svg file. Beside
    /// them go the simulation's: as designed, and under the launch day's conditions where they are
    /// given. Its data go beside it
    #[arg(long, value_name = "FILE")]
    pub plot: Option<String>,
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
    // The figure's files are refused before anything is read or flown.
    let plot = plot_files(args)?;
    let log = analyze::read_log(&args.log_args())?;
    let read = readings::read(&log);
    let (flight, site) = sited(args, &log, &read)?;
    let setup = Setup::read(&flight)?;
    let launch_day = launch_day(args, (&log, &read), &flight, &setup, &site)?;
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
    let fly = |environment: &Environment| -> Result<Simulated, Failure> {
        let builder = setup.builder_in(environment, &flight);
        let mut track = Track::new();
        builder
            .fly_with(&mut track)
            .map_err(|error| flight_refused(&error, &flight))?;
        drop(builder);
        track.simulated(environment.sim()).map_err(refused)
    };
    let stated = args.height_reads.map(|reads| match reads {
        HeightReads::Barometer => HeightKind::Barometric,
        HeightReads::HeightClimbed => HeightKind::Geometric,
    });
    // The residuals are against the launch day's flight where its conditions are given, and
    // otherwise against the flight as designed; the figure draws both.
    let (simulated, flown_in) = match &launch_day {
        Some(day) => (fly(&day.environment)?, Flown::LaunchDay),
        None => (fly(&setup.designed)?, Flown::AsDesigned),
    };
    let comparison = residuals::compare(&log, &read, &simulated, stated).map_err(refused)?;
    let designed = match (&plot, flown_in) {
        (Some(_), Flown::LaunchDay) => {
            let simulated = fly(&setup.designed)?;
            let comparison =
                residuals::compare(&log, &read, &simulated, stated).map_err(refused)?;
            Some((simulated, comparison))
        }
        _ => None,
    };

    let notes = notes(args, &log, &setup, stated);
    let designed_said = DESIGNED.to_owned();
    let mut document = Compare {
        kind: crate::trust::Kind::Compared,
        trust: crate::trust::compared(),
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
            conditions: match flown_in {
                Flown::AsDesigned => CompareConditions::AsDesigned,
                Flown::LaunchDay => CompareConditions::LaunchDay,
            },
            detail: launch_day
                .as_ref()
                .map_or_else(|| designed_said.clone(), |day| day.said.clone()),
        },
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
        plot: None,
        notes,
    };
    if let Some(files) = &plot {
        let (designed_simulated, designed_comparison, day) = match &designed {
            Some((simulated_designed, comparison_designed)) => (
                simulated_designed,
                comparison_designed,
                Some((&simulated, &comparison)),
            ),
            None => (&simulated, &comparison, None),
        };
        document.plot = Some(draw(
            files,
            &Drawn {
                log: &log,
                read: &read,
                document: &document,
                designed: (designed_simulated, designed_comparison),
                launch_day: day,
                conditions: conditions_lines(&flight, &site, &designed_said, launch_day.as_ref()),
            },
        )?);
    }
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

/// Which conditions a simulation flew in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Flown {
    /// The standard atmosphere and no wind, at the site.
    AsDesigned,
    /// The launch day's air and wind, from where [`LaunchDay::said`] says.
    LaunchDay,
}

/// The launch day's conditions, and where they came from in words.
struct LaunchDay {
    environment: Environment,
    said: String,
}

/// Where the site came from.
struct Site {
    from: SiteFrom,
}

/// The flight's options with the site the comparison flies from: the one given, or, where neither
/// --latitude nor --longitude is, the log's GNSS fixes before liftoff (or its first fix), placed
/// as `hpr analyze --ghost` places its pad. A log with no fix leaves the options as they are.
fn sited(
    args: &CompareArgs,
    log: &FlightLog,
    read: &Readings,
) -> Result<(FlightArgs, Site), Failure> {
    let mut flight = args.flight.clone();
    if flight.latitude != 0.0 || flight.longitude != 0.0 {
        return Ok((
            flight,
            Site {
                from: SiteFrom::Given,
            },
        ));
    }
    let name = file_name(&args.log);
    let pad = ghost::build(
        &[ghost::Input {
            name: &name,
            log,
            read,
        }],
        None,
    )
    .ok()
    .map(|ghost| ghost.pad);
    let from = match pad {
        Some(pad) => match (pad.latitude_deg, pad.longitude_deg, pad.from) {
            (
                Some(latitude),
                Some(longitude),
                from @ (PadFrom::FixesBeforeLiftoff | PadFrom::FirstFix),
            ) => {
                flight.latitude = latitude;
                flight.longitude = longitude;
                if from == PadFrom::FixesBeforeLiftoff {
                    SiteFrom::LogFixesBeforeLiftoff
                } else {
                    SiteFrom::LogFirstFix
                }
            }
            _ => SiteFrom::Default,
        },
        None => SiteFrom::Default,
    };
    if from == SiteFrom::Default && args.launch_time.is_some() {
        return Err(Failure::helped(
            "--launch-time: the log holds no GNSS fix to place the site by, and no --latitude or \
             --longitude gives it",
            "give the site with --latitude and --longitude, and its height with --elevation",
        ));
    }
    Ok((flight, Site { from }))
}

/// The launch day's conditions, if any are given: Open-Meteo's archive at `--launch-time`, the
/// `--weather` profile, or the values entered (`--wind`, `--temperature`, `--pressure`).
fn launch_day(
    args: &CompareArgs,
    (log, read): (&FlightLog, &Readings),
    flight: &FlightArgs,
    setup: &Setup,
    site: &Site,
) -> Result<Option<LaunchDay>, Failure> {
    if let Some(time) = &args.launch_time {
        let (time, when) = if time.eq_ignore_ascii_case("log") {
            let unix_s = read
                .liftoff
                .value()
                .and_then(|liftoff| logged_time(log, liftoff.time_s))
                .ok_or_else(|| {
                    Failure::helped(
                        "--launch-time log: the log gives no time of its own at liftoff, as an \
                     AltosUI CSV's GPS columns do with a fix",
                        "give the launch's time in UTC, such as `--launch-time 2023-06-23T23:00Z`",
                    )
                })?;
            (
                crate::weather::format_utc(unix_s),
                "the log's GPS time at liftoff",
            )
        } else {
            (time.clone(), "the time given")
        };
        let fetched = crate::weather::open_meteo(&OpenMeteoArgs {
            latitude: Some(flight.latitude),
            longitude: Some(flight.longitude),
            time: time.clone(),
            historical: true,
            model: None,
            fetching: Fetching {
                from: None,
                offline: flight.offline,
                output: None,
            },
        })?;
        let profile = crate::sim::flyable_profile(
            fetched.sounding,
            flight.elevation,
            "Open-Meteo's answer",
            "give the site's --elevation within the answer's heights; or fetch a profile with \
             `hpr weather` and give it with --weather",
        )?;
        let mut environment = setup.designed.clone();
        if let Some(wind) = profile.wind() {
            environment = environment.with_wind(wind.clone());
        }
        environment = environment.with_atmosphere(profile);
        let said = format!(
            "Open-Meteo's archive of past forecasts for {} ({when}), at its grid point {}, {}, \
             nearest {}; {}. {}",
            crate::weather::format_utc(fetched.time_unix_s),
            crate::sim_text::hemisphere(fetched.position.latitude_deg, "N", "S"),
            crate::sim_text::hemisphere(fetched.position.longitude_deg, "E", "W"),
            match site.from {
                SiteFrom::LogFixesBeforeLiftoff => "the log's GNSS fixes before liftoff",
                SiteFrom::LogFirstFix => "the log's first GNSS fix",
                SiteFrom::Given | SiteFrom::Default => "the site given",
            },
            lowercase_first(&crate::weather::read_from_line(&fetched.read_from)),
            open_meteo::ATTRIBUTION,
        );
        return Ok(Some(LaunchDay { environment, said }));
    }
    let entered_air = args.temperature.is_some() || args.pressure.is_some();
    if flight.weather.is_none() && flight.wind.is_none() && !entered_air {
        return Ok(None);
    }
    // The setup's own environment flies --weather's profile or --wind already.
    let mut environment = setup.environment.clone();
    let mut said = Vec::new();
    if let Some(path) = &flight.weather {
        said.push(format!("the weather profile {}", file_name(path)));
    }
    if let Some(wind) = flight.wind {
        said.push(format!(
            "a wind entered of {} m/s from {}°, the same at every height",
            fixed(wind, 1),
            fixed(flight.wind_from, 0)
        ));
    }
    if entered_air {
        let (air, words) = entered(args.temperature, args.pressure, flight.elevation)?;
        environment = environment.with_atmosphere(air);
        said.push(words);
    } else if flight.weather.is_none() {
        said.push("the 1976 standard atmosphere's air".to_owned());
    }
    Ok(Some(LaunchDay {
        environment,
        said: said.join("; "),
    }))
}

/// `line` with its first letter in lower case, for a line set inside a sentence.
fn lowercase_first(line: &str) -> String {
    let mut chars = line.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_lowercase().chain(chars).collect()
    })
}

/// The time the log's GPS receiver gives at its liftoff, `liftoff_s` on its clock, s since the
/// Unix epoch: from an AltosUI
/// CSV's `year` to `second` columns, the receiver's UTC date and time with a four-digit year
/// (AltOS's telemetry and its flash memory keep two digits, and the flash's date "is not reliably
/// the flight's", so neither is read). The last sample at or before liftoff with a fix (`locked`
/// 1, where the log has the column), or else the first after it; `None` with none, or with a date
/// that isn't one.
fn logged_time(log: &FlightLog, liftoff_s: f64) -> Option<i64> {
    use hpr::hpr_flightdata::log::LogFormat;
    if log.format != LogFormat::AltosCsv {
        return None;
    }
    let column = |name: &str| log.channels.iter().find(|channel| channel.column == name);
    let fields: Vec<_> = ["year", "month", "day", "hour", "minute", "second"]
        .iter()
        .map(|name| column(name))
        .collect::<Option<_>>()?;
    let locked = column("locked");
    let at = |i: usize| -> Option<i64> {
        if locked.is_some_and(|locked| locked.values.get(i) != Some(&1.0)) {
            return None;
        }
        let value = |k: usize| {
            fields[k]
                .values
                .get(i)
                .copied()
                .filter(|v| v.fract() == 0.0)
        };
        // A four-digit year from GPS's start on: a garbled one is no time the log gives.
        let year = value(0).filter(|year| (1980.0..=9999.0).contains(year))?;
        // Whole numbers, checked above, and a year of four digits: each converts exactly.
        #[allow(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "whole and small"
        )]
        let instant = hpr::hpr_io::era5::UtcTime::from_civil(
            year as i32,
            value(1)? as u32,
            value(2)? as u32,
            value(3)? as u32,
            value(4)? as u32,
            value(5)?,
        )
        .ok()?;
        #[allow(
            clippy::cast_possible_truncation,
            reason = "whole seconds of years 1980 to 9999"
        )]
        Some(instant.unix_seconds() as i64)
    };
    let times = &fields[0].time_s;
    let before = (0..times.len())
        .rev()
        .filter(|&i| times[i] <= liftoff_s)
        .find_map(at);
    before.or_else(|| {
        (0..times.len())
            .filter(|&i| times[i] > liftoff_s)
            .find_map(at)
    })
}

/// A number typed on the command line, written back in an error: as typed where it is of an
/// everyday size, else in exponent form, so `1e300` comes back as `1e300` and not as 301 digits.
fn as_given(value: f64) -> String {
    if value == 0.0 || (1e-3..1e6).contains(&value.abs()) || !value.is_finite() {
        format!("{value}")
    } else {
        format!("{value:e}")
    }
}

/// The air of a temperature, °C, and a pressure, hPa, entered at the pad `elevation_m` above sea
/// level, either one the standard atmosphere's where it isn't entered: the 1976 standard shifted to
/// pass through both ([`Ussa76::anchored`]); and those values in words. A temperature outside
/// [`TEMPERATURE_C`], or a pressure further from the standard's than [`PRESSURE_FROM_STANDARD`], is
/// refused: neither is air at a pad, and the second may be a pressure reduced to sea level.
fn entered(
    temperature_c: Option<f64>,
    pressure_hpa: Option<f64>,
    elevation_m: f64,
) -> Result<(Ussa76, String), Failure> {
    let standard = Ussa76::standard().sample(elevation_m).map_err(|error| {
        Failure::library(
            format!("--elevation {}: {error}", as_given(elevation_m)),
            "give the site's height above sea level in meters",
        )
    })?;
    if let Some(celsius) = temperature_c
        && !(TEMPERATURE_C.0..=TEMPERATURE_C.1).contains(&celsius)
    {
        return Err(Failure::helped(
            format!(
                "--temperature {}: air at a pad lies between {} and {} °C",
                as_given(celsius),
                TEMPERATURE_C.0,
                TEMPERATURE_C.1
            ),
            "give the temperature in degrees Celsius, such as `--temperature 31`",
        ));
    }
    let standard_hpa = standard.air.pressure_pa / 100.0;
    if let Some(hpa) = pressure_hpa
        // Written so that a pressure that isn't a number is refused too.
        && !(-PRESSURE_FROM_STANDARD * standard_hpa..=PRESSURE_FROM_STANDARD * standard_hpa)
            .contains(&(hpa - standard_hpa))
    {
        return Err(Failure::helped(
            format!(
                "--pressure {}: the air's pressure at a pad {} m above sea level lies within \
                 {}% of the standard atmosphere's {} hPa there",
                as_given(hpa),
                as_given(elevation_m.round()),
                fixed(100.0 * PRESSURE_FROM_STANDARD, 0),
                fixed(standard_hpa, 1)
            ),
            "give the station's own pressure in hPa, not one reduced to sea level as a weather \
             report gives it; leave --pressure out to fly the standard's",
        ));
    }
    let kelvin = temperature_c.map_or(standard.air.temperature_k, |celsius| celsius + 273.15);
    let pascal = pressure_hpa.map_or(standard.air.pressure_pa, |hpa| hpa * 100.0);
    let air = Ussa76::anchored(elevation_m, kelvin, pascal).map_err(|error| {
        Failure::library(
            format!("--temperature and --pressure: {error}"),
            "give the air at the pad in °C and hPa, such as `--temperature 31 --pressure 856`",
        )
    })?;
    let how = |entered: bool| if entered { "entered" } else { "the standard's" };
    Ok((
        air,
        format!(
            "air of {} °C ({}) and {} hPa ({}) at the pad, the 1976 standard atmosphere shifted \
             to pass through them",
            fixed(kelvin - 273.15, 1),
            how(temperature_c.is_some()),
            fixed(pascal / 100.0, 1),
            how(pressure_hpa.is_some()),
        ),
    ))
}

/// The conditions as designed, in words.
const DESIGNED: &str = "the 1976 standard atmosphere and no wind, at the site";

/// The figure's lines on the conditions: the site, each simulation's conditions, as sentences.
fn conditions_lines(
    flight: &FlightArgs,
    site: &Site,
    designed: &str,
    launch_day: Option<&LaunchDay>,
) -> Vec<String> {
    let from = match site.from {
        SiteFrom::Given => "as given",
        SiteFrom::LogFixesBeforeLiftoff => "from the log's GNSS fixes before liftoff",
        SiteFrom::LogFirstFix => "from the log's first GNSS fix",
        SiteFrom::Default => "the default: no site was given and the log holds no GNSS fix",
    };
    let rail = format!(
        "a {} m rail at {}° above the horizon",
        fixed(flight.rail_length, 2),
        fixed(flight.inclination.unwrap_or(90.0), 1)
    );
    vec![
        format!(
            "Site: {}, {}, {}; {} m above sea level; {rail}.",
            crate::sim_text::hemisphere(flight.latitude, "N", "S"),
            crate::sim_text::hemisphere(flight.longitude, "E", "W"),
            from,
            fixed(flight.elevation, 0),
        ),
        format!("As designed: {designed}."),
        match launch_day {
            Some(day) => format!("Launch day: {}.", day.said),
            None => "Launch day: no conditions given, so none is drawn. Give --launch-time for \
                     the weather source's air and wind, --weather for a profile, or enter --wind, \
                     --temperature and --pressure."
                .to_owned(),
        },
    ]
}

/// The figure's file, its data and their sidecar, as `hpr sim --plot` names them; each refused
/// before anything is read if the figure isn't an .svg file, its folder is missing, or any of them
/// is a file this run reads.
fn plot_files(args: &CompareArgs) -> Result<Option<(String, String, String)>, Failure> {
    let Some(path) = &args.plot else {
        return Ok(None);
    };
    let shown = if path.is_empty() {
        "an empty name".to_owned()
    } else {
        crate::printable(path)
    };
    let svg = Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("svg"));
    if !svg {
        return Err(Failure::helped(
            format!("{shown}: --plot writes an .svg file"),
            "name the figure's file .svg, such as `--plot flight.svg`",
        ));
    }
    if let Some(folder) = Path::new(path).parent()
        && !folder.as_os_str().is_empty()
        && !folder.is_dir()
    {
        return Err(Failure::helped(
            format!("{shown}: there is no folder {}", folder.display()),
            "make the folder first, or write the figure somewhere else",
        ));
    }
    let (data, meta) = crate::sim::plot_files(path);
    let inputs: Vec<_> = crate::sim::read_files(&args.flight)
        .into_iter()
        .chain(
            std::iter::once(&args.log)
                .chain(&args.mapping)
                .chain(&args.layout)
                .map(|path| same_file(path)),
        )
        .collect();
    for written in [path, &data, &meta] {
        if inputs.contains(&same_file(written)) {
            return Err(Failure::helped(
                format!(
                    "{shown}: --plot writes {}, and this run reads that file",
                    crate::printable(written)
                ),
                "give --plot a file of its own; its data and their sidecar are named after it",
            ));
        }
    }
    Ok(Some((path.clone(), data, meta)))
}

/// What the figure is drawn from.
struct Drawn<'a> {
    log: &'a FlightLog,
    read: &'a Readings,
    document: &'a Compare,
    /// The flight as designed and its comparison.
    designed: (&'a Simulated, &'a Comparison),
    /// The launch day's, where its conditions are given.
    launch_day: Option<(&'a Simulated, &'a Comparison)>,
    conditions: Vec<String>,
}

/// Draws the figure and writes it, its data and their sidecar.
fn draw(
    (path, data, meta): &(String, String, String),
    drawn: &Drawn<'_>,
) -> Result<ComparePlot, Failure> {
    let document = drawn.document;
    let (designed_simulated, designed_comparison) = drawn.designed;
    let kind = designed_comparison.height_read_as;
    let logged = residuals::logged_lines(drawn.log, drawn.read, designed_comparison);
    let designed = designed_simulated.read_lines(kind, designed_comparison.simulated_liftoff_s);
    let launch_day = drawn
        .launch_day
        .map(|(simulated, comparison)| simulated.read_lines(kind, comparison.simulated_liftoff_s));
    let designed_rows: Vec<CompareRow> =
        designed_comparison.rows.iter().map(row_document).collect();
    let day_rows: Option<Vec<CompareRow>> = drawn
        .launch_day
        .map(|(_, comparison)| comparison.rows.iter().map(row_document).collect());
    let readings: Vec<crate::plot::comparison::ReadingRow> = designed_rows
        .iter()
        .enumerate()
        .map(|(i, row)| crate::plot::comparison::ReadingRow {
            name: row_name(row.what).to_owned(),
            logged: analyze::together::value_bound_text(row.what, &row.logged),
            from: from_text(&row.logged),
            designed: figure_cell(row.what, &row.logged, &row.simulated),
            launch_day: day_rows.as_ref().and_then(|rows| {
                rows.get(i)
                    .filter(|day| day.what == row.what)
                    .map(|day| figure_cell(day.what, &day.logged, &day.simulated))
            }),
        })
        .collect();
    let apogee = |rows: &[CompareRow]| {
        rows.iter()
            .find(|row| row.what == TimelineWhat::ApogeeHeight)
            .and_then(|row| match row.simulated {
                TimelineCell::Read { value, .. } => Some(value),
                TimelineCell::Withheld { .. } => None,
            })
    };
    let logged_apogee = designed_rows
        .iter()
        .find(|row| row.what == TimelineWhat::ApogeeHeight)
        .and_then(|row| match row.logged {
            TimelineCell::Read { value, .. } => Some(value),
            TimelineCell::Withheld { .. } => None,
        });
    let title = format!(
        "{} against its simulation",
        Path::new(&document.log).file_name().map_or_else(
            || document.log.clone(),
            |name| name.to_string_lossy().into_owned()
        )
    );
    let logger = match document.logger.as_str() {
        "unknown" => String::new(),
        logger => format!("; logged by {logger}"),
    };
    let subtitle = format!("{}, {}{logger}", document.design, document.configuration);
    let catalog_as_of = crate::motors::catalog_as_of()?;
    let speed_withheld = match &designed_comparison.speed_m_s {
        Speed::Withheld { detail } => Some(detail.as_str()),
        Speed::Read(_) => None,
    };
    let clock = format!(
        "Clocks: lined up by a {}, within {} s; the simulation is read as {}.",
        designed_comparison.method,
        seconds_bound(designed_comparison.clock.bound_s),
        match kind {
            HeightKind::Barometric =>
                "a barometer on the pad would read it, as the log's height is",
            HeightKind::Geometric => "the height climbed, as the log's height is",
        }
    );
    let data_name = file_name(data);
    let figure = crate::plot::comparison::Figure {
        title: &title,
        subtitle: &subtitle,
        catalog_as_of: &catalog_as_of,
        data_file: &data_name,
        logged_height: &logged.height_m,
        logged_speed: logged.speed_m_s.as_deref(),
        speed_withheld,
        designed: crate::plot::comparison::Line {
            height: &designed.0,
            speed: &designed.1,
        },
        launch_day: launch_day
            .as_ref()
            .map(|(height, speed)| crate::plot::comparison::Line { height, speed }),
        apogees: (
            logged_apogee,
            apogee(&designed_rows),
            day_rows.as_deref().and_then(apogee),
        ),
        readings: &readings,
        conditions: &drawn.conditions,
        clock: &clock,
        trust: &document.trust,
    };
    let svg = crate::plot::comparison::svg(&figure);
    let (text, rows) = crate::plot::comparison::csv(&figure)
        .map_err(|error| Failure::helped(format!("{data}: {error}"), crate::BUG_HELP))?;
    std::fs::write(path, svg).map_err(|error| crate::unwritable(path, &error))?;
    std::fs::write(data, text).map_err(|error| crate::unwritable(data, &error))?;
    let sidecar = ExportMeta {
        file: data_name.clone(),
        design: document.design.clone(),
        configuration: document.configuration.clone(),
        rows,
        catalog_as_of,
        kind: crate::trust::Kind::Compared,
        trust: document.trust.clone(),
    };
    let mut bytes = Vec::new();
    crate::write_json(&mut bytes, &sidecar)
        .and_then(|()| std::fs::write(meta, bytes))
        .map_err(|error| crate::unwritable(meta, &error))?;
    Ok(ComparePlot {
        path: path.clone(),
        data: data.clone(),
        meta: meta.clone(),
        rows,
    })
}

/// A simulated reading in the figure's table: as the text gives it, but where the log's is
/// withheld too, only that, as the log's column gives the reason the reader withholds both for;
/// and a reason the log's doesn't share, by its first sentence.
fn figure_cell(what: TimelineWhat, logged: &TimelineCell, simulated: &TimelineCell) -> String {
    match (logged, simulated) {
        (TimelineCell::Withheld { .. }, TimelineCell::Withheld { .. }) => "withheld too".to_owned(),
        // The reason's first sentence: the text output gives it whole.
        (_, TimelineCell::Withheld { detail }) => match detail.split_once(". ") {
            Some((first, _)) => format!("withheld: {first}"),
            None => format!("withheld: {detail}"),
        },
        _ => simulated_text(what, simulated),
    }
}

/// Where a logged reading came from: its track and whether it was measured or derived.
fn from_text(cell: &TimelineCell) -> String {
    match cell {
        TimelineCell::Read {
            source, provenance, ..
        } => {
            let source = source.map(analyze::source_words);
            let provenance = provenance.map(analyze::together::provenance_words);
            match (source, provenance) {
                (Some(source), Some(provenance)) => format!("{provenance}, from {source}"),
                (Some(source), None) => format!("from {source}"),
                (None, Some(provenance)) => provenance.to_owned(),
                (None, None) => String::new(),
            }
        }
        TimelineCell::Withheld { .. } => String::new(),
    }
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
            "  {}: {}",
            match document.flown_in.conditions {
                CompareConditions::AsDesigned => "as designed",
                CompareConditions::LaunchDay => "on the launch day",
            },
            document.flown_in.detail
        ),
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
    lines.push("Residuals, logged less simulated".to_owned());
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
    if let Some(plot) = &document.plot {
        lines.push(String::new());
        lines.push(format!(
            "wrote {} (the log beside its simulation: altitude and vertical speed against time)",
            plot.path
        ));
        lines.push(format!(
            "wrote {} (the figure's data, {} rows)",
            plot.data, plot.rows
        ));
        lines.push(format!(
            "wrote {} (the program that wrote {})",
            plot.meta, plot.data
        ));
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

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    reason = "a test stops at the failure (clippy.toml)"
)]
mod tests {
    use hpr::hpr_flightdata::log::{Channel, LogFormat, Quantity, Stated};

    use super::*;

    /// An AltosUI CSV's GPS columns at 0, 1, 2 and 3 s, the receiver locked from `locked_from`.
    fn altos(format: LogFormat, locked_from: usize) -> FlightLog {
        let time_s = vec![0.0, 1.0, 2.0, 3.0];
        let column = |name: &str, values: Vec<f64>| {
            Channel::new(name, Quantity::Number, time_s.clone(), values)
        };
        let locked = (0..4)
            .map(|i| if i >= locked_from { 1.0 } else { 0.0 })
            .collect();
        FlightLog {
            format,
            logger: "AltOS".to_owned(),
            serial_number: None,
            firmware: None,
            flight_number: None,
            stated: Stated::default(),
            time_s: time_s.clone(),
            altitude_m: vec![0.0; 4],
            vertical_speed_m_s: None,
            temperature_k: None,
            battery_v: None,
            channels: vec![
                column("locked", locked),
                column("year", vec![2022.0; 4]),
                column("month", vec![6.0; 4]),
                column("day", vec![24.0; 4]),
                column("hour", vec![15.0; 4]),
                column("minute", vec![17.0; 4]),
                column("second", vec![14.0, 15.0, 16.0, 17.0]),
            ],
            events: Vec::new(),
            notes: Vec::new(),
        }
    }

    /// The GPS time at liftoff is the last locked sample's at or before it, or the first after;
    /// unlocked samples and other formats give none.
    #[test]
    fn the_logs_time_is_its_gps_fix_at_liftoff() {
        // 2022-06-24T15:17:16Z.
        let at_16 = 1_656_083_836;
        assert_eq!(
            logged_time(&altos(LogFormat::AltosCsv, 0), 2.5),
            Some(at_16)
        );
        assert_eq!(
            logged_time(&altos(LogFormat::AltosCsv, 0), 2.0),
            Some(at_16)
        );
        // No fix until after liftoff: the first one after.
        assert_eq!(
            logged_time(&altos(LogFormat::AltosCsv, 3), 1.5),
            Some(at_16 + 1)
        );
        assert_eq!(logged_time(&altos(LogFormat::AltosCsv, 4), 1.5), None);
        // AltOS's telemetry keeps two digits of the year: not read.
        assert_eq!(logged_time(&altos(LogFormat::GenericTable, 0), 2.5), None);
        let mut short = altos(LogFormat::AltosCsv, 0);
        short.channels[1].values = vec![22.0; 4];
        assert_eq!(logged_time(&short, 2.5), None);
        // A garbled year is no time the log gives.
        let mut garbled = altos(LogFormat::AltosCsv, 0);
        garbled.channels[1].values = vec![1e9; 4];
        assert_eq!(logged_time(&garbled, 2.5), None);
    }

    /// Entered air is the standard's where a value isn't entered, and air past the records is
    /// refused.
    #[test]
    fn entered_air_is_checked_and_anchored() {
        // The air flown passes through the values entered, at the pad.
        let (air, _) = entered(Some(35.0), Some(1000.0), 100.0).unwrap();
        let pad = air.sample(100.0).unwrap().air;
        assert!((pad.temperature_k - 308.15).abs() < 1e-9, "{pad:?}");
        assert!((pad.pressure_pa - 100_000.0).abs() < 1e-6, "{pad:?}");
        let (_, words) = entered(Some(35.0), None, 1400.0).unwrap();
        assert!(words.contains("35.0 °C (entered)"), "{words}");
        assert!(words.contains("856."), "{words}");
        assert!(words.contains("hPa (the standard's)"), "{words}");
        assert!(entered(None, Some(1013.25), 1400.0).is_err());
        assert!(entered(None, Some(1013.25), 0.0).is_ok());
        assert!(entered(Some(-91.0), None, 0.0).is_err());
        assert!(entered(Some(f64::NAN), None, 0.0).is_err());
        assert!(entered(None, Some(f64::NAN), 0.0).is_err());
        // A hostile number comes back short, not as hundreds of digits.
        for (temperature, pressure) in [(Some(1e300), None), (None, Some(1e-300))] {
            let Err(failure) = entered(temperature, pressure, 0.0) else {
                panic!("{temperature:?} {pressure:?} flown");
            };
            let said = format!("{failure:?}");
            assert!(said.contains("1e300") || said.contains("1e-300"), "{said}");
            assert!(said.len() < 600, "{said}");
        }
        assert_eq!(as_given(-0.0), "-0");
        assert_eq!(as_given(1013.25), "1013.25");
        assert_eq!(as_given(f64::NAN), "NaN");
    }
}
