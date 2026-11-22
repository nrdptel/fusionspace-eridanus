//! `cargo xtask descent-wind [--check]`: the wind reconstructed from each public flight's GPS
//! descent, set beside ERA5's (M1.14b6), from the pinned RocketPy checkout under the gitignored
//! `refs/`. It writes `validation/reports/descent-wind.{json,md}`; `--check` writes nothing and
//! fails unless the committed report reproduces. CI, without `refs/`, recomputes the committed
//! report's bands from its samples and its page from its data (`hpr_validate::tests`).

use std::fs;

use hpr_validate::descent_wind::{
    DescentWindReport, REPORT_JSON, REPORT_MD, Status, run as read_all,
};

pub const USAGE: &str =
    "  descent-wind [--check]   Reconstruct the wind from each public flight's GPS-tracked
                           descent and set it beside the flight's ERA5 wind, by height band;
                           needs refs/rocketpy. Writes validation/reports/descent-wind.{md,json};
                           --check writes nothing and fails unless the committed report
                           reproduces.";

/// Reads the flights and writes or checks the report.
pub fn run(args: &[String]) -> Result<(), String> {
    let mut check = false;
    for arg in args {
        match arg.as_str() {
            "--check" => check = true,
            other => return Err(format!("unknown argument `{other}`\n\n{USAGE}")),
        }
    }
    let root = crate::designs::root()?;
    let report = read_all(&root)?;
    for flight in &report.flights {
        let status = match &flight.status {
            Status::Compared => "compared".to_owned(),
            Status::NotCompared(why) => format!("not compared: {why}"),
            Status::NotRun(why) => format!("not run: {why}"),
        };
        println!(
            "descent-wind: {:<16} {:>4} fixes {:>4} repeats {:>4} read  {status}",
            flight.id, flight.fixes, flight.repeats, flight.read
        );
    }
    for band in report.bands.iter().chain(std::iter::once(&report.all)) {
        println!(
            "descent-wind: {:<14} m  {} flights {:>4} samples  speed mean {:+.2} RMS {:.2}  vector RMS {:.2} m/s",
            band.label,
            band.flights,
            band.samples,
            band.mean_speed_difference_m_s,
            band.rms_speed_difference_m_s,
            band.rms_vector_difference_m_s
        );
    }
    for pair in &report.receivers {
        println!(
            "descent-wind: {} two receivers: {} pairs, {:.2} m/s RMS, {:.2} m/s at most",
            pair.id,
            pair.pairs,
            pair.rms_vector_difference_m_s,
            // A bound, rounded up.
            (pair.max_vector_difference_m_s * 100.0).ceil() / 100.0
        );
    }
    let markdown = report.to_markdown();
    if check {
        let text = fs::read_to_string(root.join(REPORT_JSON))
            .map_err(|error| format!("reading {REPORT_JSON}: {error}"))?;
        let committed: DescentWindReport = serde_json::from_str(&text)
            .map_err(|error| format!("reading {REPORT_JSON}: {error}"))?;
        if committed != report {
            return Err(format!(
                "{REPORT_JSON} does not reproduce: run `cargo xtask descent-wind` and review the change"
            ));
        }
        let page = fs::read_to_string(root.join(REPORT_MD))
            .map_err(|error| format!("reading {REPORT_MD}: {error}"))?;
        committed.check_page(&page)?;
        println!("descent-wind: the committed report reproduces");
        return Ok(());
    }
    let json = serde_json::to_string_pretty(&report).map_err(|error| error.to_string())? + "\n";
    for (path, text) in [(REPORT_JSON, json), (REPORT_MD, markdown)] {
        fs::write(root.join(path), text).map_err(|error| format!("writing {path}: {error}"))?;
    }
    println!("descent-wind: wrote {REPORT_JSON} and {REPORT_MD}");
    Ok(())
}
