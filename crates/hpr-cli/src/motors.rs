//! `hpr motors`: the bundled catalog, motor files, and motors fetched from ThrustCurve.org.
//!
//! `list` prints the catalog's motors with the figures their public-domain curve files give: size,
//! masses and delays from each file's header, the rest worked out from its curve. `show` works a
//! motor's figures out from its thrust curve with `hpr_motor`, the curve the simulator flies,
//! and reads its designation as `data.md` (*Units for rocketry*) asks: the impulse class with its
//! range, the nominal average thrust beside the measured one where they differ, and the
//! propellant the maker's letters stand for where the catalog names it. Masses and sizes give
//! ounces or pounds and inches in brackets after the SI ([`crate::units`]).

use std::io::{self, Write};
use std::path::Path;

use hpr::hpr_motor::catalog::{Catalog, CatalogMotor, CurveFormat, MotorType, bundled_curve_text};
use hpr::hpr_motor::curve::ThrustCurve;
use hpr::hpr_motor::delay::{self, DelayList};
use hpr::hpr_motor::text::{ParseWarning, WarningKind as ReadWarning};
use hpr::hpr_motor::{ImpulseClass, eng, rse};

use crate::console::{Diagnostics, Level, Text};
use crate::output::{
    CatalogInfo, Delay, FileFormat, ListedMotor, MotorFigures, MotorKind, MotorList, MotorShow,
    MotorSource, Warning, WarningKind,
};
use crate::units::{bracketed, inches, inches_figure, ounces_or_pounds};
use crate::{Failure, Out};

/// `hpr motors`'s subcommands.
#[derive(Debug, clap::Subcommand)]
pub enum MotorsCommand {
    /// List the bundled catalog's motors, with the figures their curve files give
    List(ListArgs),
    /// Work out a motor's figures from its thrust curve: a catalog name, or a .eng or .rse file
    Show(ShowArgs),
    /// Search motor.fusionspace.co's motors, with who has them in stock and at what price
    Search(crate::motor_search::SearchArgs),
    /// Fetch a motor's thrust curve from ThrustCurve.org into the cache, so `hpr sim` flies it
    /// offline
    Fetch(crate::motor_fetch::FetchArgs),
}

/// `hpr motors list`'s filters. Each one given must match.
#[derive(Debug, clap::Args)]
pub struct ListArgs {
    /// Only this impulse class, such as H or 1/2A
    #[arg(long)]
    pub class: Option<String>,
    /// Only this casing diameter, mm (within 0.5 mm)
    #[arg(long, value_name = "MM", allow_hyphen_values = true, value_parser = crate::typed::number::<f64>())]
    pub diameter: Option<f64>,
    /// Only this manufacturer: its name or abbreviation, any case
    #[arg(long)]
    pub manufacturer: Option<String>,
}

/// `hpr motors show`'s argument.
#[derive(Debug, clap::Args)]
pub struct ShowArgs {
    /// A catalog motor's name (J760, 1266J760-19A), or the path of a .eng or .rse file
    pub motor: String,
}

/// A motor file format `hpr motors show` reads, by its extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotorFile {
    /// RASP `.eng`.
    Eng,
    /// RockSim `.rse`.
    Rse,
}

impl MotorFile {
    /// Every format read, in the order the command table lists them.
    pub const ALL: [Self; 2] = [Self::Eng, Self::Rse];

    /// The extension, with its dot.
    pub fn extension(self) -> &'static str {
        match self {
            Self::Eng => ".eng",
            Self::Rse => ".rse",
        }
    }

    /// The format a path's extension names, in any case.
    pub fn of(path: &str) -> Option<Self> {
        let extension = Path::new(path).extension()?.to_str()?.to_ascii_lowercase();
        Self::ALL
            .into_iter()
            .find(|format| format.extension()[1..] == extension)
    }

    pub(crate) fn output(self) -> FileFormat {
        match self {
            Self::Eng => FileFormat::Eng,
            Self::Rse => FileFormat::Rse,
        }
    }
}

/// Runs `hpr motors`.
pub(crate) fn run(command: &MotorsCommand, to: &mut Out<'_>) -> Result<(), Failure> {
    match command {
        MotorsCommand::List(args) => list(args, to),
        MotorsCommand::Show(args) => show(&args.motor, to),
        MotorsCommand::Search(args) => crate::motor_search::run(args, to),
        MotorsCommand::Fetch(args) => crate::motor_fetch::run(args, to),
    }
}

/// The bundled catalog.
pub(crate) fn catalog() -> Result<Catalog, Failure> {
    Catalog::bundled()
        .map_err(|error| Failure::helped(format!("the bundled catalog: {error}"), crate::BUG_HELP))
}

/// The bundled catalog's as-of date, `YYYY-MM-DD`: the day its curve files were downloaded, which
/// a build fixes, so a run's files stay the same from one run to the next (ADR-174 §5).
pub(crate) fn catalog_as_of() -> Result<String, Failure> {
    catalog().map(|catalog| catalog.snapshot.captured)
}

/// `hpr motors list`.
fn list(args: &ListArgs, to: &mut Out<'_>) -> Result<(), Failure> {
    let class = match &args.class {
        Some(class) => Some(class_label(class)?),
        None => None,
    };
    if let Some(diameter) = args.diameter
        && !(diameter.is_finite() && diameter > 0.0)
    {
        return Err(Failure::helped(
            format!("--diameter must be a positive number of millimeters, not {diameter}"),
            "give the casing's diameter in millimeters, such as 38 or 54",
        ));
    }
    let catalog = catalog()?;
    let manufacturer = match &args.manufacturer {
        Some(name) => Some(maker(&catalog, name)?),
        None => None,
    };
    let motors = catalog
        .motors
        .iter()
        .filter(|motor| {
            class
                .as_ref()
                .is_none_or(|c| motor.impulse_class.label() == *c)
        })
        .filter(|motor| {
            args.diameter
                .is_none_or(|mm| (motor.diameter_mm - mm).abs() <= 0.5)
        })
        .filter(|motor| {
            manufacturer.as_ref().is_none_or(|name| {
                motor.manufacturer.to_lowercase() == *name
                    || motor.manufacturer_abbrev.to_lowercase() == *name
            })
        })
        .map(listed)
        .collect::<Result<_, _>>()?;
    let list = MotorList {
        catalog: CatalogInfo {
            source: catalog.source.clone(),
            captured: catalog.snapshot.captured.clone(),
            selection: catalog.selection.clone(),
        },
        motors,
    };
    to.emit(&list, |out, _| list_text(&list, out))
}

/// The class label `--class` names, as [`ImpulseClass::label`] writes it, or a refusal.
pub(crate) fn class_label(class: &str) -> Result<String, Failure> {
    class
        .trim()
        .parse::<ImpulseClass>()
        .map(ImpulseClass::label)
        .map_err(|_| {
            Failure::helped(
                format!("--class {class} is not an impulse class"),
                "use 1/8A, 1/4A, 1/2A or a letter A to Z",
            )
        })
}

/// The lowercased maker `--manufacturer` names, or a refusal listing the catalog's makers: a
/// name that matches no maker would otherwise list nothing, as if the maker had no motors.
fn maker(catalog: &Catalog, name: &str) -> Result<String, Failure> {
    let wanted = name.trim().to_lowercase();
    let known = catalog.motors.iter().any(|motor| {
        motor.manufacturer.to_lowercase() == wanted
            || motor.manufacturer_abbrev.to_lowercase() == wanted
    });
    if known {
        return Ok(wanted);
    }
    let mut makers: Vec<&str> = catalog
        .motors
        .iter()
        .map(|motor| motor.manufacturer_abbrev.as_str())
        .collect();
    makers.sort_unstable();
    makers.dedup();
    Err(Failure::helped(
        format!("--manufacturer {name} is no maker in the bundled catalog"),
        format!("use one of {}, or a maker's full name", makers.join(", ")),
    ))
}

/// One catalog motor, as listed.
fn listed(motor: &CatalogMotor) -> Result<ListedMotor, Failure> {
    Ok(ListedMotor {
        designation: motor.designation.clone(),
        common_name: motor.common_name.clone(),
        manufacturer: motor.manufacturer.clone(),
        manufacturer_abbrev: motor.manufacturer_abbrev.clone(),
        impulse_class: motor.impulse_class.label(),
        motor_type: kind(motor.motor_type)?,
        diameter_mm: motor.diameter_mm,
        length_mm: motor.length_mm,
        total_impulse_ns: motor.total_impulse_ns,
        average_thrust_n: motor.average_thrust_n,
        burn_time_s: motor.burn_time_s,
        delays: motor.delays.clone(),
    })
}

/// `hpr_motor`'s enums are non-exhaustive. A kind this build doesn't know is refused, not given
/// the name of another; the library is this build's own, so that is HPR Sim's bug.
fn unknown(what: &str, value: impl std::fmt::Debug) -> Failure {
    Failure::helped(
        format!("{what} {value:?} is new to this build of HPR Sim"),
        crate::BUG_HELP,
    )
}

fn kind(motor_type: MotorType) -> Result<MotorKind, Failure> {
    match motor_type {
        MotorType::SingleUse => Ok(MotorKind::SingleUse),
        MotorType::Reload => Ok(MotorKind::Reload),
        MotorType::Hybrid => Ok(MotorKind::Hybrid),
        other => Err(unknown("the motor type", other)),
    }
}

/// `hpr motors list` as text: a table.
fn list_text(list: &MotorList, out: &mut Text<'_>) -> io::Result<()> {
    writeln!(
        out,
        "{} motors from {}, downloaded {}; size, masses and delays from each file's header, the \
         rest from its curve.",
        list.motors.len(),
        list.catalog.source,
        list.catalog.captured
    )?;
    if list.motors.is_empty() {
        return Ok(());
    }
    writeln!(out)?;
    let header = [
        "designation",
        "maker",
        "class",
        "dia mm (in)",
        "len mm (in)",
        "impulse N·s",
        "avg N",
        "burn s",
        "delays",
    ];
    // The size in inches too, to the places `hpr motors show` gives (ADR-210, ADR-213).
    let size = |mm: fn(&ListedMotor) -> f64, decimals: usize| {
        bracketed(
            &list
                .motors
                .iter()
                .map(|m| {
                    (
                        format!("{}", mm(m)),
                        inches_figure(mm(m) / 1000.0, decimals),
                    )
                })
                .collect::<Vec<_>>(),
        )
    };
    let diameters = size(|m| m.diameter_mm, 2);
    let lengths = size(|m| m.length_mm, 1);
    let rows: Vec<[String; 9]> = list
        .motors
        .iter()
        .zip(diameters.into_iter().zip(lengths))
        .map(|(m, (diameter, length))| {
            [
                m.designation.clone(),
                m.manufacturer_abbrev.clone(),
                m.impulse_class.clone(),
                diameter,
                length,
                format!("{:.1}", m.total_impulse_ns),
                format!("{:.1}", m.average_thrust_n),
                format!("{:.2}", m.burn_time_s),
                m.delays.clone().unwrap_or_else(|| "-".to_owned()),
            ]
        })
        .collect();
    let mut widths = header.map(|h| h.chars().count());
    for row in &rows {
        for (width, cell) in widths.iter_mut().zip(row) {
            *width = (*width).max(cell.chars().count());
        }
    }
    let line = |cells: &[&str]| -> String {
        let mut text = String::new();
        for (i, (cell, width)) in cells.iter().zip(widths).enumerate() {
            let pad = width - cell.chars().count();
            // Text columns align left, numbers right; the last column isn't padded.
            if i == cells.len() - 1 {
                text.push_str(cell);
            } else if (3..=7).contains(&i) {
                text.push_str(&" ".repeat(pad));
                text.push_str(cell);
                text.push_str("  ");
            } else {
                text.push_str(cell);
                text.push_str(&" ".repeat(pad + 2));
            }
        }
        text.trim_end().to_owned()
    };
    out.heading(&line(&header))?;
    for row in &rows {
        writeln!(out, "{}", line(&row.each_ref().map(String::as_str)))?;
    }
    Ok(())
}

/// `hpr motors show`.
fn show(motor: &str, to: &mut Out<'_>) -> Result<(), Failure> {
    let (show, propellants) = match MotorFile::of(motor) {
        Some(format) => {
            let show = from_file(motor, format)?;
            // A motor file names no propellant.
            let none = vec![None; show.motors.len()];
            (show, none)
        }
        None => from_catalog(motor)?,
    };
    to.emit(&show, |out, diagnostics| {
        show_text(&show, &propellants, out, diagnostics)
    })
}

/// The catalog's designations that begin with `name`, as [`Catalog::find`] compares names
/// (lowercase, without spaces or hyphens), by designation or common name: `J350` begins
/// `J350W-L`. At most [`MOST_NEAR`].
fn catalog_near(catalog: &Catalog, name: &str) -> Vec<String> {
    let key = normalized(name);
    if key.is_empty() {
        return Vec::new();
    }
    catalog
        .motors
        .iter()
        .filter(|motor| {
            normalized(&motor.designation).starts_with(&key)
                || normalized(&motor.common_name).starts_with(&key)
        })
        .map(|motor| motor.designation.clone())
        .take(MOST_NEAR)
        .collect()
}

/// The most catalog motors a refusal names.
const MOST_NEAR: usize = 5;

/// `name` lowercase, without spaces or hyphens, as the catalog compares names.
fn normalized(name: &str) -> String {
    name.chars()
        .filter(|c| !c.is_whitespace() && *c != '-')
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

/// Names as a list in words: `A`, `A and B`, `A, B and C`.
fn in_words(names: &[String]) -> String {
    match names {
        [] => String::new(),
        [one] => one.clone(),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
}

/// Every catalog motor `name` matches, with its bundled curve, and beside each the propellant
/// the catalog names (ThrustCurve.org's `propInfo`), which the JSON doesn't carry.
fn from_catalog(name: &str) -> Result<(MotorShow, Vec<Option<String>>), Failure> {
    let catalog = catalog()?;
    let matches: Vec<&CatalogMotor> = catalog.find(name).collect();
    if matches.is_empty() {
        // The most useful last: the catalog's list, then what ThrustCurve.org answers, then the
        // catalog's own motors that the name begins.
        let mut help = vec![
            "`hpr motors list` lists the catalog's motors, and a .eng or .rse file can be shown \
             by its path"
                .to_owned(),
        ];
        help.extend(crate::motor_fetch::cached_answer(name));
        let near = catalog_near(&catalog, name);
        if !near.is_empty() {
            help.push(format!("the catalog has {}", in_words(&near)));
        }
        return Err(Failure::Helped {
            message: format!("no motor in the bundled catalog is called {name}"),
            help,
        });
    }
    let mut show = MotorShow {
        kind: crate::trust::Kind::Copied,
        trust: crate::trust::catalog_motor(&catalog.snapshot.captured),
        motors: Vec::new(),
        warnings: Vec::new(),
    };
    let mut propellants = Vec::new();
    for motor in matches {
        let solid = motor
            .bundled_motor()
            // The bundled catalog's curves are this build's own.
            .map_err(|error| {
                Failure::library(format!("{}: {error}", motor.designation), crate::BUG_HELP)
            })?;
        // The curve `bundled_motor` flew: the first with a bundled file, which it just found.
        let Some(curve) = motor
            .curves
            .iter()
            .find(|curve| bundled_curve_text(&curve.file).is_some())
        else {
            continue;
        };
        let format = match curve.format {
            CurveFormat::Rasp => MotorFile::Eng,
            CurveFormat::RockSim => MotorFile::Rse,
            // `CurveFormat` is non-exhaustive, and the bundled catalog uses these two only.
            _ => {
                return Err(Failure::helped(
                    format!(
                        "{}: its curve's format isn't .eng or .rse",
                        motor.designation
                    ),
                    crate::BUG_HELP,
                ));
            }
        };
        let delays = motor.delays();
        show.warnings
            .extend(delay_warnings(&motor.designation, &delays)?);
        show.motors.push(figures(
            &motor.designation,
            &motor.manufacturer,
            MotorSource::Catalog {
                curve_url: curve.info_url.clone().unwrap_or_else(|| curve.url.clone()),
                format: format.output(),
            },
            Envelope {
                diameter_m: motor.diameter_mm / 1000.0,
                length_m: motor.length_mm / 1000.0,
                propellant_mass_kg: solid.propellant_initial_mass_kg(),
                loaded_mass_kg: solid.propellant_initial_mass_kg() + solid.dry().mass_kg,
            },
            solid.curve(),
            &delays,
        )?);
        propellants.push(
            motor
                .prop_info
                .as_deref()
                .map(str::trim)
                .filter(|name| !name.is_empty())
                .map(crate::printable),
        );
    }
    Ok((show, propellants))
}

/// Every motor in a `.eng` or `.rse` file.
fn from_file(path: &str, format: MotorFile) -> Result<MotorShow, Failure> {
    let bytes = crate::read_file(path)?;
    let shown = crate::printable(path);
    let text = String::from_utf8(bytes).map_err(|_| {
        Failure::helped(
            format!("{shown}: not a text file in UTF-8"),
            format!(
                "a {} file is plain text: check that this is one, or save it again as UTF-8",
                format.extension()
            ),
        )
    })?;
    // What to do about a file that breaks its format.
    let rules = format!(
        "check the file against {}, or download it again from ThrustCurve.org",
        format_page(format)
    );
    let source = || MotorSource::File {
        path: path.to_owned(),
        format: format.output(),
    };
    let refused =
        |error: hpr::hpr_motor::MotorError| Failure::library(format!("{shown}: {error}"), &rules);
    let mut show = MotorShow {
        kind: crate::trust::Kind::Copied,
        trust: crate::trust::motor_file(),
        motors: Vec::new(),
        warnings: Vec::new(),
    };
    match format {
        MotorFile::Eng => {
            let parsed = eng::parse(&text).map_err(refused)?;
            show.warnings.extend(read_warnings(&parsed.warnings)?);
            for entry in &parsed.value.entries {
                let curve = entry.thrust_curve().map_err(|error| {
                    Failure::library(format!("{shown}: {}: {error}", entry.name), &rules)
                })?;
                let delays = entry.delays();
                show.warnings.extend(delay_warnings(&entry.name, &delays)?);
                show.motors.push(figures(
                    &entry.name,
                    &entry.manufacturer,
                    source(),
                    Envelope {
                        diameter_m: entry.diameter_mm / 1000.0,
                        length_m: entry.length_mm / 1000.0,
                        propellant_mass_kg: entry.propellant_mass_kg,
                        loaded_mass_kg: entry.total_mass_kg,
                    },
                    &curve,
                    &delays,
                )?);
            }
        }
        MotorFile::Rse => {
            let parsed = rse::parse(&text).map_err(refused)?;
            show.warnings.extend(read_warnings(&parsed.warnings)?);
            for engine in &parsed.value.engines {
                let curve = engine.thrust_curve().map_err(|error| {
                    Failure::library(format!("{shown}: {}: {error}", engine.code), &rules)
                })?;
                let delays = engine.delays();
                show.warnings.extend(delay_warnings(&engine.code, &delays)?);
                show.motors.push(figures(
                    &engine.code,
                    engine.manufacturer.trim(),
                    source(),
                    Envelope {
                        diameter_m: engine.diameter_mm / 1000.0,
                        length_m: engine.length_mm / 1000.0,
                        propellant_mass_kg: engine.propellant_mass_g / 1000.0,
                        loaded_mass_kg: engine.initial_mass_g / 1000.0,
                    },
                    &curve,
                    &delays,
                )?);
            }
        }
    }
    if show.motors.is_empty() {
        return Err(Failure::helped(
            format!("{shown}: no motor could be read"),
            format!(
                "the file holds no motor entry; check that it is a {} file, as {} describes",
                format.extension(),
                format_page(format)
            ),
        ));
    }
    Ok(show)
}

/// The docs page that writes down a motor file format.
fn format_page(format: MotorFile) -> &'static str {
    match format {
        MotorFile::Eng => "https://hpr.fusionspace.co/format/eng.html",
        MotorFile::Rse => "https://hpr.fusionspace.co/format/rse.html",
    }
}

/// A motor's casing and masses, SI.
struct Envelope {
    diameter_m: f64,
    length_m: f64,
    propellant_mass_kg: f64,
    loaded_mass_kg: f64,
}

/// A motor's figures from its curve.
fn figures(
    name: &str,
    manufacturer: &str,
    source: MotorSource,
    envelope: Envelope,
    curve: &ThrustCurve,
    delays: &DelayList,
) -> Result<MotorFigures, Failure> {
    let total_impulse_ns = curve.total_impulse_ns();
    let impulse_class = ImpulseClass::from_total_impulse(total_impulse_ns)
        .map_err(|error| {
            Failure::library(
                format!("{name}: {error}"),
                "a motor's total impulse is above zero and within class Z; check the curve's \
                 thrust points, newtons against seconds",
            )
        })?
        .label();
    let (burn_start_s, burn_end_s) = curve.burn_window_s();
    Ok(MotorFigures {
        name: name.to_owned(),
        manufacturer: manufacturer.to_owned(),
        source,
        diameter_m: envelope.diameter_m,
        length_m: envelope.length_m,
        propellant_mass_kg: envelope.propellant_mass_kg,
        loaded_mass_kg: envelope.loaded_mass_kg,
        total_impulse_ns,
        impulse_class,
        average_thrust_n: curve.average_thrust_n(),
        peak_thrust_n: curve.peak_thrust_n(),
        burn_time_s: curve.burn_time_s(),
        burn_start_s,
        burn_end_s,
        curve_end_s: curve.end_time_s(),
        delays: delays
            .delays
            .iter()
            .map(|&d| delay_out(d))
            .collect::<Result<_, _>>()?,
    })
}

pub(crate) fn delay_out(delay: delay::Delay) -> Result<Delay, Failure> {
    match delay {
        delay::Delay::Seconds(s) => Ok(Delay::Seconds(s)),
        delay::Delay::Plugged => Ok(Delay::Plugged),
        delay::Delay::ZeroOrPlugged => Ok(Delay::ZeroOrPlugged),
        other => Err(unknown("the delay", other)),
    }
}

pub(crate) fn warning_kind(kind: ReadWarning) -> Result<WarningKind, Failure> {
    match kind {
        ReadWarning::Skipped => Ok(WarningKind::Skipped),
        ReadWarning::Dropped => Ok(WarningKind::Dropped),
        ReadWarning::Unusual => Ok(WarningKind::Unusual),
        other => Err(unknown("the warning kind", other)),
    }
}

pub(crate) fn read_warnings(warnings: &[ParseWarning]) -> Result<Vec<Warning>, Failure> {
    warnings
        .iter()
        .map(|warning| {
            Ok(Warning {
                motor: None,
                line: Some(warning.line),
                kind: warning_kind(warning.kind)?,
                message: warning.message.clone(),
            })
        })
        .collect()
}

fn delay_warnings(motor: &str, delays: &DelayList) -> Result<Vec<Warning>, Failure> {
    delays
        .warnings
        .iter()
        .map(|warning| {
            Ok(Warning {
                motor: Some(motor.to_owned()),
                line: None,
                kind: warning_kind(warning.kind)?,
                message: warning.message.clone(),
            })
        })
        .collect()
}

/// `hpr motors show` as text, `propellants` the propellant the catalog names for each motor.
fn show_text(
    show: &MotorShow,
    propellants: &[Option<String>],
    out: &mut dyn Write,
    diagnostics: &mut Diagnostics<'_>,
) -> io::Result<()> {
    for (i, motor) in show.motors.iter().enumerate() {
        let designation = Designation::read(&motor.name, &motor.manufacturer);
        if i > 0 {
            writeln!(out)?;
        }
        let from = match &motor.source {
            MotorSource::Catalog { .. } => "the bundled catalog".to_owned(),
            MotorSource::File { path, .. } => path.clone(),
        };
        writeln!(out, "{} ({}), from {from}", motor.name, motor.manufacturer)?;
        writeln!(
            out,
            "  impulse class    {}",
            class_range(&motor.impulse_class)
        )?;
        writeln!(out, "  total impulse    {:.1} N·s", motor.total_impulse_ns)?;
        writeln!(
            out,
            "  average thrust   {}",
            average_thrust(motor.average_thrust_n, designation.as_ref())
        )?;
        writeln!(out, "  peak thrust      {:.1} N", motor.peak_thrust_n)?;
        writeln!(
            out,
            "  burn time        {:.2} s, from {:.3} s to {:.3} s (NFPA 1125, 5% of peak)",
            motor.burn_time_s, motor.burn_start_s, motor.burn_end_s
        )?;
        writeln!(
            out,
            "  propellant       {}",
            propellant(
                designation.as_ref(),
                propellants.get(i).and_then(Option::as_deref),
                matches!(motor.source, MotorSource::Catalog { .. })
            )
        )?;
        writeln!(
            out,
            "  propellant mass  {:.1} g ({}) of {:.1} g ({}) loaded",
            motor.propellant_mass_kg * 1e3,
            ounces_or_pounds(motor.propellant_mass_kg),
            motor.loaded_mass_kg * 1e3,
            ounces_or_pounds(motor.loaded_mass_kg)
        )?;
        writeln!(
            out,
            "  casing           {} mm ({}) across, {} mm ({}) long",
            round_mm(motor.diameter_m),
            inches(motor.diameter_m, 2),
            round_mm(motor.length_m),
            inches(motor.length_m, 1)
        )?;
        writeln!(out, "  delays           {}", delays_text(&motor.delays))?;
        if let MotorSource::Catalog { curve_url, .. } = &motor.source {
            writeln!(out, "  curve file       {curve_url} (public domain)")?;
        }
    }
    crate::sim_text::trust_lines(&show.trust, out)?;
    for warning in &show.warnings {
        let at = match (&warning.motor, warning.line) {
            (Some(motor), _) => format!("{motor}: "),
            (None, Some(line)) => format!("line {line}: "),
            (None, None) => String::new(),
        };
        diagnostics.line(Level::Warning, &format!("{at}{}", warning.message))?;
    }
    Ok(())
}

/// What a motor's designation says about it, as the makers write it (`H170M`, `1266J760-19A`):
/// an optional total impulse in N·s, the impulse class (`H`, `1/2A`), the nominal average thrust
/// in newtons, then any letters joined to the thrust, the maker's code for the propellant (`M`,
/// `WS`). What follows a hyphen (a delay, `-14A`, `-P`, `-L`, or Loki's propellant, `-CT`) is
/// not read, as the same place holds a delay for one maker and a propellant for another, except
/// in the form Cesaroni's motor files write, its parts all hyphenated with the total impulse
/// first, `131-G84-GR-10A`, where the part after the thrust is the propellant's code unless it is
/// a single delay letter (`131-G84-P` is plugged; `S`, `M`, `L` and `H` name delays too).
///
/// The black-powder makers, Estes and Quest, join letters to the thrust that aren't a
/// propellant: the `T` of `A10T` and `1/4A3T` marks the 13 mm mini case. So where the maker is
/// known to be one of them, no joined letters are read as a code.
#[derive(Debug, Clone, PartialEq)]
struct Designation {
    /// The nominal average thrust, N.
    nominal_thrust_n: u32,
    /// The letters joined to the thrust, if any: the maker's propellant code.
    propellant: Option<String>,
}

impl Designation {
    /// Reads `name`, the designation of a motor `manufacturer` makes (as the catalog or the motor
    /// file names the maker; empty where unknown), or `None` where it isn't shaped like a
    /// designation.
    fn read(name: &str, manufacturer: &str) -> Option<Self> {
        let name = name.trim();
        // A Cesaroni designation leads with its total impulse, its files' with a hyphen after it.
        // A fraction of an A class (`1/2A`) leads with digits of its own.
        let (rest, hyphenated) = match ["1/8", "1/4", "1/2"]
            .iter()
            .find_map(|fraction| name.strip_prefix(fraction))
        {
            Some(rest) => (rest, false),
            None => {
                let rest = name.trim_start_matches(|c: char| c.is_ascii_digit());
                match rest.strip_prefix('-') {
                    Some(after) if rest.len() < name.len() => (after, true),
                    _ => (rest, false),
                }
            }
        };
        let mut chars = rest.chars();
        if !chars.next().is_some_and(|c| c.is_ascii_uppercase()) {
            return None;
        }
        let rest = chars.as_str();
        let digits_end = rest
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(rest.len());
        let nominal_thrust_n = rest[..digits_end].parse().ok()?;
        let rest = &rest[digits_end..];
        let letters_end = rest
            .find(|c: char| !c.is_ascii_alphabetic())
            .unwrap_or(rest.len());
        // Anything after the letters is a hyphen's part, or the name isn't a designation.
        if !(rest[letters_end..].is_empty() || rest[letters_end..].starts_with('-')) {
            return None;
        }
        let mut letters = &rest[..letters_end];
        if black_powder_maker(manufacturer) {
            letters = "";
        } else if letters.is_empty() && hyphenated {
            let part = rest[letters_end..].trim_start_matches('-');
            let part = &part[..part.find('-').unwrap_or(part.len())];
            let delay_letter = matches!(part, "P" | "S" | "M" | "L" | "H");
            if !delay_letter && part.chars().all(|c| c.is_ascii_alphabetic() || c == '_') {
                letters = part;
            }
        }
        Some(Self {
            nominal_thrust_n,
            propellant: (!letters.is_empty()).then(|| letters.to_owned()),
        })
    }
}

/// Whether `manufacturer` is Estes or Quest, the black-powder makers, as the catalog
/// (`Estes Industries`, `Quest Aerospace`) or a motor file (`Estes`, `Quest`) names them, in any
/// case.
fn black_powder_maker(manufacturer: &str) -> bool {
    let maker = manufacturer.trim().to_ascii_lowercase();
    maker.starts_with("estes") || maker.starts_with("quest")
}

/// An impulse class with its range of total impulse in the form the standards write it, the
/// lower end exclusive: `H (160.01–320 N·s)`, `A (1.26–2.50 N·s)`. NFPA 1125 (through `G`) and
/// NFPA 1127 (`H` through `O`) band each class from just above the class below's top, written
/// one step in its last decimal place above it (two decimals at least: `160.01`, `0.626`), to its
/// own top, inclusive; `1/8A` starts at zero. `P` and above extend the doubling rule in the same
/// form, as [`ImpulseClass`] does. A label that isn't a class prints alone.
fn class_range(label: &str) -> String {
    match label.parse::<ImpulseClass>() {
        Ok(class) => {
            let upper = class.upper_limit_ns();
            let below = class.lower_limit_ns();
            let lower = if below == 0.0 {
                "0".to_owned()
            } else {
                // The class below's top, as written, and one step in its last place above it.
                let decimals = written_decimals(below).max(2);
                let step = 10f64.powi(-i32::try_from(decimals).unwrap_or(2));
                format!("{:.decimals$}", below + step)
            };
            let upper = match written_decimals(upper) {
                0 => format!("{upper}"),
                decimals => format!("{upper:.0$}", decimals.max(2)),
            };
            format!("{label} ({lower}–{upper} N·s)")
        }
        Err(_) => label.to_owned(),
    }
}

/// The decimal places of `value`'s shortest exact form: 0 for 320, 1 for 2.5, 4 for 0.3125. The
/// class limits are 1.25 · 2^k N·s, so each is a short exact decimal.
fn written_decimals(value: f64) -> usize {
    let text = format!("{value}");
    text.find('.').map_or(0, |dot| text.len() - dot - 1)
}

/// The measured average thrust, and the nominal one the designation names where the two differ
/// once the measured is rounded to a whole newton, as the designation's is.
fn average_thrust(measured_n: f64, designation: Option<&Designation>) -> String {
    match designation {
        Some(named) if measured_n.round() != f64::from(named.nominal_thrust_n) => format!(
            "{measured_n:.1} N measured; {} N nominal, as the designation names it",
            named.nominal_thrust_n
        ),
        Some(_) => format!("{measured_n:.1} N measured, as the designation names it"),
        None => format!("{measured_n:.1} N"),
    }
}

/// The propellant: the designation's letters, as the maker's code, and the name the catalog gives
/// it, with no meaning made up for a code the catalog doesn't name. `from_catalog` says where the
/// motor came from, the bundled catalog or a motor file, so a missing name blames its source.
fn propellant(
    designation: Option<&Designation>,
    named: Option<&str>,
    from_catalog: bool,
) -> String {
    let code = designation.and_then(|designation| designation.propellant.as_deref());
    let source = if from_catalog {
        "the catalog"
    } else {
        "the motor file"
    };
    match (code, named) {
        (Some(code), Some(name)) => {
            format!("{code}, the maker's code for {name}, as ThrustCurve.org names it")
        }
        (Some(code), None) => {
            format!("{code}, the maker's code; {source} doesn't name the propellant")
        }
        (None, Some(name)) => format!("{name}, as ThrustCurve.org names it"),
        (None, None) => format!("not named by the designation or {source}"),
    }
}

/// A length in millimeters, to a tenth and without a trailing `.0`.
fn round_mm(length_m: f64) -> String {
    let mm = (length_m * 1e4).round() / 10.0;
    format!("{mm}")
}

fn delays_text(delays: &[Delay]) -> String {
    if delays.is_empty() {
        return "none listed".to_owned();
    }
    delays
        .iter()
        .map(|delay| match delay {
            Delay::Seconds(s) => format!("{s} s"),
            Delay::Plugged => "plugged".to_owned(),
            Delay::ZeroOrPlugged => "0 (at burnout, or plugged)".to_owned(),
        })
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The nominal thrust and the propellant's code `name` gives, `""` for none.
    fn read(name: &str) -> Option<(u32, String)> {
        Designation::read(name, "").map(|designation| {
            (
                designation.nominal_thrust_n,
                designation.propellant.unwrap_or_default(),
            )
        })
    }

    /// Each maker's form gives its nominal thrust, and its propellant's code only where the code
    /// can't be a delay: joined to the thrust, or in a Cesaroni file's hyphenated form.
    #[test]
    fn a_designation_gives_its_thrust_and_propellant_code() {
        assert_eq!(read("H170M"), Some((170, "M".to_owned())));
        assert_eq!(read("H170M-14"), Some((170, "M".to_owned())));
        assert_eq!(read("J350W-L"), Some((350, "W".to_owned())));
        assert_eq!(read("I175WS"), Some((175, "WS".to_owned())));
        assert_eq!(read("1266J760-19A"), Some((760, String::new())));
        assert_eq!(read("2245K1075-P"), Some((1075, String::new())));
        assert_eq!(read("131-G84-GR-10A"), Some((84, "GR".to_owned())));
        assert_eq!(read("168-H54-WH_LB-10A"), Some((54, "WH_LB".to_owned())));
        assert_eq!(read("21062-O3400-IM-P"), Some((3400, "IM".to_owned())));
        // Loki's catalog form: the code after a hyphen, where AeroTech writes a delay.
        assert_eq!(read("H125-CT"), Some((125, String::new())));
        assert_eq!(read("H125CT"), Some((125, "CT".to_owned())));
        assert_eq!(read("C5"), Some((5, String::new())));
        // A single delay letter after a Cesaroni file's thrust is no propellant: `P` is plugged.
        assert_eq!(read("131-G84-P"), Some((84, String::new())));
        assert_eq!(read("3300-L3200-P"), Some((3200, String::new())));
        for delay in ["S", "M", "L", "H"] {
            assert_eq!(
                read(&format!("131-G84-{delay}")),
                Some((84, String::new())),
                "{delay}"
            );
        }
        assert_eq!(read("1/2A3-4T"), Some((3, String::new())));
        for not_one in ["", "H", "170M", "h170M", "H170M2", "Motor", "-H170"] {
            assert_eq!(read(not_one), None, "{not_one}");
        }
    }

    /// Estes's and Quest's letters joined to the thrust aren't a propellant: the `T` of `A10T` and
    /// `1/4A3T` marks the 13 mm mini case. Where the maker is one of them, as the catalog or a
    /// motor file names it, no code is read; an unknown maker's joined letters still are.
    #[test]
    fn a_black_powder_makers_letters_are_no_propellant() {
        let code = |name: &str, maker: &str| {
            Designation::read(name, maker)
                .map(|designation| (designation.nominal_thrust_n, designation.propellant))
        };
        for maker in [
            "Estes Industries",
            "Estes",
            "Quest Aerospace",
            "quest",
            " ESTES ",
        ] {
            assert_eq!(code("A10T", maker), Some((10, None)), "{maker}");
            assert_eq!(code("1/2A3T", maker), Some((3, None)), "{maker}");
            assert_eq!(code("1/4A3T", maker), Some((3, None)), "{maker}");
            assert_eq!(code("C6-5", maker), Some((6, None)), "{maker}");
        }
        assert_eq!(code("H170M", "AeroTech"), Some((170, Some("M".to_owned()))));
        assert_eq!(code("A10T", ""), Some((10, Some("T".to_owned()))));
    }

    /// A class's range is in NFPA's written form, the lower end exclusive and written one step
    /// above the class below's top; a label that isn't a class prints alone.
    #[test]
    fn a_class_gives_its_range() {
        assert_eq!(class_range("H"), "H (160.01–320 N·s)");
        assert_eq!(class_range("J"), "J (640.01–1280 N·s)");
        assert_eq!(class_range("O"), "O (20480.01–40960 N·s)");
        assert_eq!(class_range("P"), "P (40960.01–81920 N·s)");
        assert_eq!(class_range("1/8A"), "1/8A (0–0.3125 N·s)");
        assert_eq!(class_range("1/4A"), "1/4A (0.3126–0.625 N·s)");
        assert_eq!(class_range("1/2A"), "1/2A (0.626–1.25 N·s)");
        assert_eq!(class_range("A"), "A (1.26–2.50 N·s)");
        assert_eq!(class_range("B"), "B (2.51–5 N·s)");
        assert_eq!(class_range("G"), "G (80.01–160 N·s)");
        assert_eq!(class_range("??"), "??");
    }

    /// The nominal thrust shows beside the measured one only where the two differ once the
    /// measured is rounded to a whole newton.
    #[test]
    fn the_nominal_thrust_shows_where_it_differs() {
        let named = Designation::read("H170M", "AeroTech");
        assert_eq!(
            average_thrust(165.4, named.as_ref()),
            "165.4 N measured; 170 N nominal, as the designation names it"
        );
        assert_eq!(
            average_thrust(169.5, named.as_ref()),
            "169.5 N measured, as the designation names it"
        );
        assert_eq!(
            average_thrust(170.49, named.as_ref()),
            "170.5 N measured, as the designation names it"
        );
        assert_eq!(
            average_thrust(170.5, named.as_ref()),
            "170.5 N measured; 170 N nominal, as the designation names it"
        );
        assert_eq!(average_thrust(165.4, None), "165.4 N");
    }

    /// The propellant line says what is known and makes up nothing: a code alone is the maker's,
    /// and a missing name is put down to where the motor came from, the catalog or the file.
    #[test]
    fn the_propellant_says_only_what_is_known() {
        let coded = Designation::read("H170M", "AeroTech");
        let uncoded = Designation::read("1266J760-19A", "Cesaroni Technology");
        assert_eq!(
            propellant(coded.as_ref(), Some("Metalstorm"), true),
            "M, the maker's code for Metalstorm, as ThrustCurve.org names it"
        );
        assert_eq!(
            propellant(coded.as_ref(), None, false),
            "M, the maker's code; the motor file doesn't name the propellant"
        );
        assert_eq!(
            propellant(coded.as_ref(), None, true),
            "M, the maker's code; the catalog doesn't name the propellant"
        );
        assert_eq!(
            propellant(uncoded.as_ref(), Some("White Thunder"), true),
            "White Thunder, as ThrustCurve.org names it"
        );
        assert_eq!(
            propellant(None, None, false),
            "not named by the designation or the motor file"
        );
        assert_eq!(
            propellant(None, None, true),
            "not named by the designation or the catalog"
        );
    }
}
