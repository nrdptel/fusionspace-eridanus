//! `cargo xtask public-predictions [--check]`: flies the listed public real flights that have not been
//! compared with their logs yet, from each one's record alone, and writes their predicted apogees
//! (M2.3d3, ADR-328 §1a, ADR-334).
//!
//! It reads the thrust files and the converted ERA5 file from the gitignored `refs/` (`cargo
//! xtask refs fetch rocketpy python`, then `validation/oracles/netcdf/euroc_2022.py`), so it
//! runs where they are, not in CI. It hashes each flight's log and never parses one. It writes
//! `validation/reports/public-predictions.{json,md}`; `--check` writes nothing and fails unless
//! the committed predictions reproduce. CI, without `refs/`, holds the committed predictions to
//! the code's list of flights and the page to its data (`hpr_validate::tests`).

use std::fs;

use hpr_validate::public_predictions::{
    PREDICTIONS_JSON, PREDICTIONS_MD, PredictionsReport, run as predict_all,
};

pub const USAGE: &str = "  public-predictions [--check]
                           Fly the public real flights not yet compared with their logs, each
                           from its record alone, and write their predicted apogees; needs
                           refs/rocketpy and the converted EuRoC 2022 weather file. Hashes
                           each log and never parses one. Writes
                           validation/reports/public-predictions.{md,json}; --check writes
                           nothing and fails unless the committed predictions reproduce.";

/// Flies the predictions and writes or checks them.
pub fn run(args: &[String]) -> Result<(), String> {
    let mut check = false;
    for arg in args {
        match arg.as_str() {
            "--check" => check = true,
            other => return Err(format!("unknown argument `{other}`\n\n{USAGE}")),
        }
    }
    let root = crate::designs::root()?;
    let report = predict_all(&root).map_err(|error| error.to_string())?;
    for row in &report.flights {
        println!(
            "public-predictions: {:<10} predicted {:>7.1} m ({:>7.1} m as a height) in {:>5.2} s, \
             Mach {:.3}, {:.1} N s, {:.3} kg at liftoff, every input traced: {}{}",
            row.id,
            row.predicted_apogee_m,
            row.predicted_height_apogee_m,
            row.predicted_time_to_apogee_s,
            row.hpr_max_mach,
            row.total_impulse_ns,
            row.liftoff_mass_kg,
            if row.all_inputs_traced { "yes" } else { "no" },
            row.catalog_curve
                .as_ref()
                .map_or(String::new(), |catalog| format!(
                    "  on the catalog {} (never counted) {:.1} m",
                    catalog.motor, catalog.predicted_apogee_m
                ))
        );
    }
    let markdown = report.to_markdown();
    report.check_consistent(&markdown)?;
    if check {
        let text = fs::read_to_string(root.join(PREDICTIONS_JSON))
            .map_err(|error| format!("reading {PREDICTIONS_JSON}: {error}"))?;
        let committed: PredictionsReport = serde_json::from_str(&text)
            .map_err(|error| format!("reading {PREDICTIONS_JSON}: {error}"))?;
        committed.reproduces(&report)?;
        let page = fs::read_to_string(root.join(PREDICTIONS_MD))
            .map_err(|error| format!("reading {PREDICTIONS_MD}: {error}"))?;
        committed.check_consistent(&page)?;
        println!("public-predictions: the committed predictions reproduce");
        return Ok(());
    }
    let json = serde_json::to_string_pretty(&report).map_err(|error| error.to_string())? + "\n";
    for (path, text) in [(PREDICTIONS_JSON, json), (PREDICTIONS_MD, markdown)] {
        fs::write(root.join(path), text).map_err(|error| format!("writing {path}: {error}"))?;
    }
    println!("public-predictions: wrote {PREDICTIONS_JSON} and {PREDICTIONS_MD}");
    Ok(())
}
