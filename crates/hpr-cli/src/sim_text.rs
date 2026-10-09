//! `hpr sim`'s text form: the figures a flyer checks first, then what flew, from where, what the
//! flight leaves out, its events and its other figures.
//!
//! The summary leads with the static margin, the apogee, the rail exit speed, the ejection delay
//! and the descent rate (ADR-144 §8d), each in a sentence. Configurations and parts are called by
//! their names ([`crate::choose`]), never by an OpenRocket UUID; `--json` prints the same flight
//! as data, ids and all.
//!
//! Heights, distances and speeds are SI first with US units in brackets, `1065.1 m (3494 ft)`
//! ([`crate::units`]; ADR-164 §6, ADR-210): feet, feet per second, miles per hour for the wind and
//! inches for the stations along the rocket.
//!
//! The flight goes to standard output; its `warning:`, `note:` and `help:` lines go to standard
//! error ([`Diagnostics`]), so a file the text is redirected to holds the flight alone.

use std::io::{self, Write};

use crate::console::{Diagnostics, Level};
use crate::output::{
    Delay, DeviceEvent, EventKind, FlagKind, InputWarning, IssueKind, Launch, SimDesign, SimDevice,
    SimEvent, SimFlight, SimMotor, SimMotorSource, Termination,
};
use crate::sim::{braked, first_opened, in_fall};
use crate::units::{feet, feet_per_second, fixed, inches, meters, miles_per_hour, speed};

/// The width of a figure's name in the summary's columns.
const NAME: usize = 22;

/// The text form of `flight`: the flight on `out`, its warnings, notes and hints on
/// `diagnostics`.
pub(crate) fn print(
    flight: &SimFlight,
    out: &mut dyn Write,
    diagnostics: &mut Diagnostics<'_>,
) -> io::Result<()> {
    design_lines(&flight.design, out, diagnostics)?;
    writeln!(out, "a simulated flight, not a measurement")?;
    writeln!(out)?;
    lead(flight, out)?;
    writeln!(out)?;
    motor_lines(&flight.motors, out)?;
    for device in &flight.recovery {
        let event = match device.opens_at {
            DeviceEvent::Apogee => "apogee".to_owned(),
            DeviceEvent::Altitude => {
                let height_m = device.height_above_ground_m.unwrap_or(f64::NAN);
                format!("{height_m} m ({}) on the way down", feet(height_m))
            }
            DeviceEvent::Ejection => "the ejection charge".to_owned(),
            DeviceEvent::Launch => "launch".to_owned(),
            DeviceEvent::Separation => "the separation".to_owned(),
        };
        let delay = if device.delay_s > 0.0 {
            format!(" + {} s", device.delay_s)
        } else {
            String::new()
        };
        let opened = device.opened_s.map_or_else(
            || "never opened".to_owned(),
            |time_s| format!("opened at {time_s:.2} s"),
        );
        let part = device
            .body
            .map_or_else(String::new, |body| format!(" on part {body}"));
        writeln!(
            out,
            "recovery: `{}`{part} at {event}{delay}, {:.3} m² of drag area, {opened}",
            device.name, device.drag_area_m2
        )?;
    }
    launch_lines(&flight.launch, out)?;
    note_lines(&flight.notes, &flight.warnings, diagnostics)?;
    for flag in &flight.flags {
        let kind = flag_prefix(flag.flag);
        diagnostics.line(Level::Warning, &format!("{kind}: {}", flag.message))?;
    }
    let kinds: Vec<FlagKind> = flight.flags.iter().map(|flag| flag.flag).collect();
    flag_help(&kinds, diagnostics)?;
    for issue in &flight.issues {
        let kind = match issue.kind {
            IssueKind::Drag => "drag",
            IssueKind::Stability => "stability",
            IssueKind::Flight => "flight",
        };
        diagnostics.line(Level::Warning, &format!("{kind}: {}", issue.message))?;
    }
    writeln!(out)?;
    events(&flight.events, out)?;
    writeln!(out)?;
    rest(flight, out)?;
    trust_lines(&flight.trust, out)
}

/// The width of the trust note's lines.
const NOTE: usize = 100;

/// `note`, how far to trust the result, after a blank line, as the result's last lines: the
/// command-line guide puts what to trust last. Its link to read more gets a line of its own.
pub(crate) fn trust_lines(note: &str, out: &mut dyn Write) -> io::Result<()> {
    writeln!(out)?;
    let (body, more) = note
        .rsplit_once(crate::trust::MORE)
        .map_or((note, None), |(body, url)| (body, Some(url)));
    for line in crate::trust::wrap(body, NOTE) {
        writeln!(out, "{line}")?;
    }
    if let Some(url) = more {
        writeln!(out, "{}{url}", crate::trust::MORE.trim_start())?;
    }
    Ok(())
}

/// The design and the configuration flown, and a hint naming the others on `diagnostics`.
pub(crate) fn design_lines(
    design: &SimDesign,
    out: &mut dyn Write,
    diagnostics: &mut Diagnostics<'_>,
) -> io::Result<()> {
    writeln!(out, "{} ({})", design.name, design.file)?;
    let flown = design
        .configurations
        .iter()
        .position(|c| c.id == design.configuration);
    match flown {
        Some(index) => writeln!(
            out,
            "configuration {} of {}: {}",
            index + 1,
            design.configurations.len(),
            design.configuration_label
        )?,
        None => writeln!(out, "configuration {}", design.configuration_label)?,
    }
    let others: Vec<String> = design
        .configurations
        .iter()
        .enumerate()
        .filter(|(index, _)| Some(*index) != flown)
        .map(|(index, c)| {
            let unflown = c
                .not_flown
                .as_ref()
                .map_or_else(String::new, |why| format!(" ({why})"));
            format!("{} {}{unflown}", index + 1, c.label)
        })
        .collect();
    if !others.is_empty() {
        diagnostics.line(
            Level::Help,
            &format!(
                "--config flies the others, by number or name: {}",
                others.join(", ")
            ),
        )?;
    }
    Ok(())
}

/// A line for each motor flown.
pub(crate) fn motor_lines(motors: &[SimMotor], out: &mut dyn Write) -> io::Result<()> {
    for motor in motors {
        let source = match &motor.source {
            SimMotorSource::Design => "the design's".to_owned(),
            SimMotorSource::Catalog { as_of } => {
                format!(
                    "from the bundled catalog, as of {}",
                    crate::printable(as_of)
                )
            }
            SimMotorSource::File { file, .. } => format!("from {file}"),
            SimMotorSource::ThrustCurve(fetched) => format!(
                "ThrustCurve.org's {}, file {}",
                crate::printable(&fetched.designation),
                crate::motor_fetch::file_words(fetched)
            ),
        };
        let unlit = if motor.unlit > 0 {
            format!(", {} never lit", motor.unlit)
        } else {
            String::new()
        };
        let mount = if motor.mount_name.trim().is_empty() {
            &motor.mount
        } else {
            &motor.mount_name
        };
        writeln!(
            out,
            "motor: {} × {} ({source}) in `{mount}`, lit {}{unlit}",
            motor.count, motor.designation, motor.ignition
        )?;
    }
    Ok(())
}

/// Where and how the rocket is launched.
pub(crate) fn launch_lines(launch: &Launch, out: &mut dyn Write) -> io::Result<()> {
    // A rail is sold and set up by its length in feet (6 ft, 8 ft, 12 ft), so to a tenth.
    let rail_length = format!(
        "{} m ({} ft)",
        launch.rail_length_m,
        fixed(launch.rail_length_m / crate::units::FOOT_M, 1)
    );
    let rail = if launch.inclination_deg == 90.0 {
        format!("a {rail_length} vertical rail")
    } else {
        format!(
            "a {rail_length} rail {}° above the horizon, leaning toward {}°",
            launch.inclination_deg, launch.heading_deg
        )
    };
    let wind = if launch.wind_speed_m_s == 0.0 {
        "calm air".to_owned()
    } else {
        format!(
            "a {} m/s ({}) wind from {}°",
            launch.wind_speed_m_s,
            miles_per_hour(launch.wind_speed_m_s),
            launch.wind_from_deg
        )
    };
    writeln!(
        out,
        "launched at {}, {}, {} m ({}) above sea level, from {rail}, in {wind}",
        hemisphere(launch.latitude_deg, "N", "S"),
        hemisphere(launch.longitude_deg, "E", "W"),
        launch.elevation_m,
        feet(launch.elevation_m),
    )?;
    Ok(())
}

/// The notes, then the warnings, the same warning about several parts once.
pub(crate) fn note_lines(
    notes: &[String],
    given: &[InputWarning],
    diagnostics: &mut Diagnostics<'_>,
) -> io::Result<()> {
    for note in notes {
        diagnostics.line(Level::Note, note)?;
    }
    // The same words about several parts of one name, such as a coupler in each tube, once.
    let mut warnings: Vec<(&str, &str, usize)> = Vec::new();
    for warning in given {
        match warnings
            .iter_mut()
            .find(|(at, message, _)| *at == warning.at && *message == warning.message)
        {
            Some((_, _, times)) => *times += 1,
            None => warnings.push((&warning.at, &warning.message, 1)),
        }
    }
    for (at, message, times) in warnings {
        let times = if times > 1 {
            format!(" (×{times})")
        } else {
            String::new()
        };
        diagnostics.line(Level::Warning, &format!("{at}: {message}{times}"))?;
    }
    Ok(())
}

/// Whether a flag says the rocket is unstable under power, by its static margin or, where it
/// has none, by its pitching moment.
fn is_unstable(flag: FlagKind) -> bool {
    matches!(
        flag,
        FlagKind::UnstableUnderPower | FlagKind::UnstableWithoutMargin
    )
}

/// The word a flag's warning starts with: `unstable` for a rocket unstable under power,
/// `envelope` for the operating envelope's flags.
pub(crate) fn flag_prefix(flag: FlagKind) -> &'static str {
    match flag {
        FlagKind::UnstableUnderPower | FlagKind::UnstableWithoutMargin => "unstable",
        FlagKind::BeyondValidatedRange
        | FlagKind::HighAngleOfAttack
        | FlagKind::OutsideCoreBand
        | FlagKind::BeyondEnvelope => "envelope",
    }
}

/// The `help:` lines after the flags `kinds`: the page on stability under power, and the
/// operating envelope's, each once, when a flag of its kind is raised.
pub(crate) fn flag_help(kinds: &[FlagKind], diagnostics: &mut Diagnostics<'_>) -> io::Result<()> {
    if kinds.iter().any(|kind| is_unstable(*kind)) {
        diagnostics.line(
            Level::Help,
            "unstable under power: \
             https://hpr.fusionspace.co/physics/metrics.html#unstable-under-power",
        )?;
    }
    if kinds.iter().any(|kind| !is_unstable(*kind)) {
        diagnostics.line(
            Level::Help,
            "the operating envelope: \
             https://hpr.fusionspace.co/VALIDATION.html#operating-envelope",
        )?;
    }
    Ok(())
}

/// The figures a flyer checks first: margin, apogee, rail exit speed, delay and descent.
fn lead(flight: &SimFlight, out: &mut dyn Write) -> io::Result<()> {
    let summary = &flight.summary;
    let rail_margin = summary
        .rail_exit_stability
        .and_then(|stability| stability.static_margin.margin_cal);
    let least = summary.min_static_margin_cal.as_ref().map(|margin| {
        format!(
            "least {:.2} calibres, at {:.2} s, before apogee",
            margin.value, margin.time_s
        )
    });
    match (rail_margin, least) {
        (Some(rail), Some(least)) => figure(
            out,
            "static margin",
            &format!("{rail:.2} calibres off the rail; {least}"),
        )?,
        (Some(rail), None) => figure(
            out,
            "static margin",
            &format!("{rail:.2} calibres off the rail"),
        )?,
        (None, Some(least)) => figure(out, "static margin", &least)?,
        (None, None) => {}
    }
    // The stations the margin off the rail came from, measured aft of the nose tip.
    if let Some(stability) = summary
        .rail_exit_stability
        .filter(|stability| stability.static_margin.margin_cal.is_some())
        && let Some(cp_m) = stability.static_margin.cp_station_m
    {
        figure(
            out,
            "CG and CP",
            &format!(
                "{} and {} aft of the nose tip, off the rail",
                station(stability.cg_station_m),
                station(cp_m)
            ),
        )?;
    }
    if let Some(apogee) = &summary.apogee {
        figure(
            out,
            "apogee",
            &format!(
                "{} above the site at {:.2} s{}",
                meters(apogee.height_above_ground_m),
                apogee.time_s,
                if flight.flags.iter().any(|flag| is_unstable(flag.flag)) {
                    ", not a prediction: unstable under power"
                } else {
                    ""
                }
            ),
        )?;
    }
    if let Some(rail_exit) = &summary.rail_exit_speed_m_s {
        figure(out, "rail exit speed", &speed(rail_exit.value))?;
    }
    if let Some(delay) = delay(flight) {
        figure(out, "delay", &delay)?;
    }
    let braked = braked(
        &flight.recovery,
        summary.apogee.as_ref().map(|apogee| apogee.time_s),
    );
    figure(out, "descent", &descent(flight, braked))
}

/// A station along the rocket, m aft of the nose tip to the millimeter, with inches:
/// `0.912 m (35.9 in)`.
fn station(station_m: f64) -> String {
    format!("{} m ({})", fixed(station_m, 3), inches(station_m, 1))
}

/// One figure of the summary, its name in a column.
fn figure(out: &mut dyn Write, name: &str, value: &str) -> io::Result<()> {
    writeln!(out, "{name:<NAME$}{value}")
}

/// The ejection delay: how long the rocket coasts from its last burnout to apogee, beside the
/// delays the design sets and, for one motor lit at launch, when its charge fires against apogee.
/// With a motor still burning at apogee there is no coast to tell, nor when the stack comes apart
/// before its apogee with nothing left to burn: the apogee is then the part's that keeps the nose,
/// under its own devices from the split. `None` with neither a coast nor a delay to tell.
fn delay(flight: &SimFlight) -> Option<String> {
    let apogee_s = flight.summary.apogee.as_ref().map(|apogee| apogee.time_s);
    // A stack that flies on after its split, a sustainer's, has its own apogee and coast.
    let split_s = flight
        .events
        .iter()
        .find(|e| e.kind == EventKind::Separation)
        .map(|e| e.time_s)
        .filter(|_| flight.summary.termination == Termination::Separated)
        .filter(|split_s| apogee_s.is_some_and(|apogee_s| *split_s < apogee_s));
    let apogee_s = apogee_s.filter(|_| split_s.is_none());
    let burnouts = || {
        flight
            .events
            .iter()
            .filter(|e| e.kind == EventKind::Burnout)
    };
    let burnout_s = apogee_s
        .filter(|apogee_s| burnouts().all(|e| e.time_s <= *apogee_s))
        .and_then(|_| burnouts().map(|e| e.time_s).reduce(f64::max));
    let coast = burnout_s
        .zip(apogee_s)
        .map(|(burnout_s, apogee_s)| format!("apogee {:.2} s after burnout", apogee_s - burnout_s));
    let mut set: Vec<String> = Vec::new();
    for (designation, delay) in flight
        .motors
        .iter()
        .filter_map(|motor| Some((motor.designation.as_str(), motor.delay?)))
    {
        let delay = match delay {
            Delay::Seconds(s) => format!("{s} s"),
            Delay::Plugged => "plugged".to_owned(),
            Delay::ZeroOrPlugged => "0 (at burnout, or plugged)".to_owned(),
        };
        let words = if flight.motors.len() == 1 {
            format!("the motor's set delay: {delay}")
        } else {
            format!("{designation}'s set delay: {delay}")
        };
        // The same motor in two mounts, once.
        if !set.contains(&words) {
            set.push(words);
        }
    }
    let set = (!set.is_empty()).then(|| set.join(", "));
    // One motor, lit at launch: the coast ends where its burn does.
    let fires = match (&flight.motors[..], burnout_s, apogee_s) {
        ([motor], Some(burnout_s), Some(apogee_s)) if motor.ignition == "at launch" => {
            match motor.delay {
                Some(Delay::Seconds(s)) if s > 0.0 => {
                    let early_s = apogee_s - (burnout_s + s);
                    Some(if early_s.abs() < 0.005 {
                        "its charge fires at apogee".to_owned()
                    } else if early_s > 0.0 {
                        format!("its charge fires {early_s:.2} s before apogee")
                    } else {
                        format!("its charge fires {:.2} s after apogee", -early_s)
                    })
                }
                _ => None,
            }
        }
        _ => None,
    };
    let apart = split_s.map(|split_s| {
        let last_s = burnouts().map(|e| e.time_s).reduce(f64::max);
        match last_s.filter(|last_s| *last_s <= split_s) {
            Some(last_s) => format!(
                "the stack comes apart {:.2} s after burnout, before apogee",
                split_s - last_s
            ),
            None => format!("the stack comes apart at {split_s:.2} s, before apogee"),
        }
    });
    let parts: Vec<String> = [coast, apart, set, fires].into_iter().flatten().collect();
    (!parts.is_empty()).then(|| parts.join("; "))
}

/// How fast the rocket comes down under each set of recovery devices open, as the next opens or
/// as it lands: the vertical speed, not the speed over the ground a wind adds to. Unless
/// `braked`, a device opened soon after apogee, the descent carries the landing's caveat.
fn descent(flight: &SimFlight, braked: bool) -> String {
    // The rocket's, or its sustainer's: a separated part's devices are its own descent's.
    let mut opened: Vec<(&SimDevice, f64)> = flight
        .recovery
        .iter()
        .filter(|device| device.body.is_none())
        .filter_map(|device| Some((device, device.opened_s?)))
        .collect();
    opened.sort_by(|a, b| a.1.total_cmp(&b.1));
    let ground = flight
        .events
        .iter()
        .find(|e| e.kind == EventKind::GroundHit);
    let end = |event: Option<&SimEvent>| {
        event.map_or_else(
            || "the flight ended before landing".to_owned(),
            |e| format!("{} at landing", speed(-e.vertical_velocity_m_s)),
        )
    };
    if opened.is_empty() {
        return "no recovery device opened, so the fall is not a prediction".to_owned();
    }
    // Devices that open together descend together.
    let mut sets: Vec<(Vec<&str>, f64)> = Vec::new();
    for (device, opened_s) in opened {
        match sets.last_mut() {
            Some((names, at_s)) if *at_s == opened_s => names.push(&device.name),
            _ => sets.push((vec![&device.name], opened_s)),
        }
    }
    let quoted = |names: &[&str]| {
        names
            .iter()
            .map(|name| format!("`{name}`"))
            .collect::<Vec<_>>()
            .join(" and ")
    };
    let mut parts = Vec::new();
    for (index, (names, _)) in sets.iter().enumerate() {
        let until = match sets.get(index + 1) {
            Some((_, next_s)) => flight
                .events
                .iter()
                .find(|e| e.kind == EventKind::Deployment && e.time_s == *next_s)
                .map(|e| {
                    let height_m = e.height_above_ground_m.max(0.0) + 0.0;
                    // A set can open on the way up, before apogee.
                    if e.vertical_velocity_m_s < 0.0 {
                        format!(
                            "{} at {}",
                            speed(-e.vertical_velocity_m_s),
                            meters(height_m)
                        )
                    } else {
                        format!("still rising at {}", meters(height_m))
                    }
                }),
            None => Some(end(ground)),
        };
        let Some(until) = until else { continue };
        parts.push(if index == 0 {
            format!("{until} under {}", quoted(names))
        } else {
            format!("{until} with {} open too", quoted(names))
        });
    }
    let caveat = if braked {
        ""
    } else {
        ": with no recovery device opened soon after apogee, not a prediction"
    };
    format!("{}{caveat}", parts.join("; "))
}

/// The events, as a table: each height and speed in SI, then in US units in brackets.
fn events(events: &[SimEvent], out: &mut dyn Write) -> io::Result<()> {
    writeln!(
        out,
        "{:<18} {:>9} {:>22} {:>24}",
        "event", "time", "height", "speed"
    )?;
    for event in events {
        let name = match event.kind {
            EventKind::Liftoff => "liftoff",
            EventKind::RailExit => "rail exit",
            EventKind::Burnout => "burnout",
            EventKind::Apogee => "apogee",
            EventKind::GroundHit => "ground hit",
            EventKind::Trigger => "charge",
            EventKind::Deployment => "deployment",
            EventKind::Release => "release",
            EventKind::Separation => "separation",
            EventKind::Ejection => "ejection",
            EventKind::Shift => "mass shift",
            EventKind::MassRelease => "mass release",
            EventKind::User => "user event",
            EventKind::Ignition => "ignition",
            EventKind::Other => "other",
        };
        // The flight ends a hair below the ground: its height is drawn as zero.
        let height_m = event.height_above_ground_m.max(0.0);
        writeln!(
            out,
            // The feet fit `(100000 ft)` with a space before it, the height of a flight past 30 km.
            "{name:<18} {:>7.2} s {:>8} m {:>11} {:>7} m/s {:>12}",
            event.time_s,
            fixed(height_m, 1),
            format!("({})", feet(height_m)),
            fixed(event.speed_m_s, 1),
            format!("({})", feet_per_second(event.speed_m_s)),
        )?;
    }
    writeln!(
        out,
        "(heights are the center of gravity's above the site; speeds are over the ground)"
    )
}

/// The figures after the events: top speed and Mach number, the landing, how the flight ended
/// and the files written.
fn rest(flight: &SimFlight, out: &mut dyn Write) -> io::Result<()> {
    let first_opened_s = first_opened(&flight.recovery).map(|(_, opened_s)| opened_s);
    let summary = &flight.summary;
    let braked = braked(
        &flight.recovery,
        summary.apogee.as_ref().map(|apogee| apogee.time_s),
    );
    if let Some(top) = &summary.max_speed_m_s {
        figure(
            out,
            "top speed",
            &format!(
                "{} at {:.2} s{}",
                crate::units::speed(top.value),
                top.time_s,
                in_fall(top, first_opened_s)
            ),
        )?;
    }
    if let Some(mach) = &summary.max_mach {
        figure(
            out,
            "top Mach number",
            &format!("{:.3}{}", mach.value, in_fall(mach, first_opened_s)),
        )?;
    }
    if let Some(landing) = &summary.landing {
        figure(
            out,
            "landing",
            &format!(
                "{} from the pad at {:.2} s, at {}{}",
                meters(landing.distance_m),
                landing.time_s,
                speed(landing.ground_hit_speed_m_s),
                if braked {
                    ""
                } else {
                    ": with no recovery device opened soon after apogee, not a prediction"
                }
            ),
        )?;
    }
    for landing in &summary.body_landings {
        let part = landing
            .body
            .map_or_else(|| "rocket".to_owned(), |body| body.to_string());
        figure(
            out,
            &format!("landing, part {part}"),
            &format!(
                "{} from the pad at {:.2} s, at {}: rough, as it flew as a point with only its \
                 devices' drag",
                meters(landing.distance_m),
                landing.time_s,
                speed(landing.ground_hit_speed_m_s),
            ),
        )?;
    }
    let ended = match summary.termination {
        Termination::GroundHit => None,
        Termination::NoLiftoff => Some("the rocket never left the rail's foot"),
        Termination::StalledOnRail => Some("the rocket stalled on the rail"),
        Termination::TimeCap => Some("the flight reached its time limit before landing"),
        Termination::StepLimit => Some("the integrator reached its step limit before landing"),
        // The part that keeps the nose landing is the flight's landing, printed above.
        Termination::Separated if summary.body_landings.iter().any(|l| l.body == Some(0)) => None,
        Termination::Separated => Some("the stack separated"),
        Termination::Other => Some("the flight ended in a way this build doesn't name"),
    };
    if let Some(ended) = ended {
        writeln!(out, "the flight ended: {ended}")?;
    }
    for export in &flight.exports {
        writeln!(out, "wrote {} ({} rows)", export.path, export.rows)?;
        if let Some(meta) = &export.meta {
            writeln!(out, "wrote {meta} (the program that wrote {})", export.path)?;
        }
    }
    if let Some(plot) = &flight.plot {
        writeln!(
            out,
            "wrote {plot} (altitude, speed and acceleration against time)"
        )?;
    }
    Ok(())
}

/// An angle north or south, east or west: `32.99° N`, `106.97° W`.
fn hemisphere(value: f64, positive: &str, negative: &str) -> String {
    if value < 0.0 {
        format!("{}° {negative}", -value)
    } else {
        format!("{value}° {positive}")
    }
}
