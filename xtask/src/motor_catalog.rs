//! `cargo xtask motor-catalog`: hpr-motor's bundled catalog index, written from a snapshot of
//! ThrustCurve.org's whole solid catalog (ADR-288, ADR-301).
//!
//! `validation/oracles/thrustcurve/snapshot.py` takes the snapshot and writes, under
//! `crates/hpr-motor/data/thrustcurve/`, every solid motor's record (`records.json`) and every
//! solid motor's data file as ThrustCurve.org's download answer lists it (`files.json`), both
//! copied unchanged from the API's answers, and each file byte for byte under `curves/` but for a
//! file its uploader restricted. ThrustCurve.org grants no license for its records and reserves
//! copyright; ADR-287 lets the project use them as published facts, with credit, and ADR-288 takes
//! in every file but a restricted one. This command reads those and nothing else.
//!
//! Every record becomes a motor, in the index whether or not a file of it reads. Its labels are
//! the record's, unchanged: maker and its short name, designation, common name, single-use or
//! reload, case and propellant. Its stated figures are the record's: total impulse, average
//! thrust, burn time and peak thrust. Its listed figures are its default file's own (ADR-186), the
//! file ADR-298 §5's ranking puts first: from the file's header, diameter, length, propellant and
//! loaded masses, and delays as written; from its curve, with [`hpr_motor::ThrustCurve`]'s own
//! methods, total impulse, peak thrust, the NFPA 1125 burn time and average thrust, and the
//! impulse class from the total impulse. A motor with no file that reads lists its record's
//! figures and can't fly. Each file's check against its motor's record (ADR-005's 1% rule,
//! [`hpr_motor::catalog::CurveCheck::of`]) is stored with it, failing or not (ADR-298 §2).
//!
//! A file is left out, listed in the index with the reason, when its uploader marked it
//! restricted, its text doesn't parse as one motor's curve, its header's sizes or masses can't be
//! a motor's, or its header's diameter is more than 10% from its record's (the check
//! [Loft lesson L43][l43] asks for: a header that said 75 mm for a 54 mm motor), or the
//! repository holds it back ([`HELD`]). [`NAMED`] names
//! each file this snapshot leaves out, with its reason: a refusal it doesn't name fails the
//! command, and so does a name no file is refused under.
//!
//! The command fails unless the index holds a motor for every record and, for every file the
//! snapshot lists, a curve or a left-out entry. With the snapshot's raw answers under `refs/`
//! (gitignored, never committed), it also fails unless the records, the file list and every curve
//! file's bytes are the answers', and prints the counts it takes from them. `--check` writes
//! nothing and fails if `catalog.json` or `src/bundled.rs` is not what the command writes.
//!
//! The command also fails over 1% ([`checks_hold`], M5.7c): a file the index stores as passing
//! whose total impulse, average thrust or burn time, worked out again from its bytes, is more than
//! 1% from its motor's committed record, or one stored as failing that is within 1% of each. It
//! needs only the committed files, so CI runs it (`motor-catalog --check` in the `validate` job).
//!
//! [l43]: https://hpr.fusionspace.co/decisions-and-roadmap.html#l43

use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use base64::Engine as _;

use crate::motor_flights;
use hpr_motor::catalog::{
    Catalog, CatalogCurve, CatalogMotor, CurveCheck, CurveFormat, CurveLicense, CurveSource,
    LeftOut, MotorType, Snapshot, StatedFigures,
};
use hpr_motor::{ImpulseClass, ThrustCurve, eng, rse};
use serde_json::Value;
use sha2::{Digest, Sha256};

pub const USAGE: &str = "\
  motor-catalog [--check]  Write hpr-motor's bundled catalog index
                           (crates/hpr-motor/data/thrustcurve/catalog.json) and
                           src/bundled.rs from the ThrustCurve.org snapshot's records.json,
                           files.json and curve files: every solid motor, each with its
                           record's labels and stated figures and its default file's own
                           figures, each file's 1% check against the record, and each file left
                           out with its reason. --check fails if either is stale.";

/// The data directory, from the workspace root.
const DATA: &str = "crates/hpr-motor/data/thrustcurve";

/// The index, from the workspace root.
const CATALOG: &str = "crates/hpr-motor/data/thrustcurve/catalog.json";

/// Every solid motor's ThrustCurve.org record, copied unchanged from the snapshot's search answer.
const RECORDS: &str = "crates/hpr-motor/data/thrustcurve/records.json";

/// Every solid motor's data file as the snapshot's download answers list it, without its bytes.
const FILES: &str = "crates/hpr-motor/data/thrustcurve/files.json";

/// Where `snapshot.py` keeps each snapshot's raw answers, under a directory named for its date.
const SNAPSHOTS: &str = "refs/snapshots/thrustcurve";

/// The module that compiles the index and the files in, from the workspace root.
const BUNDLED: &str = "crates/hpr-motor/src/bundled.rs";

/// ThrustCurve.org, which the file list's addresses are relative to.
const SITE: &str = "https://www.thrustcurve.org";

/// Where the files were downloaded from.
const FILES_URL: &str = "https://www.thrustcurve.org/simfiles/";

/// The catalog's source.
const SOURCE: &str = "ThrustCurve.org's solid motor catalog: every solid motor's record and \
every data file but a restricted one";

/// What the index holds and where its figures come from.
const SELECTION: &str = "Every single-use and reload motor in ThrustCurve.org's catalog, in \
production or not, and every data file listed for one but a file its uploader restricted or one \
HPR Sim can't read as one motor's curve, each listed with the reason it is left out. Hybrids are \
left out. Each motor's names and stated figures are ThrustCurve.org's record of it, copied \
unchanged. Its listed figures are its default file's own: size, masses and delays from the file's \
header; total impulse, average and peak thrust and the NFPA 1125 burn time (between the \
5%-of-peak crossings) worked out from its curve as HPR Sim flies it. The default file is one whose \
total impulse, average thrust and burn time are each within 1% of the record's, where one is. A \
motor with no file that reads lists its record's figures and can't fly.";

/// The files the snapshot leaves out, by data-file id, each with the reason the command gives.
const NAMED: &[(&str, &str)] = &[
    // RockSim files whose times run backwards (ADR-288 §1); each motor has a RASP file that reads.
    (
        "5f4294d20002e900000003f0",
        "unreadable: .rse line 10: time 0.08 s is before the previous point's 0.12 s",
    ),
    (
        "5f4294d20002e900000003be",
        "unreadable: .rse line 35: time 0.605 s is before the previous point's 0.61 s",
    ),
    // Loft lesson L43's error: a 75 mm header on a 54 mm reload.
    (
        "5f4294d20002e90000000482",
        "its header's diameter is 75 mm, but its record's is 54 mm",
    ),
    (
        "5f4294d20002e90000000484",
        "its header's diameter is 75 mm, but its record's is 54 mm",
    ),
    // A 38 mm header on a 29 mm single-use motor; the motor has another file that reads.
    (
        "5f4294d20002e90000000396",
        "its header's diameter is 38 mm, but its record's is 29 mm",
    ),
    // A file this repository doesn't hold the bytes of (ADR-301 §5); `snapshot.py`'s `HELD`
    // doesn't write it.
    ("5f4294d20002e900000007de", HELD),
];

/// The reason a file is left out when a check on what this repository commits keeps its bytes
/// out until the maintainer clears them (ADR-301 §5). It is left out without being read, and
/// `snapshot.py` doesn't write it.
const HELD: &str =
    "held back: its bytes aren't in this repository until the maintainer clears them";

/// Runs `cargo xtask motor-catalog`.
pub fn run(args: &[String]) -> Result<(), String> {
    let check = match args {
        [] => false,
        [flag] if flag == "--check" => true,
        _ => return Err(format!("motor-catalog takes only --check\n\n{USAGE}")),
    };
    let root = crate::designs::root()?;
    let catalog = generate(&root)?;
    let expected = match matches_raw_snapshot(&root, &catalog)? {
        Some(expected) => expected,
        None => expected(
            &records(&root)?,
            &files(&root)?.1,
            "committed records and file list's",
        ),
    };
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
    let counts = Counts::of(&catalog);
    println!(
        "{} motors, {} with a file that reads; {} files indexed, {} left out; {} files pass the 1% \
         check, and {} motors have one that does{}",
        counts.motors,
        counts.motors_flying,
        counts.files,
        counts.left_out,
        counts.files_passing,
        counts.motors_passing,
        if check { "; up to date" } else { "" }
    );
    // In --check mode the index the crate compiles in, the one that flies; else the one written.
    let held = if check {
        Catalog::bundled().map_err(|error| error.to_string())?
    } else {
        &catalog
    };
    let passing = checks_hold(&root, held, &records(&root)?)?;
    println!(
        "{passing} files stored as passing are within 1% of their records, worked out again from \
         their bytes, and the rest are not"
    );
    compare_with_thrustcurve(&root, &catalog)?;
    if check {
        motor_flights::check(held, &expected)?;
    }
    Ok(())
}

/// The 1% failure: works out each indexed file's total impulse, average thrust and burn time
/// again from its bytes under `curves/`, with [`ThrustCurve`]'s methods, and fails unless the
/// file is stored as passing exactly when each is within 1% of its motor's committed record,
/// `|file − record| ≤ 0.01 × record` (ADR-298 §4), and the motor's stated figures are the
/// record's. Returns how many files pass.
fn checks_hold(root: &Path, catalog: &Catalog, records: &[Value]) -> Result<usize, String> {
    let mut passing = 0;
    for motor in &catalog.motors {
        let record = records
            .iter()
            .find(|r| r["motorId"] == motor.motor_id.as_str())
            .ok_or_else(|| format!("{}: no committed record", motor.designation))?;
        for (stated, key) in [
            (motor.stated.total_impulse_ns, "totImpulseNs"),
            (motor.stated.average_thrust_n, "avgThrustN"),
            (motor.stated.burn_time_s, "burnTimeS"),
        ] {
            if record[key].as_f64().map(f64::to_bits) != Some(stated.to_bits()) {
                return Err(format!(
                    "{}: its stated {key} {stated} isn't its record's {}",
                    motor.designation, record[key]
                ));
            }
        }
        for curve in &motor.curves {
            let path = root.join(DATA).join(&curve.file);
            let text = fs::read_to_string(&path)
                .map_err(|error| format!("{}: {error}", path.display()))?;
            let thrust = motor
                .thrust_curve(curve, &text)
                .map_err(|error| format!("{}: {error}", curve.file))?;
            let gaps = [
                ("total impulse", thrust.total_impulse_ns(), "totImpulseNs"),
                ("average thrust", thrust.average_thrust_n(), "avgThrustN"),
                ("burn time", thrust.burn_time_s(), "burnTimeS"),
            ]
            .map(|(what, file, key)| {
                let record = record[key].as_f64().unwrap_or(f64::NAN);
                (
                    what,
                    file,
                    record,
                    record > 0.0 && (file - record).abs() <= 0.01 * record,
                )
            });
            let within = gaps.iter().all(|(.., within)| *within);
            if within != curve.check.passes {
                let (what, file, record, _) = gaps
                    .iter()
                    .find(|(.., within)| *within != curve.check.passes)
                    .copied()
                    .unwrap_or(gaps[0]);
                return Err(format!(
                    "{} ({}) is stored as {}, but its {what} {file} is {:+.4}% from its record's \
                     {record}",
                    motor.designation,
                    curve.simfile_id,
                    if curve.check.passes {
                        "passing"
                    } else {
                        "failing"
                    },
                    100.0 * (file - record) / record
                ));
            }
            passing += usize::from(within);
        }
    }
    Ok(passing)
}

/// The motors `hpr motors list` should list, every record's, and those `hpr sim --motor` should
/// fly: each with a RASP or RockSim file whose license allows bundling and that no named refusal
/// leaves out.
fn expected(records: &[Value], files: &[Value], source: &'static str) -> motor_flights::Expected {
    let id = |value: &Value| value["motorId"].as_str().unwrap_or_default().to_owned();
    motor_flights::Expected {
        listed: records.iter().map(id).collect(),
        flown: files
            .iter()
            .filter(|f| {
                f["license"] != "other"
                    && (f["format"] == "RASP" || f["format"] == "RockSim")
                    && !NAMED
                        .iter()
                        .any(|(file, _)| f["simfileId"].as_str() == Some(*file))
            })
            .map(id)
            .collect(),
        source,
    }
}

/// What the index holds, counted.
#[derive(Debug, PartialEq, Eq)]
struct Counts {
    motors: usize,
    motors_flying: usize,
    files: usize,
    left_out: usize,
    files_passing: usize,
    motors_passing: usize,
}

impl Counts {
    fn of(catalog: &Catalog) -> Self {
        let curves = || catalog.motors.iter().flat_map(|m| &m.curves);
        Self {
            motors: catalog.motors.len(),
            motors_flying: catalog
                .motors
                .iter()
                .filter(|m| !m.curves.is_empty())
                .count(),
            files: curves().count(),
            left_out: catalog.left_out.len(),
            files_passing: curves().filter(|c| c.check.passes).count(),
            motors_passing: catalog
                .motors
                .iter()
                .filter(|m| m.curves.iter().any(|c| c.check.passes))
                .count(),
        }
    }
}

/// A JSON file under the workspace root.
fn read_json(root: &Path, path: &str) -> Result<Value, String> {
    let full = root.join(path);
    let text = fs::read_to_string(&full).map_err(|error| format!("{}: {error}", full.display()))?;
    serde_json::from_str(&text).map_err(|error| format!("{}: {error}", full.display()))
}

/// The list under `key` in a committed JSON file.
fn list(root: &Path, path: &str, key: &str) -> Result<Vec<Value>, String> {
    read_json(root, path)?[key]
        .as_array()
        .cloned()
        .ok_or_else(|| format!("{path}: no `{key}` list"))
}

/// The committed records, in their file's order.
fn records(root: &Path) -> Result<Vec<Value>, String> {
    list(root, RECORDS, "records")
}

/// The committed file list, in its file's order, and the snapshot's time.
fn files(root: &Path) -> Result<(String, Vec<Value>), String> {
    let captured = read_json(root, FILES)?["captured"]
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| format!("{FILES}: no `captured` time"))?;
    Ok((captured, list(root, FILES, "files")?))
}

/// A string field, which must be there.
fn text<'a>(value: &'a Value, key: &str, of: &str) -> Result<&'a str, String> {
    value[key]
        .as_str()
        .ok_or_else(|| format!("{of}: no `{key}`"))
}

/// A string field, `None` when it is missing, null or empty.
fn optional(value: &Value, key: &str) -> Option<String> {
    value[key]
        .as_str()
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
}

/// A number field, which must be there.
fn number(value: &Value, key: &str, of: &str) -> Result<f64, String> {
    value[key]
        .as_f64()
        .ok_or_else(|| format!("{of}: no `{key}`"))
}

/// The catalog the snapshot gives.
pub(crate) fn generate(root: &Path) -> Result<Catalog, String> {
    let records = records(root)?;
    let (captured, files) = files(root)?;
    let date = captured
        .get(..10)
        .ok_or_else(|| format!("{FILES}: the snapshot's time {captured} has no date"))?
        .to_owned();
    for file in &files {
        if !records.iter().any(|r| r["motorId"] == file["motorId"]) {
            return Err(format!(
                "{FILES}: {} is listed under {}, which has no record",
                file["simfileId"], file["motorId"]
            ));
        }
    }
    let curves_dir = root.join(DATA).join("curves");
    let mut on_disk: Vec<String> = fs::read_dir(&curves_dir)
        .map_err(|error| format!("{}: {error}", curves_dir.display()))?
        .map(|entry| {
            entry
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .map_err(|error| error.to_string())
        })
        .collect::<Result<_, _>>()?;
    // A hidden file, such as the `.DS_Store` Finder leaves, is no curve, and is not bundled.
    on_disk.retain(|name| !name.starts_with('.'));
    for name in &on_disk {
        let id = name.split('.').next().unwrap_or_default();
        if !files.iter().any(|f| f["simfileId"] == id) {
            return Err(format!(
                "curves/{name} is not in {FILES}: a file is bundled only as the snapshot lists it"
            ));
        }
    }
    let mut motors = Vec::new();
    let mut left_out = Vec::new();
    for record in &records {
        let motor_id = text(record, "motorId", RECORDS)?;
        let mut read = Vec::new();
        for file in files.iter().filter(|f| f["motorId"] == motor_id) {
            let simfile_id = text(file, "simfileId", FILES)?;
            if NAMED.contains(&(simfile_id, HELD)) {
                left_out.push(LeftOut {
                    simfile_id: simfile_id.to_owned(),
                    motor_id: motor_id.to_owned(),
                    reason: HELD.to_owned(),
                });
                continue;
            }
            match curve(&curves_dir, record, file, &date)? {
                Ok(curve) => read.push(curve),
                Err(reason) => left_out.push(LeftOut {
                    simfile_id: simfile_id.to_owned(),
                    motor_id: motor_id.to_owned(),
                    reason,
                }),
            }
        }
        motors.push(motor(record, read)?);
    }
    refusals_are_named(&left_out, NAMED)?;
    motors.sort_by(|a, b| {
        a.stated
            .total_impulse_ns
            .total_cmp(&b.stated.total_impulse_ns)
            .then_with(|| a.manufacturer_abbrev.cmp(&b.manufacturer_abbrev))
            .then_with(|| a.designation.cmp(&b.designation))
            .then_with(|| a.motor_id.cmp(&b.motor_id))
    });
    Ok(Catalog {
        source: SOURCE.to_owned(),
        snapshot: Snapshot {
            files_url: FILES_URL.to_owned(),
            captured: date,
        },
        selection: SELECTION.to_owned(),
        motors,
        left_out,
    })
}

/// Fails on a refusal `named` doesn't hold, reason and all, and on a name no file is refused
/// under.
fn refusals_are_named(left_out: &[LeftOut], named: &[(&str, &str)]) -> Result<(), String> {
    let mut problems = Vec::new();
    for out in left_out {
        if !named
            .iter()
            .any(|(id, reason)| *id == out.simfile_id && *reason == out.reason)
        {
            problems.push(format!(
                "{} ({}) is left out, unnamed: {}",
                out.simfile_id, out.motor_id, out.reason
            ));
        }
    }
    for (id, reason) in named {
        if !left_out
            .iter()
            .any(|out| out.simfile_id == *id && out.reason == *reason)
        {
            problems.push(format!("{id} is named as left out ({reason}), but isn't"));
        }
    }
    if problems.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "the files left out aren't motor_catalog::NAMED's:\n{}",
            problems.join("\n")
        ))
    }
}

/// The header figures of one curve file.
struct Header {
    diameter_mm: f64,
    length_mm: f64,
    delays: Option<String>,
    propellant_mass_g: f64,
    total_mass_g: f64,
}

/// One file the index holds: its entry, its header and its curve.
struct Read {
    curve: CatalogCurve,
    header: Header,
    thrust: ThrustCurve,
}

/// One listed file, read; the inner error is the reason it is left out, the outer one a snapshot
/// the command can't index at all.
fn curve(
    curves: &Path,
    record: &Value,
    file: &Value,
    date: &str,
) -> Result<Result<Read, String>, String> {
    let id = text(file, "simfileId", FILES)?;
    let license: CurveLicense = match file.get("license") {
        None | Some(Value::Null) => CurveLicense::Unknown,
        Some(value) => serde_json::from_value(value.clone())
            .map_err(|error| format!("{FILES}: {id}'s license: {error}"))?,
    };
    let source: CurveSource = serde_json::from_value(file["source"].clone())
        .map_err(|error| format!("{FILES}: {id}'s source: {error}"))?;
    let (format, extension) = match text(file, "format", FILES)? {
        "RASP" => (CurveFormat::Rasp, "eng"),
        "RockSim" => (CurveFormat::RockSim, "rse"),
        other => return Ok(Err(format!("its format, {other}, isn't one HPR Sim reads"))),
    };
    if !license.is_bundleable() {
        return Ok(Err(
            "its uploader restricted it (license \"other\")".to_owned()
        ));
    }
    let name = format!("{id}.{extension}");
    let path = curves.join(&name);
    let text_of_file =
        fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))?;
    let (thrust, header) = match header_and_curve(&text_of_file, format) {
        Ok(read) => read,
        Err(reason) => return Ok(Err(reason)),
    };
    if let Err(reason) = check_header(record["diameter"].as_f64(), &header) {
        return Ok(Err(reason));
    }
    let stated = stated(record)?;
    let check = CurveCheck::of(&thrust, &stated);
    let data_url = text(file, "dataUrl", FILES)?;
    Ok(Ok(Read {
        curve: CatalogCurve {
            simfile_id: id.to_owned(),
            format,
            source,
            license,
            file: format!("curves/{name}"),
            sha256: Sha256::digest(text_of_file.as_bytes()).iter().fold(
                String::new(),
                |mut hex, byte| {
                    let _ = write!(hex, "{byte:02x}");
                    hex
                },
            ),
            url: format!("{SITE}{data_url}"),
            info_url: optional(file, "infoUrl").map(|url| format!("{SITE}{url}")),
            retrieved: date.to_owned(),
            check,
        },
        header,
        thrust,
    }))
}

/// A file's curve and header, or the reason it can't be read as one motor's.
fn header_and_curve(text: &str, format: CurveFormat) -> Result<(ThrustCurve, Header), String> {
    let unreadable = |error: hpr_motor::MotorError| format!("unreadable: {error}");
    match format {
        CurveFormat::Rasp => {
            let file = eng::parse(text).map_err(unreadable)?.value;
            let [entry] = &file.entries[..] else {
                return Err(format!("it holds {} motors, not one", file.entries.len()));
            };
            Ok((
                entry.thrust_curve().map_err(unreadable)?,
                Header {
                    diameter_mm: entry.diameter_mm,
                    length_mm: entry.length_mm,
                    delays: Some(entry.delays.clone()),
                    propellant_mass_g: grams(entry.propellant_mass_kg),
                    total_mass_g: grams(entry.total_mass_kg),
                },
            ))
        }
        _ => {
            let file = rse::parse(text).map_err(unreadable)?.value;
            let [engine] = &file.engines[..] else {
                return Err(format!("it holds {} motors, not one", file.engines.len()));
            };
            Ok((
                engine.thrust_curve().map_err(unreadable)?,
                Header {
                    diameter_mm: engine.diameter_mm,
                    length_mm: engine.length_mm,
                    delays: engine.delays.clone(),
                    propellant_mass_g: engine.propellant_mass_g,
                    total_mass_g: engine.initial_mass_g,
                },
            ))
        }
    }
}

/// The figures a record states.
fn stated(record: &Value) -> Result<StatedFigures, String> {
    let of = format!("{RECORDS}: {}", record["motorId"]);
    Ok(StatedFigures {
        total_impulse_ns: number(record, "totImpulseNs", &of)?,
        average_thrust_n: number(record, "avgThrustN", &of)?,
        burn_time_s: number(record, "burnTimeS", &of)?,
        max_thrust_n: record["maxThrustN"].as_f64(),
        propellant_mass_g: record["propWeightG"].as_f64(),
    })
}

/// One motor's entry: its record's labels and stated figures, its files, and its default file's
/// own figures, or its record's when no file of it reads.
fn motor(record: &Value, read: Vec<Read>) -> Result<CatalogMotor, String> {
    let of = format!("{RECORDS}: {}", record["motorId"]);
    let motor_type: MotorType = serde_json::from_value(record["type"].clone())
        .map_err(|error| format!("{of}'s type: {error}"))?;
    let stated = stated(record)?;
    let (curves, figures): (Vec<CatalogCurve>, Vec<(Header, ThrustCurve)>) = read
        .into_iter()
        .map(|r| (r.curve, (r.header, r.thrust)))
        .unzip();
    let mut motor = CatalogMotor {
        motor_id: text(record, "motorId", &of)?.to_owned(),
        manufacturer: text(record, "manufacturer", &of)?.to_owned(),
        manufacturer_abbrev: text(record, "manufacturerAbbrev", &of)?.to_owned(),
        designation: text(record, "designation", &of)?.to_owned(),
        common_name: text(record, "commonName", &of)?.to_owned(),
        impulse_class: ImpulseClass::from_total_impulse(stated.total_impulse_ns)
            .map_err(|error| format!("{of}: {error}"))?,
        motor_type,
        case_info: optional(record, "caseInfo"),
        prop_info: optional(record, "propInfo"),
        delays: optional(record, "delays"),
        diameter_mm: number(record, "diameter", &of)?,
        length_mm: number(record, "length", &of)?,
        total_impulse_ns: stated.total_impulse_ns,
        average_thrust_n: stated.average_thrust_n,
        max_thrust_n: stated.max_thrust_n,
        burn_time_s: stated.burn_time_s,
        propellant_mass_g: record["propWeightG"].as_f64(),
        total_mass_g: record["totalWeightG"].as_f64(),
        stated,
        curves,
    };
    let default = motor.ranked_curves().first().map(|c| c.simfile_id.clone());
    if let Some(default) = default {
        let at = motor
            .curves
            .iter()
            .position(|c| c.simfile_id == default)
            .ok_or_else(|| format!("{of}: its default file {default} isn't among its curves"))?;
        let (header, thrust) = &figures[at];
        let total_impulse_ns = thrust.total_impulse_ns();
        motor.impulse_class = ImpulseClass::from_total_impulse(total_impulse_ns)
            .map_err(|error| format!("{of}: {default}: {error}"))?;
        motor.delays = header.delays.clone();
        motor.diameter_mm = header.diameter_mm;
        motor.length_mm = header.length_mm;
        motor.total_impulse_ns = total_impulse_ns;
        motor.average_thrust_n = thrust.average_thrust_n();
        motor.max_thrust_n = Some(thrust.peak_thrust_n());
        motor.burn_time_s = thrust.burn_time_s();
        motor.propellant_mass_g = Some(header.propellant_mass_g);
        motor.total_mass_g = Some(header.total_mass_g);
    }
    Ok(motor)
}

/// A `.eng` header's kilograms as grams, to the microgram: `kg · 1000` leaves binary noise
/// (0.0773 kg is 77.30000000000001 g), far below any header's precision.
fn grams(kg: f64) -> f64 {
    (kg * 1e9).round() / 1e6
}

/// How far a file's header diameter may be from its motor's record's, as a ratio of the larger
/// to the smaller less one, and still be that motor's file (ADR-301 §5). On the 2026-10-28
/// snapshot every header sits within 7.3% of its record (four Cesaroni files say 150 mm, the
/// Pro150 case's name, for motors the records give as 161 mm), and the three beyond it are 31% to
/// 39% off: two 75 mm headers on 54 mm reloads, Loft lesson L43's error, and a 38 mm header on a
/// 29 mm single-use motor. Sizes closer than 10% (152 and 161 mm, 6%) aren't told apart.
const DIAMETER_BOUND: f64 = 0.10;

/// The reason a header's figures can't be a motor's, or its diameter contradicts its motor's
/// record's, `record_mm`.
fn check_header(record_mm: Option<f64>, header: &Header) -> Result<(), String> {
    if !(header.diameter_mm > 0.0 && header.length_mm > 0.0) {
        return Err("its header's size is not positive".to_owned());
    }
    if !(header.propellant_mass_g > 0.0 && header.propellant_mass_g < header.total_mass_g) {
        return Err(format!(
            "its header's propellant {} g and loaded {} g are not a motor's",
            header.propellant_mass_g, header.total_mass_g
        ));
    }
    // A record with no diameter has nothing to contradict.
    if let Some(record_mm) = record_mm.filter(|d| *d > 0.0) {
        let header_mm = header.diameter_mm;
        let (low, high) = (header_mm.min(record_mm), header_mm.max(record_mm));
        // As a difference, so a gap of exactly 10% passes: 55 / 50 − 1 rounds above 0.1.
        if high - low > DIAMETER_BOUND * low {
            return Err(format!(
                "its header's diameter is {header_mm} mm, but its record's is {record_mm} mm"
            ));
        }
    }
    Ok(())
}

/// With the snapshot's raw answers under `refs/`, fails unless the committed records, file list
/// and curve files are the answers', and prints the counts taken from them. Whether or not they
/// are there, fails unless the index holds a motor for every committed record and, for every
/// listed file, a curve or a left-out entry.
fn matches_raw_snapshot(
    root: &Path,
    catalog: &Catalog,
) -> Result<Option<motor_flights::Expected>, String> {
    let records = records(root)?;
    let (captured, files) = files(root)?;
    let counts = Counts::of(catalog);
    if counts.motors != records.len() || counts.files + counts.left_out != files.len() {
        return Err(format!(
            "the index holds {} motors and {} files, {} left out, for {} records and {} files",
            counts.motors,
            counts.files,
            counts.left_out,
            records.len(),
            files.len()
        ));
    }
    let directory = root.join(SNAPSHOTS).join(&catalog.snapshot.captured);
    if !directory.is_dir() {
        println!(
            "no raw snapshot under {}; the committed records and files aren't compared with it",
            directory.display()
        );
        return Ok(None);
    }
    let search = read_json(
        root,
        &format!("{SNAPSHOTS}/{}/search.json", catalog.snapshot.captured),
    )?;
    let results = search["results"].as_array().map_or(0, Vec::len);
    if search["matches"].as_u64() != Some(results as u64) {
        return Err(format!(
            "the raw search answer holds {results} results of {} matches",
            search["matches"]
        ));
    }
    let raw_captured = fs::read_to_string(directory.join("CAPTURED")).unwrap_or_default();
    if raw_captured.trim() != captured {
        return Err(format!(
            "{FILES} is the snapshot of {captured}, but {} holds {}",
            directory.display(),
            raw_captured.trim()
        ));
    }
    let mut solid: Vec<Value> = search["results"]
        .as_array()
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter(|r| r["type"] == "SU" || r["type"] == "reload")
        .collect();
    solid.sort_by(|a, b| a["motorId"].as_str().cmp(&b["motorId"].as_str()));
    if solid != records {
        return Err(format!(
            "{RECORDS}'s records aren't the raw search answer's solid motors"
        ));
    }
    let mut listed = Vec::new();
    let mut batch = 0;
    while let Ok(answer) = read_json(
        root,
        &format!(
            "{SNAPSHOTS}/{}/download-{batch:02}.json",
            catalog.snapshot.captured
        ),
    ) {
        for entry in answer["results"].as_array().cloned().unwrap_or_default() {
            let id = text(&entry, "simfileId", "the download answer")?.to_owned();
            if entry["license"] != "other"
                && !NAMED.contains(&(id.as_str(), HELD))
                && (entry["format"] == "RASP" || entry["format"] == "RockSim")
                && let Some(data) = entry["data"].as_str()
            {
                let bytes = base64::engine::general_purpose::STANDARD
                    .decode(data)
                    .map_err(|error| format!("{id}'s bytes in the download answer: {error}"))?;
                let extension = if entry["format"] == "RASP" {
                    "eng"
                } else {
                    "rse"
                };
                let path = root
                    .join(DATA)
                    .join("curves")
                    .join(format!("{id}.{extension}"));
                if fs::read(&path).ok() != Some(bytes) {
                    return Err(format!(
                        "{} isn't the download answer's bytes",
                        path.display()
                    ));
                }
            }
            let mut entry = entry;
            if let Some(object) = entry.as_object_mut() {
                object.remove("data");
            }
            listed.push(entry);
        }
        batch += 1;
    }
    listed.sort_by(|a, b| {
        (a["motorId"].as_str(), a["simfileId"].as_str())
            .cmp(&(b["motorId"].as_str(), b["simfileId"].as_str()))
    });
    if listed != files {
        return Err(format!(
            "{FILES}'s files aren't the raw download answers' ({batch} of them)"
        ));
    }
    for record in &records {
        let downloaded = listed
            .iter()
            .filter(|f| f["motorId"] == record["motorId"])
            .count();
        if Some(downloaded as u64) != record["dataFiles"].as_u64().or(Some(0)) {
            return Err(format!(
                "{}: {downloaded} files downloaded, {} in its record",
                record["motorId"], record["dataFiles"]
            ));
        }
    }
    println!(
        "raw snapshot {captured}: {} solid motors, {} files in {batch} download answers; the \
         records, the file list and every curve file written, all but {} held back, are the \
         answers'",
        solid.len(),
        listed.len(),
        NAMED.iter().filter(|(_, reason)| *reason == HELD).count()
    );
    Ok(Some(expected(&solid, &listed, "raw snapshot's")))
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
         from\n//! the snapshot's records, file list and curve files, and the files. Regenerate it \
         with that command\n//! instead of editing it.\n\n\
         /// The catalog index: each motor's names, the figures its default curve file gives, its \
         record's\n/// stated figures, per-curve provenance and check, and the files left out.\n\
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

/// A figure compared with ThrustCurve.org's: its name, ThrustCurve.org's key for it, and the
/// catalog's value.
type Field = (&'static str, &'static str, fn(&CatalogMotor) -> Option<f64>);

/// Prints how far each listed figure is from the motor's committed ThrustCurve.org record, over
/// the motors with a file that reads. A file more than 1% from its record is stored with its
/// failing check (ADR-298 §2), so nothing here fails.
fn compare_with_thrustcurve(root: &Path, catalog: &Catalog) -> Result<(), String> {
    let records = records(root)?;
    let fields: [Field; 8] = [
        ("total impulse", "totImpulseNs", |m| {
            Some(m.total_impulse_ns)
        }),
        ("average thrust", "avgThrustN", |m| Some(m.average_thrust_n)),
        ("burn time", "burnTimeS", |m| Some(m.burn_time_s)),
        ("peak thrust", "maxThrustN", |m| m.max_thrust_n),
        ("diameter", "diameter", |m| Some(m.diameter_mm)),
        ("length", "length", |m| Some(m.length_mm)),
        ("propellant mass", "propWeightG", |m| m.propellant_mass_g),
        ("loaded mass", "totalWeightG", |m| m.total_mass_g),
    ];
    println!("default files' figures against ThrustCurve.org's records ({RECORDS}):");
    for (what, key, ours) in fields {
        let mut gaps: Vec<(f64, &str)> = Vec::new();
        for motor in catalog.motors.iter().filter(|m| !m.curves.is_empty()) {
            let record = records
                .iter()
                .find(|r| r["motorId"] == motor.motor_id.as_str());
            let (Some(theirs), Some(ours)) = (record.and_then(|r| r[key].as_f64()), ours(motor))
            else {
                continue;
            };
            if theirs == 0.0 {
                continue;
            }
            gaps.push(((ours - theirs) / theirs, &motor.designation));
        }
        gaps.sort_by(|a, b| a.0.total_cmp(&b.0));
        if let (Some(low), Some(high)) = (gaps.first(), gaps.last()) {
            let differing = gaps.iter().filter(|(gap, _)| gap.abs() > 1e-9).count();
            println!(
                "  {what:<16} {}: {differing:>4} differ, from {:+.3}% ({}) to {:+.3}% ({})",
                gaps.len(),
                100.0 * low.0,
                low.1,
                100.0 * high.0,
                high.1
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The committed index and `bundled.rs` are what the command writes from the snapshot (#295).
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
        assert_eq!(hpr_motor::Catalog::bundled().unwrap(), &catalog);
    }

    /// Every committed record is a motor, its labels and stated figures the record's unchanged,
    /// and every listed file is a curve of its motor or left out, counted from the committed
    /// snapshot's records and file list (M5.7b1).
    #[test]
    fn every_record_is_a_motor_and_every_file_is_indexed_or_left_out() {
        let root = crate::designs::root().unwrap();
        let catalog = hpr_motor::Catalog::bundled().unwrap();
        let records = records(&root).unwrap();
        let (_, files) = files(&root).unwrap();
        assert_eq!(catalog.motors.len(), records.len());
        for record in &records {
            let found: Vec<&CatalogMotor> = catalog
                .motors
                .iter()
                .filter(|m| record["motorId"] == m.motor_id.as_str())
                .collect();
            let [motor] = found[..] else {
                panic!("{} motors under {}", found.len(), record["motorId"]);
            };
            assert_eq!(record["manufacturer"], motor.manufacturer.as_str());
            assert_eq!(
                record["manufacturerAbbrev"],
                motor.manufacturer_abbrev.as_str()
            );
            assert_eq!(record["designation"], motor.designation.as_str());
            assert_eq!(record["commonName"], motor.common_name.as_str());
            assert_eq!(
                record["type"],
                serde_json::to_value(motor.motor_type).unwrap()
            );
            assert_eq!(optional(record, "caseInfo"), motor.case_info);
            assert_eq!(optional(record, "propInfo"), motor.prop_info);
            assert_eq!(stated(record).unwrap(), motor.stated);
            if motor.curves.is_empty() {
                // No file reads: the record's own figures, and nothing to fly.
                assert_eq!(record["diameter"].as_f64(), Some(motor.diameter_mm));
                assert_eq!(record["length"].as_f64(), Some(motor.length_mm));
                assert_eq!(motor.total_impulse_ns, motor.stated.total_impulse_ns);
                assert_eq!(record["totalWeightG"].as_f64(), motor.total_mass_g);
                assert!(motor.bundled_motor().is_err());
            }
        }
        for file in &files {
            let id = file["simfileId"].as_str().unwrap();
            let motor = catalog
                .motors
                .iter()
                .find(|m| file["motorId"] == m.motor_id.as_str())
                .unwrap();
            let indexed = motor.curves.iter().filter(|c| c.simfile_id == id).count();
            let out = catalog
                .left_out
                .iter()
                .filter(|o| o.simfile_id == id && file["motorId"] == o.motor_id.as_str())
                .count();
            assert_eq!(indexed + out, 1, "{id}");
        }
        let counts = Counts::of(catalog);
        assert_eq!(counts.files + counts.left_out, files.len());
    }

    /// A refusal the list doesn't name fails, and so does a name with no refusal under it; one
    /// named with another reason is both.
    #[test]
    fn a_refusal_past_the_named_ones_fails() {
        let out = |id: &str, reason: &str| LeftOut {
            simfile_id: id.to_owned(),
            motor_id: "m".to_owned(),
            reason: reason.to_owned(),
        };
        let named = [("a", "unreadable: times run backwards")];
        assert!(refusals_are_named(&[out("a", "unreadable: times run backwards")], &named).is_ok());
        let past = refusals_are_named(
            &[
                out("a", "unreadable: times run backwards"),
                out("b", "it holds 2 motors, not one"),
            ],
            &named,
        )
        .unwrap_err();
        assert!(past.contains("b (m) is left out, unnamed"), "{past}");
        let gone = refusals_are_named(&[], &named).unwrap_err();
        assert!(gone.contains("a is named as left out"), "{gone}");
        let moved = refusals_are_named(&[out("a", "its header's size is not positive")], &named)
            .unwrap_err();
        assert!(
            moved.contains("unnamed") && moved.contains("isn't"),
            "{moved}"
        );
    }

    /// The 1% failure fires both ways (M5.7c): a file stored as passing whose record moves 5% off
    /// it fails by name, and so does a file stored as failing whose record is set to its own
    /// figures; the committed records hold.
    #[test]
    fn a_stored_check_its_record_contradicts_fails() {
        let root = crate::designs::root().unwrap();
        let catalog = hpr_motor::Catalog::bundled().unwrap();
        let committed = records(&root).unwrap();
        assert_eq!(checks_hold(&root, catalog, &committed), Ok(412));
        let (motor, curve) = catalog
            .motors
            .iter()
            .find_map(|m| m.curves.iter().find(|c| c.check.passes).map(|c| (m, c)))
            .unwrap();
        let mut moved = committed.clone();
        let record = moved
            .iter_mut()
            .find(|r| r["motorId"] == motor.motor_id.as_str())
            .unwrap();
        record["burnTimeS"] = Value::from(record["burnTimeS"].as_f64().unwrap() * 1.05);
        let mut catalog_moved = catalog.clone();
        let motor_moved = catalog_moved
            .motors
            .iter_mut()
            .find(|m| m.motor_id == motor.motor_id)
            .unwrap();
        motor_moved.stated.burn_time_s = record["burnTimeS"].as_f64().unwrap();
        let error = checks_hold(&root, &catalog_moved, &moved).unwrap_err();
        assert!(
            error.contains(&curve.simfile_id)
                && error.contains("is stored as passing, but its burn time"),
            "{error}"
        );
        // The stated figures must be the record's, so a record moved alone fails too.
        let error = checks_hold(&root, catalog, &moved).unwrap_err();
        assert!(error.contains("its stated burnTimeS"), "{error}");

        let (motor, curve) = catalog
            .motors
            .iter()
            .find_map(|m| m.curves.iter().find(|c| !c.check.passes).map(|c| (m, c)))
            .unwrap();
        let mut met = committed.clone();
        let record = met
            .iter_mut()
            .find(|r| r["motorId"] == motor.motor_id.as_str())
            .unwrap();
        record["totImpulseNs"] = Value::from(curve.check.total_impulse_ns);
        record["avgThrustN"] = Value::from(curve.check.average_thrust_n);
        record["burnTimeS"] = Value::from(curve.check.burn_time_s);
        let mut catalog_met = catalog.clone();
        let motor_met = catalog_met
            .motors
            .iter_mut()
            .find(|m| m.motor_id == motor.motor_id)
            .unwrap();
        motor_met.stated.total_impulse_ns = curve.check.total_impulse_ns;
        motor_met.stated.average_thrust_n = curve.check.average_thrust_n;
        motor_met.stated.burn_time_s = curve.check.burn_time_s;
        let error = checks_hold(&root, &catalog_met, &met).unwrap_err();
        assert!(
            error.contains(&curve.simfile_id) && error.contains("is stored as failing"),
            "{error}"
        );
    }

    /// A held-back file is the same in both lists, `snapshot.py`'s `HELD` and [`NAMED`], has no
    /// bytes under `curves/`, and is left out in the index with [`HELD`]'s reason.
    #[test]
    fn a_held_file_is_left_out_and_unwritten() {
        let root = crate::designs::root().unwrap();
        let script =
            fs::read_to_string(root.join("validation/oracles/thrustcurve/snapshot.py")).unwrap();
        let line = script.lines().find(|l| l.starts_with("HELD = {")).unwrap();
        let held: Vec<&str> = NAMED
            .iter()
            .filter(|(_, reason)| *reason == HELD)
            .map(|(id, _)| *id)
            .collect();
        assert!(!held.is_empty());
        assert_eq!(line.matches('"').count(), 2 * held.len(), "{line}");
        let catalog = hpr_motor::Catalog::bundled().unwrap();
        for id in held {
            assert!(line.contains(&format!("\"{id}\"")), "{id}");
            for extension in ["eng", "rse"] {
                let path = root.join(DATA).join(format!("curves/{id}.{extension}"));
                assert!(!path.exists(), "{}", path.display());
            }
            assert!(
                catalog
                    .left_out
                    .iter()
                    .any(|o| o.simfile_id == id && o.reason == HELD)
            );
        }
    }

    /// Loft lesson L43's header that said 75 mm for a 54 mm motor is refused by its record, in
    /// either direction, and a header within 10% of its record's diameter passes.
    #[test]
    fn a_header_diameter_its_record_contradicts_is_refused() {
        let header = |diameter_mm| Header {
            diameter_mm,
            length_mm: 329.0,
            delays: None,
            propellant_mass_g: 576.0,
            total_mass_g: 1076.8,
        };
        assert!(check_header(Some(54.0), &header(54.0)).is_ok());
        let refused = check_header(Some(54.0), &header(75.0)).unwrap_err();
        assert!(
            refused.contains("is 75 mm, but its record's is 54 mm"),
            "{refused}"
        );
        // Both ways: a header narrower than its motor is refused too.
        assert!(check_header(Some(75.0), &header(54.0)).is_err());
        assert!(check_header(Some(32.0), &header(29.0)).is_err());
        assert!(check_header(Some(29.0), &header(32.0)).is_err());
        // The Pro150 headers' 150 mm for 161 mm passes, either way round; 10% exactly passes.
        assert!(check_header(Some(161.0), &header(150.0)).is_ok());
        assert!(check_header(Some(150.0), &header(161.0)).is_ok());
        assert!(check_header(Some(50.0), &header(55.0)).is_ok());
        assert!(check_header(Some(50.0), &header(55.001)).is_err());
        // A record with no diameter has nothing to contradict.
        assert!(check_header(None, &header(75.0)).is_ok());
        assert!(check_header(Some(0.0), &header(75.0)).is_ok());
        let mut empty = header(54.0);
        empty.propellant_mass_g = empty.total_mass_g;
        assert!(check_header(None, &empty).is_err());
    }

    #[test]
    fn grams_drop_the_conversion_noise() {
        assert_eq!(grams(0.0773), 77.3);
        assert_eq!(grams(0.0036000000000000003), 3.6);
        assert_eq!(grams(2.313309), 2313.309);
    }
}
