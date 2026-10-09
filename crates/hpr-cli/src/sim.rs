//! `hpr sim`: a design flown from a rail, its summary printed and its recording exported.
//!
//! The flight is the library's own: the design becomes an [`hpr::Rocket`] with
//! [`hpr::Rocket::from_design`], and [`hpr::Flight::builder`] flies it, so what this prints is
//! what a program calling the facade gets, number for number. A `.ork`'s parachutes and streamers
//! fly as [`hpr::ork::recovery`] maps them, as OpenRocket flies them (ADR-153). A `.ork`
//! configuration whose booster drops away under power flies that separation, and one that then
//! drops its middle stage flies both (ADR-160), each device on its part and a part with none
//! tumbling, as [`hpr::ork::separated_recovery`] maps them (ADR-159);
//! any other design flies its stack whole, which the output's notes say. What would make the
//! flight another rocket than the file's is refused (ADR-106).

use std::path::{Path, PathBuf};

use hpr::hpr_design::{self, Configuration, Ignition, MountedMotor, checks};
use hpr::hpr_format::{self, container};
use hpr::hpr_io::ork;
use hpr::hpr_motor::text::{ParseWarning, WarningKind as ReadWarning};
use hpr::hpr_motor::{Delay, eng, rse};
use hpr::hpr_net::on_demand::Wanted;
use hpr::hpr_sim::metrics::{self, FlightSummary};
use hpr::hpr_sim::{self, Channel, EnvelopeFlag, FlightSettings, Recorder, export};
use hpr::{Environment, Flight, FlightBuilder, Motor, Rocket};

use crate::choose::{self, Choice};
use crate::motors::MotorFile;
use crate::output::{
    Apogee, DesignConfiguration, DesignFormat, DeviceEvent, EventKind, Export, ExportFormat,
    ExportMeta, FileFormat, FlagKind, InputWarning, IssueKind, Landing, Launch, Margin, Peak,
    SimDesign, SimDevice, SimEvent, SimFlag, SimFlight, SimIssue, SimMotor, SimMotorSource,
    Stability, Summary, Termination, ThrustCurveMotor, WarningKind,
};
use crate::{Failure, Out};

/// How long after apogee the first recovery device may open for the landing to count as braked,
/// s. Within it the rocket has fallen under 5 m (`g t²/2`) and is slower than 10 m/s, so the
/// unbraked fall, on aerodynamics that hold only at small angles of attack, has little to act on.
/// Later than that, the landing and every peak before the opening keep their caveat.
pub const BRAKED_WITHIN_S: f64 = 1.0;

/// The rail's length when `--rail-length` isn't given, m.
pub const DEFAULT_RAIL_LENGTH_M: f64 = 1.5;

/// The recording's interval when `--interval` isn't given, s.
pub const DEFAULT_INTERVAL_S: f64 = 0.01;

/// The finest recording interval `--interval` takes, s: a finer one fills memory on a long fall,
/// at a million rows every 17 minutes of flight.
pub const MIN_INTERVAL_S: f64 = 0.001;

/// `hpr sim`'s arguments.
#[derive(Debug, clap::Args)]
pub struct SimArgs {
    /// The design, its motor and the launch.
    #[command(flatten)]
    pub flight: FlightArgs,
    /// Write the flight's recording to this file: .csv, .json, .parquet, .geojson or .kml
    /// (repeat for several)
    #[arg(long, value_name = "FILE")]
    pub export: Vec<String>,
    /// Draw the flight's altitude, speed and acceleration against time, its events marked, into
    /// this .svg file
    #[arg(long, value_name = "FILE")]
    pub plot: Option<String>,
    /// The recording's interval, s, at least 0.001; a row is also recorded at every event
    #[arg(long, value_name = "S", default_value_t = DEFAULT_INTERVAL_S)]
    pub interval: f64,
}

/// What `hpr sim` and `hpr mc` fly: the design, its motor and the launch.
#[derive(Debug, clap::Args)]
pub struct FlightArgs {
    /// The design: an OpenRocket .ork file, an HPR design (.hpr, or .hprz with its attachments),
    /// or a rocket's JSON (.json)
    pub design: String,
    /// The motor configuration to fly: its number in the list the output shows, its name, or its
    /// id [default: the design's default, or its only one]
    #[arg(long, value_name = "NAME")]
    pub config: Option<String>,
    /// Fly this motor in place of the configuration's, lit at launch: a catalog designation or
    /// common name, or a .eng or .rse file
    #[arg(long, value_name = "NAME_OR_FILE")]
    pub motor: Option<String>,
    /// The motor mount that --motor goes in, by its name or its component id [default: the
    /// configuration's, or the design's only one]
    #[arg(long, value_name = "NAME", requires = "motor")]
    pub mount: Option<String>,
    /// The ejection delay of --motor's motor: seconds from burnout to its charge (0 fires it at
    /// burnout), or P for a plugged motor [default: none, so it fires no charge]
    #[arg(long, value_name = "S_OR_P", requires = "motor", value_parser = DelayArg)]
    pub delay: Option<Delay>,
    /// The launch site's latitude, degrees north (south is negative)
    #[arg(
        long,
        value_name = "DEG",
        default_value_t = 0.0,
        allow_negative_numbers = true
    )]
    pub latitude: f64,
    /// The launch site's longitude, degrees east (west is negative)
    #[arg(
        long,
        value_name = "DEG",
        default_value_t = 0.0,
        allow_negative_numbers = true
    )]
    pub longitude: f64,
    /// The launch site's height above sea level, m
    #[arg(
        long,
        value_name = "M",
        default_value_t = 0.0,
        allow_negative_numbers = true
    )]
    pub elevation: f64,
    /// The rail's length, m, from the rocket's aft end to the rail's top
    #[arg(long, value_name = "M", default_value_t = DEFAULT_RAIL_LENGTH_M)]
    pub rail_length: f64,
    /// The rail's angle above the horizon, degrees [default: 90, vertical]
    #[arg(long, value_name = "DEG")]
    pub inclination: Option<f64>,
    /// The direction the rail leans toward, degrees clockwise from true north [default: 0]
    #[arg(long, value_name = "DEG", allow_negative_numbers = true)]
    pub heading: Option<f64>,
    /// A wind of this speed at every height, m/s [default: calm]
    #[arg(long, value_name = "M_S")]
    pub wind: Option<f64>,
    /// The direction the wind blows from, degrees clockwise from north
    #[arg(
        long,
        value_name = "DEG",
        default_value_t = 0.0,
        requires = "wind",
        allow_negative_numbers = true
    )]
    pub wind_from: f64,
    /// Fly a design whose checks find errors, such as a motor wider than its mount
    #[arg(long)]
    pub accept_design_errors: bool,
    /// Take a motor the bundled catalog lacks from the cache of ThrustCurve.org only, never the
    /// network
    #[arg(long)]
    pub offline: bool,
}

/// Runs `hpr sim`.
pub(crate) fn run(args: &SimArgs, to: &mut Out<'_>) -> Result<(), Failure> {
    // Everything that can be refused is refused before the flight: the exports, the recording's
    // interval, the design, the motor, the site.
    let (exports, plot) = exports(args)?;
    if args.interval.is_nan() || args.interval < MIN_INTERVAL_S {
        return Err(Failure::Input(format!(
            "--interval {}: the recording's interval is at least {MIN_INTERVAL_S} s",
            args.interval
        )));
    }
    let mut recorder = Recorder::new(Channel::ALL.to_vec(), Some(args.interval))
        .map_err(|error| Failure::Input(format!("--interval: {error}")))?;
    let setup = Setup::read(&args.flight)?;
    let builder = setup.builder(&args.flight);
    let mut trace = plot.map(|_| crate::plot::Trace::new(args.interval));
    let flight = match (&mut trace, exports.is_empty()) {
        (None, true) => builder.fly(),
        (None, false) => builder.fly_with(&mut recorder),
        (Some(trace), true) => builder.fly_with(trace),
        (Some(trace), false) => builder.fly_with(&mut (&mut recorder, trace)),
    }
    .map_err(input)?;
    let motors = setup.motors()?;
    drop(builder);
    let Setup {
        mut read,
        configurations,
        configuration,
        configuration_name,
        configuration_label,
        flown_motors: _,
        motor_source: _,
        placed: _,
        names: _,
        design_name,
        rocket: _,
        environment,
        devices,
        separations,
        added,
    } = setup;

    let recovery = recovery_flown(
        &devices,
        (devices.len() - added, &separations),
        flight.result(),
    );
    // The stack's, or the part's that keeps the nose after a split with nothing left to burn.
    let apogee_s = flight.summary().apogee.map(|apogee| apogee.time_s);
    let braked = braked(&recovery, apogee_s);
    read.notes.extend(flight_notes(&recovery, &flight));
    let mut written = Vec::new();
    let name = format!("{design_name}, {configuration_label}");
    let catalog_as_of = crate::motors::catalog_as_of()?;
    let about = About {
        design: file_name(&args.flight.design),
        configuration: configuration_label.clone(),
        catalog_as_of: catalog_as_of.clone(),
        kind: crate::trust::Kind::Simulated,
        trust: crate::trust::flight(),
    };
    for (path, format, meta) in &exports {
        let contents = contents(
            *format,
            &recorder,
            &environment,
            flight.summary(),
            (&name, &about),
            braked,
        )
        .map_err(|error| Failure::Input(format!("{path}: {error}")))?;
        std::fs::write(path, contents)
            .map_err(|error| Failure::Input(format!("{path}: {error}")))?;
        if let Some(meta) = meta {
            let document = ExportMeta {
                file: file_name(path),
                design: file_name(&args.flight.design),
                configuration: configuration_label.clone(),
                rows: recorder.rows().len(),
                catalog_as_of: catalog_as_of.clone(),
                kind: crate::trust::Kind::Simulated,
                trust: crate::trust::flight(),
            };
            let mut text = Vec::new();
            crate::write_json(&mut text, &document)
                .and_then(|()| std::fs::write(meta, text))
                .map_err(|error| Failure::Input(format!("{meta}: {error}")))?;
        }
        written.push(Export {
            path: (*path).to_owned(),
            format: *format,
            rows: recorder.rows().len(),
            meta: meta.clone(),
        });
    }
    if let (Some(path), Some(trace)) = (plot, &trace) {
        // The fall is hatched from apogee to the first opening, or the end, as the summary marks
        // its peaks there.
        let unpredicted = apogee_s.filter(|_| !braked).map(|apogee_s| {
            let end_s = trace.points().last().map_or(apogee_s, |p| p.time_s);
            let to_s = first_opened(&recovery)
                .map(|(_, opened_s)| opened_s)
                .filter(|opened_s| *opened_s > apogee_s)
                .unwrap_or(end_s);
            (apogee_s, to_s)
        });
        let devices: Vec<String> = recovery.iter().map(|d| d.name.clone()).collect();
        let configuration_line = format!(
            "{}, configuration {configuration_label}",
            file_name(&args.flight.design)
        );
        let figure = crate::plot::svg(&crate::plot::Figure {
            title: &design_name,
            subtitle: &configuration_line,
            points: trace.points(),
            events: &flight.result().events,
            devices: &devices,
            unpredicted,
            apogee: flight.summary().apogee.map(|apogee| crate::plot::Reading {
                value: apogee.height_above_ground_m,
                time_s: apogee.time_s,
                unpredicted: false,
            }),
            // Marked as the summary's line marks it.
            top_speed: flight.summary().max_speed_m_s.map(|speed| {
                let first_opened_s = first_opened(&recovery).map(|(_, opened_s)| opened_s);
                crate::plot::Reading {
                    value: speed.value,
                    time_s: speed.time_s,
                    unpredicted: !in_fall(&peak(speed, apogee_s), first_opened_s).is_empty(),
                }
            }),
        });
        std::fs::write(path, figure).map_err(|error| Failure::Input(format!("{path}: {error}")))?;
    }
    // The figure and the exports follow the stack, which ends where it comes apart with nothing
    // left to burn; the parts' descents keep only their events.
    if flight.result().termination == hpr_sim::Termination::Separated
        && (plot.is_some() || !exports.is_empty())
    {
        read.notes.push(format!(
            "the figure and the exported rows end at the split, at {:.3} s: after it, each part's \
             flight is in the events and landings only",
            flight.result().final_sample.time_s
        ));
    }

    let document = SimFlight {
        kind: crate::trust::Kind::Simulated,
        trust: crate::trust::flight(),
        design: SimDesign {
            file: file_name(&args.flight.design),
            format: read.format,
            name: design_name,
            configuration,
            configuration_name,
            configuration_label,
            configurations,
        },
        motors,
        recovery,
        launch: launch(&args.flight),
        summary: summary(flight.summary()),
        events: events(flight.result()),
        exports: written,
        plot: plot.map(str::to_owned),
        notes: read.notes,
        warnings: read.warnings,
        flags: flags(flight.summary()),
        issues: issues(&flight),
    };
    to.emit(&document, |out, diagnostics| {
        crate::sim_text::print(&document, out, diagnostics)
    })
}

/// A design read and its configuration chosen, as `hpr sim` and `hpr mc` fly it: the rocket with
/// its devices, the launch site and what the output says about them.
pub(crate) struct Setup {
    pub(crate) read: Read,
    pub(crate) configurations: Vec<DesignConfiguration>,
    /// The flown configuration's id.
    pub(crate) configuration: String,
    pub(crate) configuration_name: String,
    /// The configuration as the output names it, with `--motor` and `--delay` where given.
    pub(crate) configuration_label: String,
    pub(crate) flown_motors: Vec<MountedMotor>,
    pub(crate) motor_source: SimMotorSource,
    /// Each motor as the assembly places it: a cluster's tubes and a pod set's pods each carry
    /// one.
    pub(crate) placed: Vec<hpr_design::PlacedMotor>,
    pub(crate) names: Vec<(String, String)>,
    pub(crate) design_name: String,
    pub(crate) rocket: Rocket,
    pub(crate) environment: Environment,
    /// The devices flown, the last [`Setup::added`] of them HPR Sim's own (a separated flight's
    /// tumbles).
    pub(crate) devices: Vec<hpr::Device>,
    pub(crate) separations: Vec<hpr::Separation>,
    pub(crate) added: usize,
}

impl Setup {
    /// Reads `flight`'s design, chooses its configuration and motor, checks the design, and
    /// makes the rocket, its devices and the launch site. Everything that can be refused before
    /// a flight is refused here.
    pub(crate) fn read(flight: &FlightArgs) -> Result<Self, Failure> {
        // A `.ork` motor with no curve of its own is fetched for the configuration flown, unless
        // `--motor` replaces it.
        let fetching = flight.motor.is_none().then_some(Fetching {
            config: flight.config.as_deref(),
            offline: flight.offline,
        });
        let mut read = read_design(&flight.design, fetching)?;
        let configurations = read.configurations();
        let (configuration, motor_source) = match &flight.motor {
            Some(name) => {
                let (motor, source, warnings) = motor(name, flight.offline)?;
                let motor = match flight.delay {
                    Some(delay) => motor
                        .with_delay(delay)
                        .map_err(|error| Failure::Input(format!("--delay: {error}")))?,
                    None => motor,
                };
                read.warnings.extend(warnings);
                let id = read.swap(flight.config.as_deref(), flight.mount.as_deref(), &motor)?;
                (id, source)
            }
            None => (
                read.flown(flight.config.as_deref())?,
                SimMotorSource::Design,
            ),
        };
        if matches!(motor_source, SimMotorSource::ThrustCurve(_)) || !read.fetched.is_empty() {
            read.notes
                .push(hpr::hpr_net::thrustcurve::ATTRIBUTION.to_owned());
        }
        let configuration_name = configurations
            .iter()
            .find(|c| c.id == configuration)
            .map(|c| c.name.clone())
            .unwrap_or_default();
        // As flown: after `--motor`, the file's configuration and the motor that replaced its own.
        let flown_label = read
            .rocket
            .configuration(&configuration)
            .map_or_else(|| configuration.clone(), hpr_label);
        let configuration_label = match (
            configurations.iter().find(|c| c.id == configuration),
            flight.motor.is_some(),
        ) {
            (Some(file), false) => file.label.clone(),
            (Some(file), true) => format!(
                "{} with --motor {}{}",
                file.label,
                flight
                    .motor
                    .as_deref()
                    .map_or_else(String::new, crate::printable),
                flight.delay.map_or_else(String::new, |delay| format!(
                    " --delay {}",
                    choose::delay_label(delay)
                ))
            ),
            (None, _) => flown_label,
        };
        // Only the configuration flown: the design's checks look at every configuration, and
        // another's error isn't this flight's.
        read.rocket.configurations.retain(|c| c.id == configuration);
        let flown_motors = read
            .rocket
            .configurations
            .first()
            .map(|flown| flown.motors.clone())
            .unwrap_or_default();
        // A `.ork` configuration's powered separations, flown as the file has them; `--motor` flies
        // none, as a motor of the user's own can't be told when to separate (`Read::swap` refuses a
        // staged configuration).
        let stagings: Vec<ork::Staging> = read
            .ork
            .as_ref()
            .filter(|_| flight.motor.is_none())
            .and_then(|motors| motors.configurations.iter().find(|c| c.id == configuration))
            .map(|chosen| chosen.stagings().cloned().collect())
            .unwrap_or_default();
        // Otherwise the stack flies whole, so a motor lit by its stage's separation would never light.
        if let Some(motor) = flown_motors
            .iter()
            .find(|motor| matches!(motor.ignition, Ignition::Separation { .. }))
            .filter(|_| stagings.is_empty())
        {
            return Err(Failure::Input(format!(
                "configuration {configuration_label} lights {} at its stage's separation, which hpr \
                 sim doesn't fly yet; the library does, as its two-stage examples show",
                motor.designation
            )));
        }
        let named = names(
            &read.rocket,
            &configurations,
            (&configuration, &configuration_label),
        );
        let (errors, findings): (Vec<_>, Vec<_>) = checks::check(&read.rocket)
            .map_err(|error| unheld(&error, &named))?
            .into_iter()
            .partition(|finding| finding.severity() == checks::Severity::Error);
        if !errors.is_empty() {
            if !flight.accept_design_errors {
                return Err(Failure::helped(
                    format!(
                        "the design's checks found {}: {}",
                        count(errors.len(), "error"),
                        findings_text(&errors, &named)
                    ),
                    "--accept-design-errors flies it anyway, and says so in a note",
                ));
            }
            read.notes.push(format!(
                "flown with --accept-design-errors past {}, so its numbers are not a buildable \
                 rocket's: {}",
                count(errors.len(), "error"),
                findings_text(&errors, &named)
            ));
        }
        read.warnings
            .extend(findings.iter().map(|finding| InputWarning {
                at: "design checks".to_owned(),
                kind: WarningKind::Unusual,
                message: finding.describe(&named),
            }));
        let assembly = read
            .rocket
            .assemble(&configuration)
            .map_err(|error| unheld(&error, &named))?;
        // The devices, and how many at the end of the list hpr added (a separated flight's tumbles).
        let (devices, separations, added) = if stagings.is_empty() {
            (flown_recovery(&mut read, &assembly), Vec::new(), 0)
        } else {
            flown_separation(&mut read, &assembly, &stagings)?
        };
        // How many of each motor fly: a cluster's tubes and a pod set's pods each carry one.
        let placed = assembly.motors;
        let names = component_names(&read.rocket);
        let design_name = read.rocket.name.clone();

        let mut rocket = Rocket::from_design(read.rocket.clone(), &configuration).map_err(input)?;
        for device in &devices {
            rocket.add_parachute(device.clone());
        }
        let mut environment = Environment::new(flight.latitude, flight.longitude, flight.elevation)
            .map_err(|error| Failure::Input(format!("the launch site: {error}")))?;
        if let Some(speed) = flight.wind {
            environment = environment
                .with_constant_wind(speed, flight.wind_from)
                .map_err(|error| Failure::Input(format!("the wind: {error}")))?;
        }
        Ok(Self {
            read,
            configurations,
            configuration,
            configuration_name,
            configuration_label,
            flown_motors,
            motor_source,
            placed,
            names,
            design_name,
            rocket,
            environment,
            devices,
            separations,
            added,
        })
    }

    /// The flight's builder, from `flight`'s rail. The builder's own rail unless an angle is
    /// given, so a vertical launch is the library's default to the bit.
    pub(crate) fn builder(&self, flight: &FlightArgs) -> FlightBuilder<'_> {
        let mut builder = Flight::builder(&self.rocket, &self.environment, flight.rail_length);
        if let Some(inclination) = flight.inclination {
            builder = builder.inclination_deg(inclination);
        }
        if let Some(heading) = flight.heading {
            builder = builder.heading_deg(heading);
        }
        if !self.separations.is_empty() {
            builder = builder.separations(self.separations.clone());
        }
        if flight.accept_design_errors {
            builder = builder.settings(FlightSettings {
                accept_design_errors: true,
                ..FlightSettings::default()
            });
        }
        builder
    }

    /// The motors flown, as the output lists them.
    pub(crate) fn motors(&self) -> Result<Vec<SimMotor>, Failure> {
        self.flown_motors
            .iter()
            .map(|flown| {
                let mine = self.placed.iter().filter(|p| {
                    p.mount == flown.mount && p.mounted.designation == flown.designation
                });
                Ok(SimMotor {
                    designation: flown.designation.clone(),
                    mount: flown.mount.clone(),
                    mount_name: self
                        .names
                        .iter()
                        .find(|(id, _)| *id == flown.mount)
                        .map(|(_, name)| name.clone())
                        .unwrap_or_default(),
                    count: mine.clone().count(),
                    unlit: mine.filter(|p| p.fails).count(),
                    source: self
                        .read
                        .fetched
                        .iter()
                        .find(|fetched| {
                            fetched.mount == flown.mount && fetched.designation == flown.designation
                        })
                        .map_or_else(
                            || self.motor_source.clone(),
                            |fetched| SimMotorSource::ThrustCurve(Box::new(fetched.motor.clone())),
                        ),
                    ignition: ignition(&flown.ignition, &self.names),
                    delay: flown.delay.map(crate::motors::delay_out).transpose()?,
                })
            })
            .collect::<Result<_, Failure>>()
    }
}

/// The launch, as the output states it.
pub(crate) fn launch(flight: &FlightArgs) -> Launch {
    Launch {
        latitude_deg: flight.latitude,
        longitude_deg: flight.longitude,
        elevation_m: flight.elevation,
        rail_length_m: flight.rail_length,
        inclination_deg: flight.inclination.unwrap_or(90.0),
        heading_deg: flight.heading.unwrap_or(0.0),
        wind_speed_m_s: flight.wind.unwrap_or(0.0),
        wind_from_deg: flight.wind_from,
        atmosphere: "standard".to_owned(),
    }
}

/// A library error, as the command reports it.
pub(crate) fn input(error: hpr::Error) -> Failure {
    Failure::Input(error.to_string())
}

/// The refusal of a design that doesn't hold together, its parts by name.
fn unheld(error: &hpr_design::DesignError, name: &dyn Fn(&str) -> String) -> Failure {
    Failure::Input(format!(
        "the design doesn't hold together: {}",
        design_error(error, name)
    ))
}

/// A design error in words, the part at fault by `name` rather than its id.
fn design_error(error: &hpr_design::DesignError, name: &dyn Fn(&str) -> String) -> String {
    match error {
        hpr_design::DesignError::Tree { id, message } => format!("{}: {message}", name(id)),
        hpr_design::DesignError::InComponent { id, source } => {
            format!("{}: {}", name(id), design_error(source, name))
        }
        other => other.to_string(),
    }
}

/// Findings as sentences, one after another ([`checks::Finding::describe`]).
fn findings_text(findings: &[checks::Finding], name: &dyn Fn(&str) -> String) -> String {
    findings
        .iter()
        .map(|finding| finding.describe(name))
        .collect::<Vec<_>>()
        .join("; ")
}

/// What the output calls a part, stage or configuration of `rocket` by its id: a part's or
/// stage's name, quoted, or a configuration's label, the one `flown` as flown; an id it doesn't
/// know, as it is.
fn names(
    rocket: &hpr_design::Rocket,
    configurations: &[DesignConfiguration],
    flown: (&str, &str),
) -> impl Fn(&str) -> String + use<> {
    let mut known: Vec<(String, String)> = vec![(flown.0.to_owned(), flown.1.to_owned())];
    known.extend(component_names(rocket).into_iter().map(|(id, name)| {
        let label = choose::part_label(&id, &name);
        (id, format!("`{label}`"))
    }));
    known.extend(rocket.stages.iter().map(|stage| {
        (
            stage.id.clone(),
            format!("`{}`", choose::part_label(&stage.id, &stage.name)),
        )
    }));
    known.extend(
        configurations
            .iter()
            .map(|c| (c.id.clone(), c.label.clone())),
    );
    move |id: &str| {
        known
            .iter()
            .find(|(known, _)| known == id)
            .map_or_else(|| crate::printable(id), |(_, label)| label.clone())
    }
}

/// `n` things, in words: `1 error`, `2 errors`.
fn count(n: usize, thing: &str) -> String {
    if n == 1 {
        format!("1 {thing}")
    } else {
        format!("{n} {thing}s")
    }
}

/// The file name of a path, for the output: the path's folder depends on where it was run from.
pub(crate) fn file_name(path: &str) -> String {
    Path::new(path).file_name().map_or_else(
        || path.to_owned(),
        |name| name.to_string_lossy().into_owned(),
    )
}

/// `--delay`: seconds from burnout to the charge, at least 0, or `P` (either case) for a plugged
/// motor. `-0` is read as 0.
fn delay_arg(text: &str) -> Option<Delay> {
    if text.eq_ignore_ascii_case("p") {
        return Some(Delay::Plugged);
    }
    match text.parse::<f64>() {
        Ok(delay_s) if delay_s.is_finite() && delay_s >= 0.0 => Some(Delay::Seconds(delay_s + 0.0)),
        _ => None,
    }
}

/// `--delay`'s parser: [`delay_arg`], refusing anything else as clap refuses a value, with what
/// to give as a hint, which `hpr` writes as a `help:` line.
#[derive(Debug, Clone, Copy)]
struct DelayArg;

impl clap::builder::TypedValueParser for DelayArg {
    type Value = Delay;

    fn parse_ref(
        &self,
        command: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Delay, clap::Error> {
        use clap::error::{ContextKind, ContextValue, ErrorKind};
        let text = value.to_string_lossy();
        delay_arg(&text).ok_or_else(|| {
            let mut error = clap::Error::new(ErrorKind::InvalidValue).with_cmd(command);
            if let Some(arg) = arg {
                error.insert(
                    ContextKind::InvalidArg,
                    ContextValue::String(arg.to_string()),
                );
            }
            error.insert(
                ContextKind::InvalidValue,
                ContextValue::String(text.into_owned()),
            );
            error.insert(
                ContextKind::Suggested,
                ContextValue::StyledStrs(vec![
                    "give the seconds from burnout to the charge, such as 10, or P for a \
                     plugged motor"
                        .into(),
                ]),
            );
            error
        })
    }
}

/// The files a run writes: the `--export` files with their formats and, for a CSV file, its
/// sidecar ([`sidecar`]); and the `--plot` file.
type Written<'a> = (
    Vec<(&'a str, ExportFormat, Option<String>)>,
    Option<&'a str>,
);

/// Where the sidecar of a CSV recording goes: beside it, `.meta.json` for `.csv`, as
/// `flight.meta.json` for `flight.csv`. A CSV opens in a spreadsheet only without comment lines,
/// so the program that wrote it is named there (the product system's `data.md`, ADR-174).
pub(crate) fn sidecar(path: &str) -> String {
    Path::new(path)
        .with_extension("meta.json")
        .to_string_lossy()
        .into_owned()
}

/// The files a flight reads: the design, and `--motor`'s where it names a file.
pub(crate) fn read_files(flight: &FlightArgs) -> Vec<PathBuf> {
    std::iter::once(flight.design.as_str())
        .chain(
            flight
                .motor
                .as_deref()
                .filter(|motor| MotorFile::of(motor).is_some()),
        )
        .map(same_file)
        .collect()
}

/// The `--export` files and their formats, and the `--plot` file, refused before the flight if a
/// format is unknown, a file's folder is missing, a file is given twice, or a file is one this run
/// reads.
fn exports(args: &SimArgs) -> Result<Written<'_>, Failure> {
    let inputs = read_files(&args.flight);
    let mut seen = Vec::new();
    let mut exports = Vec::new();
    let plot = args.plot.as_deref();
    let sidecars: Vec<(String, &str)> = args
        .export
        .iter()
        .filter(|path| matches!(export_format(path), Ok(ExportFormat::Csv)))
        .map(|path| (sidecar(path), path.as_str()))
        .collect();
    let written = args
        .export
        .iter()
        .map(|path| (path.as_str(), "--export"))
        .chain(plot.map(|path| (path, "--plot")));
    for (path, option) in written {
        let format = if option == "--plot" {
            let svg = Path::new(path)
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("svg"));
            if !svg {
                return Err(Failure::Input(format!(
                    "{path}: --plot writes an .svg file"
                )));
            }
            None
        } else {
            Some(export_format(path)?)
        };
        let file = same_file(path);
        if inputs.contains(&file) {
            return Err(Failure::Input(format!(
                "{path}: this run reads that file, and {option} would write over it"
            )));
        }
        if seen.contains(&file) {
            return Err(Failure::Input(format!(
                "{path}: {option} names a file twice"
            )));
        }
        if let Some(folder) = Path::new(path).parent()
            && !folder.as_os_str().is_empty()
            && !folder.is_dir()
        {
            return Err(Failure::Input(format!(
                "{path}: there is no folder {}",
                folder.display()
            )));
        }
        seen.push(file);
        if let Some(format) = format {
            let meta = (format == ExportFormat::Csv).then(|| sidecar(path));
            exports.push((path, format, meta));
        }
    }
    // A CSV file's sidecar is written too, so it may not be a file this run reads or writes.
    for (meta, csv) in &sidecars {
        let file = same_file(meta);
        if inputs.contains(&file) || seen.contains(&file) {
            return Err(Failure::Input(format!(
                "{csv}: --export writes its sidecar {meta} beside it, and this run reads or \
                 writes that file too"
            )));
        }
        seen.push(file);
    }
    Ok((exports, plot))
}

/// A path as the file system knows it, to compare two: the file's canonical path if it exists,
/// or its folder's with its name.
pub(crate) fn same_file(path: &str) -> PathBuf {
    let path = Path::new(path);
    if let Ok(canonical) = path.canonicalize() {
        return canonical;
    }
    let folder = match path.parent() {
        Some(folder) if !folder.as_os_str().is_empty() => folder,
        _ => Path::new("."),
    };
    match (folder.canonicalize(), path.file_name()) {
        (Ok(folder), Some(name)) => folder.join(name),
        _ => path.to_path_buf(),
    }
}

/// A design read, with the configurations it could fly and what the reader said.
pub(crate) struct Read {
    pub(crate) format: DesignFormat,
    rocket: hpr_design::Rocket,
    /// The `.ork` file's own motor configurations, flyable or not; `None` for an HPR design.
    ork: Option<ork::Motors>,
    /// Why a `.ork` rocket wasn't read exactly as written, if it wasn't
    /// ([`ork::airframe_not_as_written`]).
    airframe: Option<String>,
    /// The file's recovery settings, where it has any to read.
    recovery: Option<ork::Recovery>,
    pub(crate) notes: Vec<String>,
    pub(crate) warnings: Vec<InputWarning>,
    /// The `.ork` motors whose curves were fetched from ThrustCurve.org.
    fetched: Vec<FetchedMotor>,
    /// Why a `.ork` motor with no curve wasn't fetched, each a sentence for the refusal.
    unfetched: Vec<String>,
    /// What fetches them, each a `help:` line for the refusal.
    fetch_help: Vec<String>,
}

/// A `.ork` motor that flies a curve fetched from ThrustCurve.org.
struct FetchedMotor {
    /// The motor's mount, by its component id.
    mount: String,
    /// The motor's designation, as the file writes it.
    designation: String,
    /// The digest the file records, which the curve was supplied for.
    digest: String,
    /// What was fetched.
    motor: ThrustCurveMotor,
    /// Whether the file is the one holding OpenRocket's curve for the digest
    /// ([`crate::openrocket_curves`]).
    openrocket: bool,
}

/// The curves [`fetch_missing`] fetched, and why any wasn't.
#[derive(Default)]
struct Missing {
    supplied: ork::SuppliedCurves,
    fetched: Vec<FetchedMotor>,
    unfetched: Vec<String>,
    fetch_help: Vec<String>,
}

/// Fetches each motor of the configuration `fetching` names (or the file's default) that has no
/// curve in the file or the bundled catalog, by its manufacturer and designation, and supplies it
/// for the digest the file records. A motor with no digest can't be supplied, and one that can't
/// be fetched isn't: each says why in [`Missing::unfetched`], and the configuration's refusal
/// carries it.
fn fetch_missing(motors: &ork::Motors, fetching: Fetching<'_>) -> Result<Missing, Failure> {
    let mut missing = Missing {
        supplied: ork::SuppliedCurves::new(FETCHED_CURVES),
        ..Missing::default()
    };
    let Some(chosen) = ork_configuration(motors, fetching.config)? else {
        return Ok(missing);
    };
    for motor in &chosen.motors {
        if !matches!(
            motor.curve,
            ork::Curve::Unresolved {
                why: ork::NoCurve::NotFound,
                ..
            }
        ) {
            continue;
        }
        let Some(digest) = &motor.digest else {
            missing.unfetched.push(format!(
                "{} records no digest, which a fetched curve is supplied by",
                motor.designation
            ));
            continue;
        };
        // A digest fetched for one motor supplies every motor that records it, whatever its
        // designation: each flies, and is named as flying, that curve.
        if let Some(earlier) = missing.fetched.iter().find(|f| f.digest == *digest) {
            let shared = FetchedMotor {
                mount: motor.mount.clone(),
                designation: motor.designation.clone(),
                digest: digest.clone(),
                motor: earlier.motor.clone(),
                openrocket: earlier.openrocket,
            };
            missing.fetched.push(shared);
            continue;
        }
        let mut wanted = Wanted::by(&motor.manufacturer, &motor.designation);
        let openrocket = crate::openrocket_curves::file_for(digest);
        if let Some(file) = &openrocket {
            wanted = wanted.preferring(file);
        }
        match crate::motor_fetch::fetch(&wanted, fetching.offline, "hpr sim") {
            Ok((fetched, described)) => {
                let case = ork::CaseSize {
                    diameter_m: fetched.diameter_m(),
                    length_m: fetched.length_m(),
                };
                missing
                    .supplied
                    .insert(digest.clone(), case, fetched.solid_motor().clone())
                    .map_err(|error| Failure::Input(format!("{}: {error}", wanted.words())))?;
                missing.fetched.push(FetchedMotor {
                    mount: motor.mount.clone(),
                    designation: motor.designation.clone(),
                    digest: digest.clone(),
                    openrocket: openrocket.as_deref() == Some(described.file_id.as_str()),
                    motor: described,
                });
            }
            Err(Failure::Input(why)) => missing.unfetched.push(why),
            Err(Failure::Helped { message, help }) => {
                missing.unfetched.push(message);
                missing.fetch_help.extend(help);
            }
            Err(other) => return Err(other),
        }
    }
    Ok(missing)
}

/// Why `hpr sim --offline --config CONFIG PATH` refuses to fly a `.ork`'s configuration as the
/// file has it: the reader's reason ([`ork::NotFlown`]) for the configuration as `hpr sim` reads
/// it, with the curves it would fetch taken from the cache; `None` when the reader left nothing
/// out, and an error when the file can't be read or names no such configuration. For
/// `cargo xtask ork-cli`'s count of what `hpr sim` flies (M4.5j); the refusal a person reads is
/// [`run`]'s.
pub fn left_out(path: &str, config: &str) -> Result<Option<ork::NotFlown>, String> {
    let message = |failure: Failure| match failure {
        Failure::Input(message) => message,
        Failure::Helped { message, help } => format!("{message}; {}", help.join("; ")),
        _ => format!("{path}: not read"),
    };
    let fetching = Fetching {
        config: Some(config),
        offline: true,
    };
    let read = read_design(path, Some(fetching)).map_err(message)?;
    let motors = read
        .ork
        .as_ref()
        .ok_or_else(|| format!("{path}: not a .ork file"))?;
    let chosen = ork_configuration(motors, Some(config))
        .map_err(message)?
        .ok_or_else(|| format!("{path}: no configuration {config}"))?;
    Ok(chosen.left_out.as_ref().map(|left_out| left_out.why))
}

/// The note on the descent, after what the design says of its recovery.
fn descent(recovery: &str) -> String {
    format!(
        "{recovery}, so the rocket falls from apogee on its airframe alone, on aerodynamics that \
         hold only at small angles of attack: its landing time, speed and place, \
         and any peak it sets in the fall, are not a prediction"
    )
}

/// The recovery devices of `read`'s file for the configuration `assembly` assembled, as
/// [`hpr::ork::recovery`] maps them, with the notes on what flies and what doesn't. A refused
/// mapping flies none, and says why.
fn flown_recovery(read: &mut Read, assembly: &hpr_design::Assembly) -> Vec<hpr::Device> {
    let Some(recovery) = read
        .recovery
        .as_ref()
        .filter(|r| !r.devices.is_empty() || !r.unread.is_empty())
    else {
        return Vec::new();
    };
    match hpr::ork::recovery(recovery, &read.rocket, assembly) {
        Ok(recovered) => {
            for (name, why) in &recovered.not_deployed {
                read.notes
                    .push(format!("`{name}` never opens: {}", why.reason()));
            }
            read.notes.push(if recovered.devices.is_empty() {
                descent("no recovery device of the file opens in this configuration")
            } else {
                let one = recovered.devices.len() == 1;
                format!(
                    "the file's {} as OpenRocket flies {}: each opens fully at its event, with \
                     the file's drag coefficient or OpenRocket's own, and once one opens the \
                     rocket descends as a point under the open devices' drag alone",
                    if one {
                        "1 recovery device flies".to_owned()
                    } else {
                        format!("{} recovery devices fly", recovered.devices.len())
                    },
                    if one { "it" } else { "them" },
                )
            });
            recovered.devices
        }
        Err(refused) => {
            read.notes.push(descent(&format!(
                "the file's recovery is not flown ({refused})"
            )));
            Vec::new()
        }
    }
}

/// The stage after which the first misplaced boundary of `separations` falls: the first at or
/// aft of the one before it, or of the last of `stage_count` stages
/// ([`hpr_sim::Separation::stages_of_body`]'s rule); the first's when none is.
fn misplaced(separations: &[hpr::Separation], stage_count: usize) -> usize {
    let mut last = stage_count.checked_sub(1);
    separations
        .iter()
        .find(|separation| {
            let misplaced = last.is_none_or(|last| separation.after_stage >= last);
            last = Some(separation.after_stage);
            misplaced
        })
        .or(separations.first())
        .map_or(0, |separation| separation.after_stage)
}

/// The separations of a `.ork` configuration that drops its booster under power (and, for three
/// stages, its middle stage after it), and the devices [`hpr::ork::separated_recovery`] puts on
/// each part, with the notes on what flies, and how many devices at the end of the list HPR Sim added
/// (the tumbles). A refused mapping flies the tumbles alone ([`hpr::ork::tumbling`]), and says
/// why.
///
/// # Errors
///
/// [`Failure::Input`] when a separation can't be timed from the configuration's motors, has no
/// stage aft of it, or a part can't tumble.
fn flown_separation(
    read: &mut Read,
    assembly: &hpr_design::Assembly,
    stagings: &[ork::Staging],
) -> Result<(Vec<hpr::Device>, Vec<hpr::Separation>, usize), Failure> {
    let separations = hpr::ork::separations(stagings, assembly)
        .map_err(|error| Failure::Input(format!("the file's separation: {error}")))?;
    // A hand-edited `.hpr` can name a boundary at or past the last stage, or out of order.
    let stage_count = assembly.layout.stages.len();
    let parts = (1..=separations.len())
        .map(|body| {
            hpr_sim::Separation::stages_of_body(&separations, body, stage_count).ok_or_else(|| {
                Failure::Input(format!(
                    "the file's separation: no stage is aft of the boundary after stage {}, or \
                     it is not forward of the one before",
                    misplaced(&separations, stage_count).saturating_add(1)
                ))
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let refused = |error: hpr::ork::RecoveryRefused| Failure::Input(error.to_string());
    let tumbles = || hpr::ork::tumbling(Vec::new(), &read.rocket, assembly, &separations);
    let mapped = match &read.recovery {
        Some(recovery) => {
            hpr::ork::separated_recovery(recovery, &read.rocket, assembly, &separations)
                .map(|recovered| (recovered.devices, recovered.not_deployed, recovered.added))
        }
        None => tumbles().map(|devices| {
            let added = devices.len();
            (devices, Vec::new(), added)
        }),
    };
    let (devices, added) = match mapped {
        Ok((devices, not_deployed, added)) => {
            for (name, why) in &not_deployed {
                read.notes
                    .push(format!("`{name}` never opens: {}", why.reason()));
            }
            (devices, added)
        }
        Err(
            error @ (hpr::ork::RecoveryRefused::Tumbling { .. }
            | hpr::ork::RecoveryRefused::Coasts { .. }),
        ) => return Err(refused(error)),
        Err(error) => {
            // A part that keeps the nose with nothing to burn and no device can't tumble from
            // its apogee in place of one: it would coast from the split with no drag.
            let devices = tumbles().map_err(|tumbling| match tumbling {
                hpr::ork::RecoveryRefused::Coasts { .. } => Failure::Input(format!(
                    "{tumbling}, as the file's recovery is not flown ({error})"
                )),
                other => refused(other),
            })?;
            read.notes.push(format!(
                "the file's recovery is not flown ({error}), so each part tumbles: {} from {} \
                 split, the sustainer from its apogee",
                if parts.len() == 1 {
                    "the booster"
                } else {
                    "each dropped part"
                },
                if parts.len() == 1 { "the" } else { "its" },
            ));
            let added = devices.len();
            (devices, added)
        }
    };
    // The stack doesn't fly whole, and a part with no device of the file's tumbles.
    let unrecovered = descent("the file has no recovery device");
    read.notes
        .retain(|note| note != WHOLE_STACK && note != ORK_WHOLE_STACK && *note != unrecovered);
    // Each dropped part's stages by name, as the file writes them.
    let named = |(first, last): (usize, usize)| -> String {
        let names: Vec<String> = read
            .rocket
            .stages
            .get(first..=last)
            .unwrap_or_default()
            .iter()
            .enumerate()
            .map(|(offset, stage)| {
                if stage.name.trim().is_empty() {
                    format!("stage {}", first + offset + 1)
                } else {
                    format!("`{}`", stage.name)
                }
            })
            .collect();
        list(&names)
    };
    // A lone split with nothing ahead of it left to burn hands the flight on to no sustainer.
    let unpowered = match separations.as_slice() {
        [separation] => !separation.powered_at(assembly, stagings[0].time_s),
        _ => false,
    };
    let note = match parts.as_slice() {
        [part] if unpowered => format!(
            "the stack comes apart at {:.3} s with nothing left to burn: part 1, {}, drops away, \
             and from the split each part flies as a point with only its own devices' drag, as \
             OpenRocket flies a descent; part 1 tumbles side-on until a device of its own opens, \
             where a real booster flies nose-first for a while, and its landing is not pinned on \
             a map",
            stagings[0].time_s,
            named(*part),
        ),
        [part] => format!(
            "the stack comes apart under power at {:.3} s: part 1, {}, drops away, and the \
             sustainer, part 0, flies on; part 1 flies as a point with only its devices' drag, \
             tumbling side-on from the split until a device of its own opens, where a real \
             booster flies nose-first for a while, so its peak and landing are rough, likely too \
             low and too close to the pad, and its landing is not pinned on a map",
            stagings[0].time_s,
            named(*part),
        ),
        _ => {
            let drops: Vec<String> = parts
                .iter()
                .zip(stagings)
                .enumerate()
                .map(|(index, (part, staging))| {
                    format!(
                        "at {:.3} s part {}, {}, drops away",
                        staging.time_s,
                        index + 1,
                        named(*part)
                    )
                })
                .collect();
            format!(
                "the stack comes apart under power {} times: {}; the sustainer, part 0, flies \
                 on; each dropped part flies as a point with only its devices' drag, tumbling \
                 side-on from its split until a device of its own opens, where a real stage \
                 flies nose-first for a while, so their peaks and landings are rough, likely too \
                 low and too close to the pad, and their landings are not pinned on a map",
                parts.len(),
                list(&drops),
            )
        }
    };
    read.notes.push(note);
    // A part dropped at its stage's first burnout may take a motor still burning, whose thrust
    // its flight as a point leaves out (ADR-172).
    let lit = assembly.ignition_times_s(|_| None);
    for (index, (&(first, last), staging)) in parts.iter().zip(stagings).enumerate() {
        let (mut left_n_s, mut total_n_s) = (0.0, 0.0);
        let mut burning: Vec<String> = Vec::new();
        for (motor, lit_s) in assembly.motors.iter().zip(&lit) {
            if let Some(lit_s) = *lit_s
                && (first..=last).contains(&motor.stage)
                && lit_s <= staging.time_s
                && lit_s + motor.mounted.motor.burnout_time_s() > staging.time_s
            {
                let curve = motor.mounted.motor.curve();
                left_n_s += curve.total_impulse_ns() - curve.impulse_ns(staging.time_s - lit_s);
                total_n_s += curve.total_impulse_ns();
                burning.push(format!("`{}`", motor.mounted.designation));
            }
        }
        if !burning.is_empty() {
            read.notes.push(format!(
                "part {} drops at {:.3} s with {} still burning, as OpenRocket drops a stage's \
                 other motors at its first burnout: its flight as a point leaves out the {:.3} \
                 N·s they had left, of {:.3} N·s in all",
                index + 1,
                staging.time_s,
                list(&burning),
                left_n_s,
                total_n_s,
            ));
        }
    }
    let from_file = &devices[..devices.len() - added];
    if from_file.iter().all(|device| device.body != 0) {
        read.notes.push(
            "the sustainer has no device of the file's that opens, so HPR Sim tumbles it side-on \
             from its apogee: its landing time, speed and place are not a prediction"
                .to_owned(),
        );
    }
    if !from_file.is_empty() {
        read.notes.push(
            "each of the file's devices opens fully at its event, its part's own (a part's \
             apogee is its own), with the file's drag coefficient or OpenRocket's own, and once \
             one opens its part descends as a point under the open devices' drag alone"
                .to_owned(),
        );
    }
    Ok((devices, separations, added))
}

/// What a flown flight's own numbers add to its notes: a fall from apogee before the first device
/// opened, a device set to open above the apogee, and a dropped part that climbs above the
/// flight's apogee. `recovery` is [`recovery_flown`]'s.
pub(crate) fn flight_notes(recovery: &[SimDevice], flight: &Flight) -> Vec<String> {
    let mut notes = Vec::new();
    let apogee_s = flight.summary().apogee.map(|apogee| apogee.time_s);
    let braked = braked(recovery, apogee_s);
    if let (Some((first, opened_s)), Some(apogee_s)) = (first_opened(recovery), apogee_s)
        && !braked
    {
        notes.push(format!(
            "the rocket fell {:.1} s from apogee before `{first}` opened, on aerodynamics that \
             hold only at small angles of attack: where it lands, and any peak in that fall, are \
             not a prediction",
            opened_s - apogee_s
        ));
    }
    if let Some(apogee_m) = flight.apogee_m() {
        for device in recovery {
            if let Some(height_m) = device
                .height_above_ground_m
                .filter(|height_m| *height_m > apogee_m)
            {
                notes.push(format!(
                    "`{}` is set to open {height_m} m ({}) above the site, above the apogee, so \
                     it opened at apogee; OpenRocket's flight of such a file never opens it",
                    device.name,
                    crate::units::feet(height_m)
                ));
            }
        }
    }
    // The flight's apogee is the part's that keeps the nose; a dropped part that climbs higher
    // is said, as a waiver's height is the highest any part reaches.
    if let Some(apogee) = flight
        .summary()
        .apogee
        .filter(|_| flight.result().termination == hpr_sim::Termination::Separated)
    {
        for body in flight.result().bodies.iter().filter(|body| body.body > 0) {
            if let Some(peak) = body
                .event(hpr_sim::EventKind::Apogee)
                .filter(|peak| peak.sample.height_above_ground_m > apogee.height_above_ground_m)
            {
                notes.push(format!(
                    "part {} peaks at {} above the site at {:.2} s, above the flight's apogee, \
                     which is part 0's",
                    body.body,
                    crate::units::meters(peak.sample.height_above_ground_m),
                    peak.sample.time_s
                ));
            }
        }
    }
    notes
}

/// The device of the rocket, or the sustainer that keeps its nose, that opened first, by name,
/// and when: a separated part's devices are its own descent's, and a tumble HPR Sim added brakes no
/// fall it can predict.
pub(crate) fn first_opened(recovery: &[SimDevice]) -> Option<(&str, f64)> {
    recovery
        .iter()
        .filter(|device| device.body.is_none() && !device.added)
        .filter_map(|device| Some((device.name.as_str(), device.opened_s?)))
        .min_by(|a, b| a.1.total_cmp(&b.1))
}

/// Whether the descent counts as braked: a device opened, the first no later than
/// [`BRAKED_WITHIN_S`] after apogee (`apogee_s`; with none, any opening counts).
pub(crate) fn braked(recovery: &[SimDevice], apogee_s: Option<f64>) -> bool {
    first_opened(recovery).is_some_and(|(_, opened_s)| {
        apogee_s.is_none_or(|apogee_s| opened_s <= apogee_s + BRAKED_WITHIN_S)
    })
}

/// The devices flown, for the output, with when each opened in `result`; those from index
/// `from_file` on are HPR Sim's own, added for `separations`.
pub(crate) fn recovery_flown(
    devices: &[hpr::Device],
    (from_file, separations): (usize, &[hpr::Separation]),
    result: &hpr_sim::FlightResult,
) -> Vec<SimDevice> {
    // Each separation's time, in the order they fired: separation `k` drops body `k + 1`.
    let separated_s: Vec<f64> = result
        .events
        .iter()
        .filter(|event| event.kind == hpr_sim::EventKind::Separation)
        .map(|event| event.sample.time_s)
        .collect();
    devices
        .iter()
        .enumerate()
        .map(|(index, device)| {
            // A separated part's devices act once it flies alone: one timed before then opens as
            // it does.
            let at_split = device.body > 0
                && matches!(device.trigger, hpr::Trigger::Time { time_s }
                    if separated_s
                        .get(device.body - 1)
                        .is_some_and(|&separated_s| time_s <= separated_s));
            // One on the part that keeps the nose with a split's own trigger is
            // `hpr::ork::separated_recovery`'s `lowerstageseparation`, its delay its lag.
            let on_split = device.body == 0
                && separations
                    .iter()
                    .any(|separation| separation.trigger == device.trigger);
            let (opens_at, height_above_ground_m) = match device.trigger {
                _ if at_split || on_split => (DeviceEvent::Separation, None),
                hpr::Trigger::Apogee => (DeviceEvent::Apogee, None),
                hpr::Trigger::Altitude {
                    height_above_ground_m,
                } => (DeviceEvent::Altitude, Some(height_above_ground_m)),
                hpr::Trigger::MotorDelay { .. } => (DeviceEvent::Ejection, None),
                _ => (DeviceEvent::Launch, None),
            };
            let delay_s = match device.trigger {
                _ if at_split || on_split => device.lag_s,
                hpr::Trigger::Time { time_s } => time_s,
                _ => device.lag_s,
            };
            let deployed = hpr_sim::EventKind::Deployment(index);
            SimDevice {
                name: device.name.clone(),
                body: (device.body > 0).then_some(device.body),
                added: index >= from_file,
                opens_at,
                height_above_ground_m,
                delay_s,
                drag_area_m2: device.drag.drag_area_m2(),
                // A device of the part that keeps the nose opens on the stack, or after a split
                // with nothing left to burn on that part flying alone.
                opened_s: result
                    .events
                    .iter()
                    .filter(|_| device.body == 0)
                    .find(|event| event.kind == deployed)
                    .map(|event| event.sample.time_s)
                    .or_else(|| {
                        result
                            .bodies
                            .iter()
                            .filter(|body| body.body == device.body)
                            .find_map(|body| body.event(deployed))
                            .map(|event| event.sample.time_s)
                    }),
            }
        })
        .collect()
}

/// The note for a design of more than one stage that flies no powered separation.
const WHOLE_STACK: &str = "the stages fly as one stack: hpr sim flies a .ork file's powered \
                           separations only, so none comes apart";

/// The note for a `.ork` configuration whose stages all stay on, for the reasons the reader's
/// staging leaves a separation out of a configuration it flies.
const ORK_WHOLE_STACK: &str = "the stages fly as one stack: none of the file's separations comes \
                               apart in this configuration before apogee (each is set to never \
                               happen, waits on a motor its stage doesn't hold or never lights, \
                               as OpenRocket keeps such a stage on, or comes at or after apogee)";

/// Which `.ork` configuration's missing curves to fetch, and whether from the cache alone.
#[derive(Debug, Clone, Copy)]
struct Fetching<'a> {
    /// The `--config` asked for, if any.
    config: Option<&'a str>,
    /// `--offline`.
    offline: bool,
}

/// What [`SuppliedCurves`](ork::SuppliedCurves) are named as where a fetched curve flies.
const FETCHED_CURVES: &str = "ThrustCurve.org, found by the motor's manufacturer and designation";

/// Reads a `.ork`, an HPR design (`.hpr` or `.hprz`) or a rocket's JSON, by its extension. For a
/// `.ork`, a motor of the configuration `fetching` names that has no curve in the file or the
/// bundled catalog is fetched from ThrustCurve.org by its manufacturer and designation
/// ([`crate::motor_fetch`]), and supplied for the digest the file records.
fn read_design(path: &str, fetching: Option<Fetching<'_>>) -> Result<Read, Failure> {
    let extension = Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase);
    let bytes = || std::fs::read(path).map_err(|error| Failure::Input(format!("{path}: {error}")));
    let refused = |error: &dyn std::fmt::Display| Failure::Input(format!("{path}: {error}"));
    match extension.as_deref() {
        Some("ork") => {
            let file = ork::read(&bytes()?).map_err(|e| refused(&e))?;
            let mut design = ork::design(&file.value);
            let missing = match fetching {
                Some(fetching) => fetch_missing(&design.value.motors, fetching)?,
                None => Missing::default(),
            };
            if !missing.supplied.is_empty() {
                design = ork::design_with(&file.value, &missing.supplied);
            }
            let mut warnings = file.warnings;
            warnings.extend(design.warnings);
            let mut notes: Vec<String> = Vec::new();
            for fetched in &missing.fetched {
                let whose = if fetched.openrocket {
                    "the file whose curve is the one OpenRocket flies for the digest the file \
                     records, as HPR Sim's table of OpenRocket's examples' motors matches them"
                } else {
                    "found by the manufacturer and designation of the motor whose digest it \
                     records, it may not be the curve OpenRocket flies"
                };
                let note = format!(
                    "`{}` has no curve in the file or the bundled catalog, so it flies \
                     ThrustCurve.org's {} {}, file {}: {whose}",
                    fetched.designation,
                    crate::printable(&fetched.motor.manufacturer),
                    crate::printable(&fetched.motor.designation),
                    crate::motor_fetch::file_words(&fetched.motor)
                );
                if !notes.contains(&note) {
                    notes.push(note);
                }
            }
            let mut read = Read::of(
                DesignFormat::Ork,
                design.value,
                ork::airframe_not_as_written(&file.value),
                notes,
                warnings.iter().map(ork_warning).collect(),
            );
            read.fetched = missing.fetched;
            read.unfetched = missing.unfetched;
            read.fetch_help = missing.fetch_help;
            Ok(read)
        }
        Some(kind @ ("hpr" | "hprz")) => {
            let (document, written_as, attachments, format) = if kind == "hpr" {
                let text = String::from_utf8(bytes()?)
                    .map_err(|_| Failure::Input(format!("{path}: an .hpr file is UTF-8 text")))?;
                let opened = hpr_format::read_json(&text).map_err(|e| refused(&e))?;
                (opened.value, opened.written_as, 0, DesignFormat::Hpr)
            } else {
                let opened = container::read(&bytes()?).map_err(|e| refused(&e))?;
                let attachments = opened.value.attachments.len();
                (
                    opened.value.design,
                    opened.written_as,
                    attachments,
                    DesignFormat::Hprz,
                )
            };
            let mut notes = Vec::new();
            if written_as != hpr_format::VERSION {
                notes.push(format!(
                    "the design is version {written_as} of the HPR design format, read as version \
                     {}",
                    hpr_format::VERSION
                ));
            }
            if attachments > 0 {
                notes.push(format!(
                    "the container's {} {} not read",
                    count(attachments, "attachment"),
                    if attachments == 1 { "is" } else { "are" },
                ));
            }
            let airframe = document
                .provenance
                .source
                .as_ref()
                .and_then(|source| source.airframe_not_as_written.clone());
            Ok(Read::of(
                format,
                document.design(),
                airframe,
                notes,
                Vec::new(),
            ))
        }
        Some("json") => {
            let rocket: hpr_design::Rocket = serde_json::from_slice(&bytes()?)
                .map_err(|error| Failure::Input(format!("{path}: not a rocket's JSON: {error}")))?;
            let mut notes = vec![descent("a rocket's JSON holds no recovery devices")];
            if rocket.stages.len() > 1 {
                notes.push(WHOLE_STACK.to_owned());
            }
            Ok(Read {
                format: DesignFormat::HprJson,
                rocket,
                ork: None,
                airframe: None,
                recovery: None,
                notes,
                warnings: Vec::new(),
                fetched: Vec::new(),
                unfetched: Vec::new(),
                fetch_help: Vec::new(),
            })
        }
        _ => Err(Failure::Input(format!(
            "{path}: hpr sim reads an OpenRocket .ork file, an HPR design (.hpr or .hprz), or a \
             rocket's JSON (.json)"
        ))),
    }
}

impl Read {
    /// A design read as HPR Sim's `.ork` reader models it, with the notes on what `hpr sim` doesn't
    /// fly after `notes`.
    fn of(
        format: DesignFormat,
        design: ork::Design,
        airframe: Option<String>,
        mut notes: Vec<String>,
        warnings: Vec<InputWarning>,
    ) -> Self {
        // A file with devices gets its note once the configuration's are mapped.
        if design.recovery.devices.is_empty() && design.recovery.unread.is_empty() {
            notes.push(descent("the file has no recovery device"));
        }
        if design.rocket.stages.len() > 1 {
            notes.push(ORK_WHOLE_STACK.to_owned());
        }
        if design.is_reduced() {
            notes.push(
                "the file has parts HPR Sim keeps aside instead of flying, such as a parallel \
                 stage: the rocket flown is the rest of it"
                    .to_owned(),
            );
        }
        Self {
            format,
            rocket: design.rocket,
            ork: Some(design.motors),
            airframe,
            recovery: Some(design.recovery),
            notes,
            warnings,
            fetched: Vec::new(),
            unfetched: Vec::new(),
            fetch_help: Vec::new(),
        }
    }

    /// Every configuration of the file, and whether it flies as the file has it.
    fn configurations(&self) -> Vec<DesignConfiguration> {
        match &self.ork {
            Some(motors) => motors
                .configurations
                .iter()
                .map(|c| {
                    let not_flown = not_flown(c, self.rocket.configuration(&c.id).is_some());
                    DesignConfiguration {
                        id: c.id.clone(),
                        name: c.name.clone(),
                        label: ork_label(c),
                        flies: not_flown.is_none(),
                        not_flown,
                    }
                })
                .collect(),
            None => self
                .rocket
                .configurations
                .iter()
                .map(|c| {
                    let not_flown = c
                        .motors
                        .iter()
                        .any(|m| matches!(m.ignition, Ignition::Separation { .. }))
                        .then(|| "a motor lit at a separation".to_owned());
                    DesignConfiguration {
                        id: c.id.clone(),
                        name: c.name.clone(),
                        label: hpr_label(c),
                        flies: not_flown.is_none(),
                        not_flown,
                    }
                })
                .collect(),
        }
    }

    /// The configuration flown as the design has it: `--config`, or the default or only one.
    fn flown(&self, wanted: Option<&str>) -> Result<String, Failure> {
        let id = match &self.ork {
            Some(motors) => {
                let chosen = ork_configuration(motors, wanted)?.ok_or_else(|| {
                    unchosen(&ork_choices(motors), "the file", self.no_motor_flies(None))
                })?;
                if let Some(left_out) = &chosen.left_out {
                    let mut why: String = self
                        .unfetched
                        .iter()
                        .map(|why| format!("; {why}"))
                        .collect();
                    let mut help = self.fetch_help.clone();
                    match self.no_motor_flies(Some(chosen)) {
                        None => help.push("give a motor with --motor".to_owned()),
                        Some(blocked) if blocked == left_out.message => {
                            why.push_str("; nor can it with --motor, for that reason");
                        }
                        Some(blocked) => {
                            why.push_str(&format!("; nor can it with --motor: {blocked}"));
                        }
                    }
                    let flying: Vec<String> = self
                        .configurations()
                        .into_iter()
                        .enumerate()
                        .filter(|(_, c)| c.flies)
                        .map(|(index, c)| format!("{} {}", index + 1, c.label))
                        .collect();
                    if !flying.is_empty() {
                        help.push(format!(
                            "--config flies one of the file's configurations that fly as \
                             written: {}",
                            list(&flying)
                        ));
                    }
                    return Err(Failure::Helped {
                        message: format!(
                            "configuration {} can't be flown as the file has it: {}{why}",
                            ork_label(chosen),
                            left_out.message
                        ),
                        help,
                    });
                }
                chosen.id.clone()
            }
            None => self
                .hpr_configuration(wanted)?
                .ok_or_else(|| unchosen(&hpr_choices(&self.rocket), "the design", None))?
                .id
                .clone(),
        };
        // A configuration the reader didn't leave out is among the rocket's; this is its check.
        if self.rocket.configuration(&id).is_none() {
            let label = self
                .configurations()
                .into_iter()
                .find(|c| c.id == id)
                .map_or(id, |c| c.label);
            return Err(Failure::Input(format!(
                "configuration {label} can't be flown as the file has it"
            )));
        }
        Ok(id)
    }

    /// Why no motor of the user's own flies the `.ork` rocket in `chosen` (or, with none, in a
    /// configuration of its own), if none does: its airframe or stages aren't what `hpr sim` flies
    /// whole, or the configuration is left out for more than its motor. A configuration's
    /// `LeftOut` names only its first reason, so each is asked of the file directly.
    fn no_motor_flies(&self, chosen: Option<&ork::MotorConfiguration>) -> Option<String> {
        if let Some(why) = &self.airframe {
            return Some(if why == hpr_format::migrate::UNKNOWN {
                format!("whether the airframe was read exactly as written is {why}")
            } else {
                format!("the airframe was not read exactly as written: {why}")
            });
        }
        if let Some(chosen) = chosen {
            if let Some(staging) = chosen.stagings().next() {
                return Some(separation_text(staging));
            }
            if !chosen.inactive_stages.is_empty() {
                return Some(format!(
                    "configuration {} switches a stage off, and HPR Sim flies every stage",
                    ork_label(chosen)
                ));
            }
            if !chosen.unread.is_empty() {
                return Some(format!(
                    "a motor of configuration {} is in a part HPR Sim doesn't read",
                    ork_label(chosen)
                ));
            }
        }
        let stages = self.rocket.stages.len();
        if stages > 1 {
            return Some(format!(
                "the rocket has {stages} stages, and hpr sim can't yet tell when they would \
                 separate with a motor of your own"
            ));
        }
        match chosen.and_then(|chosen| chosen.left_out.as_ref()) {
            Some(left_out) if !motor_fixes(left_out.why) => Some(left_out.message.clone()),
            _ => None,
        }
    }

    /// Puts `motor`, lit at launch, in the configuration `--config` names, or the default or
    /// only one, or where there is none to choose, in a configuration named after the motor; and
    /// returns that configuration's id. A `.ork` rocket that no motor of the user's flies as the
    /// file's is refused ([`Read::no_motor_flies`]).
    fn swap(
        &mut self,
        wanted: Option<&str>,
        mount: Option<&str>,
        motor: &Motor,
    ) -> Result<String, Failure> {
        let chosen: Option<(String, String, Vec<String>)> = match &self.ork {
            Some(motors) => {
                let chosen = ork_configuration(motors, wanted)?;
                if let Some(why) = self.no_motor_flies(chosen) {
                    let what = chosen.map_or_else(
                        || "the rocket".to_owned(),
                        |c| format!("configuration {}", ork_label(c)),
                    );
                    return Err(Failure::Input(format!(
                        "{what} can't be flown with a motor of your own: {why}"
                    )));
                }
                chosen.map(|c| {
                    (
                        c.id.clone(),
                        ork_label(c),
                        owned(c.motors.iter().map(|m| &m.mount)),
                    )
                })
            }
            None => self.hpr_configuration(wanted)?.map(|c| {
                (
                    c.id.clone(),
                    hpr_label(c),
                    owned(c.motors.iter().map(|m| &m.mount)),
                )
            }),
        };
        let (id, label, configured) = chosen.unwrap_or_else(|| {
            let designation = motor.designation().to_owned();
            (designation.clone(), designation, Vec::new())
        });
        let mount = self.mount(mount, &label, &configured)?;
        self.fit(&id, &mount, motor);
        Ok(id)
    }

    /// The mount `--motor` goes in, by its id: `--mount`, by its name or id, or the
    /// configuration's one motor's, or the design's only mount. `label` names the configuration.
    fn mount(
        &self,
        given: Option<&str>,
        label: &str,
        configured: &[String],
    ) -> Result<String, Failure> {
        let mounts = mounts(&self.rocket);
        let names = component_names(&self.rocket);
        let named = |id: &str| {
            let name = names.iter().find(|(i, _)| i == id).map_or("", |(_, n)| n);
            choose::part_label(id, name)
        };
        let choices: Vec<Choice<'_>> = mounts
            .iter()
            .map(|id| Choice {
                id,
                label: named(id),
            })
            .collect();
        if configured.len() > 1 {
            let configured: Vec<String> = configured.iter().map(|id| named(id)).collect();
            return Err(Failure::Input(format!(
                "configuration {label} has {} motors, in {}; --motor flies one motor in place \
                 of them all, so it takes a configuration of one",
                configured.len(),
                list(&configured)
            )));
        }
        match (given, configured, &mounts[..]) {
            (Some(given), _, _) => {
                let index = choose::pick(&choices, given, false, "motor mount", "the design")?;
                Ok(mounts[index].clone())
            }
            (None, [mount], _) | (None, [], [mount]) => Ok(mount.clone()),
            (None, [], []) => Err(Failure::Input(
                "the design has no motor mount for --motor to go in".to_owned(),
            )),
            (None, _, _) => Err(Failure::helped(
                format!("the design has {} motor mounts", mounts.len()),
                format!(
                    "say which with --mount: {}",
                    choose::listed(&choices, false)
                ),
            )),
        }
    }

    /// Puts `motor` in `mount`, lit at launch, as configuration `id`'s only motor.
    fn fit(&mut self, id: &str, mount: &str, motor: &Motor) {
        let motors = vec![MountedMotor {
            mount: mount.to_owned(),
            designation: motor.designation().to_owned(),
            diameter_m: motor.diameter_m(),
            length_m: motor.length_m(),
            motor: motor.solid_motor().clone(),
            delay: motor.delay(),
            ignition: Ignition::Launch,
            failed_tubes: Vec::new(),
        }];
        let configurations = &mut self.rocket.configurations;
        match configurations.iter_mut().find(|c| c.id == id) {
            Some(existing) => existing.motors = motors,
            None => configurations.push(Configuration {
                id: id.to_owned(),
                name: String::new(),
                motors,
            }),
        }
    }

    /// An HPR design's configuration: `--config`, or its only one; `None` if it has none, or
    /// several and none was named.
    fn hpr_configuration(&self, wanted: Option<&str>) -> Result<Option<&Configuration>, Failure> {
        let configurations = &self.rocket.configurations;
        match (wanted, &configurations[..]) {
            (Some(wanted), _) => {
                let choices = hpr_choices(&self.rocket);
                let index =
                    choose::pick(&choices, wanted, true, "motor configuration", "the design")?;
                Ok(configurations.get(index))
            }
            (None, [only]) => Ok(Some(only)),
            (None, _) => Ok(None),
        }
    }
}

/// Why `hpr sim` doesn't fly a `.ork` configuration as read, in a few words, or `None` if it
/// does; `read`, whether the rocket holds it.
fn not_flown(configuration: &ork::MotorConfiguration, read: bool) -> Option<String> {
    let Some(left_out) = &configuration.left_out else {
        return (!read).then(|| "not read".to_owned());
    };
    Some(
        match left_out.why {
            ork::NotFlown::NoCurve => {
                // A motor the bundled catalog lacks is fetched by the digest the file records.
                let unfetchable = configuration.motors.iter().any(|m| {
                    matches!(
                        m.curve,
                        ork::Curve::Unresolved {
                            why: ork::NoCurve::NotFound,
                            ..
                        }
                    ) && m.digest.is_none()
                        || matches!(m.curve, ork::Curve::Unresolved { why, .. } if why != ork::NoCurve::NotFound)
                });
                if unfetchable {
                    "a motor with no curve"
                } else {
                    "its motor's curve to fetch"
                }
            }
            ork::NotFlown::UnreadMotor => "a motor in a part HPR Sim doesn't read",
            ork::NotFlown::NoMotor => "no motor",
            ork::NotFlown::InactiveStage => "a stage switched off",
            ork::NotFlown::NoSize => "a motor of no stated size",
            ork::NotFlown::IgnitionNotFlown => "an ignition HPR Sim doesn't fly",
            ork::NotFlown::AirframeNotAsWritten => "the airframe not read as written",
            ork::NotFlown::SeparationNotFlown => "a separation HPR Sim doesn't fly",
            _ => "not flown",
        }
        .to_owned(),
    )
}

/// Whether a motor of the user's own flies a configuration HPR Sim left out for this reason: it
/// does when the reason is the file's motor, and not when it is the airframe, a stage or a
/// separation.
fn motor_fixes(why: ork::NotFlown) -> bool {
    matches!(
        why,
        ork::NotFlown::NoMotor
            | ork::NotFlown::NoCurve
            | ork::NotFlown::NoSize
            | ork::NotFlown::IgnitionNotFlown
    )
}

/// The refusal when no configuration was named and none can be taken as the one; `blocked`,
/// why no motor of the user's own flies the rocket either, if none does.
fn unchosen(choices: &[Choice<'_>], what: &str, blocked: Option<String>) -> Failure {
    if choices.is_empty() {
        match blocked {
            None => Failure::helped(
                format!("{what} has no motor configuration"),
                "give a motor with --motor",
            ),
            Some(why) => Failure::Input(format!(
                "{what} has no motor configuration, nor can it fly with --motor: {why}"
            )),
        }
    } else {
        Failure::helped(
            format!(
                "{what} has {} configurations and names none the default",
                choices.len()
            ),
            format!(
                "say which with --config, by number or name: {}",
                choose::listed(choices, true)
            ),
        )
    }
}

/// A `.ork` file's configuration: `--config`, by number, name or id ([`choose::pick`]; an id's
/// case ignored, as OpenRocket's ids are UUIDs), or the file's default, or its only one; `None`
/// if it has none, or several, none of them the default, and none was named.
fn ork_configuration<'a>(
    motors: &'a ork::Motors,
    wanted: Option<&str>,
) -> Result<Option<&'a ork::MotorConfiguration>, Failure> {
    let configurations = &motors.configurations;
    match wanted {
        Some(wanted) => {
            let index = choose::pick(
                &ork_choices(motors),
                wanted,
                true,
                "motor configuration",
                "the file",
            )?;
            Ok(configurations.get(index))
        }
        None => Ok(
            match (motors.default_configuration(), &configurations[..]) {
                (Some(default), _) | (None, [default]) => Some(default),
                (None, _) => None,
            },
        ),
    }
}

/// Why a motor of the user's own can't fly a configuration that separates.
fn separation_text(staging: &ork::Staging) -> String {
    format!(
        "stage {} drops away at {:.3} s, a time set by the file's motors, so hpr sim \
         can't tell when it would with a motor of your own",
        staging.after_stage.saturating_add(1),
        staging.time_s
    )
}

/// A `.ork` configuration's name for the output ([`choose::configuration_label`]).
fn ork_label(configuration: &ork::MotorConfiguration) -> String {
    choose::configuration_label(
        &configuration.name,
        configuration
            .motors
            .iter()
            .map(|m| (m.designation.as_str(), m.delay)),
    )
}

/// An HPR design's configuration's name for the output ([`choose::configuration_label`]).
fn hpr_label(configuration: &Configuration) -> String {
    choose::configuration_label(
        &configuration.name,
        configuration
            .motors
            .iter()
            .map(|m| (m.designation.as_str(), m.delay)),
    )
}

/// A `.ork` file's configurations to choose from, in the file's order.
fn ork_choices(motors: &ork::Motors) -> Vec<Choice<'_>> {
    motors
        .configurations
        .iter()
        .map(|c| Choice {
            id: &c.id,
            label: ork_label(c),
        })
        .collect()
}

/// An HPR design's configurations to choose from, in the design's order.
fn hpr_choices(rocket: &hpr_design::Rocket) -> Vec<Choice<'_>> {
    rocket
        .configurations
        .iter()
        .map(|c| Choice {
            id: &c.id,
            label: hpr_label(c),
        })
        .collect()
}

/// Every motor mount's component id, in the design's order.
fn mounts(rocket: &hpr_design::Rocket) -> Vec<String> {
    let mut mounts = Vec::new();
    walk(rocket, &mut |component| {
        if component.motor_mount.is_some() {
            mounts.push(component.id.clone());
        }
    });
    mounts
}

/// Every component's id and name.
fn component_names(rocket: &hpr_design::Rocket) -> Vec<(String, String)> {
    let mut names = Vec::new();
    walk(rocket, &mut |component| {
        names.push((component.id.clone(), component.name.clone()));
    });
    names
}

/// Calls `visit` on every component, attached parts after their parent's.
fn walk(rocket: &hpr_design::Rocket, visit: &mut dyn FnMut(&hpr_design::Component)) {
    fn each(components: &[hpr_design::Component], visit: &mut dyn FnMut(&hpr_design::Component)) {
        for component in components {
            visit(component);
            each(&component.children, visit);
        }
    }
    for stage in &rocket.stages {
        each(&stage.components, visit);
    }
}

/// Owned copies of ids, for a message or a comparison.
fn owned<'a>(ids: impl Iterator<Item = &'a String>) -> Vec<String> {
    ids.cloned().collect()
}

fn list(items: &[String]) -> String {
    if items.is_empty() {
        "none".to_owned()
    } else {
        items.join(", ")
    }
}

/// A `--motor`: a `.eng` or `.rse` file by its extension, with the reader's warnings, or a
/// catalog name; a name the bundled catalog lacks is fetched from ThrustCurve.org, or with
/// `offline` taken from the cache alone ([`crate::motor_fetch`]).
fn motor(name: &str, offline: bool) -> Result<(Motor, SimMotorSource, Vec<InputWarning>), Failure> {
    let refused = |error: hpr::Error| Failure::Input(format!("{name}: {error}"));
    let Some(format) = MotorFile::of(name) else {
        return match Motor::from_catalog(name) {
            Ok(motor) => Ok((
                motor,
                SimMotorSource::Catalog {
                    as_of: crate::motors::catalog_as_of()?,
                },
                Vec::new(),
            )),
            Err(hpr::Error::NoSuchMotor(_)) => {
                let (motor, fetched) =
                    crate::motor_fetch::fetch(&Wanted::named(name), offline, "hpr sim")?;
                Ok((
                    motor,
                    SimMotorSource::ThrustCurve(Box::new(fetched)),
                    Vec::new(),
                ))
            }
            Err(error) => Err(refused(error)),
        };
    };
    let bytes = std::fs::read(name).map_err(|error| Failure::Input(format!("{name}: {error}")))?;
    let text = String::from_utf8(bytes)
        .map_err(|_| Failure::Input(format!("{name}: not a text file in UTF-8")))?;
    let (motor, format, warnings) = match format {
        MotorFile::Eng => (
            Motor::from_eng(&text).map_err(refused)?,
            FileFormat::Eng,
            eng::parse(&text).map(|parsed| parsed.warnings),
        ),
        MotorFile::Rse => (
            Motor::from_rse(&text).map_err(refused)?,
            FileFormat::Rse,
            rse::parse(&text).map(|parsed| parsed.warnings),
        ),
    };
    let file = file_name(name);
    let warnings = warnings
        .unwrap_or_default()
        .iter()
        .map(|warning| motor_warning(&file, warning))
        .collect();
    Ok((motor, SimMotorSource::File { file, format }, warnings))
}

fn motor_warning(file: &str, warning: &ParseWarning) -> InputWarning {
    InputWarning {
        at: format!("{file}, line {}", warning.line),
        kind: match warning.kind {
            ReadWarning::Skipped => WarningKind::Skipped,
            ReadWarning::Dropped => WarningKind::Dropped,
            _ => WarningKind::Unusual,
        },
        message: warning.message.clone(),
    }
}

pub(crate) fn ork_warning(warning: &ork::Warning) -> InputWarning {
    InputWarning {
        at: warning.at.clone(),
        kind: match warning.kind {
            ork::WarningKind::Skipped => WarningKind::Skipped,
            ork::WarningKind::Dropped => WarningKind::Dropped,
            _ => WarningKind::Unusual,
        },
        message: warning.message.clone(),
    }
}

/// An export's format, from its path's extension.
fn export_format(path: &str) -> Result<ExportFormat, Failure> {
    let extension = Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase);
    match extension.as_deref() {
        Some("csv") => Ok(ExportFormat::Csv),
        Some("json") => Ok(ExportFormat::Json),
        Some("parquet") => Ok(ExportFormat::Parquet),
        Some("geojson") => Ok(ExportFormat::Geojson),
        Some("kml") => Ok(ExportFormat::Kml),
        _ => Err(Failure::Input(format!(
            "{path}: --export writes .csv, .json, .parquet, .geojson or .kml"
        ))),
    }
}

/// A caveat for a peak the fall from apogee set before the first recovery device opened
/// (`first_opened_s`, `None` if none did): not a prediction.
pub(crate) fn in_fall(peak: &Peak, first_opened_s: Option<f64>) -> &'static str {
    if peak.after_apogee && first_opened_s.is_none_or(|opened_s| peak.time_s < opened_s) {
        ", in the fall: not a prediction"
    } else {
        ""
    }
}

/// A recording file's contents. Unless a recovery device opened (`braked`), the maps draw no
/// landing point: where the rocket came down is not a prediction, and a pin on a map reads as one.
/// Nor do they draw a separated part's, which is rough whatever opened (ADR-159).
/// What a JSON recording carries beside its rows, as a CSV's sidecar does ([`ExportMeta`]): the
/// design and configuration flown, the bundled catalog's as-of date and how far to trust it.
#[derive(serde::Serialize)]
struct About {
    design: String,
    configuration: String,
    catalog_as_of: String,
    kind: crate::trust::Kind,
    trust: String,
}

/// A recording's contents in `format`; `name` titles a KML file, and `about` is what a JSON one
/// carries beside its rows.
fn contents(
    format: ExportFormat,
    recorder: &Recorder,
    environment: &Environment,
    summary: &FlightSummary,
    (name, about): (&str, &About),
    braked: bool,
) -> Result<Vec<u8>, hpr_sim::SimError> {
    // A separated part's landing is never pinned: it flies as a point with only its devices'
    // drag, tumbling from the split, so where it lands is rough (ADR-159).
    let pinned = FlightSummary {
        body_landings: Vec::new(),
        ..summary.clone()
    };
    let unpinned = FlightSummary {
        landing: None,
        ..pinned.clone()
    };
    let summary = if braked { &pinned } else { &unpinned };
    Ok(match format {
        ExportFormat::Csv => export::csv(recorder)?.into_bytes(),
        ExportFormat::Json => export::json_with(recorder, about)?.into_bytes(),
        ExportFormat::Parquet => export::parquet(recorder)?,
        ExportFormat::Geojson => {
            export::geojson(&export::track(recorder, environment.sim())?, summary)?.into_bytes()
        }
        ExportFormat::Kml => {
            export::kml(&export::track(recorder, environment.sim())?, summary, name)?.into_bytes()
        }
    })
}

/// When a motor lights, in words.
fn ignition(ignition: &Ignition, names: &[(String, String)]) -> String {
    match ignition {
        Ignition::Launch => "at launch".to_owned(),
        Ignition::Time { time_s } => format!("{time_s} s after launch"),
        Ignition::Burnout { mount, delay_s } => {
            // The mount by its name, as the file writes it, or its id where it has none.
            let mount = names
                .iter()
                .find(|(id, name)| id == mount && !name.trim().is_empty())
                .map_or(mount, |(_, name)| name);
            format!("{delay_s} s after the motor in `{mount}` burns out")
        }
        Ignition::Separation { delay_s } => format!("{delay_s} s after its stage separates"),
        Ignition::Never => "never".to_owned(),
        other => format!("{other:?}"),
    }
}

/// The library's summary, field for field, each peak marked if it came after apogee.
fn summary(summary: &FlightSummary) -> Summary {
    let apogee_s = summary.apogee.map(|apogee| apogee.time_s);
    let mark = |p: metrics::Peak| peak(p, apogee_s);
    Summary {
        termination: match summary.termination {
            hpr_sim::Termination::GroundHit => Termination::GroundHit,
            hpr_sim::Termination::NoLiftoff => Termination::NoLiftoff,
            hpr_sim::Termination::StalledOnRail => Termination::StalledOnRail,
            hpr_sim::Termination::TimeCap => Termination::TimeCap,
            hpr_sim::Termination::StepLimit => Termination::StepLimit,
            hpr_sim::Termination::Separated => Termination::Separated,
            _ => Termination::Other,
        },
        launch_height_m: summary.launch_height_m,
        rail_exit_speed_m_s: summary.rail_exit_speed_m_s.map(mark),
        apogee: summary.apogee.map(|apogee| Apogee {
            time_s: apogee.time_s,
            height_above_ground_m: apogee.height_above_ground_m,
            gain_m: apogee.gain_m,
        }),
        max_speed_m_s: summary.max_speed_m_s.map(mark),
        max_mach: summary.max_mach.map(mark),
        max_dynamic_pressure_pa: summary.max_dynamic_pressure_pa.map(mark),
        max_acceleration_m_s2: summary.max_acceleration_m_s2.map(mark),
        max_descent_acceleration_m_s2: summary.max_descent_acceleration_m_s2.map(mark),
        min_static_margin_cal: summary.min_static_margin_cal.map(mark),
        min_flight_margin_cal: summary.min_flight_margin_cal.map(mark),
        min_powered_static_margin_cal: summary.min_powered_static_margin_cal.map(mark),
        max_powered_moment_slope_per_rad: summary.max_powered_moment_slope_per_rad.map(mark),
        rail_exit_stability: summary.rail_exit_stability.map(|stability| Stability {
            time_s: stability.time_s,
            height_above_ground_m: stability.height_above_ground_m,
            dynamic_pressure_pa: stability.dynamic_pressure_pa,
            cg_station_m: stability.cg_station_m,
            reference_diameter_m: stability.reference_diameter_m,
            static_margin: margin(&stability.static_margin),
            flight_margin: margin(&stability.flight_margin),
        }),
        max_angle_of_attack_rad: summary.max_angle_of_attack_rad.map(mark),
        landing: summary.landing.as_ref().map(landing),
        body_landings: summary.body_landings.iter().map(landing).collect(),
    }
}

/// The flight's envelope flags, each peak marked as [`summary`] marks them.
pub(crate) fn flags(summary: &FlightSummary) -> Vec<SimFlag> {
    let apogee_s = summary.apogee.map(|apogee| apogee.time_s);
    summary
        .envelope_flags()
        .iter()
        .map(|flag| SimFlag {
            flag: flag_kind(flag),
            peak: peak(flag.peak(), apogee_s),
            message: flag.message(),
        })
        .collect()
}

/// The known drag issues the flight meets, each peak marked as [`summary`] marks them.
pub(crate) fn issues(flight: &Flight) -> Vec<SimIssue> {
    let apogee_s = flight.summary().apogee.map(|apogee| apogee.time_s);
    flight
        .issue_warnings()
        .iter()
        .map(|warning| SimIssue {
            issue: warning.number(),
            kind: IssueKind::from(warning.issue.kind()),
            url: warning.issue.url(),
            max_mach: peak(warning.max_mach, apogee_s),
            parts: warning.parts.clone(),
            message: warning.message(),
        })
        .collect()
}

/// The output's name for a flag.
pub(crate) fn flag_kind(flag: &EnvelopeFlag) -> FlagKind {
    match flag {
        EnvelopeFlag::UnstableUnderPower { .. } => FlagKind::UnstableUnderPower,
        EnvelopeFlag::UnstableWithoutMargin { .. } => FlagKind::UnstableWithoutMargin,
        EnvelopeFlag::BeyondValidatedRange { .. } => FlagKind::BeyondValidatedRange,
        EnvelopeFlag::HighAngleOfAttack { .. } => FlagKind::HighAngleOfAttack,
        EnvelopeFlag::OutsideCoreBand { .. } => FlagKind::OutsideCoreBand,
        EnvelopeFlag::BeyondEnvelope { .. } => FlagKind::BeyondEnvelope,
    }
}

fn peak(peak: metrics::Peak, apogee_s: Option<f64>) -> Peak {
    Peak {
        value: peak.value,
        time_s: peak.time_s,
        height_above_ground_m: peak.height_above_ground_m,
        after_apogee: apogee_s.is_some_and(|apogee| peak.time_s > apogee),
    }
}

fn margin(margin: &metrics::Margin) -> Margin {
    Margin {
        mach: margin.mach,
        angle_of_attack_rad: margin.angle_of_attack_rad,
        roll_rad: margin.roll_rad,
        normal_force_slope_per_rad: margin.normal_force_slope_per_rad,
        slope_magnitude_sum_per_rad: margin.slope_magnitude_sum_per_rad,
        pitch_moment_slope_per_rad: margin.pitch_moment_slope_per_rad,
        cp_station_m: margin.cp_station_m,
        margin_cal: margin.margin_cal,
    }
}

fn landing(landing: &metrics::Landing) -> Landing {
    Landing {
        body: landing.body,
        time_s: landing.time_s,
        latitude_deg: landing.latitude_deg,
        longitude_deg: landing.longitude_deg,
        east_m: landing.east_m,
        north_m: landing.north_m,
        distance_m: landing.distance_m,
        ground_hit_speed_m_s: landing.ground_hit_speed_m_s,
        descent_rate_m_s: landing.descent_rate_m_s,
    }
}

/// The flight's events, in order: the stack's, then, when the stack came apart with nothing left
/// to burn ([`hpr_sim::Termination::Separated`]), those of the part that keeps the nose, part 0,
/// which flies on as a point: its devices, its apogee and its landing.
fn events(result: &hpr_sim::FlightResult) -> Vec<SimEvent> {
    let mut events: Vec<SimEvent> = result.events.iter().map(event).collect();
    if result.termination == hpr_sim::Termination::Separated {
        let nose = result.bodies.iter().filter(|body| body.body == 0);
        events.extend(nose.flat_map(|body| &body.events).map(|event| {
            let velocity = event.sample.cg_velocity_enu_m_s;
            sim_event(
                event.kind,
                [
                    event.sample.time_s,
                    event.sample.height_above_ground_m,
                    velocity.length(),
                    velocity.z,
                ],
            )
        }));
    }
    events
}

fn event(event: &hpr_sim::FlightEvent) -> SimEvent {
    let velocity = event.sample.cg_velocity_enu_m_s;
    sim_event(
        event.kind,
        [
            event.sample.time_s,
            event.sample.height_above_ground_m,
            velocity.length(),
            velocity.z,
        ],
    )
}

/// An event of `kind` at a time, s, a height above the site, m, a speed over the ground, m/s,
/// and a vertical velocity, m/s, up.
fn sim_event(
    kind: hpr_sim::EventKind,
    [
        time_s,
        height_above_ground_m,
        speed_m_s,
        vertical_velocity_m_s,
    ]: [f64; 4],
) -> SimEvent {
    use hpr_sim::EventKind as Kind;
    let (kind, index) = match kind {
        Kind::Liftoff => (EventKind::Liftoff, None),
        Kind::RailExit => (EventKind::RailExit, None),
        Kind::Burnout => (EventKind::Burnout, None),
        Kind::Apogee => (EventKind::Apogee, None),
        Kind::GroundHit => (EventKind::GroundHit, None),
        Kind::Separation => (EventKind::Separation, None),
        Kind::Trigger(i) => (EventKind::Trigger, Some(i)),
        Kind::Deployment(i) => (EventKind::Deployment, Some(i)),
        Kind::Release(i) => (EventKind::Release, Some(i)),
        Kind::Ejection(i) => (EventKind::Ejection, Some(i)),
        Kind::Shift(i) => (EventKind::Shift, Some(i)),
        Kind::MassRelease(i) => (EventKind::MassRelease, Some(i)),
        Kind::User(i) => (EventKind::User, Some(i)),
        Kind::Ignition(i) => (EventKind::Ignition, Some(i)),
        _ => (EventKind::Other, None),
    };
    SimEvent {
        kind,
        index,
        time_s,
        height_above_ground_m,
        speed_m_s,
        vertical_velocity_m_s,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each of the library's issue kinds prints under its own name, the library's: none falls
    /// into another (M10.1d6).
    #[test]
    fn each_issue_kind_prints_as_the_librarys_name() {
        for kind in hpr_sim::issues::IssueKind::ALL {
            assert_eq!(
                serde_json::to_value(IssueKind::from(kind)).unwrap(),
                serde_json::Value::from(kind.name())
            );
        }
    }

    /// The boundary a refusal names is the first that breaks the order, not the first of all:
    /// in four stages, boundaries after stages 2, 1 and 1 name the third; after 3, the last
    /// stage, the first; in order, the first, as when the design has no stage at all.
    #[test]
    fn the_misplaced_boundary_is_the_first_out_of_order() {
        let at = |stages: &[usize]| -> Vec<hpr::Separation> {
            stages
                .iter()
                .map(|&after| hpr::Separation::new(hpr::Trigger::Apogee, after))
                .collect()
        };
        assert_eq!(misplaced(&at(&[2, 2]), 4), 2);
        assert_eq!(misplaced(&at(&[2, 1, 1]), 4), 1);
        assert_eq!(misplaced(&at(&[3, 1]), 4), 3);
        assert_eq!(misplaced(&at(&[2, 0]), 4), 2);
        assert_eq!(misplaced(&at(&[1]), 0), 1);
        assert_eq!(misplaced(&[], 4), 0);
    }

    /// A design error names the part at fault, at any depth, and leaves an id it doesn't know.
    #[test]
    fn a_design_error_names_its_part() {
        let name = |id: &str| match id {
            "a1" => "`Booster`".to_owned(),
            "b2" => "`Fin set`".to_owned(),
            other => other.to_owned(),
        };
        let error = hpr_design::DesignError::InComponent {
            id: "a1".to_owned(),
            source: Box::new(hpr_design::DesignError::Tree {
                id: "b2".to_owned(),
                message: "fins attached to an inner tube".to_owned(),
            }),
        };
        assert_eq!(
            design_error(&error, &name),
            "`Booster`: `Fin set`: fins attached to an inner tube"
        );
        let unknown = hpr_design::DesignError::Tree {
            id: "c3".to_owned(),
            message: "no radius".to_owned(),
        };
        assert_eq!(design_error(&unknown, &name), "c3: no radius");
    }

    /// Each flag has the library's name in the JSON: a flight at Mach 4 and 0.5 rad, with a
    /// margin of −1 calibre under power, raises five; with no margin and a moment slope of +2 per
    /// radian under power instead, the sixth.
    #[test]
    fn each_flag_keeps_the_librarys_name() {
        let at = |value: f64| {
            Some(metrics::Peak {
                value,
                time_s: 2.0,
                height_above_ground_m: 300.0,
            })
        };
        let mut flags = hpr_sim::envelope::flags(at(4.0), at(0.5), at(-1.0), None);
        assert_eq!(flags.len(), 5);
        flags.extend(hpr_sim::envelope::flags(None, None, None, at(2.0)));
        assert_eq!(flags.len(), 6);
        for flag in &flags {
            assert_eq!(
                serde_json::to_value(flag_kind(flag)).unwrap(),
                flag.name(),
                "{flag:?}"
            );
        }
    }
}
