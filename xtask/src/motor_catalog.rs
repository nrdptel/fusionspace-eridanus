//! `cargo xtask motor-catalog`: hpr-motor's bundled catalog index, written from its public-domain
//! curve files alone (issue #295).
//!
//! The 32 curve files under `crates/hpr-motor/data/thrustcurve/curves/` are ThrustCurve.org data
//! files marked public domain. ThrustCurve.org states no terms for its API's motor records, so the
//! index copies none of their figures. For each file the command reads:
//!
//! - from the file's header: diameter, length, propellant and loaded masses, and delays (as
//!   written: `4-6`, `P`, `1000`; [`hpr_motor::DelayList::parse`] reads them);
//! - from the file's curve, with [`hpr_motor::ThrustCurve`]'s own methods, the ones the
//!   simulator flies: total impulse (the trapezoid integral of the samples), peak thrust (the
//!   largest delivered sample), burn time (the NFPA 1125 window between the 5%-of-peak crossings)
//!   and average thrust (total impulse over that burn time), and the impulse class from the total
//!   impulse;
//! - from its bytes: its SHA-256.
//!
//! [`FILES`] holds, by hand, only what a file doesn't state and nothing measured: the names that
//! identify the motor (ThrustCurve.org's id for it, designation, maker, single-use or reload, case,
//! propellant) and the file's provenance (who produced the data, and its license, download date
//! and address, all on the file's ThrustCurve.org page). The common name is the designation's class
//! letter and thrust.
//!
//! Checks that refuse to write: the file holds one motor; its header's sizes and masses are
//! positive with less propellant than loaded mass; the designation's class letter is the computed
//! impulse's class; a reload's case names the header's diameter (the check [Loft lesson L43][l43]
//! asks for: a header that said 75 mm for a 54 mm motor); and every file in the directory is in
//! the table.
//!
//! `--check` writes nothing and fails if `catalog.json` or `src/bundled.rs` is not what the command
//! writes. With ThrustCurve.org's API capture under `refs/` (gitignored, never committed), the
//! command also prints how far the files' figures are from ThrustCurve.org's stated ones.
//!
//! [l43]: https://hpr.fusionspace.co/decisions-and-roadmap.html#l43

use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use hpr_motor::catalog::{
    Catalog, CatalogCurve, CatalogMotor, CurveFormat, CurveLicense, CurveSource, MotorType,
    Snapshot,
};
use hpr_motor::{ImpulseClass, eng, rse};
use sha2::{Digest, Sha256};

pub const USAGE: &str = "\
  motor-catalog [--check]  Write hpr-motor's bundled catalog index
                           (crates/hpr-motor/data/thrustcurve/catalog.json) and
                           src/bundled.rs from the public-domain curve files alone: sizes,
                           masses and delays from each header, impulse, thrusts and burn time
                           from the curve. --check fails if either is stale.";

/// The data directory, from the workspace root.
const DATA: &str = "crates/hpr-motor/data/thrustcurve";

/// The index, from the workspace root.
const CATALOG: &str = "crates/hpr-motor/data/thrustcurve/catalog.json";

/// The module that compiles the index and the files in, from the workspace root.
const BUNDLED: &str = "crates/hpr-motor/src/bundled.rs";

/// The day the files were downloaded (`validation/oracles/thrustcurve/bundle.py`).
const RETRIEVED: &str = "2026-09-17";

/// Where the files were downloaded from.
const FILES_URL: &str = "https://www.thrustcurve.org/simfiles/";

/// The catalog's source.
const SOURCE: &str = "ThrustCurve.org data files marked public domain";

/// How the files were chosen and where the figures come from.
const SELECTION: &str = "Public-domain (license PD) ThrustCurve.org data files of in-production \
solid motors, up to three per impulse class from distinct manufacturers, chosen on 2026-09-17 \
because their curves agreed with ThrustCurve.org's stated total impulse, burn time and average \
thrust within 1%; files whose text contradicts the PD flag are left out. Every figure here is the \
file's own: size, masses and delays from its header; total impulse, average and peak thrust and \
the NFPA 1125 burn time (between the 5%-of-peak crossings) worked out from its curve as hpr flies \
it.";

/// What identifies one bundled file's motor, and the file's provenance: names and dates only.
struct Identity {
    /// ThrustCurve.org's data-file id, the file's name.
    simfile_id: &'static str,
    /// ThrustCurve.org's id for the motor the file is listed under.
    motor_id: &'static str,
    /// The motor's designation.
    designation: &'static str,
    /// The maker's name.
    manufacturer: &'static str,
    /// The maker's short name.
    manufacturer_abbrev: &'static str,
    /// Single-use or reload.
    motor_type: MotorType,
    /// The case a reload fits.
    case: Option<&'static str>,
    /// The propellant's name.
    propellant: &'static str,
    /// Who produced the data.
    source: CurveSource,
}

/// Shorthand for the table.
const fn file(
    (simfile_id, motor_id): (&'static str, &'static str),
    designation: &'static str,
    (manufacturer, manufacturer_abbrev): (&'static str, &'static str),
    motor_type: MotorType,
    case: Option<&'static str>,
    propellant: &'static str,
    source: CurveSource,
) -> Identity {
    Identity {
        simfile_id,
        motor_id,
        designation,
        manufacturer,
        manufacturer_abbrev,
        motor_type,
        case,
        propellant,
        source,
    }
}

const QUEST: (&str, &str) = ("Quest Aerospace", "Quest");
const ESTES: (&str, &str) = ("Estes Industries", "Estes");
const AEROTECH: (&str, &str) = ("AeroTech", "AeroTech");
const CESARONI: (&str, &str) = ("Cesaroni Technology", "Cesaroni");
const LOKI: (&str, &str) = ("Loki Research", "Loki");
const AMW: (&str, &str) = ("Animal Motor Works", "AMW");

use CurveSource::{Certification as CERT, Manufacturer as MFR, User as USER};
use MotorType::{Reload as RELOAD, SingleUse as SU};

/// The bundled files, in catalog order (by impulse class).
#[rustfmt::skip]
const FILES: &[Identity] = &[
    file(("5f4294d20002e9000000088e", "5f4294d2000231000000045c"), "B4", QUEST, SU, None, "Black Max", CERT),
    file(("5f4294d20002e900000008c0", "5f4294d20002310000000014"), "C5", ESTES, SU, None, "black powder", CERT),
    file(("5f923e071bca580004171648", "5f4294d20002310000000376"), "D5", QUEST, SU, None, "black powder", USER),
    file(("60159eecb94d0e00040a8433", "60159d7db94d0e00040a8404"), "E26W", AEROTECH, SU, None, "White Lightning", CERT),
    file(("5f4294d20002e900000007d3", "5f4294d200023100000003cf"), "26E31-15A", CESARONI, RELOAD, Some("Pro24-1G"), "White Thunder", CERT),
    file(("5f5e5a6a1e865c0004c95620", "5f5e57811e865c0004c955d8"), "F52C", AEROTECH, SU, None, "Classic", CERT),
    file(("5f4294d20002e9000000072c", "5f4294d20002310000000392"), "68F240-15A", CESARONI, RELOAD, Some("Pro24-3G"), "Vmax", CERT),
    file(("5f923edb1bca5800041716ab", "5f4294d200023100000003f2"), "F15", ESTES, SU, None, "black powder", USER),
    file(("5f4294d20002e90000000876", "5f4294d200023100000001d5"), "G69N", AEROTECH, RELOAD, Some("RMS-38/120"), "Warp 9", CERT),
    file(("5f4294d20002e90000000724", "5f4294d20002310000000396"), "131G84-10A", CESARONI, RELOAD, Some("Pro24-6G"), "Green3", CERT),
    file(("5f4294d20002e90000000719", "5f4294d20002310000000387"), "H170M", AEROTECH, RELOAD, Some("RMS-38/360"), "Metalstorm", MFR),
    file(("5f4294d20002e90000000735", "5f4294d20002310000000398"), "168H54-10A", CESARONI, RELOAD, Some("Pro29-3G"), "White", CERT),
    file(("5f4294d20002e90000000862", "5f4294d20002310000000447"), "H125-CT", LOKI, RELOAD, Some("38/240"), "Cocktail", CERT),
    file(("5f4294d20002e900000008bf", "5f4294d2000231000000046f"), "I175WS", AEROTECH, SU, None, "Super White Lightning", CERT),
    file(("5f4294d20002e9000000075a", "5f4294d200023100000003a0"), "411I175-14A", CESARONI, RELOAD, Some("Pro38-3G"), "White", CERT),
    file(("5f4294d20002e90000000863", "5f4294d20002310000000448"), "I377-CT", LOKI, RELOAD, Some("38/480"), "Cocktail", CERT),
    file(("5f4294d20002e9000000086b", "5f4294d2000231000000044f"), "J450DM", AEROTECH, SU, None, "Dark Matter", CERT),
    file(("5f4294d20002e9000000074a", "5f4294d200023100000003a8"), "1266J760-19A", CESARONI, RELOAD, Some("Pro54-3G"), "White Thunder", CERT),
    file(("5f4294d20002e90000000823", "5f4294d20002310000000433"), "J300LR", LOKI, RELOAD, Some("54/1200"), "Loki Red", CERT),
    file(("5f4294d20002e9000000086c", "5f4294d20002310000000450"), "K400C", AEROTECH, SU, None, "Classic", CERT),
    file(("5f4294d20002e9000000073d", "5f4294d200023100000003af"), "2245K1075-P", AMW, RELOAD, Some("AMW 54-2550"), "Skidmark", CERT),
    file(("5f4294d20002e90000000749", "5f4294d200023100000003a9"), "1633K940-18A", CESARONI, RELOAD, Some("Pro54-4G"), "White Thunder", CERT),
    file(("5f4294d20002e9000000085f", "5f4294d20002310000000444"), "L2500ST", AEROTECH, RELOAD, Some("RMS-98/5120"), "Super Thunder", CERT),
    file(("5f4294d20002e900000007cc", "5f4294d200023100000003d8"), "3300L3200-P", CESARONI, RELOAD, Some("Pro75-3G"), "Vmax", CERT),
    file(("5f4294d20002e90000000839", "5f4294d2000231000000043d"), "L1040LR", LOKI, RELOAD, Some("54/2800"), "Loki Red", CERT),
    file(("5f4294d20002e90000000875", "5f4294d20002310000000431"), "M1350W", AEROTECH, SU, None, "White Lightning", CERT),
    file(("5f4294d20002e90000000741", "5f4294d200023100000003ad"), "8187M1545-P", CESARONI, RELOAD, Some("Pro75-6GXL"), "Green3", CERT),
    file(("5f4294d20002e90000000867", "5f4294d2000231000000044c"), "M1378LR", LOKI, RELOAD, Some("54/4000"), "Loki Red", CERT),
    file(("5f4294d20002e90000000884", "5f4294d2000231000000030d"), "N3300R", AEROTECH, RELOAD, Some("RMS-98/15360"), "Redline", CERT),
    file(("5f4294d20002e900000007e0", "5f4294d200023100000003f4"), "13628N5600-P", CESARONI, RELOAD, Some("Pro98-6G"), "White Thunder", CERT),
    file(("656e92ee7f1a4b00027f15ad", "656e92d97f1a4b00027f1588"), "O6000W", AEROTECH, SU, None, "White Lightning", CERT),
    file(("5f4294d20002e900000007c0", "5f4294d200023100000003ee"), "21062O3400-P", CESARONI, RELOAD, Some("Pro98-6GXL"), "Imax", CERT),
];

/// Runs `cargo xtask motor-catalog`.
pub fn run(args: &[String]) -> Result<(), String> {
    let check = match args {
        [] => false,
        [flag] if flag == "--check" => true,
        _ => return Err(format!("motor-catalog takes only --check\n\n{USAGE}")),
    };
    let root = crate::designs::root()?;
    let catalog = generate(&root)?;
    let outputs = [
        (CATALOG, catalog_json(&catalog)?),
        (BUNDLED, bundled_rs(&catalog)),
    ];
    let mut stale = Vec::new();
    for (path, text) in &outputs {
        let full = root.join(path);
        let committed = fs::read_to_string(&full).unwrap_or_default();
        if committed != *text {
            if check {
                stale.push(*path);
            } else {
                fs::write(&full, text).map_err(|error| format!("{}: {error}", full.display()))?;
                println!("wrote {path}");
            }
        }
    }
    if !stale.is_empty() {
        return Err(format!(
            "{} not what `cargo xtask motor-catalog` writes; run it",
            stale.join(" and ")
        ));
    }
    println!(
        "{} motors, every figure from its curve file{}",
        catalog.motors.len(),
        if check { "; up to date" } else { "" }
    );
    compare_with_thrustcurve(&root, &catalog)
}

/// The catalog the files give.
pub(crate) fn generate(root: &Path) -> Result<Catalog, String> {
    let curves = root.join(DATA).join("curves");
    let mut on_disk: Vec<String> = fs::read_dir(&curves)
        .map_err(|error| format!("{}: {error}", curves.display()))?
        .map(|entry| {
            entry
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .map_err(|error| error.to_string())
        })
        .collect::<Result<_, _>>()?;
    // A hidden file, such as the `.DS_Store` Finder leaves, is no curve, and is not bundled.
    on_disk.retain(|name| !name.starts_with('.'));
    on_disk.sort();
    let motors = FILES
        .iter()
        .map(|identity| motor(&curves, identity, &on_disk))
        .collect::<Result<Vec<_>, _>>()?;
    let listed: Vec<String> = motors
        .iter()
        .flat_map(|m| &m.curves)
        .map(|c| c.file.trim_start_matches("curves/").to_owned())
        .collect();
    if let Some(extra) = on_disk.iter().find(|name| !listed.contains(name)) {
        return Err(format!(
            "curves/{extra} is not in motor_catalog::FILES: a file is bundled only with its names \
             and provenance recorded"
        ));
    }
    Ok(Catalog {
        source: SOURCE.to_owned(),
        snapshot: Snapshot {
            files_url: FILES_URL.to_owned(),
            captured: RETRIEVED.to_owned(),
        },
        selection: SELECTION.to_owned(),
        motors,
    })
}

/// The header figures of one curve file.
struct Header {
    diameter_mm: f64,
    length_mm: f64,
    delays: Option<String>,
    propellant_mass_g: f64,
    total_mass_g: f64,
}

/// One motor's entry, from its file.
fn motor(curves: &Path, identity: &Identity, on_disk: &[String]) -> Result<CatalogMotor, String> {
    let id = identity.simfile_id;
    let found: Vec<&String> = on_disk
        .iter()
        .filter(|name| name.strip_suffix(".eng").or(name.strip_suffix(".rse")) == Some(id))
        .collect();
    let [name] = found[..] else {
        return Err(format!("{id}: {} curve files, not one", found.len()));
    };
    let path = curves.join(name);
    let text = fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))?;
    let at = |error: hpr_motor::MotorError| format!("{name}: {error}");
    let (format, extension) = if name.ends_with(".eng") {
        (CurveFormat::Rasp, "eng")
    } else {
        (CurveFormat::RockSim, "rse")
    };
    let (thrust, header) = match format {
        CurveFormat::Rasp => {
            let file = eng::parse(&text).map_err(at)?.value;
            let [entry] = &file.entries[..] else {
                return Err(format!("{name}: {} motors, not one", file.entries.len()));
            };
            (
                entry.thrust_curve().map_err(at)?,
                Header {
                    diameter_mm: entry.diameter_mm,
                    length_mm: entry.length_mm,
                    delays: Some(entry.delays.clone()),
                    propellant_mass_g: grams(entry.propellant_mass_kg),
                    total_mass_g: grams(entry.total_mass_kg),
                },
            )
        }
        _ => {
            let file = rse::parse(&text).map_err(at)?.value;
            let [engine] = &file.engines[..] else {
                return Err(format!("{name}: {} motors, not one", file.engines.len()));
            };
            (
                engine.thrust_curve().map_err(at)?,
                Header {
                    diameter_mm: engine.diameter_mm,
                    length_mm: engine.length_mm,
                    delays: engine.delays.clone(),
                    propellant_mass_g: engine.propellant_mass_g,
                    total_mass_g: engine.initial_mass_g,
                },
            )
        }
    };
    check_header(name, identity, &header)?;
    let total_impulse_ns = thrust.total_impulse_ns();
    let impulse_class = ImpulseClass::from_total_impulse(total_impulse_ns).map_err(at)?;
    let common_name = common_name(identity.designation)
        .ok_or_else(|| format!("{name}: no class letter in {}", identity.designation))?;
    if !common_name.starts_with(&impulse_class.label()) {
        return Err(format!(
            "{name}: {} is named a {} but its curve's {total_impulse_ns} N·s is a {impulse_class}",
            identity.designation,
            &common_name[..1]
        ));
    }
    Ok(CatalogMotor {
        motor_id: identity.motor_id.to_owned(),
        manufacturer: identity.manufacturer.to_owned(),
        manufacturer_abbrev: identity.manufacturer_abbrev.to_owned(),
        designation: identity.designation.to_owned(),
        common_name,
        impulse_class,
        motor_type: identity.motor_type,
        case_info: identity.case.map(str::to_owned),
        prop_info: Some(identity.propellant.to_owned()),
        delays: header.delays,
        diameter_mm: header.diameter_mm,
        length_mm: header.length_mm,
        total_impulse_ns,
        average_thrust_n: thrust.average_thrust_n(),
        max_thrust_n: Some(thrust.peak_thrust_n()),
        burn_time_s: thrust.burn_time_s(),
        propellant_mass_g: Some(header.propellant_mass_g),
        total_mass_g: Some(header.total_mass_g),
        curves: vec![CatalogCurve {
            simfile_id: id.to_owned(),
            format,
            source: identity.source,
            // Only files ThrustCurve.org marks public domain are bundled (`bundle.py` checked the
            // flag and each file's text on download); hpr-motor's tests hold every entry to it.
            license: CurveLicense::PublicDomain,
            file: format!("curves/{name}"),
            sha256: Sha256::digest(text.as_bytes())
                .iter()
                .fold(String::new(), |mut hex, byte| {
                    let _ = write!(hex, "{byte:02x}");
                    hex
                }),
            url: format!("{FILES_URL}{id}/download/data.{extension}"),
            info_url: Some(format!("{FILES_URL}{id}/")),
            retrieved: RETRIEVED.to_owned(),
        }],
    })
}

/// A `.eng` header's kilograms as grams, to the microgram: `kg · 1000` leaves binary noise
/// (0.0773 kg is 77.30000000000001 g), far below any header's precision.
fn grams(kg: f64) -> f64 {
    (kg * 1e9).round() / 1e6
}

/// The designation's class letter and average thrust, the common name: `1266J760-19A` is `J760`,
/// `E26W` is `E26`, `H125-CT` is `H125`.
fn common_name(designation: &str) -> Option<String> {
    let rest = designation.trim_start_matches(|c: char| c.is_ascii_digit());
    let mut chars = rest.chars();
    let letter = chars.next().filter(char::is_ascii_uppercase)?;
    let digits: String = chars.take_while(char::is_ascii_digit).collect();
    (!digits.is_empty()).then(|| format!("{letter}{digits}"))
}

/// Refuses a header whose figures can't be a motor, or whose diameter its case contradicts.
fn check_header(name: &str, identity: &Identity, header: &Header) -> Result<(), String> {
    if !(header.diameter_mm > 0.0 && header.length_mm > 0.0) {
        return Err(format!("{name}: the header's size is not positive"));
    }
    if !(header.propellant_mass_g > 0.0 && header.propellant_mass_g < header.total_mass_g) {
        return Err(format!(
            "{name}: the header's propellant {} g and loaded {} g are not a motor",
            header.propellant_mass_g, header.total_mass_g
        ));
    }
    if let Some(case) = identity.case {
        let digits: String = case
            .chars()
            .skip_while(|c| !c.is_ascii_digit())
            .take_while(char::is_ascii_digit)
            .collect();
        let case_mm: f64 = digits
            .parse()
            .map_err(|_| format!("{name}: the case {case} names no diameter"))?;
        if case_mm != header.diameter_mm {
            return Err(format!(
                "{name}: the header's diameter is {} mm, but the case {case} is {case_mm} mm",
                header.diameter_mm
            ));
        }
    }
    Ok(())
}

/// The index as committed: pretty JSON with a final newline.
pub(crate) fn catalog_json(catalog: &Catalog) -> Result<String, String> {
    serde_json::to_string_pretty(catalog)
        .map(|text| text + "\n")
        .map_err(|error| error.to_string())
}

/// `src/bundled.rs`: the index and every file, compiled in.
pub(crate) fn bundled_rs(catalog: &Catalog) -> String {
    let mut files: Vec<&str> = catalog
        .motors
        .iter()
        .flat_map(|m| &m.curves)
        .map(|c| c.file.as_str())
        .collect();
    files.sort_unstable();
    let mut out = String::from(
        "//! The bundled ThrustCurve.org catalog: its index, written by `cargo xtask motor-catalog` \
         from\n//! the curve files alone, and the files. Regenerate it with that command instead \
         of editing it.\n\n\
         /// The catalog index: each motor's names, the figures its curve file gives, and \
         per-curve\n/// provenance.\n\
         pub(crate) const CATALOG_JSON: &str = include_str!(\"../data/thrustcurve/catalog.json\");\n\n\
         /// Each bundled curve file's text, by the path in the index.\n\
         pub(crate) const CURVE_FILES: &[(&str, &str)] = &[\n",
    );
    for file in files {
        let _ = write!(
            out,
            "    (\n        \"{file}\",\n        include_str!(\"../data/thrustcurve/{file}\"),\n    ),\n"
        );
    }
    out.push_str("];\n");
    out
}

/// A figure compared with ThrustCurve.org's: its name, ThrustCurve.org's key for it, the catalog's
/// value, and whether it is held to 1%.
type Field = (
    &'static str,
    &'static str,
    fn(&CatalogMotor) -> Option<f64>,
    bool,
);

/// With ThrustCurve.org's API capture under `refs/` (never committed), prints how far each figure
/// is from ThrustCurve.org's stated one, and refuses a total impulse, average thrust or burn time
/// more than 1% away: the bound the files were chosen by (M1.3).
fn compare_with_thrustcurve(root: &Path, catalog: &Catalog) -> Result<(), String> {
    let path = root.join("refs/snapshots/thrustcurve/motors.json");
    let Ok(text) = fs::read_to_string(&path) else {
        println!("no ThrustCurve.org API capture under refs/; no comparison with its figures");
        return Ok(());
    };
    let capture: serde_json::Value =
        serde_json::from_str(&text).map_err(|error| format!("{}: {error}", path.display()))?;
    let records = capture["results"].as_array().cloned().unwrap_or_default();
    let fields: [Field; 8] = [
        (
            "total impulse",
            "totImpulseNs",
            |m| Some(m.total_impulse_ns),
            true,
        ),
        (
            "average thrust",
            "avgThrustN",
            |m| Some(m.average_thrust_n),
            true,
        ),
        ("burn time", "burnTimeS", |m| Some(m.burn_time_s), true),
        ("peak thrust", "maxThrustN", |m| m.max_thrust_n, false),
        ("diameter", "diameter", |m| Some(m.diameter_mm), false),
        ("length", "length", |m| Some(m.length_mm), false),
        (
            "propellant mass",
            "propWeightG",
            |m| m.propellant_mass_g,
            false,
        ),
        ("loaded mass", "totalWeightG", |m| m.total_mass_g, false),
    ];
    let mut failures = Vec::new();
    println!("against ThrustCurve.org's stated figures (refs/, never committed):");
    for (what, key, ours, bounded) in fields {
        let mut gaps: Vec<(f64, &str)> = Vec::new();
        for motor in &catalog.motors {
            let record = records.iter().find(|r| {
                r["designation"] == motor.designation.as_str()
                    && r["manufacturerAbbrev"] == motor.manufacturer_abbrev.as_str()
            });
            let (Some(theirs), Some(ours)) = (record.and_then(|r| r[key].as_f64()), ours(motor))
            else {
                continue;
            };
            let relative = (ours - theirs) / theirs;
            if bounded && relative.abs() > 0.01 {
                failures.push(format!(
                    "{}: {what} {:+.2}%",
                    motor.designation,
                    100.0 * relative
                ));
            }
            gaps.push((relative, &motor.designation));
        }
        gaps.sort_by(|a, b| a.0.total_cmp(&b.0));
        if let (Some(low), Some(high)) = (gaps.first(), gaps.last()) {
            let differing = gaps.iter().filter(|(gap, _)| gap.abs() > 1e-9).count();
            println!(
                "  {what:<16} {}: {differing:>2} differ, from {:+.3}% ({}) to {:+.3}% ({})",
                gaps.len(),
                100.0 * low.0,
                low.1,
                100.0 * high.0,
                high.1
            );
        }
    }
    if failures.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "more than 1% from ThrustCurve.org's stated figures: {}",
            failures.join(", ")
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The committed index and `bundled.rs` are what the command writes from the files (#295).
    #[test]
    fn committed_motor_catalog_is_what_the_command_writes() {
        let root = crate::designs::root().unwrap();
        let catalog = generate(&root).unwrap();
        let committed = fs::read_to_string(root.join(CATALOG)).unwrap();
        assert!(
            committed == catalog_json(&catalog).unwrap(),
            "{CATALOG} is stale: run `cargo xtask motor-catalog`"
        );
        let committed = fs::read_to_string(root.join(BUNDLED)).unwrap();
        assert!(
            committed == bundled_rs(&catalog),
            "{BUNDLED} is stale: run `cargo xtask motor-catalog`"
        );
        // The index the crate compiles in reads back as the catalog written.
        assert_eq!(hpr_motor::Catalog::bundled().unwrap(), catalog);
    }

    #[test]
    fn common_names_come_from_designations() {
        for (designation, name) in [
            ("1266J760-19A", Some("J760")),
            ("E26W", Some("E26")),
            ("H125-CT", Some("H125")),
            ("13628N5600-P", Some("N5600")),
            ("B4", Some("B4")),
            ("1266-", None),
            ("J", None),
        ] {
            assert_eq!(common_name(designation).as_deref(), name, "{designation}");
        }
    }

    /// Loft lesson L43's header that said 75 mm for a 54 mm motor is refused by its case.
    #[test]
    fn a_header_diameter_its_case_contradicts_is_refused() {
        let identity = &FILES[17];
        assert_eq!(identity.case, Some("Pro54-3G"));
        let header = |diameter_mm| Header {
            diameter_mm,
            length_mm: 329.0,
            delays: None,
            propellant_mass_g: 576.0,
            total_mass_g: 1076.8,
        };
        assert!(check_header("x", identity, &header(54.0)).is_ok());
        let refused = check_header("x", identity, &header(75.0)).unwrap_err();
        assert!(refused.contains("case Pro54-3G is 54 mm"), "{refused}");
        let mut empty = header(54.0);
        empty.propellant_mass_g = empty.total_mass_g;
        assert!(check_header("x", identity, &empty).is_err());
    }

    #[test]
    fn grams_drop_the_conversion_noise() {
        assert_eq!(grams(0.0773), 77.3);
        assert_eq!(grams(0.0036000000000000003), 3.6);
        assert_eq!(grams(2.313309), 2313.309);
    }
}
