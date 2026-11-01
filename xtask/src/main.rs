//! Development automation for the hpr-sim workspace, run as `cargo xtask <command>`.
//!
//! Run `cargo xtask help` for the list of commands.

#![allow(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "a command-line tool reports on the terminal"
)]
#![allow(
    clippy::disallowed_methods,
    clippy::disallowed_types,
    reason = "dev tooling runs cargo and uses the filesystem; it is not part of the pure core"
)]

#[cfg(test)]
mod addresses;
mod aero;
mod aero_blunt;
mod aero_body;
mod aero_bullet;
mod aero_crossflow;
mod aero_drag;
mod aero_flare;
mod aero_gap;
mod aero_lip;
mod aero_mach;
mod aero_override;
mod aero_roll;
mod census;
mod cli;
#[cfg(test)]
mod conformance;
mod damaged;
mod design_checks;
mod designs;
#[cfg(test)]
mod docs;
mod evidence;
mod examples;
mod features;
mod figures;
mod fixture_flights;
mod format;
mod format_types;
mod ghost;
mod grib2;
mod guides;
mod layering;
mod logs;
mod messages;
mod motor_catalog;
mod motor_flights;
mod names;
mod ork;
mod ork_cli;
mod ork_corpus_flights;
mod ork_curves;
mod ork_export;
mod ork_export_flights;
mod ork_extensions;
mod ork_flights;
mod ork_format;
mod ork_geometry;
mod ork_library_flights;
mod ork_mass;
mod ork_motors;
mod ork_recovery;
mod ork_simulations;
mod ork_supply;
#[cfg(test)]
mod python_names;
mod real_flights;
mod records;
mod refs;
#[cfg(test)]
mod release;
mod roadmap;
mod scoreboard;
mod site;
mod spelling;
mod steps;
mod synthetic;
mod two_logs;
mod validate;
mod voice;
mod wasm_check;
mod workspace;

use std::process::ExitCode;

const USAGE_TEMPLATE: &str = "\
Usage: cargo xtask <command> [args]

Commands:
  wasm-check [cargo args]  Check the crate layering rules, and that the pure-core crates
                           build for wasm32-unknown-unknown. Extra arguments (for example
                           --locked) are passed on to `cargo check`.
{REFS}
{DESIGNS}
{DESIGN_CHECKS}
{AERO}
{VALIDATE}
{CENSUS}
{SCOREBOARD}
{CLI}
{SITE}
{SITE_MATH}
{SPELLING}
{EXAMPLES}
{FIGURES}
{GUIDES}
{EVIDENCE}
{FORMAT}
{GRIB2}
{ORK}
{ORK_FLIGHTS}
{ORK_CLI}
{ORK_CURVES}
{ORK_EXPORT_FLIGHTS}
{FIXTURE_FLIGHTS}
{LOGS}
{SYNTHETIC}
{TWO_LOGS}
{REAL_FLIGHTS}
{RECORDS}
{ROADMAP_CURRENT}
{MOTOR_CATALOG}
{NAMES}
{VOICE}
  help                     Print this message.";

fn usage() -> String {
    USAGE_TEMPLATE
        .replace("{REFS}", refs::USAGE)
        .replace("{DESIGNS}", designs::USAGE)
        .replace("{DESIGN_CHECKS}", design_checks::USAGE)
        .replace("{AERO}", aero::USAGE)
        .replace("{VALIDATE}", validate::USAGE)
        .replace("{CENSUS}", census::USAGE)
        .replace("{SCOREBOARD}", scoreboard::USAGE)
        .replace("{CLI}", cli::USAGE)
        .replace("{SITE}", site::USAGE)
        .replace("{SITE_MATH}", site::math::USAGE)
        .replace("{SPELLING}", spelling::USAGE)
        .replace("{EXAMPLES}", examples::USAGE)
        .replace("{FIGURES}", figures::USAGE)
        .replace("{GUIDES}", guides::USAGE)
        .replace("{EVIDENCE}", evidence::USAGE)
        .replace("{FORMAT}", format::USAGE)
        .replace("{GRIB2}", grib2::USAGE)
        .replace("{ORK}", ork::USAGE)
        .replace("{ORK_FLIGHTS}", ork_flights::USAGE)
        .replace("{ORK_CLI}", ork_cli::USAGE)
        .replace("{ORK_CURVES}", ork_curves::USAGE)
        .replace("{ORK_EXPORT_FLIGHTS}", ork_export_flights::USAGE)
        .replace("{FIXTURE_FLIGHTS}", fixture_flights::USAGE)
        .replace("{LOGS}", logs::USAGE)
        .replace("{SYNTHETIC}", synthetic::USAGE)
        .replace("{TWO_LOGS}", two_logs::USAGE)
        .replace("{REAL_FLIGHTS}", real_flights::USAGE)
        .replace("{RECORDS}", records::USAGE)
        .replace("{ROADMAP_CURRENT}", roadmap::CURRENT_USAGE)
        .replace("{MOTOR_CATALOG}", motor_catalog::USAGE)
        .replace("{NAMES}", names::USAGE)
        .replace("{VOICE}", voice::USAGE)
}

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let result = match args.next().as_deref() {
        Some("wasm-check") => wasm_check::run(&args.collect::<Vec<_>>()),
        Some("refs") => refs::run(&args.collect::<Vec<_>>()),
        Some("designs") => designs::run(&args.collect::<Vec<_>>()),
        Some("design-checks") => design_checks::run(&args.collect::<Vec<_>>()),
        Some("aero") => aero::run(&args.collect::<Vec<_>>()),
        Some("validate") => validate::run(&args.collect::<Vec<_>>()),
        Some("census") => census::run(&args.collect::<Vec<_>>()),
        Some("scoreboard") => scoreboard::run(&args.collect::<Vec<_>>()),
        Some("cli") => cli::run(&args.collect::<Vec<_>>()),
        Some("site") => site::run(&args.collect::<Vec<_>>()),
        Some("site-math") => site::math::run(&args.collect::<Vec<_>>()),
        Some("spelling") => spelling::run(&args.collect::<Vec<_>>()),
        Some("examples") => examples::run(&args.collect::<Vec<_>>()),
        Some("figures") => figures::run(&args.collect::<Vec<_>>()),
        Some("guides") => guides::run(&args.collect::<Vec<_>>()),
        Some("evidence") => evidence::run(&args.collect::<Vec<_>>()),
        Some("format") => format::run(&args.collect::<Vec<_>>()),
        Some("grib2-values") => grib2::run(&args.collect::<Vec<_>>()),
        Some("ork") => ork::run(&args.collect::<Vec<_>>()),
        Some("ork-flights") => ork_flights::run(&args.collect::<Vec<_>>()),
        Some("ork-cli") => ork_cli::run(&args.collect::<Vec<_>>()),
        Some("ork-curves") => ork_curves::run(&args.collect::<Vec<_>>()),
        Some("ork-export-flights") => ork_export_flights::run(&args.collect::<Vec<_>>()),
        Some("fixture-flights") => fixture_flights::run(&args.collect::<Vec<_>>()),
        Some("real-flights") => real_flights::run(&args.collect::<Vec<_>>()),
        Some("logs") => logs::run(&args.collect::<Vec<_>>()),
        Some("synthetic") => synthetic::run(&args.collect::<Vec<_>>()),
        Some("two-logs") => two_logs::run(&args.collect::<Vec<_>>()),
        Some("ghost") => ghost::run(&args.collect::<Vec<_>>()),
        Some("records") => records::run(&args.collect::<Vec<_>>()),
        Some("roadmap-current") => roadmap::run_current(&args.collect::<Vec<_>>()),
        Some("motor-catalog") => motor_catalog::run(&args.collect::<Vec<_>>()),
        Some("names") => names::run(&args.collect::<Vec<_>>()),
        Some("voice") => voice::run(&args.collect::<Vec<_>>()),
        Some("help" | "-h" | "--help") => {
            println!("{}", usage());
            Ok(())
        }
        Some(other) => Err(format!("unknown command `{other}`\n\n{}", usage())),
        None => Err(format!("no command given\n\n{}", usage())),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}
