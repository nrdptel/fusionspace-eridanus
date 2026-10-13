//! `cargo xtask logs [--check]`: reads every flight-log sample under `refs/` with
//! `hpr_flightdata::import` and writes what came of each to `validation/reports/log-import.json`
//! and `.md` (M7.1a, M7.1b, M7.1c1 and M7.1c2, ADR-251, ADR-253, ADR-254 and ADR-255).
//!
//! The samples are three sets, each file counted once by its content's SHA-256, in this order:
//!
//! - the private collection's catalog, `refs/hpr-sim-fixtures/catalog/logs.csv`, every row but
//!   those under its unpinned `_scratch/` folder, the family from the catalog's own `format`;
//! - Debrief's public fixtures and samples (`lib/parsers/__fixtures__`, `public/samples`), the
//!   family from [`DEBRIEF`], a table of their names;
//! - AerospaceNU's flight logs under `refs/research/aerospacenu-flights/`, the family from
//!   [`aerospacenu_family`], a table of their first lines.
//!
//! A family never comes from HPR Sim's own recognition of the file, so a reader that fails to
//! recognize a file of its family shows as a failure, not as a file of another family.
//!
//! The report holds counts, and per file an anonymized id (the first 16 hex digits of the file's
//! SHA-256) with the first 16 of the SHA-256 of the record read from it, as compact JSON. That
//! digest pins every reading of a sample that may not be committed, as a snapshot would (rule 4,
//! ADR-151). `--check` writes nothing, and fails unless the report reproduces and every file of
//! a family a reader has been written for ([`READ`]) is read.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use hpr_flightdata::error::LogError;
use hpr_flightdata::log::LogFormat;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const USAGE: &str =
    "  logs [--check]          reads every flight-log sample under refs/ (the collection's
                           catalog, Debrief's fixtures, AerospaceNU's logs) and writes
                           validation/reports/log-import.json and .md; --check writes nothing
                           and fails unless they reproduce and every file of a family
                           with a reader is read";

const REPORT_JSON: &str = "validation/reports/log-import.json";
const REPORT_MD: &str = "validation/reports/log-import.md";
const CATALOG: &str = "refs/hpr-sim-fixtures/catalog/logs.csv";
const COLLECTION: &str = "refs/hpr-sim-fixtures";
const DEBRIEF_DIRS: [&str; 2] = [
    "refs/fusionspace-debrief/lib/parsers/__fixtures__",
    "refs/fusionspace-debrief/public/samples",
];
const AEROSPACENU: &str = "refs/research/aerospacenu-flights";

/// The families read so far, each with the format its files must read as and the increment that
/// wrote its reader: every file of these must import, and in that format. The other families'
/// files are M7.1c6's.
pub const READ: [(&str, LogFormat, &str); 24] = [
    ("pf2", LogFormat::PerfectFlitePf2, "M7.1a"),
    ("perfectflite-csv", LogFormat::PerfectFliteCsv, "M7.1a"),
    ("perfectflite-txt", LogFormat::PerfectFliteText, "M7.1a"),
    ("altos-csv", LogFormat::AltosCsv, "M7.1a"),
    ("raven-csv", LogFormat::RavenFipCsv, "M7.1a"),
    ("rrc3-csv", LogFormat::Rrc3Csv, "M7.1b"),
    ("eggtimer-csv", LogFormat::EggtimerCsv, "M7.1b"),
    ("blueraven-csv", LogFormat::BlueRavenCsv, "M7.1b"),
    ("fwgps-csv", LogFormat::FeatherweightGpsCsv, "M7.1c1"),
    ("kml", LogFormat::Kml, "M7.1c1"),
    ("nmea", LogFormat::Nmea0183, "M7.1c1"),
    ("altimetercloud-csv", LogFormat::AltimeterCloudCsv, "M7.1c2"),
    ("entacore-aim-csv", LogFormat::EntacoreAimCsv, "M7.1c2"),
    ("pfa", LogFormat::PerfectFlitePfa, "M7.1c2"),
    ("eggtimer-txt", LogFormat::EggtimerText, "M7.1c2"),
    (
        "aerospacenu-sillygoose-txt",
        LogFormat::SillyGooseText,
        "M7.1c3",
    ),
    ("fluctus-csv", LogFormat::FluctusCsv, "M7.1c3"),
    ("arts2-csv", LogFormat::Arts2Csv, "M7.1c3"),
    (
        "aerospacenu-fcb-csv",
        LogFormat::AerospaceNuFcbCsv,
        "M7.1c4",
    ),
    (
        "aerospacenu-gs-txt",
        LogFormat::AerospaceNuGroundStationText,
        "M7.1c4",
    ),
    ("phone-launch-csv", LogFormat::PhoneLaunchCsv, "M7.1c4"),
    ("eggtimer-rtf", LogFormat::EggtimerRtf, "M7.1c4"),
    ("raven-fipa", LogFormat::RavenFipa, "M7.1c5"),
    ("rrc3-rff", LogFormat::Rrc3Rff, "M7.1c5"),
];

/// Catalog families that aren't flight logs: pictures and documents filed with a flight, and maps
/// of a flight's prediction.
const NOT_LOGS: [&str; 5] = ["empty", "jpeg", "kml-map", "pdf", "png"];

/// Debrief's fixtures and samples by name, with each one's family in the catalog's words; a file
/// missing from this table stops the survey.
const DEBRIEF: [(&str, &str); 23] = [
    ("aim-xtra.csv", "entacore-aim-csv"),
    ("altimetercloud-mercury.csv", "altimetercloud-csv"),
    ("altusmetrum-telemetrum.csv", "altos-csv"),
    ("blueraven-app-hr.csv", "blueraven-csv"),
    ("blueraven-app-lr.csv", "blueraven-csv"),
    ("blueraven-app.summary.csv", "blueraven-summary-csv"),
    ("featherweight-gps-groundstation.csv", "fwgps-csv"),
    ("featherweight-gps.csv", "fwgps-csv"),
    ("featherweight-raven-fip.csv", "raven-csv"),
    ("perfectflite-pnut.pf2", "pf2"),
    // A team's own CSV of a StratoLogger's flight, `time_s,altitude_ft,…`: generic CSV.
    ("perfectflite-stratologger.csv", "csv"),
    ("sample-spreadsheet.xlsx", "xlsx"),
    ("sample-altusmetrum.csv", "altos-csv"),
    ("sample-blueraven.csv", "blueraven-csv"),
    ("sample-blueraven.summary.csv", "blueraven-summary-csv"),
    // Debrief's six made-up flights, in its own CSV with a `Synthetic` marker row.
    ("sample-design-flight.csv", "debrief-synthetic-csv"),
    ("sample-gps-tracker.csv", "debrief-synthetic-csv"),
    ("sample-mapper.csv", "debrief-synthetic-csv"),
    ("sample-pnut.pf2", "pf2"),
    ("sample-raven-fip.csv", "raven-csv"),
    ("sample-saturated.csv", "debrief-synthetic-csv"),
    ("sample-stage-booster.csv", "debrief-synthetic-csv"),
    ("sample-stage-sustainer.csv", "debrief-synthetic-csv"),
];

/// Debrief files that aren't logs: its notes and a design.
const DEBRIEF_SKIPPED: [&str; 2] = ["README.md", "sample-design.ork"];

/// Why a file the catalog files as generic CSV, by its extension, is AltimeterCloud's.
const ALTIMETERCLOUD: &str = "AltimeterCloud's CSV of a Rocketry Ltd altimeter's flight, a Nano \
    V1's or a Mercury's, filed as generic CSV by its extension";
/// Why a file the catalog files as generic CSV is Entacore AIM's.
const ENTACORE_AIM: &str = "Entacore AIM's CSV, each channel group beside its own time column, \
    filed as generic CSV by its extension";

/// Why a file the catalog files as generic CSV is a Silicdyne Fluctus's.
const FLUCTUS: &str = "Silicdyne Fluctus's CSV, a `sep=;` line then a semicolon-separated table \
    whose names carry their units, filed as generic CSV by its extension";
/// Why a file the catalog files as generic CSV is an ARTS2's interpreted data.
const ARTS2: &str = "Ozark Aerospace ARTS2's interpreted data, a `# MET Acc Vel iAlt pAlt` \
    header then comma-separated rows, filed as generic CSV by its extension";
/// Why a file the catalog files as generic CSV is an ARTS2's raw data, for M7.1c6.
const ARTS2_RAW: &str = "Ozark Aerospace ARTS2's raw data, a `MET,Acc(HI),Acc/Baro(LO),Baro(HI),\
    State` header then the sensors' counts with no calibration in the file, filed as generic CSV \
    by its extension; read with the ARTS2's own `.odf` files";
/// Why a file the catalog files as generic CSV is the phone app's.
const PHONE_LAUNCH: &str = "a phone app's `launch….csv`, a header opening \
    `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension";
/// Why a file the catalog files as RTF is a radio telemetry stream, for M7.1c6.
const TELEMETRY_RTF: &str = "a radio telemetry stream, lines such as `255,T,…` in a layout no \
    maker's document names yet, saved inside an RTF document; filed as RTF, not a log, by its \
    extension";
/// Why a file the catalog files as RTF is an Eggtimer's download.
const EGGTIMER_RTF: &str = "an Eggtimer TRS's terminal download, its settings, summary and \
    `T,Alt,…` table, saved inside an RTF document; filed as RTF, not a log, by its extension";

/// Files whose catalog family isn't their layout, by id: the family each is, and why. An entry
/// the survey doesn't meet stops it, so the table can't outlive its files.
const RECLASSIFIED: [(&str, &str, &str, &str); 73] = [
    (
        "d306250ba73ea7f4",
        "txt",
        "rrc3-csv",
        "an mDACS export of an RRC3's flight, tab-separated and in UTF-16, opening after its \
         UTF-16 byte-order mark with a UTF-8 mark written as three UTF-16 characters; filed as \
         text by its extension",
    ),
    (
        "9c5017ab68168088",
        "altos-csv",
        "csv",
        "a spreadsheet's re-save of an AltOS export, no longer AltosUI's layout: its header \
         edited to carry units, its `height` column deleted, and three columns holding formulas",
    ),
    (
        "76a7d8937258e946",
        "csv",
        "altos-csv-damaged",
        "an AltosUI export whose header line is damaged: it names 25 columns, two of them run \
         together, where its rows hold other counts, so no reader can tell which cell is which; \
         refused",
    ),
    (
        "c7936ac4d86f259f",
        "kml",
        "kml-map",
        "a map of a flight made in a map program, not a logger's file: the tracks a \
         simulation predicted for several launch times, with pins for the pad, the predicted \
         events and where the rocket landed, and a line measuring the distance to it",
    ),
    (
        "1eac3d58c44943fa",
        "csv",
        "altimetercloud-csv",
        ALTIMETERCLOUD,
    ),
    (
        "2d8063ec6a98cac0",
        "csv",
        "altimetercloud-csv",
        ALTIMETERCLOUD,
    ),
    (
        "3c6a8343d5570aa9",
        "csv",
        "altimetercloud-csv",
        ALTIMETERCLOUD,
    ),
    (
        "54f08de4175c5e72",
        "csv",
        "altimetercloud-csv",
        ALTIMETERCLOUD,
    ),
    (
        "6e78a74048da02e5",
        "csv",
        "altimetercloud-csv",
        ALTIMETERCLOUD,
    ),
    (
        "88fcb6bb17c5d9c6",
        "csv",
        "altimetercloud-csv",
        ALTIMETERCLOUD,
    ),
    (
        "8e234dc4a78fcd61",
        "csv",
        "altimetercloud-csv",
        ALTIMETERCLOUD,
    ),
    (
        "d3ac56fff9b7d7c0",
        "csv",
        "altimetercloud-csv",
        ALTIMETERCLOUD,
    ),
    (
        "f021107424bfa9ee",
        "csv",
        "altimetercloud-csv",
        ALTIMETERCLOUD,
    ),
    (
        "f2233f659bd4b671",
        "csv",
        "altimetercloud-csv",
        ALTIMETERCLOUD,
    ),
    ("28a11da1eaf93e69", "csv", "entacore-aim-csv", ENTACORE_AIM),
    ("46d4a9ba5d9b0a58", "csv", "entacore-aim-csv", ENTACORE_AIM),
    ("4d8cf9e8c6f3aa1f", "csv", "entacore-aim-csv", ENTACORE_AIM),
    ("5dc17a47fd185243", "csv", "entacore-aim-csv", ENTACORE_AIM),
    ("5f48e98c9d3b14ee", "csv", "entacore-aim-csv", ENTACORE_AIM),
    ("7890f092f27f7d6f", "csv", "entacore-aim-csv", ENTACORE_AIM),
    ("9b3616ebe985dd0b", "csv", "entacore-aim-csv", ENTACORE_AIM),
    ("9e4dc73ea980b34a", "csv", "entacore-aim-csv", ENTACORE_AIM),
    ("a39eb686e97b5997", "csv", "entacore-aim-csv", ENTACORE_AIM),
    ("a67bdeaf23283bd7", "csv", "entacore-aim-csv", ENTACORE_AIM),
    ("bd3c3e390d38cf68", "csv", "entacore-aim-csv", ENTACORE_AIM),
    ("df76575057cffc1e", "csv", "entacore-aim-csv", ENTACORE_AIM),
    ("f5df1597457a39ea", "csv", "entacore-aim-csv", ENTACORE_AIM),
    ("fe5e01edf5f1202c", "csv", "entacore-aim-csv", ENTACORE_AIM),
    ("ff6326c2236a0723", "csv", "entacore-aim-csv", ENTACORE_AIM),
    ("0bc847e6d3d9a9ee", "csv", "fluctus-csv", FLUCTUS),
    ("5f6d8d9b2a364465", "csv", "fluctus-csv", FLUCTUS),
    ("aad887197af5df0e", "csv", "fluctus-csv", FLUCTUS),
    ("b8bd3d94f2a563d0", "csv", "fluctus-csv", FLUCTUS),
    ("f544929da1867b18", "csv", "fluctus-csv", FLUCTUS),
    ("f5ffeb304d9213e8", "csv", "fluctus-csv", FLUCTUS),
    ("88b16ea55c9a6415", "csv", "arts2-csv", ARTS2),
    ("9559781b880bbec8", "csv", "arts2-csv", ARTS2),
    ("e5f252c299d9dbe5", "csv", "arts2-csv", ARTS2),
    ("f3e56b3adb3d1581", "csv", "arts2-csv", ARTS2),
    ("b4f7a3949e33ca0d", "csv", "arts2-raw-csv", ARTS2_RAW),
    ("c392977fab8558c2", "csv", "arts2-raw-csv", ARTS2_RAW),
    ("eda453d88e88e9c5", "csv", "arts2-raw-csv", ARTS2_RAW),
    ("1a5db9dc7064aff9", "csv", "phone-launch-csv", PHONE_LAUNCH),
    ("1b9914f2b095a2ca", "csv", "phone-launch-csv", PHONE_LAUNCH),
    ("292cd2cbeeb151d1", "csv", "phone-launch-csv", PHONE_LAUNCH),
    ("2eb51f234ada08a3", "csv", "phone-launch-csv", PHONE_LAUNCH),
    ("34ffb9c40a52f8a5", "csv", "phone-launch-csv", PHONE_LAUNCH),
    ("3682e0a8fb151b8f", "csv", "phone-launch-csv", PHONE_LAUNCH),
    ("385526be706eaacb", "csv", "phone-launch-csv", PHONE_LAUNCH),
    ("53116705370ee99f", "csv", "phone-launch-csv", PHONE_LAUNCH),
    ("53d675d508e843fa", "csv", "phone-launch-csv", PHONE_LAUNCH),
    ("5488847e996f7ed6", "csv", "phone-launch-csv", PHONE_LAUNCH),
    ("5e38d29923392275", "csv", "phone-launch-csv", PHONE_LAUNCH),
    ("6d3ba122c9852293", "csv", "phone-launch-csv", PHONE_LAUNCH),
    ("80167395bac22d32", "csv", "phone-launch-csv", PHONE_LAUNCH),
    ("810aa6083d876fff", "csv", "phone-launch-csv", PHONE_LAUNCH),
    ("81693d9be93bd905", "csv", "phone-launch-csv", PHONE_LAUNCH),
    ("836ec61f238d97b3", "csv", "phone-launch-csv", PHONE_LAUNCH),
    ("af9709847c264b99", "csv", "phone-launch-csv", PHONE_LAUNCH),
    ("b1bc4b3a856d71f1", "csv", "phone-launch-csv", PHONE_LAUNCH),
    ("c8405deebb6c61de", "csv", "phone-launch-csv", PHONE_LAUNCH),
    ("cb3c53e2a90c8811", "csv", "phone-launch-csv", PHONE_LAUNCH),
    ("cba087e4c9ac6ef3", "csv", "phone-launch-csv", PHONE_LAUNCH),
    ("cc9cb757ee5ff666", "csv", "phone-launch-csv", PHONE_LAUNCH),
    ("d1da0707d6023367", "csv", "phone-launch-csv", PHONE_LAUNCH),
    ("d516f89cea3600b4", "csv", "phone-launch-csv", PHONE_LAUNCH),
    ("db13cb37b77732af", "csv", "phone-launch-csv", PHONE_LAUNCH),
    ("e556289cc743ecd9", "csv", "phone-launch-csv", PHONE_LAUNCH),
    ("ec774588ec195de6", "csv", "phone-launch-csv", PHONE_LAUNCH),
    ("3a36f081e5be9e9d", "rtf", "telemetry-rtf", TELEMETRY_RTF),
    ("70415b30a59d4411", "rtf", "eggtimer-rtf", EGGTIMER_RTF),
    ("8720c50cdeaf8ad3", "rtf", "telemetry-rtf", TELEMETRY_RTF),
    (
        "88f33383ccf621a0",
        "aerospacenu-sillygoose-txt",
        "aerospacenu-sillygoose-gs-csv",
        "a SillyGoose flight as a ground station's conversion wrote it: a `CONFIG` line, then 28 \
         comma-separated columns with no header, in another order than the board's own log, so \
         nothing in the file says which column is which; for the column mapper",
    ),
];

/// The report.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Report {
    about: String,
    sources: Vec<SourceCount>,
    families: Vec<FamilyRow>,
    /// Files moved from the catalog's family to their layout's: `id from -> to: why`.
    reclassified: Vec<String>,
    /// Files read, by the logger each record names, such as `PerfectFlite SLCF`.
    loggers: BTreeMap<String, usize>,
    /// What bears out feet in PerfectFlite's exports, whose header states no unit.
    perfectflite_exports: ExportUnits,
    pfa_checks: PfaChecks,
    /// The raw downloads against their makers' exports of the same flight, by raw family.
    #[serde(default)]
    raw_checks: BTreeMap<String, RawChecks>,
    /// How the readers decided the units no header states, by family: each decision, the part of
    /// a record's note before its first `: `, such as `` `Altitude` read in feet, from the
    /// pressure ``, with the number of files whose record carries it.
    #[serde(default)]
    unit_decisions: BTreeMap<String, BTreeMap<String, usize>>,
    /// One line a file: `id family source outcome record`, the record `-` when refused.
    files: Vec<String>,
}

/// The evidence that PerfectFlite's exports are in feet: how many peak at exactly the height of
/// a `.pf2` beside them, which is in feet, and the climb's time over `√(2h/g₀)`, a fall in vacuum
/// from the peak `h`, least and greatest: for those, read in feet, and for the others, read in
/// feet and in meters.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ExportUnits {
    files: usize,
    peak_equals_a_pf2_beside_it: usize,
    climb_over_fall_beside_a_pf2_in_feet: [f64; 2],
    climb_over_fall_others_in_feet: [f64; 2],
    climb_over_fall_others_in_meters: [f64; 2],
}

/// What the `.pfa` samples show of the rate and unit their reader takes from PerfectFlite's
/// documents, not the file: how many state as many samples as they hold, and an apogee equal to
/// their highest height; and how many first reach the height that sets off launch detection, 80
/// ft or the least it can be set to, 40 ft, at their eighth sample, 0.7 s, as 10 samples a second
/// from 0.7 s before detection place it.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct PfaChecks {
    files: usize,
    samples_as_stated: usize,
    apogee_is_the_highest: usize,
    detected_at_80_ft: usize,
    detected_at_40_ft: usize,
}

impl PfaChecks {
    fn add(&mut self, log: &hpr_flightdata::log::FlightLog) {
        const FOOT_M: f64 = 0.3048;
        self.files += 1;
        if log.stated.samples == Some(log.altitude_m.len()) {
            self.samples_as_stated += 1;
        }
        let highest = log
            .altitude_m
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);
        if log.stated.apogee_m == Some(highest) {
            self.apogee_is_the_highest += 1;
        }
        if let (Some(before), Some(eighth)) = (log.altitude_m.get(6), log.altitude_m.get(7)) {
            let crosses = |feet: f64| *before < feet * FOOT_M && feet * FOOT_M <= *eighth;
            if crosses(80.0) {
                self.detected_at_80_ft += 1;
            } else if crosses(40.0) {
                self.detected_at_40_ft += 1;
            }
        }
    }
}

/// A raw download's family against its maker's exports: of its files, how many have an export
/// beside them, in the same folder, that their reader reads, and how many of those match one of
/// them in every sample of every channel the two share; how many of the raw files' samples were
/// compared, each once however many of the export's rows show it, and how many of the
/// accelerometer's velocity samples come after the export stops that channel, not compared; the
/// worst difference in time, s, and in value, in steps of the raw format's resolution, over the
/// matching pairs; the export's channels the raw file doesn't hold, such as the export's own
/// conversions, by name with the number of pairs; and the files with an export beside them that
/// none matches, by id.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct RawChecks {
    files: usize,
    with_an_export_beside: usize,
    matching_an_export: usize,
    raw_samples_compared: usize,
    velocity_samples_after_the_export_stops: usize,
    worst_time_s: f64,
    worst_value_steps: f64,
    export_only: BTreeMap<String, usize>,
    differing: Vec<String>,
}

/// The raw families, each with its maker's export's format.
const RAW_EXPORTS: [(&str, LogFormat); 2] = [
    ("raven-fipa", LogFormat::RavenFipCsv),
    ("rrc3-rff", LogFormat::Rrc3Csv),
];

/// Whether an export's time is a raw file's, printed: the FIP's newer exports print a time to six
/// significant figures, so the two agree within half a unit of the sixth, or a microsecond, and
/// the rounding of a single-precision float, `2⁻²⁴` of the time, as a tick exactly halfway, at
/// 10.00875 s, prints 10.0087.
fn times_agree(export: f64, raw: f64) -> bool {
    let printed = if raw == 0.0 {
        0.0
    } else {
        0.5 * 10f64.powf(raw.abs().log10().floor() - 5.0)
    };
    (export - raw).abs() <= printed.max(1e-6) + raw.abs() * 2f64.powi(-24)
}

/// mDACS's clock for an export's first `rows` rows, in hundredths of a second as it prints them:
/// 0.05 s summed row by row in single precision, written to seven significant figures, then
/// rounded half up to two decimals. Its sum drifts from `n / 20` and, by 156.65 s, the printed
/// time runs a hundredth ahead: all 3,541 times of the one mDACS export beside a raw file read
/// so, where `n / 20` misses 408 of them.
fn mdacs_clock(rows: usize) -> Vec<i64> {
    let mut sum = 0.0f32;
    let mut clock = Vec::with_capacity(rows);
    for _ in 0..rows {
        // Seven significant figures: digits `d.dddddd` and an exponent, the value `digits ×
        // 10^(exponent − 6)`; in hundredths, `digits × 10^(exponent − 4)`, rounded half up.
        let written = format!("{:.6e}", f64::from(sum));
        let (mantissa, exponent) = written.split_once('e').unwrap_or((&written, "0"));
        let digits: i64 = mantissa.replace('.', "").parse().unwrap_or(i64::MAX);
        let exponent: i32 = exponent.parse().unwrap_or(0);
        clock.push(if exponent >= 4 {
            digits.saturating_mul(10i64.saturating_pow((exponent - 4).unsigned_abs()))
        } else {
            let divisor = 10i64.saturating_pow((4 - exponent).unsigned_abs());
            (digits + divisor / 2) / divisor
        });
        sum += 0.05f32;
    }
    clock
}

/// The step of a raw format's channel, in SI: its word's resolution, from the readers' docs.
fn raw_step(format: LogFormat, quantity: hpr_flightdata::log::Quantity) -> Option<f64> {
    use hpr_flightdata::log::Quantity as Q;
    const G0: f64 = 9.806_65;
    match (format, quantity) {
        (LogFormat::RavenFipa, Q::Acceleration) => Some(0.01 * G0),
        (LogFormat::RavenFipa, Q::Pressure) => Some(101_325.0 / 25_000.0),
        (LogFormat::RavenFipa, Q::Voltage | Q::Current) => Some(0.001),
        (LogFormat::RavenFipa, Q::Temperature) => Some(0.01 * 5.0 / 9.0),
        (LogFormat::RavenFipa, Q::Speed) => Some(0.1 * 0.3048),
        (LogFormat::Rrc3Rff, Q::Pressure) => Some(10.0),
        _ => None,
    }
}

/// A channel's name for matching: in lower case, with a bracketed unit after it, such as
/// `Pressure [mbar]`, taken off.
fn channel_key(column: &str) -> String {
    let column = column.trim();
    let column = match column.rfind(" [") {
        Some(at) if column.ends_with(']') && at > 0 => &column[..at],
        _ => column,
    };
    column.trim().to_ascii_lowercase()
}

/// What a raw record's comparison with an export found, where every sample compared agrees.
#[derive(Debug, Default, PartialEq)]
struct Compared {
    raw_samples: usize,
    velocity_after_the_stop: usize,
    worst_time_s: f64,
    worst_value_steps: f64,
    export_only: Vec<String>,
}

impl Compared {
    /// A sample's value against the raw one, within half a step; `false` if they differ.
    fn value(&mut self, export: f64, raw: f64, step: f64) -> bool {
        // A gap in the export is a gap in the raw file too, or a difference.
        if export.is_nan() || raw.is_nan() {
            return export.is_nan() && raw.is_nan();
        }
        let steps = (export - raw).abs() / step;
        // A difference is no sample compared, so it doesn't count toward the worst.
        let agrees = steps <= 0.5;
        if agrees {
            self.worst_value_steps = self.worst_value_steps.max(steps);
        }
        agrees
    }

    fn time(&mut self, export: f64, raw: f64) {
        self.worst_time_s = self.worst_time_s.max((export - raw).abs());
    }
}

/// How a raw record compares with an export's: `None` if a sample differs.
///
/// An export channel as long as the raw one lists its samples one for one, each at the raw
/// sample's time as the export prints it (mDACS's by [`mdacs_clock`], the FIP's by
/// [`times_agree`]). An export with one clock for every channel and more rows than the raw
/// channel has samples, a row each tick, holds each sample until the next: each row's value must
/// be the raw channel's latest sample at or before its time, or its first before that, and every
/// raw sample must show at a row of its own time, none skipped. Only the accelerometer's velocity
/// may stop early, as the FIP stops it at apogee, by ending the column or by holding its last
/// value to the end; every other channel must run to the raw file's last sample.
fn compare_raw(
    raw: &hpr_flightdata::log::FlightLog,
    export: &hpr_flightdata::log::FlightLog,
) -> Option<Compared> {
    use hpr_flightdata::log::Quantity;
    // One clock needs two channels to share it.
    let one_clock = export.channels.len() > 1
        && export
            .channels
            .windows(2)
            .all(|pair| pair[0].time_s == pair[1].time_s);
    let mut out = Compared::default();
    let mut compared_channels = 0;
    for channel in &export.channels {
        let key = channel_key(&channel.column);
        let Some(own) = raw
            .channels
            .iter()
            .find(|own| channel_key(&own.column) == key && own.quantity == channel.quantity)
        else {
            out.export_only.push(channel.column.clone());
            continue;
        };
        let step = raw_step(raw.format, own.quantity)?;
        let may_stop = own.quantity == Quantity::Speed;
        let (rows, samples) = (channel.values.len(), own.values.len());
        let reached = if rows == samples || (rows < samples && may_stop && !one_clock) {
            let clock = (raw.format == LogFormat::Rrc3Rff).then(|| mdacs_clock(rows));
            for i in 0..rows {
                let (time, raw_time) = (channel.time_s[i], own.time_s[i]);
                let agrees = match &clock {
                    Some(clock) => (time * 100.0).round() as i64 == clock[i],
                    None => times_agree(time, raw_time),
                };
                out.time(time, raw_time);
                if !agrees || !out.value(channel.values[i], own.values[i], step) {
                    return None;
                }
            }
            rows
        } else if one_clock && rows > samples {
            held(channel, own, may_stop, step, &mut out)?
        } else {
            return None;
        };
        out.raw_samples += reached;
        out.velocity_after_the_stop += samples - reached;
        compared_channels += 1;
    }
    (compared_channels > 0).then_some(out)
}

/// A one-clock export's channel against the raw one ([`compare_raw`]): the raw samples it reaches,
/// or `None` if a value differs, a raw sample shows at no row of its own time, or the channel
/// stops early where it may not.
fn held(
    channel: &hpr_flightdata::log::Channel,
    own: &hpr_flightdata::log::Channel,
    may_stop: bool,
    step: f64,
    out: &mut Compared,
) -> Option<usize> {
    let samples = own.values.len();
    // The raw sample the row shows, and how many raw samples have shown at their own time.
    let (mut j, mut timed) = (0, 0);
    for (i, (&time, &value)) in channel.time_s.iter().zip(&channel.values).enumerate() {
        let from = j;
        while j + 1 < samples && (own.time_s[j + 1] <= time || times_agree(time, own.time_s[j + 1]))
        {
            j += 1;
        }
        if j > from + 1 || timed < j {
            return None;
        }
        if timed == j && times_agree(time, own.time_s[j]) {
            out.time(time, own.time_s[j]);
            timed += 1;
        }
        if !out.value(value, own.values[j], step) {
            // Stopped early: the export holds its previous value to the end.
            let stopped = may_stop
                && i > 0
                && channel.values[i..]
                    .iter()
                    .all(|later| later.to_bits() == channel.values[i - 1].to_bits());
            return stopped.then_some(j);
        }
    }
    (timed == samples).then_some(samples)
}

/// [`RawChecks`] over the samples, each raw family's files against the exports in their folder.
fn raw_checks(samples: &[Sample]) -> BTreeMap<String, RawChecks> {
    let mut checks = BTreeMap::new();
    for (family, export_format) in RAW_EXPORTS {
        let entry: &mut RawChecks = checks.entry(family.to_owned()).or_default();
        for sample in samples.iter().filter(|sample| sample.family == family) {
            let Ok(raw) = hpr_flightdata::import::read(&sample.bytes) else {
                continue;
            };
            entry.files += 1;
            let folder = sample.path.parent();
            let exports: Vec<_> = samples
                .iter()
                .filter(|other| other.path.parent() == folder && other.path != sample.path)
                .filter(|other| hpr_flightdata::import::detect(&other.bytes) == Some(export_format))
                .filter_map(|other| hpr_flightdata::import::read(&other.bytes).ok())
                .collect();
            if exports.is_empty() {
                continue;
            }
            entry.with_an_export_beside += 1;
            match exports.iter().find_map(|export| compare_raw(&raw, export)) {
                Some(compared) => {
                    entry.matching_an_export += 1;
                    entry.raw_samples_compared += compared.raw_samples;
                    entry.velocity_samples_after_the_export_stops +=
                        compared.velocity_after_the_stop;
                    entry.worst_time_s = entry.worst_time_s.max(compared.worst_time_s);
                    entry.worst_value_steps =
                        entry.worst_value_steps.max(compared.worst_value_steps);
                    for column in compared.export_only.into_iter().collect::<BTreeSet<_>>() {
                        *entry.export_only.entry(column).or_default() += 1;
                    }
                }
                None => entry
                    .differing
                    .push(hex(&Sha256::digest(&sample.bytes))[..16].to_owned()),
            }
        }
        entry.differing.sort();
    }
    checks
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct SourceCount {
    source: String,
    /// Files the source lists.
    files: usize,
    /// Of those, files the survey hadn't met in an earlier source.
    distinct: usize,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct FamilyRow {
    family: String,
    /// The increment whose reader reads it (`M7.1a`, `M7.1b`, `M7.1c1`, `M7.1c2`, `M7.1c3`),
    /// `M7.1c6` for one not yet read, `damaged`, or `not a log`.
    increment: String,
    files: usize,
    read: usize,
    /// Refusals by kind.
    refused: BTreeMap<String, usize>,
}

/// One distinct file.
struct Sample {
    path: PathBuf,
    source: &'static str,
    family: String,
    bytes: Vec<u8>,
}

pub fn run(args: &[String]) -> Result<(), String> {
    let mut check = false;
    for arg in args {
        match arg.as_str() {
            "--check" => check = true,
            other => return Err(format!("unknown argument `{other}`\n\n{USAGE}")),
        }
    }
    let root = crate::designs::root()?;
    let (samples, sources) = samples(&root)?;
    let (report, unread) = survey(&samples, sources);
    let json = serde_json::to_string_pretty(&report).map_err(|error| error.to_string())? + "\n";
    let markdown = markdown(&report);
    if check {
        for (path, text) in [(REPORT_JSON, &json), (REPORT_MD, &markdown)] {
            let committed = fs::read_to_string(root.join(path))
                .map_err(|error| format!("reading {path}: {error}"))?;
            if committed != *text {
                return Err(format!("{path} does not reproduce; run `cargo xtask logs`"));
            }
        }
    } else {
        for (path, text) in [(REPORT_JSON, json), (REPORT_MD, markdown)] {
            fs::write(root.join(path), text).map_err(|error| format!("writing {path}: {error}"))?;
        }
        println!("logs: wrote {REPORT_JSON} and {REPORT_MD}");
    }
    for family in &report.families {
        println!(
            "logs: {:<26} {:<9} {:>4} files, {:>4} read",
            family.family, family.increment, family.files, family.read
        );
    }
    if !unread.is_empty() {
        return Err(format!(
            "{} file(s) of a family with a reader not read:\n{}",
            unread.len(),
            unread.join("\n")
        ));
    }
    if check {
        println!(
            "logs: the committed report reproduces, and every file of a family with a reader is \
             read"
        );
    }
    Ok(())
}

/// Every distinct sample, and each source's counts.
fn samples(root: &Path) -> Result<(Vec<Sample>, Vec<SourceCount>), String> {
    let mut seen = BTreeSet::new();
    let mut met = BTreeSet::new();
    let mut samples = Vec::new();
    let mut sources = Vec::new();
    let mut add = |listed: Vec<(PathBuf, String)>, source: &'static str| -> Result<(), String> {
        let files = listed.len();
        let mut distinct = 0;
        for (path, family) in listed {
            let bytes =
                fs::read(&path).map_err(|error| format!("reading {}: {error}", path.display()))?;
            let hash = hex(&Sha256::digest(&bytes));
            if seen.insert(hash.clone()) {
                distinct += 1;
                let mut family = family;
                if let Some((_, from, to, _)) =
                    RECLASSIFIED.iter().find(|(id, ..)| hash.starts_with(id))
                {
                    if family != *from {
                        return Err(format!(
                            "{} is listed as reclassified from {from}, but its family is {family}",
                            &hash[..16]
                        ));
                    }
                    family = (*to).to_owned();
                    met.insert(hash[..16].to_owned());
                }
                samples.push(Sample {
                    path,
                    source,
                    family,
                    bytes,
                });
            }
        }
        sources.push(SourceCount {
            source: source.to_owned(),
            files,
            distinct,
        });
        Ok(())
    };
    add(collection(root)?, "collection")?;
    add(debrief(root)?, "debrief")?;
    add(aerospacenu(root)?, "aerospacenu")?;
    for (id, ..) in RECLASSIFIED {
        if !met.contains(id) {
            return Err(format!(
                "the reclassified file {id} isn't among the samples"
            ));
        }
    }
    Ok((samples, sources))
}

/// The collection's catalog rows, but its scratch folder's, each with its family.
fn collection(root: &Path) -> Result<Vec<(PathBuf, String)>, String> {
    let text = fs::read_to_string(root.join(CATALOG))
        .map_err(|error| format!("reading {CATALOG}: {error} (the survey needs refs/)"))?;
    let mut records = csv_records(&text).into_iter();
    let header = records.next().ok_or(format!("{CATALOG} is empty"))?;
    let column = |name: &str| {
        header
            .iter()
            .position(|cell| cell == name)
            .ok_or(format!("{CATALOG} has no `{name}` column"))
    };
    let (path, format) = (column("path")?, column("format")?);
    let mut listed = Vec::new();
    for record in records {
        let (Some(file), Some(family)) = (record.get(path), record.get(format)) else {
            return Err(format!("{CATALOG}: a row has too few cells"));
        };
        if file.starts_with("_scratch/") {
            continue;
        }
        listed.push((root.join(COLLECTION).join(file), family.clone()));
    }
    Ok(listed)
}

/// Debrief's fixtures and samples, each family from [`DEBRIEF`].
fn debrief(root: &Path) -> Result<Vec<(PathBuf, String)>, String> {
    let mut listed = Vec::new();
    for dir in DEBRIEF_DIRS {
        for path in sorted_files(&root.join(dir))? {
            let name = file_name(&path);
            if DEBRIEF_SKIPPED.contains(&name.as_str()) {
                continue;
            }
            let family = DEBRIEF
                .iter()
                .find(|(file, _)| *file == name)
                .map(|(_, family)| (*family).to_owned())
                .ok_or(format!(
                    "{dir}/{name} isn't in the survey's table of Debrief's files"
                ))?;
            listed.push((path, family));
        }
    }
    Ok(listed)
}

/// AerospaceNU's logs: every `.csv`, `.txt`, `.pf2` and `.eeprom` but the per-flight
/// `attributes.csv`, each family from [`aerospacenu_family`].
fn aerospacenu(root: &Path) -> Result<Vec<(PathBuf, String)>, String> {
    let mut listed = Vec::new();
    let mut stack = vec![root.join(AEROSPACENU)];
    let mut files = Vec::new();
    while let Some(dir) = stack.pop() {
        for path in sorted_entries(&dir)? {
            if path.is_dir() {
                stack.push(path);
            } else {
                files.push(path);
            }
        }
    }
    files.sort();
    for path in files {
        let name = file_name(&path);
        let extension = name.rsplit('.').next().unwrap_or("").to_ascii_lowercase();
        if name == "attributes.csv"
            || !["csv", "txt", "pf2", "eeprom"].contains(&extension.as_str())
        {
            continue;
        }
        let bytes =
            fs::read(&path).map_err(|error| format!("reading {}: {error}", path.display()))?;
        listed.push((path, aerospacenu_family(&extension, &bytes).to_owned()));
    }
    Ok(listed)
}

/// An AerospaceNU log's family, from its extension and its first line that isn't blank, in the
/// catalog's words where the catalog has one (`docs/research/aerospacenu-flights.md` describes
/// each).
fn aerospacenu_family(extension: &str, bytes: &[u8]) -> &'static str {
    let text = String::from_utf8_lossy(bytes);
    let first = text
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .unwrap_or("");
    if extension == "eeprom" {
        "altos-eeprom"
    } else if extension == "pf2" || first.to_ascii_lowercase().starts_with("perfectflite") {
        "pf2"
    } else if first.starts_with("#version,") {
        "altos-csv"
    } else if first.starts_with("RUN START") {
        "aerospacenu-gs-txt"
    } else if hpr_flightdata::aerospacenu_fcb::recognizes(&text) {
        // The board's own reader's test: an empty first name, then `packetType`, `timestamp_ms`
        // or `timestamp_s`.
        "aerospacenu-fcb-csv"
    } else if first.starts_with("timestampMs") || first.starts_with("CONFIG") {
        "aerospacenu-sillygoose-txt"
    } else if extension == "csv" {
        "csv"
    } else {
        "txt"
    }
}

/// Reads every sample; the report, and the files of a family with a reader that weren't read.
fn survey(samples: &[Sample], sources: Vec<SourceCount>) -> (Report, Vec<String>) {
    let mut families: BTreeMap<String, FamilyRow> = BTreeMap::new();
    let mut files = Vec::new();
    let mut unread = Vec::new();
    let mut loggers: BTreeMap<String, usize> = BTreeMap::new();
    let mut unit_decisions: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
    let mut pfa_checks = PfaChecks::default();
    for sample in samples {
        let id = hex(&Sha256::digest(&sample.bytes))[..16].to_owned();
        let read_by = READ.iter().find(|(family, _, _)| *family == sample.family);
        let expected = read_by.map(|(_, format, _)| *format);
        let increment = if let Some((_, _, increment)) = read_by {
            increment
        } else if sample.family.ends_with("-damaged") {
            "damaged"
        } else if NOT_LOGS.contains(&sample.family.as_str()) {
            "not a log"
        } else {
            "M7.1c6"
        };
        let row = families
            .entry(sample.family.clone())
            .or_insert_with(|| FamilyRow {
                family: sample.family.clone(),
                increment: increment.to_owned(),
                files: 0,
                read: 0,
                refused: BTreeMap::new(),
            });
        row.files += 1;
        let (outcome, record) = match hpr_flightdata::import::read(&sample.bytes) {
            Ok(log) => {
                row.read += 1;
                if expected.is_some_and(|format| format != log.format) || increment == "damaged" {
                    unread.push(format!(
                        "  {} ({}, {id}): read as {}, where its family {}",
                        sample.path.display(),
                        sample.family,
                        log.format.name(),
                        match expected {
                            Some(format) => format!("reads as {}", format.name()),
                            None => "is listed as damaged, so the list is stale".to_owned(),
                        }
                    ));
                }
                *loggers.entry(log.logger.clone()).or_default() += 1;
                if log.format == LogFormat::PerfectFlitePfa {
                    pfa_checks.add(&log);
                }
                for decision in log.notes.iter().filter_map(|note| unit_decision(note)) {
                    *unit_decisions
                        .entry(sample.family.clone())
                        .or_default()
                        .entry(decision.to_owned())
                        .or_default() += 1;
                }
                // serde_json writes a NaN as null, and the record holds nothing unordered, so
                // the same file reads to the same bytes.
                let record = match serde_json::to_vec(&log) {
                    Ok(json) => hex(&Sha256::digest(&json))[..16].to_owned(),
                    Err(_) => "unwritable".to_owned(),
                };
                (format!("{:?}", log.format), record)
            }
            Err(error) => {
                let kind = refusal(&error);
                *row.refused.entry(kind.to_owned()).or_default() += 1;
                if expected.is_some() {
                    unread.push(format!(
                        "  {} ({}, {}): {error}",
                        sample.path.display(),
                        sample.family,
                        id
                    ));
                }
                (format!("refused:{kind}"), "-".to_owned())
            }
        };
        files.push(format!(
            "{id} {} {} {outcome} {record}",
            sample.family, sample.source
        ));
    }
    files.sort();
    let report = Report {
        about: "Every flight-log sample under refs/, read by hpr_flightdata::import (M7.1, \
                ADR-251, ADR-253, ADR-254, ADR-255): `cargo xtask logs`. The samples are the collection's catalog, Debrief's \
                fixtures and samples, and AerospaceNU's logs; refs/research/public-flights, a \
                download cache whose flights' logs the collection catalogs, and RocketPy's \
                processed flight tables are not loggers' files and are left out. A file's id is the first 16 hex digits of its \
                SHA-256, its record the first 16 of the SHA-256 of the JSON record read from it."
            .to_owned(),
        sources,
        families: families.into_values().collect(),
        perfectflite_exports: export_units(samples),
        pfa_checks,
        raw_checks: raw_checks(samples),
        reclassified: RECLASSIFIED
            .iter()
            .map(|(id, from, to, why)| format!("{id} {from} -> {to}: {why}"))
            .collect(),
        loggers,
        unit_decisions,
        files,
    };
    // A raw file must match an export beside it wherever one is: a decoding that differs from
    // the maker's own is as unread as a refusal.
    for (family, checks) in &report.raw_checks {
        for id in &checks.differing {
            unread.push(format!(
                "  {id} ({family}): differs from every export of its maker's beside it"
            ));
        }
    }
    (report, unread)
}

/// [`ExportUnits`] over the samples. A climb runs from the first sample above 2% of the peak to
/// the peak.
fn export_units(samples: &[Sample]) -> ExportUnits {
    const G0: f64 = 9.806_65;
    let peak = |log: &hpr_flightdata::log::FlightLog| {
        log.altitude_m
            .iter()
            .copied()
            .filter(|value| value.is_finite())
            .fold(f64::NEG_INFINITY, f64::max)
    };
    let mut files = 0;
    let mut matched = 0;
    let (mut matched_feet, mut feet, mut meters) = (Vec::new(), Vec::new(), Vec::new());
    for sample in samples {
        if !matches!(
            sample.family.as_str(),
            "perfectflite-csv" | "perfectflite-txt"
        ) {
            continue;
        }
        let Ok(log) = hpr_flightdata::import::read(&sample.bytes) else {
            continue;
        };
        files += 1;
        let top = peak(&log);
        let beside = sample
            .path
            .parent()
            .and_then(|dir| fs::read_dir(dir).ok())
            .into_iter()
            .flatten()
            .filter_map(|entry| entry.ok().map(|entry| entry.path()))
            .filter(|path| {
                path.extension()
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("pf2"))
            })
            .filter_map(|path| fs::read(path).ok())
            .filter_map(|bytes| hpr_flightdata::import::read(&bytes).ok())
            .any(|pf2| (peak(&pf2) - top).abs() < 1e-9);
        if beside {
            matched += 1;
        }
        let (Some(peak_at), Some(lift_at)) = (
            log.altitude_m.iter().position(|value| *value == top),
            log.altitude_m.iter().position(|value| *value > 0.02 * top),
        ) else {
            continue;
        };
        let climb = log.time_s[peak_at] - log.time_s[lift_at];
        let in_feet = climb / (2.0 * top / G0).sqrt();
        if beside {
            matched_feet.push(in_feet);
        } else {
            feet.push(in_feet);
            meters.push(climb / (2.0 * top / hpr_flightdata::perfectflite::FOOT_M / G0).sqrt());
        }
    }
    let range = |values: &[f64]| {
        let round = |value: f64| (value * 100.0).round() / 100.0;
        [
            round(values.iter().copied().fold(f64::INFINITY, f64::min)),
            round(values.iter().copied().fold(f64::NEG_INFINITY, f64::max)),
        ]
    };
    ExportUnits {
        files,
        peak_equals_a_pf2_beside_it: matched,
        climb_over_fall_beside_a_pf2_in_feet: range(&matched_feet),
        climb_over_fall_others_in_feet: range(&feet),
        climb_over_fall_others_in_meters: range(&meters),
    }
}

/// A refusal's kind, never its message, which can quote the file.
fn refusal(error: &LogError) -> &'static str {
    match error {
        LogError::Unrecognized { .. } => "unrecognized",
        LogError::NotThisFormat { .. } => "not-this-format",
        LogError::Syntax { .. } => "syntax",
        LogError::NoData { .. } => "no-data",
        LogError::Unit { .. } => "unit",
        LogError::Binary { .. } => "binary",
        _ => "other",
    }
}

/// The report as a page: the sources, then each family's counts.
fn markdown(report: &Report) -> String {
    let mut out = String::from(
        "# Flight-log samples, read\n\n\
         Written by `cargo xtask logs` from `log-import.json`; don't edit. Every flight-log \
         sample under `refs/` read by HPR Sim's importer (M7.1, ADR-251, ADR-253, ADR-254, ADR-255): the private \
         collection's catalog, Debrief's fixtures and samples, and AerospaceNU's logs, each file \
         once. Left out: `refs/research/public-flights`, a download cache whose flights' logs \
         the collection catalogs, and RocketPy's processed flight tables, which no logger \
         wrote. A family is the catalog's name for a layout; *unrecognized* means no reader \
         knows the layout yet, and *damaged* a file of a known layout no reader can make sense \
         of.\n\n\
         | Source | Files | Not met before |\n|---|---:|---:|\n",
    );
    for source in &report.sources {
        out += &format!(
            "| {} | {} | {} |\n",
            source.source, source.files, source.distinct
        );
    }
    out += "\n| Family | Increment | Files | Read | Refused |\n|---|---|---:|---:|---|\n";
    let mut total = (0, 0);
    for family in &report.families {
        let refused: Vec<String> = family
            .refused
            .iter()
            .map(|(kind, count)| format!("{count} {kind}"))
            .collect();
        out += &format!(
            "| {} | {} | {} | {} | {} |\n",
            family.family,
            family.increment,
            family.files,
            family.read,
            if refused.is_empty() {
                "none".to_owned()
            } else {
                refused.join(", ")
            }
        );
        total.0 += family.files;
        total.1 += family.read;
    }
    out += &format!("| **All** | | {} | {} | |\n", total.0, total.1);
    if !report.reclassified.is_empty() {
        out += "\nFiles whose catalog family isn't their layout, moved to their layout's:\n\n";
        for line in &report.reclassified {
            out += &format!("- `{}\n", line.replacen(": ", "`: ", 1));
        }
    }
    let units = &report.perfectflite_exports;
    let ratio = |pair: [f64; 2]| format!("{:.2} to {:.2}", pair[0], pair[1]);
    out += &format!(
        "\nPerfectFlite's exports state no unit for their altitude. Of the {} read, {} peak at \
         exactly the height of a `.pf2` beside them, which is in feet; read in feet, their climbs \
         take {} times as long as a fall in vacuum from the peak. The other {} take {} times, read \
         in feet, and would take {} times read in meters.\n",
        units.files,
        units.peak_equals_a_pf2_beside_it,
        ratio(units.climb_over_fall_beside_a_pf2_in_feet),
        units.files - units.peak_equals_a_pf2_beside_it,
        ratio(units.climb_over_fall_others_in_feet),
        ratio(units.climb_over_fall_others_in_meters)
    );
    let pfa = &report.pfa_checks;
    out += &format!(
        "\nA `.pfa` states neither its rate nor its unit; its reader takes 10 samples a second, in \
         feet, from PerfectFlite's documents. Of the {} read, {} hold as many heights as \
         `NumSamps:` states and {} state an `Apogee:` equal to their highest; {} first reach 80 ft, \
         the height that sets off launch detection, at their eighth sample, 0.7 s, and {} first \
         reach 40 ft, the least it can be set to, there.\n",
        pfa.files,
        pfa.samples_as_stated,
        pfa.apogee_is_the_highest,
        pfa.detected_at_80_ft,
        pfa.detected_at_40_ft
    );
    if !report.raw_checks.is_empty() {
        out += "\nThe raw downloads against their makers' exports of the same flight, an export \
                being one beside the raw file, in its folder, that its reader reads. A match agrees \
                in every sample of every channel the two share: the value within half a step of \
                the raw format's resolution, the time as the export prints it (the FIP's to six \
                significant figures; mDACS's from its own clock, 0.05 s summed in single \
                precision, which runs a hundredth of a second ahead of the RRC3's by 157 s). Each \
                raw sample is counted once; only the accelerometer's velocity may stop early, as \
                the FIP stops it at apogee, and its samples after the stop are counted apart:\n\n\
                | Family | Files | With an export | Matching one | Raw samples compared | \
                Velocity after the stop | Worst time, s | Worst value, steps | Export only | \
                Differing |\n\
                |---|---:|---:|---:|---:|---:|---:|---:|---|---|\n";
        for (family, checks) in &report.raw_checks {
            let only: Vec<String> = checks
                .export_only
                .iter()
                .map(|(column, pairs)| format!("`{column}` ({pairs})"))
                .collect();
            out += &format!(
                "| {family} | {} | {} | {} | {} | {} | {:.1e} | {:.3} | {} | {} |\n",
                checks.files,
                checks.with_an_export_beside,
                checks.matching_an_export,
                checks.raw_samples_compared,
                checks.velocity_samples_after_the_export_stops,
                checks.worst_time_s,
                checks.worst_value_steps,
                if only.is_empty() {
                    "none".to_owned()
                } else {
                    only.join(", ")
                },
                if checks.differing.is_empty() {
                    "none".to_owned()
                } else {
                    checks.differing.join(", ")
                }
            );
        }
    }
    if !report.unit_decisions.is_empty() {
        out += "\nUnits no header states, as each file's reader decided them, or kept the column \
                as written, and noted:\n\n\
                | Family | Decision | Files |\n|---|---|---:|\n";
        for (family, decisions) in &report.unit_decisions {
            for (decision, count) in decisions {
                out += &format!("| {family} | {decision} | {count} |\n");
            }
        }
    }
    out += "\nThe files read, by the logger each names:\n\n| Logger | Files |\n|---|---:|\n";
    for (logger, count) in &report.loggers {
        out += &format!("| {logger} | {count} |\n");
    }
    out
}

/// A record's note on a unit no header states, cut to its decision: the text before the first
/// `: ` of a note that opens with a column's name in backticks and says how it was `read in`, or
/// that it was `kept as written`, as the readers write them, such as `` `Velocity` read in miles
/// per hour, from the altitude``.
fn unit_decision(note: &str) -> Option<&str> {
    let (decision, _) = note.split_once(": ")?;
    (decision.starts_with('`')
        && (decision.contains("` read in ") || decision.contains("` kept as written")))
    .then_some(decision)
}

/// The records of a CSV text, quotes and doubled quotes honored.
fn csv_records(text: &str) -> Vec<Vec<String>> {
    let mut records = Vec::new();
    let mut record = Vec::new();
    let mut cell = String::new();
    let mut quoted = false;
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match (c, quoted) {
            ('"', true) if chars.peek() == Some(&'"') => {
                cell.push('"');
                chars.next();
            }
            ('"', _) => quoted = !quoted,
            (',', false) => record.push(std::mem::take(&mut cell)),
            ('\n', false) => {
                record.push(std::mem::take(&mut cell));
                records.push(std::mem::take(&mut record));
            }
            ('\r', false) => {}
            _ => cell.push(c),
        }
    }
    if !cell.is_empty() || !record.is_empty() {
        record.push(cell);
        records.push(record);
    }
    records
}

fn sorted_entries(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut paths: Vec<PathBuf> = fs::read_dir(dir)
        .map_err(|error| {
            format!(
                "reading {}: {error} (the survey needs refs/)",
                dir.display()
            )
        })?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .collect();
    paths.sort();
    Ok(paths)
}

fn sorted_files(dir: &Path) -> Result<Vec<PathBuf>, String> {
    Ok(sorted_entries(dir)?
        .into_iter()
        .filter(|path| path.is_file())
        .collect())
}

fn file_name(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A record of `format` holding the channels given as name, quantity, times and values.
    fn record(
        format: &str,
        channels: &[(&str, &str, &[f64], &[f64])],
    ) -> hpr_flightdata::log::FlightLog {
        let channels: Vec<serde_json::Value> = channels
            .iter()
            .map(|(column, quantity, time_s, values)| {
                serde_json::json!({
                    "column": column, "quantity": quantity, "time_s": time_s, "values": values,
                })
            })
            .collect();
        serde_json::from_value(serde_json::json!({
            "format": format, "logger": "test", "serial_number": null, "firmware": null,
            "flight_number": null, "stated": {}, "time_s": [], "altitude_m": [],
            "channels": channels, "notes": [],
        }))
        .unwrap()
    }

    #[test]
    fn printed_times_agree_to_six_significant_figures() {
        // A tick exactly halfway, 10.00875 s, prints 10.0087 from single precision.
        assert!(times_agree(10.0087, 10.008_75));
        assert!(times_agree(10.0088, 10.008_75));
        assert!(!times_agree(10.0086, 10.008_75));
        // Below 1 s the export prints to the microsecond or finer.
        assert!(times_agree(0.04375, 0.043_75));
        assert!(!times_agree(0.0437, 0.043_75));
        assert!(times_agree(0.0, 0.0));
    }

    #[test]
    fn mdacs_clock_sums_its_step_in_single_precision() {
        let clock = mdacs_clock(3_200);
        assert_eq!(&clock[..4], [0, 5, 10, 15]);
        // n / 20 until the single-precision sum, written to seven figures, crosses a half.
        assert_eq!(clock[1_000], 5_000);
        assert_eq!(clock[3_133], 15_666);
        assert!(clock.windows(2).all(|pair| pair[1] - pair[0] == 5
            || pair[1] - pair[0] == 6
            || pair[1] - pair[0] == 4));
    }

    /// An export of `format` beside a raw record: an acceleration channel and an altitude on a
    /// clock of its own (so no one clock), its conversions named and not compared.
    fn index_export(acceleration: &[f64], velocity: &[f64]) -> hpr_flightdata::log::FlightLog {
        let times = [0.0, 0.0025, 0.005, 0.0075];
        record(
            "raven_fip_csv",
            &[
                (
                    "Axial Accel (Gs)",
                    "acceleration",
                    &times[..acceleration.len()],
                    acceleration,
                ),
                (
                    "Velocity (Accel-Ft/Sec)",
                    "speed",
                    &times[..velocity.len()],
                    velocity,
                ),
                (
                    "[Altitude (Baro-Ft-AGL)]",
                    "altitude_above_ground",
                    &[0.0],
                    &[1.0],
                ),
            ],
        )
    }

    #[test]
    fn a_raw_record_matches_an_export_only_sample_for_sample() {
        let (g, v) = (0.01 * 9.806_65, 0.1 * 0.3048);
        let times = [0.0, 0.0025, 0.005];
        let raw = record(
            "raven_fipa",
            &[
                (
                    "Axial Accel (Gs)",
                    "acceleration",
                    &times,
                    &[g, 2.0 * g, 3.0 * g],
                ),
                (
                    "Velocity (Accel-Ft/Sec)",
                    "speed",
                    &times,
                    &[v, 2.0 * v, 3.0 * v],
                ),
            ],
        );
        let right = [g, 2.0 * g, 3.4 * g];
        let compared = compare_raw(&raw, &index_export(&right, &[v, 2.0 * v, 3.0 * v])).unwrap();
        assert_eq!(compared.raw_samples, 6);
        assert_eq!(compared.velocity_after_the_stop, 0);
        assert!(
            (compared.worst_value_steps - 0.4).abs() < 1e-9,
            "{compared:?}"
        );
        assert_eq!(compared.export_only, ["[Altitude (Baro-Ft-AGL)]"]);
        // The velocity stopped early, as the FIP stops it at apogee: counted apart.
        let stopped = compare_raw(&raw, &index_export(&right, &[v, 2.0 * v])).unwrap();
        assert_eq!(
            (stopped.raw_samples, stopped.velocity_after_the_stop),
            (5, 1)
        );
        // Any other channel stopping early is a difference.
        assert_eq!(
            compare_raw(&raw, &index_export(&right[..2], &[v, 2.0 * v, 3.0 * v])),
            None
        );
        // A value a step off, or a sample past the raw file's end, is a difference, and a
        // difference sets no worst.
        assert_eq!(
            compare_raw(
                &raw,
                &index_export(&[g, 3.0 * g, 3.0 * g], &[v, 2.0 * v, 3.0 * v])
            ),
            None
        );
        assert_eq!(
            compare_raw(&raw, &index_export(&[g; 4], &[v, 2.0 * v, 3.0 * v])),
            None
        );
        // A time the export prints apart from the raw sample's is a difference.
        let mut late = index_export(&right, &[v, 2.0 * v, 3.0 * v]);
        late.channels[0].time_s[1] = 0.0026;
        assert_eq!(compare_raw(&raw, &late), None);
    }

    #[test]
    fn an_mdacs_export_runs_on_its_own_clock() {
        let rows = 3_200;
        let times: Vec<f64> = (0..rows).map(|n| n as f64 / 20.0).collect();
        let pressure: Vec<f64> = (0..rows).map(|n| 100_000.0 - 10.0 * n as f64).collect();
        let raw = record("rrc3_rff", &[("Pressure", "pressure", &times, &pressure)]);
        let printed: Vec<f64> = mdacs_clock(rows)
            .iter()
            .map(|c| *c as f64 / 100.0)
            .collect();
        let export = |time_s: &[f64]| {
            record(
                "rrc3_csv",
                &[
                    ("Pressure", "pressure", time_s, &pressure),
                    ("Altitude", "altitude_above_ground", &[0.0], &[0.0]),
                ],
            )
        };
        let compared = compare_raw(&raw, &export(&printed)).unwrap();
        assert_eq!(compared.raw_samples, rows);
        assert!((compared.worst_time_s - 0.01).abs() < 1e-9, "{compared:?}");
        // `n / 20`, the raw clock itself, isn't how mDACS prints it this far into a log.
        assert_eq!(compare_raw(&raw, &export(&times)), None);
    }

    /// A raw pressure channel at 20 Hz from 0.00375 s, and a one-clock export of it at 800 Hz
    /// holding `at`'s value each tick.
    fn held_pair(
        samples: &[(f64, f64)],
        quantity: &str,
        at: &dyn Fn(f64) -> f64,
    ) -> (
        hpr_flightdata::log::FlightLog,
        hpr_flightdata::log::FlightLog,
    ) {
        let column = if quantity == "speed" {
            "Velocity (Accel-Ft/Sec)"
        } else {
            "Baro (Atm)"
        };
        let (times, values): (Vec<f64>, Vec<f64>) = samples.iter().copied().unzip();
        let raw = record("raven_fipa", &[(column, quantity, &times, &values)]);
        let ticks: Vec<f64> = (0..100).map(|n| f64::from(n) * 0.00125).collect();
        let held: Vec<f64> = ticks.iter().map(|t| at(*t)).collect();
        let export = record(
            "raven_fip_csv",
            &[
                (column, quantity, &ticks, &held),
                ("LiftOff", "number", &ticks, &vec![0.0; ticks.len()]),
            ],
        );
        (raw, export)
    }

    #[test]
    fn a_one_clock_export_holds_each_sample_until_the_next() {
        let samples = [
            (0.00375, 90_000.0),
            (0.05375, 89_000.0),
            (0.10375, 88_000.0),
        ];
        let each = |t: f64| {
            if t < 0.053 {
                90_000.0
            } else if t < 0.103 {
                89_000.0
            } else {
                88_000.0
            }
        };
        let (raw, export) = held_pair(&samples, "pressure", &each);
        // 100 ticks reach 0.12375 s: every raw sample shows at its own time.
        assert_eq!(compare_raw(&raw, &export).unwrap().raw_samples, 3);

        // Stopped at the second sample, the rest holding it: a velocity may stop, compared up to
        // the stop; a pressure may not.
        let stopped = |t: f64| if t < 0.053 { 90_000.0 } else { 89_000.0 };
        let (raw, export) = held_pair(&samples, "speed", &stopped);
        let compared = compare_raw(&raw, &export).unwrap();
        assert_eq!(
            (compared.raw_samples, compared.velocity_after_the_stop),
            (2, 1)
        );
        let (raw, export) = held_pair(&samples, "pressure", &stopped);
        assert_eq!(compare_raw(&raw, &export), None);

        // A sample shown a tick late is a difference.
        let late = |t: f64| {
            if t < 0.055 {
                90_000.0
            } else if t < 0.103 {
                89_000.0
            } else {
                88_000.0
            }
        };
        let (raw, export) = held_pair(&samples, "pressure", &late);
        assert_eq!(compare_raw(&raw, &export), None);

        // A raw sample between two ticks, which no row shows at its own time, is a difference
        // even where the rows hold the right values around it.
        let skipped = [(0.00375, 90_000.0), (0.0040, 90_000.0), (0.10375, 88_000.0)];
        let (raw, export) = held_pair(&skipped, "pressure", &|t| {
            if t < 0.103 { 90_000.0 } else { 88_000.0 }
        });
        assert_eq!(compare_raw(&raw, &export), None);
    }

    #[test]
    fn a_raw_file_that_differs_from_the_export_beside_it_is_unread() {
        // An invented RRC3 download of three seconds and mDACS's export of it, in one folder.
        let rows = 60;
        let mut words: Vec<u16> = Vec::new();
        for n in 0..rows {
            words.push(9_800 - n as u16);
            if n % 20 == 19 {
                words.extend([0x6e00, 0x7e00]);
            }
        }
        let rff = |words: &[u16]| {
            let mut bytes = vec![0u8];
            for value in [1i32, -1, 1, 0] {
                bytes.extend(value.to_le_bytes());
            }
            bytes.extend(b"\x0c\x02\x00\x00\x00mDACS.AnalysisSupport+FlightRecord\x09");
            bytes.extend(2i32.to_le_bytes());
            for _ in 0..2 {
                bytes.extend((words.len() as i32).to_le_bytes());
            }
            bytes.push(0x0f);
            bytes.extend(2i32.to_le_bytes());
            bytes.extend((words.len() as i32 + 1).to_le_bytes());
            bytes.push(7);
            for word in words.iter().chain(&[0]) {
                bytes.extend(word.to_le_bytes());
            }
            bytes.push(0x0b);
            bytes
        };
        let mut csv = "Time,Altitude,Pressure,Velocity,Temperature,Events,Voltages\n".to_owned();
        for (n, time) in mdacs_clock(rows).iter().enumerate() {
            let hpa = f64::from(9_800 - n as u16) / 10.0;
            // The standard atmosphere's height above the first reading, in feet.
            let height_ft = 145_366.45 * (1.0 - (hpa / 1013.25).powf(0.190_284))
                - 145_366.45 * (1.0 - (980.0f64 / 1013.25).powf(0.190_284));
            csv += &format!(
                "{}.{:02},{height_ft:.2},{hpa:.2},{:.2},71.60, -,9.20\n",
                time / 100,
                time % 100,
                height_ft / 3.0
            );
        }
        let sample = |name: &str, family: &str, bytes: Vec<u8>| Sample {
            path: PathBuf::from("flight").join(name),
            source: "test",
            family: family.to_owned(),
            bytes,
        };
        let export = sample("flight.csv", "rrc3-csv", csv.clone().into_bytes());
        assert_eq!(
            hpr_flightdata::import::detect(&export.bytes),
            Some(LogFormat::Rrc3Csv)
        );
        let raw = sample("flight.rff", "rrc3-rff", rff(&words));
        let (report, unread) = survey(&[raw, export], Vec::new());
        let checks = &report.raw_checks["rrc3-rff"];
        assert_eq!(
            (checks.files, checks.matching_an_export),
            (1, 1),
            "{unread:?}"
        );
        assert!(unread.is_empty(), "{unread:?}");

        // One reading a step off: the raw file differs, and the survey lists it unread.
        words[30] -= 1;
        let raw = sample("flight.rff", "rrc3-rff", rff(&words));
        let export = sample("flight.csv", "rrc3-csv", csv.into_bytes());
        let (report, unread) = survey(&[raw, export], Vec::new());
        let checks = &report.raw_checks["rrc3-rff"];
        assert_eq!((checks.matching_an_export, checks.differing.len()), (0, 1));
        assert!(
            unread
                .iter()
                .any(|line| line.contains("differs from every export")),
            "{unread:?}"
        );
    }

    #[test]
    fn pfa_checks_count_the_stated_figures_and_the_eighth_sample() {
        // Invented flights: the first detected at 80 ft at its eighth sample, its figures as
        // stated; the second at 40 ft, stating one sample too many and an apogee above its highest.
        let pfa = |ground: &str, heights: &[u32]| {
            let mut text = format!(
                "PerfectFlite Alt15K/WD\rGround: 100\rApogee: {ground}\rNumSamps: {}\rComments: \r\r**!!Data: \r",
                heights.len()
            );
            for h in heights {
                text += &format!("{h:05}\r");
            }
            text
        };
        let mut checks = PfaChecks::default();
        let first = pfa("120", &[2, 5, 10, 20, 35, 50, 70, 85, 100, 120, 110, 60, 0]);
        checks.add(&hpr_flightdata::import::read(first.as_bytes()).unwrap());
        let second = pfa("90", &[1, 3, 6, 10, 18, 25, 33, 45, 60, 80, 70, 40, 0])
            .replace("NumSamps: 13", "NumSamps: 14");
        checks.add(&hpr_flightdata::import::read(second.as_bytes()).unwrap());
        assert_eq!(
            checks,
            PfaChecks {
                files: 2,
                samples_as_stated: 1,
                apogee_is_the_highest: 1,
                detected_at_80_ft: 1,
                detected_at_40_ft: 1,
            }
        );
    }

    #[test]
    fn csv_records_honor_quotes() {
        let records = csv_records("a,b\n\"x, y\",\"say \"\"hi\"\"\"\r\nlast,");
        assert_eq!(
            records,
            [vec!["a", "b"], vec!["x, y", "say \"hi\""], vec!["last", ""]]
        );
    }

    #[test]
    fn aerospacenu_families_come_from_the_first_line() {
        let cases = [
            ("eeprom", "{\"anything\"", "altos-eeprom"),
            ("txt", "PerfectFlite SLCF", "pf2"),
            ("csv", "#version,serial,flight", "altos-csv"),
            ("csv", ",packetType,timestamp_ms", "aerospacenu-fcb-csv"),
            ("csv", ",timestamp_s,baro1_pres", "aerospacenu-fcb-csv"),
            (
                "txt",
                "timestampMs\tpressurePa",
                "aerospacenu-sillygoose-txt",
            ),
            ("csv", "timestamp,latitude,longitude", "csv"),
            ("txt", "{'a': 1}", "txt"),
        ];
        for (extension, first, family) in cases {
            assert_eq!(
                aerospacenu_family(extension, first.as_bytes()),
                family,
                "{first}"
            );
        }
    }

    #[test]
    fn unit_decisions_are_the_notes_before_their_evidence() {
        let decided = "`Velocity` read in miles per hour, from the altitude: the slope is 0.68";
        assert_eq!(
            unit_decision(decided),
            Some("`Velocity` read in miles per hour, from the altitude")
        );
        let listed = "`Gyro_X` and `Gyro_Y` read in degrees per second, from the manual: 300";
        assert_eq!(
            unit_decision(listed),
            Some("`Gyro_X` and `Gyro_Y` read in degrees per second, from the manual")
        );
        let kept = "`Current` kept as written, in a unit no source states: 2 rows";
        assert_eq!(
            unit_decision(kept),
            Some("`Current` kept as written, in a unit no source states")
        );
        for other in [
            "the column \"x\" isn't one this reader knows: kept",
            "`AccelG` has gravity taken out, from the pad: 0.1 g",
            "`Velocity` read in feet per second",
        ] {
            assert_eq!(unit_decision(other), None, "{other}");
        }
    }

    #[test]
    fn a_refused_file_of_an_m7_1b_family_is_named_too() {
        let samples = [Sample {
            path: "e.csv".into(),
            source: "test",
            family: "eggtimer-csv".to_owned(),
            bytes: b"not a log".to_vec(),
        }];
        let (report, unread) = survey(&samples, Vec::new());
        assert_eq!(unread.len(), 1, "{unread:?}");
        assert_eq!(report.families[0].increment, "M7.1b");
    }

    #[test]
    fn a_refused_m7_1a_file_is_named_and_counted() {
        let samples = [
            Sample {
                path: "a.csv".into(),
                source: "test",
                family: "altos-csv".to_owned(),
                bytes: b"not a log".to_vec(),
            },
            Sample {
                path: "b.png".into(),
                source: "test",
                family: "png".to_owned(),
                bytes: b"\x89PNG".to_vec(),
            },
        ];
        let (report, unread) = survey(&samples, Vec::new());
        assert_eq!(unread.len(), 1, "{unread:?}");
        assert!(unread[0].contains("a.csv"), "{unread:?}");
        let altos = &report.families[0];
        assert_eq!(
            (altos.family.as_str(), altos.increment.as_str(), altos.read),
            ("altos-csv", "M7.1a", 0)
        );
        assert_eq!(altos.refused.get("unrecognized"), Some(&1));
        assert_eq!(report.families[1].increment, "not a log");
    }
}
