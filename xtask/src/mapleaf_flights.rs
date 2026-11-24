//! `cargo xtask mapleaf-flights [--inputs | --check]`: HPR Sim against MAPLEAF on three public
//! designs (M1.14b8, ADR-378; the logic is `hpr_validate::mapleaf`).
//!
//! - `--inputs` writes `validation/fixtures/flight/mapleaf-inputs.json`, what HPR Sim flies of
//!   each design, for `validation/oracles/mapleaf/flights.py` to hand to MAPLEAF.
//! - With no flag it flies HPR Sim on the same designs and writes
//!   `validation/reports/mapleaf-flights.{json,md}` from MAPLEAF's committed flights.
//! - `--check` writes nothing and fails unless the committed inputs and report reproduce.
//!
//! Everything it reads is committed, so it runs in CI. MAPLEAF itself runs only where its
//! environment is installed (`refs/venv-mapleaf`).

use std::fs;

use hpr_validate::mapleaf::{self, INPUTS_JSON, MapleafReport, REPORT_JSON, REPORT_MD};

pub const USAGE: &str = "  mapleaf-flights [--inputs | --check]
                           HPR Sim against MAPLEAF on three public designs. --inputs writes
                           validation/fixtures/flight/mapleaf-inputs.json for
                           validation/oracles/mapleaf/flights.py to fly; with no flag, flies
                           HPR Sim and writes validation/reports/mapleaf-flights.{md,json}
                           from MAPLEAF's committed flights; --check writes nothing and fails
                           unless the committed inputs and report reproduce.";

/// Writes the inputs, or writes or checks the report.
pub fn run(args: &[String]) -> Result<(), String> {
    let (mut inputs, mut check) = (false, false);
    for arg in args {
        match arg.as_str() {
            "--inputs" => inputs = true,
            "--check" => check = true,
            other => return Err(format!("unknown argument `{other}`\n\n{USAGE}")),
        }
    }
    if inputs && check {
        return Err(format!("--inputs and --check don't go together\n\n{USAGE}"));
    }
    let root = crate::designs::root()?;
    if inputs {
        let inputs = mapleaf::inputs(&root).map_err(|error| error.to_string())?;
        let json = serde_json::to_string_pretty(&inputs).map_err(|error| error.to_string())? + "\n";
        fs::write(root.join(INPUTS_JSON), json)
            .map_err(|error| format!("writing {INPUTS_JSON}: {error}"))?;
        println!(
            "mapleaf-flights: wrote {INPUTS_JSON} ({} designs); fly MAPLEAF on it next",
            inputs.cases.len()
        );
        return Ok(());
    }
    let report = mapleaf::run(&root).map_err(|error| error.to_string())?;
    for row in &report.rows {
        for (drag, compared) in [("same drag", &row.same_drag), ("own drag", &row.own_drag)] {
            println!(
                "mapleaf-flights: {:<24} {drag:<9} apogee {:>8.2} m against {:>8.2} m \
                 ({:+.2}%), rail exit {:>7.3} m/s against {:>7.3} m/s ({:+.3}%)",
                row.name,
                compared.hpr.apogee_rise_m,
                compared.mapleaf.apogee_rise_m,
                compared.apogee_difference_percent,
                compared.hpr.rail_exit_speed_m_s,
                compared.mapleaf.rail_exit_speed_m_s,
                compared.rail_exit_difference_percent,
            );
        }
    }
    let markdown = report.to_markdown();
    report.check_consistent(&markdown)?;
    if check {
        let text = fs::read_to_string(root.join(REPORT_JSON))
            .map_err(|error| format!("reading {REPORT_JSON}: {error}"))?;
        let committed: MapleafReport = serde_json::from_str(&text)
            .map_err(|error| format!("reading {REPORT_JSON}: {error}"))?;
        committed.reproduces(&report)?;
        let page = fs::read_to_string(root.join(REPORT_MD))
            .map_err(|error| format!("reading {REPORT_MD}: {error}"))?;
        committed.check_consistent(&page)?;
        println!("mapleaf-flights: the committed report reproduces");
        return Ok(());
    }
    let json = serde_json::to_string_pretty(&report).map_err(|error| error.to_string())? + "\n";
    for (path, text) in [(REPORT_JSON, json), (REPORT_MD, markdown)] {
        fs::write(root.join(path), text).map_err(|error| format!("writing {path}: {error}"))?;
    }
    println!("mapleaf-flights: wrote {REPORT_JSON} and {REPORT_MD}");
    Ok(())
}
