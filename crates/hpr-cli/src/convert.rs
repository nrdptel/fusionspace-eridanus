//! `hpr convert`: a motor file written as a RASP `.eng` or a RockSim `.rse` file, or a design
//! written as a `.ork`, `.hpr` or `.hprz` file ([`crate::convert_design`]).
//!
//! The conversion is the library's own ([`hpr::hpr_motor::convert`]): `.eng` to `.rse` fills
//! what `.eng` doesn't give the way RockSim's files do, and `.rse` to `.eng` drops what `.eng`
//! can't hold, each named in a warning. The same format in and out rewrites the file in HPR Sim's
//! layout, with a `.eng` maker of several words joined by `_`. A catalog motor takes the
//! catalog's size and masses, the ones HPR Sim flies. What the reader flagged in the input is passed
//! on.

use std::io::{self, Write};
use std::path::Path;

use hpr::hpr_motor::DelayList;
use hpr::hpr_motor::catalog::{CatalogMotor, CurveFormat, bundled_curve_text};
use hpr::hpr_motor::convert::{self, ConvertWarning};
use hpr::hpr_motor::eng::{self, EngFile};
use hpr::hpr_motor::rse::{self, RseEngine, RseFile};
use hpr::hpr_motor::text::WarningKind as ReadWarning;

use crate::console::{Level, Paint};
use crate::convert_design;
use crate::motors::{MotorFile, catalog, read_warnings, warning_kind};
use crate::output::{Convert, ConvertMotors, ConvertedFile, MotorSource, Warning, WarningKind};
use crate::sim::same_file;
use crate::{Failure, Out};

/// `hpr convert`'s arguments.
#[derive(Debug, clap::Args)]
pub struct ConvertArgs {
    /// What to read: a motor file (.eng or .rse), a catalog motor's name, such as H170M, or a
    /// design (.ork, .hpr or .hprz)
    pub input: String,
    /// The file to write, by its extension: .eng or .rse for motors, .ork, .hpr or .hprz for a
    /// design; it is replaced if it exists
    pub output: String,
    /// A file to carry in a .hprz beside the design, under its file name; give it once for each
    #[arg(long, value_name = "FILE")]
    pub attach: Vec<String>,
    /// Delays for a .rse motor that gives none, when writing a .eng file, whose header must
    /// give them: such as 6-10-14, or P for plugged
    #[arg(long, value_name = "LIST")]
    pub delays: Option<String>,
}

/// The motors read, in their file's model.
enum Motors {
    Eng(EngFile),
    Rse(RseFile),
}

/// Runs `hpr convert`.
pub(crate) fn run(args: &ConvertArgs, to: &mut Out<'_>) -> Result<(), Failure> {
    if convert_design::is_design(&args.input) || convert_design::is_design(&args.output) {
        return convert_design::run(args, to);
    }
    let output = &args.output;
    let target = MotorFile::of(output).ok_or_else(|| {
        Failure::Input(format!(
            "{output}: hpr convert writes a .eng or a .rse file for motors, or a .ork, .hpr or \
             .hprz file for a design, named by its extension"
        ))
    })?;
    if !args.attach.is_empty() {
        return Err(Failure::Input(
            "--attach adds files to a .hprz design, and hpr convert is writing motors".to_owned(),
        ));
    }
    if let Some(delays) = &args.delays {
        let list = DelayList::parse(delays);
        if list.delays.is_empty()
            || delays.contains(char::is_whitespace)
            || list.warnings.iter().any(|w| w.kind == ReadWarning::Dropped)
        {
            return Err(Failure::Input(format!(
                "--delays {delays}: not a list of delays, such as 6-10-14, or P for plugged"
            )));
        }
    }
    let (source, format, text, figures) = read_input(&args.input)?;
    if matches!(source, MotorSource::File { .. }) && same_file(&args.input) == same_file(output) {
        return Err(Failure::Input(format!(
            "{output}: that is the file hpr convert reads, and it would write over it"
        )));
    }
    output_folder(output)?;
    let input = &args.input;
    let refused = |error| Failure::Input(format!("{input}: {error}"));
    let mut warnings = Vec::new();
    let mut motors = match format {
        MotorFile::Eng => {
            let parsed = eng::parse(&text).map_err(refused)?;
            warnings.extend(read_warnings(&parsed.warnings)?);
            Motors::Eng(parsed.value)
        }
        MotorFile::Rse => {
            let parsed = rse::parse(&text).map_err(refused)?;
            warnings.extend(read_warnings(&parsed.warnings)?);
            Motors::Rse(parsed.value)
        }
    };
    if let Some(figures) = &figures {
        warnings.extend(figures.apply(&mut motors, target == MotorFile::Rse));
    }
    // The engines a `.eng` header would have no delays for.
    let undelayed: Vec<String> = match &motors {
        Motors::Rse(file) if target == MotorFile::Eng => file
            .engines
            .iter()
            .filter(|engine| needs_delays(engine))
            .map(|engine| engine.code.clone())
            .collect(),
        _ => Vec::new(),
    };
    let has_hybrid = matches!(&motors, Motors::Rse(file) if file.engines.iter().any(is_hybrid));
    match (&args.delays, undelayed.is_empty()) {
        // A hybrid needs no delays, but the conversion refuses it, for the real reason.
        (Some(_), true) if !has_hybrid => {
            return Err(Failure::Input(
                "--delays gives the delays of a .rse motor that has none, for a .eng file; no \
                 motor this conversion writes needs them"
                    .to_owned(),
            ));
        }
        (None, false) => {
            return Err(Failure::helped(
                format!(
                    "{input}: {} give(s) no delays, and a .eng header must",
                    undelayed.join(", ")
                ),
                "give them with --delays",
            ));
        }
        _ => {}
    }
    let unwritable = |error| {
        Failure::Input(format!(
            "{input}: its motors can't be written as a {} file: {error}",
            target.extension()
        ))
    };
    let (written, names, converted) = match (motors, target) {
        (Motors::Eng(mut file), MotorFile::Eng) => {
            let joined = join_makers(&mut file);
            (
                eng::write(&file).map_err(unwritable)?,
                eng_names(&file),
                joined,
            )
        }
        (Motors::Rse(file), MotorFile::Rse) => (
            rse::write(&file).map_err(unwritable)?,
            rse_names(&file),
            Vec::new(),
        ),
        (Motors::Eng(file), MotorFile::Rse) => {
            let converted = convert::eng_to_rse(&file).map_err(refused)?;
            (
                rse::write(&converted.value).map_err(unwritable)?,
                rse_names(&converted.value),
                converted.warnings,
            )
        }
        (Motors::Rse(mut file), MotorFile::Eng) => {
            if let Some(delays) = &args.delays {
                for engine in &mut file.engines {
                    if needs_delays(engine) {
                        engine.delays = Some(delays.clone());
                    }
                }
            }
            let converted = convert::rse_to_eng(&file).map_err(refused)?;
            (
                eng::write(&converted.value).map_err(unwritable)?,
                eng_names(&converted.value),
                converted.warnings,
            )
        }
    };
    for warning in &converted {
        warnings.push(converted_warning(warning)?);
    }
    std::fs::write(output, written)
        .map_err(|error| Failure::Input(format!("{output}: {error}")))?;
    let document = ConvertMotors {
        input: source,
        output: ConvertedFile {
            path: output.clone(),
            format: target.output(),
        },
        motors: names,
        warnings,
    };
    let paint = to.paint;
    to.emit(&Convert::Motors(document.clone()), |out| {
        text_output(&document, input, out, paint)
    })
}

/// Refuses an output path whose folder doesn't exist.
pub(crate) fn output_folder(output: &str) -> Result<(), Failure> {
    match Path::new(output).parent() {
        Some(folder) if !folder.as_os_str().is_empty() && !folder.is_dir() => Err(Failure::Input(
            format!("{output}: there is no folder {}", folder.display()),
        )),
        _ => Ok(()),
    }
}

/// Whether a `.rse` engine gives no delays a `.eng` header could take: none, or none
/// [`DelayList`] reads. A hybrid is left to the conversion, which refuses it.
fn needs_delays(engine: &RseEngine) -> bool {
    !is_hybrid(engine)
        && engine
            .delays
            .as_deref()
            .is_none_or(|delays| DelayList::parse(delays).delays.is_empty())
}

/// Whether a `.rse` engine says it is a hybrid, which [`convert::rse_to_eng`] refuses.
fn is_hybrid(engine: &RseEngine) -> bool {
    engine
        .motor_type
        .as_deref()
        .is_some_and(|kind| kind.trim().eq_ignore_ascii_case("hybrid"))
}

/// Joins a maker of several words with `_`, as [`convert::rse_to_eng`] does: a `.eng` header is
/// seven fields, and OpenRocket refuses more.
fn join_makers(file: &mut EngFile) -> Vec<ConvertWarning> {
    let mut warnings = Vec::new();
    for entry in &mut file.entries {
        let joined = entry
            .manufacturer
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("_");
        if joined != entry.manufacturer {
            warnings.push(ConvertWarning {
                motor: entry.name.clone(),
                kind: hpr::hpr_motor::text::WarningKind::Unusual,
                message: format!(
                    "a .eng maker is one word, so {:?} is written {joined:?}",
                    entry.manufacturer
                ),
            });
            entry.manufacturer = joined;
        }
    }
    warnings
}

/// What the bundled catalog gives a motor, which HPR Sim flies in place of its curve file's header
/// ([`CatalogMotor::motor`](hpr::hpr_motor::catalog::CatalogMotor::motor)).
struct CatalogFigures {
    diameter_mm: f64,
    length_mm: f64,
    propellant_mass_g: Option<f64>,
    total_mass_g: Option<f64>,
}

impl CatalogFigures {
    /// Writes the catalog's figures over the file's, with a warning for each that differed, and
    /// for a `.rse` file written as one, rescales the figures worked out from them ([`rescale`]).
    fn apply(&self, motors: &mut Motors, to_rse: bool) -> Vec<Warning> {
        let mut warnings = Vec::new();
        let mut rescaled = Vec::new();
        let mut set = |motor: &str, field: &mut f64, value: f64, what: &str, unit: &str| {
            if field.to_bits() == value.to_bits() {
                return;
            }
            // A figure that differs only by a unit conversion's last bits (a header's
            // 0.0036000000000000003 kg against the catalog's 3.6 g) is the same figure: the
            // catalog's is written, with no warning.
            if (*field - value).abs() > 4.0 * f64::EPSILON * value.abs() {
                warnings.push(Warning {
                    motor: Some(motor.to_owned()),
                    line: None,
                    kind: WarningKind::Unusual,
                    message: format!(
                        "its curve file gives a {what} of {field} {unit}; the catalog gives \
                         {value} {unit}, which HPR Sim flies and this file takes"
                    ),
                });
            }
            *field = value;
        };
        match motors {
            Motors::Eng(file) => {
                for entry in &mut file.entries {
                    let name = entry.name.clone();
                    set(
                        &name,
                        &mut entry.diameter_mm,
                        self.diameter_mm,
                        "diameter",
                        "mm",
                    );
                    set(&name, &mut entry.length_mm, self.length_mm, "length", "mm");
                    // Written as `.eng`, a header that is already the mass hpr flies keeps its
                    // digits; as `.rse`, the catalog's grams are written as it gives them.
                    let flown = |kg: f64, g: f64| !to_rse && kg.to_bits() == flown_kg(g).to_bits();
                    if let Some(g) = self.propellant_mass_g
                        && !flown(entry.propellant_mass_kg, g)
                    {
                        let kg = convert::g_to_kg(g);
                        set(
                            &name,
                            &mut entry.propellant_mass_kg,
                            kg,
                            "propellant mass",
                            "kg",
                        );
                    }
                    if let Some(g) = self.total_mass_g
                        && !flown(entry.total_mass_kg, g)
                    {
                        let kg = convert::g_to_kg(g);
                        set(&name, &mut entry.total_mass_kg, kg, "loaded mass", "kg");
                    }
                }
            }
            Motors::Rse(file) => {
                for engine in &mut file.engines {
                    let code = engine.code.clone();
                    let (length, propellant, initial) = (
                        engine.length_mm,
                        engine.propellant_mass_g,
                        engine.initial_mass_g,
                    );
                    set(
                        &code,
                        &mut engine.diameter_mm,
                        self.diameter_mm,
                        "diameter",
                        "mm",
                    );
                    set(&code, &mut engine.length_mm, self.length_mm, "length", "mm");
                    if let Some(g) = self.propellant_mass_g {
                        set(
                            &code,
                            &mut engine.propellant_mass_g,
                            g,
                            "propellant mass",
                            "g",
                        );
                    }
                    if let Some(g) = self.total_mass_g {
                        set(&code, &mut engine.initial_mass_g, g, "loaded mass", "g");
                    }
                    if to_rse {
                        rescaled.extend(rescale(engine, length, propellant, initial));
                    }
                }
            }
        }
        warnings.extend(rescaled);
        warnings
    }
}

/// The mass HPR Sim flies for a catalog mass of `g` grams ([`CatalogMotor::motor`]'s product).
fn flown_kg(g: f64) -> f64 {
    g * 1e-3
}

/// Rescales the `.rse` figures worked out from the masses and the length, once the catalog's
/// have replaced the file's (`length`, `propellant` and `initial` are the file's): `massFrac =
/// 100 propWt / initWt`; `Isp`, which goes as `1 / propWt`; each point's `m`, which goes as
/// `propWt` ([`convert::eng_to_rse`]'s rule, `m₀ (1 − I / Itot)`); and each point's `cg`, which
/// goes as the length. A figure the file doesn't give stays out.
fn rescale(engine: &mut RseEngine, length: f64, propellant: f64, initial: f64) -> Option<Warning> {
    let mut changed = Vec::new();
    if (engine.initial_mass_g != initial || engine.propellant_mass_g != propellant)
        && let Some(fraction) = &mut engine.mass_fraction_pct
        && engine.initial_mass_g > 0.0
    {
        *fraction = 100.0 * engine.propellant_mass_g / engine.initial_mass_g;
        changed.push("massFrac");
    }
    if engine.propellant_mass_g != propellant && propellant > 0.0 && engine.propellant_mass_g > 0.0
    {
        // Multiplied, then divided: a ratio taken first would round once more.
        let new = engine.propellant_mass_g;
        if let Some(isp) = &mut engine.isp_s {
            *isp = *isp * propellant / new;
            changed.push("Isp");
        }
        let mut any = false;
        for mass in engine.points.iter_mut().filter_map(|p| p.mass_g.as_mut()) {
            *mass = *mass * new / propellant;
            any = true;
        }
        if any {
            changed.push("m");
        }
    }
    if engine.length_mm != length && length > 0.0 {
        let new = engine.length_mm;
        let mut any = false;
        for cg in engine.points.iter_mut().filter_map(|p| p.cg_mm.as_mut()) {
            *cg = *cg * new / length;
            any = true;
        }
        if any {
            changed.push("cg");
        }
    }
    (!changed.is_empty()).then(|| Warning {
        motor: Some(engine.code.clone()),
        line: None,
        kind: WarningKind::Unusual,
        message: format!(
            "rescaled {} to the catalog's figures: they are worked out from the masses and the \
             length",
            changed.join(", ")
        ),
    })
}

/// The input's source, format and text, and for a catalog motor the figures HPR Sim flies: a motor
/// file by its extension, or else a catalog motor.
fn read_input(
    input: &str,
) -> Result<(MotorSource, MotorFile, String, Option<CatalogFigures>), Failure> {
    if let Some(format) = MotorFile::of(input) {
        let bytes =
            std::fs::read(input).map_err(|error| Failure::Input(format!("{input}: {error}")))?;
        let text = String::from_utf8(bytes)
            .map_err(|_| Failure::Input(format!("{input}: not a text file in UTF-8")))?;
        let source = MotorSource::File {
            path: input.to_owned(),
            format: format.output(),
        };
        return Ok((source, format, text, None));
    }
    let catalog = catalog()?;
    let matches: Vec<&CatalogMotor> = catalog.find(input).collect();
    let motor = match matches[..] {
        [motor] => motor,
        [] => {
            return Err(Failure::helped(
                format!(
                    "{input} is neither a .eng or .rse file nor a motor in the bundled catalog"
                ),
                "`hpr motors list` lists the catalog's motors",
            ));
        }
        _ => {
            let names: Vec<&str> = matches.iter().map(|m| m.designation.as_str()).collect();
            return Err(Failure::helped(
                format!(
                    "{input} names {} catalog motors: {}",
                    matches.len(),
                    names.join(", ")
                ),
                "give one's designation",
            ));
        }
    };
    let (curve, text) = motor
        .curves
        .iter()
        .find_map(|curve| bundled_curve_text(&curve.file).map(|text| (curve, text)))
        .ok_or_else(|| {
            Failure::Input(format!("{}: no curve of it is bundled", motor.designation))
        })?;
    let format = match curve.format {
        CurveFormat::Rasp => MotorFile::Eng,
        CurveFormat::RockSim => MotorFile::Rse,
        // `CurveFormat` is non-exhaustive, and the bundled catalog uses these two only.
        _ => {
            return Err(Failure::Input(format!(
                "{}: its curve's format isn't .eng or .rse",
                motor.designation
            )));
        }
    };
    let source = MotorSource::Catalog {
        curve_url: curve.info_url.clone().unwrap_or_else(|| curve.url.clone()),
        format: format.output(),
    };
    let figures = CatalogFigures {
        diameter_mm: motor.diameter_mm,
        length_mm: motor.length_mm,
        propellant_mass_g: motor.propellant_mass_g,
        total_mass_g: motor.total_mass_g,
    };
    Ok((source, format, text.to_owned(), Some(figures)))
}

fn eng_names(file: &EngFile) -> Vec<String> {
    file.entries
        .iter()
        .map(|entry| entry.name.clone())
        .collect()
}

fn rse_names(file: &RseFile) -> Vec<String> {
    file.engines
        .iter()
        .map(|engine| engine.code.clone())
        .collect()
}

fn converted_warning(warning: &ConvertWarning) -> Result<Warning, Failure> {
    Ok(Warning {
        motor: Some(warning.motor.clone()),
        line: None,
        kind: warning_kind(warning.kind)?,
        message: warning.message.clone(),
    })
}

/// `hpr convert` as text: what was read, what was written, and the warnings.
fn text_output(
    document: &ConvertMotors,
    input: &str,
    out: &mut dyn Write,
    paint: Paint,
) -> io::Result<()> {
    let from = match &document.input {
        MotorSource::Catalog { curve_url, .. } => format!("the bundled catalog ({curve_url})"),
        MotorSource::File { .. } => file_name(input),
    };
    writeln!(out, "read   {from}")?;
    writeln!(
        out,
        "wrote  {}: {}",
        file_name(&document.output.path),
        document.motors.join(", ")
    )?;
    for warning in &document.warnings {
        let at = match (&warning.motor, warning.line) {
            (Some(motor), _) => format!("{motor}: "),
            (None, Some(line)) => format!("line {line}: "),
            (None, None) => String::new(),
        };
        paint.line(out, Level::Warning, &format!("{at}{}", warning.message))?;
    }
    Ok(())
}

/// The file name of a path, for the text: its folder depends on where it was run from.
pub(crate) fn file_name(path: &str) -> String {
    Path::new(path).file_name().map_or_else(
        || path.to_owned(),
        |name| name.to_string_lossy().into_owned(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each figure worked out from a replaced mass or length follows it: `massFrac` as
    /// `propWt / initWt`, `Isp` as `1 / propWt`, `m` as `propWt` and `cg` as the length.
    #[test]
    fn rescaled_figures_follow_the_replaced_ones() {
        let text = "<engine-database><engine-list><engine mfg=\"M\" code=\"X1\" dia=\"29\" \
                    len=\"100\" initWt=\"80\" propWt=\"40\" massFrac=\"50\" Isp=\"100\"><data>\
                    <eng-data t=\"0\" f=\"0\" m=\"40\" cg=\"50\"/><eng-data t=\"1\" f=\"20\" \
                    m=\"20\" cg=\"50\"/><eng-data t=\"2\" f=\"0\" m=\"0\" cg=\"50\"/></data>\
                    </engine></engine-list></engine-database>";
        let mut motors = Motors::Rse(rse::parse(text).unwrap().value);
        let figures = CatalogFigures {
            diameter_mm: 29.0,
            length_mm: 110.0,
            propellant_mass_g: Some(50.0),
            total_mass_g: Some(100.0),
        };
        let warnings = figures.apply(&mut motors, true);
        let Motors::Rse(file) = motors else {
            unreachable!("apply keeps the format")
        };
        let engine = &file.engines[0];
        assert_eq!(engine.mass_fraction_pct, Some(50.0));
        assert_eq!(engine.isp_s, Some(80.0));
        let masses: Vec<Option<f64>> = engine.points.iter().map(|p| p.mass_g).collect();
        assert_eq!(masses, [Some(50.0), Some(25.0), Some(0.0)]);
        let cgs: Vec<Option<f64>> = engine.points.iter().map(|p| p.cg_mm).collect();
        assert_eq!(cgs, [Some(55.0); 3]);
        assert_eq!(
            warnings.last().map(|w| w.message.as_str()),
            Some(
                "rescaled massFrac, Isp, m, cg to the catalog's figures: they are worked out \
                 from the masses and the length"
            )
        );
        let messages: Vec<&str> = warnings.iter().map(|w| w.message.as_str()).collect();
        assert!(messages.contains(
            &"its curve file gives a length of 100 mm; the catalog gives 110 mm, which hpr flies \
              and this file takes"
        ));
    }

    /// A header figure that differs from the catalog's warns, and is replaced; one that differs
    /// only by the grams-to-kilograms conversion's last bits is the same figure, replaced with no
    /// warning, up to 4 ε of the catalog's value and no further.
    #[test]
    fn a_header_that_differs_warns_and_a_conversion_s_last_bits_do_not() {
        let header = |propellant_kg: f64| {
            let text = format!("X1 29 100 P {propellant_kg} 0.08 M\n0.5 20\n1.0 0\n;\n");
            Motors::Eng(eng::parse(&text).unwrap().value)
        };
        let figures = |propellant_g: f64| CatalogFigures {
            diameter_mm: 29.0,
            length_mm: 100.0,
            propellant_mass_g: Some(propellant_g),
            total_mass_g: Some(80.0),
        };
        let written = |motors: &Motors| match motors {
            Motors::Eng(file) => file.entries[0].propellant_mass_kg,
            Motors::Rse(_) => unreachable!("apply keeps the format"),
        };
        // 11.1 g against a header's 0.0169 kg: a real disagreement.
        let mut motors = header(0.0169);
        let warnings = figures(11.1).apply(&mut motors, true);
        assert_eq!(
            warnings
                .iter()
                .map(|w| w.message.as_str())
                .collect::<Vec<_>>(),
            [
                "its curve file gives a propellant mass of 0.0169 kg; the catalog gives 0.0111 \
                 kg, which hpr flies and this file takes"
            ]
        );
        assert_eq!(written(&motors), 0.0111);
        // The B4's 0.0036000000000000003 kg against 3.6 g: the same figure.
        let kg = convert::g_to_kg(3.6);
        let past = f64::from_bits(0.0036_f64.to_bits() + 1);
        assert_eq!(past, 0.003_600_000_000_000_000_3);
        let mut motors = header(past);
        assert!(figures(3.6).apply(&mut motors, true).is_empty());
        assert_eq!(written(&motors).to_bits(), kg.to_bits());
        // Past 4 ε of the catalog's value, it warns again.
        let far = kg + 5.0 * f64::EPSILON * kg;
        let mut motors = header(far);
        assert_eq!(figures(3.6).apply(&mut motors, true).len(), 1);
    }
}
