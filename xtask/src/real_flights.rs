//! `cargo xtask real-flights [--check]`: flies RocketPy's example rockets that have a flight log
//! and compares each with its log (M2.3b, ADR-082), then flies the five public flights whose
//! predictions were merged first and puts each merged prediction beside its log's apogee and
//! its error (M2.3d2, ADR-328 §1b).
//!
//! The logs, thrust files and ERA5 files are read from the pinned checkouts under the
//! gitignored `refs/` (`cargo xtask refs fetch rocketpy` and `rocketry-files`), so this runs
//! where those checkouts are, not in CI. It writes `validation/reports/real-flights.{json,md}`; `--check` writes nothing and
//! fails unless the committed report reproduces. CI, without `refs/`, holds the committed report's
//! summary to its rows and its page to its data (`hpr_validate::tests`).

use std::fs;

use hpr_validate::real_flight::{REPORT_JSON, REPORT_MD, RealFlightReport, run as fly_all};

pub const USAGE: &str = "  real-flights [--check]
                           Fly RocketPy's example rockets that have a flight log, in their
                           ERA5 weather, and compare each with its log; then the five public
                           flights predicted first, each merged prediction beside its error;
                           needs refs/rocketpy and refs/public-flights/rocketry-files.
                           Writes validation/reports/real-flights.{md,json} and each flight's
                           residuals, flight-residuals.{md,json}, and apogee predicted from
                           each one's boost, from-boost.{md,json}; --check writes nothing and
                           fails unless the committed reports reproduce.";

/// Flies the flights and writes or checks the report.
pub fn run(args: &[String]) -> Result<(), String> {
    let mut check = false;
    for arg in args {
        match arg.as_str() {
            "--check" => check = true,
            other => return Err(format!("unknown argument `{other}`\n\n{USAGE}")),
        }
    }
    let root = crate::designs::root()?;
    let report = fly_all(&root).map_err(|error| error.to_string())?;
    for row in &report.flights {
        println!(
            "real-flights: {:<16} log {:>7.1} m in {:>5.2} s  hpr {:>7.1} m in {:>5.2} s  {:>+6.2}%  \
             trace RMS {:>6.1} m ({:.2}%)  as heights {:>+6.2}%  on the example's drag {:>+6.2}%{}",
            row.id,
            row.log_apogee_m,
            row.log_time_to_apogee_s,
            row.hpr_apogee_m,
            row.hpr_time_to_apogee_s,
            row.apogee_error_percent,
            row.trace_rms_m,
            row.trace_rms_percent,
            100.0 * (row.hpr_height_apogee_m - row.log_apogee_m) / row.log_apogee_m,
            row.example_drag_apogee_error_percent,
            row.recorded_thrust
                .as_ref()
                .map_or(String::new(), |flown| format!(
                    "  on the recorded thrust {:+.2}%",
                    flown.apogee_error_percent
                ))
        );
    }
    for row in &report.predicted_first {
        println!(
            "real-flights: {:<16} log {:>7.1} m  merged {:>7.1} m  hpr {:>7.1} m  {:>+6.2}%  \
             as heights {:>+6.2}%  {}{}",
            row.id,
            row.log_apogee_m,
            row.merged_apogee_m,
            row.hpr_apogee_m,
            row.apogee_error_percent,
            100.0 * (row.hpr_height_apogee_m - row.log_apogee_m) / row.log_apogee_m,
            if row.all_inputs_traced {
                "traced"
            } else {
                "not traced"
            },
            row.revised_because
                .as_ref()
                .map_or(String::new(), |because| format!("  revised, because {because}"))
        );
    }
    let summary = &report.summary;
    println!(
        "real-flights: {} flights, mean absolute apogee error {:.2}% (target {}%), outside it: {}",
        summary.flights,
        summary.mean_absolute_apogee_error_percent,
        summary.target_percent,
        if summary.outliers.is_empty() {
            "none".to_owned()
        } else {
            summary.outliers.join(", ")
        }
    );
    let markdown = report.to_markdown();
    report.check_consistent(&markdown)?;
    if check {
        let text = fs::read_to_string(root.join(REPORT_JSON))
            .map_err(|error| format!("reading {REPORT_JSON}: {error}"))?;
        let committed: RealFlightReport = serde_json::from_str(&text)
            .map_err(|error| format!("reading {REPORT_JSON}: {error}"))?;
        committed.reproduces(&report)?;
        let page = fs::read_to_string(root.join(REPORT_MD))
            .map_err(|error| format!("reading {REPORT_MD}: {error}"))?;
        committed.check_consistent(&page)?;
        println!("real-flights: the committed report reproduces");
        crate::flight_residuals::run(&root, true)?;
        return crate::from_boost::run(&root, &report, true);
    }
    let json = serde_json::to_string_pretty(&report).map_err(|error| error.to_string())? + "\n";
    for (path, text) in [(REPORT_JSON, json), (REPORT_MD, markdown)] {
        fs::write(root.join(path), text).map_err(|error| format!("writing {path}: {error}"))?;
    }
    println!("real-flights: wrote {REPORT_JSON} and {REPORT_MD}");
    crate::flight_residuals::run(&root, false)?;
    crate::from_boost::run(&root, &report, false)
}
