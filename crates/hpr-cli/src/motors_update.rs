//! `hpr motors update`: the bundled catalog described (its date, counts, passing files and size),
//! and the motors and files ThrustCurve.org lists that it lacks, fetched into the cache so they
//! are found offline after ([M5.7d](https://hpr.fusionspace.co/decisions-and-roadmap.html#m5-7d);
//! [ADR-288](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0288-every-solid-motor-ships-in-the-package.md)
//! §4). [`hpr::hpr_net::catalog_update`] does the comparison. Offline, it reads what an earlier
//! update cached, or with any answer it needs missing from the cache names the bundle's date.

use std::io::Write;

use hpr::hpr_motor::Catalog;
use hpr::hpr_motor::catalog::bundled_size_bytes;
use hpr::hpr_net::catalog_update::{self, Bundle, Newer};
use hpr::hpr_net::on_demand::{self, Wanted};
use hpr::hpr_net::thrustcurve::{self, Format, ThrustCurveError};
use hpr::hpr_net::{Client, NetError, Transport};

use crate::console::Level;
use crate::motor_fetch::{build, command, shell_word};
use crate::output::{
    BundledCatalog, CatalogCheck, FileFormat, MotorsUpdate, NewerFile, NewerMotor,
};
use crate::weather::{client, now_s, read_from, read_from_line};
use crate::{Failure, Out, printable};

/// `hpr motors update`'s arguments.
#[derive(Debug, clap::Args)]
pub struct UpdateArgs {
    /// Answer from the cache only, never the network: an earlier update's answers, or the bundled
    /// catalog alone when one is missing
    #[arg(long)]
    pub offline: bool,
}

/// Runs `hpr motors update`.
pub(crate) fn run(args: &UpdateArgs, to: &mut Out<'_>) -> Result<(), Failure> {
    let catalog = crate::motors::catalog()?;
    let client = client(args.offline, "hpr motors update")?;
    let document = update(&client, catalog, &Bundle::of(catalog))?;
    to.emit(&document, |out, diagnostics| {
        out.lines(&bundle_lines(&document.bundle), &[])?;
        match &document.checked {
            Some(checked) => {
                out.lines(&check_lines(checked), &[])?;
            }
            None => diagnostics.line(
                Level::Note,
                &format!(
                    "offline, with no whole update cached; online, `hpr motors update` looks \
                     for motors and files newer than {}",
                    document.bundle.captured
                ),
            )?,
        }
        writeln!(out, "{}", document.attribution.join(" "))
    })
}

/// The document for `catalog`, compared with ThrustCurve.org through `client` as `bundle`. Each
/// newer motor and file is then looked up by name as `hpr motors fetch` and `hpr sim --motor`
/// look it up, so the answers they read are cached too.
fn update<T: Transport>(
    client: &Client<T>,
    catalog: &Catalog,
    bundle: &Bundle,
) -> Result<MotorsUpdate, Failure> {
    let counts = catalog.counts();
    let bundled = BundledCatalog {
        captured: catalog.snapshot.captured.clone(),
        motors: counts.motors,
        motors_flying: counts.motors_flying,
        files: counts.files,
        left_out: counts.left_out,
        files_passing: counts.files_passing,
        motors_passing: counts.motors_passing,
        size_bytes: bundled_size_bytes(),
    };
    let now = now_s();
    let checked = match catalog_update::update(client, bundle, now) {
        Ok(found) => Some(CatalogCheck {
            read_from: found
                .fetched
                .first()
                .map(read_from)
                .ok_or_else(|| Failure::helped("the update read no answer", crate::BUG_HELP))?,
            listed: found.listed,
            looked_at: found.looked_at,
            newer: found
                .newer
                .iter()
                .map(|newer| newer_motor(client, catalog, newer, now))
                .collect(),
            unlisted: found.unlisted,
        }),
        Err(ThrustCurveError::Net(NetError::NotCached { .. })) => None,
        Err(error) => return Err(refused(&error, &bundled.captured)),
    };
    Ok(MotorsUpdate {
        attribution: vec![thrustcurve::ATTRIBUTION.to_owned()],
        bundle: bundled,
        checked,
    })
}

/// A newer motor as the document lists it, with the command that finds it from the cache: for a
/// motor the bundle lacks, `hpr sim --motor` by its designation where that finds it alone and the
/// bundled catalog has no motor by that name ([`flies_by_name`]), else `hpr motors fetch` with its
/// maker; for a newer file of a bundled motor, `hpr motors fetch` with
/// its maker and the file. Looking it up caches what that command reads.
fn newer_motor<T: Transport>(
    client: &Client<T>,
    catalog: &Catalog,
    newer: &Newer,
    now: u64,
) -> NewerMotor {
    let record = &newer.record;
    let maker = record
        .manufacturer_abbrev
        .clone()
        .unwrap_or_else(|| record.manufacturer.clone());
    let finds = |wanted: &Wanted| {
        on_demand::find_with(client, wanted, now, build)
            .ok()
            .filter(|(found, _)| found.record.motor_id == record.motor_id)
            .map(|(found, _)| found.file.simfile_id)
    };
    let files: Vec<NewerFile> = newer
        .files
        .iter()
        .map(|file| {
            let reads = if newer.new_motor {
                None
            } else {
                let wanted = Wanted::by(&maker, &record.designation).preferring(&file.simfile_id);
                (finds(&wanted).as_deref() == Some(file.simfile_id.as_str()))
                    .then(|| command(&wanted))
                    .flatten()
            };
            NewerFile {
                file_id: file.simfile_id.clone(),
                format: match file.format {
                    Format::RockSim => FileFormat::Rse,
                    _ => FileFormat::Eng,
                },
                measured_by: file.source.clone(),
                license: file.license.clone(),
                command: reads,
            }
        })
        .collect();
    let flies = if newer.new_motor {
        let alone = Wanted::named(&record.designation);
        let by_maker = Wanted::by(&maker, &record.designation);
        if flies_by_name(catalog, &record.designation)
            && finds(&alone).is_some()
            && let Some(word) = shell_word(&record.designation)
        {
            Some(format!("hpr sim --motor {word}"))
        } else if finds(&by_maker).is_some() {
            command(&by_maker)
        } else {
            None
        }
    } else {
        None
    };
    NewerMotor {
        manufacturer: maker,
        designation: record.designation.clone(),
        common_name: record.common_name.clone(),
        motor_id: record.motor_id.clone(),
        new_motor: newer.new_motor,
        command: flies,
        files,
    }
}

/// Whether `hpr sim --motor NAME` reaches the cache for `name`: only when the bundled catalog
/// finds no motor by it ([`crate::sim::catalog_or_fetched`] tries the catalog first). A new motor
/// whose designation is a bundled motor's designation or common name would fly that motor.
fn flies_by_name(catalog: &Catalog, name: &str) -> bool {
    matches!(
        hpr::Motor::from_catalog_in(catalog, name),
        Err(hpr::Error::NoSuchMotor(_))
    )
}

/// Why ThrustCurve.org couldn't be checked, with the bundle's date: the bundled catalog flies as
/// it is.
fn refused(error: &ThrustCurveError, captured: &str) -> Failure {
    let mut help = match error {
        ThrustCurveError::Net(net) => vec![crate::fetch_help(net)],
        _ => Vec::new(),
    };
    help.push(format!(
        "the bundled catalog, ThrustCurve.org's of {captured}, flies with no network as it is"
    ));
    Failure::Helped {
        message: printable(&format!("ThrustCurve.org couldn't be checked: {error}")),
        help,
    }
}

/// The bundled catalog in words.
fn bundle_lines(bundle: &BundledCatalog) -> Vec<String> {
    vec![
        format!(
            "The bundled catalog: ThrustCurve.org's solid motors as of {}",
            bundle.captured
        ),
        format!(
            "  motors           {}, {} with a file that reads",
            bundle.motors, bundle.motors_flying
        ),
        format!(
            "  files            {}, and {} left out",
            bundle.files, bundle.left_out
        ),
        format!(
            "  passing          {} files within 1% of their records, of {} motors",
            bundle.files_passing, bundle.motors_passing
        ),
        format!("  size             {}", megabytes(bundle.size_bytes)),
    ]
}

/// A size in megabytes (10⁶ bytes), to one decimal, with the bytes: `4.2 MB (4163396 bytes)`.
fn megabytes(bytes: usize) -> String {
    // A size well inside f64's exact integers; the bytes print exactly beside it.
    #[allow(clippy::cast_precision_loss, reason = "a few megabytes, exact in f64")]
    let mb = bytes as f64 / 1e6;
    format!("{mb:.1} MB ({bytes} bytes)")
}

/// What ThrustCurve.org's answer showed, in words.
fn check_lines(checked: &CatalogCheck) -> Vec<String> {
    let mut lines = vec![format!(
        "ThrustCurve.org lists {} solid motors ({})",
        checked.listed,
        read_from_line(&checked.read_from).to_lowercase()
    )];
    if checked.newer.is_empty() {
        lines.push("  none newer than the bundle, and no newer file".to_owned());
    }
    for motor in &checked.newer {
        let what = if motor.new_motor {
            "a motor the bundle lacks"
        } else {
            "the bundled motor's newer files"
        };
        lines.push(printable(&format!(
            "  {} {}, {what}:",
            motor.manufacturer, motor.designation
        )));
        for file in &motor.files {
            let format = match file.format {
                FileFormat::Eng => "RASP (.eng)",
                FileFormat::Rse => "RockSim (.rse)",
            };
            lines.push(printable(&format!(
                "    file {}, {format}, from {}",
                file.file_id,
                file.measured_by.as_deref().unwrap_or("a source not stated")
            )));
            if let Some(command) = &file.command {
                lines.push(format!("      `{command}` reads it, offline too"));
            }
        }
        if let Some(command) = &motor.command {
            // `hpr sim --motor` flies it; `hpr motors fetch`, for a name several makers share,
            // reads it.
            let does = if command.starts_with("hpr sim") {
                "flies"
            } else {
                "reads"
            };
            lines.push(format!("    `{command}` {does} it, offline too"));
        }
    }
    if !checked.unlisted.is_empty() {
        lines.push(format!(
            "  {} of the bundle's motors are no longer listed; they stay in the bundle",
            checked.unlisted.len()
        ));
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    use hpr::hpr_net::{Cache, Mode, Replay};

    fn replay() -> Replay {
        Replay::open(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../hpr-net/tests/fixtures/replay"),
        )
        .unwrap()
    }

    /// ThrustCurve.org's answers recorded on 2026-10-29 (`hpr-net`'s `tests/catalog_update.rs`)
    /// against the bundle with ZEVETA's ROS-40-5 and AeroTech H128W's RockSim file taken out: the
    /// update lists both, and caches what `hpr motors fetch` reads, so each command it prints
    /// works offline after, over the same cache. `hpr sim --motor` looks a fetched motor up as
    /// `hpr motors fetch` does given the name alone (`Wanted::named`), so it reads the same answers.
    #[test]
    fn a_newer_motor_and_file_from_recorded_answers_land_in_the_cache() {
        let dir = tempfile::tempdir().unwrap();
        let catalog = Catalog::bundled().unwrap();
        let mut bundle = Bundle::of(catalog);
        assert!(bundle.remove_motor("6aa672715eabcd0002df9b36"));
        assert!(bundle.remove_file("5f4294d20002e90000000397"));
        let client = Client::new(replay(), Cache::new(dir.path()), Mode::Online);
        let document = update(&client, catalog, &bundle).unwrap();
        let checked = document.checked.as_ref().unwrap();
        assert_eq!((checked.listed, checked.looked_at), (1_004, 2));
        let lines = check_lines(checked).join("\n");
        // The bundled catalog still has a ROS-40-5, which `hpr sim --motor ROS-40-5` would fly,
        // so the command printed for the new motor names its maker.
        assert_eq!(
            lines,
            "ThrustCurve.org lists 1004 solid motors (fetched now)\n  \
             ZEVETA ROS-40-5, a motor the bundle lacks:\n    \
             file 6aa6d2a7593d99000250d4f8, RASP (.eng), from user\n    \
             `hpr motors fetch --manufacturer ZEVETA ROS-40-5` reads it, offline too\n  \
             AeroTech H128W, the bundled motor's newer files:\n    \
             file 5f4294d20002e90000000397, RockSim (.rse), from user\n      \
             `hpr motors fetch --manufacturer AeroTech --file 5f4294d20002e90000000397 H128W` \
             reads it, offline too"
        );

        // Offline over the same cache, each command printed finds its motor and file.
        let printed = [
            (
                checked.newer[0].command.clone().unwrap(),
                &checked.newer[0].files[0].file_id,
            ),
            (
                checked.newer[1].files[0].command.clone().unwrap(),
                &checked.newer[1].files[0].file_id,
            ),
        ];
        for (command, file) in printed {
            let args: Vec<&str> = command.split(' ').collect();
            let (mut out, mut err) = (Vec::new(), Vec::new());
            let exit = crate::run_offline(args, &mut out, &mut err, dir.path());
            let (out, err) = (String::from_utf8_lossy(&out), String::from_utf8_lossy(&err));
            assert_eq!(exit, crate::Exit::Success, "{command}: {out} {err}");
            assert!(
                out.contains("from the cache") && out.contains(file.as_str()),
                "{out}"
            );
        }
    }

    /// `hpr sim --motor NAME` is printed only for a name the bundled catalog finds no motor by: a
    /// bundled designation or common name would fly the bundled motor.
    #[test]
    fn the_sim_command_is_only_for_a_name_the_bundle_lacks() {
        let catalog = Catalog::bundled().unwrap();
        assert!(!flies_by_name(catalog, "ROS-40-5"));
        assert!(!flies_by_name(catalog, "H128W"));
        // A common name another motor's designation shares (`J360`) is in the catalog too.
        assert!(!flies_by_name(catalog, "J360"));
        assert!(flies_by_name(catalog, "Z99-NOT-A-MOTOR"));
    }

    /// Against the bundle as it is, the recorded answers find nothing newer, and offline over that
    /// cache `hpr motors update` reads the same from it.
    #[test]
    fn the_bundle_as_it_is_has_nothing_newer() {
        let dir = tempfile::tempdir().unwrap();
        let catalog = Catalog::bundled().unwrap();
        let client = Client::new(replay(), Cache::new(dir.path()), Mode::Online);
        let document = update(&client, catalog, &Bundle::of(catalog)).unwrap();
        let checked = document.checked.as_ref().unwrap();
        assert!(checked.newer.is_empty() && checked.unlisted.is_empty());
        assert_eq!(checked.looked_at, 0);
        let (mut out, mut err) = (Vec::new(), Vec::new());
        let exit = crate::run_offline(
            vec!["hpr", "motors", "update", "--offline"],
            &mut out,
            &mut err,
            dir.path(),
        );
        assert_eq!(exit, crate::Exit::Success);
        let text = String::from_utf8_lossy(&out);
        assert!(
            text.contains("ThrustCurve.org lists 1004 solid motors (from the cache"),
            "{text}"
        );
        assert!(text.contains("none newer than the bundle"), "{text}");
    }

    /// Offline with nothing cached, the update names the bundle's date and succeeds: the bundle
    /// flies as it is.
    #[test]
    fn offline_with_nothing_cached_names_the_bundles_date() {
        let dir = tempfile::tempdir().unwrap();
        let (mut out, mut err) = (Vec::new(), Vec::new());
        let exit = crate::run_offline(
            vec!["hpr", "motors", "update", "--offline"],
            &mut out,
            &mut err,
            dir.path(),
        );
        assert_eq!(exit, crate::Exit::Success);
        let (out, err) = (String::from_utf8_lossy(&out), String::from_utf8_lossy(&err));
        assert!(
            out.starts_with(
                "The bundled catalog: ThrustCurve.org's solid motors as of 2026-10-28\n"
            ),
            "{out}"
        );
        assert!(
            err.contains("no whole update cached") && err.contains("newer than 2026-10-28"),
            "{err}"
        );
    }

    /// A network failure is a failure that names the bundle's date.
    #[test]
    fn a_failed_check_names_the_bundles_date() {
        struct Down;
        impl Transport for Down {
            fn get(&self, _url: &str) -> Result<Vec<u8>, String> {
                Err("connection refused".to_owned())
            }
        }
        let dir = tempfile::tempdir().unwrap();
        let catalog = Catalog::bundled().unwrap();
        let client = Client::new(Down, Cache::new(dir.path()), Mode::Online);
        let Err(Failure::Helped { message, help }) = update(&client, catalog, &Bundle::of(catalog))
        else {
            panic!("the update should fail");
        };
        assert!(
            message.starts_with("ThrustCurve.org couldn't be checked"),
            "{message}"
        );
        assert!(
            help.last()
                .unwrap()
                .contains("ThrustCurve.org's of 2026-10-28"),
            "{help:?}"
        );
    }

    #[test]
    fn sizes_print_in_megabytes_with_the_bytes() {
        assert_eq!(megabytes(4_170_078), "4.2 MB (4170078 bytes)");
        assert_eq!(megabytes(0), "0.0 MB (0 bytes)");
    }
}
