//! `cargo xtask ghost`: writes ghost files ([`hpr_flightdata::ghost`]) from an invented flight and
//! from every real flight log in RocketPy's repository, then has outside readers read them back
//! (`validation/oracles/ghost/read_back.mjs`; [M7.2g][m7-2g], [ADR-313][adr-313]).
//!
//! The logs are fetched into `refs/` and never committed; CI holds the invented flight only, as
//! ADR-280 planned. The report, `validation/reports/ghost-files.md`, holds counts and heights
//! computed from the public logs. Needs `node` and `npm`, and the network the first time.
//!
//! [m7-2g]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions-and-roadmap.md#m7-2g
//! [adr-313]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0313-ghost-files.md

use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::process::Command;

use hpr_flightdata::custom;
use hpr_flightdata::ghost::{self, Ghost, Input, PadFrom, UpFrom};
use hpr_flightdata::import;
use hpr_flightdata::log::FlightLog;
use hpr_flightdata::readings;

const USAGE: &str = "\
ghost [--check] [--invented]

Writes each ghost file's JSON, CZML and glTF to a scratch folder and reads them back with outside
readers (node, validation/oracles/ghost). Rewrites validation/reports/ghost-files.md.

--check     fails if the report differs, and if refs/rocketpy isn't fetched
--invented  only the invented flight, which needs no refs/ and writes no report (CI)";

const REPORT: &str = "validation/reports/ghost-files.md";
const ORACLE: &str = "validation/oracles/ghost";
const MAPPINGS: &str = "validation/fixtures/logs/rocketpy-mappings";
const ROCKETS: &str = "refs/rocketpy/data/rockets";
const TRACKED: &str = "validation/fixtures/logs/ghost/invented-tracked.csv";
const SECOND_BOARD: &str = "validation/fixtures/logs/two-logs/second-board.csv";

/// Every real flight log in RocketPy's repository with an altitude or a GNSS track, its path under
/// `refs/rocketpy/data/rockets`, and whether a mapping states its columns: its own header doesn't.
/// Left out: its drag, thrust, mass, tank and weather curves, `camoes/actuation.csv` (an air
/// brake's commands), and `hedy/cats_tust`'s `baro.csv` and `imu.csv` (no altitude or track).
pub const ROCKETPY_LOGS: [(&str, bool); 22] = [
    ("NDRT_2020/ndrt_2020_flight_data.csv", false),
    ("andromeda/flight_data.csv", false),
    ("juno3/cots_altimeter.csv", false),
    (
        "prometheus/2022-06-24-serial-5115-flight-0001-TeleMetrum.csv",
        false,
    ),
    (
        "prometheus/2022-06-24-serial-6583-flight-0003-TeleMega.csv",
        false,
    ),
    ("EPFL_Bella_Lui/bella_lui_flight_data_filtered.csv", true),
    ("astg/altimeter_halcyon.csv", true),
    ("astg/gnss_halcyon.csv", true),
    ("astra/flight_data.csv", true),
    ("camoes/flight_data.csv", true),
    ("camoes/gnss_flight.csv", true),
    ("erebus11/flight_data_filtered.csv", true),
    ("genesis/flight_data_faraday.csv", true),
    ("genesis/gnss_faraday.csv", true),
    ("hedy/cats_tust/filteredDataInfo.csv", true),
    ("hedy/cats_tust/flightInfo.csv", true),
    ("hedy/cats_tust/gnssInfo.csv", true),
    ("juno3/cots_GNSS.csv", true),
    ("juno3/srad_telemetry.csv", true),
    ("lince/main_data.csv", true),
    ("polito/altimeter_cavour.csv", true),
    ("valkyrie/flightInfo_merged.csv", true),
];

/// One name for one path, as a mapping's file and a ghost's are named.
fn stem(path: &str) -> String {
    path.trim_end_matches(".csv").replace('/', "-")
}

/// Reads a log by its format's own reader or by the mapping beside the fixtures.
fn read_rocketpy(root: &Path, path: &str, mapped: bool) -> Result<FlightLog, String> {
    let bytes = fs::read(root.join(ROCKETS).join(path)).map_err(|e| format!("{path}: {e}"))?;
    if mapped {
        let file = root.join(MAPPINGS).join(format!("{}.json", stem(path)));
        let text = fs::read_to_string(&file).map_err(|e| format!("{}: {e}", file.display()))?;
        let mapping = custom::mapping_from_json(&text).map_err(|e| format!("{path}: {e}"))?;
        custom::read_by_mapping(&bytes, &mapping, None).map_err(|e| format!("{path}: {e}"))
    } else {
        import::read_one(&bytes, None, None).map_err(|e| format!("{path}: {e}"))
    }
}

/// Lays out `logs` by name and writes the three files.
fn write_all(folder: &Path, stem: &str, logs: &[(&str, &FlightLog)]) -> Result<Ghost, String> {
    let reads: Vec<_> = logs.iter().map(|(_, log)| readings::read(log)).collect();
    let inputs: Vec<Input<'_>> = logs
        .iter()
        .zip(&reads)
        .map(|((name, log), read)| Input { name, log, read })
        .collect();
    let built = ghost::build(&inputs, None).map_err(|e| format!("{stem}: {e}"))?;
    let json = hpr_cli::ghost_json(&built).map_err(|e| format!("{stem}: {e}"))?;
    let write = |extension: &str, bytes: &[u8]| {
        fs::write(folder.join(format!("{stem}.{extension}")), bytes)
            .map_err(|e| format!("{stem}.{extension}: {e}"))
    };
    write("json", &json)?;
    write("czml", built.to_czml().as_bytes())?;
    write("gltf", built.to_gltf().as_bytes())?;
    Ok(built)
}

/// Installs the pinned readers once and runs them over `folder`; each line of their output.
fn read_back(root: &Path, folder: &Path) -> Result<Vec<String>, String> {
    let oracle = root.join(ORACLE);
    if !oracle.join("node_modules").exists() {
        let npm = if cfg!(windows) { "npm.cmd" } else { "npm" };
        let status = Command::new(npm)
            .args(["ci", "--silent", "--no-audit", "--no-fund"])
            .current_dir(&oracle)
            .status()
            .map_err(|e| format!("npm: {e}"))?;
        if !status.success() {
            return Err("npm ci failed".to_owned());
        }
    }
    let output = Command::new("node")
        .arg(oracle.join("read_back.mjs"))
        .arg(root.join("schema/cli/ghost.schema.json"))
        .arg(folder)
        .output()
        .map_err(|e| format!("node: {e}"))?;
    let lines: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::to_owned)
        .collect();
    if !output.status.success() {
        let failed: Vec<&String> = lines.iter().filter(|l| l.contains("\"error\"")).collect();
        return Err(format!(
            "the outside readers refused a ghost file: {failed:?}\n{}",
            String::from_utf8_lossy(&output.stderr)
                .lines()
                .filter(|line| !line.starts_with("unknown format"))
                .collect::<Vec<_>>()
                .join("\n"),
        ));
    }
    Ok(lines)
}

/// The report's row for one log's ghost.
fn row(path: &str, mapped: bool, built: &Ghost) -> String {
    let track = &built.tracks[0];
    let top = track
        .up_m
        .iter()
        .flatten()
        .copied()
        .fold(f64::NAN, f64::max);
    let far = track
        .east_m
        .iter()
        .zip(&track.north_m)
        .filter_map(|(east, north)| Some(east.as_ref()?.hypot(*north.as_ref()?)))
        .fold(f64::NAN, f64::max);
    let up_from = match track.up_from {
        UpFrom::LogAltitude => "the log's altitude",
        UpFrom::GnssAltitude => "GNSS altitude",
        UpFrom::Unknown => "none",
    };
    let pad = match built.pad.from {
        PadFrom::FixesBeforeLiftoff => "fixes before liftoff",
        PadFrom::FirstFix => "first fix",
        PadFrom::Unknown => "not known",
        PadFrom::Stated => "stated",
    };
    let shown = |value: f64| {
        if value.is_nan() {
            "none".to_owned()
        } else {
            format!("{value:.0} m")
        }
    };
    format!(
        "| `{path}` | {} | {} | {} | {up_from} | {} | {} | {} of {} | {} | {pad} |\n",
        if mapped { "mapping" } else { "own reader" },
        if track.synced { "liftoff" } else { "own clock" },
        track.time_s.len(),
        shown(top),
        shown(far),
        track.fixes.kept,
        track.fixes.given,
        track.heights_too_fast,
    )
}

/// The report's whole text.
fn report(rows: &str, moments: usize) -> String {
    let mut text = String::new();
    let _ = write!(
        text,
        "# Ghost files from RocketPy's flight logs\n\n\
         Written by `cargo xtask ghost`. Each of the {count} real flight logs in RocketPy's \
         repository with an altitude or a GNSS track, read and laid out as a ghost file; each file \
         read back as JSON by Ajv, as CZML by CesiumJS and as glTF by Khronos's glTF-Validator and \
         glTF-Transform, and each format held to the others ({moments} moments compared). \
         \"Highest\" and \"farthest\" are what the file holds, in metres from the pad.\n\n\
         | Log | Read by | Clock | Moments | Height from | Highest | Farthest | Fixes kept | \
         Heights left out | Pad |\n\
         |---|---|---|---:|---|---:|---:|---:|---:|---|\n\
         {rows}",
        count = ROCKETPY_LOGS.len(),
    );
    text
}

/// Runs `cargo xtask ghost`.
pub fn run(args: &[String]) -> Result<(), String> {
    let (mut check, mut invented_only) = (false, false);
    for arg in args {
        match arg.as_str() {
            "--check" => check = true,
            "--invented" => invented_only = true,
            other => return Err(format!("unknown argument `{other}`\n\n{USAGE}")),
        }
    }
    let root = crate::designs::root()?;
    let scratch = root.join("target/ghost-files");
    if scratch.exists() {
        fs::remove_dir_all(&scratch).map_err(|e| format!("{}: {e}", scratch.display()))?;
    }
    fs::create_dir_all(&scratch).map_err(|e| format!("{}: {e}", scratch.display()))?;

    let read_fixture = |path: &str| -> Result<FlightLog, String> {
        let bytes = fs::read(root.join(path)).map_err(|e| format!("{path}: {e}"))?;
        import::read_one(&bytes, None, None).map_err(|e| format!("{path}: {e}"))
    };
    let (tracked, second) = (read_fixture(TRACKED)?, read_fixture(SECOND_BOARD)?);
    write_all(
        &scratch,
        "invented",
        &[
            ("invented-tracked.csv", &tracked),
            ("second-board.csv", &second),
        ],
    )?;

    let fetched = root.join(ROCKETS).exists();
    if check && !fetched {
        return Err(format!("{ROCKETS} isn't fetched: cargo xtask refs fetch"));
    }
    let mut rows = String::new();
    if !invented_only && fetched {
        for (path, mapped) in ROCKETPY_LOGS {
            let log = read_rocketpy(&root, path, mapped)?;
            let name = path.rsplit('/').next().unwrap_or(path);
            let built = write_all(&scratch, &stem(path), &[(name, &log)])?;
            rows.push_str(&row(path, mapped, &built));
        }
    }
    let lines = read_back(&root, &scratch)?;
    let moments: usize = lines
        .iter()
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
        .map(|line| {
            let count = |key: &str| line[key].as_u64().unwrap_or(0) as usize;
            count("czml_moments") + count("gltf_moments")
        })
        .sum();
    println!(
        "ghost: {} ghost files read back; {moments} moments compared",
        lines.len()
    );
    if rows.is_empty() {
        return Ok(());
    }
    let text = report(&rows, moments);
    let path = root.join(REPORT);
    if check {
        let committed = fs::read_to_string(&path).map_err(|e| format!("{REPORT}: {e}"))?;
        if committed.replace("\r\n", "\n") != text {
            return Err(format!("{REPORT} is stale: run `cargo xtask ghost`"));
        }
    } else {
        fs::write(&path, text).map_err(|e| format!("{REPORT}: {e}"))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each mapped log has its mapping, each mapping a log, so the list can't lose one.
    #[test]
    fn every_mapping_belongs_to_a_listed_log() {
        let root = crate::designs::root().unwrap();
        let mut mapped: Vec<String> = ROCKETPY_LOGS
            .iter()
            .filter(|(_, mapped)| *mapped)
            .map(|(path, _)| format!("{}.json", stem(path)))
            .collect();
        mapped.sort();
        let mut files: Vec<String> = fs::read_dir(root.join(MAPPINGS))
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        files.sort();
        assert_eq!(mapped, files);
    }

    /// Where `refs/` holds them, each log's highest height sits by the apogee RocketPy's own
    /// notebooks state for that flight (`docs/examples/*_flight_sim.ipynb`): the check that each
    /// mapping's unit is right, since a foot read as a metre misses by a factor of 3.28.
    #[test]
    fn each_logs_highest_height_is_the_apogee_rocketpy_states() {
        let root = crate::designs::root().unwrap();
        if !root.join(ROCKETS).exists() {
            eprintln!("skipped: {ROCKETS} isn't fetched");
            return;
        }
        let stated = [
            ("NDRT_2020/ndrt_2020_flight_data.csv", 1_316.7),
            ("andromeda/flight_data.csv", 3_443.0),
            ("juno3/cots_altimeter.csv", 3_213.0),
            (
                "prometheus/2022-06-24-serial-5115-flight-0001-TeleMetrum.csv",
                3_903.77,
            ),
            (
                "prometheus/2022-06-24-serial-6583-flight-0003-TeleMega.csv",
                3_898.37,
            ),
            ("EPFL_Bella_Lui/bella_lui_flight_data_filtered.csv", 458.97),
            ("astg/altimeter_halcyon.csv", 3_450.0),
            ("astra/flight_data.csv", 3_250.0),
            ("camoes/flight_data.csv", 3_015.0),
            ("erebus11/flight_data_filtered.csv", 3_020.0),
            ("genesis/flight_data_faraday.csv", 2_916.0),
            ("hedy/cats_tust/filteredDataInfo.csv", 5_231.53),
            ("hedy/cats_tust/flightInfo.csv", 5_231.53),
            ("lince/main_data.csv", 3_587.0),
            ("polito/altimeter_cavour.csv", 2_789.0),
            ("valkyrie/flightInfo_merged.csv", 2_098.02),
        ];
        let folder = root.join("target/ghost-test");
        fs::create_dir_all(&folder).unwrap();
        for (path, apogee) in stated {
            let (_, mapped) = ROCKETPY_LOGS.iter().find(|(p, _)| *p == path).unwrap();
            let log = read_rocketpy(&root, path, *mapped).unwrap();
            let built = write_all(&folder, "one", &[(path, &log)]).unwrap();
            let top = built.tracks[0]
                .up_m
                .iter()
                .flatten()
                .copied()
                .fold(f64::NAN, f64::max);
            // Valkyrie's log peaks 1.9% under its card, the widest here.
            let share: f64 = top / apogee - 1.0;
            assert!(share.abs() < 0.02, "{path}: {top} against {apogee}");
        }
    }

    /// Where `refs/` holds them, the list is every log whose header names a height or a place,
    /// less the files it says aren't logs: so a log RocketPy adds can't go unseen.
    #[test]
    fn the_list_is_every_flight_log_rocketpy_holds() {
        let root = crate::designs::root().unwrap();
        let rockets = root.join(ROCKETS);
        if !rockets.exists() {
            eprintln!("skipped: {ROCKETS} isn't fetched");
            return;
        }
        fn walk(folder: &Path, found: &mut Vec<std::path::PathBuf>) {
            for entry in fs::read_dir(folder).unwrap().map(Result::unwrap) {
                let path = entry.path();
                if path.is_dir() {
                    walk(&path, found);
                } else if path.extension().is_some_and(|e| e == "csv") {
                    found.push(path);
                }
            }
        }
        let mut found = Vec::new();
        walk(&rockets, &mut found);
        let mut logs: Vec<String> = found
            .iter()
            .filter(|path| {
                let bytes = fs::read(path).unwrap();
                let text = String::from_utf8_lossy(&bytes);
                let header = text.lines().next().unwrap_or("").to_lowercase();
                header.split(',').any(|cell| {
                    let cell = cell.trim();
                    cell.contains("alt")
                        || cell.contains("lat")
                        || cell == "height"
                        || cell.starts_with("z (")
                })
            })
            .map(|path| {
                path.strip_prefix(&rockets)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/")
            })
            .collect();
        logs.sort();
        let mut listed: Vec<String> = ROCKETPY_LOGS.iter().map(|(p, _)| (*p).to_owned()).collect();
        listed.sort();
        assert_eq!(logs, listed);
    }
}
