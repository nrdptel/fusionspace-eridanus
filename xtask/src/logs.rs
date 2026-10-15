//! `cargo xtask logs [--check] [--sheets DIR]`: reads every flight-log sample under `refs/` with
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
//!   [`aerospacenu_family`], a table of their first lines;
//! - the files inside those of the three that are zip archives ([`ARCHIVE_FAMILY`]), unpacked by
//!   `hpr_flightdata::archive` as `archives`, the family of each one no set holds from
//!   [`ARCHIVED`], a table of their ids.
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
use hpr_flightdata::log::{Channel, FlightLog, LogFormat};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const USAGE: &str = "  logs [--check] [--sheets DIR]
                           reads every flight-log sample under refs/ (the collection's
                           catalog, Debrief's fixtures, AerospaceNU's logs) and writes
                           validation/reports/log-import.json and .md; --check writes nothing
                           and fails unless they reproduce and every file of a family
                           with a reader is read; --sheets writes each sample workbook to
                           DIR/<id>.xlsx and its sheets' text to DIR/<id>/<n>.csv, for
                           validation/oracles/xlsx/cells.py (DIR under refs/: it's private)";

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
/// wrote its reader: every file of these must import, and in that format. The workbooks are read
/// sheet by sheet ([`WORKBOOK_FAMILY`]); the other families' files are [`LEFT_TO`]'s.
pub const READ: [(&str, LogFormat, &str); 35] = [
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
    // The catalog files some Blue Raven summaries under each name, and the trackers' under the
    // second; one reader reads both.
    (
        "blueraven-summary-csv",
        LogFormat::FeatherweightSummaryCsv,
        "M7.1c6",
    ),
    (
        "featherweight-summary-csv",
        LogFormat::FeatherweightSummaryCsv,
        "M7.1c6",
    ),
    // The catalog names R-DAS text after the course whose flights it holds.
    ("e80-imu-txt", LogFormat::RdasText, "M7.1c8"),
    ("altos-telem", LogFormat::AltosTelemetry, "M7.1c9"),
    ("altos-eeprom", LogFormat::AltosEeprom, "M7.1c10"),
    ("arts2-raw-csv", LogFormat::Arts2RawCsv, "M7.1c11"),
    ("odf", LogFormat::Arts2Odf, "M7.1c11"),
    ("missileworks-rtx", LogFormat::RtxRaw, "M7.1c12"),
    ("rtx-csv", LogFormat::RtxCsv, "M7.1c12"),
    ("rtx-txt", LogFormat::RtxText, "M7.1c12"),
    ("table-csv", LogFormat::GenericTable, "M7.1c13"),
];

/// The family of zip archives, read when they unpack ([`ARCHIVE_INCREMENT`]): each of their files
/// is then a sample of its own.
const ARCHIVE_FAMILY: &str = "zip";

/// The increment that unpacks [`ARCHIVE_FAMILY`]'s files.
const ARCHIVE_INCREMENT: &str = "M7.1c7";

/// The family of Excel workbooks of a flight's log, read when a sheet of each reads
/// ([`WORKBOOK_INCREMENT`]): its sheets are written out as text and each read as a file.
const WORKBOOK_FAMILY: &str = "workbook-xlsx";

/// The increment that reads [`WORKBOOK_FAMILY`]'s sheets.
const WORKBOOK_INCREMENT: &str = "M7.1c14";

/// The increment the families with no reader yet are left to.
const LEFT_TO: &str = "M7.1c15";

/// Catalog families that aren't flight logs: pictures and documents filed with a flight, maps of a
/// flight's prediction, checklists and an altimeter's settings, a flyer's own program's reading of
/// a log or its description of a rocket, a flyer's summary of flights, a team flight computer's
/// saved state, a simulation's export, a wind profile, and Debrief's made-up flights, which say so in their first line.
const NOT_LOGS: [&str; 15] = [
    "blueraven-events-txt",
    "blueraven-settings-csv",
    "blueraven-settings-xlsx",
    "checklist",
    "debrief-synthetic-csv",
    "empty",
    "flight-computer-params-csv",
    "flight-summary-xlsx",
    "jpeg",
    "kml-map",
    "openrocket-simulation-csv",
    "pdf",
    "png",
    "rocket-parameters",
    "wind-profile-csv",
];

/// The files inside the sample zip archives that no source holds on its own, by id, each with its
/// family, decided by reading it as the catalog's families were decided. A file missing from the
/// table stops the survey, as does an entry it doesn't meet. Files an archive shares with a
/// source, byte for byte, are counted once, as that source's.
const ARCHIVED: [(&str, &str); 19] = [
    // A Blue Raven's high-rate and low-rate files, as the app exports them.
    ("92aab3f3d5a5604a", "blueraven-csv"),
    ("93227763f88a65cc", "blueraven-csv"),
    ("a18dd4fa3f9a3379", "blueraven-csv"),
    ("0da542e62eb9221a", "blueraven-csv"),
    ("c96241f103ecc57e", "blueraven-csv"),
    ("632d9640d30c91a6", "blueraven-csv"),
    ("1bd061a33dad51bf", "blueraven-csv"),
    ("4acaa35bbf175a1c", "blueraven-csv"),
    // The app's summaries of those flights.
    ("261d753bc922b301", "blueraven-summary-csv"),
    ("ba85f42d4d359fca", "blueraven-summary-csv"),
    ("c67e39be7ddf16d3", "blueraven-summary-csv"),
    // A GPS tracker's track as KML.
    ("ee9fef531a92dd3b", "kml"),
    // A Blue Raven's deployment settings, as a spreadsheet and as CSV.
    ("447c80b042a64226", "blueraven-settings-xlsx"),
    ("8bb7e4827716686e", "blueraven-settings-csv"),
    // A flyer's own program's events read from a low-rate file, and its description of the rocket.
    ("ee725a7e0b36471d", "blueraven-events-txt"),
    ("dc014691471f301d", "blueraven-events-txt"),
    ("e72518ad914a3f67", "blueraven-events-txt"),
    ("b70be23d233b6648", "rocket-parameters"),
    // A wind profile, altitude against wind speed and direction.
    ("7513ab8018b8019f", "wind-profile-csv"),
];

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
    // Debrief's six made-up flights, in its own CSV with a `Synthetic` marker row whose text says
    // the file "is not a recording of anything": not a log.
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
/// Why a file the catalog files as generic CSV is an ARTS2's raw data.
const ARTS2_RAW: &str = "Ozark Aerospace ARTS2's raw data, a `MET,Acc(HI),Acc/Baro(LO),Baro(HI),\
    State` header then the sensors' counts with no calibration in the file, filed as generic CSV \
    by its extension; the flight's `.odf` holds the same records with the calibration";
/// Why a file the catalog files as generic CSV is the phone app's.
const PHONE_LAUNCH: &str = "a phone app's `launch….csv`, a header opening \
    `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension";
/// Why a file the catalog files as generic CSV is an RTx's CSV export.
const RTX_CSV: &str = "Missile Works mDACS's CSV export of an RTx GPS tracker's fixes, a \
    `UTC Time,Latitude,Longitude,Altitude,…` header then one row a fix, filed as generic CSV by \
    its extension";
/// Why a file the catalog files as generic CSV is read as a table by its header.
const TABLE_CSV: &str = "a table no other reader takes, a header row naming its clock with a \
    unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension";
/// Why a file the catalog files as text is an RTx's text export.
const RTX_TXT: &str = "Missile Works mDACS's text export of an RTx GPS tracker's fixes, a \
    tab-separated `UTC Lat Lon Alt …` header then one row a fix, filed as text by its extension";
/// Why a file the catalog files as RTF is a radio telemetry stream, for M7.1c7.
const TELEMETRY_RTF: &str = "a radio telemetry stream, lines such as `255,T,…` in a layout no \
    maker's document names yet, saved inside an RTF document; filed as RTF, not a log, by its \
    extension";
/// Why a file the catalog files as RTF is an Eggtimer's download.
const EGGTIMER_RTF: &str = "an Eggtimer TRS's terminal download, its settings, summary and \
    `T,Alt,…` table, saved inside an RTF document; filed as RTF, not a log, by its extension";

/// Why a spreadsheet is a workbook of logs, its sheets each read as a file of its own.
const WORKBOOK: &str = "an Excel workbook, `.xlsx`, of a flight's log: a sheet or several, each a \
    table a reader takes once the cells are written out as text, such as a Blue Raven's high-rate \
    and low-rate files side by side or a table naming its units; filed as a spreadsheet";
/// Why a spreadsheet is a flyer's summary of flights, not a log.
const FLIGHT_SUMMARY_XLSX: &str = "a flyer's summary of flights in a spreadsheet, a few rows of \
    names, dates and figures by flight or by launch, and no samples; not a log";
/// Why a spreadsheet is a team's checklist, not a log.
const CHECKLIST: &str = "a team's checklist for preparing a flight, its steps in a spreadsheet's \
    cells and no measurement; not a log";
/// Why a spreadsheet is a Blue Raven's settings, not a log.
const BLUERAVEN_SETTINGS: &str = "a Blue Raven's deployment settings saved as a spreadsheet, a \
    `Hardware_Channel:` row naming its outputs (`Apo_Pri`, `Apo_Sec`, …) and their conditions; an \
    altimeter's settings, not a flight's data";

/// Files whose catalog family isn't their layout, by id: the family each is, and why. An entry
/// the survey doesn't meet stops it, so the table can't outlive its files.
const RECLASSIFIED: [(&str, &str, &str, &str); 160] = [
    ("02cf95e874f824f7", "xlsx", "workbook-xlsx", WORKBOOK),
    ("084653fe77f2ca50", "xlsx", "workbook-xlsx", WORKBOOK),
    ("229efb1695f3277f", "xlsx", "workbook-xlsx", WORKBOOK),
    ("2d2455b75cfe7672", "xlsx", "workbook-xlsx", WORKBOOK),
    ("3e56a501d1b71a8a", "xlsx", "workbook-xlsx", WORKBOOK),
    ("44a479aaaf2812ce", "xlsx", "workbook-xlsx", WORKBOOK),
    ("8855863e76ea3fbc", "xlsx", "workbook-xlsx", WORKBOOK),
    ("9ee24565e8232902", "xlsx", "workbook-xlsx", WORKBOOK),
    ("acc19f3b1096cb74", "xlsx", "workbook-xlsx", WORKBOOK),
    ("b27cc498f8d15caa", "xlsx", "workbook-xlsx", WORKBOOK),
    ("c34428a0bff32b60", "xlsx", "workbook-xlsx", WORKBOOK),
    ("c3e5272b33d0ecdb", "xlsx", "workbook-xlsx", WORKBOOK),
    ("d21437764a14c64b", "xlsx", "workbook-xlsx", WORKBOOK),
    ("e3ff82bb5d72ff52", "xlsx", "workbook-xlsx", WORKBOOK),
    ("ef7b7dbbf32a8e67", "xlsx", "workbook-xlsx", WORKBOOK),
    ("f7b6e0a3273d6b79", "xlsx", "workbook-xlsx", WORKBOOK),
    (
        "331cacde5f5bbc07",
        "xlsx",
        "flight-summary-xlsx",
        FLIGHT_SUMMARY_XLSX,
    ),
    (
        "9e277f9ff863b66e",
        "xlsx",
        "flight-summary-xlsx",
        FLIGHT_SUMMARY_XLSX,
    ),
    (
        "e972c010b8861aca",
        "xlsx",
        "flight-summary-xlsx",
        FLIGHT_SUMMARY_XLSX,
    ),
    ("ccf8bcecd91d5b91", "csv", "table-csv", TABLE_CSV),
    ("061c084cf2d6155c", "csv", "table-csv", TABLE_CSV),
    ("065306f8cf7fc60c", "csv", "table-csv", TABLE_CSV),
    ("0e2537156abd776a", "csv", "table-csv", TABLE_CSV),
    ("111209c05b9be47d", "csv", "table-csv", TABLE_CSV),
    ("32deb5cb173b5983", "csv", "table-csv", TABLE_CSV),
    ("4243bc06d5f49f2a", "csv", "table-csv", TABLE_CSV),
    ("469c003f029e47b4", "csv", "table-csv", TABLE_CSV),
    ("4eef930c2b534a42", "csv", "table-csv", TABLE_CSV),
    ("5428e790bc55cec5", "csv", "table-csv", TABLE_CSV),
    ("5cb271496c8e5623", "csv", "table-csv", TABLE_CSV),
    ("67fe4200832a39ad", "csv", "table-csv", TABLE_CSV),
    ("6821b3c5c90eeaf3", "csv", "table-csv", TABLE_CSV),
    ("6bc2c4836a5a3504", "csv", "table-csv", TABLE_CSV),
    ("76dad6a7c38d259b", "csv", "table-csv", TABLE_CSV),
    ("7e7df671ac0ab9ad", "csv", "table-csv", TABLE_CSV),
    ("82ebfd0a31998e39", "csv", "table-csv", TABLE_CSV),
    ("89e3cfd45c66aa71", "csv", "table-csv", TABLE_CSV),
    ("956ff350f72097d1", "csv", "table-csv", TABLE_CSV),
    ("9c6ffdeffee7488a", "csv", "table-csv", TABLE_CSV),
    ("b348b373dbe7073f", "csv", "table-csv", TABLE_CSV),
    ("bdd32fb1d19bf31e", "csv", "table-csv", TABLE_CSV),
    ("cf949df321940506", "csv", "table-csv", TABLE_CSV),
    ("cface22527bb390b", "csv", "table-csv", TABLE_CSV),
    ("dab6728a9ab22228", "csv", "table-csv", TABLE_CSV),
    ("e79347ccfd0bd43b", "csv", "table-csv", TABLE_CSV),
    ("eafaeb9b290d7529", "csv", "table-csv", TABLE_CSV),
    ("ebd3b757555b564c", "csv", "table-csv", TABLE_CSV),
    ("ec3a7fcc83121635", "csv", "table-csv", TABLE_CSV),
    ("fd61caa7235b81b6", "csv", "table-csv", TABLE_CSV),
    ("096375243584d110", "csv", "rtx-csv", RTX_CSV),
    ("1d34c9a2ea3264f0", "csv", "rtx-csv", RTX_CSV),
    ("2920bddb383455de", "csv", "rtx-csv", RTX_CSV),
    ("3d1d6c4ef87ce0da", "csv", "rtx-csv", RTX_CSV),
    ("fc937944649779db", "csv", "rtx-csv", RTX_CSV),
    ("0cfb74a4c988654b", "txt", "rtx-txt", RTX_TXT),
    ("74201deabc8b669a", "txt", "rtx-txt", RTX_TXT),
    ("8b482eb6ae9af583", "txt", "rtx-txt", RTX_TXT),
    ("a65103199f08052b", "txt", "rtx-txt", RTX_TXT),
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
    ("0ad26ad4cfc8539b", "ods", "checklist", CHECKLIST),
    ("34affe179ab66e72", "ods", "checklist", CHECKLIST),
    ("8e23a8dc0c55248b", "ods", "checklist", CHECKLIST),
    ("c560a04cf2a5dccf", "ods", "checklist", CHECKLIST),
    (
        "0f9a81a6d3406353",
        "xlsx",
        "blueraven-settings-xlsx",
        BLUERAVEN_SETTINGS,
    ),
    (
        "1887d4ff0c2ac130",
        "xlsx",
        "blueraven-settings-xlsx",
        BLUERAVEN_SETTINGS,
    ),
    (
        "40d01ca400d6085a",
        "xlsx",
        "blueraven-settings-xlsx",
        BLUERAVEN_SETTINGS,
    ),
    (
        "526a2e2c94f25728",
        "xlsx",
        "blueraven-settings-xlsx",
        BLUERAVEN_SETTINGS,
    ),
    (
        "63543ab201ee2fbd",
        "xlsx",
        "blueraven-settings-xlsx",
        BLUERAVEN_SETTINGS,
    ),
    (
        "936ac194735a318a",
        "xlsx",
        "blueraven-settings-xlsx",
        BLUERAVEN_SETTINGS,
    ),
    (
        "afcf758d866f6e78",
        "xlsx",
        "blueraven-settings-xlsx",
        BLUERAVEN_SETTINGS,
    ),
    (
        "d2a7db8b9da843ef",
        "xlsx",
        "blueraven-settings-xlsx",
        BLUERAVEN_SETTINGS,
    ),
    (
        "d3354f2054b6b18c",
        "xlsx",
        "blueraven-settings-xlsx",
        BLUERAVEN_SETTINGS,
    ),
    (
        "77e28512645441b1",
        "csv",
        "perfectflite-txt",
        "a PerfectFlite text export with no header row, tab-separated, each temperature written \
         with its unit's letter; filed as CSV by its extension",
    ),
    (
        "0ed0a85f84b31849",
        "txt",
        "perfectflite-txt",
        "a PerfectFlite text export with no header row, its temperatures in °F",
    ),
    (
        "c9bcbd7bdcf795bd",
        "txt",
        "perfectflite-txt",
        "a PerfectFlite text export with no header row, its temperatures in °F",
    ),
    (
        "06ba9b8df63e90ca",
        "txt",
        "perfectflite-txt",
        "a PerfectFlite text export with no header row, its temperatures in °C",
    ),
    (
        "a0f814e69afe42b6",
        "csv",
        "perfectflite-csv",
        "a PerfectFlite CSV export whose header names its temperature `Temp`, without the text \
         export's dot, each temperature written with its unit's letter",
    ),
    (
        "6ad288f13a3ec6d8",
        "csv",
        "blueraven-csv",
        "a Blue Raven high-rate file whose header opens `Flight_Time_(s),Sync`, with no date or \
         time of day",
    ),
    (
        "ff3ad892e63f060d",
        "csv",
        "table-csv",
        "a Blue Raven low-rate file a person edited in a spreadsheet: no date or time of day, a \
         column with no name, and two columns named in words the app doesn't write; its names \
         state their units, so it reads as a table by its header",
    ),
    (
        "12fd94ba690475b8",
        "txt",
        "blueraven-events-txt",
        "a flyer's own program's events read from a Blue Raven's low-rate file, titled with the \
         file it read",
    ),
    (
        "4aae566440b3f002",
        "txt",
        "blueraven-events-txt",
        "a flyer's own program's events read from a Blue Raven's low-rate file, titled with the \
         file it read",
    ),
    (
        "b26cc0ad79d74a7b",
        "txt",
        "rocket-parameters",
        "a flyer's own program's description of a rocket and its motor, `# Name = value` lines",
    ),
    (
        "b2f63e295244a1c5",
        "txt",
        "rocket-parameters",
        "a flyer's own program's description of a rocket and its motor, `# Name = value` lines",
    ),
    (
        "386d1424975a627f",
        "txt",
        "rocket-parameters",
        "a team's description of a rocket for its own program, `name: value` lines",
    ),
    (
        "45df38a00638c4ae",
        "csv",
        "flight-computer-params-csv",
        "a team flight computer's saved state, one row: a boot count, gyro biases and the \
         pressure before boost",
    ),
    (
        "ca5fce4c1fca4c14",
        "csv",
        "flight-computer-params-csv",
        "a team flight computer's saved state, one row: a boot count, gyro biases and the \
         pressure before boost",
    ),
    (
        "62b4c3516138c59a",
        "csv",
        "flight-computer-params-csv",
        "a team flight computer's saved state, one row: a boot count, gyro biases and the \
         pressure before boost",
    ),
    (
        "35d8b51bbba48517",
        "csv",
        "openrocket-simulation-csv",
        "an OpenRocket simulation's export, `# Simulation` comment lines then simulated values: \
         a simulation, not a flight",
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
    /// The Featherweight summaries against the files of samples beside them, by logger.
    #[serde(default)]
    summary_checks: BTreeMap<String, SummaryChecks>,
    /// AltOS's radio telemetry against AltosUI's exports of the same flight.
    #[serde(default)]
    telemetry_checks: TelemetryChecks,
    /// AltOS's raw downloads against AltosUI's exports of the same flight and its telemetry.
    #[serde(default)]
    eeprom_checks: EepromChecks,
    /// ARTS2's `.odf` files against the raw CSV, the interpreted export and an AltOS board's CSV of
    /// the same flight.
    #[serde(default)]
    arts2_checks: Arts2Checks,
    /// Missile Works RTx's `.rtx` files against the reader's record and the exports of the same
    /// flight beside them.
    #[serde(default)]
    rtx_checks: RtxChecks,
    /// The tables read by their headers, each column's decision counted.
    #[serde(default)]
    table_checks: Vec<TableFile>,
    /// The Excel workbooks, each sheet's outcome counted.
    workbook_checks: Vec<WorkbookFile>,
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
/// feet and in meters, and each of the others by id, `id feet meters`.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ExportUnits {
    files: usize,
    peak_equals_a_pf2_beside_it: usize,
    climb_over_fall_beside_a_pf2_in_feet: [f64; 2],
    climb_over_fall_others_in_feet: [f64; 2],
    climb_over_fall_others_in_meters: [f64; 2],
    climb_over_fall_others_by_id: Vec<String>,
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
        // A channel with no samples, on either side, or with times and values that don't pair up,
        // compares with nothing.
        let paired = |c: &hpr_flightdata::log::Channel| c.time_s.len() == c.values.len();
        if own.values.is_empty() || channel.values.is_empty() || !paired(own) || !paired(channel) {
            return None;
        }
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
        let at_its_time = timed == j && times_agree(time, own.time_s[j]);
        if !out.value(value, own.values[j], step) {
            // Stopped early: the export holds its previous value to the end.
            let stopped = may_stop
                && i > 0
                && channel.values[i..]
                    .iter()
                    .all(|later| later.to_bits() == channel.values[i - 1].to_bits());
            return stopped.then_some(j);
        }
        if at_its_time {
            out.time(time, own.time_s[j]);
            timed += 1;
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

/// AltOS's radio telemetry against AltosUI's exports of the same flight, and its board's
/// estimates against each other: each file's packets ([`TelemetryFile`]); of the files, how many
/// have an AltOS CSV export beside them, in the same folder, that its reader reads, and how many
/// of those agree with one of them; of those files' sensor packets, how many there are and how
/// many fall at a tick the export also has a row at; of those, how many equal the export's
/// pressure there, and how many meet only rows that repeat the pressure of the row before them;
/// the worst difference in temperature, K, at the pressures equal; how many GPS values (a
/// latitude, a longitude or an altitude) meet a fix in the export at their tick, and the worst
/// difference in latitude or longitude, degrees, and in altitude, m; the least and greatest
/// ratio of the telemetry's highest `height` to the export's; and, over every file, the least
/// and greatest median ratio of `height`'s climb to `speed` after burnout and of `speed`'s change
/// to `acceleration` in the climb ([`scales`]). The files with an export beside them that none
/// agrees with, those whose ratios fall outside [`SCALE_BAND`], and those with too few fast
/// packets for a ratio, by id.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct TelemetryChecks {
    per_file: Vec<TelemetryFile>,
    files: usize,
    with_an_export_beside: usize,
    agreeing_with_an_export: usize,
    sensor_packets_beside_an_export: usize,
    at_the_exports_ticks: usize,
    pressures_equal: usize,
    pressures_at_repeated_rows: usize,
    worst_temperature_k: f64,
    gps_values_compared: usize,
    worst_position_deg: f64,
    worst_gps_altitude_m: f64,
    height_over_export: Option<[f64; 2]>,
    climb_over_speed: Option<[f64; 2]>,
    speed_change_over_acceleration: Option<[f64; 2]>,
    differing: Vec<String>,
    inconsistent: Vec<String>,
    unchecked: Vec<String>,
}

/// One AltOS telemetry file's packets, as its record's notes count them: its id, the packets
/// heard, those that failed the radio's CRC, the board's power-ons, and whether its pressure was
/// read in pascals.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct TelemetryFile {
    id: String,
    packets_heard: usize,
    failing_the_crc: usize,
    power_ons: usize,
    pressure_in_pascals: bool,
}

impl TelemetryFile {
    /// The counts from a record's notes, as `hpr_flightdata::altos_telem` writes them: `N packets
    /// heard: M failed the radio's CRC …` and `the board's tick starts again K times …`.
    fn of(id: String, log: &hpr_flightdata::log::FlightLog) -> Self {
        let number_before = |note: &str, marker: &str| -> Option<usize> {
            note.split_once(marker)?.0.rsplit(' ').next()?.parse().ok()
        };
        let mut file = Self {
            id,
            power_ons: 1,
            pressure_in_pascals: log.channels.iter().any(|c| c.column == "pressure"),
            ..Self::default()
        };
        for note in &log.notes {
            if let Some((heard, rest)) = note.split_once(" packets heard: ") {
                file.packets_heard = heard.parse().unwrap_or(0);
                file.failing_the_crc = number_before(rest, " failed").unwrap_or(0);
            }
            if let Some(restarts) = number_before(note, " times, a power-on each") {
                file.power_ons = restarts + 1;
            }
        }
        file
    }
}

/// What the export prints AltOS's temperature to, a tenth of a °C: the telemetry's, in
/// hundredths, agrees within half of it.
const TEMPERATURE_AGREES_K: f64 = 0.05 + 1e-9;

/// How far a GPS fix in the telemetry may be from the export's row at its tick, which carries
/// its last fix, perhaps a few seconds older: 0.01° is about a kilometer, where a misread scale
/// would move a position by tens of degrees.
const POSITION_AGREES_DEG: f64 = 0.01;

/// The same for the GPS altitude: 100 m, where feet read as meters would move a pad at 1,000 m
/// above sea level by 2,280 m.
const GPS_ALTITUDE_AGREES_M: f64 = 100.0;

/// How near 1 the ratios of [`scales`] and of the highest heights must be: well inside a factor
/// of 16, the scale of `acceleration` and `speed`, or of 3.28, feet against meters.
const SCALE_BAND: std::ops::RangeInclusive<f64> = 0.8..=1.25;

/// The median ratio of `height`'s climb to `speed`, over pairs of consecutive sensor packets
/// from 0.05 to 0.25 s apart, both after burnout (states 5 to 7, coast to main) at more than
/// 20 m/s; and of `speed`'s change to `acceleration`, the same but in the climb (states 3 and 4,
/// boost and fast) at more than 20 m/s². Each is 1 if the three are in the units the reader
/// takes; `None` where no pair qualifies. The board sends a sensor packet ten times a second in
/// the climb and once a second in the descent (*AltOS Telemetry*, §2.4), so the first ratio's
/// pairs come from the coast, the climb after burnout.
fn scales(log: &hpr_flightdata::log::FlightLog) -> (Option<f64>, Option<f64>) {
    let values = |column: &str| {
        log.channels
            .iter()
            .find(|channel| channel.column == column)
            .map(|channel| (&channel.time_s, &channel.values))
    };
    let (Some((time, state)), Some((_, height)), Some((_, speed)), Some((_, acceleration))) = (
        values("state"),
        values("height"),
        values("speed"),
        values("acceleration"),
    ) else {
        return (None, None);
    };
    let n = time
        .len()
        .min(state.len())
        .min(height.len())
        .min(speed.len())
        .min(acceleration.len());
    let (mut climb, mut change) = (Vec::new(), Vec::new());
    for i in 1..n {
        let dt = time[i] - time[i - 1];
        if !(dt > 0.05 && dt <= 0.25) {
            continue;
        }
        let both = |range: std::ops::RangeInclusive<f64>| {
            range.contains(&state[i]) && range.contains(&state[i - 1])
        };
        let v = (speed[i] + speed[i - 1]) / 2.0;
        if both(5.0..=7.0) && v.abs() > 20.0 {
            climb.push((height[i] - height[i - 1]) / dt / v);
        }
        let a = (acceleration[i] + acceleration[i - 1]) / 2.0;
        if both(3.0..=4.0) && a.abs() > 20.0 {
            change.push((speed[i] - speed[i - 1]) / dt / a);
        }
    }
    let median = |mut ratios: Vec<f64>| {
        ratios.retain(|ratio| ratio.is_finite());
        ratios.sort_by(f64::total_cmp);
        let middle = ratios.len() / 2;
        match ratios.len() {
            0 => None,
            k if k % 2 == 1 => Some(ratios[middle]),
            _ => Some((ratios[middle - 1] + ratios[middle]) / 2.0),
        }
    };
    (median(climb), median(change))
}

/// One turn of AltOS's 16-bit tick, in ticks of 10 ms: 655.36 s.
const ALTOS_TURN: i64 = 65_536;

/// How AltOS telemetry's pressures compare with an AltosUI export's ([`TelemetryChecks`]): the
/// pressures equal and those at repeated rows, or `None` if one differs, if the two share no
/// tick, or if either has no `boost` to line their clocks up by.
///
/// The telemetry's clock is the board's tick in seconds, its turns unwound from the file's first
/// packet; the export's `clock` column is the same tick, its turns unwound from the export's
/// first row. The two differ by whole turns, and their `boost` events, each the first in that
/// state, say how many: the nearest whole number of turns between them.
fn compare_telemetry(
    telemetry: &hpr_flightdata::log::FlightLog,
    export: &hpr_flightdata::log::FlightLog,
) -> Option<TelemetryCompared> {
    fn channel<'a>(
        log: &'a hpr_flightdata::log::FlightLog,
        column: &str,
    ) -> Option<&'a hpr_flightdata::log::Channel> {
        log.channels
            .iter()
            .find(|channel| channel.column == column)
            .filter(|channel| channel.time_s.len() == channel.values.len())
    }
    let boost = |log: &hpr_flightdata::log::FlightLog| {
        log.events
            .iter()
            .find(|event| event.name == "boost")
            .map(|event| event.time_s)
    };
    let ticks = |seconds: f64| (seconds * 100.0).round() as i64;
    let (clock, pressure) = (channel(export, "clock")?, channel(export, "pressure")?);
    if clock.time_s != pressure.time_s {
        return None;
    }
    let export_boost = boost(export)?;
    let row = clock.time_s.iter().position(|time| *time == export_boost)?;
    let turns = (ticks(boost(telemetry)?) - ticks(clock.values[row])) as f64 / ALTOS_TURN as f64;
    let offset = turns.round() as i64 * ALTOS_TURN;
    let mut rows: BTreeMap<i64, Vec<usize>> = BTreeMap::new();
    for (i, value) in clock.values.iter().enumerate() {
        if value.is_finite() {
            rows.entry(ticks(*value)).or_default().push(i);
        }
    }
    let own = channel(telemetry, "pressure")?;
    let own_temperature = channel(telemetry, "temperature")?;
    let temperature = channel(export, "temperature")?;
    if own_temperature.time_s != own.time_s || temperature.time_s != clock.time_s {
        return None;
    }
    let mut out = TelemetryCompared {
        sensor_packets: own.values.len(),
        ..TelemetryCompared::default()
    };
    for (k, (time, value)) in own.time_s.iter().zip(&own.values).enumerate() {
        let Some(at) = rows.get(&(ticks(*time) - offset)) else {
            continue;
        };
        if let Some(&i) = at
            .iter()
            .find(|&&i| (pressure.values[i] - value).abs() < 0.5)
        {
            out.pressures_equal += 1;
            let difference = (temperature.values[i] - own_temperature.values[k]).abs();
            if difference.is_nan() || difference > TEMPERATURE_AGREES_K {
                return None;
            }
            out.worst_temperature_k = out.worst_temperature_k.max(difference);
        } else if at
            .iter()
            .all(|&i| i > 0 && pressure.values[i].to_bits() == pressure.values[i - 1].to_bits())
        {
            out.pressures_at_repeated_rows += 1;
        } else {
            return None;
        }
    }
    // The GPS fixes: the export's columns by quantity, its GPS altitude being its second
    // `altitude`.
    fn by_quantity(
        log: &hpr_flightdata::log::FlightLog,
        quantity: hpr_flightdata::log::Quantity,
    ) -> Option<&hpr_flightdata::log::Channel> {
        log.channels
            .iter()
            .find(|channel| channel.quantity == quantity)
            .filter(|channel| channel.time_s.len() == channel.values.len())
    }
    use hpr_flightdata::log::Quantity;
    for (quantity, bound) in [
        (Quantity::Latitude, POSITION_AGREES_DEG),
        (Quantity::Longitude, POSITION_AGREES_DEG),
        (Quantity::GnssAltitude, GPS_ALTITUDE_AGREES_M),
    ] {
        let worst = if quantity == Quantity::GnssAltitude {
            &mut out.worst_gps_altitude_m
        } else {
            &mut out.worst_position_deg
        };
        let (Some(own), Some(theirs)) = (
            by_quantity(telemetry, quantity),
            by_quantity(export, quantity),
        ) else {
            continue;
        };
        if theirs.time_s != clock.time_s {
            return None;
        }
        // AltOS writes its sentinel, 2,147,483,647, where it has no fix.
        let fix = |value: f64| value.is_finite() && value.abs() < 1e9;
        let mut compared = 0;
        for (time, value) in own.time_s.iter().zip(&own.values) {
            let Some(at) = rows.get(&(ticks(*time) - offset)) else {
                continue;
            };
            for &i in at {
                let their = theirs.values[i];
                if !fix(*value) || !fix(their) {
                    continue;
                }
                let difference = (their - value).abs();
                if difference.is_nan() || difference > bound {
                    return None;
                }
                *worst = worst.max(difference);
                compared += 1;
            }
        }
        // Fixes on both sides that never meet at a tick compare nothing: no pass.
        if compared == 0
            && own.values.iter().any(|v| fix(*v))
            && theirs.values.iter().any(|v| fix(*v))
        {
            return None;
        }
        out.gps_values_compared += compared;
    }
    let highest = |channel: Option<&hpr_flightdata::log::Channel>| {
        channel?
            .values
            .iter()
            .copied()
            .filter(|value| value.is_finite())
            .max_by(f64::total_cmp)
    };
    out.height_over_export = ratio(
        highest(channel(telemetry, "height")),
        highest(channel(export, "height")),
    );
    (out.pressures_equal > 0).then_some(out)
}

/// What [`compare_telemetry`] found, where nothing differs.
#[derive(Debug, Default, PartialEq)]
struct TelemetryCompared {
    sensor_packets: usize,
    gps_values_compared: usize,
    pressures_equal: usize,
    pressures_at_repeated_rows: usize,
    worst_temperature_k: f64,
    worst_position_deg: f64,
    worst_gps_altitude_m: f64,
    height_over_export: Option<f64>,
}

/// [`TelemetryChecks`] over the samples: each AltOS telemetry file against the AltOS CSV exports in
/// its folder.
fn telemetry_checks(samples: &[Sample]) -> TelemetryChecks {
    let mut checks = TelemetryChecks::default();
    for sample in samples
        .iter()
        .filter(|sample| sample.family == "altos-telem")
    {
        let Ok(telemetry) = hpr_flightdata::import::read(&sample.bytes) else {
            continue;
        };
        checks.files += 1;
        let id = hex(&Sha256::digest(&sample.bytes))[..16].to_owned();
        checks
            .per_file
            .push(TelemetryFile::of(id.clone(), &telemetry));
        let (climb, change) = scales(&telemetry);
        let mut consistent = true;
        for (ratio, range) in [
            (climb, &mut checks.climb_over_speed),
            (change, &mut checks.speed_change_over_acceleration),
        ] {
            if let Some(ratio) = ratio {
                widen(range, ratio);
                consistent &= SCALE_BAND.contains(&ratio);
            }
        }
        if climb.is_none() || change.is_none() {
            checks.unchecked.push(id.clone());
        } else if !consistent {
            checks.inconsistent.push(id.clone());
        }
        let folder = sample.path.parent();
        let exports: Vec<_> = samples
            .iter()
            .filter(|other| other.path.parent() == folder && other.path != sample.path)
            .filter(|other| {
                hpr_flightdata::import::detect(&other.bytes) == Some(LogFormat::AltosCsv)
            })
            .filter_map(|other| hpr_flightdata::import::read(&other.bytes).ok())
            .collect();
        if exports.is_empty() {
            continue;
        }
        checks.with_an_export_beside += 1;
        match exports
            .iter()
            .find_map(|export| compare_telemetry(&telemetry, export))
        {
            Some(compared) => {
                checks.agreeing_with_an_export += 1;
                checks.sensor_packets_beside_an_export += compared.sensor_packets;
                checks.at_the_exports_ticks +=
                    compared.pressures_equal + compared.pressures_at_repeated_rows;
                checks.gps_values_compared += compared.gps_values_compared;
                checks.pressures_equal += compared.pressures_equal;
                checks.pressures_at_repeated_rows += compared.pressures_at_repeated_rows;
                checks.worst_temperature_k =
                    checks.worst_temperature_k.max(compared.worst_temperature_k);
                checks.worst_position_deg =
                    checks.worst_position_deg.max(compared.worst_position_deg);
                checks.worst_gps_altitude_m = checks
                    .worst_gps_altitude_m
                    .max(compared.worst_gps_altitude_m);
                match compared.height_over_export {
                    Some(ratio) => {
                        widen(&mut checks.height_over_export, ratio);
                        if !SCALE_BAND.contains(&ratio) && !checks.inconsistent.contains(&id) {
                            checks.inconsistent.push(id);
                        }
                    }
                    None => {
                        if !checks.inconsistent.contains(&id) {
                            checks.inconsistent.push(id);
                        }
                    }
                }
            }
            None => checks.differing.push(id),
        }
    }
    checks.differing.sort();
    checks.inconsistent.sort();
    checks.unchecked.sort();
    checks.per_file.sort_by(|a, b| a.id.cmp(&b.id));
    checks
}

/// AltOS's raw downloads, `.eeprom`, against AltosUI's exports of the same flight and its radio
/// telemetry: each file's records as its record's notes count them, with what it was compared
/// with ([`EepromFile`]); of the files, how many have an export of the same flight beside them, in
/// the same folder (an AltosUI CSV whose `serial` and `flight` are the file's), and how many of
/// those agree with it; those whose export set its acceleration's zero at the +1 g calibration,
/// by id; how many have the board's radio telemetry beside them (a `.telem` of the same serial
/// number) and how many agree with it; the files that differ from either, by id; and the least
/// and greatest of the climb ratios inside the reader's band, with how many files have one, and
/// the files whose ratio falls outside it, by id and ratio, their counts kept as written.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct EepromChecks {
    per_file: Vec<EepromFile>,
    files: usize,
    with_a_climb_ratio: usize,
    climb_ratios: Option<[f64; 2]>,
    climb_outside_the_band: Vec<String>,
    with_an_export_beside: usize,
    agreeing_with_an_export: usize,
    zeroed_at_the_plus_calibration: Vec<String>,
    with_telemetry_beside: usize,
    agreeing_with_telemetry: usize,
    differing: Vec<String>,
}

/// One AltOS `.eeprom` file: its id, form and log format, and whether the format came from the
/// product, the configuration stating none; its records, the checksums checked (the JSON form's;
/// one failing refuses the file, so none here fails), the erased records, the records repeated
/// exactly and dropped, those after a silence and left out, and whether its last line was cut
/// short; the smallest step between two of its pressures, Pa; the ratio of the climb by the
/// pressure to the climb by the acceleration over the first second after boost, the reader's
/// climb check, and whether it falls outside the reader's band, the acceleration's counts kept as
/// written; and what the export and the telemetry beside it showed.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct EepromFile {
    id: String,
    form: String,
    log_format: u32,
    format_from_product: bool,
    smallest_pressure_step_pa: Option<f64>,
    climb_ratio: Option<f64>,
    climb_outside_the_band: bool,
    records: usize,
    checksums_checked: usize,
    erased: usize,
    duplicates_dropped: usize,
    after_a_silence: usize,
    last_line_cut: bool,
    export: Option<EepromAgainstExport>,
    telemetry: Option<EepromAgainstTelemetry>,
}

/// A `.eeprom` file against the export of its flight, where every value compared agrees: how many
/// of its pressures, temperatures, accelerations and Mega voltages met a row of the export at
/// their tick, and the worst difference of each; whether the export's acceleration is zeroed at
/// the +1 g calibration, not at the flight record's ground reading, the worst then being the
/// difference from that offset; and the export's rows, and those at a tick where the file holds no
/// sample of any channel, which must be none.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct EepromAgainstExport {
    pressures: usize,
    worst_pressure_pa: f64,
    pressure_step_pa: f64,
    temperatures: usize,
    worst_temperature_k: f64,
    accelerations: usize,
    worst_acceleration_m_s2: f64,
    zeroed_at_the_plus_calibration: bool,
    voltages: usize,
    worst_voltage_v: f64,
    rows: usize,
    rows_without_a_sample: usize,
}

/// A `.eeprom` file against the board's radio telemetry of the same flight: how many of its
/// pressures fall at a tick a telemetry sensor packet holds, and how many of those equal it.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct EepromAgainstTelemetry {
    ticks_shared: usize,
    pressures_equal: usize,
}

impl EepromFile {
    /// The counts from a record's notes, as `hpr_flightdata::altos_eeprom` writes them.
    fn of(id: String, log: &FlightLog) -> Self {
        let leading = |note: &str| -> usize {
            note.split(' ')
                .next()
                .and_then(|n| n.parse().ok())
                .unwrap_or(0)
        };
        let mut file = Self {
            id,
            ..Self::default()
        };
        for note in &log.notes {
            if let Some((records, rest)) = note.split_once(" records of ") {
                file.records = records.parse().unwrap_or(0);
                file.log_format = rest
                    .split_once("log format ")
                    .and_then(|(_, after)| after.split(',').next())
                    .and_then(|n| n.parse().ok())
                    .unwrap_or(0);
                if rest.contains("from the JSON form") {
                    file.form = "JSON".to_owned();
                    file.checksums_checked = file.records;
                } else {
                    file.form = "text".to_owned();
                }
            } else if note.contains(" erased records ") {
                file.erased = leading(note);
            } else if note.contains(" records repeated exactly") {
                file.duplicates_dropped = leading(note);
            } else if note.contains(" records from the first more than 10 s") {
                file.after_a_silence = leading(note);
            } else if note.starts_with("the file ends without a line break") {
                file.last_line_cut = true;
            } else if note.starts_with("the configuration states no log format") {
                file.format_from_product = true;
            } else if note.starts_with("`pressure` and `acceleration` held to each other")
                || note.starts_with("over the first second after boost the rocket climbs")
            {
                file.climb_outside_the_band = !note.starts_with("`pressure`");
                file.climb_ratio = note
                    .split_once("a ratio of ")
                    .and_then(|(_, rest)| rest.split(',').next())
                    .and_then(|ratio| ratio.parse().ok());
            }
        }
        if let Some(pressure) = log.channels.iter().find(|c| {
            c.column == "pressure" && c.quantity == hpr_flightdata::log::Quantity::Pressure
        }) {
            let mut values: Vec<f64> = pressure
                .values
                .iter()
                .copied()
                .filter(|v| v.is_finite())
                .collect();
            values.sort_by(f64::total_cmp);
            file.smallest_pressure_step_pa = values
                .windows(2)
                .map(|pair| pair[1] - pair[0])
                .filter(|step| *step > 1e-9)
                .min_by(f64::total_cmp);
        }
        file
    }
}

/// The tick of a `.eeprom` record's zero, the flight record's, from its clock note.
fn eeprom_zero_tick(log: &FlightLog) -> Option<i64> {
    log.notes.iter().find_map(|note| {
        note.split_once("from the flight record's at tick ")?
            .1
            .split(' ')
            .next()?
            .parse()
            .ok()
    })
}

/// An AltosUI export as text, read by its header's names, apart from HPR Sim's AltOS CSV reader so
/// that the check stands on its own: the header, `#version,…` or `version,serial,…` within the
/// first 100 lines, and the rows after it, each split at its commas. Only columns named before
/// any damage to the header line up, so the check reads `time`, `clock`, `serial`, `flight`, the
/// pressure, temperature and acceleration, and a Mega's voltages, which come first.
struct AltosUiExport {
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl AltosUiExport {
    fn read(bytes: &[u8]) -> Option<Self> {
        let text = String::from_utf8_lossy(bytes);
        let mut lines = text.lines();
        let header = lines.by_ref().take(100).find(|line| {
            let line = line.trim();
            line.starts_with("#version,") || line.starts_with("version,serial,")
        })?;
        let columns = header
            .trim()
            .trim_start_matches('#')
            .split(',')
            .map(|name| name.trim().to_owned())
            .collect();
        let rows = lines
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(|line| line.split(',').map(|cell| cell.trim().to_owned()).collect())
            .collect();
        Some(Self { columns, rows })
    }

    fn column(&self, name: &str) -> Option<usize> {
        self.columns.iter().position(|column| column == name)
    }

    /// A row's number in a column; `None` for a gap or AltOS's not-set value, 2,147,483,647.
    fn value(&self, row: usize, column: usize) -> Option<f64> {
        let value: f64 = self.rows.get(row)?.get(column)?.parse().ok()?;
        (value.is_finite() && value.abs() < 1e9).then_some(value)
    }

    /// The step a column is printed to: a unit of its last decimal.
    fn step(&self, column: usize) -> f64 {
        let decimals = self
            .rows
            .iter()
            .filter_map(|row| row.get(column))
            .filter_map(|cell| cell.split_once('.').map(|(_, decimals)| decimals.len()))
            .max()
            .unwrap_or(0);
        10f64.powi(-(decimals as i32))
    }

    /// The first row's cell in a column, as text.
    fn first(&self, name: &str) -> Option<&str> {
        Some(self.rows.first()?.get(self.column(name)?)?.as_str())
    }
}

/// A Mega's voltages, as both name them.
const EEPROM_VOLTAGES: [&str; 7] = [
    "pyro",
    "igniter_a",
    "igniter_b",
    "igniter_c",
    "igniter_d",
    "drogue_voltage",
    "main_voltage",
];

/// A `.eeprom` record against an export ([`EepromAgainstExport`]): `Ok(None)` if the export isn't
/// of its flight, `Err` naming the first value that differs.
///
/// The two line up by tick. The export's `time` is the tick from the flight record's, in seconds,
/// as the record's clock is; but an export whose `time` equals its `clock` at every row counts
/// from the tick itself, the record's clock note giving the flight record's. Among the sample
/// logs only one export does, beside file 219767cba01f7d9d, the export the report also files as
/// `altos-csv-damaged`, and it alone zeroes the acceleration at the +1 g calibration. At each of
/// the record's samples, the export's last row at that hundredth must agree within half the
/// export's print step; a gap or not-set value there is no comparison. The other way, every row of
/// the export must fall at a tick the file holds a sample at, of any channel.
fn compare_eeprom(
    eeprom: &FlightLog,
    export: &AltosUiExport,
) -> Result<Option<EepromAgainstExport>, String> {
    use hpr_flightdata::log::Quantity;
    let same = export.first("serial") == eeprom.serial_number.as_deref()
        && export.first("flight").and_then(|f| f.parse::<u32>().ok()) == eeprom.flight_number;
    if !same {
        return Ok(None);
    }
    let zero = eeprom_zero_tick(eeprom).ok_or("no clock note")?;
    let time = export.column("time").ok_or("no time column")?;
    let clock = export.column("clock");
    let from_tick = clock.is_some_and(|clock| {
        (0..export.rows.len()).all(|row| {
            export.value(row, clock).is_some()
                && export.value(row, clock) == export.value(row, time)
        })
    });
    let mut rows: BTreeMap<i64, usize> = BTreeMap::new();
    for row in 0..export.rows.len() {
        if let Some(seconds) = export.value(row, time) {
            rows.insert((seconds * 100.0).round() as i64, row);
        }
    }
    let key = |seconds: f64| (seconds * 100.0).round() as i64 + if from_tick { zero } else { 0 };
    let figure = |name: &str| {
        eeprom
            .stated
            .figures
            .iter()
            .find(|f| f.name == name)
            .and_then(|f| f.values.first().copied())
    };
    let mut out = EepromAgainstExport::default();
    let mut columns: Vec<(&str, Quantity)> = vec![
        ("pressure", Quantity::Pressure),
        ("temperature", Quantity::Temperature),
        ("acceleration", Quantity::Acceleration),
    ];
    columns.extend(
        EEPROM_VOLTAGES
            .iter()
            .map(|name| (*name, Quantity::Voltage)),
    );
    for (name, quantity) in columns {
        let (Some(own), Some(column)) = (
            eeprom
                .channels
                .iter()
                .find(|c| c.column == name && c.quantity == quantity),
            export.column(name),
        ) else {
            continue;
        };
        let step = export.step(column);
        let half = step / 2.0 + 1e-9;
        let mut differences = Vec::new();
        for (seconds, value) in own.time_s.iter().zip(&own.values) {
            let Some(&row) = rows.get(&key(*seconds)) else {
                continue;
            };
            let (Some(theirs), true) = (export.value(row, column), value.is_finite()) else {
                continue;
            };
            let theirs = if quantity == Quantity::Temperature {
                theirs + 273.15
            } else {
                theirs
            };
            differences.push(value - theirs);
        }
        let worst = |offset: f64| {
            differences
                .iter()
                .map(|d| (d - offset).abs())
                .fold(0.0, f64::max)
        };
        let mut worst_seen = worst(0.0);
        if worst_seen > half && quantity == Quantity::Acceleration {
            // AltosUI may zero the acceleration at the +1 g calibration, not at the flight
            // record's ground reading: a constant offset of (ground − plus) / span × g.
            if let (Some(ground), Some(plus), Some(minus)) = (
                figure("ground_accel"),
                figure("accel_cal_plus"),
                figure("accel_cal_minus"),
            ) {
                let offset = (ground - plus) / ((minus - plus) / 2.0) * 9.806_65;
                if worst(offset) <= half {
                    out.zeroed_at_the_plus_calibration = true;
                    worst_seen = worst(offset);
                }
            }
        }
        if worst_seen > half {
            return Err(format!(
                "{name} differs by {worst_seen}, past half its step {step}"
            ));
        }
        let n = differences.len();
        match quantity {
            Quantity::Pressure => {
                (out.pressures, out.worst_pressure_pa, out.pressure_step_pa) =
                    (n, worst_seen, step);
            }
            Quantity::Temperature => (out.temperatures, out.worst_temperature_k) = (n, worst_seen),
            Quantity::Acceleration => {
                (out.accelerations, out.worst_acceleration_m_s2) = (n, worst_seen);
            }
            _ => {
                out.voltages += n;
                out.worst_voltage_v = out.worst_voltage_v.max(worst_seen);
            }
        }
    }
    if out.pressures == 0 {
        return Err("no pressure met a row of the export".to_owned());
    }
    // The other way: every row of the export at a tick the file holds a sample at, of any channel.
    let own: BTreeSet<i64> = eeprom
        .channels
        .iter()
        .flat_map(|channel| channel.time_s.iter().map(|seconds| key(*seconds)))
        .collect();
    for row in 0..export.rows.len() {
        if let Some(seconds) = export.value(row, time) {
            out.rows += 1;
            if !own.contains(&((seconds * 100.0).round() as i64)) {
                out.rows_without_a_sample += 1;
            }
        }
    }
    if out.rows_without_a_sample > 0 {
        return Err(format!(
            "{} of the export's {} rows fall at a tick the file holds no sample at",
            out.rows_without_a_sample, out.rows
        ));
    }
    Ok(Some(out))
}

/// A `.eeprom` record against the board's radio telemetry ([`EepromAgainstTelemetry`]): `Err` if
/// a pressure differs or none meets a sensor packet. The telemetry's clock is the tick from its
/// first packet's turn, the record's the tick from its flight record's: the two differ by whole
/// turns, as many as lie nearest between their `boost` events.
fn compare_eeprom_telemetry(
    eeprom: &FlightLog,
    telemetry: &FlightLog,
) -> Result<EepromAgainstTelemetry, String> {
    let zero = eeprom_zero_tick(eeprom).ok_or("no clock note")?;
    let ticks = |seconds: f64| (seconds * 100.0).round() as i64;
    let boost = |log: &FlightLog| {
        log.events
            .iter()
            .find(|event| event.name == "boost")
            .map(|event| ticks(event.time_s))
    };
    let (Some(theirs), Some(ours)) = (boost(telemetry), boost(eeprom)) else {
        return Err("no boost to line the clocks up by".to_owned());
    };
    let turns = ((theirs - (ours + zero)) as f64 / ALTOS_TURN as f64).round() as i64;
    fn pascals(log: &FlightLog) -> Option<&Channel> {
        log.channels.iter().find(|c| {
            c.column == "pressure" && c.quantity == hpr_flightdata::log::Quantity::Pressure
        })
    }
    let (Some(own), Some(packets)) = (pascals(eeprom), pascals(telemetry)) else {
        return Err("no pressure in pascals on both sides".to_owned());
    };
    let mut at: BTreeMap<i64, Vec<f64>> = BTreeMap::new();
    for (seconds, value) in packets.time_s.iter().zip(&packets.values) {
        at.entry(ticks(*seconds)).or_default().push(*value);
    }
    let mut out = EepromAgainstTelemetry::default();
    for (seconds, value) in own.time_s.iter().zip(&own.values) {
        let Some(sent) = at.get(&(ticks(*seconds) + zero + turns * ALTOS_TURN)) else {
            continue;
        };
        out.ticks_shared += 1;
        if sent.iter().all(|pa| (pa - value).abs() < 0.5) {
            out.pressures_equal += 1;
        } else {
            return Err(format!(
                "a pressure of {value} where the telemetry sent {sent:?}"
            ));
        }
    }
    if out.ticks_shared == 0 {
        return Err("no pressure falls at a telemetry packet's tick".to_owned());
    }
    Ok(out)
}

/// [`EepromChecks`] over the samples: each AltOS `.eeprom` file against the AltosUI exports and the
/// telemetry in its folder.
fn eeprom_checks(samples: &[Sample]) -> EepromChecks {
    let mut checks = EepromChecks::default();
    for sample in samples
        .iter()
        .filter(|sample| sample.family == "altos-eeprom")
    {
        let Ok(eeprom) = hpr_flightdata::import::read(&sample.bytes) else {
            continue;
        };
        checks.files += 1;
        let id = hex(&Sha256::digest(&sample.bytes))[..16].to_owned();
        let mut file = EepromFile::of(id.clone(), &eeprom);
        let folder = sample.path.parent();
        let beside = || {
            samples
                .iter()
                .filter(move |other| other.path.parent() == folder && other.path != sample.path)
        };
        let mut differs = false;
        let mut exports = 0;
        for export in beside().filter_map(|other| AltosUiExport::read(&other.bytes)) {
            match compare_eeprom(&eeprom, &export) {
                Ok(None) => {}
                Ok(Some(compared)) => {
                    exports += 1;
                    if file.export.is_none() {
                        file.export = Some(compared);
                    }
                }
                Err(_) => {
                    exports += 1;
                    differs = true;
                }
            }
        }
        if exports > 0 {
            checks.with_an_export_beside += 1;
            if !differs {
                checks.agreeing_with_an_export += 1;
            }
        }
        if file
            .export
            .as_ref()
            .is_some_and(|export| export.zeroed_at_the_plus_calibration)
        {
            checks.zeroed_at_the_plus_calibration.push(id.clone());
        }
        let telemetry: Vec<FlightLog> = beside()
            .filter(|other| {
                hpr_flightdata::import::detect(&other.bytes) == Some(LogFormat::AltosTelemetry)
            })
            .filter_map(|other| hpr_flightdata::import::read(&other.bytes).ok())
            .filter(|log| log.serial_number == eeprom.serial_number)
            .collect();
        if !telemetry.is_empty() {
            checks.with_telemetry_beside += 1;
            let mut agrees = true;
            for log in &telemetry {
                match compare_eeprom_telemetry(&eeprom, log) {
                    Ok(compared) => {
                        if file.telemetry.is_none() {
                            file.telemetry = Some(compared);
                        }
                    }
                    Err(_) => agrees = false,
                }
            }
            if agrees {
                checks.agreeing_with_telemetry += 1;
            } else {
                differs = true;
            }
        }
        if differs {
            checks.differing.push(id);
        }
        checks.per_file.push(file);
    }
    checks.differing.sort();
    checks.zeroed_at_the_plus_calibration.sort();
    checks.per_file.sort_by(|a, b| a.id.cmp(&b.id));
    let ratios: Vec<f64> = checks
        .per_file
        .iter()
        .filter(|file| !file.climb_outside_the_band)
        .filter_map(|file| file.climb_ratio)
        .collect();
    checks.climb_outside_the_band = checks
        .per_file
        .iter()
        .filter(|file| file.climb_outside_the_band)
        .map(|file| match file.climb_ratio {
            Some(ratio) => format!("{} ({ratio:.3})", file.id),
            None => file.id.clone(),
        })
        .collect();
    checks.with_a_climb_ratio = ratios.len();
    checks.climb_ratios = ratios
        .iter()
        .copied()
        .min_by(f64::total_cmp)
        .zip(ratios.iter().copied().max_by(f64::total_cmp))
        .map(|(least, greatest)| [least, greatest]);
    checks
}

/// A Featherweight summary's logger, as its record names it, with the format of the file of
/// samples the app exports beside it and the columns of that file whose start is the pad's
/// altitude above sea level, the first the file has: a Blue Raven's barometric altitude, or a
/// tracker's own GNSS altitude in its log or in a ground station's (not the station's `GS Alt
/// asl`).
const SUMMARY_TRACKS: [(&str, LogFormat, &[&str]); 2] = [
    (
        "Featherweight Blue Raven",
        LogFormat::BlueRavenCsv,
        &["Baro_Altitude_ASL_(feet)"],
    ),
    (
        "Featherweight GPS tracker",
        LogFormat::FeatherweightGpsCsv,
        &["ALT", "TRACKER Alt asl"],
    ),
];

/// The first of `columns` that `track` has, as a channel.
fn track_channel<'a>(track: &'a FlightLog, columns: &[&str]) -> Option<&'a Channel> {
    columns.iter().find_map(|column| {
        track
            .channels
            .iter()
            .find(|channel| channel.column == *column)
    })
}

/// How near 1 a summary's figure over a file's must be for the two to agree. The summaries
/// write their lengths in feet; were a file's in meters, the ratio would be 3.28 or 0.30, so any
/// band narrower than a factor of √3.28 ≈ 1.81 either way tells the two units apart, and 10%
/// also leaves out most other flights a folder holds. The figures that agree do so far closer
/// (the report gives their least and greatest ratio).
const SUMMARY_AGREES: f64 = 0.10;

/// How many of a file's first altitudes above sea level the pad's altitude is the median of.
const PAD_SAMPLES: usize = 20;

/// A logger's Featherweight summaries against the files of samples exported beside them, in the
/// same folder, that their reader reads and that hold an altitude (a Blue Raven's low-rate file,
/// not its high-rate one; a tracker's file with a GNSS altitude): how many summaries there are,
/// and have such a file beside them; of those, how many state a maximum altitude in SI, how many
/// state one that agrees with a file's highest altitude above the pad, and how many state a pad
/// altitude that agrees with a file's, each within [`SUMMARY_AGREES`], with the least and
/// greatest ratio, summary over file, of those that do; the summaries with a file beside that
/// agree in neither, by id; and those that would agree only were their feet read as meters,
/// which contradicts the unit they write.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct SummaryChecks {
    files: usize,
    with_a_file_beside: usize,
    stating_an_apogee: usize,
    apogee_agrees: usize,
    apogee_ratios: Option<[f64; 2]>,
    pad_agrees: usize,
    pad_ratios: Option<[f64; 2]>,
    agreeing_in_neither: Vec<String>,
    contradicted: Vec<String>,
}

/// The median of the first [`PAD_SAMPLES`] finite values; `None` if there are none.
fn pad_median(values: &[f64]) -> Option<f64> {
    let mut first: Vec<f64> = values
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .take(PAD_SAMPLES)
        .collect();
    first.sort_by(f64::total_cmp);
    let middle = first.len() / 2;
    match first.len() {
        0 => None,
        n if n % 2 == 1 => Some(first[middle]),
        _ => Some((first[middle - 1] + first[middle]) / 2.0),
    }
}

/// A summary's stated figure over a file's, `None` unless both are finite and the file's isn't 0.
fn ratio(stated: Option<f64>, file: Option<f64>) -> Option<f64> {
    let (stated, file) = (stated?, file?);
    (stated.is_finite() && file.is_finite() && file != 0.0).then(|| stated / file)
}

/// How a summary's figure fares against the files beside it: the ratio nearest 1 among those that
/// agree, or none, and whether one would agree were the summary's feet meters.
fn agreement(ratios: &[f64]) -> (Option<f64>, bool) {
    const FOOT_M: f64 = 0.3048;
    let agrees = |ratio: f64| (ratio - 1.0).abs() <= SUMMARY_AGREES;
    let nearest = ratios
        .iter()
        .copied()
        .filter(|ratio| agrees(*ratio))
        .min_by(|a, b| (a - 1.0).abs().total_cmp(&(b - 1.0).abs()));
    let as_meters = ratios.iter().any(|ratio| agrees(ratio / FOOT_M));
    (nearest, nearest.is_none() && as_meters)
}

/// Widens a least-and-greatest pair to hold `value`.
fn widen(range: &mut Option<[f64; 2]>, value: f64) {
    *range = Some(match *range {
        Some([least, greatest]) => [least.min(value), greatest.max(value)],
        None => [value, value],
    });
}

/// [`SummaryChecks`] over the samples, each summary against the files of samples in its folder.
fn summary_checks(samples: &[Sample]) -> BTreeMap<String, SummaryChecks> {
    let mut checks: BTreeMap<String, SummaryChecks> = BTreeMap::new();
    for sample in samples {
        let Ok(summary) = hpr_flightdata::import::read(&sample.bytes) else {
            continue;
        };
        if summary.format != LogFormat::FeatherweightSummaryCsv {
            continue;
        }
        let Some((logger, track_format, pad_columns)) = SUMMARY_TRACKS
            .iter()
            .find(|(logger, ..)| *logger == summary.logger)
        else {
            continue;
        };
        let entry = checks.entry((*logger).to_owned()).or_default();
        entry.files += 1;
        let folder = sample.path.parent();
        let beside: Vec<FlightLog> = samples
            .iter()
            .filter(|other| other.path.parent() == folder && other.path != sample.path)
            .filter(|other| hpr_flightdata::import::detect(&other.bytes) == Some(*track_format))
            .filter_map(|other| hpr_flightdata::import::read(&other.bytes).ok())
            .filter(|track| {
                !track.altitude_m.is_empty() || track_channel(track, pad_columns).is_some()
            })
            .collect();
        if beside.is_empty() {
            continue;
        }
        entry.with_a_file_beside += 1;
        let stated = &summary.stated;
        if stated.apogee_m.is_some() {
            entry.stating_an_apogee += 1;
        }
        let highest = |values: &[f64]| {
            values
                .iter()
                .copied()
                .filter(|value| value.is_finite())
                .max_by(f64::total_cmp)
        };
        let (mut apogees, mut pads) = (Vec::new(), Vec::new());
        for track in &beside {
            let pad_channel = track_channel(track, pad_columns);
            // The Blue Raven's track is its barometer's altitude above the pad; a tracker's is
            // its receiver's above sea level, so its highest is taken above the stated pad.
            let peak = if *track_format == LogFormat::BlueRavenCsv {
                highest(&track.altitude_m)
            } else {
                pad_channel
                    .and_then(|channel| highest(&channel.values))
                    .zip(stated.ground_elevation_msl_m)
                    .map(|(peak, pad)| peak - pad)
            };
            apogees.extend(ratio(stated.apogee_m, peak));
            let pad = pad_channel.and_then(|channel| pad_median(&channel.values));
            pads.extend(ratio(stated.ground_elevation_msl_m, pad));
        }
        let id = hex(&Sha256::digest(&sample.bytes))[..16].to_owned();
        let (apogee, apogee_as_meters) = agreement(&apogees);
        let (pad, pad_as_meters) = agreement(&pads);
        if let Some(ratio) = apogee {
            entry.apogee_agrees += 1;
            widen(&mut entry.apogee_ratios, ratio);
        }
        if let Some(ratio) = pad {
            entry.pad_agrees += 1;
            widen(&mut entry.pad_ratios, ratio);
        }
        if apogee.is_none() && pad.is_none() {
            entry.agreeing_in_neither.push(id.clone());
        }
        if apogee_as_meters || pad_as_meters {
            entry.contradicted.push(id);
        }
    }
    for entry in checks.values_mut() {
        entry.agreeing_in_neither.sort();
        entry.contradicted.sort();
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
    /// The increment whose reader reads it (`M7.1a`, `M7.1b`, `M7.1c1` … `M7.1c14`), `M7.1c15` for
    /// one not yet read, `damaged`, or `not a log`.
    increment: String,
    files: usize,
    read: usize,
    /// Refusals by kind.
    refused: BTreeMap<String, usize>,
}

/// ARTS2's `.odf` files against the files of the same flight beside them, in their folder, a file
/// each ([`Arts2File`]), and the files that differ from a raw CSV or an interpreted export beside
/// them, by id.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct Arts2Checks {
    per_file: Vec<Arts2File>,
    differing: Vec<String>,
}

/// One `.odf`: its id; its records, decoded here from the bytes apart from the reader; its climb
/// check's ratio, from its record's note; whether the raw CSV beside it holds the same records,
/// word for word and 0.02 s apart (`None` with none beside it); how it compares with the
/// interpreted export beside it ([`Arts2Export`]); and, where an AltOS board's CSV or `.eeprom`
/// lies beside it, the ARTS2's pad and least pressures less the AltOS board's, kPa: differences
/// only, as the pressures themselves would give a private flight's site and apogee.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct Arts2File {
    id: String,
    records: usize,
    climb_ratio: Option<f64>,
    raw_csv_matches: Option<bool>,
    export: Option<Arts2Export>,
    altos_pad_kpa: Option<f64>,
    altos_least_kpa: Option<f64>,
}

/// An `.odf` against DataAnalyzer's interpreted export of the same flight: the records after the
/// first compared, each at the export's row at its time; the largest difference in `Acc`, in the
/// accelerometer's counts; whether the export's first row, the baselines', reads zero in `Acc`
/// and `pAlt`, the columns its reader keeps as written there; its rows after the records, and
/// whether they all read zero in those; the rows whose `pAlt` is a gap; how many pressures `pAlt`
/// meets one record later meet two values of it (clashes); and how many break its order
/// (inversions): from the highest pressure down, `pAlt` is zero at the pad's pressure and above,
/// which DataAnalyzer writes no height below, and rises at every lower one. Reported only: the
/// largest difference of `pAlt` from the pressure altitude in the U.S. Standard Atmosphere, 1976,
/// above the pad's, none below it, m. The number of pressures isn't kept: with a count's step it
/// would give a private flight's apogee.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct Arts2Export {
    rows_compared: usize,
    worst_acceleration_counts: f64,
    first_row_zero: bool,
    rows_after: usize,
    rows_after_zero: bool,
    gaps: usize,
    clashes: usize,
    inversions: usize,
    worst_from_the_standard_m: f64,
}

/// Whether an `.odf` differs from the files of its flight beside it: a raw CSV whose words differ,
/// or an interpreted export off by more than half a count in `Acc`, not zero where it must be,
/// with a gap in `pAlt`, or with a clash or an inversion in its order.
fn arts2_differs(file: &Arts2File) -> bool {
    file.raw_csv_matches == Some(false)
        || file.export.as_ref().is_some_and(|e| {
            e.worst_acceleration_counts.is_nan()
                || e.worst_acceleration_counts > 0.5
                || !e.first_row_zero
                || !e.rows_after_zero
                || e.gaps > 0
                || e.clashes > 0
                || e.inversions > 0
        })
}

/// The `.odf`'s records' four words, decoded from its bytes as the reader's module docs lay them
/// out: the count at byte 289, the records from byte 305, eight bytes each.
fn odf_records(bytes: &[u8]) -> Vec<[u16; 4]> {
    let word = |at: usize| u16::from_le_bytes([bytes[at], bytes[at + 1]]);
    let n = u32::from_le_bytes([bytes[289], bytes[290], bytes[291], bytes[292]]) as usize;
    (0..n)
        .map(|k| [0, 2, 4, 6].map(|w| word(305 + 8 * k + w)))
        .collect()
}

/// The records' time, s: 50 a second from the first.
fn odf_time(k: usize) -> f64 {
    k as f64 / 50.0
}

/// [`Arts2Checks`] over the samples.
fn arts2_checks(samples: &[Sample]) -> Arts2Checks {
    use hpr_flightdata::log::Quantity;
    let mut checks = Arts2Checks::default();
    for sample in samples.iter().filter(|sample| sample.family == "odf") {
        // Only a file read as an `.odf` has the layout its bytes are decoded by here.
        let Ok(odf) = hpr_flightdata::import::read(&sample.bytes) else {
            continue;
        };
        if odf.format != LogFormat::Arts2Odf {
            continue;
        }
        let id = hex(&Sha256::digest(&sample.bytes))[..16].to_owned();
        let records = odf_records(&sample.bytes);
        let beside = |format: LogFormat| -> Vec<FlightLog> {
            samples
                .iter()
                .filter(|other| other.path.parent() == sample.path.parent())
                .filter(|other| hpr_flightdata::import::detect(&other.bytes) == Some(format))
                .filter_map(|other| hpr_flightdata::import::read(&other.bytes).ok())
                .collect()
        };
        let column = |log: &FlightLog, name: &str| -> Option<Channel> {
            log.channels.iter().find(|c| c.column == name).cloned()
        };
        let climb_ratio = odf.notes.iter().find_map(|note| {
            let (_, rest) = note.split_once("a ratio of ")?;
            rest.split(',').next()?.trim().parse().ok()
        });
        let raw_csvs = beside(LogFormat::Arts2RawCsv);
        let raw_csv_matches = (!raw_csvs.is_empty()).then(|| {
            raw_csvs.iter().any(|csv| {
                let words: Vec<Option<Channel>> = ["Acc(HI)", "Acc/Baro(LO)", "Baro(HI)", "State"]
                    .iter()
                    .map(|name| column(csv, name))
                    .collect();
                words.iter().all(|channel| {
                    channel.as_ref().is_some_and(|c| {
                        c.values.len() == records.len()
                            && c.time_s
                                .iter()
                                .enumerate()
                                .all(|(k, t)| (t - odf_time(k)).abs() < 1e-9)
                    })
                }) && records.iter().enumerate().all(|(k, record)| {
                    (0..4).all(|w| {
                        words[w].as_ref().map(|c| c.values[k]) == Some(f64::from(record[w]))
                    })
                })
            })
        });
        // A g in counts, from the calibration sums the record states.
        let per_g = odf
            .stated
            .figures
            .iter()
            .find(|figure| figure.name == "calibration sums")
            .map(|figure| (figure.values[2] - figure.values[0]).abs() / 128.0);
        let acceleration = column(&odf, "Acc(HI)");
        let pressure = column(&odf, "Baro(HI)");
        let pad_pa = odf
            .stated
            .figures
            .iter()
            .find(|figure| figure.name == "Zero Altitude baseline")
            .map(|figure| figure.values[0]);
        let export = match (per_g, &acceleration, &pressure, pad_pa) {
            (Some(per_g), Some(acceleration), Some(pressure), Some(pad_pa)) => {
                beside(LogFormat::Arts2Csv)
                    .iter()
                    .map(|export| {
                        arts2_against_export(
                            export,
                            &acceleration.values,
                            &pressure.values,
                            pad_pa,
                            per_g,
                        )
                    })
                    .min_by(|a, b| {
                        a.worst_acceleration_counts
                            .total_cmp(&b.worst_acceleration_counts)
                    })
            }
            _ => None,
        };
        // AltosUI's export, or the board's raw download where the export doesn't read.
        let altos_pressure = [LogFormat::AltosCsv, LogFormat::AltosEeprom]
            .into_iter()
            .flat_map(beside)
            .find_map(|log| log.channels_of(Quantity::Pressure).next().cloned());
        let least = |values: &[f64]| values.iter().copied().fold(f64::INFINITY, f64::min);
        let (altos_pad_kpa, altos_least_kpa) = match (&altos_pressure, &pressure, pad_pa) {
            (Some(altos), Some(ours), Some(pad_pa)) => (
                altos.values.first().map(|first| (pad_pa - first) / 1_000.0),
                Some((least(&ours.values[1..]) - least(&altos.values)) / 1_000.0),
            ),
            _ => (None, None),
        };
        checks.per_file.push(Arts2File {
            id,
            records: records.len(),
            climb_ratio,
            raw_csv_matches,
            export,
            altos_pad_kpa,
            altos_least_kpa,
        });
    }
    checks.differing = checks
        .per_file
        .iter()
        .filter(|file| arts2_differs(file))
        .map(|file| file.id.clone())
        .collect();
    checks.per_file.sort_by(|a, b| a.id.cmp(&b.id));
    checks.differing.sort();
    checks
}

/// An `.odf`'s acceleration, m/s², and pressure, Pa, one value a record, against an interpreted
/// export's ([`Arts2Export`]); `pad_pa` the pad's pressure and `per_g` a g in counts.
fn arts2_against_export(
    export: &FlightLog,
    acceleration: &[f64],
    pressure: &[f64],
    pad_pa: f64,
    per_g: f64,
) -> Arts2Export {
    use hpr_flightdata::log::Quantity;
    let export_acceleration = export.channels_of(Quantity::Acceleration).next();
    // The two columns the export's reader keeps as written: it marks `Vel`'s and `iAlt`'s
    // closing zeros gaps.
    let zero_at = |k: usize| {
        export_acceleration.and_then(|c| c.values.get(k)) == Some(&0.0)
            && export.altitude_m.get(k) == Some(&0.0)
    };
    let rows = export.time_s.len();
    let mut result = Arts2Export {
        first_row_zero: zero_at(0),
        rows_after: rows.saturating_sub(pressure.len()),
        rows_after_zero: (pressure.len()..rows).all(zero_at),
        ..Arts2Export::default()
    };
    let count = 9.806_65 / per_g;
    let atmosphere = hpr_atmos::ussa76::Ussa76::standard();
    let height = |pa: f64| atmosphere.pressure_altitude_m(pa).unwrap_or(f64::NAN);
    let mut by_pressure: BTreeMap<u64, BTreeSet<u64>> = BTreeMap::new();
    for k in 1..pressure.len() {
        // A record the export holds no row for, at its time, can't match it.
        let (Some(&t), Some(&p_alt)) = (export.time_s.get(k), export.altitude_m.get(k)) else {
            result.worst_acceleration_counts = f64::INFINITY;
            break;
        };
        if (t - odf_time(k)).abs() > 1e-9 {
            result.worst_acceleration_counts = f64::INFINITY;
            break;
        }
        result.rows_compared += 1;
        let theirs = export_acceleration
            .and_then(|c| c.values.get(k))
            .copied()
            .unwrap_or(f64::NAN);
        let off = ((acceleration[k] - theirs) / count).abs();
        result.worst_acceleration_counts = if off.is_nan() {
            f64::INFINITY
        } else {
            result.worst_acceleration_counts.max(off)
        };
        if p_alt.is_nan() {
            result.gaps += 1;
            continue;
        }
        // Positive floats' bits sort as their values do.
        by_pressure
            .entry(pressure[k - 1].to_bits())
            .or_default()
            .insert(p_alt.to_bits());
        // DataAnalyzer writes no `pAlt` below the pad's.
        let standard = (height(pressure[k - 1]) - height(pad_pa)).max(0.0);
        result.worst_from_the_standard_m = result
            .worst_from_the_standard_m
            .max((standard - p_alt).abs());
    }
    result.clashes = by_pressure
        .values()
        .filter(|values| values.len() > 1)
        .count();
    // From the highest pressure down: zero at the pad's pressure and above, then rising.
    let mut previous = 0.0;
    for (bits, values) in by_pressure.iter().rev() {
        let value = values
            .iter()
            .map(|bits| f64::from_bits(*bits))
            .fold(f64::NAN, f64::max);
        let in_order = if f64::from_bits(*bits) >= pad_pa {
            value == 0.0
        } else {
            value > previous
        };
        if !in_order {
            result.inversions += 1;
        }
        previous = previous.max(value);
    }
    result
}

/// Missile Works RTx's `.rtx` files against the files of the same flight beside them, in their
/// folder, a file each ([`RtxFile`]), and the files that differ from the reader's record or an
/// export beside them, by id.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct RtxChecks {
    per_file: Vec<RtxFile>,
    differing: Vec<String>,
}

/// A table read by its header ([`hpr_flightdata::generic`]): its id, rows and columns, the
/// clock's unit, and its other columns counted by what the reader made of them: read in the unit
/// its name states; kept as written by the header (no unit the reader knows, a name that says
/// another quantity, an error or a difference, or a length, a time or a unit that may stand for
/// another under a name that doesn't say what it measures); kept as written
/// because a value contradicts the unit; and text, no channel (`events` counts the events taken
/// from it). Where two channels measure the same quantity under the same name in two units, such
/// as `alt(ft)` and `alt(m)`, `two_units_differ` is the largest difference between them, in SI,
/// over the rows where both have a value. Counts only: the public report names no column a
/// private file writes.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct TableFile {
    id: String,
    rows: usize,
    columns: usize,
    clock_unit: String,
    read: usize,
    kept_by_header: usize,
    contradicted: usize,
    text: usize,
    events: usize,
    two_units_differ: Option<f64>,
}

/// One Excel workbook's sheets: how many, and each sheet's outcome, the format it read as or the
/// kind of its refusal (`no-data` for a sheet with no value, such as one holding only a chart).
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct WorkbookFile {
    id: String,
    sheets: usize,
    outcomes: BTreeMap<String, usize>,
}

/// The workbooks' sheets, by outcome.
fn workbook_checks(samples: &[Sample]) -> Vec<WorkbookFile> {
    let mut out = Vec::new();
    for sample in samples.iter().filter(|s| s.family == WORKBOOK_FAMILY) {
        let Ok(sheets) = hpr_flightdata::workbook::read(&sample.bytes) else {
            continue;
        };
        let mut outcomes = BTreeMap::new();
        for sheet in &sheets {
            let outcome = match &sheet.log {
                Ok(log) => log.format.name().to_owned(),
                Err(error) => refusal(error).to_owned(),
            };
            *outcomes.entry(outcome).or_default() += 1;
        }
        out.push(WorkbookFile {
            id: hex(&Sha256::digest(&sample.bytes))[..16].to_owned(),
            sheets: sheets.len(),
            outcomes,
        });
    }
    out.sort_by(|a, b| a.id.cmp(&b.id));
    out
}

/// Each table read by its header, as [`TableFile`] counts it.
fn table_checks(samples: &[Sample]) -> Vec<TableFile> {
    use hpr_flightdata::generic;
    use hpr_flightdata::log::Quantity;
    let mut files = Vec::new();
    for sample in samples {
        let Ok(log) = hpr_flightdata::import::read(&sample.bytes) else {
            continue;
        };
        if log.format != LogFormat::GenericTable {
            continue;
        }
        let text = hpr_flightdata::import::decode(&sample.bytes);
        let Ok(mapping) = generic::mapping(&text) else {
            continue;
        };
        let mut file = TableFile {
            id: hex(&Sha256::digest(&sample.bytes))[..16].to_owned(),
            rows: log.channels.first().map_or(0, |c| c.values.len()),
            columns: mapping.columns.len(),
            clock_unit: mapping
                .columns
                .get(mapping.clock)
                .and_then(|c| c.unit.clone())
                .unwrap_or_default(),
            events: log.events.len(),
            ..TableFile::default()
        };
        // The channels follow the columns in order, the clock and the text columns left out.
        let mut channels = log.channels.iter().peekable();
        for (index, column) in mapping.columns.iter().enumerate() {
            if index == mapping.clock {
                continue;
            }
            match channels.peek() {
                Some(channel) if channel.column == column.name => {
                    match (column.unit.is_some(), channel.quantity) {
                        (true, Quantity::Unrecognized) => file.contradicted += 1,
                        (true, _) => file.read += 1,
                        (false, _) => file.kept_by_header += 1,
                    }
                    channels.next();
                }
                _ => file.text += 1,
            }
        }
        let base = |name: &str| {
            name.split(['(', '['])
                .next()
                .unwrap_or_default()
                .trim()
                .to_lowercase()
        };
        for (i, a) in log.channels.iter().enumerate() {
            for b in &log.channels[i + 1..] {
                if a.quantity != b.quantity
                    || a.quantity == Quantity::Unrecognized
                    || a.column == b.column
                    || base(&a.column) != base(&b.column)
                {
                    continue;
                }
                let largest = a
                    .values
                    .iter()
                    .zip(&b.values)
                    .filter(|(x, y)| x.is_finite() && y.is_finite())
                    .map(|(x, y)| (x - y).abs())
                    .fold(0.0_f64, f64::max);
                file.two_units_differ = Some(file.two_units_differ.unwrap_or(0.0).max(largest));
            }
        }
        files.push(file);
    }
    files.sort_by(|a, b| a.id.cmp(&b.id));
    files
}

/// One `.rtx`: its id; its records, decoded here from the bytes apart from the reader; whether the
/// reader's record holds the same fields, record for record; each CSV or text export beside it
/// ([`RtxExport`]); the KML beside it, if any ([`RtxExport`], its feet unchecked); and, reported
/// only, `SOG` over the speed between the fixes a second either side of it, and how far `COG` lies
/// from the bearing between them, the medians over the fixes where `SOG` is at least 15, and the
/// climb from the lowest of the 120 fixes before the highest to the highest over the highest
/// altitude of a PerfectFlite log beside it: ratios and differences only, as the figures themselves would give
/// a private flight's site and apogee.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct RtxFile {
    id: String,
    records: usize,
    reader_matches: bool,
    exports: Vec<RtxExport>,
    kml: Option<RtxExport>,
    sog_over_speed: Option<f64>,
    cog_off_bearing_deg: Option<f64>,
    climb_over_barometric: Option<f64>,
}

/// An export against its `.rtx`: its format's name; its rows; whether they are every record but
/// the last, in order, each row's time and numbers the record's and its position the record's or,
/// written in degrees, the record's cut toward zero at the sixth decimal, short of it by less than
/// 10⁻⁶°; the largest difference in a position, degrees; and whether every row's altitude is the
/// record's meters × 3.280 84 rounded down to a foot (for the KML, its meters as written).
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct RtxExport {
    format: String,
    rows: usize,
    every_record_but_the_last: bool,
    worst_position_deg: f64,
    altitude_as_written: bool,
}

/// Whether an `.rtx` differs from the reader's record of it or a file beside it.
fn rtx_differs(file: &RtxFile) -> bool {
    !file.reader_matches
        || file
            .exports
            .iter()
            .chain(&file.kml)
            .any(|e| !e.every_record_but_the_last || !e.altitude_as_written)
}

/// The `.rtx`'s records' 16 words, decoded from its bytes as the reader's module docs lay them
/// out: `_size` at byte 186, the slots from byte 204, each slot times 513,180,409 modulo 10⁹.
fn rtx_records(bytes: &[u8]) -> Vec<[u64; 16]> {
    let int =
        |at: usize| i32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]]);
    let size = usize::try_from(int(186)).unwrap_or(0);
    (0..size / 16)
        .map(|r| {
            let mut words = [0u64; 16];
            for (w, word) in words.iter_mut().enumerate() {
                let slot = u64::try_from(int(204 + 4 * (16 * r + w))).unwrap_or(u64::MAX);
                *word = slot.wrapping_mul(513_180_409) % 1_000_000_000;
            }
            words
        })
        .collect()
}

/// A record's fields as the RTx's software exports them, in the export's units: the time of day,
/// s, `NaN` if it isn't one; the latitude and longitude, degrees; then the altitude, m, `Velocity`, `VDOP`, `NavStatus`,
/// `Sats`, `SOG` and `COG`.
fn rtx_fields(words: &[u64; 16]) -> [f64; 10] {
    let hhmmss = words[1] * 65_536 + words[0];
    // A time that isn't a time of day, as a garbled record's, is a gap.
    let (hours, minutes, seconds) = (hhmmss / 10_000, hhmmss / 100 % 100, hhmmss % 100);
    let of_day = if hours < 24 && minutes < 60 && seconds <= 60 {
        (hours * 3_600 + minutes * 60 + seconds) as f64
    } else {
        f64::NAN
    };
    let degrees = |dm: u64, lo: u64, hi: u64, negative: bool| {
        // The decimals' leading 1 taken off before they are scaled, as the reader does.
        let minutes = (dm % 100) as f64 + (hi * 65_536 + lo).wrapping_sub(100_000) as f64 / 1e5;
        let value = (dm / 100) as f64 + minutes / 60.0;
        if negative { -value } else { value }
    };
    [
        of_day,
        degrees(
            words[2],
            words[3],
            words[4],
            words[11] & 0xff == u64::from(b'S'),
        ),
        degrees(
            words[5],
            words[6],
            words[7],
            words[11] >> 8 == u64::from(b'W'),
        ),
        words[8] as f64,
        words[9] as f64,
        words[10] as f64,
        (words[12] & 0xff) as f64,
        (words[12] >> 8) as f64,
        words[13] as f64,
        words[14] as f64,
    ]
}

/// [`RtxChecks`] over the samples.
fn rtx_checks(samples: &[Sample]) -> RtxChecks {
    let mut checks = RtxChecks::default();
    for sample in samples.iter().filter(|s| s.family == "missileworks-rtx") {
        // Only a file read as an `.rtx` has the layout its bytes are decoded by here.
        let Ok(log) = hpr_flightdata::import::read(&sample.bytes) else {
            continue;
        };
        if log.format != LogFormat::RtxRaw {
            continue;
        }
        let id = hex(&Sha256::digest(&sample.bytes))[..16].to_owned();
        let records: Vec<[f64; 10]> = rtx_records(&sample.bytes).iter().map(rtx_fields).collect();
        let beside = |formats: &[LogFormat]| -> Vec<FlightLog> {
            samples
                .iter()
                .filter(|other| other.path.parent() == sample.path.parent())
                .filter(|other| {
                    hpr_flightdata::import::detect(&other.bytes)
                        .is_some_and(|format| formats.contains(&format))
                })
                .filter_map(|other| hpr_flightdata::import::read(&other.bytes).ok())
                .collect()
        };
        // The log's clock counts from the first record's time of day.
        let zero = records
            .iter()
            .map(|r| r[0])
            .find(|t| !t.is_nan())
            .unwrap_or(f64::NAN);
        let reader_matches = rtx_matches(&log, &records, zero, records.len(), false).is_some();
        // An export beside it that is read as the RTx's but fails to read differs.
        let unread_exports = samples
            .iter()
            .filter(|other| other.path.parent() == sample.path.parent())
            .filter(|other| {
                hpr_flightdata::import::detect(&other.bytes)
                    .is_some_and(|f| matches!(f, LogFormat::RtxCsv | LogFormat::RtxText))
            })
            .filter(|other| hpr_flightdata::import::read(&other.bytes).is_err())
            .map(|other| RtxExport {
                format: hpr_flightdata::import::detect(&other.bytes)
                    .map_or("", |f| f.name())
                    .to_owned(),
                worst_position_deg: f64::NAN,
                ..RtxExport::default()
            });
        let exports = beside(&[LogFormat::RtxCsv, LogFormat::RtxText])
            .iter()
            .map(|export| {
                let worst = rtx_matches(
                    export,
                    &records,
                    zero,
                    records.len().saturating_sub(1),
                    true,
                );
                RtxExport {
                    format: export.format.name().to_owned(),
                    rows: export.stated.samples.unwrap_or(0),
                    every_record_but_the_last: worst.is_some(),
                    worst_position_deg: worst.unwrap_or(f64::NAN),
                    altitude_as_written: rtx_feet(export, &records),
                }
            })
            .chain(unread_exports)
            .collect();
        let kml = beside(&[LogFormat::Kml])
            .iter()
            .find(|kml| kml.logger == "Missile Works RTx")
            .map(|kml| rtx_against_kml(kml, &records));
        let (sog_over_speed, cog_off_bearing_deg) = rtx_motion(&records);
        let barometric = beside(&[LogFormat::PerfectFlitePf2])
            .iter()
            .flat_map(|pf2| pf2.altitude_m.clone())
            .fold(f64::NAN, f64::max);
        let climb_over_barometric = (barometric > 0.0).then(|| {
            let (peak_at, peak) = records
                .iter()
                .enumerate()
                .map(|(k, r)| (k, r[3]))
                .fold((0, f64::NEG_INFINITY), |a, b| if b.1 > a.1 { b } else { a });
            let pad = records[peak_at.saturating_sub(120)..=peak_at]
                .iter()
                .map(|r| r[3])
                .fold(f64::INFINITY, f64::min);
            (peak - pad) / barometric
        });
        checks.per_file.push(RtxFile {
            id,
            records: records.len(),
            reader_matches,
            exports,
            kml,
            sog_over_speed,
            cog_off_bearing_deg,
            climb_over_barometric,
        });
    }
    checks.differing = checks
        .per_file
        .iter()
        .filter(|file| rtx_differs(file))
        .map(|file| file.id.clone())
        .collect();
    checks.per_file.sort_by(|a, b| a.id.cmp(&b.id));
    for file in &mut checks.per_file {
        file.exports.sort_by(|a, b| a.format.cmp(&b.format));
    }
    checks.differing.sort();
    checks
}

/// The largest difference in a position, degrees, if a record holds the first `n` of the `.rtx`'s
/// records and no more, each field as decoded here: the time from `zero`, a gap where the
/// record's is, the numbers exactly, and the altitude and the positions exactly; or, for an
/// `export`, the altitude unchecked ([`rtx_feet`] checks its feet) and a position the record's
/// or the record's cut toward zero at the sixth decimal ([`rtx_cut`]). A time of day after
/// midnight isn't in the sample files and isn't checked here.
fn rtx_matches(
    log: &FlightLog,
    records: &[[f64; 10]],
    zero: f64,
    n: usize,
    export: bool,
) -> Option<f64> {
    use hpr_flightdata::log::Quantity;
    let by_quantity = |quantity: Quantity, k: usize| log.channels_of(quantity).nth(k);
    // Each field's channel, by the order the readers write them in.
    let fields = [
        (by_quantity(Quantity::Latitude, 0), 1),
        (by_quantity(Quantity::Longitude, 0), 2),
        (by_quantity(Quantity::Unrecognized, 0), 4),
        (by_quantity(Quantity::Number, 0), 5),
        (by_quantity(Quantity::Number, 1), 6),
        (by_quantity(Quantity::Number, 2), 7),
        (by_quantity(Quantity::Unrecognized, 1), 8),
        (by_quantity(Quantity::Angle, 0), 9),
    ];
    let altitude = by_quantity(Quantity::GnssAltitude, 0);
    let time = altitude.map(|c| &c.time_s)?;
    if time.len() != n || records.len() < n {
        return None;
    }
    let mut worst: f64 = 0.0;
    let close = |a: f64, b: f64, k: usize, worst: &mut f64| {
        if k == 1 || k == 2 {
            *worst = worst.max((a - b).abs());
            a == b || (export && rtx_cut(a, b))
        } else {
            (a - b).abs() <= 1e-9
        }
    };
    let matches = (0..n).all(|r| {
        let record = &records[r];
        let t = record[0] - zero;
        (time[r] == t || (time[r] - t).abs() < 1e-9 || (time[r].is_nan() && t.is_nan()))
            && (export || altitude.is_some_and(|c| c.values[r] == record[3]))
            && fields.iter().all(|(channel, k)| {
                channel.is_some_and(|c| {
                    c.values.len() == n && close(c.values[r], record[*k], *k, &mut worst)
                })
            })
    });
    matches.then_some(worst)
}

/// Whether a position written in degrees is the true one cut toward zero at the sixth decimal:
/// short of it, away from zero's side, by less than 10⁻⁶°, within 10⁻⁹° for the reading's
/// rounding.
fn rtx_cut(written: f64, truth: f64) -> bool {
    let short = truth.abs() - written.abs();
    written.signum() == truth.signum() && (-1e-9..1e-6 + 1e-9).contains(&short)
}

/// Whether every row of an export writes its record's altitude as its meters × 3.280 84 rounded
/// down to a foot.
fn rtx_feet(export: &FlightLog, records: &[[f64; 10]]) -> bool {
    use hpr_flightdata::log::Quantity;
    export
        .channels_of(Quantity::GnssAltitude)
        .next()
        .is_some_and(|c| {
            c.values
                .iter()
                .zip(records)
                .all(|(m, record)| (m / 0.3048 - (record[3] * 3.280_84).floor()).abs() < 1e-6)
        })
}

/// The RTx's KML against its `.rtx`: its points every record but the last, its positions the
/// record's cut toward zero at the sixth decimal ([`rtx_cut`]), and its altitude the record's
/// meters as written.
fn rtx_against_kml(kml: &FlightLog, records: &[[f64; 10]]) -> RtxExport {
    use hpr_flightdata::log::Quantity;
    let column = |quantity: Quantity| kml.channels_of(quantity).next().map(|c| c.values.clone());
    let (latitude, longitude, altitude) = (
        column(Quantity::Latitude).unwrap_or_default(),
        column(Quantity::Longitude).unwrap_or_default(),
        column(Quantity::GnssAltitude).unwrap_or_default(),
    );
    let rows = latitude.len();
    let n = records.len().saturating_sub(1);
    let worst_position_deg = (0..n.min(rows).min(longitude.len()))
        .map(|r| {
            (latitude[r] - records[r][1])
                .abs()
                .max((longitude[r] - records[r][2]).abs())
        })
        .fold(0.0, f64::max);
    RtxExport {
        format: LogFormat::Kml.name().to_owned(),
        rows,
        every_record_but_the_last: rows == n
            && longitude.len() == n
            && (0..n).all(|r| {
                rtx_cut(latitude[r], records[r][1]) && rtx_cut(longitude[r], records[r][2])
            }),
        worst_position_deg,
        altitude_as_written: altitude.len() == n && (0..n).all(|r| altitude[r] == records[r][3]),
    }
}

/// `SOG` over the speed between the fixes either side of it, m/s, and how far `COG` lies from the
/// bearing between them, degrees, either way: the medians over the fixes where `SOG` is at least
/// 15 and the fixes either side are times of day 2 s apart, on a sphere of 6,371 km; `None` with
/// none.
fn rtx_motion(records: &[[f64; 10]]) -> (Option<f64>, Option<f64>) {
    let (mut ratios, mut offsets) = (Vec::new(), Vec::new());
    for k in 1..records.len().saturating_sub(1) {
        let (before, here, after) = (&records[k - 1], &records[k], &records[k + 1]);
        // A gap's `NaN` time is no time of day: the fixes around it are skipped.
        if here[8] < 15.0
            || !(before[0].is_finite() && after[0].is_finite())
            || (after[0] - before[0] - 2.0).abs() > 1e-9
        {
            continue;
        }
        let meters_per_degree = 6_371_000.0 * std::f64::consts::PI / 180.0;
        let north = (after[1] - before[1]) * meters_per_degree;
        let east = (after[2] - before[2]) * meters_per_degree * before[1].to_radians().cos();
        let speed = north.hypot(east) / 2.0;
        if speed > 0.0 {
            ratios.push(here[8] / speed);
            let bearing = east.atan2(north).to_degrees().rem_euclid(360.0);
            offsets.push(((here[9] - bearing + 180.0).rem_euclid(360.0) - 180.0).abs());
        }
    }
    let median = |mut values: Vec<f64>| {
        values.sort_by(f64::total_cmp);
        (!values.is_empty()).then(|| values[values.len() / 2])
    };
    (median(ratios), median(offsets))
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
    let mut sheets_dir = None;
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--check" => check = true,
            "--sheets" => {
                sheets_dir = Some(
                    args.next()
                        .ok_or(format!("--sheets needs a folder\n\n{USAGE}"))?
                        .clone(),
                );
            }
            other => return Err(format!("unknown argument `{other}`\n\n{USAGE}")),
        }
    }
    let root = crate::designs::root()?;
    let (samples, sources) = samples(&root)?;
    if let Some(dir) = sheets_dir {
        return write_sheets(&samples, Path::new(&dir));
    }
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

/// Each workbook sample as `<id>.xlsx` in `dir`, and its sheets' text as `<id>/<n>.csv`, `n`
/// from 0 in the workbook's order, for an outside reader to check the text against.
fn write_sheets(samples: &[Sample], dir: &Path) -> Result<(), String> {
    // The sheets are other people's data: only under `refs/`, which git ignores.
    if !dir.starts_with("refs") && !dir.starts_with(crate::designs::root()?.join("refs")) {
        return Err(format!(
            "--sheets writes private data: give a folder under refs/, not {}",
            dir.display()
        ));
    }
    let mut books = 0;
    for sample in samples.iter().filter(|s| s.family == WORKBOOK_FAMILY) {
        let id = hex(&Sha256::digest(&sample.bytes))[..16].to_owned();
        let sheets = hpr_flightdata::workbook::sheets(&sample.bytes)
            .map_err(|error| format!("{id}: {error}"))?;
        let folder = dir.join(&id);
        fs::create_dir_all(&folder).map_err(|error| format!("{}: {error}", folder.display()))?;
        let book = dir.join(format!("{id}.xlsx"));
        fs::write(&book, &sample.bytes).map_err(|error| format!("{}: {error}", book.display()))?;
        for (n, sheet) in sheets.iter().enumerate() {
            let path = folder.join(format!("{n}.csv"));
            fs::write(&path, &sheet.text)
                .map_err(|error| format!("{}: {error}", path.display()))?;
        }
        books += 1;
    }
    println!(
        "logs: wrote {books} workbooks and their sheets to {}",
        dir.display()
    );
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
    // The archives' files, after every source, so that a file an archive shares with a source
    // is that source's.
    let mut files = 0;
    let mut archived = Vec::new();
    for archive in samples
        .iter()
        .filter(|sample| sample.family == ARCHIVE_FAMILY)
    {
        let members = hpr_flightdata::archive::unpack(&archive.bytes)
            .map_err(|error| format!("{}: {error}", archive.path.display()))?;
        for member in members {
            files += 1;
            let hash = hex(&Sha256::digest(&member.bytes));
            if !seen.insert(hash.clone()) {
                continue;
            }
            let Some((_, family)) = ARCHIVED.iter().find(|(id, _)| hash.starts_with(id)) else {
                return Err(format!(
                    "{} in {} has no family in ARCHIVED",
                    &hash[..16],
                    archive.path.display()
                ));
            };
            met.insert(hash[..16].to_owned());
            // Under the archive's own path, folders and all, but never outside it: the checks
            // that look beside a file look only among the archive's files.
            let mut path = archive.path.clone();
            path.extend(
                member
                    .name
                    .split(['/', '\\'])
                    .filter(|part| !matches!(*part, "" | "." | "..")),
            );
            archived.push(Sample {
                path,
                source: "archives",
                family: (*family).to_owned(),
                bytes: member.bytes,
            });
        }
    }
    for (id, _) in ARCHIVED {
        if !met.contains(id) {
            return Err(format!(
                "the archived file {id} isn't in any sample archive"
            ));
        }
    }
    sources.push(SourceCount {
        source: "archives".to_owned(),
        files,
        distinct: archived.len(),
    });
    samples.extend(archived);
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
        } else if sample.family == ARCHIVE_FAMILY {
            ARCHIVE_INCREMENT
        } else if sample.family == WORKBOOK_FAMILY {
            WORKBOOK_INCREMENT
        } else if sample.family.ends_with("-damaged") {
            "damaged"
        } else if NOT_LOGS.contains(&sample.family.as_str()) {
            "not a log"
        } else {
            LEFT_TO
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
        if sample.family == ARCHIVE_FAMILY {
            let (outcome, record) = match hpr_flightdata::archive::unpack(&sample.bytes) {
                Ok(members) => {
                    row.read += 1;
                    // The archive's record: its files' ids, in order.
                    let ids: Vec<String> = members
                        .iter()
                        .map(|member| hex(&Sha256::digest(&member.bytes))[..16].to_owned())
                        .collect();
                    (
                        format!("Archive:{}", members.len()),
                        hex(&Sha256::digest(ids.join(",").as_bytes()))[..16].to_owned(),
                    )
                }
                Err(error) => {
                    let kind = refusal(&error);
                    *row.refused.entry(kind.to_owned()).or_default() += 1;
                    unread.push(format!(
                        "  {} ({}, {id}): {error}",
                        sample.path.display(),
                        sample.family
                    ));
                    (format!("refused:{kind}"), "-".to_owned())
                }
            };
            files.push(format!(
                "{id} {} {} {outcome} {record}",
                sample.family, sample.source
            ));
            continue;
        }
        if sample.family == WORKBOOK_FAMILY {
            // A workbook is read when a sheet of it reads; its record is each sheet's, in order.
            let (outcome, record) = match hpr_flightdata::workbook::read(&sample.bytes) {
                Ok(sheets) if sheets.iter().any(|sheet| sheet.log.is_ok()) => {
                    row.read += 1;
                    let mut records = Vec::new();
                    for sheet in &sheets {
                        records.push(match &sheet.log {
                            Ok(log) => {
                                *loggers.entry(log.logger.clone()).or_default() += 1;
                                let json = serde_json::to_vec(log).unwrap_or_default();
                                format!("{:?}:{}", log.format, &hex(&Sha256::digest(&json))[..16])
                            }
                            Err(error) => refusal(error).to_owned(),
                        });
                    }
                    (
                        format!("Workbook:{}", sheets.len()),
                        hex(&Sha256::digest(records.join(",").as_bytes()))[..16].to_owned(),
                    )
                }
                outcome => {
                    let error = match outcome {
                        Err(error) => error.to_string(),
                        Ok(_) => "no sheet of it reads".to_owned(),
                    };
                    *row.refused.entry("workbook".to_owned()).or_default() += 1;
                    unread.push(format!(
                        "  {} ({}, {id}): {error}",
                        sample.path.display(),
                        sample.family
                    ));
                    ("refused:workbook".to_owned(), "-".to_owned())
                }
            };
            files.push(format!(
                "{id} {} {} {outcome} {record}",
                sample.family, sample.source
            ));
            continue;
        }
        let (outcome, record) = match hpr_flightdata::import::read(&sample.bytes) {
            Ok(log) => {
                row.read += 1;
                // The table reader is tried last and takes any table naming a clock: a file it
                // reads whose family isn't listed as one, a not-a-log file among them, is flagged.
                let stray_table = log.format == LogFormat::GenericTable
                    && expected != Some(LogFormat::GenericTable);
                if expected.is_some_and(|format| format != log.format)
                    || increment == "damaged"
                    || stray_table
                {
                    unread.push(format!(
                        "  {} ({}, {id}): read as {}, where its family {}",
                        sample.path.display(),
                        sample.family,
                        log.format.name(),
                        match expected {
                            Some(format) => format!("reads as {}", format.name()),
                            None if increment == "damaged" => {
                                "is listed as damaged, so the list is stale".to_owned()
                            }
                            None => format!("is listed as {increment}, not as a table"),
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
                fixtures and samples, AerospaceNU's logs, and the files inside the zip archives among them; refs/research/public-flights, a \
                download cache whose flights' logs the collection catalogs, and RocketPy's \
                processed flight tables are not loggers' files and are left out. A file's id is the first 16 hex digits of its \
                SHA-256, its record the first 16 of the SHA-256 of the JSON record read from it."
            .to_owned(),
        sources,
        families: families.into_values().collect(),
        perfectflite_exports: export_units(samples),
        pfa_checks,
        raw_checks: raw_checks(samples),
        summary_checks: summary_checks(samples),
        telemetry_checks: telemetry_checks(samples),
        eeprom_checks: eeprom_checks(samples),
        arts2_checks: arts2_checks(samples),
        rtx_checks: rtx_checks(samples),
        table_checks: table_checks(samples),
        workbook_checks: workbook_checks(samples),
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
    // Telemetry that differs from AltosUI's export of the same flight is as unread as a refusal.
    for id in &report.telemetry_checks.differing {
        unread.push(format!(
            "  {id} (altos-telem): differs from every AltosUI export beside it"
        ));
    }
    for id in &report.telemetry_checks.inconsistent {
        unread.push(format!(
            "  {id} (altos-telem): its board's height, speed and acceleration, or its height and \
             the export's, aren't in one another's units"
        ));
    }
    for id in &report.telemetry_checks.unchecked {
        unread.push(format!(
            "  {id} (altos-telem): too few fast packets to check its board's height, speed and \
             acceleration against each other"
        ));
    }
    // A raw download that differs from AltosUI's export or the telemetry of its flight is as
    // unread as a refusal.
    for id in &report.eeprom_checks.differing {
        unread.push(format!(
            "  {id} (altos-eeprom): differs from the AltosUI export or the telemetry of its \
             flight beside it"
        ));
    }
    // An `.odf` that differs from the raw CSV or the interpreted export of its flight is as
    // unread as a refusal.
    for id in &report.arts2_checks.differing {
        unread.push(format!(
            "  {id} (odf): differs from the raw CSV or the interpreted export of its flight beside it"
        ));
    }
    // An `.rtx` that differs from its reader's record or the exports of its flight is as unread
    // as a refusal.
    for id in &report.rtx_checks.differing {
        unread.push(format!(
            "  {id} (missileworks-rtx): differs from its record, or from an export of its flight \
             beside it"
        ));
    }
    // A summary whose lengths agree with the file beside it only as meters contradicts the feet
    // it writes.
    for (logger, checks) in &report.summary_checks {
        for id in &checks.contradicted {
            unread.push(format!(
                "  {id} ({logger}'s summary): agrees with a file beside it only were its feet \
                 meters"
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
    let mut by_id = Vec::new();
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
            let in_meters = climb / (2.0 * top / hpr_flightdata::perfectflite::FOOT_M / G0).sqrt();
            feet.push(in_feet);
            meters.push(in_meters);
            by_id.push(format!(
                "{} {in_feet:.2} {in_meters:.2}",
                &hex(&Sha256::digest(&sample.bytes))[..16]
            ));
        }
    }
    by_id.sort();
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
        climb_over_fall_others_by_id: by_id,
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
        LogError::Archive { .. } => "archive",
        LogError::Workbook { .. } => "workbook",
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
    if !units.climb_over_fall_others_by_id.is_empty() {
        let each: Vec<String> = units
            .climb_over_fall_others_by_id
            .iter()
            .filter_map(|line| {
                let mut parts = line.split(' ');
                Some(format!(
                    "`{}` {} and {}",
                    parts.next()?,
                    parts.next()?,
                    parts.next()?
                ))
            })
            .collect();
        out += &format!(
            "By id, those {}, in feet and in meters: {}.\n",
            each.len(),
            each.join("; ")
        );
    }
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
                precision, which runs a hundredth of a second ahead of the RRC3's from 156.65 s). Each \
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
    if !report.summary_checks.is_empty() {
        out += "\nThe Featherweight summaries against the files of samples exported beside each, \
                in its folder, that their reader reads: a Blue Raven's low-rate CSV, a GPS \
                tracker's CSV, each holding an altitude. The stated maximum altitude is compared with a file's highest \
                altitude above the pad (for a tracker, its highest GNSS altitude less the pad \
                altitude the summary states), and the stated pad altitude with the median of a \
                file's first 20 altitudes above sea level, each in SI; a figure agrees within 10%, \
                and its ratio, summary over file, is the one nearest 1. A folder may hold other \
                flights' files, so a summary may agree with none; one that would agree only were \
                its feet meters contradicts its unit and fails the check:\n\n\
                | Logger | Summaries | With a file beside | Stating an apogee | Apogee agrees | \
                Its ratios | Pad agrees | Its ratios | Agreeing in neither | Contradicted |\n\
                |---|---:|---:|---:|---:|---|---:|---|---|---|\n";
        let range = |range: &Option<[f64; 2]>| match range {
            Some([least, greatest]) => format!("{least:.4} to {greatest:.4}"),
            None => "none".to_owned(),
        };
        let ids = |ids: &[String]| {
            if ids.is_empty() {
                "none".to_owned()
            } else {
                ids.join(", ")
            }
        };
        for (logger, checks) in &report.summary_checks {
            out += &format!(
                "| {logger} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
                checks.files,
                checks.with_a_file_beside,
                checks.stating_an_apogee,
                checks.apogee_agrees,
                range(&checks.apogee_ratios),
                checks.pad_agrees,
                range(&checks.pad_ratios),
                ids(&checks.agreeing_in_neither),
                ids(&checks.contradicted)
            );
        }
    }
    let telemetry = &report.telemetry_checks;
    if telemetry.files > 0 {
        let files: String = telemetry
            .per_file
            .iter()
            .map(|file| {
                format!(
                    "| {} | {} | {} | {:.1}% | {} | {} |\n",
                    file.id,
                    file.packets_heard,
                    file.failing_the_crc,
                    100.0 * file.failing_the_crc as f64 / file.packets_heard.max(1) as f64,
                    file.power_ons,
                    if file.pressure_in_pascals {
                        "yes"
                    } else {
                        "no"
                    }
                )
            })
            .collect();
        let range = |range: &Option<[f64; 2]>| match range {
            Some([least, greatest]) => format!("{least:.4} to {greatest:.4}"),
            None => "none".to_owned(),
        };
        let ids = |ids: &[String]| {
            if ids.is_empty() {
                "none".to_owned()
            } else {
                ids.join(", ")
            }
        };
        out += &format!(
            "\nAltOS's radio telemetry, each file's packets as its record counts them:\n\n\
             | File | Packets heard | Failing the radio's CRC | Share | Power-ons | Pressure \
             in pascals |\n|---|---:|---:|---:|---:|---|\n{}\n\
             Against AltosUI's exports of the same flight, an export being an AltOS CSV beside \
             the telemetry, in its folder, that its reader reads. The two clocks are the \
             board's tick, which turns over every 655.36 s; their `boost` states say by how many \
             turns they differ. At each tick both hold, the telemetry's pressure must equal the \
             export's, but where the export's rows there repeat the pressure of the row before \
             them, which are counted apart; at the pressures equal, the temperatures must agree \
             within 0.05 K, half the export's step, and at every tick both hold a GPS fix, the \
             positions within 0.01° and the GPS altitudes within 100 m, the export's row \
             carrying its last fix; fixes on both sides that never meet fail. Of {} telemetry \
             files, {} have an export beside them and {} agree with one: of their {} sensor \
             packets, {} fall at a tick the export holds, {} pressures equal and {} at repeated \
             rows; worst temperature {:.3} K; {} GPS values compared, worst position {:.5}°, \
             GPS altitude {:.0} m; differing: {}.\n\n\
             The board's own estimates, sent in its telemetry, against each other: the median \
             ratio of `height`'s climb to `speed` after burnout, from {}, and of `speed`'s \
             change to `acceleration` in the climb, from {}; and the telemetry's highest \
             `height` over the export's, from {}. Each is 1 if the three are in the units read, \
             and must lie from 0.8 to 1.25; outside: {}; too few fast packets for a ratio: \
             {}.\n",
            files,
            telemetry.files,
            telemetry.with_an_export_beside,
            telemetry.agreeing_with_an_export,
            telemetry.sensor_packets_beside_an_export,
            telemetry.at_the_exports_ticks,
            telemetry.pressures_equal,
            telemetry.pressures_at_repeated_rows,
            telemetry.worst_temperature_k,
            telemetry.gps_values_compared,
            telemetry.worst_position_deg,
            telemetry.worst_gps_altitude_m,
            ids(&telemetry.differing),
            range(&telemetry.climb_over_speed),
            range(&telemetry.speed_change_over_acceleration),
            range(&telemetry.height_over_export),
            ids(&telemetry.inconsistent),
            ids(&telemetry.unchecked)
        );
    }
    let eeprom = &report.eeprom_checks;
    if eeprom.files > 0 {
        let pair = |n: usize, worst: f64, unit: &str, digits: usize| {
            if n == 0 {
                "–".to_owned()
            } else {
                format!("{n}, {worst:.digits$} {unit}")
            }
        };
        let files: String = eeprom
            .per_file
            .iter()
            .map(|file| {
                let (pressure, temperature, acceleration, voltage, rows) = match &file.export {
                    Some(e) => (
                        pair(e.pressures, e.worst_pressure_pa, "Pa", 4),
                        pair(e.temperatures, e.worst_temperature_k, "K", 3),
                        format!(
                            "{}{}",
                            pair(e.accelerations, e.worst_acceleration_m_s2, "m/s²", 4),
                            if e.zeroed_at_the_plus_calibration {
                                " (zeroed at +1 g)"
                            } else {
                                ""
                            }
                        ),
                        pair(e.voltages, e.worst_voltage_v, "V", 4),
                        format!("{}, {} without", e.rows, e.rows_without_a_sample),
                    ),
                    None => (
                        "–".to_owned(),
                        "–".to_owned(),
                        "–".to_owned(),
                        "–".to_owned(),
                        "–".to_owned(),
                    ),
                };
                let telemetry = match &file.telemetry {
                    Some(t) => format!("{} of {}", t.pressures_equal, t.ticks_shared),
                    None => "–".to_owned(),
                };
                format!(
                    "| {} | {} | {}{} | {} | {} | {} | {} | {} | {} | {} | {} | {pressure} | \
                     {temperature} | {acceleration} | {voltage} | {rows} | {telemetry} |\n",
                    file.id,
                    file.form,
                    file.log_format,
                    if file.format_from_product {
                        " (from the product)"
                    } else {
                        ""
                    },
                    file.records,
                    if file.form == "JSON" {
                        format!("{}, 0 failing", file.checksums_checked)
                    } else {
                        "none printed".to_owned()
                    },
                    file.erased,
                    file.duplicates_dropped,
                    file.after_a_silence,
                    if file.last_line_cut { "yes" } else { "no" },
                    file.smallest_pressure_step_pa
                        .map_or("–".to_owned(), |step| format!("{step:.2} Pa")),
                    file.climb_ratio
                        .map_or("–".to_owned(), |ratio| format!("{ratio:.3}")),
                )
            })
            .collect();
        let ids = |ids: &[String]| {
            if ids.is_empty() {
                "none".to_owned()
            } else {
                ids.join(", ")
            }
        };
        out += &format!(
            "\nAltOS's raw downloads, `.eeprom`, each file's records as its record counts them, \
             and against AltosUI's export and the board's radio telemetry of the same flight \
             where they lie beside it:\n\n\
             | File | Form | Log format | Records | Checksums | Erased | Repeats dropped | After \
             a silence | Last line cut | Pressure step | Climb ratio | Pressures vs export | \
             Temperatures | Accelerations | Mega voltages | Export rows, without a sample | \
             Pressures vs telemetry |\n\
             |---|---|---|---:|---|---:|---:|---:|---|---:|---:|---|---|---|---|---|---|\n{}\n\
             An export of the same flight is an AltosUI CSV in the file's folder whose `serial` \
             and `flight` are the file's, read by its header's names apart from HPR Sim's AltOS \
             CSV reader. The two line up by tick, the export's `time` counting from the flight \
             record's tick, or, where it equals its `clock` at every row, from the tick itself. \
             At each of the file's samples the export's last row at that hundredth must agree \
             within half the export's print step (each cell above: the values compared and the \
             worst difference): the pressure, the temperature where an MS5607 gives it, the \
             acceleration, and a Mega's pyro, igniter, apogee and main voltages. An export whose \
             acceleration differs from the file's by one constant, the zero AltosUI may set at \
             the +1 g calibration instead of the flight record's ground reading, within half a \
             step, is counted apart. The other way, every row of the export must fall at a tick \
             where the file holds a sample of some channel. The telemetry is the board's `.telem` \
             of the same serial number, its clock lined up by whole turns from the two `boost` \
             events; every pressure at a tick a sensor packet holds must equal it. The pressure \
             step is the smallest difference between two of the file's pressures. The climb \
             ratio is the reader's climb check on a board with an accelerometer: the climb over \
             the first second after boost by the pressure in the standard atmosphere over the \
             climb by the acceleration integrated twice from rest, which the reader holds from \
             0.6 to 5/3, keeping the acceleration's counts as written outside it, and a \
             TeleMetrum v1's pressure's too; inside it, over {} files, it lies from {}; outside: \
             {}. A log format from \
             the product is format 1 for a TeleMetrum v1 whose configuration states none. Of {} \
             files, {} have an export beside them and {} agree with it; zeroed at the +1 g \
             calibration: {}; {} have telemetry beside them and {} agree with it; differing: \
             {}.\n",
            files,
            eeprom.with_a_climb_ratio,
            match eeprom.climb_ratios {
                Some([least, greatest]) => format!("{least:.3} to {greatest:.3}"),
                None => "none".to_owned(),
            },
            ids(&eeprom.climb_outside_the_band),
            eeprom.files,
            eeprom.with_an_export_beside,
            eeprom.agreeing_with_an_export,
            ids(&eeprom.zeroed_at_the_plus_calibration),
            eeprom.with_telemetry_beside,
            eeprom.agreeing_with_telemetry,
            ids(&eeprom.differing)
        );
    }
    let arts2 = &report.arts2_checks;
    if !arts2.per_file.is_empty() {
        let kpa = |difference: Option<f64>| {
            difference.map_or("–".to_owned(), |difference| format!("{difference:+.2}"))
        };
        let rows: String = arts2
            .per_file
            .iter()
            .map(|file| {
                let export = match &file.export {
                    Some(e) => format!(
                        "{} | {:.3} | {} | {}{} | {} | {} | {} | {:.1}",
                        e.rows_compared,
                        e.worst_acceleration_counts,
                        if e.first_row_zero { "yes" } else { "no" },
                        e.rows_after,
                        if e.rows_after_zero {
                            ", zero"
                        } else {
                            ", not zero"
                        },
                        e.gaps,
                        e.clashes,
                        e.inversions,
                        e.worst_from_the_standard_m
                    ),
                    None => "– | – | – | – | – | – | – | –".to_owned(),
                };
                format!(
                    "| {} | {} | {} | {} | {export} | {} | {} |\n",
                    file.id,
                    file.records,
                    file.climb_ratio
                        .map_or("–".to_owned(), |ratio| format!("{ratio:.3}")),
                    match file.raw_csv_matches {
                        Some(true) => "same",
                        Some(false) => "differs",
                        None => "–",
                    },
                    kpa(file.altos_pad_kpa),
                    kpa(file.altos_least_kpa)
                )
            })
            .collect();
        out += &format!(
            "\nARTS2's `.odf` files, each against the files of its flight beside it: its raw CSV, \
             word for word; DataAnalyzer's interpreted export, `Acc` in the accelerometer's counts \
             on every record after the first, and `pAlt` against the pressure one record \
             earlier, as DataAnalyzer takes it, by order only (each pressure meeting one `pAlt`, \
             zero at the pad's pressure and above and rising as the pressure falls below it, and \
             no `pAlt` a gap), its difference from the standard atmosphere's \
             reported only, as one export takes its altitude otherwise; and the pad's and the \
             least pressure less an AltOS board's flown with it, kPa, reported only:\n\n\
             | File | Records | Climb ratio | Raw CSV | Rows compared | Worst `Acc`, counts | \
             First row zero | Rows after | `pAlt` gaps | Clashes | Inversions | `pAlt` from the \
             standard, m | Pad less AltOS, kPa | Least less AltOS, kPa |\n\
             |---|---:|---:|---|---:|---:|---|---|---:|---:|---:|---:|---:|---:|\n{rows}\n\
             Differing: {}.\n",
            if arts2.differing.is_empty() {
                "none".to_owned()
            } else {
                arts2.differing.join(", ")
            }
        );
    }
    let rtx = &report.rtx_checks;
    if !rtx.per_file.is_empty() {
        let figure = |value: Option<f64>, digits: usize| {
            value.map_or("–".to_owned(), |value| format!("{value:.digits$}"))
        };
        let yes = |value: bool| if value { "yes" } else { "no" };
        let rows: String = rtx
            .per_file
            .iter()
            .map(|file| {
                let exports: Vec<String> = file
                    .exports
                    .iter()
                    .chain(&file.kml)
                    .map(|e| {
                        format!(
                            "{}: {} rows, {}, {:.2} × 10⁻⁶°, altitude {}",
                            e.format,
                            e.rows,
                            if e.every_record_but_the_last {
                                "every record but the last"
                            } else {
                                "differs"
                            },
                            e.worst_position_deg * 1e6,
                            if e.altitude_as_written {
                                "as written"
                            } else {
                                "differs"
                            }
                        )
                    })
                    .collect();
                format!(
                    "| {} | {} | {} | {} | {} | {} | {} |\n",
                    file.id,
                    file.records,
                    yes(file.reader_matches),
                    if exports.is_empty() {
                        "–".to_owned()
                    } else {
                        exports.join("; ")
                    },
                    figure(file.sog_over_speed, 2),
                    figure(file.cog_off_bearing_deg, 1),
                    figure(file.climb_over_barometric, 3)
                )
            })
            .collect();
        out += &format!(
            "\nMissile Works RTx's `.rtx` files, each decoded here from its bytes apart from the \
             reader and held to the reader's record, field for field, and to the exports of its \
             flight beside it: the CSV and text exports' rows and the KML's points every record \
             but the last, their times and numbers equal, their positions equal or, written in \
             degrees, cut toward zero at the sixth decimal (the largest difference given), their \
             altitude the record's meters × 3.280 84 rounded down to a foot (the KML's, its \
             meters as written). Reported only, the medians over the fixes where `SOG` is at \
             least 15 of `SOG` over the speed between the fixes either side, m/s, and of how far \
             `COG` lies from the bearing between them, degrees, either way; and the climb from the lowest of the 120 \
             fixes before the highest to the highest, over the highest altitude of a PerfectFlite \
             log flown beside the tracker:\n\n\
             | File | Records | Reader's record | Exports beside it | `SOG` over speed | `COG` \
             off bearing, ° | Climb over barometric |\n\
             |---|---:|---|---|---:|---:|---:|\n{rows}\nDiffering: {}.\n",
            if rtx.differing.is_empty() {
                "none".to_owned()
            } else {
                rtx.differing.join(", ")
            }
        );
    }
    if !report.table_checks.is_empty() {
        let tables = &report.table_checks;
        let sum = |field: fn(&TableFile) -> usize| tables.iter().map(field).sum::<usize>();
        let mut rows = String::new();
        for table in tables {
            rows += &format!(
                "| `{}` | {} | {} | `{}` | {} | {} | {} | {} | {} | {} |\n",
                table.id,
                table.rows,
                table.columns,
                table.clock_unit,
                table.read,
                table.kept_by_header,
                table.contradicted,
                table.text,
                table.events,
                table
                    .two_units_differ
                    .map_or_else(|| "–".to_owned(), |d| format!("{d:.2e}"))
            );
        }
        out += &format!(
            "\nThe {} tables read by their headers (M7.1c13): each column but the clock read in the \
             unit its name states, kept as written by its header (no unit the reader knows, a name \
             that says another quantity, an error or a difference, or a length, a time or a unit \
             that may stand for another under a name that doesn't say what it measures), kept as \
             written because a value contradicts its unit, or text; *Two units* is \
             the largest difference, in SI, between two channels of one quantity under one name in \
             two units. In all, {} columns read in a stated unit, {} kept by their headers, {} by \
             their values, {} of text.\n\n\
             | File | Rows | Columns | Clock | Read | Kept by header | Contradicted | Text | \
             Events | Two units |\n\
             |---|---:|---:|---|---:|---:|---:|---:|---:|---:|\n{rows}",
            tables.len(),
            sum(|t| t.read),
            sum(|t| t.kept_by_header),
            sum(|t| t.contradicted),
            sum(|t| t.text),
        );
    }
    if !report.workbook_checks.is_empty() {
        let books = &report.workbook_checks;
        let mut totals: BTreeMap<&str, usize> = BTreeMap::new();
        let mut rows = String::new();
        for book in books {
            let outcomes: Vec<String> = book
                .outcomes
                .iter()
                .map(|(outcome, n)| {
                    *totals.entry(outcome).or_default() += n;
                    format!("{n} {outcome}")
                })
                .collect();
            rows += &format!(
                "| `{}` | {} | {} |\n",
                book.id,
                book.sheets,
                outcomes.join(", ")
            );
        }
        let totals: Vec<String> = totals
            .iter()
            .map(|(outcome, n)| format!("{n} {outcome}"))
            .collect();
        out += &format!(
            "\nThe {} Excel workbooks (M7.1c14): each sheet's cells written out as text, as the \
             workbook stores them, and read as a file of its own; a sheet's outcome is the layout \
             it reads as, or why it doesn't (*no-data* for a sheet with no value, such as one \
             holding only a chart). In all, {} sheets: {}.\n\n\
             | File | Sheets | Outcomes |\n|---|---:|---|\n{rows}",
            books.len(),
            books.iter().map(|book| book.sheets).sum::<usize>(),
            totals.join(", ")
        );
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
/// per hour, from the altitude``, or a Featherweight summary's [`UNKNOWN_FIGURE`].
fn unit_decision(note: &str) -> Option<&str> {
    let (decision, _) = note.split_once(": ")?;
    ((decision.starts_with('`')
        && (decision.contains("` read in ") || decision.contains("` kept as written")))
        || decision == UNKNOWN_FIGURE)
        .then_some(decision)
}

/// A Featherweight summary's note on a figure its reader doesn't know, before its colon: the name
/// comes after, so the public report counts the note without quoting what a file wrote.
const UNKNOWN_FIGURE: &str = "a figure HPR Sim doesn't know kept as written";

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
    fn an_empty_or_ragged_channel_is_a_difference_not_a_panic() {
        let ticks = [0.0, 0.00125, 0.0025];
        let one_clock = |values: &[f64]| {
            record(
                "raven_fip_csv",
                &[
                    ("Baro (Atm)", "pressure", &ticks, values),
                    ("LiftOff", "number", &ticks, &[0.0; 3]),
                ],
            )
        };
        let empty = record("raven_fipa", &[("Baro (Atm)", "pressure", &[], &[])]);
        assert_eq!(compare_raw(&empty, &one_clock(&[1.0; 3])), None);
        // An export's velocity with no rows hasn't stopped early: it holds nothing to compare.
        let velocity = record(
            "raven_fipa",
            &[("Velocity (Accel-Ft/Sec)", "speed", &ticks, &[1.0; 3])],
        );
        let none = record(
            "raven_fip_csv",
            &[("Velocity (Accel-Ft/Sec)", "speed", &[], &[])],
        );
        assert_eq!(compare_raw(&velocity, &none), None);
        // A raw channel, or an export's, with fewer times than values.
        let ragged = record(
            "raven_fipa",
            &[("Baro (Atm)", "pressure", &[0.0], &[1.0, 1.0])],
        );
        assert_eq!(compare_raw(&ragged, &one_clock(&[1.0; 3])), None);
        let raw = record(
            "raven_fipa",
            &[("Baro (Atm)", "pressure", &ticks, &[1.0; 3])],
        );
        let short_times = record(
            "raven_fip_csv",
            &[
                ("Baro (Atm)", "pressure", &ticks[..2], &[1.0; 3]),
                (
                    "[Altitude (Baro-Ft-AGL)]",
                    "altitude_above_ground",
                    &[0.0],
                    &[1.0],
                ),
            ],
        );
        assert_eq!(compare_raw(&raw, &short_times), None);
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

    /// A summary agrees with the Blue Raven file beside it in feet; one whose figures are the
    /// same flight's meters, labeled feet, agrees only were its feet meters, and fails the check.
    #[test]
    fn a_summary_whose_feet_are_meters_is_contradicted() {
        let mut csv = String::from(
            "Year,Month,Day,Time,Flight_Time_(s),Sync,Baro_Altitude_ASL_(feet),\
             Baro_Altitude_AGL_(feet)\n",
        );
        // 20 rows on the pad, then the flight, a row a second.
        let flight = [500.0, 1000.0, 500.0, 0.0];
        for (i, agl) in [0.0; 20].iter().chain(&flight).enumerate() {
            csv += &format!(
                "2026,5,23,10:15:{i:02}.000,{}.0,0,{},{agl}\n",
                i as f64 - 20.0,
                1000.0 + agl
            );
        }
        let sample = |name: &str, family: &str, bytes: Vec<u8>| Sample {
            path: PathBuf::from("flight").join(name),
            source: "test",
            family: family.to_owned(),
            bytes,
        };
        let summary = |feet: &str| {
            format!("Rocket Name,Demo\nMax Altitude,{feet} feet\nPad altitude ASL,{feet} feet\n")
                .into_bytes()
        };
        let low_rate = || sample("low.csv", "blueraven-csv", csv.clone().into_bytes());
        let read = |feet: &str| {
            survey(
                &[
                    sample("summary.csv", "blueraven-summary-csv", summary(feet)),
                    low_rate(),
                ],
                Vec::new(),
            )
        };
        let (report, unread) = read("1000.00");
        let checks = &report.summary_checks["Featherweight Blue Raven"];
        assert_eq!(
            (
                checks.files,
                checks.with_a_file_beside,
                checks.stating_an_apogee,
                checks.apogee_agrees,
                checks.pad_agrees
            ),
            (1, 1, 1, 1, 1)
        );
        assert_eq!(checks.apogee_ratios, Some([1.0, 1.0]));
        assert!(
            checks.contradicted.is_empty() && unread.is_empty(),
            "{unread:?}"
        );

        // 1,000 ft is 304.8 m: the same flight's figures in meters, written as feet.
        let (report, unread) = read("304.80");
        let checks = &report.summary_checks["Featherweight Blue Raven"];
        assert_eq!((checks.apogee_agrees, checks.pad_agrees), (0, 0));
        assert_eq!(checks.contradicted.len(), 1);
        assert_eq!(checks.agreeing_in_neither, checks.contradicted);
        assert!(
            unread
                .iter()
                .any(|line| line.contains("only were its feet meters")),
            "{unread:?}"
        );

        // Another flight's file, 3 times higher: no agreement, and no contradiction either.
        let (report, unread) = read("3000.00");
        let checks = &report.summary_checks["Featherweight Blue Raven"];
        assert_eq!((checks.apogee_agrees, checks.contradicted.len()), (0, 0));
        assert!(unread.is_empty(), "{unread:?}");
    }

    /// Each length is checked on its own: one written in meters fails the check though the other
    /// agrees, and a ratio just inside 10% agrees where one just outside doesn't.
    #[test]
    fn each_length_is_checked_and_the_band_is_ten_percent() {
        // 20 rows on the pad at 1,000 ft above sea level, then a climb to 1,000 ft above it.
        let mut csv = String::from(
            "Year,Month,Day,Time,Flight_Time_(s),Sync,Baro_Altitude_ASL_(feet),\
             Baro_Altitude_AGL_(feet)\n",
        );
        for (i, agl) in [0.0; 20].iter().chain(&[500.0, 1000.0, 0.0]).enumerate() {
            csv += &format!(
                "2026,5,23,10:15:{i:02}.000,{}.0,0,{},{agl}\n",
                i as f64 - 20.0,
                1000.0 + agl
            );
        }
        let checks = |apogee_ft: f64, pad_ft: f64| {
            let summary = format!(
                "Rocket Name,Demo\nMax Altitude,{apogee_ft} feet\nPad altitude ASL,{pad_ft} feet\n"
            );
            let sample = |name: &str, family: &str, bytes: Vec<u8>| Sample {
                path: PathBuf::from("flight").join(name),
                source: "test",
                family: family.to_owned(),
                bytes,
            };
            let (report, unread) = survey(
                &[
                    sample("summary.csv", "blueraven-summary-csv", summary.into_bytes()),
                    sample("low.csv", "blueraven-csv", csv.clone().into_bytes()),
                ],
                Vec::new(),
            );
            let checks = &report.summary_checks["Featherweight Blue Raven"];
            (
                checks.apogee_agrees,
                checks.pad_agrees,
                checks.contradicted.len(),
                unread.len(),
            )
        };
        // The apogee in meters, the pad right; then the reverse.
        assert_eq!(checks(304.8, 1000.0), (0, 1, 1, 1));
        assert_eq!(checks(1000.0, 304.8), (1, 0, 1, 1));
        // 9% high agrees, 11% high doesn't, and neither contradicts the feet.
        assert_eq!(checks(1090.0, 1000.0), (1, 1, 0, 0));
        assert_eq!(checks(1110.0, 1000.0), (0, 1, 0, 0));
    }

    /// A tracker's summary is checked against its own GNSS altitude, the highest taken above the
    /// pad the summary states; its feet written as meters fail the check.
    #[test]
    fn a_trackers_summary_is_checked_above_its_stated_pad() {
        const TRACKER: &[u8] =
            include_bytes!("../../crates/hpr-flightdata/tests/samples/fwgps-tracker-older.csv");
        let track = hpr_flightdata::import::read(TRACKER).unwrap();
        let altitude = &track_channel(&track, &["ALT"]).unwrap().values;
        let pad_m = pad_median(altitude).unwrap();
        let peak_m = altitude.iter().copied().fold(f64::MIN, f64::max);
        let sample = |name: &str, family: &str, bytes: Vec<u8>| Sample {
            path: PathBuf::from("flight").join(name),
            source: "test",
            family: family.to_owned(),
            bytes,
        };
        let run = |scale: f64| {
            let summary = format!(
                "Rocket Name,Demo\nLaunch time UTC,2024-02-29\nMax Altitude,{:.2} feet\n\
                 Pad altitude,{:.2} ft\n",
                (peak_m - pad_m) / 0.3048 * scale,
                pad_m / 0.3048 * scale
            );
            let (report, unread) = survey(
                &[
                    sample(
                        "summary.csv",
                        "featherweight-summary-csv",
                        summary.into_bytes(),
                    ),
                    sample("track.csv", "fwgps-csv", TRACKER.to_vec()),
                ],
                Vec::new(),
            );
            let checks = &report.summary_checks["Featherweight GPS tracker"];
            (
                checks.stating_an_apogee,
                checks.apogee_agrees,
                checks.pad_agrees,
                checks.contradicted.len(),
                unread.len(),
            )
        };
        assert_eq!(run(1.0), (1, 1, 1, 0, 0));
        // The same flight's meters, written as feet.
        assert_eq!(run(0.3048), (1, 0, 0, 1, 1));
    }

    #[test]
    fn an_unknown_figures_note_is_counted_without_its_name() {
        let note =
            "a figure HPR Sim doesn't know kept as written: `Spin rate`, the file writes `3`";
        assert_eq!(unit_decision(note), Some(UNKNOWN_FIGURE));
    }

    #[test]
    fn a_pad_is_the_median_of_the_first_finite_altitudes() {
        assert_eq!(pad_median(&[]), None);
        assert_eq!(pad_median(&[f64::NAN, 3.0, 1.0, 2.0]), Some(2.0));
        assert_eq!(pad_median(&[4.0, 1.0]), Some(2.5));
        let mut long = vec![10.0; PAD_SAMPLES];
        long.extend([1000.0; 100]);
        assert_eq!(pad_median(&long), Some(10.0));
    }

    /// A telemetry line: serial 7, its tick, a TeleMetrum v2 sensor packet, the CRC bit set.
    fn telemetry_line(
        tick: u16,
        state: u8,
        pres: i32,
        temp: i16,
        acc: i16,
        speed: i16,
        height: i16,
    ) -> String {
        let mut bytes = vec![34u8];
        bytes.extend(7u16.to_le_bytes());
        bytes.extend(tick.to_le_bytes());
        bytes.extend([0x0a, state]);
        bytes.extend(0i16.to_le_bytes());
        bytes.extend(pres.to_le_bytes());
        for value in [temp, acc, speed, height] {
            bytes.extend(value.to_le_bytes());
        }
        bytes.resize(33, 0);
        bytes.extend([0xf4, 0x80]);
        let sum = bytes[1..]
            .iter()
            .fold(0x5au8, |sum, b| sum.wrapping_add(*b));
        bytes.push(sum);
        let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        format!("TELEM {hex}\n")
    }

    /// AltOS telemetry against an AltosUI export beside it: equal pressures and temperatures agree,
    /// a row that repeats the one before is counted apart, a pressure that differs fails the
    /// check, and so do a board's speed and height out of each other's units.
    #[test]
    fn telemetry_is_checked_against_the_export_beside_it() {
        // Boost from tick 1,000 at 100 m/s², then the descent at 30 m/s: a packet every 0.1 s.
        let packets = |speed_scale: f64| {
            let mut rows = Vec::new();
            for k in 0..6i16 {
                let t = f64::from(k) / 10.0;
                let speed = (100.0 * t * 16.0 / speed_scale) as i16;
                rows.push((
                    1_000 + 10 * k as u16,
                    3,
                    97_000 - 100 * i32::from(k),
                    1_600,
                    speed,
                    (50.0 * t * t) as i16,
                ));
            }
            for k in 0..6i16 {
                rows.push((
                    2_000 + 10 * k as u16,
                    6,
                    90_000 + 30 * i32::from(k),
                    -16,
                    (-30.0 * 16.0 / speed_scale) as i16,
                    1_000 - 3 * k,
                ));
            }
            rows
        };
        // With `extra`, a packet at tick 1,023 too, between two of the export's rows.
        let telemetry_with = |speed_scale: f64, extra: bool| {
            let mut text = telemetry_line(900, 2, 98_000, 2_150, 0, 0, 0);
            for (tick, state, pres, acc, speed, height) in packets(speed_scale) {
                text += &telemetry_line(tick, state, pres, 2_150, acc, speed, height);
                if extra && tick == 1_020 {
                    text += &telemetry_line(1_023, 3, 96_000, 2_150, 1_600, 368, 3);
                }
            }
            text.into_bytes()
        };
        let telemetry = |speed_scale: f64| telemetry_with(speed_scale, false);
        let names = [(2, "pad"), (3, "boost"), (6, "drogue")];
        // The export's clock `turns` turns of the tick ahead of the telemetry's, as when the
        // two start counting at different turns.
        let export_turned = |change: Option<(usize, f64)>, repeat_at: Option<u16>, turns: f64| {
            let mut csv = String::from(
                "#version,serial,flight,time,clock,state,state_name,pressure,height,temperature\n",
            );
            let mut rows: Vec<(u16, u8, f64, f64)> = vec![(900, 2, 98_000.0, 0.0)];
            for (tick, state, pres, _, _, height) in packets(1.0) {
                rows.push((tick, state, f64::from(pres), f64::from(height)));
            }
            if let Some((row, by)) = change {
                rows[row].2 += by;
            }
            let mut written = Vec::new();
            for row in rows {
                written.push(row);
                if Some(row.0) == repeat_at {
                    // A row between the sensor samples, 0.03 s later, repeating the pressure.
                    written.push((row.0 + 3, row.1, row.2, row.3));
                }
            }
            for (tick, state, pres, height) in written {
                let name = names.iter().find(|(n, _)| *n == state).unwrap().1;
                csv += &format!(
                    "5,7,1,{:.2},{:.2},{state},{name},{pres:.2},{height:.2},21.5\n",
                    (f64::from(tick) - 1_000.0) / 100.0,
                    (f64::from(tick) + turns * 65_536.0) / 100.0
                );
            }
            csv.into_bytes()
        };
        let export = |change, repeat_at| export_turned(change, repeat_at, 0.0);
        let sample = |name: &str, family: &str, bytes: Vec<u8>| Sample {
            path: PathBuf::from("flight").join(name),
            source: "test",
            family: family.to_owned(),
            bytes,
        };
        let run = |telemetry: Vec<u8>, export: Vec<u8>| {
            survey(
                &[
                    sample("flight.telem", "altos-telem", telemetry),
                    sample("flight.csv", "altos-csv", export),
                ],
                Vec::new(),
            )
        };

        let (report, unread) = run(telemetry(1.0), export(None, Some(1_020)));
        let checks = &report.telemetry_checks;
        assert!(unread.is_empty(), "{unread:?}");
        assert_eq!(
            (
                checks.files,
                checks.with_an_export_beside,
                checks.agreeing_with_an_export
            ),
            (1, 1, 1)
        );
        // 13 telemetry pressures at the export's ticks; the repeated row is at a tick of its own,
        // which no telemetry packet has.
        assert_eq!(
            (checks.pressures_equal, checks.pressures_at_repeated_rows),
            (13, 0)
        );
        assert!(checks.worst_temperature_k < 1e-9);
        assert_eq!(
            (
                checks.sensor_packets_beside_an_export,
                checks.at_the_exports_ticks
            ),
            (13, 13)
        );
        let file = &checks.per_file[0];
        assert_eq!(
            (
                file.packets_heard,
                file.failing_the_crc,
                file.power_ons,
                file.pressure_in_pascals
            ),
            (13, 0, 1, true)
        );
        for range in [
            checks.height_over_export,
            checks.climb_over_speed,
            checks.speed_change_over_acceleration,
        ] {
            let [least, greatest] = range.unwrap();
            assert!(
                (least - 1.0).abs() < 1e-9 && (greatest - 1.0).abs() < 1e-9,
                "{range:?}"
            );
        }

        // A telemetry packet at the repeated row's tick: counted apart, not a difference.
        let (report, unread) = run(telemetry_with(1.0, true), export(None, Some(1_020)));
        assert_eq!(report.telemetry_checks.pressures_at_repeated_rows, 1);
        assert!(unread.is_empty(), "{unread:?}");

        // One pressure 5 Pa off: the file differs, and `--check` fails.
        let (report, unread) = run(telemetry(1.0), export(Some((4, 5.0)), None));
        assert_eq!(report.telemetry_checks.differing.len(), 1);
        assert!(
            unread
                .iter()
                .any(|line| line.contains("differs from every AltosUI export")),
            "{unread:?}"
        );

        // The export's clock three turns ahead: the `boost` events line the two up.
        let (report, unread) = run(telemetry(1.0), export_turned(None, None, 3.0));
        assert_eq!(report.telemetry_checks.pressures_equal, 13);
        assert!(unread.is_empty(), "{unread:?}");

        // The board's speed twice what its height and acceleration bear out: both ratios exist
        // and fall outside the band.
        let (report, unread) = run(telemetry(0.5), export(None, None));
        let checks = &report.telemetry_checks;
        let climb = checks.climb_over_speed.unwrap();
        let change = checks.speed_change_over_acceleration.unwrap();
        assert!((climb[0] - 0.5).abs() < 1e-9 && (change[0] - 2.0).abs() < 1e-9);
        assert_eq!(checks.inconsistent.len(), 1);
        assert!(
            unread
                .iter()
                .any(|line| line.contains("aren't in one another's units")),
            "{unread:?}"
        );

        // 16 times too slow: the speed after burnout falls under 20 m/s, so no ratio, and no
        // pass, counted apart from a unit mismatch.
        let (report, unread) = run(telemetry(16.0), export(None, None));
        let checks = &report.telemetry_checks;
        assert_eq!((checks.unchecked.len(), checks.inconsistent.len()), (1, 0));
        assert!(
            unread
                .iter()
                .any(|line| line.contains("too few fast packets")),
            "{unread:?}"
        );
    }

    /// AltosUI's export of a `.eeprom` record's flight, as rows `(tick, state, pressure Pa,
    /// temperature °C, acceleration m/s²)`: a row at each tick the record holds a pressure at
    /// or after, each column its latest value there, as an export holds the last sample.
    fn eeprom_export_rows(log: &FlightLog) -> Vec<(i64, f64, f64, f64, f64)> {
        let channel = |name: &str| {
            let c = log.channels.iter().find(|c| c.column == name).unwrap();
            let ticks: Vec<i64> = c
                .time_s
                .iter()
                .map(|t| (t * 100.0).round() as i64)
                .collect();
            (ticks, c.values.clone())
        };
        let latest = |(ticks, values): &(Vec<i64>, Vec<f64>), at: i64, before: f64| {
            ticks
                .iter()
                .zip(values)
                .rfind(|(tick, _)| **tick <= at)
                .map_or(before, |(_, value)| *value)
        };
        let (state, pressure) = (channel("state"), channel("pressure"));
        let (temperature, acceleration) = (channel("temperature"), channel("acceleration"));
        let first = pressure.0[0];
        let ticks: BTreeSet<i64> = log
            .channels
            .iter()
            .flat_map(|c| c.time_s.iter().map(|t| (t * 100.0).round() as i64))
            .filter(|tick| *tick >= first)
            .collect();
        ticks
            .into_iter()
            .map(|tick| {
                (
                    tick,
                    latest(&state, tick, 2.0),
                    latest(&pressure, tick, f64::NAN),
                    latest(&temperature, tick, f64::NAN) - 273.15,
                    latest(&acceleration, tick, f64::NAN),
                )
            })
            .collect()
    }

    /// The export's text: serial and flight, `time` from the flight record's tick (65,530) and
    /// `clock` from the tick itself, each value to two decimals.
    fn eeprom_export(serial: u32, rows: &[(i64, f64, f64, f64, f64)]) -> Vec<u8> {
        let names = [(2, "pad"), (3, "boost"), (5, "coast"), (6, "drogue")];
        let mut csv = String::from(
            "#version,serial,flight,time,clock,state,state_name,acceleration,pressure,height,\
             temperature\n",
        );
        let pad = rows[0].2;
        for (tick, state, pressure, temperature, acceleration) in rows {
            let name = names
                .iter()
                .find(|(n, _)| f64::from(*n) == *state)
                .unwrap()
                .1;
            let height = 44_330.8
                * ((pad / 101_325.0).powf(0.190_263) - (pressure / 101_325.0).powf(0.190_263));
            csv += &format!(
                "5,{serial},3,{:.2},{:.2},{state},{name},{acceleration:.2},{pressure:.2},\
                 {height:.2},{temperature:.2}\n",
                *tick as f64 / 100.0,
                (*tick + 65_530) as f64 / 100.0
            );
        }
        csv.into_bytes()
    }

    /// A `.eeprom` file against the AltosUI export and the radio telemetry beside it: a matched
    /// export agrees; a pressure one print step off, an export row at a tick the file holds no
    /// sample at, or an acceleration off by a constant other than the +1 g zero's, differs; the
    /// +1 g zero's own constant is counted apart; a telemetry pressure that differs is caught.
    #[test]
    fn an_eeprom_file_is_checked_against_the_export_and_telemetry_beside_it() {
        let original: &[u8] = include_bytes!(
            "../../crates/hpr-flightdata/tests/samples/altos-eeprom-telemetrum-v2.eeprom"
        );
        let sample = |name: &str, family: &str, bytes: Vec<u8>| Sample {
            path: PathBuf::from("flight").join(name),
            source: "test",
            family: family.to_owned(),
            bytes,
        };
        let id = |bytes: &[u8]| hex(&Sha256::digest(bytes))[..16].to_owned();
        let run = |eeprom: &[u8], beside: Vec<Sample>| {
            let mut samples = vec![sample("flight.eeprom", "altos-eeprom", eeprom.to_vec())];
            samples.extend(beside);
            let (report, unread) = survey(&samples, Vec::new());
            let ours: Vec<String> = unread
                .into_iter()
                .filter(|line| line.contains(&id(eeprom)))
                .collect();
            (report.eeprom_checks, ours)
        };
        let log = hpr_flightdata::import::read(original).unwrap();
        let rows = eeprom_export_rows(&log);
        let with_export = |eeprom: &[u8], rows: &[(i64, f64, f64, f64, f64)]| {
            run(
                eeprom,
                vec![sample("flight.csv", "altos-csv", eeprom_export(1234, rows))],
            )
        };

        // The matched pair agrees, both ways.
        let (checks, unread) = with_export(original, &rows);
        assert!(unread.is_empty(), "{unread:?}");
        assert_eq!(
            (
                checks.files,
                checks.with_an_export_beside,
                checks.agreeing_with_an_export
            ),
            (1, 1, 1)
        );
        assert!(checks.differing.is_empty() && checks.zeroed_at_the_plus_calibration.is_empty());
        let file = &checks.per_file[0];
        let export = file.export.as_ref().unwrap();
        assert_eq!((export.pressures, export.accelerations), (21, 21));
        assert_eq!(export.rows_without_a_sample, 0);
        assert!(export.worst_pressure_pa < 1e-9 && export.worst_acceleration_m_s2 <= 0.005);
        assert_eq!(
            (file.records, file.log_format, file.form.as_str()),
            (32, 7, "JSON")
        );
        assert!(file.climb_ratio.is_some_and(|r| (r - 1.0).abs() < 0.01));
        assert_eq!(checks.climb_ratios.map(|r| r[0]), file.climb_ratio);
        assert_eq!(file.smallest_pressure_step_pa, Some(1.0));

        // A pressure one print step, 0.01 Pa, off at one row: the file differs, and is unread.
        let mut moved = rows.clone();
        let row = moved.iter().position(|row| row.0 == 50).unwrap();
        moved[row].2 += 0.01;
        let (checks, unread) = with_export(original, &moved);
        assert_eq!(checks.differing, [id(original)]);
        assert_eq!(checks.agreeing_with_an_export, 0);
        assert!(
            unread
                .iter()
                .any(|line| line.contains("differs from the AltosUI export")),
            "{unread:?}"
        );

        // A row at a tick the file holds no sample at, 0.15 s: the export differs.
        let mut extra = rows.clone();
        let at = extra.iter().position(|row| row.0 > 15).unwrap();
        let held = extra[at - 1];
        extra.insert(at, (15, held.1, held.2, held.3, held.4));
        let (checks, _) = with_export(original, &extra);
        assert_eq!(checks.differing, [id(original)]);

        // The +1 g zero: a calibration of 990 and 1,030 puts +1 g 10 counts, half a g, below the
        // ground reading of 1,000. An export whose acceleration is the file's less half a g is
        // counted apart; less 0.6 g, it differs.
        let text = String::from_utf8_lossy(original)
            .replace("\"accel_cal_minus\": 1040", "\"accel_cal_minus\": 1030")
            .replace("\"accel_cal_plus\": 1000", "\"accel_cal_plus\": 990")
            .into_bytes();
        let offset = |g: f64| {
            let mut rows = rows.clone();
            for row in &mut rows {
                row.4 -= g * 9.806_65;
            }
            rows
        };
        let (checks, unread) = with_export(&text, &offset(0.5));
        assert!(unread.is_empty(), "{unread:?}");
        assert_eq!(checks.zeroed_at_the_plus_calibration, [id(&text)]);
        assert!(checks.differing.is_empty());
        assert!(
            checks.per_file[0]
                .export
                .as_ref()
                .unwrap()
                .zeroed_at_the_plus_calibration
        );
        let (checks, _) = with_export(&text, &offset(0.6));
        assert_eq!(checks.differing, [id(&text)]);
        assert!(checks.zeroed_at_the_plus_calibration.is_empty());
        // Nor is a constant offset the +1 g zero where the calibration puts +1 g at the ground
        // reading, as the original's does.
        let (checks, _) = with_export(original, &offset(0.5));
        assert_eq!(checks.differing, [id(original)]);

        // The board's telemetry, serial 7: a packet on the pad, then one at each of the file's
        // samples from boost on, with its pressure. All equal agrees; one 5 Pa off differs.
        let seven = String::from_utf8_lossy(original)
            .replace("\"serial\": 1234", "\"serial\": 7")
            .into_bytes();
        let telemetry = |off: i32| {
            let mut text = telemetry_line(65_500, 2, 110_002, 2_000, 0, 0, 0);
            for (tick, state, pressure, ..) in rows.iter().filter(|row| row.0 >= 1) {
                let state = if *state < 3.0 { 3 } else { *state as u8 };
                let pres = *pressure as i32 + if *tick == 50 { off } else { 0 };
                let tick = ((65_530 + tick) % 65_536) as u16;
                text += &telemetry_line(tick, state, pres, 2_000, 0, 0, 0);
            }
            text.into_bytes()
        };
        let (checks, unread) = run(
            &seven,
            vec![sample("flight.telem", "altos-telem", telemetry(0))],
        );
        assert!(unread.is_empty(), "{unread:?}");
        assert_eq!(
            (checks.with_telemetry_beside, checks.agreeing_with_telemetry),
            (1, 1)
        );
        let telemetry_check = checks.per_file[0].telemetry.as_ref().unwrap();
        assert_eq!(
            (
                telemetry_check.ticks_shared,
                telemetry_check.pressures_equal
            ),
            (17, 17)
        );
        let (checks, unread) = run(
            &seven,
            vec![sample("flight.telem", "altos-telem", telemetry(5))],
        );
        assert_eq!(checks.agreeing_with_telemetry, 0);
        assert_eq!(checks.differing, [id(&seven)]);
        assert!(
            unread.iter().any(|line| line.contains("or the telemetry")),
            "{unread:?}"
        );
    }

    /// An `.odf` against the raw CSV beside it: the invented pair matches, word for word; one
    /// state changed in the CSV differs, and leaves the `.odf` unread.
    #[test]
    fn an_odf_is_checked_against_the_raw_csv_beside_it() {
        let odf: &[u8] = include_bytes!("../../crates/hpr-flightdata/tests/samples/arts2-raw.odf");
        let csv = include_str!("../../crates/hpr-flightdata/tests/samples/arts2-raw.csv");
        let id = hex(&Sha256::digest(odf))[..16].to_owned();
        let run = |csv: &str| {
            let sample = |name: &str, family: &str, bytes: Vec<u8>| Sample {
                path: PathBuf::from("flight").join(name),
                source: "test",
                family: family.to_owned(),
                bytes,
            };
            let samples = vec![
                sample("flight.odf", "odf", odf.to_vec()),
                sample("flight.csv", "arts2-raw-csv", csv.as_bytes().to_vec()),
            ];
            let (report, unread) = survey(&samples, Vec::new());
            let ours: Vec<String> = unread.into_iter().filter(|l| l.contains(&id)).collect();
            (report.arts2_checks, ours)
        };
        let (checks, unread) = run(csv);
        assert!(unread.is_empty(), "{unread:?}");
        let file = &checks.per_file[0];
        assert_eq!(file.records, 150);
        assert_eq!(file.raw_csv_matches, Some(true));
        assert_eq!(file.climb_ratio, Some(1.012));
        assert!(file.export.is_none() && file.altos_pad_kpa.is_none());

        let row = csv.lines().find(|l| l.starts_with("0.060,")).unwrap();
        let changed = csv.replacen(row, &format!("{}9", &row[..row.len() - 1]), 1);
        assert_ne!(changed, csv);
        let (checks, unread) = run(&changed);
        assert_eq!(checks.per_file[0].raw_csv_matches, Some(false));
        assert_eq!(checks.differing, [id]);
        assert_eq!(unread.len(), 1, "{unread:?}");
    }

    /// An `.odf`'s values against an interpreted export: the invented export, put on the
    /// `.odf`'s clock, against an acceleration equal to its own and pressures falling as its
    /// `pAlt` one row later rises, agrees; an acceleration 0.6 counts off, a pressure meeting two
    /// `pAlt`s, and a lower pressure meeting a lower `pAlt` are each caught.
    #[test]
    fn an_odf_is_checked_against_an_interpreted_export() {
        use hpr_flightdata::log::Quantity;
        let mut export = hpr_flightdata::import::read(include_bytes!(
            "../../crates/hpr-flightdata/tests/samples/arts2-metric.csv"
        ))
        .unwrap();
        let rows = export.time_s.len();
        export.time_s = (0..rows).map(odf_time).collect();
        let acceleration = export
            .channels_of(Quantity::Acceleration)
            .next()
            .unwrap()
            .values
            .clone();
        // A g of 8 counts: one count is 9.806 65 / 8 m/s².
        let count = 9.806_65 / 8.0;
        let pressure: Vec<f64> = (0..rows)
            .map(|k| 100_000.0 - 10.0 * export.altitude_m.get(k + 1).copied().unwrap_or(0.0))
            .collect();
        let check = |acceleration: &[f64], pressure: &[f64]| {
            arts2_against_export(&export, acceleration, pressure, 100_000.0, 8.0)
        };
        let agreed = check(&acceleration, &pressure);
        assert_eq!(
            (agreed.rows_compared, agreed.worst_acceleration_counts),
            (rows - 1, 0.0)
        );
        assert!(agreed.first_row_zero && agreed.rows_after == 0 && agreed.rows_after_zero);
        assert_eq!((agreed.clashes, agreed.inversions), (0, 0));

        let mut off = acceleration.clone();
        off[5] += 0.6 * count;
        assert!((check(&off, &pressure).worst_acceleration_counts - 0.6).abs() < 1e-9);
        // The pressure before the highest `pAlt` given to a row whose `pAlt` differs.
        let top = (1..rows)
            .max_by(|a, b| export.altitude_m[*a].total_cmp(&export.altitude_m[*b]))
            .unwrap();
        let mut clash = pressure.clone();
        clash[1] = clash[top - 1];
        assert_eq!(check(&acceleration, &clash).clashes, 1);
        // The same row's pressure made lower than the top's, its `pAlt` lower too.
        let mut inverted = pressure.clone();
        inverted[1] = clash[top - 1] - 1.0;
        assert!(check(&acceleration, &inverted).inversions > 0);
        // An export a row short of the records can't match.
        let mut longer = pressure.clone();
        longer.push(90_000.0);
        let mut more = acceleration.clone();
        more.push(0.0);
        assert!(
            check(&more, &longer)
                .worst_acceleration_counts
                .is_infinite()
        );

        // A `pAlt` of zeros everywhere is out of order below the pad, and one that is a gap is
        // counted.
        let against = |export: &FlightLog| {
            arts2_against_export(export, &acceleration, &pressure, 100_000.0, 8.0)
        };
        // A pressure at the pad's must read zero: a `pAlt` of 1 mm there is out of order, though
        // it is still the least above zero.
        let on_pad = (1..rows).find(|k| export.altitude_m[*k] == 0.0).unwrap();
        let mut lifted = export.clone();
        lifted.altitude_m[on_pad] = 0.001;
        assert_eq!(against(&lifted).inversions, 1);
        let mut zeros = export.clone();
        zeros.altitude_m = vec![0.0; rows];
        assert!(against(&zeros).inversions > 0);
        let mut gap = export.clone();
        gap.altitude_m[top] = f64::NAN;
        assert_eq!(against(&gap).gaps, 1);
    }

    /// An `.odf` differs at half a count and a hair, with a `pAlt` gap, a clash, an inversion, a
    /// first row or a row after the records not zero, or a raw CSV that differs; and not
    /// otherwise.
    #[test]
    fn what_makes_an_odf_differ() {
        let agreeing = || Arts2Export {
            rows_compared: 10,
            worst_acceleration_counts: 0.49,
            first_row_zero: true,
            rows_after_zero: true,
            ..Arts2Export::default()
        };
        let file = |export: Arts2Export, raw: Option<bool>| Arts2File {
            raw_csv_matches: raw,
            export: Some(export),
            ..Arts2File::default()
        };
        assert!(!arts2_differs(&file(agreeing(), Some(true))));
        assert!(!arts2_differs(&Arts2File::default()));
        assert!(arts2_differs(&file(agreeing(), Some(false))));
        let cases = [
            Arts2Export {
                worst_acceleration_counts: 0.51,
                ..agreeing()
            },
            Arts2Export {
                worst_acceleration_counts: f64::NAN,
                ..agreeing()
            },
            Arts2Export {
                first_row_zero: false,
                ..agreeing()
            },
            Arts2Export {
                rows_after_zero: false,
                ..agreeing()
            },
            Arts2Export {
                gaps: 1,
                ..agreeing()
            },
            Arts2Export {
                clashes: 1,
                ..agreeing()
            },
            Arts2Export {
                inversions: 1,
                ..agreeing()
            },
        ];
        for export in cases {
            let shown = format!("{export:?}");
            assert!(arts2_differs(&file(export, None)), "{shown}");
        }
    }

    /// An interpreted export beside an `.odf` that doesn't match it, here one on another clock,
    /// leaves the `.odf` unread.
    #[test]
    fn an_odf_differing_from_the_export_beside_it_is_unread() {
        let odf: &[u8] = include_bytes!("../../crates/hpr-flightdata/tests/samples/arts2-raw.odf");
        let export: &[u8] =
            include_bytes!("../../crates/hpr-flightdata/tests/samples/arts2-metric.csv");
        let id = hex(&Sha256::digest(odf))[..16].to_owned();
        let sample = |name: &str, family: &str, bytes: &[u8]| Sample {
            path: PathBuf::from("flight").join(name),
            source: "test",
            family: family.to_owned(),
            bytes: bytes.to_vec(),
        };
        let samples = vec![
            sample("flight.odf", "odf", odf),
            sample("export.csv", "arts2-csv", export),
        ];
        let (report, unread) = survey(&samples, Vec::new());
        let file = &report.arts2_checks.per_file[0];
        assert!(
            file.export
                .as_ref()
                .is_some_and(|e| e.worst_acceleration_counts.is_infinite())
        );
        assert_eq!(report.arts2_checks.differing, std::slice::from_ref(&id));
        assert!(unread.iter().any(|line| line.contains(&id)), "{unread:?}");
    }

    /// The invented `.rtx` and its exports.
    fn rtx_samples(csv: &str) -> Vec<Sample> {
        let sample = |name: &str, family: &str, bytes: &[u8]| Sample {
            path: PathBuf::from("flight").join(name),
            source: "test",
            family: family.to_owned(),
            bytes: bytes.to_vec(),
        };
        vec![
            sample(
                "flight.rtx",
                "missileworks-rtx",
                include_bytes!("../../crates/hpr-flightdata/tests/samples/rtx.rtx"),
            ),
            sample("export.csv", "rtx-csv", csv.as_bytes()),
            sample(
                "export.txt",
                "rtx-txt",
                include_bytes!("../../crates/hpr-flightdata/tests/samples/rtx.txt"),
            ),
        ]
    }

    const RTX_CSV: &str = include_str!("../../crates/hpr-flightdata/tests/samples/rtx.csv");

    /// The invented `.rtx` agrees with its record and both exports: every fix but the last, the
    /// CSV's positions exactly, the text file's cut at the sixth decimal, the feet rounded down.
    #[test]
    fn an_rtx_agrees_with_the_exports_beside_it() {
        let (report, unread) = survey(&rtx_samples(RTX_CSV), Vec::new());
        let checks = &report.rtx_checks;
        assert!(checks.differing.is_empty(), "{checks:?}");
        assert!(unread.is_empty(), "{unread:?}");
        let file = &checks.per_file[0];
        assert_eq!(file.records, 30);
        assert!(file.reader_matches);
        let formats: Vec<(&str, usize, bool, bool)> = file
            .exports
            .iter()
            .map(|e| {
                (
                    e.format.as_str(),
                    e.rows,
                    e.every_record_but_the_last,
                    e.altitude_as_written,
                )
            })
            .collect();
        assert_eq!(
            formats,
            [("RTx CSV", 29, true, true), ("RTx text", 29, true, true)]
        );
        assert_eq!(file.exports[0].worst_position_deg, 0.0);
        assert!(file.exports[1].worst_position_deg > 0.0);
        assert!(file.exports[1].worst_position_deg < 1e-6);
        assert_eq!(file.kml, None);
        assert_eq!(file.climb_over_barometric, None);
    }

    /// An export that differs from its `.rtx`, in a number, a foot or a row too many, leaves the
    /// `.rtx` unread.
    #[test]
    fn an_rtx_differing_from_an_export_is_unread() {
        let lines: Vec<&str> = RTX_CSV.split("\r\n").collect();
        let id = hex(&Sha256::digest(include_bytes!(
            "../../crates/hpr-flightdata/tests/samples/rtx.rtx"
        )))[..16]
            .to_owned();
        let edit = |row: usize, cell: usize, value: &str| {
            let mut lines = lines.clone();
            let mut cells: Vec<String> = lines[row].split(',').map(str::to_owned).collect();
            cells[cell] = value.to_owned();
            let edited = cells.join(",");
            lines[row] = &edited;
            lines.join("\r\n")
        };
        // A course one degree off, a foot rounded up rather than down, the last fix written, and
        // a course that isn't a number, so the export fails to read.
        let mut with_last = RTX_CSV.to_owned();
        with_last += "180027,3259.01321N,10658.54263W,4593,0,2,148,0,16,127\r\n";
        for (csv, every, feet) in [
            (edit(5, 9, "53"), false, true),
            (edit(1, 3, "4594"), true, false),
            (with_last, false, true),
            (edit(5, 9, "east"), false, false),
        ] {
            let (report, unread) = survey(&rtx_samples(&csv), Vec::new());
            let export = &report.rtx_checks.per_file[0].exports[0];
            assert_eq!(
                (export.every_record_but_the_last, export.altitude_as_written),
                (every, feet),
                "{csv}"
            );
            assert_eq!(report.rtx_checks.differing, std::slice::from_ref(&id));
            assert!(unread.iter().any(|line| line.contains(&id)), "{unread:?}");
        }
    }

    /// A position cut toward zero passes; rounded away from it, short by a whole digit, or on the
    /// other side of zero, it doesn't.
    #[test]
    fn an_rtx_position_must_be_cut_toward_zero() {
        let truth = -106.975_720_166_666_7;
        assert!(rtx_cut(-106.975_720, truth));
        assert!(rtx_cut(truth, truth));
        assert!(!rtx_cut(-106.975_721, truth));
        assert!(!rtx_cut(-106.975_719, truth));
        assert!(!rtx_cut(106.975_720, truth));
        assert!(rtx_cut(32.983_539, 32.983_539_000_000_1));
        assert!(!rtx_cut(32.983_540, 32.983_539_4));
    }
}
